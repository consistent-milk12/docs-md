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
[`serde_json`][serde_json](#serde-json):

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

[std::fs]: https://doc.rust-lang.org/stable/std/fs/
[std::io::Error]: https://doc.rust-lang.org/stable/std/io/struct.Error.html
[std::io::Read]: https://doc.rust-lang.org/stable/std/io/trait.Read.html
[serde_json](#serde-json): https://crates.io/crates/serde_json

## Modules

- [`os`](os/index.md) - OS-specific functionality.

## Structs

### `OpenOptions`

```rust
struct OpenOptions();
```

Wrapper around [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)

#### Implementations

- `fn new() -> Self`
  Creates a blank new set of options ready for configuration.

- `fn read(self: &mut Self, read: bool) -> &mut Self`
  Sets the option for read access.

- `fn write(self: &mut Self, write: bool) -> &mut Self`
  Sets the option for write access.

- `fn append(self: &mut Self, append: bool) -> &mut Self`
  Sets the option for the append mode.

- `fn truncate(self: &mut Self, truncate: bool) -> &mut Self`
  Sets the option for truncating a previous file.

- `fn create(self: &mut Self, create: bool) -> &mut Self`
  Sets the option to create a new file, or open it if it already exists.

- `fn create_new(self: &mut Self, create_new: bool) -> &mut Self`
  Sets the option to create a new file, failing if it already exists.

- `fn open<P>(self: &Self, path: P) -> io::Result<crate::File>`
  Opens a file at `path` with the options specified by `self`.

- `fn from_options(options: fs::OpenOptions) -> Self`
  Constructs `Self` from [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html)

- `fn options(self: &Self) -> &fs::OpenOptions`
  Returns a reference to the underlying [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html).

- `fn options_mut(self: &mut Self) -> &mut fs::OpenOptions`
  Returns a mutable reference to the underlying [`std::fs::OpenOptions`](https://doc.rust-lang.org/stable/std/fs/struct.OpenOptions.html).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OpenOptions`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl OpenOptionsExt`

- `fn mode(self: &mut Self, mode: u32) -> &mut Self`

- `fn custom_flags(self: &mut Self, flags: i32) -> &mut Self`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

