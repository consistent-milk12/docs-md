*[cargo_docs_md](../../index.md) / [generator](../index.md) / [render_shared](index.md)*

---

# Module `render_shared`

Shared rendering functions for documentation generation.

This module contains standalone rendering functions that can be used by both
single-crate ([`ItemRenderer`](super::ItemRenderer)) and multi-crate
([`MultiCrateModuleRenderer`](crate::multi_crate::generator)) renderers.

These functions handle the core markdown generation logic without being tied
to a specific rendering context, avoiding code duplication between the two modes.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourcePathConfig`](#sourcepathconfig) | struct | Information needed to transform source paths to relative links. |
| [`CategorizedTraitItems`](#categorizedtraititems) | struct | Categorized trait items for structured rendering. |
| [`RendererUtils`](#rendererutils) | struct | Unit struct to organize path related utility functions related to renderer functions. |
| [`TraitRenderer`](#traitrenderer) | struct | Unit struct to organize trait related functions. |
| [`RendererInternals`](#rendererinternals) | struct | Unit struct containing renderer functions. |
| [`DocsProcessor`](#docsprocessor) | trait | Check if a render context can resolve documentation. |

## Structs

### `SourcePathConfig`

```rust
struct SourcePathConfig {
    pub source_dir_name: String,
    pub depth: usize,
}
```

*Defined in `src/generator/render_shared.rs:30-37`*

Information needed to transform source paths to relative links.

When generating source location references, this config enables transforming
absolute cargo registry paths to relative links pointing to the local
`.source_{timestamp}` directory.

#### Fields

- **`source_dir_name`**: `String`

  The `.source_{timestamp}` directory name (e.g., `.source_1733660400`).

- **`depth`**: `usize`

  Depth of the current markdown file from `generated_docs/`.
  Used to calculate the correct number of `../` prefixes.

#### Implementations

- <span id="sourcepathconfig-new"></span>`fn new(source_dir: &Path, current_file: &str) -> Self`

- <span id="sourcepathconfig-with-depth"></span>`fn with_depth(&self, current_file: &str) -> Self`

#### Trait Implementations

##### `impl Clone for SourcePathConfig`

- <span id="sourcepathconfig-clone"></span>`fn clone(&self) -> SourcePathConfig` — [`SourcePathConfig`](#sourcepathconfig)

##### `impl Debug for SourcePathConfig`

- <span id="sourcepathconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SourcePathConfig`

##### `impl IntoEither for SourcePathConfig`

##### `impl OwoColorize for SourcePathConfig`

##### `impl Pointable for SourcePathConfig`

- <span id="sourcepathconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcepathconfig-pointable-type-init"></span>`type Init = T`

- <span id="sourcepathconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcepathconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcepathconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcepathconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourcePathConfig`

### `CategorizedTraitItems<'a>`

```rust
struct CategorizedTraitItems<'a> {
    pub required_methods: Vec<&'a rustdoc_types::Item>,
    pub provided_methods: Vec<&'a rustdoc_types::Item>,
    pub associated_types: Vec<&'a rustdoc_types::Item>,
    pub associated_consts: Vec<&'a rustdoc_types::Item>,
}
```

*Defined in `src/generator/render_shared.rs:76-88`*

Categorized trait items for structured rendering.

#### Fields

- **`required_methods`**: `Vec<&'a rustdoc_types::Item>`

  Required methods (no default body)

- **`provided_methods`**: `Vec<&'a rustdoc_types::Item>`

  Provided methods (have default body)

- **`associated_types`**: `Vec<&'a rustdoc_types::Item>`

  Associated types

- **`associated_consts`**: `Vec<&'a rustdoc_types::Item>`

  Associated constants

#### Implementations

- <span id="categorizedtraititems-categorize-trait-items"></span>`fn categorize_trait_items(trait_items: &[Id], krate: &'a Crate) -> Self`

#### Trait Implementations

##### `impl Default for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-default"></span>`fn default() -> CategorizedTraitItems<'a>` — [`CategorizedTraitItems`](#categorizedtraititems)

##### `impl Instrument for CategorizedTraitItems<'a>`

##### `impl IntoEither for CategorizedTraitItems<'a>`

##### `impl OwoColorize for CategorizedTraitItems<'a>`

##### `impl Pointable for CategorizedTraitItems<'a>`

- <span id="categorizedtraititems-pointable-const-align"></span>`const ALIGN: usize`

- <span id="categorizedtraititems-pointable-type-init"></span>`type Init = T`

- <span id="categorizedtraititems-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="categorizedtraititems-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="categorizedtraititems-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="categorizedtraititems-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CategorizedTraitItems<'a>`

### `RendererUtils`

```rust
struct RendererUtils;
```

*Defined in `src/generator/render_shared.rs:127`*

Unit struct to organize path related utility functions related to renderer functions.

#### Implementations

- <span id="rendererutils-sanitize-path"></span>`fn sanitize_path(path: &str) -> Cow<'_, str>`

- <span id="rendererutils-sanitize-self-param"></span>`fn sanitize_self_param(param: &str) -> Cow<'_, str>`

- <span id="rendererutils-write-tuple-fields"></span>`fn write_tuple_fields(out: &mut String, fields: &[Option<Id>], krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererutils-transform-cargo-path"></span>`fn transform_cargo_path(absolute_path: &Path, source_dir_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl Instrument for RendererUtils`

##### `impl IntoEither for RendererUtils`

##### `impl OwoColorize for RendererUtils`

##### `impl Pointable for RendererUtils`

- <span id="rendererutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rendererutils-pointable-type-init"></span>`type Init = T`

- <span id="rendererutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rendererutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rendererutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rendererutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for RendererUtils`

### `TraitRenderer`

```rust
struct TraitRenderer;
```

*Defined in `src/generator/render_shared.rs:253`*

Unit struct to organize trait related functions.

#### Implementations

- <span id="traitrenderer-write-trait-bounds"></span>`fn write_trait_bounds(out: &mut String, bounds: &[rustdoc_types::GenericBound], type_renderer: TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="traitrenderer-render-trait-definition"></span>`fn render_trait_definition(md: &mut String, name: &str, t: &rustdoc_types::Trait, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="traitrenderer-render-trait-item"></span>`fn render_trait_item<F>(md: &mut String, item: &Item, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

#### Trait Implementations

##### `impl Instrument for TraitRenderer`

##### `impl IntoEither for TraitRenderer`

##### `impl OwoColorize for TraitRenderer`

##### `impl Pointable for TraitRenderer`

- <span id="traitrenderer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitrenderer-pointable-type-init"></span>`type Init = T`

- <span id="traitrenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitrenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitrenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitrenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TraitRenderer`

### `RendererInternals`

```rust
struct RendererInternals;
```

*Defined in `src/generator/render_shared.rs:415`*

Unit struct containing renderer functions.
Helpful because free functions are annoying.

#### Implementations

- <span id="rendererinternals-render-struct-definition"></span>`fn render_struct_definition(md: &mut String, name: &str, s: &rustdoc_types::Struct, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-struct-fields"></span>`fn render_struct_fields<F>(md: &mut String, fields: &[Id], krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-enum-definition"></span>`fn render_enum_definition(md: &mut String, name: &str, e: &rustdoc_types::Enum, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-enum-variant"></span>`fn render_enum_variant(md: &mut String, variant: &Item, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-enum-variants-docs"></span>`fn render_enum_variants_docs<F>(md: &mut String, variants: &[Id], krate: &Crate, process_docs: F)`

- <span id="rendererinternals-render-function-definition"></span>`fn render_function_definition(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-constant-definition"></span>`fn render_constant_definition(md: &mut String, name: &str, type_: &rustdoc_types::Type, const_: &rustdoc_types::Constant, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-type-alias-definition"></span>`fn render_type_alias_definition(md: &mut String, name: &str, ta: &rustdoc_types::TypeAlias, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-macro-heading"></span>`fn render_macro_heading(md: &mut String, name: &str)`

- <span id="rendererinternals-render-impl-items"></span>`fn render_impl_items<F, L>(md: &mut String, impl_block: &Impl, krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: &Option<F>, create_type_link: &Option<L>, parent_type_name: Option<&str>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-function-type-links-inline"></span>`fn render_function_type_links_inline<L>(md: &mut String, f: &rustdoc_types::Function, type_renderer: TypeRenderer<'_>, create_link: &L)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-impl-function"></span>`fn render_impl_function(md: &mut String, name: &str, f: &rustdoc_types::Function, type_renderer: TypeRenderer<'_>, parent_type_name: Option<&str>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-append-docs"></span>`fn append_docs(md: &mut String, docs: Option<String>)`

- <span id="rendererinternals-render-collapsible-start"></span>`fn render_collapsible_start(summary: &str) -> String`

- <span id="rendererinternals-render-collapsible-end"></span>`const fn render_collapsible_end() -> &'static str`

- <span id="rendererinternals-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl, type_renderer: &TypeRenderer<'_>) -> String` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-source-location"></span>`fn render_source_location(span: Option<&Span>, source_path_config: Option<&SourcePathConfig>) -> String` — [`SourcePathConfig`](#sourcepathconfig)

- <span id="rendererinternals-render-union-definition"></span>`fn render_union_definition(md: &mut String, name: &str, u: &rustdoc_types::Union, krate: &Crate, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-union-fields"></span>`fn render_union_fields<F>(md: &mut String, fields: &[Id], krate: &Crate, type_renderer: &TypeRenderer<'_>, process_docs: F)` — [`TypeRenderer`](../../types/index.md#typerenderer)

- <span id="rendererinternals-render-static-definition"></span>`fn render_static_definition(md: &mut String, name: &str, s: &rustdoc_types::Static, type_renderer: &TypeRenderer<'_>)` — [`TypeRenderer`](../../types/index.md#typerenderer)

#### Trait Implementations

##### `impl Instrument for RendererInternals`

##### `impl IntoEither for RendererInternals`

##### `impl OwoColorize for RendererInternals`

##### `impl Pointable for RendererInternals`

- <span id="rendererinternals-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rendererinternals-pointable-type-init"></span>`type Init = T`

- <span id="rendererinternals-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rendererinternals-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rendererinternals-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rendererinternals-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for RendererInternals`

## Traits

### `DocsProcessor`

```rust
trait DocsProcessor { ... }
```

*Defined in `src/generator/render_shared.rs:1327-1330`*

Check if a render context can resolve documentation.

This trait provides a unified way to process docs from different contexts.

#### Required Methods

- `fn process_item_docs(&self, item: &Item) -> Option<String>`

  Process documentation for an item, resolving intra-doc links.

#### Implementors

- `(&T, &str)`

