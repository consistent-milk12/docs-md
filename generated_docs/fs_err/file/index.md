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

*Defined in [`fs-err-3.2.0/src/file.rs:13-16`](../../../.source_1765210505/fs-err-3.2.0/src/file.rs#L13-L16)*

Wrapper around `std::fs::File` which adds more helpful
information to all errors.


#### Implementations

- <span id="file-open"></span>`fn open<P>(path: P) -> Result<Self, io::Error>`

- <span id="file-create"></span>`fn create<P>(path: P) -> Result<Self, io::Error>`

- <span id="file-create-new"></span>`fn create_new<P>(path: P) -> Result<Self, io::Error>`

- <span id="file-options"></span>`fn options() -> OpenOptions` — [`OpenOptions`](../open_options/index.md#openoptions)

- <span id="file-sync-all"></span>`fn sync_all(&self) -> Result<(), io::Error>`

- <span id="file-sync-data"></span>`fn sync_data(&self) -> Result<(), io::Error>`

- <span id="file-set-len"></span>`fn set_len(&self, size: u64) -> Result<(), io::Error>`

- <span id="file-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata, io::Error>`

- <span id="file-try-clone"></span>`fn try_clone(&self) -> Result<Self, io::Error>`

- <span id="file-set-permissions"></span>`fn set_permissions(&self, perm: fs::Permissions) -> Result<(), io::Error>`

#### Trait Implementations

##### `impl AsFd for crate::File`

- <span id="cratefile-as-fd"></span>`fn as_fd(&self) -> BorrowedFd<'_>`

##### `impl AsRawFd for crate::File`

- <span id="cratefile-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl Debug for File`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileExt for crate::File`

- <span id="cratefile-read-at"></span>`fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

- <span id="cratefile-write-at"></span>`fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>`

##### `impl IntoRawFd for crate::File`

- <span id="cratefile-into-raw-fd"></span>`fn into_raw_fd(self) -> RawFd`

##### `impl Read for File`

- <span id="file-read"></span>`fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>`

- <span id="file-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize>`

##### `impl Sealed for crate::File`

##### `impl Seek for File`

- <span id="file-seek"></span>`fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>`

##### `impl Write for File`

- <span id="file-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="file-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="file-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

## Functions

### `open`

```rust
fn open(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:20-22`](../../../.source_1765210505/fs-err-3.2.0/src/file.rs#L20-L22)*

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:25-27`](../../../.source_1765210505/fs-err-3.2.0/src/file.rs#L25-L27)*

