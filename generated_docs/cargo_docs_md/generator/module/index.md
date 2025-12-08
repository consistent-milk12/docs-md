*[cargo_docs_md](../../index.md) / [generator](../index.md) / [module](index.md)*

---

# Module `module`

Module markdown rendering for documentation generation.

This module provides the [`ModuleRenderer`](../index.md) struct which handles rendering
a Rust module's documentation to markdown format, including all its items
organized by type.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ModuleRenderer`](#modulerenderer) | struct | Renders a module to markdown. |
| [`CategorizedItems`](#categorizeditems) | struct | Items categorized by type for organized rendering. |

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

- <span id="modulerenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](../index.md)

- <span id="modulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="modulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

- <span id="modulerenderer-categorize-items"></span>`fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport(&self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-render-all-sections"></span>`fn render_all_sections(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-build-toc-entries"></span>`fn build_toc_entries(&self, items: &CategorizedItems<'_>) -> Vec<TocEntry>` — [`CategorizedItems`](#categorizeditems), [`TocEntry`](../index.md)

- <span id="modulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self, items: &CategorizedItems<'_>) -> Vec<QuickRefEntry>` — [`CategorizedItems`](#categorizeditems), [`QuickRefEntry`](../index.md)

- <span id="modulerenderer-render-modules-section"></span>`fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)])`

- <span id="modulerenderer-render-structs-section"></span>`fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)])`

- <span id="modulerenderer-render-enums-section"></span>`fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)])`

- <span id="modulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

- <span id="modulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

- <span id="modulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

- <span id="modulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

- <span id="modulerenderer-render-type-aliases-section"></span>`fn render_type_aliases_section(&self, md: &mut String, type_aliases: &[&Item])`

#### Trait Implementations

##### `impl<T> Instrument for ModuleRenderer<'a>`

##### `impl<T> IntoEither for ModuleRenderer<'a>`

##### `impl<D> OwoColorize for ModuleRenderer<'a>`

##### `impl<T> Pointable for ModuleRenderer<'a>`

- <span id="modulerenderer-align"></span>`const ALIGN: usize`

- <span id="modulerenderer-init"></span>`type Init = T`

- <span id="modulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="modulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="modulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="modulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="categorizeditems-sort"></span>`fn sort(&mut self)`

#### Trait Implementations

##### `impl<'a> Default for CategorizedItems<'a>`

- <span id="categorizeditems-default"></span>`fn default() -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

##### `impl<T> Instrument for CategorizedItems<'a>`

##### `impl<T> IntoEither for CategorizedItems<'a>`

##### `impl<D> OwoColorize for CategorizedItems<'a>`

##### `impl<T> Pointable for CategorizedItems<'a>`

- <span id="categorizeditems-align"></span>`const ALIGN: usize`

- <span id="categorizeditems-init"></span>`type Init = T`

- <span id="categorizeditems-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizeditems-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizeditems-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizeditems-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for CategorizedItems<'a>`

