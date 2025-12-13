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

##### `impl Any for ReadDir`

- <span id="readdir-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadDir`

- <span id="readdir-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadDir`

- <span id="readdir-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReadDir`

- <span id="readdir-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadDir`

- <span id="readdir-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadDir`

- <span id="readdir-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ReadDir`

- <span id="readdir-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="readdir-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="readdir-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ReadDir`

- <span id="readdir-iterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="readdir-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ReadDir`

- <span id="readdir-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readdir-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadDir`

- <span id="readdir-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readdir-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Returns the full path to the file that this entry represents.

  

  Wrapper for [`DirEntry::path`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.path).

- <span id="direntry-metadata"></span>`fn metadata(&self) -> io::Result<fs::Metadata>`

  Returns the metadata for the file that this entry points at.

  

  Wrapper for [`DirEntry::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.metadata).

- <span id="direntry-file-type"></span>`fn file_type(&self) -> io::Result<fs::FileType>`

  Returns the file type for the file that this entry points at.

  

  Wrapper for [`DirEntry::file_type`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.file_type).

- <span id="direntry-file-name"></span>`fn file_name(&self) -> OsString`

  Returns the file name of this directory entry without any leading path component(s).

  

  Wrapper for [`DirEntry::file_name`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.file_name).

#### Trait Implementations

##### `impl Any for DirEntry`

- <span id="direntry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DirEntry`

- <span id="direntry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DirEntry`

- <span id="direntry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DirEntry`

- <span id="direntry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- <span id="direntry-direntryext-ino"></span>`fn ino(&self) -> u64`

##### `impl<T> From for DirEntry`

- <span id="direntry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DirEntry`

- <span id="direntry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DirEntry`

- <span id="direntry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="direntry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DirEntry`

- <span id="direntry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="direntry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `read_dir`

```rust
fn read_dir<P: Into<std::path::PathBuf>>(path: P) -> io::Result<ReadDir>
```

*Defined in [`fs-err-3.2.0/src/dir.rs:11-18`](../../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L11-L18)*

Returns an iterator over the entries within a directory.

Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html).

