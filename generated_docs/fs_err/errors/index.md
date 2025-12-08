*[fs_err](../index.md) / [errors](index.md)*

---

# Module `errors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | Contains an IO error that has a file path attached. |
| [`SourceDestError`](#sourcedesterror) | struct | Error type used by functions like `fs::copy` that holds two paths. |
| [`ErrorKind`](#errorkind) | enum |  |
| [`SourceDestErrorKind`](#sourcedesterrorkind) | enum |  |

## Structs

### `Error`

```rust
struct Error {
    kind: ErrorKind,
    source: io::Error,
    path: std::path::PathBuf,
}
```

Contains an IO error that has a file path attached.

This type is never returned directly, but is instead wrapped inside yet
another IO error.

#### Implementations

- <span id="error-build"></span>`fn build(source: io::Error, kind: ErrorKind, path: impl Into<PathBuf>) -> io::Error` — [`ErrorKind`](#errorkind)

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `SourceDestError`

```rust
struct SourceDestError {
    kind: SourceDestErrorKind,
    source: io::Error,
    from_path: std::path::PathBuf,
    to_path: std::path::PathBuf,
}
```

Error type used by functions like `fs::copy` that holds two paths.

#### Implementations

- <span id="sourcedesterror-build"></span>`fn build(source: io::Error, kind: SourceDestErrorKind, from_path: impl Into<PathBuf>, to_path: impl Into<PathBuf>) -> io::Error` — [`SourceDestErrorKind`](#sourcedesterrorkind)

#### Trait Implementations

##### `impl Debug for SourceDestError`

- <span id="sourcedesterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SourceDestError`

- <span id="sourcedesterror-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SourceDestError`

- <span id="sourcedesterror-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

- <span id="sourcedesterror-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<T> ToString for SourceDestError`

- <span id="sourcedesterror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    OpenFile,
    CreateFile,
    CreateDir,
    SyncFile,
    SetLen,
    Metadata,
    Clone,
    SetPermissions,
    Read,
    Seek,
    Write,
    Flush,
    ReadDir,
    RemoveFile,
    RemoveDir,
    Canonicalize,
    ReadLink,
    SymlinkMetadata,
    FileExists,
    Lock,
    Unlock,
    ReadAt,
    WriteAt,
}
```

#### Trait Implementations

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SourceDestErrorKind`

```rust
enum SourceDestErrorKind {
    Copy,
    HardLink,
    Rename,
    SoftLink,
    Symlink,
}
```

#### Trait Implementations

##### `impl Clone for SourceDestErrorKind`

- <span id="sourcedesterrorkind-clone"></span>`fn clone(&self) -> SourceDestErrorKind` — [`SourceDestErrorKind`](#sourcedesterrorkind)

##### `impl Copy for SourceDestErrorKind`

##### `impl Debug for SourceDestErrorKind`

- <span id="sourcedesterrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

