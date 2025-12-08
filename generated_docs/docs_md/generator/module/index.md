*[docs_md](../../index.md) / [generator](../index.md) / [module](index.md)*

---

# Module `module`

Module markdown rendering for documentation generation.

This module provides the [`ModuleRenderer`](../index.md) struct which handles rendering
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

The renderer is generic over [`RenderContext`](../index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`is_root`**: `bool`

  Whether this is the crate root module.

#### Implementations

- `fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](../index.md)

- `fn process_docs(self: &Self, item: &Item) -> Option<String>`

- `fn render(self: &Self, item: &Item) -> String`

- `fn categorize_items(self: &Self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

- `fn expand_glob_reexport(self: &Self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](#categorizeditems)

- `fn render_all_sections(self: &Self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

- `fn render_modules_section(self: &Self, md: &mut String, modules: &[(&Id, &Item)])`

- `fn render_structs_section(self: &Self, md: &mut String, structs: &[(&Id, &Item)])`

- `fn render_enums_section(self: &Self, md: &mut String, enums: &[(&Id, &Item)])`

- `fn render_traits_section(self: &Self, md: &mut String, traits: &[(&Id, &Item)])`

- `fn render_functions_section(self: &Self, md: &mut String, functions: &[&Item])`

- `fn render_macros_section(self: &Self, md: &mut String, macros: &[&Item])`

- `fn render_constants_section(self: &Self, md: &mut String, constants: &[&Item])`

- `fn render_type_aliases_section(self: &Self, md: &mut String, type_aliases: &[&Item])`

#### Trait Implementations

##### `impl<T> Instrument for ModuleRenderer<'a>`

##### `impl<T> IntoEither for ModuleRenderer<'a>`

##### `impl<D> OwoColorize for ModuleRenderer<'a>`

##### `impl<T> Pointable for ModuleRenderer<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for ModuleRenderer<'a>`

### `CategorizedItems<'a>`

```rust
struct CategorizedItems<'a> {
    modules: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    structs: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    enums: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    traits: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    functions: Vec<&'a rustdoc_types::Item>,
    macros: Vec<&'a rustdoc_types::Item>,
    constants: Vec<&'a rustdoc_types::Item>,
    type_aliases: Vec<&'a rustdoc_types::Item>,
}
```

Items categorized by type for organized rendering.

Items are sorted into buckets by their type so they can be rendered
in consistent sections.

#### Fields

- **`modules`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Child modules (need ID for linking).

- **`structs`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Struct definitions (need ID for impl lookup).

- **`enums`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Enum definitions (need ID for impl lookup).

- **`traits`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Trait definitions (need ID for impl lookup).

- **`functions`**: `Vec<&'a rustdoc_types::Item>`

  Standalone functions.

- **`macros`**: `Vec<&'a rustdoc_types::Item>`

  Macro definitions.

- **`constants`**: `Vec<&'a rustdoc_types::Item>`

  Constants and statics.

- **`type_aliases`**: `Vec<&'a rustdoc_types::Item>`

  Type alias definitions.

#### Implementations

- `fn sort(self: &mut Self)`

#### Trait Implementations

##### `impl<'a> Default for CategorizedItems<'a>`

- `fn default() -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

##### `impl<T> Instrument for CategorizedItems<'a>`

##### `impl<T> IntoEither for CategorizedItems<'a>`

##### `impl<D> OwoColorize for CategorizedItems<'a>`

##### `impl<T> Pointable for CategorizedItems<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for CategorizedItems<'a>`

