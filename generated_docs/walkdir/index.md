# Crate `walkdir`

Crate `walkdir` provides an efficient and cross platform implementation
of recursive directory traversal. Several options are exposed to control
iteration, such as whether to follow symbolic links (default off), limit the
maximum number of simultaneous open file descriptors and the ability to
efficiently skip descending into directories.

To use this crate, add `walkdir` as a dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
walkdir = "2"
```

# From the top

The [`WalkDir`](#walkdir) type builds iterators. The [`DirEntry`](dent/index.md) type describes values
yielded by the iterator. Finally, the [`Error`](error/index.md) type is a small wrapper around
`std::io::Error` with additional information, such as if a loop was detected
while following symbolic links (not enabled by default).




# Example

The following code recursively iterates over the directory given and prints
the path for each entry:

```no_run
use walkdir::WalkDir;
use walkdir::Error;

fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo") {
    println!("{}", entry?.path().display());
}
Ok(())
}
```

Or, if you'd like to iterate over all entries and ignore any errors that
may arise, use `filter_map`. (e.g., This code below will silently skip
directories that the owner of the running process does not have permission to
access.)

```no_run
use walkdir::WalkDir;

for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
}
```

# Example: follow symbolic links

The same code as above, except `follow_links` is enabled:

```no_run
use walkdir::WalkDir;
use walkdir::Error;

fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").follow_links(true) {
    println!("{}", entry?.path().display());
}
Ok(())
}
```

# Example: skip hidden files and directories on unix

This uses the `filter_entry` iterator adapter to avoid yielding hidden files
and directories efficiently (i.e. without recursing into hidden directories):

```no_run
use walkdir::{DirEntry, WalkDir};
use walkdir::Error;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn try_main() -> Result<(), Error> {
let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
Ok(())
}
```


## Contents

- [Modules](#modules)
  - [`dent`](#dent)
  - [`error`](#error)
  - [`util`](#util)
- [Structs](#structs)
  - [`DirEntry`](#direntry)
  - [`Error`](#error)
  - [`WalkDir`](#walkdir)
  - [`WalkDirOptions`](#walkdiroptions)
  - [`IntoIter`](#intoiter)
  - [`Ancestor`](#ancestor)
  - [`FilterEntry`](#filterentry)
- [Enums](#enums)
  - [`DirList`](#dirlist)
- [Traits](#traits)
  - [`DirEntryExt`](#direntryext)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
- [Macros](#macros)
  - [`itry!`](#itry)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dent`](#dent) | mod |  |
| [`error`](#error) | mod |  |
| [`util`](#util) | mod |  |
| [`DirEntry`](#direntry) | struct |  |
| [`Error`](#error) | struct |  |
| [`WalkDir`](#walkdir) | struct | A builder to create an iterator for recursively walking a directory. |
| [`WalkDirOptions`](#walkdiroptions) | struct |  |
| [`IntoIter`](#intoiter) | struct | An iterator for recursively descending into a directory. |
| [`Ancestor`](#ancestor) | struct | An ancestor is an item in the directory tree traversed by walkdir, and is used to check for loops in the tree when traversing symlinks. |
| [`FilterEntry`](#filterentry) | struct | A recursive directory iterator that skips entries. |
| [`DirList`](#dirlist) | enum | A sequence of unconsumed directory entries. |
| [`DirEntryExt`](#direntryext) | trait |  |
| [`Result`](#result) | type | A result type for walkdir operations. |
| [`itry!`](#itry) | macro | Like try, but for iterators that return [`Option<Result<_, _>>`]. |

## Modules

- [`dent`](dent/index.md)
- [`error`](error/index.md)
- [`util`](util/index.md)

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

*Defined in [`walkdir-2.5.0/src/dent.rs:35-59`](../../.source_1765633015/walkdir-2.5.0/src/dent.rs#L35-L59)*

A directory entry.

This is the type of value that is yielded from the iterators defined in
this crate.

On Unix systems, this type implements the [`DirEntryExt`](dent/index.md) trait, which
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

  

  

- <span id="direntry-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata>` — [`Result`](#result)

  Return the metadata for the file that this entry points to.

  

  This will follow symbolic links if and only if the [`WalkDir`](#walkdir) value

  has `follow_links` enabled.

  

  # Platform behavior

  

  This always calls `std::fs::symlink_metadata`.

  

  If this entry is a symbolic link and `follow_links` is enabled, then

  `std::fs::metadata` is called instead.

  

  # Errors

  

  Similar to `std::fs::metadata`, returns errors for path values that

  the program does not have permissions to access or if the path does not

  exist.

  

  

  

- <span id="direntry-metadata-internal"></span>`fn metadata_internal(&self) -> Result<fs::Metadata>` — [`Result`](#result)

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

- <span id="direntry-from-entry"></span>`fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](#result)

- <span id="direntry-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](#result), [`DirEntry`](dent/index.md#direntry)

#### Trait Implementations

##### `impl Any for DirEntry`

- <span id="direntry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DirEntry`

- <span id="direntry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DirEntry`

- <span id="direntry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DirEntry`

- <span id="direntry-clone"></span>`fn clone(&self) -> DirEntry` — [`DirEntry`](dent/index.md#direntry)

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

### `Error`

```rust
struct Error {
    depth: usize,
    inner: ErrorInner,
}
```

*Defined in [`walkdir-2.5.0/src/error.rs:28-31`](../../.source_1765633015/walkdir-2.5.0/src/error.rs#L28-L31)*

An error produced by recursively walking a directory.

This error type is a light wrapper around `std::io::Error`. In
particular, it adds the following information:

* The depth at which the error occurred in the file tree, relative to the
root.
* The path, if any, associated with the IO error.
* An indication that a loop occurred when following symbolic links. In this
case, there is no underlying IO error.

To maintain good ergonomics, this type has a
[`impl From<Error> for std::io::Error`][impl] defined which preserves the original context.
This allows you to use an `io::Result` with methods in this crate if you don't care about
accessing the underlying error data in a structured form.




#### Implementations

- <span id="error-path"></span>`fn path(&self) -> Option<&Path>`

  Returns the path associated with this error if one exists.

  

  For example, if an error occurred while opening a directory handle,

  the error will include the path passed to `std::fs::read_dir`.

- <span id="error-loop-ancestor"></span>`fn loop_ancestor(&self) -> Option<&Path>`

  Returns the path at which a cycle was detected.

  

  If no cycle was detected, [`None`](#none) is returned.

  

  A cycle is detected when a directory entry is equivalent to one of

  its ancestors.

  

  To get the path to the child directory entry in the cycle, use the

  [`path`](#path) method.

  

- <span id="error-depth"></span>`fn depth(&self) -> usize`

  Returns the depth at which this error occurred relative to the root.

  

  The smallest depth is `0` and always corresponds to the path given to

  the `new` function on [`WalkDir`](#walkdir). Its direct descendents have depth

  `1`, and their descendents have depth `2`, and so on.

  

- <span id="error-io-error"></span>`fn io_error(&self) -> Option<&io::Error>`

  Inspect the original `io::Error` if there is one.

  

  [`None`](#none) is returned if the [`Error`](error/index.md) doesn't correspond to an

  `io::Error`. This might happen, for example, when the error was

  produced because a cycle was found in the directory tree while

  following symbolic links.

  

  This method returns a borrowed value that is bound to the lifetime of the [`Error`](error/index.md). To

  obtain an owned value, the `into_io_error` can be used instead.

  

  > This is the original `io::Error` and is _not_ the same as

  > [`impl From<Error> for std::io::Error`][impl] which contains additional context about the

  error.

  

  # Example

  

  ```rust,no_run

  use std::io;

  use std::path::Path;

  

  use walkdir::WalkDir;

  

  for entry in WalkDir::new("foo") {

      match entry {

          Ok(entry) => println!("{}", entry.path().display()),

          Err(err) => {

              let path = err.path().unwrap_or(Path::new("")).display();

              println!("failed to access entry {}", path);

              if let Some(inner) = err.io_error() {

                  match inner.kind() {

                      io::ErrorKind::InvalidData => {

                          println!(

                              "entry contains invalid data: {}",

                              inner)

                      }

                      io::ErrorKind::PermissionDenied => {

                          println!(

                              "Missing permission to read entry: {}",

                              inner)

                      }

                      _ => {

                          println!(

                              "Unexpected error occurred: {}",

                              inner)

                      }

                  }

              }

          }

      }

  }

  ```

  

  

  

  

  

- <span id="error-into-io-error"></span>`fn into_io_error(self) -> Option<io::Error>`

  Similar to `io_error` except consumes self to convert to the original

  `io::Error` if one exists.

  

- <span id="error-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- <span id="error-from-entry"></span>`fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](dent/index.md#direntry)

- <span id="error-from-io"></span>`fn from_io(depth: usize, err: io::Error) -> Self`

- <span id="error-from-loop"></span>`fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-error-description"></span>`fn description(&self) -> &str`

- <span id="error-error-cause"></span>`fn cause(&self) -> Option<&dyn error::Error>`

- <span id="error-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkDir`

```rust
struct WalkDir {
    opts: WalkDirOptions,
    root: std::path::PathBuf,
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:234-237`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L234-L237)*

A builder to create an iterator for recursively walking a directory.

Results are returned in depth first fashion, with directories yielded
before their contents. If `contents_first` is true, contents are yielded
before their directories. The order is unspecified but if `sort_by` is
given, directory entries are sorted according to this function. Directory
entries `.` and `..` are always omitted.

If an error occurs at any point during iteration, then it is returned in
place of its corresponding directory entry and iteration continues as
normal. If an error occurs while opening a directory for reading, then it
is not descended into (but the error is still yielded by the iterator).
Iteration may be stopped at any time. When the iterator is destroyed, all
resources associated with it are freed.


# Usage

This type implements `IntoIterator` so that it may be used as the subject
of a `for` loop. You may need to call [`into_iter`](#into-iter) explicitly if you want
to use iterator adapters such as `filter_entry`.

Idiomatic use of this type should use method chaining to set desired
options. For example, this only shows entries with a depth of `1`, `2` or
`3` (relative to `foo`):

```no_run
use walkdir::WalkDir;
use walkdir::Error;

fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1).max_depth(3) {
    println!("{}", entry?.path().display());
}
Ok(())
}
```



Note that the iterator by default includes the top-most directory. Since
this is the only directory yielded with depth `0`, it is easy to ignore it
with the `min_depth` setting:

```no_run
use walkdir::WalkDir;
use walkdir::Error;

fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1) {
    println!("{}", entry?.path().display());
}
Ok(())
}
```

This will only return descendents of the `foo` directory and not `foo`
itself.

# Loops

This iterator (like most/all recursive directory iterators) assumes that
no loops can be made with *hard* links on your file system. In particular,
this would require creating a hard link to a directory such that it creates
a loop. On most platforms, this operation is illegal.

Note that when following symbolic/soft links, loops are detected and an
error is reported.

#### Implementations

- <span id="walkdir-new"></span>`fn new<P: AsRef<Path>>(root: P) -> Self`

  Create a builder for a recursive directory iterator starting at the

  file path `root`. If `root` is a directory, then it is the first item

  yielded by the iterator. If `root` is a file, then it is the first

  and only item yielded by the iterator. If `root` is a symlink, then it

  is always followed for the purposes of directory traversal. (A root

  `DirEntry` still obeys its documentation with respect to symlinks and

  the `follow_links` setting.)

- <span id="walkdir-min-depth"></span>`fn min_depth(self, depth: usize) -> Self`

  Set the minimum depth of entries yielded by the iterator.

  

  The smallest depth is `0` and always corresponds to the path given

  to the `new` function on this type. Its direct descendents have depth

  `1`, and their descendents have depth `2`, and so on.

- <span id="walkdir-max-depth"></span>`fn max_depth(self, depth: usize) -> Self`

  Set the maximum depth of entries yield by the iterator.

  

  The smallest depth is `0` and always corresponds to the path given

  to the `new` function on this type. Its direct descendents have depth

  `1`, and their descendents have depth `2`, and so on.

  

  Note that this will not simply filter the entries of the iterator, but

  it will actually avoid descending into directories when the depth is

  exceeded.

- <span id="walkdir-follow-links"></span>`fn follow_links(self, yes: bool) -> Self`

  Follow symbolic links. By default, this is disabled.

  

  When `yes` is `true`, symbolic links are followed as if they were

  normal directories and files. If a symbolic link is broken or is

  involved in a loop, an error is yielded.

  

  When enabled, the yielded [`DirEntry`](dent/index.md) values represent the target of

  the link while the path corresponds to the link. See the [`DirEntry`](dent/index.md)

  type for more details.

- <span id="walkdir-follow-root-links"></span>`fn follow_root_links(self, yes: bool) -> Self`

  Follow symbolic links if these are the root of the traversal.

  By default, this is enabled.

  

  When `yes` is `true`, symbolic links on root paths are followed

  which is effective if the symbolic link points to a directory.

  If a symbolic link is broken or is involved in a loop, an error is yielded

  as the first entry of the traversal.

  

  When enabled, the yielded [`DirEntry`](dent/index.md) values represent the target of

  the link while the path corresponds to the link. See the [`DirEntry`](dent/index.md)

  type for more details, and all future entries will be contained within

  the resolved directory behind the symbolic link of the root path.

- <span id="walkdir-max-open"></span>`fn max_open(self, n: usize) -> Self`

  Set the maximum number of simultaneously open file descriptors used

  by the iterator.

  

  `n` must be greater than or equal to `1`. If `n` is `0`, then it is set

  to `1` automatically. If this is not set, then it defaults to some

  reasonably low number.

  

  This setting has no impact on the results yielded by the iterator

  (even when `n` is `1`). Instead, this setting represents a trade off

  between scarce resources (file descriptors) and memory. Namely, when

  the maximum number of file descriptors is reached and a new directory

  needs to be opened to continue iteration, then a previous directory

  handle is closed and has its unyielded entries stored in memory. In

  practice, this is a satisfying trade off because it scales with respect

  to the *depth* of your file tree. Therefore, low values (even `1`) are

  acceptable.

  

  Note that this value does not impact the number of system calls made by

  an exhausted iterator.

  

  # Platform behavior

  

  On Windows, if `follow_links` is enabled, then this limit is not

  respected. In particular, the maximum number of file descriptors opened

  is proportional to the depth of the directory tree traversed.

- <span id="walkdir-sort-by"></span>`fn sort_by<F>(self, cmp: F) -> Self`

  Set a function for sorting directory entries with a comparator

  function.

  

  If a compare function is set, the resulting iterator will return all

  paths in sorted order. The compare function will be called to compare

  entries from the same directory.

  

  ```rust,no_run

  use std::cmp;

  use std::ffi::OsString;

  use walkdir::WalkDir;

  

  WalkDir::new("foo").sort_by(|a,b| a.file_name().cmp(b.file_name()));

  ```

- <span id="walkdir-sort-by-key"></span>`fn sort_by_key<K, F>(self, cmp: F) -> Self`

  Set a function for sorting directory entries with a key extraction

  function.

  

  If a compare function is set, the resulting iterator will return all

  paths in sorted order. The compare function will be called to compare

  entries from the same directory.

  

  ```rust,no_run

  use std::cmp;

  use std::ffi::OsString;

  use walkdir::WalkDir;

  

  WalkDir::new("foo").sort_by_key(|a| a.file_name().to_owned());

  ```

- <span id="walkdir-sort-by-file-name"></span>`fn sort_by_file_name(self) -> Self`

  Sort directory entries by file name, to ensure a deterministic order.

  

  This is a convenience function for calling `Self::sort_by()`.

  

  ```rust,no_run

  use walkdir::WalkDir;

  

  WalkDir::new("foo").sort_by_file_name();

  ```

- <span id="walkdir-contents-first"></span>`fn contents_first(self, yes: bool) -> Self`

  Yield a directory's contents before the directory itself. By default,

  this is disabled.

  

  When `yes` is `false` (as is the default), the directory is yielded

  before its contents are read. This is useful when, e.g. you want to

  skip processing of some directories.

  

  When `yes` is `true`, the iterator yields the contents of a directory

  before yielding the directory itself. This is useful when, e.g. you

  want to recursively delete a directory.

  

  # Example

  

  Assume the following directory tree:

  

  ```text

  foo/

    abc/

      qrs

      tuv

    def/

  ```

  

  With contents_first disabled (the default), the following code visits

  the directory tree in depth-first order:

  

  ```no_run

  use walkdir::WalkDir;

  

  for entry in WalkDir::new("foo") {

      let entry = entry.unwrap();

      println!("{}", entry.path().display());

  }

  

  // foo

  // foo/abc

  // foo/abc/qrs

  // foo/abc/tuv

  // foo/def

  ```

  

  With contents_first enabled:

  

  ```no_run

  use walkdir::WalkDir;

  

  for entry in WalkDir::new("foo").contents_first(true) {

      let entry = entry.unwrap();

      println!("{}", entry.path().display());

  }

  

  // foo/abc/qrs

  // foo/abc/tuv

  // foo/abc

  // foo/def

  // foo

  ```

- <span id="walkdir-same-file-system"></span>`fn same_file_system(self, yes: bool) -> Self`

  Do not cross file system boundaries.

  

  When this option is enabled, directory traversal will not descend into

  directories that are on a different file system from the root path.

  

  Currently, this option is only supported on Unix and Windows. If this

  option is used on an unsupported platform, then directory traversal

  will immediately return an error and will not yield any entries.

#### Trait Implementations

##### `impl Any for WalkDir`

- <span id="walkdir-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkDir`

- <span id="walkdir-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkDir`

- <span id="walkdir-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for WalkDir`

- <span id="walkdir-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WalkDir`

- <span id="walkdir-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkDir`

- <span id="walkdir-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for WalkDir`

- <span id="walkdir-intoiterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="walkdir-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter`

- <span id="walkdir-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](#intoiter)

##### `impl<U> TryFrom for WalkDir`

- <span id="walkdir-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walkdir-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkDir`

- <span id="walkdir-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walkdir-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WalkDirOptions`

```rust
struct WalkDirOptions {
    follow_links: bool,
    follow_root_links: bool,
    max_open: usize,
    min_depth: usize,
    max_depth: usize,
    sorter: Option<Box<dyn FnMut(&DirEntry, &DirEntry) -> std::cmp::Ordering + Send + Sync>>,
    contents_first: bool,
    same_file_system: bool,
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:239-255`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L239-L255)*

#### Trait Implementations

##### `impl Any for WalkDirOptions`

- <span id="walkdiroptions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WalkDirOptions`

- <span id="walkdiroptions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WalkDirOptions`

- <span id="walkdiroptions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for WalkDirOptions`

- <span id="walkdiroptions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error>`

##### `impl<T> From for WalkDirOptions`

- <span id="walkdiroptions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WalkDirOptions`

- <span id="walkdiroptions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for WalkDirOptions`

- <span id="walkdiroptions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="walkdiroptions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WalkDirOptions`

- <span id="walkdiroptions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="walkdiroptions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter`

```rust
struct IntoIter {
    opts: WalkDirOptions,
    start: Option<std::path::PathBuf>,
    stack_list: Vec<DirList>,
    stack_path: Vec<Ancestor>,
    oldest_opened: usize,
    depth: usize,
    deferred_dirs: Vec<DirEntry>,
    root_device: Option<u64>,
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:566-606`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L566-L606)*

An iterator for recursively descending into a directory.

A value with this type must be constructed with the [`WalkDir`](#walkdir) type, which
uses a builder pattern to set options such as min/max depth, max open file
descriptors and whether the iterator should follow symbolic links. After
constructing a `WalkDir`, call `.into_iter()` at the end of the chain.

The order of elements yielded by this iterator is unspecified.



#### Fields

- **`opts`**: `WalkDirOptions`

  Options specified in the builder. Depths, max fds, etc.

- **`start`**: `Option<std::path::PathBuf>`

  The start path.
  
  This is only `Some(...)` at the beginning. After the first iteration,
  this is always `None`.

- **`stack_list`**: `Vec<DirList>`

  A stack of open (up to max fd) or closed handles to directories.
  An open handle is a plain `fs::ReadDir` while a closed handle is
  a `Vec<fs::DirEntry>` corresponding to the as-of-yet consumed entries.
  

- **`stack_path`**: `Vec<Ancestor>`

  A stack of file paths.
  
  This is *only* used when `follow_links` is enabled. In all other
  cases this stack is empty.
  

- **`oldest_opened`**: `usize`

  An index into `stack_list` that points to the oldest open directory
  handle. If the maximum fd limit is reached and a new directory needs to
  be read, the handle at this index is closed before the new directory is
  opened.

- **`depth`**: `usize`

  The current depth of iteration (the length of the stack at the
  beginning of each iteration).

- **`deferred_dirs`**: `Vec<DirEntry>`

  A list of DirEntries corresponding to directories, that are
  yielded after their contents has been fully yielded. This is only
  used when `contents_first` is enabled.

- **`root_device`**: `Option<u64>`

  The device of the root file path when the first call to `next` was
  made.
  
  If the `same_file_system` option isn't enabled, then this is always
  `None`. Conversely, if it is enabled, this is always `Some(...)` after
  handling the root path.

#### Implementations

- <span id="intoiter-skip-current-dir"></span>`fn skip_current_dir(&mut self)`

  Skips the current directory.

  

  This causes the iterator to stop traversing the contents of the least

  recently yielded directory. This means any remaining entries in that

  directory will be skipped (including sub-directories).

  

  Note that the ergonomics of this method are questionable since it

  borrows the iterator mutably. Namely, you must write out the looping

  condition manually. For example, to skip hidden entries efficiently on

  unix systems:

  

  ```no_run

  use walkdir::{DirEntry, WalkDir};

  

  fn is_hidden(entry: &DirEntry) -> bool {

      entry.file_name()

           .to_str()

           .map(|s| s.starts_with("."))

           .unwrap_or(false)

  }

  

  let mut it = WalkDir::new("foo").into_iter();

  loop {

      let entry = match it.next() {

          None => break,

          Some(Err(err)) => panic!("ERROR: {}", err),

          Some(Ok(entry)) => entry,

      };

      if is_hidden(&entry) {

          if entry.file_type().is_dir() {

              it.skip_current_dir();

          }

          continue;

      }

      println!("{}", entry.path().display());

  }

  ```

  

  You may find it more convenient to use the `filter_entry` iterator

  adapter. (See its documentation for the same example functionality as

  above.)

- <span id="intoiter-filter-entry"></span>`fn filter_entry<P>(self, predicate: P) -> FilterEntry<Self, P>` — [`FilterEntry`](#filterentry)

  Yields only entries which satisfy the given predicate and skips

  descending into directories that do not satisfy the given predicate.

  

  The predicate is applied to all entries. If the predicate is

  true, iteration carries on as normal. If the predicate is false, the

  entry is ignored and if it is a directory, it is not descended into.

  

  This is often more convenient to use than `skip_current_dir`. For

  example, to skip hidden files and directories efficiently on unix

  systems:

  

  ```no_run

  use walkdir::{DirEntry, WalkDir};

  use walkdir::Error;

  

  fn is_hidden(entry: &DirEntry) -> bool {

      entry.file_name()

           .to_str()

           .map(|s| s.starts_with("."))

           .unwrap_or(false)

  }

  

  fn try_main() -> Result<(), Error> {

  for entry in WalkDir::new("foo")

                       .into_iter()

                       .filter_entry(|e| !is_hidden(e)) {

      println!("{}", entry?.path().display());

  }

  Ok(())

  }

  ```

  

  Note that the iterator will still yield errors for reading entries that

  may not satisfy the predicate.

  

  Note that entries skipped with `min_depth` and `max_depth` are not

  passed to this predicate.

  

  Note that if the iterator has `contents_first` enabled, then this

  method is no different than calling the standard `Iterator::filter`

  method (because directory entries are yielded after they've been

  descended into).

  

  

- <span id="intoiter-handle-entry"></span>`fn handle_entry(&mut self, dent: DirEntry) -> Option<Result<DirEntry>>` — [`DirEntry`](dent/index.md#direntry), [`Result`](#result)

- <span id="intoiter-get-deferred-dir"></span>`fn get_deferred_dir(&mut self) -> Option<DirEntry>` — [`DirEntry`](dent/index.md#direntry)

- <span id="intoiter-push"></span>`fn push(&mut self, dent: &DirEntry) -> Result<()>` — [`DirEntry`](dent/index.md#direntry), [`Result`](#result)

- <span id="intoiter-pop"></span>`fn pop(&mut self)`

- <span id="intoiter-follow"></span>`fn follow(&self, dent: DirEntry) -> Result<DirEntry>` — [`DirEntry`](dent/index.md#direntry), [`Result`](#result)

- <span id="intoiter-check-loop"></span>`fn check_loop<P: AsRef<Path>>(&self, child: P) -> Result<()>` — [`Result`](#result)

- <span id="intoiter-is-same-file-system"></span>`fn is_same_file_system(&mut self, dent: &DirEntry) -> Result<bool>` — [`DirEntry`](dent/index.md#direntry), [`Result`](#result)

- <span id="intoiter-skippable"></span>`fn skippable(&self) -> bool`

#### Trait Implementations

##### `impl Any for IntoIter`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IntoIter`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for IntoIter`

##### `impl<U> Into for IntoIter`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-iterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](dent/index.md#direntry)

  Advances the iterator and returns the next value.

  

  # Errors

  

  If the iterator fails to retrieve the next value, this method returns

  an error value. The error will be wrapped in an Option::Some.

##### `impl<U> TryFrom for IntoIter`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ancestor`

```rust
struct Ancestor {
    path: std::path::PathBuf,
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:611-620`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L611-L620)*

An ancestor is an item in the directory tree traversed by walkdir, and is
used to check for loops in the tree when traversing symlinks.

#### Fields

- **`path`**: `std::path::PathBuf`

  The path of this ancestor.

#### Implementations

- <span id="ancestor-new"></span>`fn new(dent: &DirEntry) -> io::Result<Ancestor>` — [`DirEntry`](dent/index.md#direntry), [`Ancestor`](#ancestor)

  Create a new ancestor from the given directory path.

- <span id="ancestor-is-same"></span>`fn is_same(&self, child: &Handle) -> io::Result<bool>`

  Returns true if and only if the given open file handle corresponds to

  the same directory as this ancestor.

#### Trait Implementations

##### `impl Any for Ancestor`

- <span id="ancestor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ancestor`

- <span id="ancestor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ancestor`

- <span id="ancestor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Ancestor`

- <span id="ancestor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Ancestor`

- <span id="ancestor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ancestor`

- <span id="ancestor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Ancestor`

- <span id="ancestor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ancestor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ancestor`

- <span id="ancestor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ancestor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterEntry<I, P>`

```rust
struct FilterEntry<I, P> {
    it: I,
    predicate: P,
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:1055-1058`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L1055-L1058)*

A recursive directory iterator that skips entries.

Values of this type are created by calling `.filter_entry()` on an
`IntoIter`, which is formed by calling `.into_iter()` on a `WalkDir`.

Directories that fail the predicate `P` are skipped. Namely, they are
never yielded and never descended into.

Entries that are skipped with the `min_depth` and `max_depth` options
are not passed through this filter.

If opening a handle to a directory resulted in an error, then it is yielded
and no corresponding call to the predicate is made.

Type parameter `I` refers to the underlying iterator and `P` refers to the
predicate, which is usually `FnMut(&DirEntry) -> bool`.





#### Implementations

- <span id="filterentry-filter-entry"></span>`fn filter_entry(self, predicate: P) -> FilterEntry<Self, P>` — [`FilterEntry`](#filterentry)

  Yields only entries which satisfy the given predicate and skips

  descending into directories that do not satisfy the given predicate.

  

  The predicate is applied to all entries. If the predicate is

  true, iteration carries on as normal. If the predicate is false, the

  entry is ignored and if it is a directory, it is not descended into.

  

  This is often more convenient to use than `skip_current_dir`. For

  example, to skip hidden files and directories efficiently on unix

  systems:

  

  ```no_run

  use walkdir::{DirEntry, WalkDir};

  use walkdir::Error;

  

  fn is_hidden(entry: &DirEntry) -> bool {

      entry.file_name()

           .to_str()

           .map(|s| s.starts_with("."))

           .unwrap_or(false)

  }

  

  fn try_main() -> Result<(), Error> {

  for entry in WalkDir::new("foo")

                       .into_iter()

                       .filter_entry(|e| !is_hidden(e)) {

      println!("{}", entry?.path().display());

  }

  Ok(())

  }

  ```

  

  Note that the iterator will still yield errors for reading entries that

  may not satisfy the predicate.

  

  Note that entries skipped with `min_depth` and `max_depth` are not

  passed to this predicate.

  

  Note that if the iterator has `contents_first` enabled, then this

  method is no different than calling the standard `Iterator::filter`

  method (because directory entries are yielded after they've been

  descended into).

  

  

- <span id="filterentry-skip-current-dir"></span>`fn skip_current_dir(&mut self)`

  Skips the current directory.

  

  This causes the iterator to stop traversing the contents of the least

  recently yielded directory. This means any remaining entries in that

  directory will be skipped (including sub-directories).

  

  Note that the ergonomics of this method are questionable since it

  borrows the iterator mutably. Namely, you must write out the looping

  condition manually. For example, to skip hidden entries efficiently on

  unix systems:

  

  ```no_run

  use walkdir::{DirEntry, WalkDir};

  

  fn is_hidden(entry: &DirEntry) -> bool {

      entry.file_name()

           .to_str()

           .map(|s| s.starts_with("."))

           .unwrap_or(false)

  }

  

  let mut it = WalkDir::new("foo").into_iter();

  loop {

      let entry = match it.next() {

          None => break,

          Some(Err(err)) => panic!("ERROR: {}", err),

          Some(Ok(entry)) => entry,

      };

      if is_hidden(&entry) {

          if entry.file_type().is_dir() {

              it.skip_current_dir();

          }

          continue;

      }

      println!("{}", entry.path().display());

  }

  ```

  

  You may find it more convenient to use the `filter_entry` iterator

  adapter. (See its documentation for the same example functionality as

  above.)

#### Trait Implementations

##### `impl Any for FilterEntry<I, P>`

- <span id="filterentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterEntry<I, P>`

- <span id="filterentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterEntry<I, P>`

- <span id="filterentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug, P: fmt::Debug> Debug for FilterEntry<I, P>`

- <span id="filterentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FilterEntry<I, P>`

- <span id="filterentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<P> FusedIterator for FilterEntry<IntoIter, P>`

##### `impl<U> Into for FilterEntry<I, P>`

- <span id="filterentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for FilterEntry<I, P>`

- <span id="filterentry-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="filterentry-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="filterentry-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<P> Iterator for FilterEntry<IntoIter, P>`

- <span id="filterentry-iterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="filterentry-iterator-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](dent/index.md#direntry)

  Advances the iterator and returns the next value.

  

  # Errors

  

  If the iterator fails to retrieve the next value, this method returns

  an error value. The error will be wrapped in an `Option::Some`.

##### `impl<U> TryFrom for FilterEntry<I, P>`

- <span id="filterentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filterentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterEntry<I, P>`

- <span id="filterentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filterentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `DirList`

```rust
enum DirList {
    Opened {
        depth: usize,
        it: result::Result<std::fs::ReadDir, Option<Error>>,
    },
    Closed(vec::IntoIter<Result<DirEntry>>),
}
```

*Defined in [`walkdir-2.5.0/src/lib.rs:661-677`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L661-L677)*

A sequence of unconsumed directory entries.

This represents the opened or closed state of a directory handle. When
open, future entries are read by iterating over the raw `fs::ReadDir`.
When closed, all future entries are read into memory. Iteration then
proceeds over a `Vec<fs::DirEntry>`.



#### Variants

- **`Opened`**

  An opened handle.
  
  This includes the depth of the handle itself.
  
  If there was an error with the initial `fs::read_dir` call, then it
  is stored here. (We use an `Option<...>` to make yielding the error
  exactly once simpler.)
  
  

- **`Closed`**

  A closed handle.
  
  All remaining directory entries are read into memory.

#### Implementations

- <span id="dirlist-close"></span>`fn close(&mut self)`

#### Trait Implementations

##### `impl Any for DirList`

- <span id="dirlist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DirList`

- <span id="dirlist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DirList`

- <span id="dirlist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DirList`

- <span id="dirlist-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DirList`

- <span id="dirlist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DirList`

- <span id="dirlist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DirList`

- <span id="dirlist-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dirlist-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="dirlist-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DirList`

- <span id="dirlist-iterator-type-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="dirlist-iterator-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](dent/index.md#direntry)

##### `impl<U> TryFrom for DirList`

- <span id="dirlist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dirlist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DirList`

- <span id="dirlist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dirlist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `DirEntryExt`

```rust
trait DirEntryExt { ... }
```

*Defined in [`walkdir-2.5.0/src/dent.rs:339-343`](../../.source_1765633015/walkdir-2.5.0/src/dent.rs#L339-L343)*

Unix-specific extension methods for `walkdir::DirEntry`

#### Required Methods

- `fn ino(&self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

#### Implementors

- [`DirEntry`](dent/index.md#direntry)

## Type Aliases

### `Result<T>`

```rust
type Result<T> = ::std::result::Result<T, Error>;
```

*Defined in [`walkdir-2.5.0/src/lib.rs:157`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L157)*

A result type for walkdir operations.

Note that this result type embeds the error type in this crate. This
is only useful if you care about the additional information provided by
the error (such as the path associated with the error or whether a loop
was dectected). If you want things to Just Work, then you can use
`io::Result` instead since the error type in this package will
automatically convert to an `io::Result` when using the `try!` macro.



## Macros

### `itry!`

*Defined in [`walkdir-2.5.0/src/lib.rs:137-144`](../../.source_1765633015/walkdir-2.5.0/src/lib.rs#L137-L144)*

Like try, but for iterators that return `Option<Result<_, _>>`.


