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
    pub minimal_sources: bool,
    pub no_gitignore: bool,
}
```

*Defined in `src/source/collector.rs:67-86`*

Options for source collection.

#### Fields

- **`include_dev`**: `bool`

  Include dev-dependencies.

- **`output`**: `Option<std::path::PathBuf>`

  Custom output directory (overrides timestamp-based naming).

- **`dry_run`**: `bool`

  Dry run - don't actually copy files.

- **`minimal_sources`**: `bool`

  Only copy `src/` directory and `Cargo.toml` (minimal mode).
  
  By default (false), the entire crate directory is copied to ensure
  all source files are available (including `build.rs`, modules outside
  `src/`, etc.).

- **`no_gitignore`**: `bool`

  Skip adding `.source_*` pattern to `.gitignore`.

#### Trait Implementations

##### `impl Any for CollectOptions`

- <span id="collectoptions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectOptions`

- <span id="collectoptions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectOptions`

- <span id="collectoptions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CollectOptions`

- <span id="collectoptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CollectOptions`

- <span id="collectoptions-default"></span>`fn default() -> CollectOptions` — [`CollectOptions`](collector/index.md#collectoptions)

##### `impl<T> From for CollectOptions`

- <span id="collectoptions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CollectOptions`

##### `impl<U> Into for CollectOptions`

- <span id="collectoptions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CollectOptions`

##### `impl OwoColorize for CollectOptions`

##### `impl Pointable for CollectOptions`

- <span id="collectoptions-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectoptions-pointable-type-init"></span>`type Init = T`

- <span id="collectoptions-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectoptions-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectoptions-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectoptions-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CollectOptions`

- <span id="collectoptions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectoptions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CollectOptions`

- <span id="collectoptions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectoptions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for CollectedCrate`

- <span id="collectedcrate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectedCrate`

- <span id="collectedcrate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectedCrate`

- <span id="collectedcrate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CollectedCrate`

- <span id="collectedcrate-clone"></span>`fn clone(&self) -> CollectedCrate` — [`CollectedCrate`](collector/index.md#collectedcrate)

##### `impl CloneToUninit for CollectedCrate`

- <span id="collectedcrate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CollectedCrate`

- <span id="collectedcrate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for CollectedCrate`

- <span id="collectedcrate-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for CollectedCrate`

##### `impl<T> From for CollectedCrate`

- <span id="collectedcrate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CollectedCrate`

##### `impl<U> Into for CollectedCrate`

- <span id="collectedcrate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CollectedCrate`

##### `impl OwoColorize for CollectedCrate`

##### `impl Pointable for CollectedCrate`

- <span id="collectedcrate-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectedcrate-pointable-type-init"></span>`type Init = T`

- <span id="collectedcrate-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectedcrate-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectedcrate-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectedcrate-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for CollectedCrate`

- <span id="collectedcrate-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl ToOwned for CollectedCrate`

- <span id="collectedcrate-toowned-type-owned"></span>`type Owned = T`

- <span id="collectedcrate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="collectedcrate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CollectedCrate`

- <span id="collectedcrate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectedcrate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CollectedCrate`

- <span id="collectedcrate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectedcrate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for CollectionResult`

- <span id="collectionresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectionResult`

- <span id="collectionresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectionResult`

- <span id="collectionresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CollectionResult`

- <span id="collectionresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CollectionResult`

- <span id="collectionresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CollectionResult`

##### `impl<U> Into for CollectionResult`

- <span id="collectionresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CollectionResult`

##### `impl OwoColorize for CollectionResult`

##### `impl Pointable for CollectionResult`

- <span id="collectionresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectionresult-pointable-type-init"></span>`type Init = T`

- <span id="collectionresult-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectionresult-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectionresult-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectionresult-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CollectionResult`

- <span id="collectionresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectionresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CollectionResult`

- <span id="collectionresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectionresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CollectionResult`

### `SourceCollector`

```rust
struct SourceCollector {
    metadata: cargo_metadata::Metadata,
    registry_path: std::path::PathBuf,
}
```

*Defined in `src/source/collector.rs:90-96`*

Collector for gathering dependency sources.

#### Fields

- **`metadata`**: `cargo_metadata::Metadata`

  Cargo metadata for the workspace.

- **`registry_path`**: `std::path::PathBuf`

  Path to cargo registry sources.

#### Implementations

- <span id="sourcecollector-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../error/index.md#error)

  Create a new collector for the current directory.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcecollector-from-manifest"></span>`fn from_manifest(manifest_path: Option<&Path>) -> Result<Self, Error>` — [`Error`](../error/index.md#error)

  Create a new collector from a specific manifest path.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcecollector-collect"></span>`fn collect(&self, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](collector/index.md#collectoptions), [`CollectionResult`](collector/index.md#collectionresult), [`Error`](../error/index.md#error)

  Collect all dependency sources.

  

  # Errors

  

  Returns an error if collection fails.

- <span id="sourcecollector-generate-output-dir"></span>`fn generate_output_dir(&self) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

  Generate a timestamp-based output directory name.

- <span id="sourcecollector-find-registry-source"></span>`fn find_registry_source(&self, name: &str, version: &str) -> Option<PathBuf>`

  Find a crate's source in the cargo registry.

- <span id="sourcecollector-copy-crate-source"></span>`fn copy_crate_source(source: &Path, dest: &Path, minimal: bool) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Copy crate source to destination.

  

  If `minimal` is false (default), copies the entire crate directory.

  If `minimal` is true, only copies `src/` and `Cargo.toml`.

  

  In both modes, `Cargo.toml` is renamed to `Crate.toml` to avoid

  confusing cargo when the collected sources are in the workspace.

- <span id="sourcecollector-get-dev-only-packages"></span>`fn get_dev_only_packages(&self) -> HashSet<PackageId>`

  Get the set of package IDs that are dev-only dependencies.

  

  A package is considered dev-only if it is only reachable from workspace

  members via dev-dependencies (not normal or build dependencies).

- <span id="sourcecollector-dry-run-collect"></span>`fn dry_run_collect(&self, output_dir: &Path, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](collector/index.md#collectoptions), [`CollectionResult`](collector/index.md#collectionresult), [`Error`](../error/index.md#error)

  Perform a dry run, returning what would be collected.

- <span id="sourcecollector-update-gitignore"></span>`fn update_gitignore(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Update .gitignore to include .source_* pattern.

- <span id="sourcecollector-list-dependencies"></span>`fn list_dependencies(&self) -> Vec<(&str, &str)>`

  List all external dependencies.

- <span id="sourcecollector-copy-dir-recursive"></span>`fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Recursively copy a directory.

#### Trait Implementations

##### `impl Any for SourceCollector`

- <span id="sourcecollector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceCollector`

- <span id="sourcecollector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceCollector`

- <span id="sourcecollector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceCollector`

- <span id="sourcecollector-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SourceCollector`

- <span id="sourcecollector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceCollector`

##### `impl<U> Into for SourceCollector`

- <span id="sourcecollector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceCollector`

##### `impl OwoColorize for SourceCollector`

##### `impl Pointable for SourceCollector`

- <span id="sourcecollector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcecollector-pointable-type-init"></span>`type Init = T`

- <span id="sourcecollector-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcecollector-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcecollector-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcecollector-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SourceCollector`

- <span id="sourcecollector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcecollector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceCollector`

- <span id="sourcecollector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcecollector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for SourceManifest`

- <span id="sourcemanifest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceManifest`

- <span id="sourcemanifest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceManifest`

- <span id="sourcemanifest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceManifest`

- <span id="sourcemanifest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for SourceManifest`

- <span id="sourcemanifest-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for SourceManifest`

##### `impl<T> From for SourceManifest`

- <span id="sourcemanifest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceManifest`

##### `impl<U> Into for SourceManifest`

- <span id="sourcemanifest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceManifest`

##### `impl OwoColorize for SourceManifest`

##### `impl Pointable for SourceManifest`

- <span id="sourcemanifest-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcemanifest-pointable-type-init"></span>`type Init = T`

- <span id="sourcemanifest-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcemanifest-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcemanifest-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcemanifest-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SourceManifest`

- <span id="sourcemanifest-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<U> TryFrom for SourceManifest`

- <span id="sourcemanifest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcemanifest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceManifest`

- <span id="sourcemanifest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcemanifest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new `SourceLocator` with the default registry path.

  

  # Errors

  

  Returns an error if the home directory cannot be determined.

- <span id="sourcelocator-with-registry-path"></span>`const fn with_registry_path(registry_path: PathBuf) -> Self`

  Create a `SourceLocator` with a custom registry path.

- <span id="sourcelocator-load-metadata"></span>`fn load_metadata(&mut self, manifest_path: &Path) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Load cargo metadata from a project directory.

  

  This enables more accurate source location by using the exact

  paths from cargo's dependency resolution.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcelocator-load-metadata-from-current-dir"></span>`fn load_metadata_from_current_dir(&mut self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Load cargo metadata from the current directory.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcelocator-locate"></span>`fn locate(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

  Locate the source directory for a crate by name and version.

  

  First tries to use cargo metadata if available, then falls back

  to scanning the registry directory.

  

  # Errors

  

  Returns an error if the source cannot be found.

- <span id="sourcelocator-locate-from-metadata"></span>`fn locate_from_metadata(metadata: &Metadata, name: &str, version: &str) -> Option<PathBuf>`

  Locate source using cargo metadata.

- <span id="sourcelocator-locate-in-registry"></span>`fn locate_in_registry(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

  Locate source by scanning the registry directory.

- <span id="sourcelocator-packages"></span>`fn packages(&self) -> Option<&[Package]>`

  Get all packages from the loaded metadata.

  

  Returns `None` if metadata hasn't been loaded.

- <span id="sourcelocator-workspace-root"></span>`fn workspace_root(&self) -> Option<&Path>`

  Get the workspace root from loaded metadata.

  

  Returns `None` if metadata hasn't been loaded.

- <span id="sourcelocator-all-dependency-sources"></span>`fn all_dependency_sources(&self) -> Result<Vec<(String, String, PathBuf)>, Error>` — [`Error`](../error/index.md#error)

  Find all dependency sources for a workspace.

  

  Returns a list of (name, version, path) tuples for all dependencies

  that have sources in the registry.

  

  # Errors

  

  Returns an error if metadata hasn't been loaded.

- <span id="sourcelocator-list-registry-crates"></span>`fn list_registry_crates(&self) -> Result<Vec<(String, String)>, Error>` — [`Error`](../error/index.md#error)

  List all available crate versions in the registry.

  

  Returns a list of (name, version) tuples.

  

  # Errors

  

  Returns an error if the registry cannot be read.

#### Trait Implementations

##### `impl Any for SourceLocator`

- <span id="sourcelocator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceLocator`

- <span id="sourcelocator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceLocator`

- <span id="sourcelocator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceLocator`

- <span id="sourcelocator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SourceLocator`

- <span id="sourcelocator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceLocator`

##### `impl<U> Into for SourceLocator`

- <span id="sourcelocator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceLocator`

##### `impl OwoColorize for SourceLocator`

##### `impl Pointable for SourceLocator`

- <span id="sourcelocator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourcelocator-pointable-type-init"></span>`type Init = T`

- <span id="sourcelocator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourcelocator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourcelocator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourcelocator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SourceLocator`

- <span id="sourcelocator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcelocator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceLocator`

- <span id="sourcelocator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcelocator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new source parser for a crate.

- <span id="sourceparser-parse-crate"></span>`fn parse_crate(&self) -> Result<CrateSource, Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

  Parse an entire crate starting from its root.

  

  # Errors

  

  Returns an error if any source file cannot be parsed.

- <span id="sourceparser-find-entry-point"></span>`fn find_entry_point(&self) -> Result<PathBuf, Error>` — [`Error`](../error/index.md#error)

  Find the crate entry point (lib.rs or main.rs).

- <span id="sourceparser-parse-module-file"></span>`fn parse_module_file(&self, path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

  Parse a single module file and its submodules.

- <span id="sourceparser-process-file"></span>`fn process_file(&self, file: &File, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

  Process a parsed file, extracting items and following submodules.

- <span id="sourceparser-process-item"></span>`fn process_item(&self, item: &Item, file_path: &Path, module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

  Process a single item from a file.

- <span id="sourceparser-process-module"></span>`fn process_module(&self, module: &ItemMod, current_file: &Path, parent_module_path: &str, source: &mut CrateSource) -> Result<(), Error>` — [`CrateSource`](types/index.md#cratesource), [`Error`](../error/index.md#error)

  Process a module declaration, potentially following to an external file.

- <span id="sourceparser-find-module-file"></span>`fn find_module_file(&self, current_file: &Path, module_name: &str) -> Option<PathBuf>`

  Find the file for an external module declaration.

- <span id="sourceparser-extract-function"></span>`fn extract_function(&self, func: &ItemFn, file_path: &Path, module_path: &str) -> FunctionInfo` — [`FunctionInfo`](types/index.md#functioninfo)

  Extract function information.

- <span id="sourceparser-extract-struct"></span>`fn extract_struct(&self, s: &ItemStruct, file_path: &Path, module_path: &str) -> StructInfo` — [`StructInfo`](types/index.md#structinfo)

  Extract struct information.

- <span id="sourceparser-extract-enum"></span>`fn extract_enum(&self, e: &ItemEnum, file_path: &Path, module_path: &str) -> EnumInfo` — [`EnumInfo`](types/index.md#enuminfo)

  Extract enum information.

- <span id="sourceparser-extract-trait"></span>`fn extract_trait(&self, t: &ItemTrait, file_path: &Path, module_path: &str) -> TraitInfo` — [`TraitInfo`](types/index.md#traitinfo)

  Extract trait information.

- <span id="sourceparser-extract-impl"></span>`fn extract_impl(&self, impl_block: &ItemImpl, file_path: &Path, module_path: &str) -> ImplInfo` — [`ImplInfo`](types/index.md#implinfo)

  Extract impl block information.

- <span id="sourceparser-extract-const"></span>`fn extract_const(&self, c: &ItemConst, file_path: &Path, module_path: &str) -> ConstInfo` — [`ConstInfo`](types/index.md#constinfo)

  Extract constant information.

- <span id="sourceparser-extract-static"></span>`fn extract_static(&self, s: &ItemStatic, file_path: &Path, module_path: &str) -> StaticInfo` — [`StaticInfo`](types/index.md#staticinfo)

  Extract static information.

- <span id="sourceparser-extract-type-alias"></span>`fn extract_type_alias(&self, t: &ItemType, file_path: &Path, module_path: &str) -> TypeAliasInfo` — [`TypeAliasInfo`](types/index.md#typealiasinfo)

  Extract type alias information.

- <span id="sourceparser-line-of"></span>`fn line_of<T: Spanned>(item: &T) -> usize`

  Extract the starting line number from a spanned item.

  

  Uses `proc-macro2`'s span-locations feature to get accurate line numbers.

- <span id="sourceparser-extract-doc-comments"></span>`fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String>`

  Extract doc comments from attributes.

  

  Doc comments in Rust are represented as `#[doc = "..."]` attributes.

- <span id="sourceparser-extract-fields"></span>`fn extract_fields(fields: &Fields) -> Vec<FieldInfo>` — [`FieldInfo`](types/index.md#fieldinfo)

  Extract field information from struct/enum fields.

- <span id="sourceparser-parse-file"></span>`fn parse_file(path: &Path) -> Result<File, Error>` — [`Error`](../error/index.md#error)

  Parse a single file without traversing modules.

  

  Useful for quick parsing of individual files.

  

  # Errors

  

  Returns an error if the file cannot be read or parsed.

#### Trait Implementations

##### `impl Any for SourceParser`

- <span id="sourceparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceParser`

- <span id="sourceparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceParser`

- <span id="sourceparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceParser`

- <span id="sourceparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceParser`

- <span id="sourceparser-default"></span>`fn default() -> SourceParser` — [`SourceParser`](parser/index.md#sourceparser)

##### `impl<T> From for SourceParser`

- <span id="sourceparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceParser`

##### `impl<U> Into for SourceParser`

- <span id="sourceparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceParser`

##### `impl OwoColorize for SourceParser`

##### `impl Pointable for SourceParser`

- <span id="sourceparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceparser-pointable-type-init"></span>`type Init = T`

- <span id="sourceparser-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceparser-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceparser-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceparser-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SourceParser`

- <span id="sourceparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceParser`

- <span id="sourceparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ConstInfo`

- <span id="constinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstInfo`

- <span id="constinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstInfo`

- <span id="constinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ConstInfo`

- <span id="constinfo-clone"></span>`fn clone(&self) -> ConstInfo` — [`ConstInfo`](types/index.md#constinfo)

##### `impl CloneToUninit for ConstInfo`

- <span id="constinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ConstInfo`

- <span id="constinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ConstInfo`

- <span id="constinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ConstInfo`

##### `impl<U> Into for ConstInfo`

- <span id="constinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ConstInfo`

##### `impl OwoColorize for ConstInfo`

##### `impl Pointable for ConstInfo`

- <span id="constinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="constinfo-pointable-type-init"></span>`type Init = T`

- <span id="constinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="constinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="constinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="constinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ConstInfo`

- <span id="constinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="constinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ConstInfo`

- <span id="constinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstInfo`

- <span id="constinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new empty `CrateSource`.

- <span id="cratesource-find-function"></span>`fn find_function(&self, path: &str) -> Option<&FunctionInfo>` — [`FunctionInfo`](types/index.md#functioninfo)

  Look up a function by its full path.

- <span id="cratesource-find-struct"></span>`fn find_struct(&self, path: &str) -> Option<&StructInfo>` — [`StructInfo`](types/index.md#structinfo)

  Look up a struct by its full path.

- <span id="cratesource-find-enum"></span>`fn find_enum(&self, path: &str) -> Option<&EnumInfo>` — [`EnumInfo`](types/index.md#enuminfo)

  Look up an enum by its full path.

- <span id="cratesource-find-trait"></span>`fn find_trait(&self, path: &str) -> Option<&TraitInfo>` — [`TraitInfo`](types/index.md#traitinfo)

  Look up a trait by its full path.

- <span id="cratesource-find-const"></span>`fn find_const(&self, path: &str) -> Option<&ConstInfo>` — [`ConstInfo`](types/index.md#constinfo)

  Look up a constant by its full path.

- <span id="cratesource-find-static"></span>`fn find_static(&self, path: &str) -> Option<&StaticInfo>` — [`StaticInfo`](types/index.md#staticinfo)

  Look up a static by its full path.

- <span id="cratesource-private-items-in-module"></span>`fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>>` — [`PrivateItem`](types/index.md#privateitem)

  Get all private items (functions, structs, etc.) in a module.

#### Trait Implementations

##### `impl Any for CrateSource`

- <span id="cratesource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrateSource`

- <span id="cratesource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrateSource`

- <span id="cratesource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CrateSource`

- <span id="cratesource-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateSource`

- <span id="cratesource-default"></span>`fn default() -> CrateSource` — [`CrateSource`](types/index.md#cratesource)

##### `impl<T> From for CrateSource`

- <span id="cratesource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CrateSource`

##### `impl<U> Into for CrateSource`

- <span id="cratesource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CrateSource`

##### `impl OwoColorize for CrateSource`

##### `impl Pointable for CrateSource`

- <span id="cratesource-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratesource-pointable-type-init"></span>`type Init = T`

- <span id="cratesource-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratesource-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratesource-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratesource-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CrateSource`

- <span id="cratesource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cratesource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrateSource`

- <span id="cratesource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cratesource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for EnumInfo`

- <span id="enuminfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumInfo`

- <span id="enuminfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumInfo`

- <span id="enuminfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EnumInfo`

- <span id="enuminfo-clone"></span>`fn clone(&self) -> EnumInfo` — [`EnumInfo`](types/index.md#enuminfo)

##### `impl CloneToUninit for EnumInfo`

- <span id="enuminfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for EnumInfo`

- <span id="enuminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EnumInfo`

- <span id="enuminfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for EnumInfo`

##### `impl<U> Into for EnumInfo`

- <span id="enuminfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EnumInfo`

##### `impl OwoColorize for EnumInfo`

##### `impl Pointable for EnumInfo`

- <span id="enuminfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="enuminfo-pointable-type-init"></span>`type Init = T`

- <span id="enuminfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enuminfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enuminfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enuminfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for EnumInfo`

- <span id="enuminfo-toowned-type-owned"></span>`type Owned = T`

- <span id="enuminfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enuminfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EnumInfo`

- <span id="enuminfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enuminfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumInfo`

- <span id="enuminfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enuminfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FieldInfo`

- <span id="fieldinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldInfo`

- <span id="fieldinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldInfo`

- <span id="fieldinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FieldInfo`

- <span id="fieldinfo-clone"></span>`fn clone(&self) -> FieldInfo` — [`FieldInfo`](types/index.md#fieldinfo)

##### `impl CloneToUninit for FieldInfo`

- <span id="fieldinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FieldInfo`

- <span id="fieldinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FieldInfo`

- <span id="fieldinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for FieldInfo`

##### `impl<U> Into for FieldInfo`

- <span id="fieldinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FieldInfo`

##### `impl OwoColorize for FieldInfo`

##### `impl Pointable for FieldInfo`

- <span id="fieldinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fieldinfo-pointable-type-init"></span>`type Init = T`

- <span id="fieldinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fieldinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fieldinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fieldinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FieldInfo`

- <span id="fieldinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FieldInfo`

- <span id="fieldinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldInfo`

- <span id="fieldinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for FunctionInfo`

- <span id="functioninfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FunctionInfo`

- <span id="functioninfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FunctionInfo`

- <span id="functioninfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FunctionInfo`

- <span id="functioninfo-clone"></span>`fn clone(&self) -> FunctionInfo` — [`FunctionInfo`](types/index.md#functioninfo)

##### `impl CloneToUninit for FunctionInfo`

- <span id="functioninfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FunctionInfo`

- <span id="functioninfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FunctionInfo`

- <span id="functioninfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for FunctionInfo`

##### `impl<U> Into for FunctionInfo`

- <span id="functioninfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FunctionInfo`

##### `impl OwoColorize for FunctionInfo`

##### `impl Pointable for FunctionInfo`

- <span id="functioninfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="functioninfo-pointable-type-init"></span>`type Init = T`

- <span id="functioninfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="functioninfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="functioninfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="functioninfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FunctionInfo`

- <span id="functioninfo-toowned-type-owned"></span>`type Owned = T`

- <span id="functioninfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="functioninfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FunctionInfo`

- <span id="functioninfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="functioninfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FunctionInfo`

- <span id="functioninfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="functioninfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ImplInfo`

- <span id="implinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplInfo`

- <span id="implinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplInfo`

- <span id="implinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImplInfo`

- <span id="implinfo-clone"></span>`fn clone(&self) -> ImplInfo` — [`ImplInfo`](types/index.md#implinfo)

##### `impl CloneToUninit for ImplInfo`

- <span id="implinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImplInfo`

- <span id="implinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ImplInfo`

- <span id="implinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ImplInfo`

##### `impl<U> Into for ImplInfo`

- <span id="implinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ImplInfo`

##### `impl OwoColorize for ImplInfo`

##### `impl Pointable for ImplInfo`

- <span id="implinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="implinfo-pointable-type-init"></span>`type Init = T`

- <span id="implinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ImplInfo`

- <span id="implinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="implinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ImplInfo`

- <span id="implinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplInfo`

- <span id="implinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for MacroInfo`

- <span id="macroinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MacroInfo`

- <span id="macroinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MacroInfo`

- <span id="macroinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MacroInfo`

- <span id="macroinfo-clone"></span>`fn clone(&self) -> MacroInfo` — [`MacroInfo`](types/index.md#macroinfo)

##### `impl CloneToUninit for MacroInfo`

- <span id="macroinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MacroInfo`

- <span id="macroinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MacroInfo`

- <span id="macroinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MacroInfo`

##### `impl<U> Into for MacroInfo`

- <span id="macroinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MacroInfo`

##### `impl OwoColorize for MacroInfo`

##### `impl Pointable for MacroInfo`

- <span id="macroinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="macroinfo-pointable-type-init"></span>`type Init = T`

- <span id="macroinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="macroinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="macroinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="macroinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MacroInfo`

- <span id="macroinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="macroinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macroinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MacroInfo`

- <span id="macroinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macroinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MacroInfo`

- <span id="macroinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macroinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StaticInfo`

- <span id="staticinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StaticInfo`

- <span id="staticinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StaticInfo`

- <span id="staticinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StaticInfo`

- <span id="staticinfo-clone"></span>`fn clone(&self) -> StaticInfo` — [`StaticInfo`](types/index.md#staticinfo)

##### `impl CloneToUninit for StaticInfo`

- <span id="staticinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StaticInfo`

- <span id="staticinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StaticInfo`

- <span id="staticinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for StaticInfo`

##### `impl<U> Into for StaticInfo`

- <span id="staticinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StaticInfo`

##### `impl OwoColorize for StaticInfo`

##### `impl Pointable for StaticInfo`

- <span id="staticinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="staticinfo-pointable-type-init"></span>`type Init = T`

- <span id="staticinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="staticinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="staticinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="staticinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StaticInfo`

- <span id="staticinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="staticinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="staticinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StaticInfo`

- <span id="staticinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="staticinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StaticInfo`

- <span id="staticinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="staticinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for StructInfo`

- <span id="structinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StructInfo`

- <span id="structinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StructInfo`

- <span id="structinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StructInfo`

- <span id="structinfo-clone"></span>`fn clone(&self) -> StructInfo` — [`StructInfo`](types/index.md#structinfo)

##### `impl CloneToUninit for StructInfo`

- <span id="structinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StructInfo`

- <span id="structinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StructInfo`

- <span id="structinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for StructInfo`

##### `impl<U> Into for StructInfo`

- <span id="structinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StructInfo`

##### `impl OwoColorize for StructInfo`

##### `impl Pointable for StructInfo`

- <span id="structinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="structinfo-pointable-type-init"></span>`type Init = T`

- <span id="structinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="structinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="structinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="structinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StructInfo`

- <span id="structinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="structinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="structinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StructInfo`

- <span id="structinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="structinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StructInfo`

- <span id="structinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="structinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TraitInfo`

- <span id="traitinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitInfo`

- <span id="traitinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitInfo`

- <span id="traitinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TraitInfo`

- <span id="traitinfo-clone"></span>`fn clone(&self) -> TraitInfo` — [`TraitInfo`](types/index.md#traitinfo)

##### `impl CloneToUninit for TraitInfo`

- <span id="traitinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TraitInfo`

- <span id="traitinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TraitInfo`

- <span id="traitinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TraitInfo`

##### `impl<U> Into for TraitInfo`

- <span id="traitinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TraitInfo`

##### `impl OwoColorize for TraitInfo`

##### `impl Pointable for TraitInfo`

- <span id="traitinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="traitinfo-pointable-type-init"></span>`type Init = T`

- <span id="traitinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="traitinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="traitinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="traitinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TraitInfo`

- <span id="traitinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="traitinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TraitInfo`

- <span id="traitinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitInfo`

- <span id="traitinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for TypeAliasInfo`

- <span id="typealiasinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeAliasInfo`

- <span id="typealiasinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeAliasInfo`

- <span id="typealiasinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeAliasInfo`

- <span id="typealiasinfo-clone"></span>`fn clone(&self) -> TypeAliasInfo` — [`TypeAliasInfo`](types/index.md#typealiasinfo)

##### `impl CloneToUninit for TypeAliasInfo`

- <span id="typealiasinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TypeAliasInfo`

- <span id="typealiasinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TypeAliasInfo`

- <span id="typealiasinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TypeAliasInfo`

##### `impl<U> Into for TypeAliasInfo`

- <span id="typealiasinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TypeAliasInfo`

##### `impl OwoColorize for TypeAliasInfo`

##### `impl Pointable for TypeAliasInfo`

- <span id="typealiasinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="typealiasinfo-pointable-type-init"></span>`type Init = T`

- <span id="typealiasinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="typealiasinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="typealiasinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="typealiasinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TypeAliasInfo`

- <span id="typealiasinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="typealiasinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typealiasinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TypeAliasInfo`

- <span id="typealiasinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typealiasinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeAliasInfo`

- <span id="typealiasinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typealiasinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for VariantInfo`

- <span id="variantinfo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VariantInfo`

- <span id="variantinfo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VariantInfo`

- <span id="variantinfo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VariantInfo`

- <span id="variantinfo-clone"></span>`fn clone(&self) -> VariantInfo` — [`VariantInfo`](types/index.md#variantinfo)

##### `impl CloneToUninit for VariantInfo`

- <span id="variantinfo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for VariantInfo`

- <span id="variantinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VariantInfo`

- <span id="variantinfo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for VariantInfo`

##### `impl<U> Into for VariantInfo`

- <span id="variantinfo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for VariantInfo`

##### `impl OwoColorize for VariantInfo`

##### `impl Pointable for VariantInfo`

- <span id="variantinfo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="variantinfo-pointable-type-init"></span>`type Init = T`

- <span id="variantinfo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="variantinfo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="variantinfo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="variantinfo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for VariantInfo`

- <span id="variantinfo-toowned-type-owned"></span>`type Owned = T`

- <span id="variantinfo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="variantinfo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VariantInfo`

- <span id="variantinfo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="variantinfo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VariantInfo`

- <span id="variantinfo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="variantinfo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for PrivateItem<'a>`

- <span id="privateitem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrivateItem<'a>`

- <span id="privateitem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrivateItem<'a>`

- <span id="privateitem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PrivateItem<'a>`

- <span id="privateitem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PrivateItem<'a>`

- <span id="privateitem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for PrivateItem<'a>`

##### `impl<U> Into for PrivateItem<'a>`

- <span id="privateitem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PrivateItem<'a>`

##### `impl OwoColorize for PrivateItem<'a>`

##### `impl Pointable for PrivateItem<'a>`

- <span id="privateitem-pointable-const-align"></span>`const ALIGN: usize`

- <span id="privateitem-pointable-type-init"></span>`type Init = T`

- <span id="privateitem-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="privateitem-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="privateitem-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="privateitem-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for PrivateItem<'a>`

- <span id="privateitem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="privateitem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrivateItem<'a>`

- <span id="privateitem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="privateitem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

