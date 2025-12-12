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

##### `impl Clone for CollectedCrate`

- <span id="collectedcrate-clone"></span>`fn clone(&self) -> CollectedCrate` — [`CollectedCrate`](#collectedcrate)

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

- <span id="collectoptions-default"></span>`fn default() -> CollectOptions` — [`CollectOptions`](#collectoptions)

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

- <span id="sourcecollector-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcecollector-from-manifest"></span>`fn from_manifest(manifest_path: Option<&Path>) -> Result<Self, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcecollector-collect"></span>`fn collect(&self, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](#collectoptions), [`CollectionResult`](#collectionresult), [`Error`](../../error/index.md#error)

- <span id="sourcecollector-generate-output-dir"></span>`fn generate_output_dir(&self) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcecollector-find-registry-source"></span>`fn find_registry_source(&self, name: &str, version: &str) -> Option<PathBuf>`

- <span id="sourcecollector-copy-crate-source"></span>`fn copy_crate_source(source: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcecollector-get-dev-only-packages"></span>`fn get_dev_only_packages(&self) -> HashSet<PackageId>`

- <span id="sourcecollector-dry-run-collect"></span>`fn dry_run_collect(&self, output_dir: &Path, options: &CollectOptions) -> Result<CollectionResult, Error>` — [`CollectOptions`](#collectoptions), [`CollectionResult`](#collectionresult), [`Error`](../../error/index.md#error)

- <span id="sourcecollector-update-gitignore"></span>`fn update_gitignore(&self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcecollector-list-dependencies"></span>`fn list_dependencies(&self) -> Vec<(&str, &str)>`

- <span id="sourcecollector-copy-dir-recursive"></span>`fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

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

### `TimeUtils`

```rust
struct TimeUtils;
```

*Defined in `src/source/collector.rs:457`*

#### Implementations

- <span id="timeutils-chrono-lite-now"></span>`fn chrono_lite_now() -> String`

- <span id="timeutils-is-leap-year"></span>`const fn is_leap_year(year: u64) -> bool`

#### Trait Implementations

##### `impl Instrument for TimeUtils`

##### `impl IntoEither for TimeUtils`

##### `impl OwoColorize for TimeUtils`

##### `impl Pointable for TimeUtils`

- <span id="timeutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="timeutils-pointable-type-init"></span>`type Init = T`

- <span id="timeutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="timeutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="timeutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="timeutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TimeUtils`

