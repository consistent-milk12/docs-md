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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplUtils`](#implutils) | struct | Utility functions to work with impl's and generics |
| [`ImplRenderer`](#implrenderer) | struct | Renders impl blocks to markdown. |
| [`BLANKET_TRAITS`](#blanket-traits) | const | Blanket trait implementations to filter from output. |
| [`TRIVIAL_DERIVE_TRAITS`](#trivial-derive-traits) | const | Trivial derive trait implementations that can be collapsed. |
| [`TRIVIAL_DERIVE_DESCRIPTIONS`](#trivial-derive-descriptions) | const | Short descriptions for trivial derive traits, used in summary tables. |

## Structs

### `ImplUtils`

```rust
struct ImplUtils;
```

*Defined in `src/generator/impls.rs:104`*

Utility functions to work with impl's and generics

#### Implementations

- <span id="implutils-is-trivial-derive-impl"></span>`fn is_trivial_derive_impl(impl_block: &Impl) -> bool`

- <span id="implutils-get-trivial-derive-description"></span>`fn get_trivial_derive_description(trait_name: &str) -> Option<&'static str>`

- <span id="implutils-is-blanket-impl"></span>`fn is_blanket_impl(impl_block: &Impl) -> bool`

- <span id="implutils-is-generic-type"></span>`fn is_generic_type(ty: &Type) -> bool`

- <span id="implutils-generic-args-contain-generic"></span>`fn generic_args_contain_generic(args: &GenericArgs) -> bool`

- <span id="implutils-extract-impl-signature-generics"></span>`fn extract_impl_signature_generics(impl_block: &Impl) -> HashSet<String>`

- <span id="implutils-extract-impl-visible-generics"></span>`fn extract_impl_visible_generics(impl_block: &Impl) -> HashSet<String>`

- <span id="implutils-extract-type-generics-into"></span>`fn extract_type_generics_into(ty: &Type, names: &mut HashSet<String>)`

- <span id="implutils-extract-generic-args-into"></span>`fn extract_generic_args_into(args: &GenericArgs, names: &mut HashSet<String>)`

- <span id="implutils-extract-term-generics-into"></span>`fn extract_term_generics_into(term: &Term, names: &mut HashSet<String>)`

- <span id="implutils-extract-bound-generics-into"></span>`fn extract_bound_generics_into(bound: &GenericBound, names: &mut HashSet<String>)`

- <span id="implutils-has-hidden-generic-refs"></span>`fn has_hidden_generic_refs(impl_block: &Impl, krate: &Crate) -> bool`

#### Trait Implementations

##### `impl Instrument for ImplUtils`

##### `impl IntoEither for ImplUtils`

##### `impl OwoColorize for ImplUtils`

##### `impl Pointable for ImplUtils`

- <span id="implutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implutils-pointable-type-init"></span>`type Init = T`

- <span id="implutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ImplUtils`

### `ImplRenderer<'a>`

```rust
struct ImplRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/generator/impls.rs:738-747`*

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

- <span id="implrenderer-render-generic-args-for-impl"></span>`fn render_generic_args_for_impl(&self, args: &GenericArgs) -> String`

#### Trait Implementations

##### `impl Instrument for ImplRenderer<'a>`

##### `impl IntoEither for ImplRenderer<'a>`

##### `impl OwoColorize for ImplRenderer<'a>`

##### `impl Pointable for ImplRenderer<'a>`

- <span id="implrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implrenderer-pointable-type-init"></span>`type Init = T`

- <span id="implrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ImplRenderer<'a>`

## Constants

### `BLANKET_TRAITS`
```rust
const BLANKET_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:41-55`*

Blanket trait implementations to filter from output.

These are automatically derived by the compiler and add noise to documentation
without providing useful information. Users who want them can use `--include-blanket-impls`.

### `TRIVIAL_DERIVE_TRAITS`
```rust
const TRIVIAL_DERIVE_TRAITS: &[&str];
```

*Defined in `src/generator/impls.rs:70-86`*

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

*Defined in `src/generator/impls.rs:91-101`*

Short descriptions for trivial derive traits, used in summary tables.

Maps trait names to brief descriptions for the collapsible summary table.

