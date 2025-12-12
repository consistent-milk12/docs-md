*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [generator](index.md)*

---

# Module `generator`

Multi-crate documentation generator.

This module provides [`MultiCrateGenerator`](#multicrategenerator) which orchestrates
documentation generation across multiple crates with cross-crate linking.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ResolvedItem`](#resolveditem) | struct | Result of resolving a potential re-export to its actual item. |
| [`CategorizedItems`](#categorizeditems) | struct | Categorized module items for rendering. |
| [`MultiCrateGenerator`](#multicrategenerator) | struct | Generator for multi-crate documentation. |
| [`MultiCrateModuleRenderer`](#multicratemodulerenderer) | struct | Module renderer for multi-crate context. |

## Structs

### `ResolvedItem<'a>`

```rust
struct ResolvedItem<'a> {
    name: &'a str,
    item: &'a rustdoc_types::Item,
    id: rustdoc_types::Id,
    source_crate: Option<&'a str>,
}
```

*Defined in `src/multi_crate/generator.rs:50-62`*

Result of resolving a potential re-export to its actual item.

This struct captures all context needed for rendering, eliminiating the need
for duplicated resolution logic in each render method. For local items,
`source_crate` is `None`. For cross-crate re-exports, it contains the
source crate name for proper type rendering and impl lookup.

# Note on `Copy`

This struct derives `Copy` because `Id` is `Copy` (newtype over `u32`),
and all other fields are references or `Option` of references.

#### Fields

- **`name`**: `&'a str`

  Display name (from `Use.name` for re-exports, `Item.name` otherwise)

- **`item`**: `&'a rustdoc_types::Item`

  The actual resolved item (target of re-export, or original item)

- **`id`**: `rustdoc_types::Id`

  ID of the resolved item - always the actual item's ID, never a dummy

- **`source_crate`**: `Option<&'a str>`

  Source crate name for cross-crate re-exports (`None` for local items)

#### Trait Implementations

##### `impl Clone for ResolvedItem<'a>`

- <span id="resolveditem-clone"></span>`fn clone(&self) -> ResolvedItem<'a>` — [`ResolvedItem`](#resolveditem)

##### `impl Copy for ResolvedItem<'a>`

##### `impl Debug for ResolvedItem<'a>`

- <span id="resolveditem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for ResolvedItem<'a>`

##### `impl IntoEither for ResolvedItem<'a>`

##### `impl OwoColorize for ResolvedItem<'a>`

##### `impl Pointable for ResolvedItem<'a>`

- <span id="resolveditem-pointable-const-align"></span>`const ALIGN: usize`

- <span id="resolveditem-pointable-type-init"></span>`type Init = T`

- <span id="resolveditem-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="resolveditem-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="resolveditem-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="resolveditem-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ResolvedItem<'a>`

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

*Defined in `src/multi_crate/generator.rs:68-79`*

Categorized module items for rendering.

Collects items by category during module traversal, eliminating the need
for 8 separate vector parameters in TOC/QuickRef generation.

#### Implementations

- <span id="categorizeditems-new"></span>`const fn new() -> Self`

- <span id="categorizeditems-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="categorizeditems-add-item"></span>`fn add_item(&mut self, id: &'a Id, item: &'a Item)`

- <span id="categorizeditems-add-reexport"></span>`fn add_reexport(&mut self, id: &'a Id, use_item: &'a Item, target: &Item)`

- <span id="categorizeditems-build-toc-entries"></span>`fn build_toc_entries(&self) -> Vec<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

- <span id="categorizeditems-build-section"></span>`fn build_section(items: &[&Item], section: &str, anchor: &str, is_macro: bool) -> Option<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

- <span id="categorizeditems-build-section-with-ids"></span>`fn build_section_with_ids(items: &[(&Id, &Item)], section: &str, anchor: &str) -> Option<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

- <span id="categorizeditems-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self) -> Vec<QuickRefEntry>` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

- <span id="categorizeditems-add-quick-ref-entries"></span>`fn add_quick_ref_entries(entries: &mut Vec<QuickRefEntry>, items: &[&Item], kind: &'static str, is_macro: bool)` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

- <span id="categorizeditems-add-quick-ref-entries-with-ids"></span>`fn add_quick_ref_entries_with_ids(entries: &mut Vec<QuickRefEntry>, items: &[(&Id, &Item)], kind: &'static str)` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

- <span id="categorizeditems-get-item-name"></span>`fn get_item_name(item: &Item) -> &str`

- <span id="categorizeditems-expand-glob-reexport"></span>`fn expand_glob_reexport(&mut self, use_item: &rustdoc_types::Use, krate: &'a Crate, view: &SingleCrateView<'_>, seen_items: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

#### Trait Implementations

##### `impl Instrument for CategorizedItems<'a>`

##### `impl IntoEither for CategorizedItems<'a>`

##### `impl OwoColorize for CategorizedItems<'a>`

##### `impl Pointable for CategorizedItems<'a>`

- <span id="categorizeditems-pointable-const-align"></span>`const ALIGN: usize`

- <span id="categorizeditems-pointable-type-init"></span>`type Init = T`

- <span id="categorizeditems-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizeditems-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizeditems-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizeditems-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CategorizedItems<'a>`

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

*Defined in `src/multi_crate/generator.rs:418-424`*

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

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="multicrategenerator-collect-rendered-items"></span>`fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>>`

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](../context/index.md#singlecrateview)

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../context/index.md#singlecrateview), [`Error`](../../error/index.md#error)

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../context/index.md#singlecrateview), [`Error`](../../error/index.md#error)

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../../error/index.md#error)

#### Trait Implementations

##### `impl Instrument for MultiCrateGenerator<'a>`

##### `impl IntoEither for MultiCrateGenerator<'a>`

##### `impl OwoColorize for MultiCrateGenerator<'a>`

##### `impl Pointable for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrategenerator-pointable-type-init"></span>`type Init = T`

- <span id="multicrategenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrategenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrategenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrategenerator-drop"></span>`unsafe fn drop(ptr: usize)`

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

*Defined in `src/multi_crate/generator.rs:744-756`*

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

- <span id="multicratemodulerenderer-maybe-render-source-location"></span>`fn maybe_render_source_location(&self, item: &Item) -> String`

- <span id="multicratemodulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

- <span id="multicratemodulerenderer-render-module-contents"></span>`fn render_module_contents(&self, md: &mut String, module: &rustdoc_types::Module, _parent: &Item)`

- <span id="multicratemodulerenderer-render-modules-section"></span>`fn render_modules_section(md: &mut String, modules: &[&Item], krate: &Crate)`

- <span id="multicratemodulerenderer-render-structs-section"></span>`fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)])`

- <span id="multicratemodulerenderer-render-enums-section"></span>`fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)])`

- <span id="multicratemodulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

- <span id="multicratemodulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

- <span id="multicratemodulerenderer-render-type-aliases-section"></span>`fn render_type_aliases_section(&self, md: &mut String, types: &[&Item])`

- <span id="multicratemodulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

- <span id="multicratemodulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

- <span id="multicratemodulerenderer-get-item-name-and-summary"></span>`fn get_item_name_and_summary(item: &Item) -> (String, String)`

- <span id="multicratemodulerenderer-get-item-name-and-summary-with-fallback"></span>`fn get_item_name_and_summary_with_fallback(item: &Item, krate: Option<&Crate>) -> (String, String)`

- <span id="multicratemodulerenderer-get-item-name"></span>`fn get_item_name(item: &Item) -> &str`

- <span id="multicratemodulerenderer-build-toc-entries"></span>`fn build_toc_entries(modules: &[&Item], structs: &[(&Id, &Item)], enums: &[(&Id, &Item)], traits: &[(&Id, &Item)], functions: &[&Item], types: &[&Item], constants: &[&Item], macros: &[&Item]) -> Vec<TocEntry>` — [`TocEntry`](../../generator/toc/index.md#tocentry)

- <span id="multicratemodulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(modules: &[&Item], structs: &[(&Id, &Item)], enums: &[(&Id, &Item)], traits: &[(&Id, &Item)], functions: &[&Item], types: &[&Item], constants: &[&Item], macros: &[&Item]) -> Vec<QuickRefEntry>` — [`QuickRefEntry`](../../generator/quick_ref/index.md#quickrefentry)

- <span id="multicratemodulerenderer-render-struct"></span>`fn render_struct(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="multicratemodulerenderer-render-enum"></span>`fn render_enum(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="multicratemodulerenderer-render-trait"></span>`fn render_trait(&self, md: &mut String, item_id: Id, item: &Item)`

- <span id="multicratemodulerenderer-render-trait-implementors"></span>`fn render_trait_implementors(&self, md: &mut String, trait_id: Id)`

- <span id="multicratemodulerenderer-render-function"></span>`fn render_function(&self, md: &mut String, item: &Item)`

- <span id="multicratemodulerenderer-render-constant"></span>`fn render_constant(&self, md: &mut String, item: &Item)`

- <span id="multicratemodulerenderer-render-type-alias"></span>`fn render_type_alias(&self, md: &mut String, item: &Item)`

- <span id="multicratemodulerenderer-render-macro"></span>`fn render_macro(&self, md: &mut String, item: &Item)`

- <span id="multicratemodulerenderer-resolve-reexport"></span>`fn resolve_reexport<'b>(self: &'b Self, item: &'b Item) -> Option<(&'b str, &'b Item)>`

- <span id="multicratemodulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport<'b>(&self, modules: &mut Vec<&'b Item>, structs: &mut Vec<(&'b Id, &'b Item)>, enums: &mut Vec<(&'b Id, &'b Item)>, traits: &mut Vec<(&'b Id, &'b Item)>, functions: &mut Vec<&'b Item>, types: &mut Vec<&'b Item>, constants: &mut Vec<&'b Item>, macros: &mut Vec<&'b Item>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<Id>)`

- <span id="multicratemodulerenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id, source_crate_name: Option<&str>)`

#### Trait Implementations

##### `impl Instrument for MultiCrateModuleRenderer<'a>`

##### `impl IntoEither for MultiCrateModuleRenderer<'a>`

##### `impl OwoColorize for MultiCrateModuleRenderer<'a>`

##### `impl Pointable for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicratemodulerenderer-pointable-type-init"></span>`type Init = T`

- <span id="multicratemodulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratemodulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratemodulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratemodulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MultiCrateModuleRenderer<'a>`

