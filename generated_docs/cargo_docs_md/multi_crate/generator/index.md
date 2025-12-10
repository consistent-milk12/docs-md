*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [generator](index.md)*

---

# Module `generator`

Multi-crate documentation generator.

This module provides [`MultiCrateGenerator`](#multicrategenerator) which orchestrates
documentation generation across multiple crates with cross-crate linking.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiCrateGenerator`](#multicrategenerator) | struct | Generator for multi-crate documentation. |
| [`MultiCrateModuleRenderer`](#multicratemodulerenderer) | struct | Module renderer for multi-crate context. |

## Structs

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

*Defined in `src/multi_crate/generator.rs:56-62`*

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

- <span id="multicrategenerator-const-align"></span>`const ALIGN: usize`

- <span id="multicrategenerator-type-init"></span>`type Init = T`

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

*Defined in `src/multi_crate/generator.rs:381-393`*

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

- <span id="multicratemodulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport<'b>(&self, modules: &mut Vec<&'b Item>, structs: &mut Vec<(&'b Id, &'b Item)>, enums: &mut Vec<(&'b Id, &'b Item)>, traits: &mut Vec<(&'b Id, &'b Item)>, functions: &mut Vec<&'b Item>, types: &mut Vec<&'b Item>, constants: &mut Vec<&'b Item>, macros: &mut Vec<&'b Item>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<Id>)`

- <span id="multicratemodulerenderer-render-impl-blocks"></span>`fn render_impl_blocks(&self, md: &mut String, item_id: Id, source_crate_name: Option<&str>)`

#### Trait Implementations

##### `impl Instrument for MultiCrateModuleRenderer<'a>`

##### `impl IntoEither for MultiCrateModuleRenderer<'a>`

##### `impl OwoColorize for MultiCrateModuleRenderer<'a>`

##### `impl Pointable for MultiCrateModuleRenderer<'a>`

- <span id="multicratemodulerenderer-const-align"></span>`const ALIGN: usize`

- <span id="multicratemodulerenderer-type-init"></span>`type Init = T`

- <span id="multicratemodulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratemodulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratemodulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratemodulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MultiCrateModuleRenderer<'a>`

