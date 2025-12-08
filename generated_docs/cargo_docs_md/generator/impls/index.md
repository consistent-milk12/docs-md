*[cargo_docs_md](../../index.md) / [generator](../index.md) / [impls](index.md)*

---

# Module `impls`

Implementation block rendering for documentation generation.

This module provides the [`ImplRenderer`](#implrenderer) struct which handles rendering
impl blocks (both inherent and trait implementations) to markdown format.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplRenderer`](#implrenderer) | struct | Renders impl blocks to markdown. |
| [`is_blanket_impl`](#is_blanket_impl) | fn | Check if an impl block is for a blanket trait that should be filtered. |
| [`BLANKET_TRAITS`](#blanket_traits) | const | Blanket trait implementations to filter from output. |

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

The renderer is generic over [`RenderContext`](../index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Cached type renderer to avoid repeated construction.

#### Implementations

- <span id="implrenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../index.md)

- <span id="implrenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="implrenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id)`

- <span id="implrenderer-render-trait-impl"></span>`fn render_trait_impl(&self, md: &mut String, impl_block: &Impl)`

- <span id="implrenderer-render-impl-methods"></span>`fn render_impl_methods(&self, md: &mut String, impl_block: &Impl)`

- <span id="implrenderer-render-generic-args-for-impl"></span>`fn render_generic_args_for_impl(&self, args: &rustdoc_types::GenericArgs) -> String`

#### Trait Implementations

##### `impl<T> Instrument for ImplRenderer<'a>`

##### `impl<T> IntoEither for ImplRenderer<'a>`

##### `impl<D> OwoColorize for ImplRenderer<'a>`

##### `impl<T> Pointable for ImplRenderer<'a>`

- <span id="implrenderer-align"></span>`const ALIGN: usize`

- <span id="implrenderer-init"></span>`type Init = T`

- <span id="implrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

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

