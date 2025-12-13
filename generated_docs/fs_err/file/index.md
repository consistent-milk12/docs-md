*[fs_err](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`File`](#file) | struct | Wrapper around [`std::fs::File`][std::fs::File] which adds more helpful information to all errors. |
| [`open`](#open) | fn |  |
| [`create`](#create) | fn |  |

## Modules

- [`unix`](unix/index.md)

## Structs

### `File`

```rust
struct File {
    file: fs::File,
    path: std::path::PathBuf,
}
```

*Defined in [`fs-err-3.2.0/src/file.rs:13-16`](../../../.source_1765521767/fs-err-3.2.0/src/file.rs#L13-L16)*

Wrapper around `std::fs::File` which adds more helpful
information to all errors.


#### Implementations

- <span id="file-open"></span>`fn open<P>(path: P) -> Result<Self, io::Error>`

  Attempts to open a file in read-only mode.

  

  Wrapper for [`File::open`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).

- <span id="file-create"></span>`fn create<P>(path: P) -> Result<Self, io::Error>`

  Opens a file in write-only mode.

  

  Wrapper for [`File::create`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create).

- <span id="file-create-new"></span>`fn create_new<P>(path: P) -> Result<Self, io::Error>`

  Opens a file in read-write mode.

  

  Wrapper for [`File::create_new`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create_new).

- <span id="file-options"></span>`fn options() -> OpenOptions` — [`OpenOptions`](../open_options/index.md#openoptions)

  Returns a new `OpenOptions` object.

  

  Wrapper for [`File::options`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.options).

- <span id="file-sync-all"></span>`fn sync_all(&self) -> Result<(), io::Error>`

  Attempts to sync all OS-internal metadata to disk.

  

  Wrapper for [`File::sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all).

- <span id="file-sync-data"></span>`fn sync_data(&self) -> Result<(), io::Error>`

  This function is similar to `sync_all`, except that it might not synchronize file metadata to the filesystem.

  

  Wrapper for [`File::sync_data`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_data).

- <span id="file-set-len"></span>`fn set_len(&self, size: u64) -> Result<(), io::Error>`

  Truncates or extends the underlying file, updating the size of this file to become `size`.

  

  Wrapper for [`File::set_len`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_len).

- <span id="file-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata, io::Error>`

  Queries metadata about the underlying file.

  

  Wrapper for [`File::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.metadata).

- <span id="file-try-clone"></span>`fn try_clone(&self) -> Result<Self, io::Error>`

  Creates a new `File` instance that shares the same underlying file handle as the

  existing `File` instance. Reads, writes, and seeks will affect both `File`

  instances simultaneously.

  

  Wrapper for [`File::try_clone`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.try_clone).

- <span id="file-set-permissions"></span>`fn set_permissions(&self, perm: fs::Permissions) -> Result<(), io::Error>`

  Changes the permissions on the underlying file.

  

  Wrapper for [`File::set_permissions`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_permissions).

#### Trait Implementations

##### `impl Any for File`

- <span id="file-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsFd for crate::File`

- <span id="cratefile-asfd-as-fd"></span>`fn as_fd(&self) -> BorrowedFd<'_>`

##### `impl AsRawFd for crate::File`

- <span id="cratefile-asrawfd-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl<T> Borrow for File`

- <span id="file-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for File`

- <span id="file-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for File`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileExt for crate::File`

- <span id="cratefile-fileext-read-at"></span>`fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

- <span id="cratefile-fileext-write-at"></span>`fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>`

##### `impl<T> From for File`

- <span id="file-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for File`

- <span id="file-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoRawFd for crate::File`

- <span id="cratefile-intorawfd-into-raw-fd"></span>`fn into_raw_fd(self) -> RawFd`

##### `impl Read for File`

- <span id="file-read"></span>`fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>`

- <span id="file-read-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize>`

##### `impl Sealed for crate::File`

##### `impl Seek for File`

- <span id="file-seek"></span>`fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>`

##### `impl<U> TryFrom for File`

- <span id="file-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for File`

- <span id="file-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for File`

- <span id="file-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="file-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="file-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

## Functions

### `open`

```rust
fn open(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:20-22`](../../../.source_1765521767/fs-err-3.2.0/src/file.rs#L20-L22)*

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:25-27`](../../../.source_1765521767/fs-err-3.2.0/src/file.rs#L25-L27)*

