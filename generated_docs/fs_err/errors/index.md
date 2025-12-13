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

*Defined in [`fs-err-3.2.0/src/errors.rs:49-53`](../../../.source_1765633015/fs-err-3.2.0/src/errors.rs#L49-L53)*

Contains an IO error that has a file path attached.

This type is never returned directly, but is instead wrapped inside yet
another IO error.

#### Implementations

- <span id="error-build"></span>`fn build(source: io::Error, kind: ErrorKind, path: impl Into<PathBuf>) -> io::Error` — [`ErrorKind`](#errorkind)

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

- <span id="error-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

- <span id="error-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

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

### `SourceDestError`

```rust
struct SourceDestError {
    kind: SourceDestErrorKind,
    source: io::Error,
    from_path: std::path::PathBuf,
    to_path: std::path::PathBuf,
}
```

*Defined in [`fs-err-3.2.0/src/errors.rs:157-162`](../../../.source_1765633015/fs-err-3.2.0/src/errors.rs#L157-L162)*

Error type used by functions like `fs::copy` that holds two paths.

#### Implementations

- <span id="sourcedesterror-build"></span>`fn build(source: io::Error, kind: SourceDestErrorKind, from_path: impl Into<PathBuf>, to_path: impl Into<PathBuf>) -> io::Error` — [`SourceDestErrorKind`](#sourcedesterrorkind)

#### Trait Implementations

##### `impl Any for SourceDestError`

- <span id="sourcedesterror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceDestError`

- <span id="sourcedesterror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceDestError`

- <span id="sourcedesterror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SourceDestError`

- <span id="sourcedesterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SourceDestError`

- <span id="sourcedesterror-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for SourceDestError`

- <span id="sourcedesterror-error-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

- <span id="sourcedesterror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<T> From for SourceDestError`

- <span id="sourcedesterror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SourceDestError`

- <span id="sourcedesterror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for SourceDestError`

- <span id="sourcedesterror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SourceDestError`

- <span id="sourcedesterror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcedesterror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceDestError`

- <span id="sourcedesterror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcedesterror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`fs-err-3.2.0/src/errors.rs:9-42`](../../../.source_1765633015/fs-err-3.2.0/src/errors.rs#L9-L42)*

#### Trait Implementations

##### `impl Any for ErrorKind`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl CloneToUninit for ErrorKind`

- <span id="errorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ErrorKind`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ErrorKind`

- <span id="errorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="errorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ErrorKind`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`fs-err-3.2.0/src/errors.rs:140-153`](../../../.source_1765633015/fs-err-3.2.0/src/errors.rs#L140-L153)*

#### Trait Implementations

##### `impl Any for SourceDestErrorKind`

- <span id="sourcedesterrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceDestErrorKind`

- <span id="sourcedesterrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceDestErrorKind`

- <span id="sourcedesterrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourceDestErrorKind`

- <span id="sourcedesterrorkind-clone"></span>`fn clone(&self) -> SourceDestErrorKind` — [`SourceDestErrorKind`](#sourcedesterrorkind)

##### `impl CloneToUninit for SourceDestErrorKind`

- <span id="sourcedesterrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SourceDestErrorKind`

##### `impl Debug for SourceDestErrorKind`

- <span id="sourcedesterrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SourceDestErrorKind`

- <span id="sourcedesterrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SourceDestErrorKind`

- <span id="sourcedesterrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SourceDestErrorKind`

- <span id="sourcedesterrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="sourcedesterrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourcedesterrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceDestErrorKind`

- <span id="sourcedesterrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourcedesterrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceDestErrorKind`

- <span id="sourcedesterrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourcedesterrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

