*[walkdir](../index.md) / [dent](index.md)*

---

# Module `dent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DirEntry`](#direntry) | struct | A directory entry. |
| [`DirEntryExt`](#direntryext) | trait | Unix-specific extension methods for `walkdir::DirEntry` |

## Structs

### `DirEntry`

```rust
struct DirEntry {
    path: std::path::PathBuf,
    ty: std::fs::FileType,
    follow_link: bool,
    depth: usize,
    ino: u64,
}
```

*Defined in [`walkdir-2.5.0/src/dent.rs:35-59`](../../../.source_1765633015/walkdir-2.5.0/src/dent.rs#L35-L59)*

A directory entry.

This is the type of value that is yielded from the iterators defined in
this crate.

On Unix systems, this type implements the [`DirEntryExt`](#direntryext) trait, which
provides efficient access to the inode number of the directory entry.

# Differences with `std::fs::DirEntry`

This type mostly mirrors the type by the same name in `std::fs`. There
are some differences however:

* All recursive directory iterators must inspect the entry's type.
Therefore, the value is stored and its access is guaranteed to be cheap and
successful.
* [`path`](#path) and `file_name` return borrowed variants.
* If `follow_links` was enabled on the originating iterator, then all
operations except for [`path`](#path) operate on the link target. Otherwise, all
operations operate on the symbolic link.






#### Fields

- **`path`**: `std::path::PathBuf`

  The path as reported by the `fs::ReadDir` iterator (even if it's a
  symbolic link).
  

- **`ty`**: `std::fs::FileType`

  The file type. Necessary for recursive iteration, so store it.

- **`follow_link`**: `bool`

  Is set when this entry was created from a symbolic link and the user
  expects the iterator to follow symbolic links.

- **`depth`**: `usize`

  The depth at which this entry was generated relative to the root.

- **`ino`**: `u64`

  The underlying inode number (Unix only).

#### Implementations

- <span id="direntry-path"></span>`fn path(&self) -> &Path`

  The full path that this entry represents.

  

  The full path is created by joining the parents of this entry up to the

  root initially given to `WalkDir::new` with the file name of this

  entry.

  

  Note that this *always* returns the path reported by the underlying

  directory entry, even when symbolic links are followed. To get the

  target path, use `path_is_symlink` to (cheaply) check if this entry

  corresponds to a symbolic link, and `std::fs::read_link` to resolve

  the target.

  

  

- <span id="direntry-into-path"></span>`fn into_path(self) -> PathBuf`

  The full path that this entry represents.

  

  Analogous to [`path`](#path), but moves ownership of the path.

- <span id="direntry-path-is-symlink"></span>`fn path_is_symlink(&self) -> bool`

  Returns `true` if and only if this entry was created from a symbolic

  link. This is unaffected by the `follow_links` setting.

  

  When `true`, the value returned by the [`path`](#path) method is a

  symbolic link name. To get the full target path, you must call

  `std::fs::read_link(entry.path())`.

  

  

- <span id="direntry-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata>` — [`Result`](../index.md#result)

  Return the metadata for the file that this entry points to.

  

  This will follow symbolic links if and only if the [`WalkDir`](../index.md) value

  has `follow_links` enabled.

  

  # Platform behavior

  

  This always calls `std::fs::symlink_metadata`.

  

  If this entry is a symbolic link and `follow_links` is enabled, then

  `std::fs::metadata` is called instead.

  

  # Errors

  

  Similar to `std::fs::metadata`, returns errors for path values that

  the program does not have permissions to access or if the path does not

  exist.

  

  

  

- <span id="direntry-metadata-internal"></span>`fn metadata_internal(&self) -> Result<fs::Metadata>` — [`Result`](../index.md#result)

- <span id="direntry-file-type"></span>`fn file_type(&self) -> fs::FileType`

  Return the file type for the file that this entry points to.

  

  If this is a symbolic link and `follow_links` is `true`, then this

  returns the type of the target.

  

  This never makes any system calls.

- <span id="direntry-file-name"></span>`fn file_name(&self) -> &OsStr`

  Return the file name of this entry.

  

  If this entry has no file name (e.g., `/`), then the full path is

  returned.

- <span id="direntry-depth"></span>`fn depth(&self) -> usize`

  Returns the depth at which this entry was created relative to the root.

  

  The smallest depth is `0` and always corresponds to the path given

  to the `new` function on `WalkDir`. Its direct descendents have depth

  `1`, and their descendents have depth `2`, and so on.

- <span id="direntry-is-dir"></span>`fn is_dir(&self) -> bool`

  Returns true if and only if this entry points to a directory.

- <span id="direntry-from-entry"></span>`fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](../index.md#result)

- <span id="direntry-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](../index.md#result), [`DirEntry`](#direntry)

#### Trait Implementations

##### `impl Any for DirEntry`

- <span id="direntry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DirEntry`

- <span id="direntry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DirEntry`

- <span id="direntry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DirEntry`

- <span id="direntry-clone"></span>`fn clone(&self) -> DirEntry` — [`DirEntry`](#direntry)

##### `impl CloneToUninit for DirEntry`

- <span id="direntry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DirEntry`

- <span id="direntry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- <span id="direntry-direntryext-ino"></span>`fn ino(&self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

  structure.

##### `impl<T> From for DirEntry`

- <span id="direntry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DirEntry`

- <span id="direntry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for DirEntry`

- <span id="direntry-toowned-type-owned"></span>`type Owned = T`

- <span id="direntry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="direntry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DirEntry`

- <span id="direntry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="direntry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DirEntry`

- <span id="direntry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="direntry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `DirEntryExt`

```rust
trait DirEntryExt { ... }
```

*Defined in [`walkdir-2.5.0/src/dent.rs:339-343`](../../../.source_1765633015/walkdir-2.5.0/src/dent.rs#L339-L343)*

Unix-specific extension methods for `walkdir::DirEntry`

#### Required Methods

- `fn ino(&self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

#### Implementors

- [`DirEntry`](#direntry)

