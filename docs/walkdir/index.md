# Crate `walkdir`

Crate `walkdir` provides an efficient and cross platform implementation
of recursive directory traversal. Several options are exposed to control
iteration, such as whether to follow symbolic links (default off), limit the
maximum number of simultaneous open file descriptors and the ability to
efficiently skip descending into directories.

To use this crate, add `walkdir` as a dependency to your project's
`Cargo.toml`:

```toml
[dependencies](#dependencies)

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
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo") {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
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
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").follow_links(true) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
```

# Example: skip hidden files and directories on unix

This uses the [`filter_entry`](#filter-entry) iterator adapter to avoid yielding hidden files
and directories efficiently (i.e. without recursing into hidden directories):

```no_run
use walkdir::{DirEntry, WalkDir};
# use walkdir::Error;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

# fn try_main() -> Result<(), Error> {
let walker = WalkDir::new("foo").into_iter();
for entry in walker.filter_entry(|e| !is_hidden(e)) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
```


## Structs

### `DirEntry`

```rust
struct DirEntry {
    // [REDACTED: Private Fields]
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
* [`path`](#path) and [`file_name`](#file-name) return borrowed variants.
* If [`follow_links`](#follow-links) was enabled on the originating iterator, then all
operations except for [`path`](#path) operate on the link target. Otherwise, all
operations operate on the symbolic link.






#### Implementations

- `fn path(self: &Self) -> &Path`
  The full path that this entry represents.

- `fn into_path(self: Self) -> PathBuf`
  The full path that this entry represents.

- `fn path_is_symlink(self: &Self) -> bool`
  Returns `true` if and only if this entry was created from a symbolic

- `fn metadata(self: &Self) -> Result<fs::Metadata>`
  Return the metadata for the file that this entry points to.

- `fn file_type(self: &Self) -> fs::FileType`
  Return the file type for the file that this entry points to.

- `fn file_name(self: &Self) -> &OsStr`
  Return the file name of this entry.

- `fn depth(self: &Self) -> usize`
  Returns the depth at which this entry was created relative to the root.

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

- `fn clone(self: &Self) -> DirEntry`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DirEntryExt`

- `fn ino(self: &Self) -> u64`
  Returns the underlying `d_ino` field in the contained `dirent`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Error`

```rust
struct Error {
    // [REDACTED: Private Fields]
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
[`impl From<Error> for std::io::Error`][impl](#impl)
 defined which preserves the original context.
This allows you to use an `io::Result` with methods in this crate if you don't care about
accessing the underlying error data in a structured form.


[impl](#impl)
: struct.Error.html#impl-From%3CError%3E

#### Implementations

- `fn path(self: &Self) -> Option<&Path>`
  Returns the path associated with this error if one exists.

- `fn loop_ancestor(self: &Self) -> Option<&Path>`
  Returns the path at which a cycle was detected.

- `fn depth(self: &Self) -> usize`
  Returns the depth at which this error occurred relative to the root.

- `fn io_error(self: &Self) -> Option<&io::Error>`
  Inspect the original [`io::Error`] if there is one.

- `fn into_io_error(self: Self) -> Option<io::Error>`
  Similar to [`io_error`] except consumes self to convert to the original

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

- `fn description(self: &Self) -> &str`

- `fn cause(self: &Self) -> Option<&dyn error::Error>`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `WalkDir`

```rust
struct WalkDir {
    // [REDACTED: Private Fields]
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
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1).max_depth(3) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
```



Note that the iterator by default includes the top-most directory. Since
this is the only directory yielded with depth `0`, it is easy to ignore it
with the [`min_depth`](#min-depth) setting:

```no_run
use walkdir::WalkDir;
# use walkdir::Error;

# fn try_main() -> Result<(), Error> {
for entry in WalkDir::new("foo").min_depth(1) {
    println!("{}", entry?.path().display());
}
# Ok(())
# }
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
  Create a builder for a recursive directory iterator starting at the

- `fn min_depth(self: Self, depth: usize) -> Self`
  Set the minimum depth of entries yielded by the iterator.

- `fn max_depth(self: Self, depth: usize) -> Self`
  Set the maximum depth of entries yield by the iterator.

- `fn follow_links(self: Self, yes: bool) -> Self`
  Follow symbolic links. By default, this is disabled.

- `fn follow_root_links(self: Self, yes: bool) -> Self`
  Follow symbolic links if these are the root of the traversal.

- `fn max_open(self: Self, n: usize) -> Self`
  Set the maximum number of simultaneously open file descriptors used

- `fn sort_by<F>(self: Self, cmp: F) -> Self`
  Set a function for sorting directory entries with a comparator

- `fn sort_by_key<K, F>(self: Self, cmp: F) -> Self`
  Set a function for sorting directory entries with a key extraction

- `fn sort_by_file_name(self: Self) -> Self`
  Sort directory entries by file name, to ensure a deterministic order.

- `fn contents_first(self: Self, yes: bool) -> Self`
  Yield a directory's contents before the directory itself. By default,

- `fn same_file_system(self: Self, yes: bool) -> Self`
  Do not cross file system boundaries.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator`

- `type Item = Result<DirEntry, Error>`

- `type IntoIter = IntoIter`

- `fn into_iter(self: Self) -> IntoIter`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `IntoIter`

```rust
struct IntoIter {
    // [REDACTED: Private Fields]
}
```

An iterator for recursively descending into a directory.

A value with this type must be constructed with the [`WalkDir`](#walkdir) type, which
uses a builder pattern to set options such as min/max depth, max open file
descriptors and whether the iterator should follow symbolic links. After
constructing a `WalkDir`, call [`.into_iter()`](#into-iter) at the end of the chain.

The order of elements yielded by this iterator is unspecified.



#### Implementations

- `fn skip_current_dir(self: &mut Self)`
  Skips the current directory.

- `fn filter_entry<P>(self: Self, predicate: P) -> FilterEntry<Self, P>`
  Yields only entries which satisfy the given predicate and skips

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator`

##### `impl Iterator`

- `type Item = Result<DirEntry, Error>`

- `fn next(self: &mut Self) -> Option<Result<DirEntry>>`
  Advances the iterator and returns the next value.

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FilterEntry<I, P>`

```rust
struct FilterEntry<I, P> {
    // [REDACTED: Private Fields]
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

- `fn filter_entry(self: Self, predicate: P) -> FilterEntry<Self, P>`
  Yields only entries which satisfy the given predicate and skips

- `fn skip_current_dir(self: &mut Self)`
  Skips the current directory.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl FusedIterator<P>`

##### `impl Iterator<P>`

- `type Item = Result<DirEntry, Error>`

- `fn next(self: &mut Self) -> Option<Result<DirEntry>>`
  Advances the iterator and returns the next value.

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<I: $crate::fmt::Debug, P: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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



