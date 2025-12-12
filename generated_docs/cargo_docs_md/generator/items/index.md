*[cargo_docs_md](../../index.md) / [generator](../index.md) / [items](index.md)*

---

# Module `items`

Item rendering for documentation generation.

This module provides the [`ItemRenderer`](#itemrenderer) struct which handles rendering
individual Rust items (structs, enums, traits, functions, macros, constants,
and type aliases) to markdown format.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ItemRenderer`](#itemrenderer) | struct | Renders individual Rust items to markdown. |

## Structs

### `ItemRenderer<'a>`

```rust
struct ItemRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/generator/items.rs:29-39`*

Renders individual Rust items to markdown.

This struct provides methods for rendering each type of documentable item:
- Structs with fields and implementations
- Enums with variants and implementations
- Traits with methods and associated types
- Functions with signatures
- Macros
- Constants
- Type aliases

The renderer is generic over [`RenderContext`](../context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Type renderer for converting rustdoc types to strings.
  Stored once to avoid redundant construction in each render method.

#### Implementations

- <span id="itemrenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../context/index.md#rendercontext)

- <span id="itemrenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="itemrenderer-maybe-render-source-location"></span>`fn maybe_render_source_location(&self, item: &Item) -> String`

- <span id="itemrenderer-resolve-use-target"></span>`fn resolve_use_target<'b>(&self, item: &'b Item) -> Option<(&'b str, &'b Item)>`

- <span id="itemrenderer-render-struct"></span>`fn render_struct(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="itemrenderer-render-enum"></span>`fn render_enum(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="itemrenderer-render-trait"></span>`fn render_trait(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="itemrenderer-render-trait-implementors"></span>`fn render_trait_implementors(&self, md: &mut String, trait_id: Id)`

- <span id="itemrenderer-render-function"></span>`fn render_function(&self, md: &mut String, item: &Item)`

- <span id="itemrenderer-render-function-type-links"></span>`fn render_function_type_links(&self, md: &mut String, f: &rustdoc_types::Function)`

- <span id="itemrenderer-render-macro"></span>`fn render_macro(&self, md: &mut String, item: &Item)`

- <span id="itemrenderer-render-constant"></span>`fn render_constant(&self, md: &mut String, item: &Item)`

- <span id="itemrenderer-render-type-alias"></span>`fn render_type_alias(&self, md: &mut String, item: &Item)`

- <span id="itemrenderer-render-union"></span>`fn render_union(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="itemrenderer-render-static"></span>`fn render_static(&self, md: &mut String, item: &Item)`

#### Trait Implementations

##### `impl Instrument for ItemRenderer<'a>`

##### `impl IntoEither for ItemRenderer<'a>`

##### `impl OwoColorize for ItemRenderer<'a>`

##### `impl Pointable for ItemRenderer<'a>`

- <span id="itemrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itemrenderer-pointable-type-init"></span>`type Init = T`

- <span id="itemrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itemrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itemrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itemrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ItemRenderer<'a>`

