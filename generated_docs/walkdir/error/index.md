*[walkdir](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | An error produced by recursively walking a directory. |
| [`ErrorInner`](#errorinner) | enum |  |

## Structs

### `Error`

```rust
struct Error {
    depth: usize,
    inner: ErrorInner,
}
```

*Defined in [`walkdir-2.5.0/src/error.rs:28-31`](../../../.source_1765210505/walkdir-2.5.0/src/error.rs#L28-L31)*

An error produced by recursively walking a directory.

This error type is a light wrapper around `std::io::Error`. In
particular, it adds the following information:

* The depth at which the error occurred in the file tree, relative to the
root.
* The path, if any, associated with the IO error.
* An indication that a loop occurred when following symbolic links. In this
case, there is no underlying IO error.

To maintain good ergonomics, this type has a
[`impl From<Error> for std::io::Error`][impl] defined which preserves the original context.
This allows you to use an `io::Result` with methods in this crate if you don't care about
accessing the underlying error data in a structured form.




#### Implementations

- <span id="error-path"></span>`fn path(&self) -> Option<&Path>`

- <span id="error-loop-ancestor"></span>`fn loop_ancestor(&self) -> Option<&Path>`

- <span id="error-depth"></span>`fn depth(&self) -> usize`

- <span id="error-io-error"></span>`fn io_error(&self) -> Option<&io::Error>`

- <span id="error-into-io-error"></span>`fn into_io_error(self) -> Option<io::Error>`

- <span id="error-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- <span id="error-from-entry"></span>`fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](../dent/index.md)

- <span id="error-from-io"></span>`fn from_io(depth: usize, err: io::Error) -> Self`

- <span id="error-from-loop"></span>`fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-description"></span>`fn description(&self) -> &str`

- <span id="error-cause"></span>`fn cause(&self) -> Option<&dyn error::Error>`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ErrorInner`

```rust
enum ErrorInner {
    Io {
        path: Option<std::path::PathBuf>,
        err: io::Error,
    },
    Loop {
        ancestor: std::path::PathBuf,
        child: std::path::PathBuf,
    },
}
```

*Defined in [`walkdir-2.5.0/src/error.rs:34-37`](../../../.source_1765210505/walkdir-2.5.0/src/error.rs#L34-L37)*

#### Trait Implementations

##### `impl Debug for ErrorInner`

- <span id="errorinner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

