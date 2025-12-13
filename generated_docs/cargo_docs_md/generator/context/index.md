*[cargo_docs_md](../../index.md) / [generator](../index.md) / [context](index.md)*

---

# Module `context`

Shared context for documentation generation.

This module provides the [`GeneratorContext`](#generatorcontext) struct which holds all shared
state needed during markdown generation, including the crate data, lookup
maps, and configuration options.

# Trait Hierarchy

The rendering context is split into focused traits for better abstraction:

- [`ItemAccess`](#itemaccess) - Core data access (crate, items, impls)
- [`ItemFilter`](#itemfilter) - Visibility and filtering logic
- [`LinkResolver`](#linkresolver) - Link creation and documentation processing
- [`RenderContext`](#rendercontext) - Combined super-trait for convenience

This allows components to depend only on the traits they need, improving
testability and reducing coupling.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GeneratorContext`](#generatorcontext) | struct | Shared context containing all data needed for documentation generation. |
| [`ItemAccess`](#itemaccess) | trait | Core data access for crate documentation. |
| [`ItemFilter`](#itemfilter) | trait | Item visibility and filtering logic. |
| [`LinkResolver`](#linkresolver) | trait | Link creation and documentation processing. |
| [`RenderContext`](#rendercontext) | trait | Combined rendering context trait. |

## Structs

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    crate_name: String,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
    pub config: crate::generator::config::RenderConfig,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
    source_path_config: Option<crate::generator::render_shared::SourcePathConfig>,
}
```

*Defined in `src/generator/context.rs:135-168`*

Shared context containing all data needed for documentation generation.

This struct is passed to all rendering components and provides:
- Access to the parsed crate data
- Impl block lookup for rendering implementations
- Link registry for cross-references
- CLI configuration options

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The parsed rustdoc JSON crate.

- **`crate_name`**: `String`

  The crate name (extracted from root module).

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Maps type IDs to all impl blocks for that type.
  
  Used for rendering "Implementations" and "Trait Implementations" sections.

- **`link_registry`**: `crate::linker::LinkRegistry`

  Registry for creating cross-reference links between items.

- **`args`**: `&'a crate::Args`

  CLI arguments containing output path, format, and options.

- **`config`**: `crate::generator::config::RenderConfig`

  Rendering configuration options.

- **`path_name_index`**: `std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>`

  Pre-built index mapping item names to their IDs for fast lookup.
  
  Built once at construction time from `krate.paths` and shared across
  all `DocLinkProcessor` instances for efficiency.

- **`source_path_config`**: `Option<crate::generator::render_shared::SourcePathConfig>`

  Base source path configuration for transforming cargo registry paths.
  
  `None` if source locations are disabled or no `.source_*` dir detected.
  The `depth` field is set to 0; use `source_path_config_for_file()` to
  get a config with the correct depth for a specific file.

#### Implementations

- <span id="generatorcontext-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Self` — [`Args`](../../index.md#args), [`RenderConfig`](../config/index.md#renderconfig)

  Create a new generator context from crate data and CLI arguments.

  

  Builds the path map, impl map, and link registry needed for generation.

  

  # Arguments

  

  * `krate` - The parsed rustdoc JSON crate

  * `args` - CLI arguments containing output path, format, and options

  * `config` - Rendering configuration options

- <span id="generatorcontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

  Set the source directory for path transformation.

  

  This can be called after construction if a `.source_*` directory

  is detected or specified via CLI. Only has effect if `source_locations`

  is enabled in the config.

- <span id="generatorcontext-build-impl-map"></span>`fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>>`

  Build a map from type ID to all impl blocks for that type.

  

  This enables rendering the "Implementations" and "Trait Implementations"

  sections for structs, enums, and other types.

  

  Uses the `impls` field on Struct/Enum/Union items directly rather than

  scanning all items and checking the `for_` field. This provides clearer

  semantics and leverages `rustdoc_types` structured data.

- <span id="generatorcontext-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

  Generate a sort key for an impl block.

  

  Inherent impls (no trait) sort before trait impls.

  Trait impls are sorted by trait name.

- <span id="generatorcontext-should-include-item"></span>`const fn should_include_item(&self, item: &Item) -> bool`

  Check if an item should be included based on visibility settings.

  

  By default, all items are included. If `--exclude-private`

  is set, only public items are included.

  

  # Visibility Levels

  

  - `Public` - Always included

  - `Crate`, `Restricted`, `Default` - Included by default, excluded with `--exclude-private`

- <span id="generatorcontext-count-modules"></span>`fn count_modules(&self, item: &Item) -> usize`

  Count the total number of modules that will be generated.

  

  Used to initialize the progress bar with the correct total.

  Respects the `--exclude-private` flag when counting.

- <span id="generatorcontext-build-path-name-index"></span>`fn build_path_name_index(krate: &'a Crate) -> HashMap<&'a str, Vec<Id>>`

  Build an index mapping item names to their IDs for fast lookup.

  

  This index is built once at context construction time and shared

  across all `DocLinkProcessor` instances, eliminating redundant

  index building for each item with documentation.

#### Trait Implementations

##### `impl Any for GeneratorContext<'a>`

- <span id="generatorcontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GeneratorContext<'a>`

- <span id="generatorcontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GeneratorContext<'a>`

- <span id="generatorcontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for GeneratorContext<'a>`

- <span id="generatorcontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for GeneratorContext<'a>`

##### `impl<U> Into for GeneratorContext<'a>`

- <span id="generatorcontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GeneratorContext<'a>`

##### `impl ItemAccess for GeneratorContext<'_>`

- <span id="generatorcontext-itemaccess-krate"></span>`fn krate(&self) -> &Crate`

- <span id="generatorcontext-itemaccess-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="generatorcontext-itemaccess-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="generatorcontext-itemaccess-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="generatorcontext-itemaccess-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="generatorcontext-itemaccess-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../config/index.md#renderconfig)

- <span id="generatorcontext-itemaccess-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../render_shared/index.md#sourcepathconfig)

##### `impl ItemFilter for GeneratorContext<'_>`

- <span id="generatorcontext-itemfilter-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="generatorcontext-itemfilter-include-private"></span>`fn include_private(&self) -> bool`

- <span id="generatorcontext-itemfilter-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for GeneratorContext<'_>`

- <span id="generatorcontext-linkresolver-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../linker/index.md#linkregistry)

- <span id="generatorcontext-linkresolver-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="generatorcontext-linkresolver-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for GeneratorContext<'a>`

##### `impl Pointable for GeneratorContext<'a>`

- <span id="generatorcontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="generatorcontext-pointable-type-init"></span>`type Init = T`

- <span id="generatorcontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generatorcontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generatorcontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generatorcontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for GeneratorContext<'a>`

##### `impl<U> TryFrom for GeneratorContext<'a>`

- <span id="generatorcontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generatorcontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GeneratorContext<'a>`

- <span id="generatorcontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generatorcontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for GeneratorContext<'a>`

## Traits

### `ItemAccess`

```rust
trait ItemAccess { ... }
```

*Defined in `src/generator/context.rs:37-63`*

Core data access for crate documentation.

Provides read-only access to the crate structure, items, and impl blocks.

#### Required Methods

- `fn krate(&self) -> &Crate`

  Get the crate being documented.

- `fn crate_name(&self) -> &str`

  Get the crate name.

- `fn get_item(&self, id: &Id) -> Option<&Item>`

  Get an item by its ID.

- `fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

  Get impl blocks for a type.

- `fn crate_version(&self) -> Option<&str>`

  Get the crate version for display in headers.

- `fn render_config(&self) -> &RenderConfig`

  Get the rendering configuration.

#### Provided Methods

- `fn source_path_config_for_file(&self, _current_file: &str) -> Option<SourcePathConfig>`

  Get source path config for a specific file.

#### Implementors

- [`GeneratorContext`](#generatorcontext)
- [`SingleCrateView`](../../multi_crate/context/index.md#singlecrateview)

### `ItemFilter`

```rust
trait ItemFilter { ... }
```

*Defined in `src/generator/context.rs:68-79`*

Item visibility and filtering logic.

Determines which items should be included in the generated documentation.

#### Required Methods

- `fn should_include_item(&self, item: &Item) -> bool`

  Check if an item should be included based on visibility.

- `fn include_private(&self) -> bool`

  Whether private items should be included.

- `fn include_blanket_impls(&self) -> bool`

  Whether blanket trait implementations should be included.

#### Implementors

- [`GeneratorContext`](#generatorcontext)
- [`SingleCrateView`](../../multi_crate/context/index.md#singlecrateview)

### `LinkResolver`

```rust
trait LinkResolver { ... }
```

*Defined in `src/generator/context.rs:84-113`*

Link creation and documentation processing.

Handles intra-doc link resolution and markdown link generation.

#### Required Methods

- `fn link_registry(&self) -> Option<&LinkRegistry>`

  Get the link registry for single-crate mode.

- `fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

  Process documentation string with intra-doc link resolution.

- `fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

  Create a markdown link to an item.

#### Implementors

- [`GeneratorContext`](#generatorcontext)
- [`SingleCrateView`](../../multi_crate/context/index.md#singlecrateview)

### `RenderContext`

```rust
trait RenderContext: ItemAccess + ItemFilter + LinkResolver { ... }
```

*Defined in `src/generator/context.rs:126`*

Combined rendering context trait.

This trait combines [`ItemAccess`](#itemaccess), [`ItemFilter`](#itemfilter), and [`LinkResolver`](#linkresolver)
for components that need full access to the rendering context.

Most renderers should use this trait for convenience, but components
with limited requirements can depend on individual sub-traits.

#### Implementors

- `T`

