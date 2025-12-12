*[fs_err](../index.md) / [dir](index.md)*

---

# Module `dir`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`ReadDir`](#readdir) | struct | Wrapper around [`std::fs::ReadDir`][std::fs::ReadDir] which adds more helpful information to all errors. |
| [`DirEntry`](#direntry) | struct | Wrapper around [`std::fs::DirEntry`][std::fs::DirEntry] which adds more helpful information to all errors. |
| [`read_dir`](#read-dir) | fn | Returns an iterator over the entries within a directory. |

## Modules

- [`unix`](unix/index.md)

## Structs

### `ReadDir`

```rust
struct ReadDir {
    inner: fs::ReadDir,
    path: std::path::PathBuf,
}
```

*Defined in [`fs-err-3.2.0/src/dir.rs:28-31`](../../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L28-L31)*

Wrapper around `std::fs::ReadDir` which adds more
helpful information to all errors.

This struct is created via `fs_err::read_dir`.



#### Trait Implementations

##### `impl Debug for ReadDir`

- <span id="readdir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ReadDir`

- <span id="readdir-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="readdir-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="readdir-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ReadDir`

- <span id="readdir-iterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="readdir-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DirEntry`

```rust
struct DirEntry {
    inner: fs::DirEntry,
}
```

*Defined in [`fs-err-3.2.0/src/dir.rs:51-53`](../../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L51-L53)*

Wrapper around `std::fs::DirEntry` which adds more
helpful information to all errors.


#### Implementations

- <span id="direntry-path"></span>`fn path(&self) -> PathBuf`

- <span id="direntry-metadata"></span>`fn metadata(&self) -> io::Result<fs::Metadata>`

- <span id="direntry-file-type"></span>`fn file_type(&self) -> io::Result<fs::FileType>`

- <span id="direntry-file-name"></span>`fn file_name(&self) -> OsString`

#### Trait Implementations

##### `impl Debug for DirEntry`

- <span id="direntry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- <span id="direntry-ino"></span>`fn ino(&self) -> u64`

## Functions

### `read_dir`

```rust
fn read_dir<P: Into<std::path::PathBuf>>(path: P) -> io::Result<ReadDir>
```

*Defined in [`fs-err-3.2.0/src/dir.rs:11-18`](../../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L11-L18)*

Returns an iterator over the entries within a directory.

Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html).

