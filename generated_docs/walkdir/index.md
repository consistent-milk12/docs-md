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

The [`WalkDir`](#walkdir) type builds iterators. The [`DirEntry`](#direntry) type describes values
yielded by the iterator. Finally, the [`Error`](#error) type is a small wrapper around
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
| [`Ancestor`](#ancestor) | struct | An ancestor is an item in the directory tree traversed by walkdir, and is |
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

- <span id="direntry-metadata"></span>`fn metadata(&self) -> Result<fs::Metadata>` — [`Result`](#result)

- <span id="direntry-metadata-internal"></span>`fn metadata_internal(&self) -> Result<fs::Metadata>` — [`Result`](#result)

- <span id="direntry-file-type"></span>`fn file_type(&self) -> fs::FileType`

- <span id="direntry-file-name"></span>`fn file_name(&self) -> &OsStr`

- <span id="direntry-depth"></span>`fn depth(&self) -> usize`

- <span id="direntry-is-dir"></span>`fn is_dir(&self) -> bool`

- <span id="direntry-from-entry"></span>`fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](#result)

- <span id="direntry-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](#result), [`DirEntry`](#direntry)

#### Trait Implementations

##### `impl Clone for DirEntry`

- <span id="direntry-clone"></span>`fn clone(&self) -> DirEntry` — [`DirEntry`](#direntry)

##### `impl Debug for DirEntry`

- <span id="direntry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- <span id="direntry-ino"></span>`fn ino(&self) -> u64`

### `Error`

```rust
struct Error {
    depth: usize,
    inner: ErrorInner,
}
```

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

- <span id="error-loop-ancestor"></span>`fn loop_ancestor(&self) -> Option<&Path>`

- <span id="error-depth"></span>`fn depth(&self) -> usize`

- <span id="error-io-error"></span>`fn io_error(&self) -> Option<&io::Error>`

- <span id="error-into-io-error"></span>`fn into_io_error(self) -> Option<io::Error>`

- <span id="error-from-path"></span>`fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- <span id="error-from-entry"></span>`fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](#direntry)

- <span id="error-from-io"></span>`fn from_io(depth: usize, err: io::Error) -> Self`

- <span id="error-from-loop"></span>`fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- <span id="error-description"></span>`fn description(&self) -> &str`

- <span id="error-cause"></span>`fn cause(&self) -> Option<&dyn error::Error>`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `WalkDir`

```rust
struct WalkDir {
    opts: WalkDirOptions,
    root: std::path::PathBuf,
}
```

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

- <span id="walkdir-min-depth"></span>`fn min_depth(self, depth: usize) -> Self`

- <span id="walkdir-max-depth"></span>`fn max_depth(self, depth: usize) -> Self`

- <span id="walkdir-follow-links"></span>`fn follow_links(self, yes: bool) -> Self`

- <span id="walkdir-follow-root-links"></span>`fn follow_root_links(self, yes: bool) -> Self`

- <span id="walkdir-max-open"></span>`fn max_open(self, n: usize) -> Self`

- <span id="walkdir-sort-by"></span>`fn sort_by<F>(self, cmp: F) -> Self`

- <span id="walkdir-sort-by-key"></span>`fn sort_by_key<K, F>(self, cmp: F) -> Self`

- <span id="walkdir-sort-by-file-name"></span>`fn sort_by_file_name(self) -> Self`

- <span id="walkdir-contents-first"></span>`fn contents_first(self, yes: bool) -> Self`

- <span id="walkdir-same-file-system"></span>`fn same_file_system(self, yes: bool) -> Self`

#### Trait Implementations

##### `impl Debug for WalkDir`

- <span id="walkdir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for WalkDir`

- <span id="walkdir-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="walkdir-intoiter"></span>`type IntoIter = IntoIter`

- <span id="walkdir-into-iter"></span>`fn into_iter(self) -> IntoIter` — [`IntoIter`](#intoiter)

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

#### Trait Implementations

##### `impl Debug for WalkDirOptions`

- <span id="walkdiroptions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error>`

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

- <span id="intoiter-filter-entry"></span>`fn filter_entry<P>(self, predicate: P) -> FilterEntry<Self, P>` — [`FilterEntry`](#filterentry)

- <span id="intoiter-handle-entry"></span>`fn handle_entry(&mut self, dent: DirEntry) -> Option<Result<DirEntry>>` — [`DirEntry`](#direntry), [`Result`](#result)

- <span id="intoiter-get-deferred-dir"></span>`fn get_deferred_dir(&mut self) -> Option<DirEntry>` — [`DirEntry`](#direntry)

- <span id="intoiter-push"></span>`fn push(&mut self, dent: &DirEntry) -> Result<()>` — [`DirEntry`](#direntry), [`Result`](#result)

- <span id="intoiter-pop"></span>`fn pop(&mut self)`

- <span id="intoiter-follow"></span>`fn follow(&self, dent: DirEntry) -> Result<DirEntry>` — [`DirEntry`](#direntry), [`Result`](#result)

- <span id="intoiter-check-loop"></span>`fn check_loop<P: AsRef<Path>>(&self, child: P) -> Result<()>` — [`Result`](#result)

- <span id="intoiter-is-same-file-system"></span>`fn is_same_file_system(&mut self, dent: &DirEntry) -> Result<bool>` — [`DirEntry`](#direntry), [`Result`](#result)

- <span id="intoiter-skippable"></span>`fn skippable(&self) -> bool`

#### Trait Implementations

##### `impl Debug for IntoIter`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for IntoIter`

##### `impl<I> IntoIterator for IntoIter`

- <span id="intoiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IntoIter`

- <span id="intoiter-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](#direntry)

### `Ancestor`

```rust
struct Ancestor {
    path: std::path::PathBuf,
}
```

An ancestor is an item in the directory tree traversed by walkdir, and is
used to check for loops in the tree when traversing symlinks.

#### Fields

- **`path`**: `std::path::PathBuf`

  The path of this ancestor.

#### Implementations

- <span id="ancestor-new"></span>`fn new(dent: &DirEntry) -> io::Result<Ancestor>` — [`DirEntry`](#direntry), [`Ancestor`](#ancestor)

- <span id="ancestor-is-same"></span>`fn is_same(&self, child: &Handle) -> io::Result<bool>`

#### Trait Implementations

##### `impl Debug for Ancestor`

- <span id="ancestor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FilterEntry<I, P>`

```rust
struct FilterEntry<I, P> {
    it: I,
    predicate: P,
}
```

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

- <span id="filterentry-skip-current-dir"></span>`fn skip_current_dir(&mut self)`

#### Trait Implementations

##### `impl<I: fmt::Debug, P: fmt::Debug> Debug for FilterEntry<I, P>`

- <span id="filterentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<P> FusedIterator for FilterEntry<IntoIter, P>`

##### `impl<I> IntoIterator for FilterEntry<I, P>`

- <span id="filterentry-item"></span>`type Item = <I as Iterator>::Item`

- <span id="filterentry-intoiter"></span>`type IntoIter = I`

- <span id="filterentry-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<P> Iterator for FilterEntry<IntoIter, P>`

- <span id="filterentry-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="filterentry-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](#direntry)

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

##### `impl Debug for DirList`

- <span id="dirlist-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for DirList`

- <span id="dirlist-item"></span>`type Item = <I as Iterator>::Item`

- <span id="dirlist-intoiter"></span>`type IntoIter = I`

- <span id="dirlist-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DirList`

- <span id="dirlist-item"></span>`type Item = Result<DirEntry, Error>`

- <span id="dirlist-next"></span>`fn next(&mut self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](#direntry)

## Traits

### `DirEntryExt`

```rust
trait DirEntryExt { ... }
```

Unix-specific extension methods for `walkdir::DirEntry`

#### Required Methods

- `fn ino(&self) -> u64`

  Returns the underlying `d_ino` field in the contained `dirent`

#### Implementors

- [`DirEntry`](#direntry)

## Type Aliases

### `Result<T>`

```rust
type Result<T> = ::std::result::Result<T, Error>;
```

A result type for walkdir operations.

Note that this result type embeds the error type in this crate. This
is only useful if you care about the additional information provided by
the error (such as the path associated with the error or whether a loop
was dectected). If you want things to Just Work, then you can use
`io::Result` instead since the error type in this package will
automatically convert to an `io::Result` when using the `try!` macro.



## Macros

### `itry!`

Like try, but for iterators that return `Option<Result<_, _>>`.


