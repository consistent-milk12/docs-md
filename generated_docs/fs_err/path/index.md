*[fs_err](../index.md) / [path](index.md)*

---

# Module `path`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PathExt`](#pathext) | trait | Defines aliases on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) for `fs_err` functions. |

## Traits

### `PathExt`

```rust
trait PathExt: crate::Sealed { ... }
```

*Defined in [`fs-err-3.2.1/src/path.rs:12-39`](../../../.source_1765900590/fs-err-3.2.1/src/path.rs#L12-L39)*

Defines aliases on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) for `fs_err` functions.

This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn PathExt::fs_err_try_exists(&self) -> io::Result<bool>`

  Returns Ok(true) if the path points at an existing entity.
  
  Wrapper for [`Path::try_exists`](https://doc.rust-lang.org/std/path/struct.Path.html#method.try_exists).

- `fn PathExt::fs_err_metadata(&self) -> io::Result<fs::Metadata>`

  Given a path, query the file system to get information about a file, directory, etc.
  
  Wrapper for [`crate::metadata`](../index.md).

- `fn PathExt::fs_err_symlink_metadata(&self) -> io::Result<fs::Metadata>`

  Query the metadata about a file without following symlinks.
  
  Wrapper for [`crate::symlink_metadata`](../index.md).

- `fn PathExt::fs_err_canonicalize(&self) -> io::Result<PathBuf>`

  Returns the canonical, absolute form of a path with all intermediate components
  normalized and symbolic links resolved.
  
  Wrapper for [`crate::canonicalize`](../index.md).

- `fn PathExt::fs_err_read_link(&self) -> io::Result<PathBuf>`

  Reads a symbolic link, returning the file that the link points to.
  
  Wrapper for [`crate::read_link`](../index.md).

- `fn PathExt::fs_err_read_dir(&self) -> io::Result<crate::ReadDir>`

  Returns an iterator over the entries within a directory.
  
  Wrapper for [`crate::read_dir`](../index.md).

#### Implementors

- `std::path::Path`

