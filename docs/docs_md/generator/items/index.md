*[docs_md](../../index.md) / [generator](../index.md) / [items](index.md)*

---

# Module `items`

Item rendering for documentation generation.

This module provides the [`ItemRenderer`](#itemrenderer) struct which handles rendering
individual Rust items (structs, enums, traits, functions, macros, constants,
and type aliases) to markdown format.

## Structs

### `ItemRenderer<'a>`

```rust
struct ItemRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

Renders individual Rust items to markdown.

This struct provides methods for rendering each type of documentable item:
- Structs with fields and implementations
- Enums with variants and implementations
- Traits with methods and associated types
- Functions with signatures
- Macros
- Constants
- Type aliases

The renderer is generic over [`RenderContext`](../index.md), allowing it to work with
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

- `fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self` — [`RenderContext`](../index.md)

- `fn process_docs(self: &Self, item: &Item) -> Option<String>`

- `fn resolve_use_target<'b>(self: &Self, item: &'b Item) -> Option<(&'b str, &'b Item)>`

- `fn render_struct(self: &Self, md: &mut String, item_id: Id, item: &Item)`

- `fn render_enum(self: &Self, md: &mut String, item_id: Id, item: &Item)`

- `fn render_trait(self: &Self, md: &mut String, item: &Item)`

- `fn render_function(self: &Self, md: &mut String, item: &Item)`

- `fn render_function_type_links(self: &Self, md: &mut String, f: &rustdoc_types::Function)`

- `fn render_macro(self: &Self, md: &mut String, item: &Item)`

- `fn render_constant(self: &Self, md: &mut String, item: &Item)`

- `fn render_type_alias(self: &Self, md: &mut String, item: &Item)`

#### Trait Implementations

##### `impl<T> Instrument for ItemRenderer<'a>`

##### `impl<T> IntoEither for ItemRenderer<'a>`

##### `impl<D> OwoColorize for ItemRenderer<'a>`

##### `impl<T> Pointable for ItemRenderer<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for ItemRenderer<'a>`

