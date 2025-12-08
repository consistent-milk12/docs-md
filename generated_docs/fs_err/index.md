# Crate `fs_err`

fs-err is a drop-in replacement for `std::fs` that provides more
helpful messages on errors. Extra information includes which operations was
attempted and any involved paths.

# Error Messages

Using `std::fs`, if this code fails:

```no_run
use std::fs::File;
let file = File::open("does not exist.txt")?;
Ok::<(), std::io::Error>(())
```

The error message that Rust gives you isn't very useful:

```txt
The system cannot find the file specified. (os error 2)
```

...but if we use fs-err instead, our error contains more actionable information:

```txt
failed to open file `does not exist.txt`: The system cannot find the file specified. (os error 2)
```

# Usage

fs-err's API is the same as `std::fs`, so migrating code to use it is easy.

```no_run
// use std::fs;
use fs_err as fs;

let contents = fs::read_to_string("foo.txt")?;

println!("Read foo.txt: {}", contents);

Ok::<(), std::io::Error>(())
```

fs-err uses `std::io::Error` for all errors. This helps fs-err
compose well with traits from the standard library like
`std::io::Read` and crates that use them like
[`serde_json`][`serde_json`](../serde_json/index.md):

```no_run
use fs_err::File;

let file = File::open("my-config.json")?;

// If an I/O error occurs inside serde_json, the error will include a file path
// as well as what operation was being performed.
let decoded: Vec<String> = serde_json::from_reader(file)?;

println!("Program config: {:?}", decoded);

Ok::<(), Box<dyn std::error::Error>>(())
```

# Feature flags

* `expose_original_error`: when enabled, the [`Error::source()`](https://doc.rust-lang.org/stable/std/error/trait.Error.html#method.source) method of errors returned by this crate return the original `io::Error`. To avoid duplication in error messages,
  this also suppresses printing its message in their `Display` implementation, so make sure that you are printing the full error chain.
* `debug`: Debug filesystem errors faster by exposing more information. When a filesystem command
  fails, the error message might say "file does not exist." But it won't say **why** it doesn't exist.
  Perhaps the programmer misspelled the filename, perhaps that directory doesn't exist, or if it does,
  but the current user doesn't have permissions to see the contents. This feature analyzes the filesystem
  to output various "facts" that will help a developer debug the root of the current error.
  * Warning: Exposes filesystem metadata. This feature exposes additional metadata about your filesystem
    such as directory contents and permissions, which may be sensitive. Only enable `debug` when
    error messages won't be displayed to the end user, or they have access to filesystem metadata some
    other way.
  * Warning: This may slow down your program. This feature will trigger additional filesystem calls when
    errors occur, which may cause performance issues. Do not use if filesystem errors are common on a
    performance-sensitive "hotpath." Use in scenarios where developer hours are more expensive than
    compute time.
  * To mitigate performance and security concerns, consider only enabling this feature in `dev-dependencies`:
  * Requires Rust 1.79 or later

```toml
[dev-dependencies]
fs-err = { features = ["debug"] }
```

To use with the `tokio` feature, use `debug_tokio`:

```toml
[dependencies]
fs-err = { features = ["debug_tokio", "tokio"] }
```

# Minimum Supported Rust Version

The oldest rust version this crate is tested on is **1.40**.

This crate will generally be conservative with rust version updates. It uses the [`autocfg`](https://crates.io/crates/autocfg) crate to allow wrapping new APIs without incrementing the MSRV.

If the `tokio` feature is enabled, this crate will inherit the MSRV of the selected [`tokio`](https://crates.io/crates/tokio) version.





## Contents

- [Modules](#modules)
  - [`dir`](#dir)
  - [`errors`](#errors)
  - [`file`](#file)
  - [`open_options`](#open_options)
  - [`os`](#os)
  - [`path`](#path)
  - [`private`](#private)
  - [`unix`](#unix)
  - [`unix`](#unix)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`ReadDir`](#readdir)
  - [`DirEntry`](#direntry)
  - [`File`](#file)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`read`](#read)
  - [`read_to_string`](#read_to_string)
  - [`write`](#write)
  - [`copy`](#copy)
  - [`create_dir`](#create_dir)
  - [`create_dir_all`](#create_dir_all)
  - [`remove_dir`](#remove_dir)
  - [`remove_dir_all`](#remove_dir_all)
  - [`remove_file`](#remove_file)
  - [`metadata`](#metadata)
  - [`exists`](#exists)
  - [`canonicalize`](#canonicalize)
  - [`hard_link`](#hard_link)
  - [`read_link`](#read_link)
  - [`rename`](#rename)
  - [`soft_link`](#soft_link)
  - [`symlink_metadata`](#symlink_metadata)
  - [`set_permissions`](#set_permissions)
  - [`initial_buffer_size`](#initial_buffer_size)
  - [`read_dir`](#read_dir)
  - [`open`](#open)
  - [`create`](#create)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dir`](#dir) | mod |  |
| [`errors`](#errors) | mod |  |
| [`file`](#file) | mod |  |
| [`open_options`](#open_options) | mod |  |
| [`os`](#os) | mod | OS-specific functionality. |
| [`path`](#path) | mod |  |
| [`private`](#private) | mod |  |
| [`unix`](#unix) | mod |  |
| [`unix`](#unix) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`ReadDir`](#readdir) | struct | Wrapper around [`std::fs::ReadDir`][std::fs::ReadDir] which adds more |
| [`DirEntry`](#direntry) | struct | Wrapper around [`std::fs::DirEntry`][std::fs::DirEntry] which adds more |
| [`File`](#file) | struct | Wrapper around [`std::fs::File`][std::fs::File] which adds more helpful |
| [`unnamed`](#unnamed) | trait |  |
| [`read`](#read) | fn | Read the entire contents of a file into a bytes vector. |
| [`read_to_string`](#read_to_string) | fn | Read the entire contents of a file into a string. |
| [`write`](#write) | fn | Write a slice as the entire contents of a file. |
| [`copy`](#copy) | fn | Copies the contents of one file to another. |
| [`create_dir`](#create_dir) | fn | Creates a new, empty directory at the provided path. |
| [`create_dir_all`](#create_dir_all) | fn | Recursively create a directory and all of its parent components if they are missing. |
| [`remove_dir`](#remove_dir) | fn | Removes an empty directory. |
| [`remove_dir_all`](#remove_dir_all) | fn | Removes a directory at this path, after removing all its contents. |
| [`remove_file`](#remove_file) | fn | Removes a file from the filesystem. |
| [`metadata`](#metadata) | fn | Given a path, query the file system to get information about a file, directory, etc. |
| [`exists`](#exists) | fn | Returns `Ok(true)` if the path points at an existing entity. |
| [`canonicalize`](#canonicalize) | fn | Returns the canonical, absolute form of a path with all intermediate components |
| [`hard_link`](#hard_link) | fn | Creates a new hard link on the filesystem. |
| [`read_link`](#read_link) | fn | Reads a symbolic link, returning the file that the link points to. |
| [`rename`](#rename) | fn | Rename a file or directory to a new name, replacing the original file if to already exists. |
| [`soft_link`](#soft_link) | fn | Wrapper for [`fs::soft_link`](https://doc.rust-lang.org/stable/std/fs/fn.soft_link.html). |
| [`symlink_metadata`](#symlink_metadata) | fn | Query the metadata about a file without following symlinks. |
| [`set_permissions`](#set_permissions) | fn | Changes the permissions found on a file or a directory. |
| [`initial_buffer_size`](#initial_buffer_size) | fn |  |
| [`read_dir`](#read_dir) | fn | Returns an iterator over the entries within a directory. |
| [`open`](#open) | fn |  |
| [`create`](#create) | fn |  |

## Modules

- [`dir`](dir/index.md) - 
- [`errors`](errors/index.md) - 
- [`file`](file/index.md) - 
- [`open_options`](open_options/index.md) - 
- [`os`](os/index.md) - OS-specific functionality.
- [`path`](path/index.md) - 
- [`private`](private/index.md) - 
- [`unix`](unix/index.md) - 
- [`unix`](unix/index.md) - 

## Structs

### `OpenOptions`

```rust
struct OpenOptions(fs::OpenOptions);
```

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- <span id="openoptions-from-options"></span>`fn from_options(options: fs::OpenOptions) -> Self`

- <span id="openoptions-options"></span>`fn options(&self) -> &fs::OpenOptions`

- <span id="openoptions-options-mut"></span>`fn options_mut(&mut self) -> &mut fs::OpenOptions`

#### Trait Implementations

##### `impl Clone for OpenOptions`

- <span id="openoptions-clone"></span>`fn clone(&self) -> OpenOptions` — [`OpenOptions`](#openoptions)

##### `impl Debug for OpenOptions`

- <span id="openoptions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OpenOptionsExt for crate::OpenOptions`

- <span id="crateopenoptions-mode"></span>`fn mode(&mut self, mode: u32) -> &mut Self`

- <span id="crateopenoptions-custom-flags"></span>`fn custom_flags(&mut self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

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

- <span id="readdir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for ReadDir`

- <span id="readdir-item"></span>`type Item = <I as Iterator>::Item`

- <span id="readdir-intoiter"></span>`type IntoIter = I`

- <span id="readdir-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ReadDir`

- <span id="readdir-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="readdir-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DirEntry`

```rust
struct DirEntry {
    inner: fs::DirEntry,
}
```

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

## Traits

## Functions

### `read`

```rust
fn read<P: AsRef<std::path::Path>>(path: P) -> io::Result<Vec<u8>>
```

Read the entire contents of a file into a bytes vector.

Wrapper for [`fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html).

### `read_to_string`

```rust
fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> io::Result<String>
```

Read the entire contents of a file into a string.

Wrapper for [`fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html).

### `write`

```rust
fn write<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()>
```

Write a slice as the entire contents of a file.

Wrapper for [`fs::write`](https://doc.rust-lang.org/stable/std/fs/fn.write.html).

### `copy`

```rust
fn copy<P, Q>(from: P, to: Q) -> io::Result<u64>
where
    P: AsRef<std::path::Path>,
    Q: AsRef<std::path::Path>
```

Copies the contents of one file to another. This function will also copy the
permission bits of the original file to the destination file.

Wrapper for [`fs::copy`](https://doc.rust-lang.org/stable/std/fs/fn.copy.html).

### `create_dir`

```rust
fn create_dir<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

Creates a new, empty directory at the provided path.

Wrapper for [`fs::create_dir`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir.html).

### `create_dir_all`

```rust
fn create_dir_all<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

Recursively create a directory and all of its parent components if they are missing.

Wrapper for [`fs::create_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir_all.html).

### `remove_dir`

```rust
fn remove_dir<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

Removes an empty directory.

Wrapper for [`fs::remove_dir`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir.html).

### `remove_dir_all`

```rust
fn remove_dir_all<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

Removes a directory at this path, after removing all its contents. Use carefully!

Wrapper for [`fs::remove_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html).

### `remove_file`

```rust
fn remove_file<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

Removes a file from the filesystem.

Wrapper for [`fs::remove_file`](https://doc.rust-lang.org/stable/std/fs/fn.remove_file.html).

### `metadata`

```rust
fn metadata<P: AsRef<std::path::Path>>(path: P) -> io::Result<fs::Metadata>
```

Given a path, query the file system to get information about a file, directory, etc.

Wrapper for [`fs::metadata`](https://doc.rust-lang.org/stable/std/fs/fn.metadata.html).

### `exists`

```rust
fn exists<P: AsRef<std::path::Path>>(path: P) -> io::Result<bool>
```

Returns `Ok(true)` if the path points at an existing entity.

Wrapper for [`fs::exists`](https://doc.rust-lang.org/stable/std/fs/fn.exists.html).

### `canonicalize`

```rust
fn canonicalize<P: AsRef<std::path::Path>>(path: P) -> io::Result<std::path::PathBuf>
```

Returns the canonical, absolute form of a path with all intermediate components
normalized and symbolic links resolved.

Wrapper for [`fs::canonicalize`](https://doc.rust-lang.org/stable/std/fs/fn.canonicalize.html).

### `hard_link`

```rust
fn hard_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

Creates a new hard link on the filesystem.

Wrapper for [`fs::hard_link`](https://doc.rust-lang.org/stable/std/fs/fn.hard_link.html).

### `read_link`

```rust
fn read_link<P: AsRef<std::path::Path>>(path: P) -> io::Result<std::path::PathBuf>
```

Reads a symbolic link, returning the file that the link points to.

Wrapper for [`fs::read_link`](https://doc.rust-lang.org/stable/std/fs/fn.read_link.html).

### `rename`

```rust
fn rename<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(from: P, to: Q) -> io::Result<()>
```

Rename a file or directory to a new name, replacing the original file if to already exists.

Wrapper for [`fs::rename`](https://doc.rust-lang.org/stable/std/fs/fn.rename.html).

### `soft_link`

```rust
fn soft_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

Wrapper for [`fs::soft_link`](https://doc.rust-lang.org/stable/std/fs/fn.soft_link.html).

### `symlink_metadata`

```rust
fn symlink_metadata<P: AsRef<std::path::Path>>(path: P) -> io::Result<fs::Metadata>
```

Query the metadata about a file without following symlinks.

Wrapper for [`fs::symlink_metadata`](https://doc.rust-lang.org/stable/std/fs/fn.symlink_metadata.html).

### `set_permissions`

```rust
fn set_permissions<P: AsRef<std::path::Path>>(path: P, perm: fs::Permissions) -> io::Result<()>
```

Changes the permissions found on a file or a directory.

Wrapper for [`fs::set_permissions`](https://doc.rust-lang.org/stable/std/fs/fn.set_permissions.html).

### `initial_buffer_size`

```rust
fn initial_buffer_size(file: &std::fs::File) -> usize
```

### `read_dir`

```rust
fn read_dir<P: Into<std::path::PathBuf>>(path: P) -> io::Result<ReadDir>
```

Returns an iterator over the entries within a directory.

Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html).

### `open`

```rust
fn open(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

