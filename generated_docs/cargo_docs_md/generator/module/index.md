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

- <span id="modulerenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](../context/index.md#rendercontext)

  Create a new module renderer.

  

  # Arguments

  

  * `ctx` - Render context (implements `RenderContext` trait)

  * `current_file` - Path of this file (for relative link calculation)

  * `is_root` - True if this is the crate root module

- <span id="modulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

  Process documentation string to resolve intra-doc links.

  

  Delegates to the render context's `process_docs` method, which handles

  both single-crate and multi-crate link resolution.

- <span id="modulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

  Generate the complete markdown content for a module.

  

  # Output Structure

  

  ```markdown

  Crate `name` (or Module `name`)

  

  [module documentation]

  

  ## Contents (if items exceed threshold)

  - [Structs](#structs)

    - [`Parser`](#parser)

  

  ## Modules

  - [submodule](link) - first line of docs

  

  ## Structs

  ### `StructName`

  [struct definition and docs]

  

  ## Enums

  ...

  ```

- <span id="modulerenderer-categorize-items"></span>`fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

  Categorize module items by type for organized rendering.

  

  Items are categorized into groups for structured documentation.

  - Modules (for navigation)

  - Types (structs, enums, unions, type aliases)

  - Traits

  - Functions

  - Constants and statics

  - Macros

- <span id="modulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport(&self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](#categorizeditems)

  Expand a glob re-export by adding all public items from the target module.

- <span id="modulerenderer-render-all-sections"></span>`fn render_all_sections(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

  Render all item sections with horizontal rule separators.

  

  Sections are rendered in this order:

  1. Modules (navigation, no separator before)

  2. Types (structs, enums, unions, type aliases)

  3. Traits

  4. Functions

  5. Constants

  6. Statics

  7. Macros

  

  Horizontal rules (`---`) are added between major sections for

  visual separation in the rendered output.

- <span id="modulerenderer-render-types-section"></span>`fn render_types_section(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](#categorizeditems)

  Render the Types section (structs, enums, unions, type aliases).

  

  All type definitions are grouped under a single "Types" heading,

  with each item type rendered in subsections:

  

  ```markdown

  ## Types

  

  ### `MyStruct`

  [struct definition]

  

  ### `MyEnum`

  [enum definition]

  

  ### `MyUnion`

  [union definition]

  

  ### `MyAlias`

  [type alias definition]

  ```

- <span id="modulerenderer-render-statics-section"></span>`fn render_statics_section(&self, md: &mut String, statics: &[&Item])`

  Render the Statics section.

- <span id="modulerenderer-build-toc-entries"></span>`fn build_toc_entries(items: &CategorizedItems<'_>) -> Vec<TocEntry>` — [`CategorizedItems`](#categorizeditems), [`TocEntry`](../toc/index.md#tocentry)

  Build TOC entries from categorized items.

  

  Creates a hierarchical structure for the table of contents:

  - Modules section

  - Types section (with children: structs, enums, unions, type aliases)

  - Traits section

  - Functions section

  - Constants section

  - Statics section

  - Macros section

- <span id="modulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self, items: &CategorizedItems<'_>) -> Vec<QuickRefEntry>` — [`CategorizedItems`](#categorizeditems), [`QuickRefEntry`](../quick_ref/index.md#quickrefentry)

  Build quick reference entries from categorized items.

  

  Creates a flat list of entries for the quick reference table,

  including all item types with their names, kinds, and summaries.

  For re-exports, uses the target item's docs when the re-export lacks its own.

- <span id="modulerenderer-get-item-summary"></span>`fn get_item_summary(&self, item: &Item, item_id: Id) -> String`

  Get summary for an item, with fallback for re-exports.

  

  For re-exports (`ItemEnum::Use`), if the item has no docs, falls back

  to the target item's documentation.

- <span id="modulerenderer-render-modules-section"></span>`fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)])`

  Render the Modules section with links to submodules.

- <span id="modulerenderer-get-module-summary"></span>`fn get_module_summary(&self, item: &Item, item_id: Id) -> String`

  Get summary for a module, with fallback for re-exports.

- <span id="modulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

  Render the Traits section.

- <span id="modulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

  Render the Functions section.

- <span id="modulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

  Render the Macros section.

- <span id="modulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

  Render the Constants section.

#### Trait Implementations

##### `impl Any for ModuleRenderer<'a>`

- <span id="modulerenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ModuleRenderer<'a>`

- <span id="modulerenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ModuleRenderer<'a>`

- <span id="modulerenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ModuleRenderer<'a>`

- <span id="modulerenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ModuleRenderer<'a>`

##### `impl<U> Into for ModuleRenderer<'a>`

- <span id="modulerenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ModuleRenderer<'a>`

##### `impl OwoColorize for ModuleRenderer<'a>`

##### `impl Pointable for ModuleRenderer<'a>`

- <span id="modulerenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="modulerenderer-pointable-type-init"></span>`type Init = T`

- <span id="modulerenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="modulerenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="modulerenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="modulerenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ModuleRenderer<'a>`

- <span id="modulerenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="modulerenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ModuleRenderer<'a>`

- <span id="modulerenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="modulerenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in `src/generator/module.rs:837-871`*

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

  Check if the Types section has any items.

  

  Returns true if there are any structs, enums, unions, or type aliases.

- <span id="categorizeditems-sort"></span>`fn sort(&mut self)`

  Sort all item categories alphabetically by name for deterministic output.

  

  This ensures consistent ordering regardless of `HashMap` iteration order

  in the rustdoc JSON index.

#### Trait Implementations

##### `impl Any for CategorizedItems<'a>`

- <span id="categorizeditems-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CategorizedItems<'a>`

- <span id="categorizeditems-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CategorizedItems<'a>`

- <span id="categorizeditems-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for CategorizedItems<'a>`

- <span id="categorizeditems-default"></span>`fn default() -> CategorizedItems<'a>` — [`CategorizedItems`](#categorizeditems)

##### `impl<T> From for CategorizedItems<'a>`

- <span id="categorizeditems-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CategorizedItems<'a>`

##### `impl<U> Into for CategorizedItems<'a>`

- <span id="categorizeditems-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CategorizedItems<'a>`

##### `impl OwoColorize for CategorizedItems<'a>`

##### `impl Pointable for CategorizedItems<'a>`

- <span id="categorizeditems-pointable-const-align"></span>`const ALIGN: usize`

- <span id="categorizeditems-pointable-type-init"></span>`type Init = T`

- <span id="categorizeditems-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizeditems-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizeditems-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizeditems-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CategorizedItems<'a>`

- <span id="categorizeditems-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="categorizeditems-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CategorizedItems<'a>`

- <span id="categorizeditems-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="categorizeditems-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CategorizedItems<'a>`

