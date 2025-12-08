*[fs_err](../index.md) / [errors](index.md)*

---

# Module `errors`

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

- `fn build(source: io::Error, kind: ErrorKind, path: impl Into<PathBuf>) -> io::Error` — [`ErrorKind`](#errorkind)

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn cause(self: &Self) -> Option<&dyn StdError>`

- `fn source(self: &Self) -> Option<&dyn StdError>`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

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

- `fn build(source: io::Error, kind: SourceDestErrorKind, from_path: impl Into<PathBuf>, to_path: impl Into<PathBuf>) -> io::Error` — [`SourceDestErrorKind`](#sourcedesterrorkind)

#### Trait Implementations

##### `impl Debug for SourceDestError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SourceDestError`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SourceDestError`

- `fn cause(self: &Self) -> Option<&dyn StdError>`

- `fn source(self: &Self) -> Option<&dyn StdError>`

##### `impl<T> ToString for SourceDestError`

- `fn to_string(self: &Self) -> String`

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

- `fn clone(self: &Self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> SourceDestErrorKind` — [`SourceDestErrorKind`](#sourcedesterrorkind)

##### `impl Copy for SourceDestErrorKind`

##### `impl Debug for SourceDestErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

