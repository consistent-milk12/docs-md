*[fs_err](../index.md) / [file](index.md)*

---

# Module `file`

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

- `fn open<P>(path: P) -> Result<Self, io::Error>`

- `fn create<P>(path: P) -> Result<Self, io::Error>`

- `fn create_new<P>(path: P) -> Result<Self, io::Error>`

- `fn options() -> OpenOptions` — [`OpenOptions`](../index.md)

- `fn sync_all(self: &Self) -> Result<(), io::Error>`

- `fn sync_data(self: &Self) -> Result<(), io::Error>`

- `fn set_len(self: &Self, size: u64) -> Result<(), io::Error>`

- `fn metadata(self: &Self) -> Result<fs::Metadata, io::Error>`

- `fn try_clone(self: &Self) -> Result<Self, io::Error>`

- `fn set_permissions(self: &Self, perm: fs::Permissions) -> Result<(), io::Error>`

#### Trait Implementations

##### `impl AsFd for crate::File`

- `fn as_fd(self: &Self) -> BorrowedFd<'_>`

##### `impl AsRawFd for crate::File`

- `fn as_raw_fd(self: &Self) -> RawFd`

##### `impl Debug for File`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FileExt for crate::File`

- `fn read_at(self: &Self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

- `fn write_at(self: &Self, buf: &[u8], offset: u64) -> io::Result<usize>`

##### `impl IntoRawFd for crate::File`

- `fn into_raw_fd(self: Self) -> RawFd`

##### `impl Read for File`

- `fn read(self: &mut Self, buf: &mut [u8]) -> std::io::Result<usize>`

- `fn read_vectored(self: &mut Self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize>`

##### `impl Sealed for crate::File`

##### `impl Seek for File`

- `fn seek(self: &mut Self, pos: std::io::SeekFrom) -> std::io::Result<u64>`

##### `impl Write for File`

- `fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- `fn flush(self: &mut Self) -> std::io::Result<()>`

## Functions

### `open`

```rust
fn open(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

