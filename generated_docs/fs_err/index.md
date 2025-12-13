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
  - [`open_options`](#open-options)
  - [`os`](#os)
  - [`path`](#path)
  - [`private`](#private)
  - [`unix`](#unix)
  - [`unix`](#unix)
- [Structs](#structs)
  - [`OpenOptions`](#openoptions)
  - [`ReadDir`](#readdir)
  - [`DirEntry`](#direntry)
  - [`File`](#file)
- [Traits](#traits)
  - [`PathExt`](#pathext)
- [Functions](#functions)
  - [`read`](#read)
  - [`read_to_string`](#read-to-string)
  - [`write`](#write)
  - [`copy`](#copy)
  - [`create_dir`](#create-dir)
  - [`create_dir_all`](#create-dir-all)
  - [`remove_dir`](#remove-dir)
  - [`remove_dir_all`](#remove-dir-all)
  - [`remove_file`](#remove-file)
  - [`metadata`](#metadata)
  - [`exists`](#exists)
  - [`canonicalize`](#canonicalize)
  - [`hard_link`](#hard-link)
  - [`read_link`](#read-link)
  - [`rename`](#rename)
  - [`soft_link`](#soft-link)
  - [`symlink_metadata`](#symlink-metadata)
  - [`set_permissions`](#set-permissions)
  - [`initial_buffer_size`](#initial-buffer-size)
  - [`read_dir`](#read-dir)
  - [`open`](#open)
  - [`create`](#create)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dir`](#dir) | mod |  |
| [`errors`](#errors) | mod |  |
| [`file`](#file) | mod |  |
| [`open_options`](#open-options) | mod |  |
| [`os`](#os) | mod | OS-specific functionality. |
| [`path`](#path) | mod |  |
| [`private`](#private) | mod |  |
| [`unix`](#unix) | mod |  |
| [`unix`](#unix) | mod |  |
| [`OpenOptions`](#openoptions) | struct |  |
| [`ReadDir`](#readdir) | struct | Wrapper around [`std::fs::ReadDir`][std::fs::ReadDir] which adds more helpful information to all errors. |
| [`DirEntry`](#direntry) | struct | Wrapper around [`std::fs::DirEntry`][std::fs::DirEntry] which adds more helpful information to all errors. |
| [`File`](#file) | struct | Wrapper around [`std::fs::File`][std::fs::File] which adds more helpful information to all errors. |
| [`PathExt`](#pathext) | trait |  |
| [`read`](#read) | fn | Read the entire contents of a file into a bytes vector. |
| [`read_to_string`](#read-to-string) | fn | Read the entire contents of a file into a string. |
| [`write`](#write) | fn | Write a slice as the entire contents of a file. |
| [`copy`](#copy) | fn | Copies the contents of one file to another. |
| [`create_dir`](#create-dir) | fn | Creates a new, empty directory at the provided path. |
| [`create_dir_all`](#create-dir-all) | fn | Recursively create a directory and all of its parent components if they are missing. |
| [`remove_dir`](#remove-dir) | fn | Removes an empty directory. |
| [`remove_dir_all`](#remove-dir-all) | fn | Removes a directory at this path, after removing all its contents. |
| [`remove_file`](#remove-file) | fn | Removes a file from the filesystem. |
| [`metadata`](#metadata) | fn | Given a path, query the file system to get information about a file, directory, etc. |
| [`exists`](#exists) | fn | Returns `Ok(true)` if the path points at an existing entity. |
| [`canonicalize`](#canonicalize) | fn | Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved. |
| [`hard_link`](#hard-link) | fn | Creates a new hard link on the filesystem. |
| [`read_link`](#read-link) | fn | Reads a symbolic link, returning the file that the link points to. |
| [`rename`](#rename) | fn | Rename a file or directory to a new name, replacing the original file if to already exists. |
| [`soft_link`](#soft-link) | fn | Wrapper for [`fs::soft_link`](https://doc.rust-lang.org/stable/std/fs/fn.soft_link.html). |
| [`symlink_metadata`](#symlink-metadata) | fn | Query the metadata about a file without following symlinks. |
| [`set_permissions`](#set-permissions) | fn | Changes the permissions found on a file or a directory. |
| [`initial_buffer_size`](#initial-buffer-size) | fn |  |
| [`read_dir`](#read-dir) | fn | Returns an iterator over the entries within a directory. |
| [`open`](#open) | fn |  |
| [`create`](#create) | fn |  |

## Modules

- [`dir`](dir/index.md)
- [`errors`](errors/index.md)
- [`file`](file/index.md)
- [`open_options`](open_options/index.md)
- [`os`](os/index.md) — OS-specific functionality.
- [`path`](path/index.md)
- [`private`](private/index.md)
- [`unix`](unix/index.md)
- [`unix`](unix/index.md)

## Structs

### `OpenOptions`

```rust
struct OpenOptions(fs::OpenOptions);
```

*Defined in [`fs-err-3.2.0/src/open_options.rs:7`](../../.source_1765521767/fs-err-3.2.0/src/open_options.rs#L7)*

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- <span id="openoptions-new"></span>`fn new() -> Self`

  Creates a blank new set of options ready for configuration.

  

  Wrapper for [`std::fs::OpenOptions::new`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.new)

- <span id="openoptions-read"></span>`fn read(&mut self, read: bool) -> &mut Self`

  Sets the option for read access.

  

  Wrapper for [`std::fs::OpenOptions::read`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.read)

- <span id="openoptions-write"></span>`fn write(&mut self, write: bool) -> &mut Self`

  Sets the option for write access.

  

  Wrapper for [`std::fs::OpenOptions::write`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write)

- <span id="openoptions-append"></span>`fn append(&mut self, append: bool) -> &mut Self`

  Sets the option for the append mode.

  

  Wrapper for [`std::fs::OpenOptions::append`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append)

- <span id="openoptions-truncate"></span>`fn truncate(&mut self, truncate: bool) -> &mut Self`

  Sets the option for truncating a previous file.

  

  Wrapper for [`std::fs::OpenOptions::truncate`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate)

- <span id="openoptions-create"></span>`fn create(&mut self, create: bool) -> &mut Self`

  Sets the option to create a new file, or open it if it already exists.

  

  Wrapper for [`std::fs::OpenOptions::create`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create)

- <span id="openoptions-create-new"></span>`fn create_new(&mut self, create_new: bool) -> &mut Self`

  Sets the option to create a new file, failing if it already exists.

  

  Wrapper for [`std::fs::OpenOptions::create_new`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new)

- <span id="openoptions-open"></span>`fn open<P>(&self, path: P) -> io::Result<crate::File>` — [`File`](#file)

  Opens a file at `path` with the options specified by `self`.

  

  Wrapper for [`std::fs::OpenOptions::open`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open)

#### Trait Implementations

##### `impl Any for OpenOptions`

- <span id="openoptions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OpenOptions`

- <span id="openoptions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OpenOptions`

- <span id="openoptions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OpenOptions`

- <span id="openoptions-clone"></span>`fn clone(&self) -> OpenOptions` — [`OpenOptions`](open_options/index.md#openoptions)

##### `impl CloneToUninit for OpenOptions`

- <span id="openoptions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OpenOptions`

- <span id="openoptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OpenOptions`

- <span id="openoptions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OpenOptions`

- <span id="openoptions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OpenOptionsExt for crate::OpenOptions`

- <span id="crateopenoptions-openoptionsext-mode"></span>`fn mode(&mut self, mode: u32) -> &mut Self`

- <span id="crateopenoptions-openoptionsext-custom-flags"></span>`fn custom_flags(&mut self, flags: i32) -> &mut Self`

##### `impl Sealed for crate::OpenOptions`

##### `impl ToOwned for OpenOptions`

- <span id="openoptions-toowned-type-owned"></span>`type Owned = T`

- <span id="openoptions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="openoptions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OpenOptions`

- <span id="openoptions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="openoptions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OpenOptions`

- <span id="openoptions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="openoptions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReadDir`

```rust
struct ReadDir {
    inner: fs::ReadDir,
    path: std::path::PathBuf,
}
```

*Defined in [`fs-err-3.2.0/src/dir.rs:28-31`](../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L28-L31)*

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

*Defined in [`fs-err-3.2.0/src/dir.rs:51-53`](../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L51-L53)*

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

### `File`

```rust
struct File {
    file: fs::File,
    path: std::path::PathBuf,
}
```

*Defined in [`fs-err-3.2.0/src/file.rs:13-16`](../../.source_1765521767/fs-err-3.2.0/src/file.rs#L13-L16)*

Wrapper around `std::fs::File` which adds more helpful
information to all errors.


#### Implementations

- <span id="file-open"></span>`fn open<P>(path: P) -> Result<Self, io::Error>`

  Attempts to open a file in read-only mode.

  

  Wrapper for [`File::open`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open).

- <span id="file-create"></span>`fn create<P>(path: P) -> Result<Self, io::Error>`

  Opens a file in write-only mode.

  

  Wrapper for [`File::create`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create).

- <span id="file-create-new"></span>`fn create_new<P>(path: P) -> Result<Self, io::Error>`

  Opens a file in read-write mode.

  

  Wrapper for [`File::create_new`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create_new).

- <span id="file-options"></span>`fn options() -> OpenOptions` — [`OpenOptions`](open_options/index.md#openoptions)

  Returns a new `OpenOptions` object.

  

  Wrapper for [`File::options`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.options).

- <span id="file-sync-all"></span>`fn sync_all(&self) -> Result<(), io::Error>`

  Attempts to sync all OS-internal metadata to disk.

  

  Wrapper for [`File::sync_all`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_all).

- <span id="file-sync-data"></span>`fn sync_data(&self) -> Result<(), io::Error>`

  This function is similar to `sync_all`, except that it might not synchronize file metadata to the filesystem.

  

  Wrapper for [`File::sync_data`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.sync_data).

- <span id="file-set-len"></span>`fn set_len(&self, size: u64) -> Result<(), io::Error>`

  Truncates or extends the underlying file, updating the size of this file to become `size`.

  

  Wrapper for [`File::set_len`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_len).

- <span id="file-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata, io::Error>`

  Queries metadata about the underlying file.

  

  Wrapper for [`File::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.metadata).

- <span id="file-try-clone"></span>`fn try_clone(&self) -> Result<Self, io::Error>`

  Creates a new `File` instance that shares the same underlying file handle as the

  existing `File` instance. Reads, writes, and seeks will affect both `File`

  instances simultaneously.

  

  Wrapper for [`File::try_clone`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.try_clone).

- <span id="file-set-permissions"></span>`fn set_permissions(&self, perm: fs::Permissions) -> Result<(), io::Error>`

  Changes the permissions on the underlying file.

  

  Wrapper for [`File::set_permissions`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_permissions).

#### Trait Implementations

##### `impl Any for File`

- <span id="file-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsFd for crate::File`

- <span id="cratefile-asfd-as-fd"></span>`fn as_fd(&self) -> BorrowedFd<'_>`

##### `impl AsRawFd for crate::File`

- <span id="cratefile-asrawfd-as-raw-fd"></span>`fn as_raw_fd(&self) -> RawFd`

##### `impl<T> Borrow for File`

- <span id="file-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for File`

- <span id="file-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for File`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FileExt for crate::File`

- <span id="cratefile-fileext-read-at"></span>`fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>`

- <span id="cratefile-fileext-write-at"></span>`fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>`

##### `impl<T> From for File`

- <span id="file-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for File`

- <span id="file-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoRawFd for crate::File`

- <span id="cratefile-intorawfd-into-raw-fd"></span>`fn into_raw_fd(self) -> RawFd`

##### `impl Read for File`

- <span id="file-read"></span>`fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>`

- <span id="file-read-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize>`

##### `impl Sealed for crate::File`

##### `impl Seek for File`

- <span id="file-seek"></span>`fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>`

##### `impl<U> TryFrom for File`

- <span id="file-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="file-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for File`

- <span id="file-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="file-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for File`

- <span id="file-write"></span>`fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>`

- <span id="file-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize>`

- <span id="file-write-flush"></span>`fn flush(&mut self) -> std::io::Result<()>`

## Traits

### `PathExt`

```rust
trait PathExt: crate::Sealed { ... }
```

*Defined in [`fs-err-3.2.0/src/path.rs:12-39`](../../.source_1765521767/fs-err-3.2.0/src/path.rs#L12-L39)*

Defines aliases on [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) for `fs_err` functions.

This trait is sealed and can not be implemented by other crates.

#### Required Methods

- `fn fs_err_try_exists(&self) -> io::Result<bool>`

  Returns Ok(true) if the path points at an existing entity.

- `fn fs_err_metadata(&self) -> io::Result<fs::Metadata>`

  Given a path, query the file system to get information about a file, directory, etc.

- `fn fs_err_symlink_metadata(&self) -> io::Result<fs::Metadata>`

  Query the metadata about a file without following symlinks.

- `fn fs_err_canonicalize(&self) -> io::Result<PathBuf>`

  Returns the canonical, absolute form of a path with all intermediate components

- `fn fs_err_read_link(&self) -> io::Result<PathBuf>`

  Reads a symbolic link, returning the file that the link points to.

- `fn fs_err_read_dir(&self) -> io::Result<crate::ReadDir>`

  Returns an iterator over the entries within a directory.

#### Implementors

- `std::path::Path`

## Functions

### `read`

```rust
fn read<P: AsRef<std::path::Path>>(path: P) -> io::Result<Vec<u8>>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:136-143`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L136-L143)*

Read the entire contents of a file into a bytes vector.

Wrapper for [`fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html).

### `read_to_string`

```rust
fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> io::Result<String>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:148-155`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L148-L155)*

Read the entire contents of a file into a string.

Wrapper for [`fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html).

### `write`

```rust
fn write<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:160-166`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L160-L166)*

Write a slice as the entire contents of a file.

Wrapper for [`fs::write`](https://doc.rust-lang.org/stable/std/fs/fn.write.html).

### `copy`

```rust
fn copy<P, Q>(from: P, to: Q) -> io::Result<u64>
where
    P: AsRef<std::path::Path>,
    Q: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:172-181`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L172-L181)*

Copies the contents of one file to another. This function will also copy the
permission bits of the original file to the destination file.

Wrapper for [`fs::copy`](https://doc.rust-lang.org/stable/std/fs/fn.copy.html).

### `create_dir`

```rust
fn create_dir<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:186-192`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L186-L192)*

Creates a new, empty directory at the provided path.

Wrapper for [`fs::create_dir`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir.html).

### `create_dir_all`

```rust
fn create_dir_all<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:197-203`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L197-L203)*

Recursively create a directory and all of its parent components if they are missing.

Wrapper for [`fs::create_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir_all.html).

### `remove_dir`

```rust
fn remove_dir<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:208-214`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L208-L214)*

Removes an empty directory.

Wrapper for [`fs::remove_dir`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir.html).

### `remove_dir_all`

```rust
fn remove_dir_all<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:219-225`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L219-L225)*

Removes a directory at this path, after removing all its contents. Use carefully!

Wrapper for [`fs::remove_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html).

### `remove_file`

```rust
fn remove_file<P>(path: P) -> io::Result<()>
where
    P: AsRef<std::path::Path>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:230-236`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L230-L236)*

Removes a file from the filesystem.

Wrapper for [`fs::remove_file`](https://doc.rust-lang.org/stable/std/fs/fn.remove_file.html).

### `metadata`

```rust
fn metadata<P: AsRef<std::path::Path>>(path: P) -> io::Result<fs::Metadata>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:241-244`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L241-L244)*

Given a path, query the file system to get information about a file, directory, etc.

Wrapper for [`fs::metadata`](https://doc.rust-lang.org/stable/std/fs/fn.metadata.html).

### `exists`

```rust
fn exists<P: AsRef<std::path::Path>>(path: P) -> io::Result<bool>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:250-253`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L250-L253)*

Returns `Ok(true)` if the path points at an existing entity.

Wrapper for [`fs::exists`](https://doc.rust-lang.org/stable/std/fs/fn.exists.html).

### `canonicalize`

```rust
fn canonicalize<P: AsRef<std::path::Path>>(path: P) -> io::Result<std::path::PathBuf>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:259-262`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L259-L262)*

Returns the canonical, absolute form of a path with all intermediate components
normalized and symbolic links resolved.

Wrapper for [`fs::canonicalize`](https://doc.rust-lang.org/stable/std/fs/fn.canonicalize.html).

### `hard_link`

```rust
fn hard_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:267-272`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L267-L272)*

Creates a new hard link on the filesystem.

Wrapper for [`fs::hard_link`](https://doc.rust-lang.org/stable/std/fs/fn.hard_link.html).

### `read_link`

```rust
fn read_link<P: AsRef<std::path::Path>>(path: P) -> io::Result<std::path::PathBuf>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:277-280`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L277-L280)*

Reads a symbolic link, returning the file that the link points to.

Wrapper for [`fs::read_link`](https://doc.rust-lang.org/stable/std/fs/fn.read_link.html).

### `rename`

```rust
fn rename<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(from: P, to: Q) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:285-290`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L285-L290)*

Rename a file or directory to a new name, replacing the original file if to already exists.

Wrapper for [`fs::rename`](https://doc.rust-lang.org/stable/std/fs/fn.rename.html).

### `soft_link`

```rust
fn soft_link<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(src: P, dst: Q) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:295-301`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L295-L301)*

Wrapper for [`fs::soft_link`](https://doc.rust-lang.org/stable/std/fs/fn.soft_link.html).

### `symlink_metadata`

```rust
fn symlink_metadata<P: AsRef<std::path::Path>>(path: P) -> io::Result<fs::Metadata>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:306-310`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L306-L310)*

Query the metadata about a file without following symlinks.

Wrapper for [`fs::symlink_metadata`](https://doc.rust-lang.org/stable/std/fs/fn.symlink_metadata.html).

### `set_permissions`

```rust
fn set_permissions<P: AsRef<std::path::Path>>(path: P, perm: fs::Permissions) -> io::Result<()>
```

*Defined in [`fs-err-3.2.0/src/lib.rs:315-319`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L315-L319)*

Changes the permissions found on a file or a directory.

Wrapper for [`fs::set_permissions`](https://doc.rust-lang.org/stable/std/fs/fn.set_permissions.html).

### `initial_buffer_size`

```rust
fn initial_buffer_size(file: &std::fs::File) -> usize
```

*Defined in [`fs-err-3.2.0/src/lib.rs:321-323`](../../.source_1765521767/fs-err-3.2.0/src/lib.rs#L321-L323)*

### `read_dir`

```rust
fn read_dir<P: Into<std::path::PathBuf>>(path: P) -> io::Result<ReadDir>
```

*Defined in [`fs-err-3.2.0/src/dir.rs:11-18`](../../.source_1765521767/fs-err-3.2.0/src/dir.rs#L11-L18)*

Returns an iterator over the entries within a directory.

Wrapper for [`fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html).

### `open`

```rust
fn open(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:20-22`](../../.source_1765521767/fs-err-3.2.0/src/file.rs#L20-L22)*

### `create`

```rust
fn create(path: &std::path::Path) -> Result<std::fs::File, impl FnOnce(std::path::PathBuf) -> io::Error>
```

*Defined in [`fs-err-3.2.0/src/file.rs:25-27`](../../.source_1765521767/fs-err-3.2.0/src/file.rs#L25-L27)*

