*[fs_err](../index.md) / [dir](index.md)*

---

# Module `dir`

## Modules

- [`unix`](unix/index.md) - 

## Structs

### `ReadDir`

```rust
struct ReadDir {
    inner: fs::ReadDir,
    path: std::path::PathBuf,
}
```

Wrapper around `std::fs::ReadDir` which adds more
helpful information to all errors.

This struct is created via `fs_err::read_dir`.



#### Trait Implementations

##### `impl Debug for ReadDir`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ReadDir`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for ReadDir`

- `type Item = Result<DirEntry, Error>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `DirEntry`

```rust
struct DirEntry {
    inner: fs::DirEntry,
}
```

Wrapper around `std::fs::DirEntry` which adds more
helpful information to all errors.


#### Implementations

- `fn path(self: &Self) -> PathBuf`

- `fn metadata(self: &Self) -> io::Result<fs::Metadata>`

- `fn file_type(self: &Self) -> io::Result<fs::FileType>`

- `fn file_name(self: &Self) -> OsString`

#### Trait Implementations

##### `impl Debug for DirEntry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DirEntryExt for DirEntry`

- `fn ino(self: &Self) -> u64`

## Functions

### `read_dir`

```rust
fn read_dir<P: Into<std::path::PathBuf>>(path: P) -> io::Result<ReadDir>
```

Returns an iterator over the entries within a directory.

Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html).

