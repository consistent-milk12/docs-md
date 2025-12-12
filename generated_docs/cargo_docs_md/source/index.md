*[cargo_docs_md](../index.md) / [source](index.md)*

---

# Module `source`

Source code parsing for enhanced documentation.

This module provides functionality to collect and parse Rust source code
from dependencies, extracting information not available in rustdoc JSON:

- Function bodies and implementation details
- Private items (functions, structs, etc.)
- Constant and static values
- Macro definitions
- Test examples

# Architecture

The source parsing system has four main components:

1. [`SourceCollector`](collector/index.md) - Collects dependency sources to `.source_*/`
2. [`SourceLocator`](locator/index.md) - Finds crate sources in the Cargo registry
3. [`SourceParser`](parser/index.md) - Parses Rust source files using `syn`
4. [`types`](types/index.md) - Data structures for parsed source information

# Workflow

```no_run
use cargo_docs_md::source::{SourceCollector, CollectOptions};

// Collect dependency sources to .source_{timestamp}/
let collector = SourceCollector::new()?;
let result = collector.collect(&CollectOptions::default())?;
println!("Collected {} crates to {}", result.crates_collected, result.output_dir.display());
Ok::<(), cargo_docs_md::error::Error>(())
```

# Feature Flag

This module requires the `source-parsing` feature:

```toml
cargo-docs-md = { version = "0.1", features = ["source-parsing"] }
```

## Contents

- [Modules](#modules)
  - [`collector`](#collector)
  - [`integration`](#integration)
  - [`locator`](#locator)
  - [`parser`](#parser)
  - [`types`](#types)
- [Structs](#structs)
  - [`CollectOptions`](#collectoptions)
  - [`CollectedCrate`](#collectedcrate)
  - [`CollectionResult`](#collectionresult)
  - [`SourceCollector`](#sourcecollector)
  - [`SourceManifest`](#sourcemanifest)
  - [`SourceLocator`](#sourcelocator)
  - [`SourceParser`](#sourceparser)
  - [`ConstInfo`](#constinfo)
  - [`CrateSource`](#cratesource)
  - [`EnumInfo`](#enuminfo)
  - [`FieldInfo`](#fieldinfo)
  - [`FunctionInfo`](#functioninfo)
  - [`ImplInfo`](#implinfo)
  - [`MacroInfo`](#macroinfo)
  - [`StaticInfo`](#staticinfo)
  - [`StructInfo`](#structinfo)
  - [`TraitInfo`](#traitinfo)
  - [`TypeAliasInfo`](#typealiasinfo)
  - [`VariantInfo`](#variantinfo)
- [Enums](#enums)
  - [`PrivateItem`](#privateitem)
- [Functions](#functions)
  - [`find_source_dir`](#find-source-dir)
  - [`extract_source_timestamp`](#extract-source-timestamp)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`collector`](#collector) | mod | Source collector for copying dependency sources to a local directory. |
| [`integration`](#integration) | mod | Generator integration for source-enhanced documentation. |
| [`locator`](#locator) | mod | Source locator for finding crate sources in the Cargo registry. |
| [`parser`](#parser) | mod | Source code parser using `syn`. |
| [`types`](#types) | mod | Types for representing parsed source code information. |
| [`CollectOptions`](#collectoptions) | struct |  |
| [`CollectedCrate`](#collectedcrate) | struct |  |
| [`CollectionResult`](#collectionresult) | struct |  |
| [`SourceCollector`](#sourcecollector) | struct |  |
| [`SourceManifest`](#sourcemanifest) | struct |  |
| [`SourceLocator`](#sourcelocator) | struct |  |
| [`SourceParser`](#sourceparser) | struct |  |
| [`ConstInfo`](#constinfo) | struct |  |
| [`CrateSource`](#cratesource) | struct |  |
| [`EnumInfo`](#enuminfo) | struct |  |
| [`FieldInfo`](#fieldinfo) | struct |  |
| [`FunctionInfo`](#functioninfo) | struct |  |
| [`ImplInfo`](#implinfo) | struct |  |
| [`MacroInfo`](#macroinfo) | struct |  |
| [`StaticInfo`](#staticinfo) | struct |  |
| [`StructInfo`](#structinfo) | struct |  |
| [`TraitInfo`](#traitinfo) | struct |  |
| [`TypeAliasInfo`](#typealiasinfo) | struct |  |
| [`VariantInfo`](#variantinfo) | struct |  |
| [`PrivateItem`](#privateitem) | enum |  |
| [`find_source_dir`](#find-source-dir) | fn | Find the most recent `.source_*` directory in the given root. |
| [`extract_source_timestamp`](#extract-source-timestamp) | fn | Extract timestamp from a `.source_*` directory path. |

## Modules

- [`collector`](collector/index.md) — Source collector for copying dependency sources to a local directory.
- [`integration`](integration/index.md) — Generator integration for source-enhanced documentation.
- [`locator`](locator/index.md) — Source locator for finding crate sources in the Cargo registry.
- [`parser`](parser/index.md) — Source code parser using `syn`.
- [`types`](types/index.md) — Types for representing parsed source code information.

## Structs

### `CollectOptions`

```rust
struct CollectOptions {
    pub include_dev: bool,
    pub output: Option<std::path::PathBuf>,
    pub dry_run: bool,
}
```

*Defined in `src/source/collector.rs:67-76`*

Options for source collection.

#### Fields

- **`include_dev`**: `bool`

  Include dev-dependencies.

- **`output`**: `Option<std::path::PathBuf>`

  Custom output directory (overrides timestamp-based naming).

- **`dry_run`**: `bool`

  Dry run - don't actually copy files.

#### Trait Implementations

##### `impl Debug for CollectOptions`

- <span id="collectoptions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CollectOptions`

- <span id="collectoptions-default"></span>`fn default() -> CollectOptions` — [`CollectOptions`](collector/index.md#collectoptions)

##### `impl Instrument for CollectOptions`

##### `impl IntoEither for CollectOptions`

##### `impl OwoColorize for CollectOptions`

##### `impl Pointable for CollectOptions`

- <span id="collectoptions-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectoptions-pointable-type-init"></span>`type Init = T`

- <span id="collectoptions-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectoptions-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectoptions-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectoptions-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CollectOptions`

### `CollectedCrate`

```rust
struct CollectedCrate {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub features: Vec<String>,
    pub description: Option<String>,
    pub source_path: String,
}
```

*Defined in `src/source/collector.rs:19-37`*

Metadata about a collected crate.

#### Fields

- **`name`**: `String`

  Crate name.

- **`version`**: `String`

  Crate version.

- **`edition`**: `String`

  Rust edition.

- **`features`**: `Vec<String>`

  Enabled features.

- **`description`**: `Option<String>`

  Crate description.

- **`source_path`**: `String`

  Relative path within the .source_*/ directory.

#### Trait Implementations

##### `impl Clone for CollectedCrate`

- <span id="collectedcrate-clone"></span>`fn clone(&self) -> CollectedCrate` — [`CollectedCrate`](collector/index.md#collectedcrate)

##### `impl Debug for CollectedCrate`

- <span id="collectedcrate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for CollectedCrate`

- <span id="collectedcrate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for CollectedCrate`

##### `impl Instrument for CollectedCrate`

##### `impl IntoEither for CollectedCrate`

##### `impl OwoColorize for CollectedCrate`

##### `impl Pointable for CollectedCrate`

- <span id="collectedcrate-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectedcrate-pointable-type-init"></span>`type Init = T`

- <span id="collectedcrate-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectedcrate-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectedcrate-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectedcrate-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for CollectedCrate`

- <span id="collectedcrate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl WithSubscriber for CollectedCrate`

### `CollectionResult`

```rust
struct CollectionResult {
    pub output_dir: std::path::PathBuf,
    pub crates_collected: usize,
    pub skipped: Vec<String>,
}
```

*Defined in `src/source/collector.rs:54-63`*

Result of a collection operation.

#### Fields

- **`output_dir`**: `std::path::PathBuf`

  Path to the created .source_*/ directory.

- **`crates_collected`**: `usize`

  Number of crates collected.

- **`skipped`**: `Vec<String>`

  Crates that were skipped (not found in registry).

#### Trait Implementations

##### `impl Debug for CollectionResult`

- <span id="collectionresult-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for CollectionResult`

##### `impl IntoEither for CollectionResult`

##### `impl OwoColorize for CollectionResult`

##### `impl Pointable for CollectionResult`

- <span id="collectionresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectionresult-pointable-type-init"></span>`type Init = T`

- <span id="collectionresult-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectionresult-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectionresult-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectionresult-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CollectionResult`

### `SourceCollector`

```rust
struct SourceCollector {
    metadata: cargo_metadata::Metadata,
    registry_path: std::path::PathBuf,
}
```

*Defined in `src/source/collector.rs:80-86`*

Collector for gathering dependency sources.

#### Fields

- **`metadata`**: `cargo_metadata::Metadata`

  Cargo metadata for the workspace.

- **`registry_path`**: `std::path::PathBuf`

  Path to cargo registry sources.

#### Implementations

- <span id="sourcecollector-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcecollector-from-manifest"></span>`fn from_manifest(manifest_path: Option<&Path>) -> Result<Self, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcecollector-collect"></span>`fn collect(&self, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](collector/index.md#collectoptions), [`CollectionResult`](collector/index.md#collectionresult), [`Error`](../error/index.md#error)

- <span id="sourcecollector-generate-output-dir"></span>`fn generate_output_dir(&self) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcecollector-find-registry-source"></span>`fn find_registry_source(&self, name: &str, version: &str) -> Option<PathBuf>`

- <span id="sourcecollector-copy-crate-source"></span>`fn copy_crate_source(source: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="sourcecollector-get-dev-only-packages"></span>`fn get_dev_only_packages(&self) -> HashSet<PackageId>`

- <span id="sourcecollector-dry-run-collect"></span>`fn dry_run_collect(&self, output_dir: &Path, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](collector/index.md#collectoptions), [`CollectionResult`](collector/index.md#collectionresult), [`Error`](../error/index.md#error)

- <span id="sourcecollector-update-gitignore"></span>`fn update_gitignore(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="sourcecollector-list-dependencies"></span>`fn list_dependencies(&self) -> Vec<(&str, &str)>`

- <span id="sourcecollector-copy-dir-recursive"></span>`fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Debug for SourceCollector`

- <span id="sourcecollector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SourceCollector`

##### `impl IntoEither for SourceCollector`

##### `impl OwoColorize for SourceCollector`

##### `impl Pointable for SourceCollector`

- <span id="sourcecollector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcecollector-pointable-type-init"></span>`type Init = T`

- <span id="sourcecollector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcecollector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcecollector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcecollector-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceCollector`

### `SourceManifest`

```rust
struct SourceManifest {
    pub collected_at: String,
    pub workspace_root: String,
    pub crates: std::collections::HashMap<String, CollectedCrate>,
}
```

*Defined in `src/source/collector.rs:41-50`*

Manifest stored in `.source_*/manifest.json`.

#### Fields

- **`collected_at`**: `String`

  When the sources were collected.

- **`workspace_root`**: `String`

  Workspace root path.

- **`crates`**: `std::collections::HashMap<String, CollectedCrate>`

  Collected crates by key "{name}-{version}".

#### Trait Implementations

##### `impl Debug for SourceManifest`

- <span id="sourcemanifest-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for SourceManifest`

- <span id="sourcemanifest-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for SourceManifest`

##### `impl Instrument for SourceManifest`

##### `impl IntoEither for SourceManifest`

##### `impl OwoColorize for SourceManifest`

##### `impl Pointable for SourceManifest`

- <span id="sourcemanifest-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcemanifest-pointable-type-init"></span>`type Init = T`

- <span id="sourcemanifest-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcemanifest-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcemanifest-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcemanifest-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SourceManifest`

- <span id="sourcemanifest-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl WithSubscriber for SourceManifest`

### `SourceLocator`

```rust
struct SourceLocator {
    registry_path: std::path::PathBuf,
    metadata: Option<cargo_metadata::Metadata>,
}
```

*Defined in `src/source/locator.rs:17-24`*

Locates crate sources in the Cargo registry.

The locator can find sources either by scanning the registry directly
or by using `cargo metadata` to find exact paths for dependencies.

#### Fields

- **`registry_path`**: `std::path::PathBuf`

  Path to the cargo registry source directory.
  Typically `~/.cargo/registry/src/`.

- **`metadata`**: `Option<cargo_metadata::Metadata>`

  Cached cargo metadata (if loaded from a project).

#### Implementations

- <span id="sourcelocator-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-with-registry-path"></span>`const fn with_registry_path(registry_path: PathBuf) -> Self`

- <span id="sourcelocator-load-metadata"></span>`fn load_metadata(&mut self, manifest_path: &Path) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-load-metadata-from-current-dir"></span>`fn load_metadata_from_current_dir(&mut self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-locate"></span>`fn locate(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-locate-from-metadata"></span>`fn locate_from_metadata(metadata: &Metadata, name: &str, version: &str) -> Option<PathBuf>`

- <span id="sourcelocator-locate-in-registry"></span>`fn locate_in_registry(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-packages"></span>`fn packages(&self) -> Option<&[Package]>`

- <span id="sourcelocator-workspace-root"></span>`fn workspace_root(&self) -> Option<&Path>`

- <span id="sourcelocator-all-dependency-sources"></span>`fn all_dependency_sources(&self) -> Result<Vec<(String, String, PathBuf)>, Error>` — [`Error`](../error/index.md#error)

- <span id="sourcelocator-list-registry-crates"></span>`fn list_registry_crates(&self) -> Result<Vec<(String, String)>, Error>` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Debug for SourceLocator`

- <span id="sourcelocator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SourceLocator`

##### `impl IntoEither for SourceLocator`

##### `impl OwoColorize for SourceLocator`

##### `impl Pointable for SourceLocator`

- <span id="sourcelocator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcelocator-pointable-type-init"></span>`type Init = T`

- <span id="sourcelocator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcelocator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcelocator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcelocator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceLocator`

### `SourceParser`

```rust
struct SourceParser {
    crate_name: String,
    crate_version: String,
    crate_root: std::path::PathBuf,
}
```

*Defined in `src/source/parser.rs:32-41`*

Parser for Rust source code using `syn`.

#### Fields

- **`crate_name`**: `String`

  The crate name being parsed.

- **`crate_version`**: `String`

  The crate version.

- **`crate_root`**: `std::path::PathBuf`

  Root path of the crate.

#### Implementations

- <span id="sourceparser-new"></span>`const fn new(name: String, version: String, root_path: PathBuf) -> Self`

- <span id="sourceparser-parse-crate"></span>`fn parse_crate(&self) -> Result<CrateSource, Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

- <span id="sourceparser-find-entry-point"></span>`fn find_entry_point(&self) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

- <span id="sourceparser-parse-module-file"></span>`fn parse_module_file(&self, path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

- <span id="sourceparser-process-file"></span>`fn process_file(&self, file: &File, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

- <span id="sourceparser-process-item"></span>`fn process_item(&self, item: &Item, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

- <span id="sourceparser-process-module"></span>`fn process_module(&self, module: &ItemMod, current_file: &Path, parent_module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

- <span id="sourceparser-find-module-file"></span>`fn find_module_file(&self, current_file: &Path, module_name: &str) -> Option<PathBuf>`

- <span id="sourceparser-extract-function"></span>`fn extract_function(&self, func: &ItemFn, file_path: &Path, module_path: &str) -> FunctionInfo` — [`FunctionInfo`](types/index.md#functioninfo)

- <span id="sourceparser-extract-struct"></span>`fn extract_struct(&self, s: &ItemStruct, file_path: &Path, module_path: &str) -> StructInfo` — [`StructInfo`](types/index.md#structinfo)

- <span id="sourceparser-extract-enum"></span>`fn extract_enum(&self, e: &ItemEnum, file_path: &Path, module_path: &str) -> EnumInfo` — [`EnumInfo`](types/index.md#enuminfo)

- <span id="sourceparser-extract-trait"></span>`fn extract_trait(&self, t: &ItemTrait, file_path: &Path, module_path: &str) -> TraitInfo` — [`TraitInfo`](types/index.md#traitinfo)

- <span id="sourceparser-extract-impl"></span>`fn extract_impl(&self, impl_block: &ItemImpl, file_path: &Path, module_path: &str) -> ImplInfo` — [`ImplInfo`](types/index.md#implinfo)

- <span id="sourceparser-extract-const"></span>`fn extract_const(&self, c: &ItemConst, file_path: &Path, module_path: &str) -> ConstInfo` — [`ConstInfo`](types/index.md#constinfo)

- <span id="sourceparser-extract-static"></span>`fn extract_static(&self, s: &ItemStatic, file_path: &Path, module_path: &str) -> StaticInfo` — [`StaticInfo`](types/index.md#staticinfo)

- <span id="sourceparser-extract-type-alias"></span>`fn extract_type_alias(&self, t: &ItemType, file_path: &Path, module_path: &str) -> TypeAliasInfo` — [`TypeAliasInfo`](types/index.md#typealiasinfo)

- <span id="sourceparser-line-of"></span>`fn line_of<T: Spanned>(item: &T) -> usize`

- <span id="sourceparser-extract-doc-comments"></span>`fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String>`

- <span id="sourceparser-extract-fields"></span>`fn extract_fields(fields: &Fields) -> Vec<FieldInfo>` — [`FieldInfo`](types/index.md#fieldinfo)

- <span id="sourceparser-parse-file"></span>`fn parse_file(path: &Path) -> Result<File, Error>` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Debug for SourceParser`

- <span id="sourceparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceParser`

- <span id="sourceparser-default"></span>`fn default() -> SourceParser` — [`SourceParser`](parser/index.md#sourceparser)

##### `impl Instrument for SourceParser`

##### `impl IntoEither for SourceParser`

##### `impl OwoColorize for SourceParser`

##### `impl Pointable for SourceParser`

- <span id="sourceparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceparser-pointable-type-init"></span>`type Init = T`

- <span id="sourceparser-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceparser-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceparser-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceparser-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceParser`

### `ConstInfo`

```rust
struct ConstInfo {
    pub name: String,
    pub module_path: String,
    pub ty: String,
    pub value: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:170-194`*

Information about a constant.

#### Fields

- **`name`**: `String`

  The constant name.

- **`module_path`**: `String`

  Full module path.

- **`ty`**: `String`

  The type of the constant.

- **`value`**: `String`

  The value expression as source code.

- **`is_public`**: `bool`

  Whether this constant is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for ConstInfo`

- <span id="constinfo-clone"></span>`fn clone(&self) -> ConstInfo` — [`ConstInfo`](types/index.md#constinfo)

##### `impl Debug for ConstInfo`

- <span id="constinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for ConstInfo`

##### `impl IntoEither for ConstInfo`

##### `impl OwoColorize for ConstInfo`

##### `impl Pointable for ConstInfo`

- <span id="constinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="constinfo-pointable-type-init"></span>`type Init = T`

- <span id="constinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="constinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="constinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="constinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ConstInfo`

### `CrateSource`

```rust
struct CrateSource {
    pub name: String,
    pub version: String,
    pub root_path: std::path::PathBuf,
    pub functions: Vec<FunctionInfo>,
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
    pub traits: Vec<TraitInfo>,
    pub impls: Vec<ImplInfo>,
    pub constants: Vec<ConstInfo>,
    pub statics: Vec<StaticInfo>,
    pub type_aliases: Vec<TypeAliasInfo>,
    pub macros: Vec<MacroInfo>,
}
```

*Defined in `src/source/types.rs:276-312`*

Aggregated source information for an entire crate.

#### Fields

- **`name`**: `String`

  Crate name.

- **`version`**: `String`

  Crate version (from Cargo.toml).

- **`root_path`**: `std::path::PathBuf`

  Root path of the crate source.

- **`functions`**: `Vec<FunctionInfo>`

  All parsed functions (including methods).

- **`structs`**: `Vec<StructInfo>`

  All parsed structs.

- **`enums`**: `Vec<EnumInfo>`

  All parsed enums.

- **`traits`**: `Vec<TraitInfo>`

  All parsed traits.

- **`impls`**: `Vec<ImplInfo>`

  All parsed impl blocks.

- **`constants`**: `Vec<ConstInfo>`

  All parsed constants.

- **`statics`**: `Vec<StaticInfo>`

  All parsed statics.

- **`type_aliases`**: `Vec<TypeAliasInfo>`

  All parsed type aliases.

- **`macros`**: `Vec<MacroInfo>`

  All parsed macro definitions.

#### Implementations

- <span id="cratesource-new"></span>`fn new(name: String, version: String, root_path: PathBuf) -> Self`

- <span id="cratesource-find-function"></span>`fn find_function(&self, path: &str) -> Option<&FunctionInfo>` — [`FunctionInfo`](types/index.md#functioninfo)

- <span id="cratesource-find-struct"></span>`fn find_struct(&self, path: &str) -> Option<&StructInfo>` — [`StructInfo`](types/index.md#structinfo)

- <span id="cratesource-find-enum"></span>`fn find_enum(&self, path: &str) -> Option<&EnumInfo>` — [`EnumInfo`](types/index.md#enuminfo)

- <span id="cratesource-find-trait"></span>`fn find_trait(&self, path: &str) -> Option<&TraitInfo>` — [`TraitInfo`](types/index.md#traitinfo)

- <span id="cratesource-find-const"></span>`fn find_const(&self, path: &str) -> Option<&ConstInfo>` — [`ConstInfo`](types/index.md#constinfo)

- <span id="cratesource-find-static"></span>`fn find_static(&self, path: &str) -> Option<&StaticInfo>` — [`StaticInfo`](types/index.md#staticinfo)

- <span id="cratesource-private-items-in-module"></span>`fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>>` — [`PrivateItem`](types/index.md#privateitem)

#### Trait Implementations

##### `impl Debug for CrateSource`

- <span id="cratesource-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateSource`

- <span id="cratesource-default"></span>`fn default() -> CrateSource` — [`CrateSource`](types/index.md#cratesource)

##### `impl Instrument for CrateSource`

##### `impl IntoEither for CrateSource`

##### `impl OwoColorize for CrateSource`

##### `impl Pointable for CrateSource`

- <span id="cratesource-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratesource-pointable-type-init"></span>`type Init = T`

- <span id="cratesource-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratesource-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratesource-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratesource-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for CrateSource`

### `EnumInfo`

```rust
struct EnumInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
    pub variants: Vec<VariantInfo>,
}
```

*Defined in `src/source/types.rs:82-106`*

Information about a parsed enum.

#### Fields

- **`name`**: `String`

  The enum name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full enum definition as source code.

- **`is_public`**: `bool`

  Whether this enum is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

- **`variants`**: `Vec<VariantInfo>`

  Variant information.

#### Trait Implementations

##### `impl Clone for EnumInfo`

- <span id="enuminfo-clone"></span>`fn clone(&self) -> EnumInfo` — [`EnumInfo`](types/index.md#enuminfo)

##### `impl Debug for EnumInfo`

- <span id="enuminfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for EnumInfo`

##### `impl IntoEither for EnumInfo`

##### `impl OwoColorize for EnumInfo`

##### `impl Pointable for EnumInfo`

- <span id="enuminfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enuminfo-pointable-type-init"></span>`type Init = T`

- <span id="enuminfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enuminfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enuminfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enuminfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for EnumInfo`

### `FieldInfo`

```rust
struct FieldInfo {
    pub name: Option<String>,
    pub ty: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
}
```

*Defined in `src/source/types.rs:66-78`*

Information about a struct or enum field.

#### Fields

- **`name`**: `Option<String>`

  Field name (None for tuple struct fields).

- **`ty`**: `String`

  Field type as a string.

- **`is_public`**: `bool`

  Whether this field is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments for this field.

#### Trait Implementations

##### `impl Clone for FieldInfo`

- <span id="fieldinfo-clone"></span>`fn clone(&self) -> FieldInfo` — [`FieldInfo`](types/index.md#fieldinfo)

##### `impl Debug for FieldInfo`

- <span id="fieldinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for FieldInfo`

##### `impl IntoEither for FieldInfo`

##### `impl OwoColorize for FieldInfo`

##### `impl Pointable for FieldInfo`

- <span id="fieldinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fieldinfo-pointable-type-init"></span>`type Init = T`

- <span id="fieldinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fieldinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fieldinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fieldinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for FieldInfo`

### `FunctionInfo`

```rust
struct FunctionInfo {
    pub name: String,
    pub module_path: String,
    pub signature: String,
    pub body: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:10-34`*

Information about a parsed function, including its body.

#### Fields

- **`name`**: `String`

  The function name.

- **`module_path`**: `String`

  Full module path (e.g., `crate::module::submodule`).

- **`signature`**: `String`

  The function signature as a string.

- **`body`**: `String`

  The function body as source code.

- **`is_public`**: `bool`

  Whether this function is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments extracted from `///` or `//!` attributes.

- **`source_file`**: `std::path::PathBuf`

  Source file where this function is defined.

- **`line_number`**: `usize`

  Line number where the function starts.

#### Trait Implementations

##### `impl Clone for FunctionInfo`

- <span id="functioninfo-clone"></span>`fn clone(&self) -> FunctionInfo` — [`FunctionInfo`](types/index.md#functioninfo)

##### `impl Debug for FunctionInfo`

- <span id="functioninfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for FunctionInfo`

##### `impl IntoEither for FunctionInfo`

##### `impl OwoColorize for FunctionInfo`

##### `impl Pointable for FunctionInfo`

- <span id="functioninfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="functioninfo-pointable-type-init"></span>`type Init = T`

- <span id="functioninfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="functioninfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="functioninfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="functioninfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for FunctionInfo`

### `ImplInfo`

```rust
struct ImplInfo {
    pub self_ty: String,
    pub trait_name: Option<String>,
    pub module_path: String,
    pub methods: Vec<FunctionInfo>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:148-166`*

Information about an impl block.

#### Fields

- **`self_ty`**: `String`

  The type being implemented for (e.g., `MyStruct`).

- **`trait_name`**: `Option<String>`

  The trait being implemented (if any).

- **`module_path`**: `String`

  Full module path where this impl is defined.

- **`methods`**: `Vec<FunctionInfo>`

  Methods in this impl block.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for ImplInfo`

- <span id="implinfo-clone"></span>`fn clone(&self) -> ImplInfo` — [`ImplInfo`](types/index.md#implinfo)

##### `impl Debug for ImplInfo`

- <span id="implinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for ImplInfo`

##### `impl IntoEither for ImplInfo`

##### `impl OwoColorize for ImplInfo`

##### `impl Pointable for ImplInfo`

- <span id="implinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implinfo-pointable-type-init"></span>`type Init = T`

- <span id="implinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ImplInfo`

### `MacroInfo`

```rust
struct MacroInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:254-272`*

Information about a macro definition.

#### Fields

- **`name`**: `String`

  The macro name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full macro definition as source code.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for MacroInfo`

- <span id="macroinfo-clone"></span>`fn clone(&self) -> MacroInfo` — [`MacroInfo`](types/index.md#macroinfo)

##### `impl Debug for MacroInfo`

- <span id="macroinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for MacroInfo`

##### `impl IntoEither for MacroInfo`

##### `impl OwoColorize for MacroInfo`

##### `impl Pointable for MacroInfo`

- <span id="macroinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="macroinfo-pointable-type-init"></span>`type Init = T`

- <span id="macroinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="macroinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="macroinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="macroinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MacroInfo`

### `StaticInfo`

```rust
struct StaticInfo {
    pub name: String,
    pub module_path: String,
    pub ty: String,
    pub value: String,
    pub is_mutable: bool,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:198-225`*

Information about a static variable.

#### Fields

- **`name`**: `String`

  The static name.

- **`module_path`**: `String`

  Full module path.

- **`ty`**: `String`

  The type of the static.

- **`value`**: `String`

  The value expression as source code.

- **`is_mutable`**: `bool`

  Whether this static is mutable.

- **`is_public`**: `bool`

  Whether this static is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for StaticInfo`

- <span id="staticinfo-clone"></span>`fn clone(&self) -> StaticInfo` — [`StaticInfo`](types/index.md#staticinfo)

##### `impl Debug for StaticInfo`

- <span id="staticinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for StaticInfo`

##### `impl IntoEither for StaticInfo`

##### `impl OwoColorize for StaticInfo`

##### `impl Pointable for StaticInfo`

- <span id="staticinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="staticinfo-pointable-type-init"></span>`type Init = T`

- <span id="staticinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="staticinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="staticinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="staticinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for StaticInfo`

### `StructInfo`

```rust
struct StructInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
    pub fields: Vec<FieldInfo>,
}
```

*Defined in `src/source/types.rs:38-62`*

Information about a parsed struct.

#### Fields

- **`name`**: `String`

  The struct name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full struct definition as source code.

- **`is_public`**: `bool`

  Whether this struct is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

- **`fields`**: `Vec<FieldInfo>`

  Field information (for struct fields).

#### Trait Implementations

##### `impl Clone for StructInfo`

- <span id="structinfo-clone"></span>`fn clone(&self) -> StructInfo` — [`StructInfo`](types/index.md#structinfo)

##### `impl Debug for StructInfo`

- <span id="structinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for StructInfo`

##### `impl IntoEither for StructInfo`

##### `impl OwoColorize for StructInfo`

##### `impl Pointable for StructInfo`

- <span id="structinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="structinfo-pointable-type-init"></span>`type Init = T`

- <span id="structinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="structinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="structinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="structinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for StructInfo`

### `TraitInfo`

```rust
struct TraitInfo {
    pub name: String,
    pub module_path: String,
    pub definition: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:123-144`*

Information about a parsed trait.

#### Fields

- **`name`**: `String`

  The trait name.

- **`module_path`**: `String`

  Full module path.

- **`definition`**: `String`

  The full trait definition as source code.

- **`is_public`**: `bool`

  Whether this trait is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for TraitInfo`

- <span id="traitinfo-clone"></span>`fn clone(&self) -> TraitInfo` — [`TraitInfo`](types/index.md#traitinfo)

##### `impl Debug for TraitInfo`

- <span id="traitinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TraitInfo`

##### `impl IntoEither for TraitInfo`

##### `impl OwoColorize for TraitInfo`

##### `impl Pointable for TraitInfo`

- <span id="traitinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitinfo-pointable-type-init"></span>`type Init = T`

- <span id="traitinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TraitInfo`

### `TypeAliasInfo`

```rust
struct TypeAliasInfo {
    pub name: String,
    pub module_path: String,
    pub aliased_type: String,
    pub is_public: bool,
    pub doc_comments: Vec<String>,
    pub source_file: std::path::PathBuf,
    pub line_number: usize,
}
```

*Defined in `src/source/types.rs:229-250`*

Information about a type alias.

#### Fields

- **`name`**: `String`

  The type alias name.

- **`module_path`**: `String`

  Full module path.

- **`aliased_type`**: `String`

  The aliased type as a string.

- **`is_public`**: `bool`

  Whether this type alias is public.

- **`doc_comments`**: `Vec<String>`

  Doc comments.

- **`source_file`**: `std::path::PathBuf`

  Source file location.

- **`line_number`**: `usize`

  Line number.

#### Trait Implementations

##### `impl Clone for TypeAliasInfo`

- <span id="typealiasinfo-clone"></span>`fn clone(&self) -> TypeAliasInfo` — [`TypeAliasInfo`](types/index.md#typealiasinfo)

##### `impl Debug for TypeAliasInfo`

- <span id="typealiasinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TypeAliasInfo`

##### `impl IntoEither for TypeAliasInfo`

##### `impl OwoColorize for TypeAliasInfo`

##### `impl Pointable for TypeAliasInfo`

- <span id="typealiasinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="typealiasinfo-pointable-type-init"></span>`type Init = T`

- <span id="typealiasinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typealiasinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typealiasinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typealiasinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TypeAliasInfo`

### `VariantInfo`

```rust
struct VariantInfo {
    pub name: String,
    pub doc_comments: Vec<String>,
    pub fields: Vec<FieldInfo>,
}
```

*Defined in `src/source/types.rs:110-119`*

Information about an enum variant.

#### Fields

- **`name`**: `String`

  Variant name.

- **`doc_comments`**: `Vec<String>`

  Doc comments for this variant.

- **`fields`**: `Vec<FieldInfo>`

  Fields (for tuple or struct variants).

#### Trait Implementations

##### `impl Clone for VariantInfo`

- <span id="variantinfo-clone"></span>`fn clone(&self) -> VariantInfo` — [`VariantInfo`](types/index.md#variantinfo)

##### `impl Debug for VariantInfo`

- <span id="variantinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for VariantInfo`

##### `impl IntoEither for VariantInfo`

##### `impl OwoColorize for VariantInfo`

##### `impl Pointable for VariantInfo`

- <span id="variantinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="variantinfo-pointable-type-init"></span>`type Init = T`

- <span id="variantinfo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="variantinfo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="variantinfo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="variantinfo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for VariantInfo`

## Enums

### `PrivateItem<'a>`

```rust
enum PrivateItem<'a> {
    Function(&'a FunctionInfo),
    Struct(&'a StructInfo),
    Enum(&'a EnumInfo),
    Const(&'a ConstInfo),
    TypeAlias(&'a TypeAliasInfo),
}
```

*Defined in `src/source/types.rs:409-424`*

A reference to a private item for rendering.

#### Variants

- **`Function`**

  A private function.

- **`Struct`**

  A private struct.

- **`Enum`**

  A private enum.

- **`Const`**

  A private constant.

- **`TypeAlias`**

  A private type alias.

#### Trait Implementations

##### `impl Debug for PrivateItem<'a>`

- <span id="privateitem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for PrivateItem<'a>`

##### `impl IntoEither for PrivateItem<'a>`

##### `impl OwoColorize for PrivateItem<'a>`

##### `impl Pointable for PrivateItem<'a>`

- <span id="privateitem-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privateitem-pointable-type-init"></span>`type Init = T`

- <span id="privateitem-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privateitem-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privateitem-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privateitem-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for PrivateItem<'a>`

## Functions

### `find_source_dir`

```rust
fn find_source_dir(root: &std::path::Path) -> Option<std::path::PathBuf>
```

*Defined in `src/source/mod.rs:85-108`*

Find the most recent `.source_*` directory in the given root.

Scans for directories matching `.source_*` pattern and returns
the one with the highest timestamp (most recent).

# Arguments

* `root` - Directory to search in (typically workspace or project root)

# Returns

Path to the most recent `.source_*` directory, or `None` if not found.

# Example

```no_run
use std::path::Path;
use cargo_docs_md::source::find_source_dir;

if let Some(source_dir) = find_source_dir(Path::new(".")) {
    println!("Found source directory: {}", source_dir.display());
}
```

### `extract_source_timestamp`

```rust
fn extract_source_timestamp(path: &std::path::Path) -> u64
```

*Defined in `src/source/mod.rs:113-119`*

Extract timestamp from a `.source_*` directory path.

Returns the numeric timestamp suffix, or 0 if parsing fails.

