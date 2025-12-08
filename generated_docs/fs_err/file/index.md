*[fs_err](../index.md) / [file](index.md)*

---

# Module `file`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`File`](#file) | struct | Wrapper around [`std::fs::File`][std::fs::File] which adds more helpful |
| [`open`](#open) | fn |  |
| [`create`](#create) | fn |  |

## Modules

- [`unix`](unix/index.md) - 

## Structs

### `File`

```rust
struct File {
    file: fs::File,
    path: std::path::PathBuf,
}
```

Wrapper around `std::fs::File` which adds more helpful
information to all errors.


#### Implementations

- <span id="file-lock"></span>`fn lock(&self) -> Result<(), io::Error>`

- <span id="file-lock-shared"></span>`fn lock_shared(&self) -> Result<(), io::Error>`

- <span id="file-try-lock"></span>`fn try_lock(&self) -> Result<(), fs::TryLockError>`

- <span id="file-try-lock-shared"></span>`fn try_lock_shared(&self) -> Result<(), fs::TryLockError>`

- <span id="file-unlock"></span>`fn unlock(&self) -> Result<(), io::Error>`

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

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

