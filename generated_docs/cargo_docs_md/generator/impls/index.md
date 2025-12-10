*[cargo_docs_md](../../index.md) / [generator](../index.md) / [impls](index.md)*

---

# Module `impls`

Implementation block rendering for documentation generation.

This module provides the [`ImplRenderer`](#implrenderer) struct which handles rendering
impl blocks (both inherent and trait implementations) to markdown format.

# Path Types in `rustdoc_types::Impl`

When working with impl blocks, there are two different path representations:

- **`Impl.trait_: Option<Path>`** - The trait being implemented
  - `Path.path` is a `String` like `"Clone"` or `"std::fmt::Debug"`
  - Use for display/rendering in documentation output
  - Extract trait name with `path.rsplit("::").next()`

- **`ItemSummary.path: Vec<String>`** - Structured path from `krate.paths`
  - Example: `["std", "clone", "Clone"]`
  - Use for lookups and resolution via item IDs

The `Path.path` string is already formatted for display, while
`ItemSummary.path` is structured for programmatic manipulation.

## Contents

- [Structs](#structs)
  - [`ImplRenderer`](#implrenderer)
- [Functions](#functions)
  - [`is_trivial_derive_impl`](#is_trivial_derive_impl)
  - [`get_trivial_derive_description`](#get_trivial_derive_description)
  - [`is_blanket_impl`](#is_blanket_impl)
  - [`is_generic_type`](#is_generic_type)
  - [`generic_args_contain_generic`](#generic_args_contain_generic)
- [Constants](#constants)
  - [`BLANKET_TRAITS`](#blanket_traits)
  - [`TRIVIAL_DERIVE_TRAITS`](#trivial_derive_traits)
  - [`TRIVIAL_DERIVE_DESCRIPTIONS`](#trivial_derive_descriptions)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplRenderer`](#implrenderer) | struct | Renders impl blocks to markdown. |
| [`is_trivial_derive_impl`](#is_trivial_derive_impl) | fn | Check if an impl block is for a trivial derive trait. |
| [`get_trivial_derive_description`](#get_trivial_derive_description) | fn | Get the description for a trivial derive trait. |
| [`is_blanket_impl`](#is_blanket_impl) | fn | Check if an impl block is for a blanket trait that should be filtered. |
| [`is_generic_type`](#is_generic_type) | fn | Check if a type is generic (contains a type parameter like `T`). |
| [`generic_args_contain_generic`](#generic_args_contain_generic) | fn | Check if generic args contain any generic type parameters. |
| [`BLANKET_TRAITS`](#blanket_traits) | const | Blanket trait implementations to filter from output. |
| [`TRIVIAL_DERIVE_TRAITS`](#trivial_derive_traits) | const | Trivial derive trait implementations that can be collapsed. |
| [`TRIVIAL_DERIVE_DESCRIPTIONS`](#trivial_derive_descriptions) | const | Short descriptions for trivial derive traits, used in summary tables. |

## Structs

### `ImplRenderer<'a>`

```rust
struct ImplRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/generator/impls.rs:248-257`*

Renders impl blocks to markdown.

This struct handles:
- Inherent implementations (`impl MyType { ... }`)
- Trait implementations (`impl Trait for MyType { ... }`)
- Method signatures within impl blocks
- Associated types and constants

The renderer is generic over [`RenderContext`](../context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Cached type renderer to avoid repeated construction.

#### Implementations

- <span id="implrenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../context/index.md#rendercontext)

- <span id="implrenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="implrenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id)`

- <span id="implrenderer-render-trivial-derives-collapsed"></span>`fn render_trivial_derives_collapsed(md: &mut String, impls: &[&&Impl])`

- <span id="implrenderer-render-trait-impl"></span>`fn render_trait_impl(&self, md: &mut String, impl_block: &Impl)`

- <span id="implrenderer-render-impl-methods"></span>`fn render_impl_methods(&self, md: &mut String, impl_block: &Impl)`

- <span id="implrenderer-render-generic-args-for-impl"></span>`fn render_generic_args_for_impl(&self, args: &rustdoc_types::GenericArgs) -> String`

#### Trait Implementations

##### `impl Instrument for ImplRenderer<'a>`

##### `impl IntoEither for ImplRenderer<'a>`

##### `impl OwoColorize for ImplRenderer<'a>`

##### `impl Pointable for ImplRenderer<'a>`

- <span id="implrenderer-const-align"></span>`const ALIGN: usize`

- <span id="implrenderer-type-init"></span>`type Init = T`

- <span id="implrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ImplRenderer<'a>`

## Functions

### `is_trivial_derive_impl`

```rust
fn is_trivial_derive_impl(impl_block: &rustdoc_types::Impl) -> bool
```

*Defined in `src/generator/impls.rs:113-127`*

Check if an impl block is for a trivial derive trait.

Returns `true` if the impl is for one of the commonly derived traits
`(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)`.

# Examples

```rust
use rustdoc_types::Impl;
// For an impl block with trait "Clone", returns true
// For an impl block with trait "Display", returns false
```

### `get_trivial_derive_description`

```rust
fn get_trivial_derive_description(trait_name: &str) -> Option<&'static str>
```

*Defined in `src/generator/impls.rs:133-142`*

Get the description for a trivial derive trait.

Returns `None` if the trait is not in the trivial derives list.

### `is_blanket_impl`

```rust
fn is_blanket_impl(impl_block: &rustdoc_types::Impl) -> bool
```

*Defined in `src/generator/impls.rs:149-163`*

Check if an impl block is for a blanket trait that should be filtered.

Returns `true` if the impl is for one of the commonly auto-derived traits
that add noise to documentation (From, Into, Any, Borrow, etc.).

### `is_generic_type`

```rust
fn is_generic_type(ty: &rustdoc_types::Type) -> bool
```

*Defined in `src/generator/impls.rs:188-218`*

Check if a type is generic (contains a type parameter like `T`).

This is used to determine whether to show generic parameters in impl blocks.
For blanket impls like `impl<T> Trait for T`, we only show the generics if
the `for_` type is actually generic. When the impl is instantiated for a
concrete type like `TocEntry`, we hide the generics.

# Examples

```text
// Generic type - returns true
is_generic_type(&Type::Generic("T")) == true

// Concrete type - returns false
is_generic_type(&Type::ResolvedPath { name: "TocEntry", .. }) == false

// Container with generic - returns true
is_generic_type(&Type::ResolvedPath {
    name: "Vec",
    args: Some(GenericArgs::AngleBracketed { args: [Type::Generic("T")] })
}) == true
```

### `generic_args_contain_generic`

```rust
fn generic_args_contain_generic(args: &rustdoc_types::GenericArgs) -> bool
```

*Defined in `src/generator/impls.rs:221-236`*

Check if generic args contain any generic type parameters.

## Constants

### `BLANKET_TRAITS`
```rust
const BLANKET_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:38-52`*

Blanket trait implementations to filter from output.

These are automatically derived by the compiler and add noise to documentation
without providing useful information. Users who want them can use `--include-blanket-impls`.

### `TRIVIAL_DERIVE_TRAITS`
```rust
const TRIVIAL_DERIVE_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:67-83`*

Trivial derive trait implementations that can be collapsed.

These are traits commonly derived via `#[derive(...)]` that have standard,
predictable implementations. When `RenderConfig.hide_trivial_derives` is enabled,
these are grouped into a collapsible `<details>` block with a summary table.

The list includes:
- **Cloning**: `Clone`, `Copy`
- **Formatting**: `Debug`
- **Default values**: `Default`
- **Equality**: `PartialEq`, `Eq`
- **Hashing**: `Hash`
- **Ordering**: `PartialOrd`, `Ord`

### `TRIVIAL_DERIVE_DESCRIPTIONS`
```rust
const TRIVIAL_DERIVE_DESCRIPTIONS: &[(&str, &str)];
```

*Defined in `src/generator/impls.rs:88-98`*

Short descriptions for trivial derive traits, used in summary tables.

Maps trait names to brief descriptions for the collapsible summary table.

