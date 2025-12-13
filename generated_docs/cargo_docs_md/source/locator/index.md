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

  Create a new `SourceLocator` with the default registry path.

  

  # Errors

  

  Returns an error if the home directory cannot be determined.

- <span id="sourcelocator-with-registry-path"></span>`const fn with_registry_path(registry_path: PathBuf) -> Self`

  Create a `SourceLocator` with a custom registry path.

- <span id="sourcelocator-load-metadata"></span>`fn load_metadata(&mut self, manifest_path: &Path) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Load cargo metadata from a project directory.

  

  This enables more accurate source location by using the exact

  paths from cargo's dependency resolution.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcelocator-load-metadata-from-current-dir"></span>`fn load_metadata_from_current_dir(&mut self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Load cargo metadata from the current directory.

  

  # Errors

  

  Returns an error if cargo metadata cannot be loaded.

- <span id="sourcelocator-locate"></span>`fn locate(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

  Locate the source directory for a crate by name and version.

  

  First tries to use cargo metadata if available, then falls back

  to scanning the registry directory.

  

  # Errors

  

  Returns an error if the source cannot be found.

- <span id="sourcelocator-locate-from-metadata"></span>`fn locate_from_metadata(metadata: &Metadata, name: &str, version: &str) -> Option<PathBuf>`

  Locate source using cargo metadata.

- <span id="sourcelocator-locate-in-registry"></span>`fn locate_in_registry(&self, name: &str, version: &str) -> Result<PathBuf, Error>` — [`Error`](../../error/index.md#error)

  Locate source by scanning the registry directory.

- <span id="sourcelocator-packages"></span>`fn packages(&self) -> Option<&[Package]>`

  Get all packages from the loaded metadata.

  

  Returns `None` if metadata hasn't been loaded.

- <span id="sourcelocator-workspace-root"></span>`fn workspace_root(&self) -> Option<&Path>`

  Get the workspace root from loaded metadata.

  

  Returns `None` if metadata hasn't been loaded.

- <span id="sourcelocator-all-dependency-sources"></span>`fn all_dependency_sources(&self) -> Result<Vec<(String, String, PathBuf)>, Error>` — [`Error`](../../error/index.md#error)

  Find all dependency sources for a workspace.

  

  Returns a list of (name, version, path) tuples for all dependencies

  that have sources in the registry.

  

  # Errors

  

  Returns an error if metadata hasn't been loaded.

- <span id="sourcelocator-list-registry-crates"></span>`fn list_registry_crates(&self) -> Result<Vec<(String, String)>, Error>` — [`Error`](../../error/index.md#error)

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

### `ParseUtils`

```rust
struct ParseUtils;
```

*Defined in `src/source/locator.rs:261`*

#### Implementations

- <span id="parseutils-parse-crate-dir-name"></span>`fn parse_crate_dir_name(dir_name: &str) -> Option<(&str, &str)>`

  Parse a crate directory name like "serde-1.0.228" into (name, version).

#### Trait Implementations

##### `impl Any for ParseUtils`

- <span id="parseutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseUtils`

- <span id="parseutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseUtils`

- <span id="parseutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ParseUtils`

- <span id="parseutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for ParseUtils`

##### `impl<U> Into for ParseUtils`

- <span id="parseutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ParseUtils`

##### `impl OwoColorize for ParseUtils`

##### `impl Pointable for ParseUtils`

- <span id="parseutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="parseutils-pointable-type-init"></span>`type Init = T`

- <span id="parseutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="parseutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="parseutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="parseutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ParseUtils`

- <span id="parseutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseUtils`

- <span id="parseutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for ParseUtils`

