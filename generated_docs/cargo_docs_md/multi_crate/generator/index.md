*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [generator](index.md)*

---

# Module `generator`

Multi-crate documentation generator.

This module provides [`MultiCrateGenerator`](#multicrategenerator) which orchestrates
documentation generation across multiple crates with cross-crate linking.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CategorizedItems`](#categorizeditems) | struct | Categorized module items for rendering. |
| [`MultiCrateGenerator`](#multicrategenerator) | struct | Generator for multi-crate documentation. |
| [`MultiCrateModuleRenderer`](#multicratemodulerenderer) | struct | Module renderer for multi-crate context. |

## Structs

### `CategorizedItems<'a>`

```rust
struct CategorizedItems<'a> {
    modules: Vec<&'a rustdoc_types::Item>,
    structs: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    enums: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    unions: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    traits: Vec<(&'a rustdoc_types::Id, &'a rustdoc_types::Item)>,
    functions: Vec<&'a rustdoc_types::Item>,
    types: Vec<&'a rustdoc_types::Item>,
    constants: Vec<&'a rustdoc_types::Item>,
    statics: Vec<&'a rustdoc_types::Item>,
    macros: Vec<&'a rustdoc_types::Item>,
}
```

*Defined in `src/multi_crate/generator.rs:46-57`*

Categorized module items for rendering.

Collects items by category during module traversal, eliminating the need
for 8 separate vector parameters in TOC/QuickRef generation.

Note: This is currently unused - kept for potential future multi-crate
rendering improvements.

#### Implementations

- <span id="categorizeditems-new"></span>`const fn new() -> Self`

  Create empty categorized items collection.

- <span id="categorizeditems-is-empty"></span>`const fn is_empty(&self) -> bool`

  Check if all categories are empty.

- <span id="categorizeditems-add-item"></span>`fn add_item(&mut self, id: &'a Id, item: &'a Item)`

  Add an item to the appropriate category based on its type.

- <span id="categorizeditems-add-reexport"></span>`fn add_reexport(&mut self, id: &'a Id, use_item: &'a Item, target: &Item)`

  Add an item by category based on target item type (for re-exports).

  

  The `id` is the Use item's ID, and `target` is the resolved target item

  used to determine the category. The `use_item` is stored (not target)

  because `get_item_name` handles Use items specially.

- <span id="categorizeditems-build-toc-entries"></span>`fn build_toc_entries(&self) -> Vec<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

  Build TOC entries from categorized items.

  

  Preserves the standard rustdoc section order:

  Modules → Structs → Enums → Unions → Traits → Functions → Type Aliases → Constants → Statics → Macros

- <span id="categorizeditems-build-section"></span>`fn build_section(items: &[&Item], section: &str, anchor: &str, is_macro: bool) -> Option<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

  Build a TOC section for items without IDs.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="categorizeditems-build-section-with-ids"></span>`fn build_section_with_ids(items: &[(&Id, &Item)], section: &str, anchor: &str) -> Option<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

  Build a TOC section for items with IDs.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="categorizeditems-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self) -> Vec<QuickRefEntry>` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

  Build Quick Reference entries from categorized items.

  

  Preserves the standard rustdoc section order:

  Modules → Structs → Enums → Unions → Traits → Functions → Type Aliases → Constants → Statics → Macros

- <span id="categorizeditems-add-quick-ref-entries"></span>`fn add_quick_ref_entries(entries: &mut Vec<QuickRefEntry>, items: &[&Item], kind: &'static str, is_macro: bool)` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

  Add quick ref entries for items without IDs.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="categorizeditems-add-quick-ref-entries-with-ids"></span>`fn add_quick_ref_entries_with_ids(entries: &mut Vec<QuickRefEntry>, items: &[(&Id, &Item)], kind: &'static str)` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

  Add quick ref entries for items with IDs.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="categorizeditems-get-item-name"></span>`fn get_item_name(item: &Item) -> &str`

  Get the display name for an item, handling re-exports.

- <span id="categorizeditems-expand-glob-reexport"></span>`fn expand_glob_reexport(&mut self, use_item: &rustdoc_types::Use, krate: &'a Crate, view: &SingleCrateView<'_>, seen_items: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

  Expand a glob re-export into this collection.

  

  Iterates through items in the target module and adds them to the

  appropriate category vectors. Uses `seen_items` to avoid duplicates.

  

  # Arguments

  

  * `use_item` - The glob Use item to expand

  * `krate` - The crate containing the target module

  * `view` - The single-crate view for visibility filtering

  * `seen_items` - Set of already-processed item IDs (mutated)

#### Trait Implementations

##### `impl Any for CategorizedItems<'a>`

- <span id="categorizeditems-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CategorizedItems<'a>`

- <span id="categorizeditems-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CategorizedItems<'a>`

- <span id="categorizeditems-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

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

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

*Defined in `src/multi_crate/generator.rs:397-403`*

Generator for multi-crate documentation.

Produces a directory structure with one subdirectory per crate,
each containing nested markdown files with cross-crate linking.

# Output Structure

```text
output/
├── tracing/
│   ├── index.md
│   └── span/
│       └── index.md
├── tracing_core/
│   ├── index.md
│   └── subscriber/
│       └── index.md
└── SUMMARY.md        # If --mdbook enabled
```

#### Fields

- **`ctx`**: `crate::multi_crate::MultiCrateContext<'a>`

  Multi-crate context with unified registry.

- **`args`**: `&'a crate::Args`

  CLI arguments.

#### Implementations

- <span id="multicrategenerator-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](../collection/index.md#cratecollection), [`Args`](../../index.md#args), [`RenderConfig`](../../generator/config/index.md#renderconfig)

  Create a new multi-crate generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `args` - CLI arguments

  * `config` - Rendering configuration options

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate documentation for all crates.

  

  Creates the output directory structure, generates docs for each crate

  in parallel using rayon, and optionally generates SUMMARY.md for

  mdBook compatibility.

  

  # Errors

  

  Returns an error if any file operation fails.

- <span id="multicrategenerator-collect-rendered-items"></span>`fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>>`

  Collect the IDs of all items that would be rendered.

  

  This walks the module tree for each crate using the same visibility

  rules as rendering, collecting the IDs of items that will have

  documentation generated for them.

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

  Collect rendered item IDs for a single crate.

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

  Recursively collect rendered item IDs from a module.

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../context/index.md#singlecrateview), [`Error`](../../error/index.md#error)

  Generate documentation for a single crate.

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../context/index.md#singlecrateview), [`Error`](../../error/index.md#error)

  Generate a module directory with index.md and child modules.

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../../error/index.md#error)

  Create a progress bar.

  

  # Errors

  

  Returns an error if the progress bar template is invalid.

#### Trait Implementations

##### `impl Any for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateGenerator<'a>`

##### `impl<U> Into for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateGenerator<'a>`

##### `impl OwoColorize for MultiCrateGenerator<'a>`

##### `impl Pointable for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrategenerator-pointable-type-init"></span>`type Init = T`

- <span id="multicrategenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrategenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrategenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrategenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicrategenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicrategenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateGenerator<'a>`

### `MultiCrateModuleRenderer<'a>`

```rust
struct MultiCrateModuleRenderer<'a> {
    view: &'a crate::multi_crate::context::SingleCrateView<'a>,
    file_path: &'a str,
    is_root: bool,
    type_renderer: crate::types::TypeRenderer<'a>,
}
```

*Defined in `src/multi_crate/generator.rs:723-735`*

Module renderer for multi-crate context.

Wraps the standard module rendering with multi-crate link resolution.

This renderer handles special cases that aren't covered by the standard
`ModuleRenderer`, particularly re-exports (`pub use`) which need to
resolve items across crate boundaries.

#### Fields

- **`view`**: `&'a crate::multi_crate::context::SingleCrateView<'a>`

  Single-crate view for this crate (implements `RenderContext`).

- **`file_path`**: `&'a str`

  Current file path for link resolution.

- **`is_root`**: `bool`

  Whether this is the crate root.

- **`type_renderer`**: `crate::types::TypeRenderer<'a>`

  Cached type renderer to avoid repeated construction.

#### Implementations

- <span id="multicratemodulerenderer-new"></span>`const fn new(view: &'a SingleCrateView<'a>, file_path: &'a str, is_root: bool) -> Self` — [`SingleCrateView`](../context/index.md#singlecrateview)

  Create a new multi-crate module renderer.

- <span id="multicratemodulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

  Process documentation for an item.

  

  Delegates to the view's `process_docs` method which handles:

  - Stripping duplicate titles

  - Converting doc links to markdown links

  - Processing code blocks

- <span id="multicratemodulerenderer-maybe-render-source-location"></span>`fn maybe_render_source_location(&self, item: &Item) -> String`

  Render source location if enabled in config.

  

  Returns the source location string if `source_locations` is enabled,

  otherwise returns an empty string. Uses the source path config to

  generate clickable links to the `.source_*` directory when available.

- <span id="multicratemodulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

  Render a module item to markdown.

- <span id="multicratemodulerenderer-render-module-contents"></span>`fn render_module_contents(&self, md: &mut String, module: &rustdoc_types::Module, _parent: &Item)`

  Render module contents (items, types, functions, etc.).

- <span id="multicratemodulerenderer-render-modules-section"></span>`fn render_modules_section(md: &mut String, modules: &[&Item], krate: &Crate)`

  Render modules section (links to subdirectories).

- <span id="multicratemodulerenderer-render-structs-section"></span>`fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)])`

  Render structs section with full detail.

- <span id="multicratemodulerenderer-render-enums-section"></span>`fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)])`

  Render enums section with full detail.

- <span id="multicratemodulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

  Render traits section with full detail.

- <span id="multicratemodulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

  Render functions section with full detail.

- <span id="multicratemodulerenderer-render-type-aliases-section"></span>`fn render_type_aliases_section(&self, md: &mut String, types: &[&Item])`

  Render type aliases section with full detail.

- <span id="multicratemodulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

  Render constants section with full detail.

- <span id="multicratemodulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

  Render macros section with full detail.

- <span id="multicratemodulerenderer-get-item-name-and-summary"></span>`fn get_item_name_and_summary(item: &Item) -> (String, String)`

  Get name and summary for an item, handling re-exports.

  

  For re-exports (Use items), if the Use item has no docs, falls back to

  the target item's docs when a crate reference is provided.

- <span id="multicratemodulerenderer-get-item-name-and-summary-with-fallback"></span>`fn get_item_name_and_summary_with_fallback(item: &Item, krate: Option<&Crate>) -> (String, String)`

  Get name and summary for an item with optional fallback lookup.

  

  When `krate` is provided and the item is a re-export without docs,

  looks up the target item's docs as a fallback.

- <span id="multicratemodulerenderer-get-item-name"></span>`fn get_item_name(item: &Item) -> &str`

  Get the display name for an item, handling re-exports.

  

  For `Use` items (re-exports), the name is stored in `use_item.name`.

  For all other items, the name is in `item.name`.

- <span id="multicratemodulerenderer-build-toc-entries"></span>`fn build_toc_entries(modules: &[&Item], structs: &[(&Id, &Item)], enums: &[(&Id, &Item)], traits: &[(&Id, &Item)], functions: &[&Item], types: &[&Item], constants: &[&Item], macros: &[&Item]) -> Vec<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

  Build TOC entries from categorized module items.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="multicratemodulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(modules: &[&Item], structs: &[(&Id, &Item)], enums: &[(&Id, &Item)], traits: &[(&Id, &Item)], functions: &[&Item], types: &[&Item], constants: &[&Item], macros: &[&Item]) -> Vec<QuickRefEntry>` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

  Build Quick Reference entries from categorized module items.

  

  Uses `slugify_anchor()` for anchor generation to match heading anchors.

- <span id="multicratemodulerenderer-render-struct"></span>`fn render_struct(&self, md: &mut String, item_id: Id, item: &Item)`

  Render a struct definition to markdown.

- <span id="multicratemodulerenderer-render-enum"></span>`fn render_enum(&self, md: &mut String, item_id: Id, item: &Item)`

  Render an enum definition to markdown.

- <span id="multicratemodulerenderer-render-trait"></span>`fn render_trait(&self, md: &mut String, item_id: Id, item: &Item)`

  Render a trait definition to markdown.

- <span id="multicratemodulerenderer-render-trait-implementors"></span>`fn render_trait_implementors(&self, md: &mut String, trait_id: Id)`

  Render the implementors section for a trait.

  

  Uses `Trait.implementations` field for direct lookup instead of scanning

  all items in the crate index, providing O(k) performance where k is the

  number of implementors.

- <span id="multicratemodulerenderer-render-function"></span>`fn render_function(&self, md: &mut String, item: &Item)`

  Render a function definition to markdown.

  FIX: Handle re-exports: Resolve to actual function item.

- <span id="multicratemodulerenderer-render-constant"></span>`fn render_constant(&self, md: &mut String, item: &Item)`

  Render a constant definition to markdown.

  Handles re-exports by resolving to the actual constant item.

- <span id="multicratemodulerenderer-render-type-alias"></span>`fn render_type_alias(&self, md: &mut String, item: &Item)`

  Render a type alias to markdown.

  Handles re-exports by resolving to the actual type alias item.

- <span id="multicratemodulerenderer-render-macro"></span>`fn render_macro(&self, md: &mut String, item: &Item)`

  Render a macro to markdown.

  Handles re-exports by resolving to the actual macro item.

- <span id="multicratemodulerenderer-resolve-reexport"></span>`fn resolve_reexport<'b>(self: &'b Self, item: &'b Item) -> Option<(&'b str, &'b Item)>`

- <span id="multicratemodulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport<'b>(&self, modules: &mut Vec<&'b Item>, structs: &mut Vec<(&'b Id, &'b Item)>, enums: &mut Vec<(&'b Id, &'b Item)>, traits: &mut Vec<(&'b Id, &'b Item)>, functions: &mut Vec<&'b Item>, types: &mut Vec<&'b Item>, constants: &mut Vec<&'b Item>, macros: &mut Vec<&'b Item>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<Id>)`

  Expand a glob re-export into the category vectors.

- <span id="multicratemodulerenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id, source_crate_name: Option<&str>)`

  Render impl blocks for a type, including cross-crate impls.

  

  # Arguments

  

  * `md` - The markdown output buffer

  * `item_id` - The ID of the item to render impl blocks for

  * `source_crate_name` - Optional source crate name for cross-crate re-exports.

    When provided, impls are looked up from the source crate.

#### Trait Implementations

##### `impl Any for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateModuleRenderer<'a>`

##### `impl<U> Into for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateModuleRenderer<'a>`

##### `impl OwoColorize for MultiCrateModuleRenderer<'a>`

##### `impl Pointable for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicratemodulerenderer-pointable-type-init"></span>`type Init = T`

- <span id="multicratemodulerenderer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratemodulerenderer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratemodulerenderer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratemodulerenderer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicratemodulerenderer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicratemodulerenderer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateModuleRenderer<'a>`

