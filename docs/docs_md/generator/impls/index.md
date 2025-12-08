*[docs_md](../../index.md) / [generator](../index.md) / [impls](index.md)*

---

# Module `impls`

Implementation block rendering for documentation generation.

This module provides the [`ImplRenderer`](#implrenderer) struct which handles rendering
impl blocks (both inherent and trait implementations) to markdown format.

## Structs

### `ImplRenderer<'a>`

```rust
struct ImplRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

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

- `fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../context/index.md)

- `fn process_docs(self: &Self, item: &Item) -> Option<String>`

- `fn render_impl_blocks(self: &Self, md: &mut String, item_id: Id)`

- `fn render_trait_impl(self: &Self, md: &mut String, impl_block: &Impl)`

- `fn render_impl_methods(self: &Self, md: &mut String, impl_block: &Impl)`

- `fn render_generic_args_for_impl(self: &Self, args: &rustdoc_types::GenericArgs) -> String`

#### Trait Implementations

##### `impl<T> Instrument for ImplRenderer<'a>`

##### `impl<T> IntoEither for ImplRenderer<'a>`

##### `impl<D> OwoColorize for ImplRenderer<'a>`

##### `impl<T> Pointable for ImplRenderer<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for ImplRenderer<'a>`

## Functions

### `is_blanket_impl`

```rust
fn is_blanket_impl(impl_block: &rustdoc_types::Impl) -> bool
```

Check if an impl block is for a blanket trait that should be filtered.

Returns `true` if the impl is for one of the commonly auto-derived traits
that add noise to documentation (From, Into, Any, Borrow, etc.).

## Constants

### `BLANKET_TRAITS`

```rust
const BLANKET_TRAITS: &[&str];
```

Blanket trait implementations to filter from output.

These are automatically derived by the compiler and add noise to documentation
without providing useful information. Users who want them can use `--include-blanket-impls`.

