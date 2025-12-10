*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [context](index.md)*

---

# Module `context`

Multi-crate generation context.

This module provides [`MultiCrateContext`](#multicratecontext) which holds shared state
during multi-crate documentation generation, and [`SingleCrateView`](#singlecrateview)
which provides a single-crate interface for existing rendering code.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiCrateContext`](#multicratecontext) | struct | Shared context for multi-crate documentation generation. |
| [`SingleCrateView`](#singlecrateview) | struct | View of a single crate within multi-crate context. |

## Structs

### `MultiCrateContext<'a>`

```rust
struct MultiCrateContext<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    registry: crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    config: crate::generator::config::RenderConfig,
    cross_crate_impls: std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
    source_path_config: Option<crate::generator::render_shared::SourcePathConfig>,
}
```

*Defined in `src/multi_crate/context.rs:41-64`*

Shared context for multi-crate documentation generation.

Holds references to all crates, the unified link registry, and
CLI configuration. Used by [`MultiCrateGenerator`](../generator/index.md) to coordinate
generation across crates.


#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  All crates being documented.

- **`registry`**: `crate::multi_crate::UnifiedLinkRegistry`

  Unified link registry for cross-crate resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`config`**: `crate::generator::config::RenderConfig`

  Rendering configuration options.

- **`cross_crate_impls`**: `std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Pre-computed cross-crate impl blocks.
  
  Maps target crate name -> type name -> impl blocks from other crates.
  This is computed once during construction rather than per-view.

- **`source_path_config`**: `Option<crate::generator::render_shared::SourcePathConfig>`

  Base source path configuration for transforming cargo registry paths.
  
  `None` if source locations are disabled or no `.source_*` dir detected.

#### Implementations

- <span id="multicratecontext-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](../collection/index.md#cratecollection), [`Args`](../../index.md#args), [`RenderConfig`](../../generator/config/index.md#renderconfig)

- <span id="multicratecontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../../generator/render_shared/index.md#sourcepathconfig)

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](../collection/index.md#cratecollection)

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](../collection/index.md#cratecollection)

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../registry/index.md#unifiedlinkregistry)

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](../../index.md#args)

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](#singlecrateview)

- <span id="multicratecontext-find-item"></span>`fn find_item(&self, id: &Id) -> Option<(&str, &Item)>`

- <span id="multicratecontext-get-cross-crate-impls"></span>`fn get_cross_crate_impls(&self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

- <span id="multicratecontext-get-impl-target-path"></span>`fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

#### Trait Implementations

##### `impl Instrument for MultiCrateContext<'a>`

##### `impl IntoEither for MultiCrateContext<'a>`

##### `impl OwoColorize for MultiCrateContext<'a>`

##### `impl Pointable for MultiCrateContext<'a>`

- <span id="multicratecontext-const-align"></span>`const ALIGN: usize`

- <span id="multicratecontext-type-init"></span>`type Init = T`

- <span id="multicratecontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratecontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratecontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratecontext-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MultiCrateContext<'a>`

### `SingleCrateView<'a>`

```rust
struct SingleCrateView<'a> {
    crate_name: &'a str,
    krate: &'a rustdoc_types::Crate,
    registry: &'a crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    ctx: &'a MultiCrateContext<'a>,
    impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    cross_crate_impls: Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
    type_name_to_id: std::collections::HashMap<String, rustdoc_types::Id>,
}
```

*Defined in `src/multi_crate/context.rs:278-303`*

View of a single crate within multi-crate context.

Provides an interface similar to [`GeneratorContext`](../../generator/context/index.md) but uses
[`UnifiedLinkRegistry`](../registry/index.md) for cross-crate link resolution. This
allows existing rendering code to work with minimal changes.


#### Fields

- **`crate_name`**: `&'a str`

  Name of this crate (borrowed from the context).

- **`krate`**: `&'a rustdoc_types::Crate`

  The crate being rendered.

- **`registry`**: `&'a crate::multi_crate::UnifiedLinkRegistry`

  Unified registry for link resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`ctx`**: `&'a MultiCrateContext<'a>`

  Reference to the parent multi-crate context for cross-crate lookups.

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Map from type ID to impl blocks (local crate only).

- **`cross_crate_impls`**: `Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Reference to pre-computed cross-crate impl blocks from context.
  Maps type name to impl blocks from other crates.

- **`type_name_to_id`**: `std::collections::HashMap<String, rustdoc_types::Id>`

  Map from type name to type ID for cross-crate impl lookup.

#### Implementations

- <span id="singlecrateview-new"></span>`fn new(crate_name: &'a str, krate: &'a Crate, registry: &'a UnifiedLinkRegistry, args: &'a Args, ctx: &'a MultiCrateContext<'a>) -> Self` — [`UnifiedLinkRegistry`](../registry/index.md#unifiedlinkregistry), [`Args`](../../index.md#args), [`MultiCrateContext`](#multicratecontext)

- <span id="singlecrateview-build-impl-map"></span>`fn build_impl_map(&mut self)`

- <span id="singlecrateview-build-type-name-map"></span>`fn build_type_name_map(&mut self)`

- <span id="singlecrateview-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- <span id="singlecrateview-crate-name"></span>`const fn crate_name(&self) -> &str`

- <span id="singlecrateview-krate"></span>`const fn krate(&self) -> &Crate`

- <span id="singlecrateview-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../registry/index.md#unifiedlinkregistry)

- <span id="singlecrateview-args"></span>`const fn args(&self) -> &Args` — [`Args`](../../index.md#args)

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>>`

- <span id="singlecrateview-get-all-impls"></span>`fn get_all_impls(&self, id: Id) -> Vec<&'a Impl>`

- <span id="singlecrateview-get-impls-from-crate"></span>`fn get_impls_from_crate(&self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

- <span id="singlecrateview-get-impl-target-id-from-type"></span>`const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

- <span id="singlecrateview-should-include-item"></span>`const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool`

- <span id="singlecrateview-count-modules"></span>`fn count_modules(&self) -> usize`

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

- <span id="singlecrateview-resolve-name"></span>`fn resolve_name(&self, name: &str) -> Option<(String, Id)>`

- <span id="singlecrateview-lookup-item-across-crates"></span>`fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)>`

- <span id="singlecrateview-get-crate"></span>`fn get_crate(&self, name: &str) -> Option<&Crate>`

- <span id="singlecrateview-resolve-external-path"></span>`fn resolve_external_path(&self, path: &str) -> Option<(&str, &Item, Id)>`

- <span id="singlecrateview-process-backtick-links"></span>`fn process_backtick_links(&self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

- <span id="singlecrateview-process-plain-links"></span>`fn process_plain_links(&self, docs: &str, current_file: &str) -> String`

- <span id="singlecrateview-resolve-plain-link"></span>`fn resolve_plain_link(&self, link_text: &str, current_file: &str) -> Option<String>`

- <span id="singlecrateview-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

- <span id="singlecrateview-build-link-to-id"></span>`fn build_link_to_id(&self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

- <span id="singlecrateview-resolve-crate-path"></span>`fn resolve_crate_path(&self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

- <span id="singlecrateview-split-type-and-anchor"></span>`fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

- <span id="singlecrateview-build-markdown-link"></span>`fn build_markdown_link(&self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

- <span id="singlecrateview-compute-cross-crate-path"></span>`fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

- <span id="singlecrateview-strip-crate-prefix"></span>`fn strip_crate_prefix(path: &str) -> &str`

- <span id="singlecrateview-looks-like-external-reference"></span>`fn looks_like_external_reference(link_text: &str) -> bool`

#### Trait Implementations

##### `impl Instrument for SingleCrateView<'a>`

##### `impl IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- <span id="singlecrateview-krate"></span>`fn krate(&self) -> &Crate`

- <span id="singlecrateview-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="singlecrateview-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="singlecrateview-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="singlecrateview-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../../generator/config/index.md#renderconfig)

- <span id="singlecrateview-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../../generator/render_shared/index.md#sourcepathconfig)

##### `impl ItemFilter for SingleCrateView<'_>`

- <span id="singlecrateview-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="singlecrateview-include-private"></span>`fn include_private(&self) -> bool`

- <span id="singlecrateview-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- <span id="singlecrateview-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../linker/index.md#linkregistry)

- <span id="singlecrateview-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for SingleCrateView<'a>`

##### `impl Pointable for SingleCrateView<'a>`

- <span id="singlecrateview-const-align"></span>`const ALIGN: usize`

- <span id="singlecrateview-type-init"></span>`type Init = T`

- <span id="singlecrateview-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="singlecrateview-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="singlecrateview-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="singlecrateview-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for SingleCrateView<'a>`

##### `impl WithSubscriber for SingleCrateView<'a>`

