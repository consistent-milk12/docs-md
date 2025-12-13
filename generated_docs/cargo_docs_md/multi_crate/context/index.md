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

*Defined in `src/multi_crate/context.rs:39-62`*

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

  Create a new multi-crate context.

  

  Builds the unified link registry and pre-computes cross-crate impls.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `args` - CLI arguments

  * `config` - Rendering configuration options

- <span id="multicratecontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

  Set the source directory for path transformation.

  

  This can be called after construction if a `.source_*` directory

  is detected or specified via CLI. Only has effect if `source_locations`

  is enabled in the config.

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../../generator/render_shared/index.md#sourcepathconfig)

  Get source path config for a specific file.

  

  Returns `None` if source locations are disabled or no source dir configured.

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](../collection/index.md#cratecollection)

  Build the cross-crate impl map for all crates.

  

  Scans all crates once and groups impl blocks by their target crate

  and type name. This avoids O(n*m) scanning per view creation.

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](../collection/index.md#cratecollection)

  Get the crate collection.

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../registry/index.md#unifiedlinkregistry)

  Get the unified link registry.

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](../../index.md#args)

  Get CLI arguments.

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](#singlecrateview)

  Create a single-crate view for rendering one crate.

  

  This bridges multi-crate mode to existing single-crate rendering

  code by providing a compatible interface that uses the unified

  registry for cross-crate link resolution.

- <span id="multicratecontext-find-item"></span>`fn find_item(&self, id: &Id) -> Option<(&str, &Item)>`

  Find an item across all crates by ID.

  

  Searches through all crates in the collection to find an item with

  the given ID. This is useful for resolving re-exports that point to

  items in external crates.

  

  # Returns

  

  A tuple of `(crate_name, item)` if found, or `None` if the item

  doesn't exist in any crate.

- <span id="multicratecontext-get-cross-crate-impls"></span>`fn get_cross_crate_impls(&self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

  Get pre-computed cross-crate impl blocks for a target crate.

  

  Returns a map from type name to impl blocks from other crates.

  This data is pre-computed during context construction for efficiency.

  

  # Returns

  

  Reference to the type-name -> impl-blocks map, or `None` if the

  crate is not in the collection.

- <span id="multicratecontext-get-impl-target-path"></span>`fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

  Get the target type path for an impl block.

#### Trait Implementations

##### `impl Any for MultiCrateContext<'a>`

- <span id="multicratecontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateContext<'a>`

- <span id="multicratecontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateContext<'a>`

- <span id="multicratecontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateContext<'a>`

- <span id="multicratecontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateContext<'a>`

##### `impl<U> Into for MultiCrateContext<'a>`

- <span id="multicratecontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateContext<'a>`

##### `impl OwoColorize for MultiCrateContext<'a>`

##### `impl Pointable for MultiCrateContext<'a>`

- <span id="multicratecontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicratecontext-pointable-type-init"></span>`type Init = T`

- <span id="multicratecontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratecontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratecontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratecontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateContext<'a>`

- <span id="multicratecontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicratecontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateContext<'a>`

- <span id="multicratecontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicratecontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in `src/multi_crate/context.rs:276-301`*

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

  Create a new single-crate view.

- <span id="singlecrateview-build-impl-map"></span>`fn build_impl_map(&mut self)`

  Build the impl map for all types.

  

  Uses the `impls` field on Struct/Enum/Union items directly rather than

  scanning all items and checking the `for_` field. This provides clearer

  semantics and leverages `rustdoc_types` structured data.

- <span id="singlecrateview-build-type-name-map"></span>`fn build_type_name_map(&mut self)`

  Build a map from type name to type ID.

  

  This is used to look up cross-crate impls by type name.

- <span id="singlecrateview-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

  Generate a sort key for impl blocks.

- <span id="singlecrateview-crate-name"></span>`const fn crate_name(&self) -> &str`

  Get the crate name.

- <span id="singlecrateview-krate"></span>`const fn krate(&self) -> &Crate`

  Get the crate being rendered.

- <span id="singlecrateview-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../registry/index.md#unifiedlinkregistry)

  Get the unified registry.

- <span id="singlecrateview-args"></span>`const fn args(&self) -> &Args` — [`Args`](../../index.md#args)

  Get CLI arguments.

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>>`

  Get impl blocks for a type (local crate only).

- <span id="singlecrateview-get-all-impls"></span>`fn get_all_impls(&self, id: Id) -> Vec<&'a Impl>`

  Get all impl blocks for a type, including cross-crate impls.

  

  This method merges local impls (from this crate) with impls from

  other crates that implement traits for this type.

- <span id="singlecrateview-get-impls-from-crate"></span>`fn get_impls_from_crate(&self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

  Get impl blocks for a type from a specific crate.

  

  This is used for cross-crate re-exports where we need to look up

  impl blocks from the source crate rather than the current crate.

  

  # Arguments

  

  * `id` - The ID of the type to get impls for

  * `source_krate` - The crate to look up impls from

  

  # Returns

  

  A vector of impl blocks found in the source crate for the given type ID.

- <span id="singlecrateview-get-impl-target-id-from-type"></span>`const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

  Extract the target ID from a Type (for impl block matching).

- <span id="singlecrateview-should-include-item"></span>`const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool`

  Check if an item should be included based on visibility.

- <span id="singlecrateview-count-modules"></span>`fn count_modules(&self) -> usize`

  Count modules for progress reporting.

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

  Create a markdown link using the unified registry.

- <span id="singlecrateview-resolve-name"></span>`fn resolve_name(&self, name: &str) -> Option<(String, Id)>`

  Resolve a name to a crate and ID.

- <span id="singlecrateview-lookup-item-across-crates"></span>`fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)>`

  Look up an item across all crates by ID.

  

  This is useful for resolving re-exports that point to items in

  external crates. First checks the local crate, then searches

  all other crates in the collection.

  

  # Returns

  

  A tuple of `(crate_name, item)` if found, or `None` if the item

  doesn't exist in any crate.

- <span id="singlecrateview-get-crate"></span>`fn get_crate(&self, name: &str) -> Option<&Crate>`

  Get a crate by name from the collection.

  

  This is useful for getting the source crate context when rendering

  re-exported items from other crates.

  

  # Returns

  

  The crate if found, or `None` if no crate with that name exists.

- <span id="singlecrateview-resolve-external-path"></span>`fn resolve_external_path(&self, path: &str) -> Option<(&str, &Item, Id)>`

  Resolve a path like `regex_automata::Regex` to an item.

  

  This is used for external re-exports where `use_item.id` is `None`

  but the source path is available.

  

  # Returns

  

  A tuple of `(source_crate, item, item_id)` if found.

- <span id="singlecrateview-process-backtick-links"></span>`fn process_backtick_links(&self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

  Process backtick links like ``Span`` to markdown links.

- <span id="singlecrateview-process-plain-links"></span>`fn process_plain_links(&self, docs: &str, current_file: &str) -> String`

  Process plain links like `[enter]` to markdown links.

  

  Uses the registry to resolve links to proper paths. If the item exists

  in the registry, creates a link to its location. If on the current page

  and has a heading anchor, uses an anchor link.

  

  Skips matches that are:

  - Inside inline code (backticks)

  - Already markdown links (followed by `(` or `[`)

- <span id="singlecrateview-resolve-plain-link"></span>`fn resolve_plain_link(&self, link_text: &str, current_file: &str) -> Option<String>`

  Resolve a plain link `[name]` to a markdown link.

  

  Returns `Some(markdown_link)` if the item can be resolved,

  `None` if it should remain as plain text.

- <span id="singlecrateview-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

  Resolve a link text to a markdown link using the registry.

  

  This function attempts to convert rustdoc link syntax into valid markdown

  links that work in the generated documentation.

  

  # Arguments

  * `link_text` - The raw link target from rustdoc (e.g., "`crate::config::ConfigBuilder::method`")

  * `item_links` - Map of link texts to Item IDs from rustdoc's `links` field

  * `current_file` - The markdown file being generated (e.g., "ureq/index.md")

  

  # Returns

  * `Some(markdown_link)` - A formatted markdown link like `[`text`](path.md#anchor)`

  * `None` - If the link cannot be resolved (will be rendered as inline code)

  

  # Examples

  

  ```text

  Input:  link_text = "crate::config::ConfigBuilder::http_status_as_error"

          current_file = "ureq/index.md"

  Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")

  

  Input:  link_text = "ConfigBuilder"

          current_file = "ureq/agent/index.md"

  Output: Some("[`ConfigBuilder`](../config/index.md#configbuilder)")

  

  Input:  link_text = "std::io::Error"  (external crate, not in registry)

          current_file = "ureq/index.md"

  Output: None  (rendered as `std::io::Error` inline code)

  ```

- <span id="singlecrateview-build-link-to-id"></span>`fn build_link_to_id(&self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

  Build a link to an item by ID.

  

  This is the simplest path when we already have a resolved Item ID from

  rustdoc's links map. We just need to look up the file path in our registry.

  

  # Arguments

  * `id` - The rustdoc Item ID to link to

  * `current_file` - Source file for relative path computation

  * `display_name` - Text to show in the link

  * `anchor` - Optional anchor (e.g., method name)

  

  # Example Transformation

  

  ```text

  Input:

    id = Id(123)  (rustdoc's internal ID for ConfigBuilder)

    current_file = "ureq/agent/index.md"

    display_name = "ConfigBuilder"

    anchor = None

  

  Step 1: Look up ID in registry

    registry.get_path("ureq", Id(123)) → Some("config/index.md")

  

  Step 2: Build markdown link

    build_markdown_link("ureq/agent/index.md", "ureq", "config/index.md", "ConfigBuilder", None)

    → "[`ConfigBuilder`](../config/index.md)"

  

  Output: Some("[`ConfigBuilder`](../config/index.md)")

  ```

- <span id="singlecrateview-resolve-crate-path"></span>`fn resolve_crate_path(&self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

  Resolve `crate::path::Item` or `crate::path::Item::method` patterns.

  

  This handles the common rustdoc pattern where docs reference items using

  crate-relative paths. The tricky part is distinguishing between:

  - `crate::module::Type` (link to Type, no anchor)

  - `crate::module::Type::method` (link to Type with #method anchor)

  - `crate::module::Type::Variant` (link to Type with #Variant anchor)

  

  # Arguments

  * `path_without_crate` - The path after stripping "`crate::`" prefix

  * `display_name` - Full original text for display (includes "`crate::`")

  * `current_file` - Source file for relative path computation

  

  # Example Transformation

  

  ```text

  Input:

    path_without_crate = "config::ConfigBuilder::http_status_as_error"

    display_name = "crate::config::ConfigBuilder::http_status_as_error"

    current_file = "ureq/index.md"

  

  Step 1: Split into type path and anchor

    split_type_and_anchor("config::ConfigBuilder::http_status_as_error")

    → ("config::ConfigBuilder", Some("http_status_as_error"))

    (lowercase "http_status_as_error" indicates a method)

  

  Step 2: Extract the type name (last segment of type path)

    "config::ConfigBuilder".rsplit("::").next() → "ConfigBuilder"

  

  Step 3: Resolve type name in registry

    registry.resolve_name("ConfigBuilder", "ureq") → Some(("ureq", Id(123)))

    registry.get_path("ureq", Id(123)) → Some("config/index.md")

  

  Step 4: Build markdown link with anchor

    build_markdown_link("ureq/index.md", "ureq", "config/index.md",

                        "crate::config::ConfigBuilder::http_status_as_error",

                        Some("http_status_as_error"))

    → "[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)"

  

  Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")

  ```

- <span id="singlecrateview-split-type-and-anchor"></span>`fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

  Split `config::ConfigBuilder::method` into (`config::ConfigBuilder`, Some("method")).

  

  Detects methods (lowercase) and enum variants (`Type::Variant` pattern).

  

  # Detection Rules

  

  1. **Methods/fields**: Last segment starts with lowercase

     - `Type::method` → (Type, method)

     - `mod::Type::field_name` → (`mod::Type`, `field_name`)

  

  2. **Enum variants**: Two consecutive uppercase segments

     - `Option::Some` → (Option, Some)

     - `mod::Error::IoError` → (`mod::Error`, `IoError`)

  

  3. **Nested types**: Uppercase but no uppercase predecessor

     - `mod::OuterType::InnerType` → (`mod::OuterType::InnerType`, None)

  

  # Examples

  

  ```text

  "ConfigBuilder::http_status_as_error"

    Last segment "http_status_as_error" starts lowercase → method

    → ("ConfigBuilder", Some("http_status_as_error"))

  

  "config::ConfigBuilder::new"

    Last segment "new" starts lowercase → method

    → ("config::ConfigBuilder", Some("new"))

  

  "Option::Some"

    "Option" uppercase, "Some" uppercase → enum variant

    → ("Option", Some("Some"))

  

  "error::Error::Io"

    "Error" uppercase, "Io" uppercase → enum variant

    → ("error::Error", Some("Io"))

  

  "config::ConfigBuilder"

    "config" lowercase, "ConfigBuilder" uppercase → not a variant

    → ("config::ConfigBuilder", None)

  

  "Vec"

    No "::" separator

    → ("Vec", None)

  ```

- <span id="singlecrateview-build-markdown-link"></span>`fn build_markdown_link(&self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

  Build a markdown link, handling same-crate and cross-crate cases.

  

  This is the core function that computes relative paths between markdown

  files and formats the final link.

  

  # Arguments

  * `current_file` - The file we're generating (e.g., "ureq/agent/index.md")

  * `target_crate` - The crate containing the target item

  * `target_path` - Path to target within its crate (e.g., "config/index.md")

  * `display_name` - Text to show in the link

  * `anchor` - Optional anchor suffix (e.g., "`method_name`")

  

  # Path Computation Examples

  

  ## Same Crate Examples

  

  ```text

  Example 1: Link from index to nested module

     current_file = "ureq/index.md"

     target_crate = "ureq"

     target_path = "config/index.md"

  

     Step 1: Strip crate prefix from current

       "ureq/index.md" -> "index.md"

  

     Step 2: Compute relative path

       from "index.md" to "config/index.md"

       -> "config/index.md"

  

     Output: "[`display`](config/index.md)"

  

  Example 2: Link from nested to sibling module

     current_file = "ureq/agent/index.md"

     target_crate = "ureq"

     target_path = "config/index.md"

  

     Step 1: Strip crate prefix

       "ureq/agent/index.md" -> "agent/index.md"

  

     Step 2: Compute relative path

       from "agent/index.md" to "config/index.md"

       -> "config/index.md"

  

     Output: "[`display`][../config/index.md]"

  

  ## Cross-Crate Examples

  

  ```text

  Example 3: Link from one crate to another

    current_file = "ureq/agent/index.md"

    target_crate = "http"

    target_path  = "status/index.md"

  

    Step 1: Strip crate prefix

      "ureq/agent/index.md" → "agent/index.md"

  

    Step 2: Count depth (number of '/' in local path)

      "agent/index.md" has 1 slash → depth = 1

  

    Step 3: Build cross-crate path

      Go up (depth + 1) levels: "../" * 2 = "../../"

      Then into target crate: "../../http/status/index.md"

  

    Output: "[`display`](../../http/status/index.md)"

  

  Example 4: Cross-crate from root

    current_file = "ureq/index.md"

    target_crate = "http"

    target_path  = "index.md"

  

    depth = 0 (no slashes in "index.md")

    prefix = "../" * 1 = "../"

  

    Output: "[`display`](../http/index.md)"

  ```rust

- <span id="singlecrateview-compute-cross-crate-path"></span>`fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

  Compute a relative path for cross-crate linking.

  

  Given the local portion of the current file path (without crate prefix),

  computes the `../` prefix needed to navigate to another crate's file.

  

  # Arguments

  * `current_local` - Current file path within crate (e.g., "agent/index.md")

  * `target_crate` - Name of the target crate

  * `target_path` - Path within target crate (e.g., "status/index.md")

  

  # Examples

  

  ```text

  // From root of one crate to another

  compute_cross_crate_path("index.md", "http", "index.md")

    → "../http/index.md"

  

  // From nested module to another crate

  compute_cross_crate_path("agent/index.md", "http", "status/index.md")

    → "../../http/status/index.md"

  

  // From deeply nested to another crate root

  compute_cross_crate_path("a/b/c/index.md", "other", "index.md")

    → "../../../../other/index.md"

  ```

- <span id="singlecrateview-strip-crate-prefix"></span>`fn strip_crate_prefix(path: &str) -> &str`

  Strip the crate prefix from a file path.

  

  File paths in our system includes the crate name as the first directory.

  This helper removes it to get the crate-local path.

  

  # Examples

  

  ```text

  "ureq/config/index.md" -> "config/index.md"

  "ureq/index.md"        -> "index.md"

  "http/status/index.md" -> "status/index.md"

  "simple.md"            -> "simple.md" (no slash returns as is)

  ```

- <span id="singlecrateview-looks-like-external-reference"></span>`fn looks_like_external_reference(link_text: &str) -> bool`

  Check if a link text looks like an intentional external crate reference.

  

  Simple names like "Wide", "Error", "Default" are often meant to be

  local anchors or type aliases, not cross-crate links.

#### Trait Implementations

##### `impl Any for SingleCrateView<'a>`

- <span id="singlecrateview-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SingleCrateView<'a>`

- <span id="singlecrateview-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SingleCrateView<'a>`

- <span id="singlecrateview-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SingleCrateView<'a>`

- <span id="singlecrateview-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SingleCrateView<'a>`

##### `impl<U> Into for SingleCrateView<'a>`

- <span id="singlecrateview-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- <span id="singlecrateview-itemaccess-krate"></span>`fn krate(&self) -> &Crate`

- <span id="singlecrateview-itemaccess-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="singlecrateview-itemaccess-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="singlecrateview-itemaccess-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="singlecrateview-itemaccess-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="singlecrateview-itemaccess-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../../generator/config/index.md#renderconfig)

- <span id="singlecrateview-itemaccess-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../../generator/render_shared/index.md#sourcepathconfig)

##### `impl ItemFilter for SingleCrateView<'_>`

- <span id="singlecrateview-itemfilter-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="singlecrateview-itemfilter-include-private"></span>`fn include_private(&self) -> bool`

- <span id="singlecrateview-itemfilter-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- <span id="singlecrateview-linkresolver-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../linker/index.md#linkregistry)

- <span id="singlecrateview-linkresolver-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="singlecrateview-linkresolver-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for SingleCrateView<'a>`

##### `impl Pointable for SingleCrateView<'a>`

- <span id="singlecrateview-pointable-const-align"></span>`const ALIGN: usize`

- <span id="singlecrateview-pointable-type-init"></span>`type Init = T`

- <span id="singlecrateview-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="singlecrateview-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="singlecrateview-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="singlecrateview-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for SingleCrateView<'a>`

##### `impl<U> TryFrom for SingleCrateView<'a>`

- <span id="singlecrateview-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="singlecrateview-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SingleCrateView<'a>`

- <span id="singlecrateview-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="singlecrateview-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SingleCrateView<'a>`

