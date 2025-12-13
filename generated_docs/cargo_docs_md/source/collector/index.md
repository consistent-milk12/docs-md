*[cargo_docs_md](../../index.md) / [source](../index.md) / [collector](index.md)*

---

# Module `collector`

Source collector for copying dependency sources to a local directory.

This module provides functionality to collect dependency source code
from `~/.cargo/registry/src/` into a local `.source_{timestamp}/` directory.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CollectedCrate`](#collectedcrate) | struct | Metadata about a collected crate. |
| [`SourceManifest`](#sourcemanifest) | struct | Manifest stored in `.source_*/manifest.json`. |
| [`CollectionResult`](#collectionresult) | struct | Result of a collection operation. |
| [`CollectOptions`](#collectoptions) | struct | Options for source collection. |
| [`SourceCollector`](#sourcecollector) | struct | Collector for gathering dependency sources. |
| [`TimeUtils`](#timeutils) | struct |  |

## Structs

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

- <span id="collectedcrate-clone"></span>`fn clone(&self) -> CollectedCrate` — [`CollectedCrate`](#collectedcrate)

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

- <span id="collectoptions-default"></span>`fn default() -> CollectOptions` — [`CollectOptions`](#collectoptions)

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

- <span id="sourcecollector-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../../error/index.md#error)

  Create a new collector for the current directory.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcecollector-from-manifest"></span>`fn from_manifest(manifest_path: Option<&Path>) -> Result<Self, Error>` — [`Error`](../../error/index.md#error)

  Create a new collector from a specific manifest path.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcecollector-collect"></span>`fn collect(&self, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](#collectoptions), [`CollectionResult`](#collectionresult), [`Error`](../../error/index.md#error)

  Collect all dependency sources.

  

  # Errors

  

  Returns an error if collection fails.

- <span id="sourcecollector-generate-output-dir"></span>`fn generate_output_dir(&self) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

  Generate a timestamp-based output directory name.

- <span id="sourcecollector-find-registry-source"></span>`fn find_registry_source(&self, name: &str, version: &str) -> Option<PathBuf>`

  Find a crate's source in the cargo registry.

- <span id="sourcecollector-copy-crate-source"></span>`fn copy_crate_source(source: &Path, dest: &Path, minimal: bool) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Copy crate source to destination.

  

  If `minimal` is false (default), copies the entire crate directory.

  If `minimal` is true, only copies `src/` and `Cargo.toml`.

  

  In both modes, `Cargo.toml` is renamed to `Crate.toml` to avoid

  confusing cargo when the collected sources are in the workspace.

- <span id="sourcecollector-get-dev-only-packages"></span>`fn get_dev_only_packages(&self) -> HashSet<PackageId>`

  Get the set of package IDs that are dev-only dependencies.

  

  A package is considered dev-only if it is only reachable from workspace

  members via dev-dependencies (not normal or build dependencies).

- <span id="sourcecollector-dry-run-collect"></span>`fn dry_run_collect(&self, output_dir: &Path, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](#collectoptions), [`CollectionResult`](#collectionresult), [`Error`](../../error/index.md#error)

  Perform a dry run, returning what would be collected.

- <span id="sourcecollector-update-gitignore"></span>`fn update_gitignore(&self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Update .gitignore to include .source_* pattern.

- <span id="sourcecollector-list-dependencies"></span>`fn list_dependencies(&self) -> Vec<(&str, &str)>`

  List all external dependencies.

- <span id="sourcecollector-copy-dir-recursive"></span>`fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

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

### `TimeUtils`

```rust
struct TimeUtils;
```

*Defined in `src/source/collector.rs:509`*

#### Implementations

- <span id="timeutils-chrono-lite-now"></span>`fn chrono_lite_now() -> String`

  Simple ISO 8601 timestamp without external dependency.

- <span id="timeutils-is-leap-year"></span>`const fn is_leap_year(year: u64) -> bool`

#### Trait Implementations

##### `impl Any for TimeUtils`

- <span id="timeutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TimeUtils`

- <span id="timeutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TimeUtils`

- <span id="timeutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TimeUtils`

- <span id="timeutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TimeUtils`

##### `impl<U> Into for TimeUtils`

- <span id="timeutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TimeUtils`

##### `impl OwoColorize for TimeUtils`

##### `impl Pointable for TimeUtils`

- <span id="timeutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="timeutils-pointable-type-init"></span>`type Init = T`

- <span id="timeutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="timeutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="timeutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="timeutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TimeUtils`

- <span id="timeutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="timeutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TimeUtils`

- <span id="timeutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="timeutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for TimeUtils`

