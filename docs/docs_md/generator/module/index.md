*[docs_md](../../index.md) / [generator](../index.md) / [module](index.md)*

---

# Module `module`

Module markdown rendering for documentation generation.

This module provides the [`ModuleRenderer`](#modulerenderer) struct which handles rendering
a Rust module's documentation to markdown format, including all its items
organized by type.

## Structs

### `ModuleRenderer<'a>`

```rust
struct ModuleRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    is_root: bool,
}
```

Renders a module to markdown.

This struct handles the complete rendering of a module's documentation page,
including:
- Title (Crate or Module heading)
- Module-level documentation
- Sections for each item type (Modules, Structs, Enums, etc.)

The renderer is generic over [`RenderContext`](../context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`is_root`**: `bool`

  Whether this is the crate root module.

#### Implementations

- `fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](../../../generator/context/index.md)

- `fn process_docs(self: &Self, item: &Item) -> Option<String>`

- `fn render(self: &Self, item: &Item) -> String`

- `fn categorize_items(self: &Self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](../../../generator/module/index.md)

- `fn render_all_sections(self: &Self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](../../../generator/module/index.md)

- `fn render_modules_section(self: &Self, md: &mut String, modules: &[(&Id, &Item)])`

- `fn render_structs_section(self: &Self, md: &mut String, structs: &[(&Id, &Item)])`

- `fn render_enums_section(self: &Self, md: &mut String, enums: &[(&Id, &Item)])`

- `fn render_traits_section(self: &Self, md: &mut String, traits: &[(&Id, &Item)])`

- `fn render_functions_section(self: &Self, md: &mut String, functions: &[&Item])`

- `fn render_macros_section(self: &Self, md: &mut String, macros: &[&Item])`

- `fn render_constants_section(self: &Self, md: &mut String, constants: &[&Item])`

- `fn render_type_aliases_section(self: &Self, md: &mut String, type_aliases: &[&Item])`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

