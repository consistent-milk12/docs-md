*[walkdir](../index.md) / [dent](index.md)*

---

# Module `dent`

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

A directory entry.

This is the type of value that is yielded from the iterators defined in
this crate.

On Unix systems, this type implements the [`DirEntryExt`](../index.md) trait, which
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

- `fn path(self: &Self) -> &Path`

- `fn into_path(self: Self) -> PathBuf`

- `fn path_is_symlink(self: &Self) -> bool`

- `fn metadata(self: &Self) -> Result<fs::Metadata>` — [`Result`](../index.md)

- `fn metadata_internal(self: &Self) -> Result<fs::Metadata>` — [`Result`](../index.md)

- `fn file_type(self: &Self) -> fs::FileType`

- `fn file_name(self: &Self) -> &OsStr`

- `fn depth(self: &Self) -> usize`

- `fn is_dir(self: &Self) -> bool`

- `fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](../index.md)

- `fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](../index.md), [`DirEntry`](../index.md)

#### Trait Implementations

##### `impl Clone for DirEntry`

- `fn clone(self: &Self) -> DirEntry` — [`DirEntry`](../index.md)

##### `impl Debug for DirEntry`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- `fn ino(self: &Self) -> u64`

## Traits

### `DirEntryExt`

```rust
trait DirEntryExt { ... }
```

Unix-specific extension methods for `walkdir::DirEntry`

#### Required Methods

- `fn ino(self: &Self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

