*[cargo_docs_md](../../index.md) / [source](../index.md) / [locator](index.md)*

---

# Module `locator`

Source locator for finding crate sources in the Cargo registry.

This module provides utilities to locate crate source code in
`~/.cargo/registry/src/` based on crate name and version.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SourceLocator`](#sourcelocator) | struct | Locates crate sources in the Cargo registry. |
| [`ParseUtils`](#parseutils) | struct |  |

## Structs

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

- <span id="sourcelocator-new"></span>`fn new() -> Result<Self, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-with-registry-path"></span>`const fn with_registry_path(registry_path: PathBuf) -> Self`

- <span id="sourcelocator-load-metadata"></span>`fn load_metadata(&mut self, manifest_path: &Path) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-load-metadata-from-current-dir"></span>`fn load_metadata_from_current_dir(&mut self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-locate"></span>`fn locate(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-locate-from-metadata"></span>`fn locate_from_metadata(metadata: &Metadata, name: &str, version: &str) -> Option<PathBuf>`

- <span id="sourcelocator-locate-in-registry"></span>`fn locate_in_registry(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-packages"></span>`fn packages(&self) -> Option<&[Package]>`

- <span id="sourcelocator-workspace-root"></span>`fn workspace_root(&self) -> Option<&Path>`

- <span id="sourcelocator-all-dependency-sources"></span>`fn all_dependency_sources(&self) -> Result<Vec<(String, String, PathBuf)>, Error>` — [`Error`](../../error/index.md#error)

- <span id="sourcelocator-list-registry-crates"></span>`fn list_registry_crates(&self) -> Result<Vec<(String, String)>, Error>` — [`Error`](../../error/index.md#error)

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

### `ParseUtils`

```rust
struct ParseUtils;
```

*Defined in `src/source/locator.rs:261`*

#### Implementations

- <span id="parseutils-parse-crate-dir-name"></span>`fn parse_crate_dir_name(dir_name: &str) -> Option<(&str, &str)>`

#### Trait Implementations

##### `impl Instrument for ParseUtils`

##### `impl IntoEither for ParseUtils`

##### `impl OwoColorize for ParseUtils`

##### `impl Pointable for ParseUtils`

- <span id="parseutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="parseutils-pointable-type-init"></span>`type Init = T`

- <span id="parseutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="parseutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="parseutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="parseutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ParseUtils`

