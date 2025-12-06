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
may arise, use [`filter_map`](#filter-map). (e.g., This code below will silently skip
directories that the owner of the running process does not have permission to
access.)

```no_run
use walkdir::WalkDir;

for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
    println!("{}", entry.path().display());
}
```

# Example: follow symbolic links

The same code as above, except [`follow_links`](#follow-links) is enabled:

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

This uses the [`filter_entry`](#filter-entry) iterator adapter to avoid yielding hidden files
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

On Unix systems, this type implements the [`DirEntryExt`](dent/index.md) trait, which
provides efficient access to the inode number of the directory entry.

# Differences with `std::fs::DirEntry`

This type mostly mirrors the type by the same name in `std::fs`. There
are some differences however:

* All recursive directory iterators must inspect the entry's type.
Therefore, the value is stored and its access is guaranteed to be cheap and
successful.
* [`path`](#path) and [`file_name`](#file-name) return borrowed variants.
* If [`follow_links`](#follow-links) was enabled on the originating iterator, then all
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

- `fn metadata(self: &Self) -> Result<fs::Metadata>` — [`Result`](#result)

- `fn metadata_internal(self: &Self) -> Result<fs::Metadata>` — [`Result`](#result)

- `fn file_type(self: &Self) -> fs::FileType`

- `fn file_name(self: &Self) -> &OsStr`

- `fn depth(self: &Self) -> usize`

- `fn is_dir(self: &Self) -> bool`

- `fn from_entry(depth: usize, ent: &fs::DirEntry) -> Result<DirEntry>` — [`Result`](#result)

- `fn from_path(depth: usize, pb: PathBuf, follow: bool) -> Result<DirEntry>` — [`Result`](#result), [`DirEntry`](dent/index.md)

#### Trait Implementations

##### `impl Clone for DirEntry`

- `fn clone(self: &Self) -> DirEntry` — [`DirEntry`](dent/index.md)

##### `impl Debug for DirEntry`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DirEntryExt for DirEntry`

- `fn ino(self: &Self) -> u64`

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
[`impl From<Error> for std::io::Error`][impl](#impl) defined which preserves the original context.
This allows you to use an `io::Result` with methods in this crate if you don't care about
accessing the underlying error data in a structured form.




#### Implementations

- `fn path(self: &Self) -> Option<&Path>`

- `fn loop_ancestor(self: &Self) -> Option<&Path>`

- `fn depth(self: &Self) -> usize`

- `fn io_error(self: &Self) -> Option<&io::Error>`

- `fn into_io_error(self: Self) -> Option<io::Error>`

- `fn from_path(depth: usize, pb: PathBuf, err: io::Error) -> Self`

- `fn from_entry(dent: &DirEntry, err: io::Error) -> Self` — [`DirEntry`](dent/index.md)

- `fn from_io(depth: usize, err: io::Error) -> Self`

- `fn from_loop(depth: usize, ancestor: &Path, child: &Path) -> Self`

#### Trait Implementations

##### `impl Debug for Error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Error`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for Error`

- `fn description(self: &Self) -> &str`

- `fn cause(self: &Self) -> Option<&dyn error::Error>`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error`

- `fn to_string(self: &Self) -> String`

### `WalkDir`

```rust
struct WalkDir {
    opts: WalkDirOptions,
    root: std::path::PathBuf,
}
```

A builder to create an iterator for recursively walking a directory.

Results are returned in depth first fashion, with directories yielded
before their contents. If [`contents_first`](#contents-first) is true, contents are yielded
before their directories. The order is unspecified but if [`sort_by`](#sort-by) is
given, directory entries are sorted according to this function. Directory
entries `.` and `..` are always omitted.

If an error occurs at any point during iteration, then it is returned in
place of its corresponding directory entry and iteration continues as
normal. If an error occurs while opening a directory for reading, then it
is not descended into (but the error is still yielded by the iterator).
Iteration may be stopped at any time. When the iterator is destroyed, all
resources associated with it are freed.


# Usage

This type implements [`IntoIterator`](#intoiterator) so that it may be used as the subject
of a `for` loop. You may need to call [`into_iter`](#into-iter) explicitly if you want
to use iterator adapters such as [`filter_entry`](#filter-entry).

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
with the [`min_depth`](#min-depth) setting:

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

- `fn new<P: AsRef<Path>>(root: P) -> Self`

- `fn min_depth(self: Self, depth: usize) -> Self`

- `fn max_depth(self: Self, depth: usize) -> Self`

- `fn follow_links(self: Self, yes: bool) -> Self`

- `fn follow_root_links(self: Self, yes: bool) -> Self`

- `fn max_open(self: Self, n: usize) -> Self`

- `fn sort_by<F>(self: Self, cmp: F) -> Self`

- `fn sort_by_key<K, F>(self: Self, cmp: F) -> Self`

- `fn sort_by_file_name(self: Self) -> Self`

- `fn contents_first(self: Self, yes: bool) -> Self`

- `fn same_file_system(self: Self, yes: bool) -> Self`

#### Trait Implementations

##### `impl Debug for WalkDir`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator for WalkDir`

- `type Item = Result<DirEntry, Error>`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter` — [`IntoIter`](#intoiter)

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
constructing a `WalkDir`, call [`.into_iter()`](#into-iter) at the end of the chain.

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
  
  This is *only* used when [`follow_links`](#follow-links) is enabled. In all other
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

- `fn skip_current_dir(self: &mut Self)`

- `fn filter_entry<P>(self: Self, predicate: P) -> FilterEntry<Self, P>` — [`FilterEntry`](#filterentry)

- `fn handle_entry(self: &mut Self, dent: DirEntry) -> Option<Result<DirEntry>>` — [`DirEntry`](dent/index.md), [`Result`](#result)

- `fn get_deferred_dir(self: &mut Self) -> Option<DirEntry>` — [`DirEntry`](dent/index.md)

- `fn push(self: &mut Self, dent: &DirEntry) -> Result<()>` — [`DirEntry`](dent/index.md), [`Result`](#result)

- `fn pop(self: &mut Self)`

- `fn follow(self: &Self, dent: DirEntry) -> Result<DirEntry>` — [`DirEntry`](dent/index.md), [`Result`](#result)

- `fn check_loop<P: AsRef<Path>>(self: &Self, child: P) -> Result<()>` — [`Result`](#result)

- `fn is_same_file_system(self: &mut Self, dent: &DirEntry) -> Result<bool>` — [`DirEntry`](dent/index.md), [`Result`](#result)

- `fn skippable(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for IntoIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FusedIterator for IntoIter`

##### `impl<I> IntoIterator for IntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IntoIter`

- `type Item = Result<DirEntry, Error>`

- `fn next(self: &mut Self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](dent/index.md)

### `FilterEntry<I, P>`

```rust
struct FilterEntry<I, P> {
    it: I,
    predicate: P,
}
```

A recursive directory iterator that skips entries.

Values of this type are created by calling [`.filter_entry()`](#filter-entry) on an
`IntoIter`, which is formed by calling [`.into_iter()`](#into-iter) on a `WalkDir`.

Directories that fail the predicate `P` are skipped. Namely, they are
never yielded and never descended into.

Entries that are skipped with the [`min_depth`](#min-depth) and [`max_depth`](#max-depth) options
are not passed through this filter.

If opening a handle to a directory resulted in an error, then it is yielded
and no corresponding call to the predicate is made.

Type parameter `I` refers to the underlying iterator and `P` refers to the
predicate, which is usually `FnMut(&DirEntry) -> bool`.





#### Implementations

- `fn filter_entry(self: Self, predicate: P) -> FilterEntry<Self, P>` — [`FilterEntry`](#filterentry)

- `fn skip_current_dir(self: &mut Self)`

#### Trait Implementations

##### `impl<I: $crate::fmt::Debug, P: $crate::fmt::Debug> Debug for FilterEntry<I, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<P> FusedIterator for FilterEntry<IntoIter, P>`

##### `impl<I> IntoIterator for FilterEntry<I, P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<P> Iterator for FilterEntry<IntoIter, P>`

- `type Item = Result<DirEntry, Error>`

- `fn next(self: &mut Self) -> Option<Result<DirEntry>>` — [`Result`](#result), [`DirEntry`](dent/index.md)

## Traits

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
automatically convert to an `io::Result` when using the [`try!`](#try) macro.



