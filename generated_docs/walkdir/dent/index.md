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

*Defined in [`walkdir-2.5.0/src/dent.rs:35-59`](../../../.source_1765521767/walkdir-2.5.0/src/dent.rs#L35-L59)*

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

- <span id="direntry-into-path"></span>`fn into_path(self) -> PathBuf`

- <span id="direntry-path-is-symlink"></span>`fn path_is_symlink(&self) -> bool`

- <span id="direntry-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata>` — [`Result`](../index.md#result)

- <span id="direntry-metadata-internal"></span>`fn metadata_internal(&self) -> Result<fs::Metadata>` — [`Result`](../index.md#result)

- <span id="direntry-file-type"></span>`fn file_type(&self) -> fs::FileType`

- <span id="direntry-file-name"></span>`fn file_name(&self) -> &OsStr`

- <span id="direntry-depth"></span>`fn depth(&self) -> usize`

- <span id="direntry-is-dir"></span>`fn is_dir(&self) -> bool`

- <span id="direntry-from-entry"></span>`fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](../index.md#result)

- <span id="direntry-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](../index.md#result), [`DirEntry`](#direntry)

#### Trait Implementations

##### `impl Clone for DirEntry`

- <span id="direntry-clone"></span>`fn clone(&self) -> DirEntry` — [`DirEntry`](#direntry)

##### `impl Debug for DirEntry`

- <span id="direntry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- <span id="direntry-ino"></span>`fn ino(&self) -> u64`

## Traits

### `DirEntryExt`

```rust
trait DirEntryExt { ... }
```

*Defined in [`walkdir-2.5.0/src/dent.rs:339-343`](../../../.source_1765521767/walkdir-2.5.0/src/dent.rs#L339-L343)*

Unix-specific extension methods for `walkdir::DirEntry`

#### Required Methods

- `fn ino(&self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

#### Implementors

- [`DirEntry`](#direntry)

