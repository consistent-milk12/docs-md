*[cargo_docs_md](../../index.md) / [generator](../index.md) / [module](index.md)*

---

# Module `module`

Module markdown rendering for documentation generation.

This module provides the [`ModuleRenderer`](#modulerenderer) struct which handles rendering
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

*Defined in `src/generator/module.rs:28-37`*

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

- <span id="modulerenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](../context/index.md)

- <span id="modulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="modulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

- <span id="modulerenderer-categorize-items"></span>`fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport(&self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-render-all-sections"></span>`fn render_all_sections(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-render-types-section"></span>`fn render_types_section(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

- <span id="modulerenderer-render-statics-section"></span>`fn render_statics_section(&self, md: &mut String, statics: &[&Item])`

- <span id="modulerenderer-build-toc-entries"></span>`fn build_toc_entries(items: &CategorizedItems<'_>) -> Vec<TocEntry>` — [`CategorizedItems`](#categorizeditems), [`TocEntry`](../toc/index.md)

- <span id="modulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self, items: &CategorizedItems<'_>) -> Vec<QuickRefEntry>` — [`CategorizedItems`](#categorizeditems), [`QuickRefEntry`](../quick_ref/index.md)

- <span id="modulerenderer-get-item-summary"></span>`fn get_item_summary(&self, item: &Item, item_id: Id) -> String`

- <span id="modulerenderer-render-modules-section"></span>`fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)])`

- <span id="modulerenderer-get-module-summary"></span>`fn get_module_summary(&self, item: &Item, item_id: Id) -> String`

- <span id="modulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

- <span id="modulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

- <span id="modulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

- <span id="modulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

#### Trait Implementations

##### `impl Instrument for ModuleRenderer<'a>`

##### `impl IntoEither for ModuleRenderer<'a>`

##### `impl OwoColorize for ModuleRenderer<'a>`

##### `impl Pointable for ModuleRenderer<'a>`

- <span id="modulerenderer-const-align"></span>`const ALIGN: usize`

- <span id="modulerenderer-type-init"></span>`type Init = T`

- <span id="modulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="modulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="modulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="modulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ModuleRenderer<'a>`

### `CategorizedItems<'a>`

```rust
struct CategorizedItems<'a> {
    modules: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    structs: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    enums: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    unions: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    type_aliases: Vec<&'a rustdoc_types::Item>,
    traits: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    functions: Vec<&'a rustdoc_types::Item>,
    constants: Vec<&'a rustdoc_types::Item>,
    statics: Vec<&'a rustdoc_types::Item>,
    macros: Vec<&'a rustdoc_types::Item>,
}
```

*Defined in `src/generator/module.rs:829-863`*

Items categorized by type for organized rendering.

Items are sorted into buckets by their type so they can be rendered
in consistent sections. The structure groups related items:

- **Types**: Structs, enums, unions, and type aliases
- **Traits**: Trait definitions
- **Functions**: Standalone functions
- **Constants**: Constants and statics
- **Macros**: Macro definitions

This organization improves navigation by grouping related items together.

#### Fields

- **`modules`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Child modules (need ID for linking).
  Rendered first for navigation purposes.

- **`structs`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Struct definitions (need ID for impl lookup).

- **`enums`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Enum definitions (need ID for impl lookup).

- **`unions`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Union definitions (need ID for impl lookup).

- **`type_aliases`**: `Vec<&'a rustdoc_types::Item>`

  Type alias definitions.

- **`traits`**: `Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>`

  Trait definitions (need ID for impl lookup).

- **`functions`**: `Vec<&'a rustdoc_types::Item>`

  Standalone functions.

- **`constants`**: `Vec<&'a rustdoc_types::Item>`

  Constants.

- **`statics`**: `Vec<&'a rustdoc_types::Item>`

  Static variables.

- **`macros`**: `Vec<&'a rustdoc_types::Item>`

  Macro definitions.

#### Implementations

- <span id="categorizeditems-has-types"></span>`const fn has_types(&self) -> bool`

- <span id="categorizeditems-sort"></span>`fn sort(&mut self)`

#### Trait Implementations

##### `impl Default for CategorizedItems<'a>`

- <span id="categorizeditems-default"></span>`fn default() -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

##### `impl Instrument for CategorizedItems<'a>`

##### `impl IntoEither for CategorizedItems<'a>`

##### `impl OwoColorize for CategorizedItems<'a>`

##### `impl Pointable for CategorizedItems<'a>`

- <span id="categorizeditems-const-align"></span>`const ALIGN: usize`

- <span id="categorizeditems-type-init"></span>`type Init = T`

- <span id="categorizeditems-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizeditems-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizeditems-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizeditems-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CategorizedItems<'a>`

