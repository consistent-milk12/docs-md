*[walkdir](../index.md) / [error](index.md)*

---

# Module `error`

## Structs

### `Error`

```rust
struct Error {
    depth: usize,
    inner: ErrorInner,
}
```

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

- `fn path(self: &Self) -> Option<&Path>`

- `fn loop_ancestor(self: &Self) -> Option<&Path>`

- `fn depth(self: &Self) -> usize`

- `fn io_error(self: &Self) -> Option<&io::Error>`

- `fn into_io_error(self: Self) -> Option<io::Error>`

- `fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- `fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](../dent/index.md)

- `fn from_io(depth: usize, err: io::Error) -> Self`

- `fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn description(self: &Self) -> &str`

- `fn cause(self: &Self) -> Option<&dyn error::Error>`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

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

#### Trait Implementations

##### `impl Debug for ErrorInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

