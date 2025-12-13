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

*Defined in [`walkdir-2.5.0/src/error.rs:28-31`](../../../.source_1765633015/walkdir-2.5.0/src/error.rs#L28-L31)*

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

  Returns the path associated with this error if one exists.

  

  For example, if an error occurred while opening a directory handle,

  the error will include the path passed to `std::fs::read_dir`.

- <span id="error-loop-ancestor"></span>`fn loop_ancestor(&self) -> Option<&Path>`

  Returns the path at which a cycle was detected.

  

  If no cycle was detected, [`None`](#none) is returned.

  

  A cycle is detected when a directory entry is equivalent to one of

  its ancestors.

  

  To get the path to the child directory entry in the cycle, use the

  [`path`](#path) method.

  

- <span id="error-depth"></span>`fn depth(&self) -> usize`

  Returns the depth at which this error occurred relative to the root.

  

  The smallest depth is `0` and always corresponds to the path given to

  the `new` function on [`WalkDir`](../index.md). Its direct descendents have depth

  `1`, and their descendents have depth `2`, and so on.

  

- <span id="error-io-error"></span>`fn io_error(&self) -> Option<&io::Error>`

  Inspect the original `io::Error` if there is one.

  

  [`None`](#none) is returned if the [`Error`](#error) doesn't correspond to an

  `io::Error`. This might happen, for example, when the error was

  produced because a cycle was found in the directory tree while

  following symbolic links.

  

  This method returns a borrowed value that is bound to the lifetime of the [`Error`](#error). To

  obtain an owned value, the `into_io_error` can be used instead.

  

  > This is the original `io::Error` and is _not_ the same as

  > [`impl From<Error> for std::io::Error`][impl] which contains additional context about the

  error.

  

  # Example

  

  ```rust,no_run

  use std::io;

  use std::path::Path;

  

  use walkdir::WalkDir;

  

  for entry in WalkDir::new("foo") {

      match entry {

          Ok(entry) => println!("{}", entry.path().display()),

          Err(err) => {

              let path = err.path().unwrap_or(Path::new("")).display();

              println!("failed to access entry {}", path);

              if let Some(inner) = err.io_error() {

                  match inner.kind() {

                      io::ErrorKind::InvalidData => {

                          println!(

                              "entry contains invalid data: {}",

                              inner)

                      }

                      io::ErrorKind::PermissionDenied => {

                          println!(

                              "Missing permission to read entry: {}",

                              inner)

                      }

                      _ => {

                          println!(

                              "Unexpected error occurred: {}",

                              inner)

                      }

                  }

              }

          }

      }

  }

  ```

  

  

  

  

  

- <span id="error-into-io-error"></span>`fn into_io_error(self) -> Option<io::Error>`

  Similar to `io_error` except consumes self to convert to the original

  `io::Error` if one exists.

  

- <span id="error-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- <span id="error-from-entry"></span>`fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](../dent/index.md#direntry)

- <span id="error-from-io"></span>`fn from_io(depth: usize, err: io::Error) -> Self`

- <span id="error-from-loop"></span>`fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-description"></span>`fn description(&self) -> &str`

- <span id="error-error-cause"></span>`fn cause(&self) -> Option<&dyn error::Error>`

- <span id="error-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`walkdir-2.5.0/src/error.rs:34-37`](../../../.source_1765633015/walkdir-2.5.0/src/error.rs#L34-L37)*

#### Trait Implementations

##### `impl Any for ErrorInner`

- <span id="errorinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorInner`

- <span id="errorinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorInner`

- <span id="errorinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ErrorInner`

- <span id="errorinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorInner`

- <span id="errorinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorInner`

- <span id="errorinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ErrorInner`

- <span id="errorinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorInner`

- <span id="errorinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

