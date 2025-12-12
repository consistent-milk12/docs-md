# Crate `camino`

UTF-8 encoded paths.

`camino` is an extension of the [`std::path`](../fs_err/path/index.md) module that adds new [`Utf8PathBuf`](#utf8pathbuf) and [`Utf8Path`](#utf8path)
types. These are like the standard library's [`PathBuf`](../clap_builder/index.md) and [`Path`](../rustdoc_types/index.md) types, except they are
guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
contents as strings, they implement [`Display`](../miette_derive/fmt/index.md), etc.

The [`std::path`](../fs_err/path/index.md) types are not guaranteed to be valid UTF-8. This is the right decision for the standard library,
since it must be as general as possible. However, on all platforms, non-Unicode paths are vanishingly uncommon for a
number of reasons:
* Unicode won. There are still some legacy codebases that store paths in encodings like Shift-JIS, but most
  have been converted to Unicode at this point.
* Unicode is the common subset of supported paths across Windows and Unix platforms. (On Windows, Rust stores paths
  as [an extension to UTF-8](https://simonsapin.github.io/wtf-8/), and converts them to UTF-16 at Win32
  API boundaries.)
* There are already many systems, such as Cargo, that only support UTF-8 paths. If your own tool interacts with any such
  system, you can assume that paths are valid UTF-8 without creating any additional burdens on consumers.
* The ["makefile problem"](https://www.mercurial-scm.org/wiki/EncodingStrategy#The_.22makefile_problem.22)
  (which also applies to `Cargo.toml`, and any other metadata file that lists the names of other files) has *no general,
  cross-platform solution* in systems that support non-UTF-8 paths. However, restricting paths to UTF-8 eliminates
  this problem.

Therefore, many programs that want to manipulate paths *do* assume they contain UTF-8 data, and convert them to [`str`](../clap_builder/builder/str/index.md)s
as  necessary. However, because this invariant is not encoded in the [`Path`](../rustdoc_types/index.md) type, conversions such as
`path.to_str().unwrap()` need to be repeated again and again, creating a frustrating experience.

Instead, `camino` allows you to check that your paths are UTF-8 *once*, and then manipulate them
as valid UTF-8 from there on, avoiding repeated lossy and confusing conversions.

## Contents

- [Modules](#modules)
  - [`serde_impls`](#serde-impls)
- [Structs](#structs)
  - [`Utf8PathBuf`](#utf8pathbuf)
  - [`Utf8Path`](#utf8path)
  - [`Utf8Ancestors`](#utf8ancestors)
  - [`Utf8Components`](#utf8components)
  - [`Iter`](#iter)
  - [`Utf8PrefixComponent`](#utf8prefixcomponent)
  - [`ReadDirUtf8`](#readdirutf8)
  - [`Utf8DirEntry`](#utf8direntry)
  - [`FromPathBufError`](#frompathbuferror)
  - [`FromPathError`](#frompatherror)
  - [`FromOsStringError`](#fromosstringerror)
  - [`FromOsStrError`](#fromosstrerror)
- [Enums](#enums)
  - [`Utf8Component`](#utf8component)
  - [`Utf8Prefix`](#utf8prefix)
- [Functions](#functions)
  - [`absolute_utf8`](#absolute-utf8)
  - [`str_assume_utf8`](#str-assume-utf8)
- [Macros](#macros)
  - [`impl_cmp!`](#impl-cmp)
  - [`impl_cmp_std_path!`](#impl-cmp-std-path)
  - [`impl_cmp_str!`](#impl-cmp-str)
  - [`impl_cmp_os_str!`](#impl-cmp-os-str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`serde_impls`](#serde-impls) | mod | Serde implementations for the types in this crate. |
| [`Utf8PathBuf`](#utf8pathbuf) | struct | An owned, mutable UTF-8 path (akin to [`String`]). |
| [`Utf8Path`](#utf8path) | struct | A slice of a UTF-8 path (akin to [`str`]). |
| [`Utf8Ancestors`](#utf8ancestors) | struct | An iterator over [`Utf8Path`] and its ancestors. |
| [`Utf8Components`](#utf8components) | struct | An iterator over the [`Utf8Component`]s of a [`Utf8Path`]. |
| [`Iter`](#iter) | struct | An iterator over the [`Utf8Component`]s of a [`Utf8Path`], as [`str`] slices. |
| [`Utf8PrefixComponent`](#utf8prefixcomponent) | struct | A structure wrapping a Windows path prefix as well as its unparsed string representation. |
| [`ReadDirUtf8`](#readdirutf8) | struct | Iterator over the entries in a directory. |
| [`Utf8DirEntry`](#utf8direntry) | struct | Entries returned by the [`ReadDirUtf8`] iterator. |
| [`FromPathBufError`](#frompathbuferror) | struct | A possible error value while converting a [`PathBuf`] to a [`Utf8PathBuf`]. |
| [`FromPathError`](#frompatherror) | struct | A possible error value while converting a [`Path`] to a [`Utf8Path`]. |
| [`FromOsStringError`](#fromosstringerror) | struct | A possible error value while converting a [`OsString`] to a [`Utf8PathBuf`]. |
| [`FromOsStrError`](#fromosstrerror) | struct | A possible error value while converting a [`OsStr`] to a [`Utf8Path`]. |
| [`Utf8Component`](#utf8component) | enum | A single component of a path. |
| [`Utf8Prefix`](#utf8prefix) | enum | Windows path prefixes, e.g., `C:` or `\\server\share`. |
| [`absolute_utf8`](#absolute-utf8) | fn | Makes the path absolute without accessing the filesystem, converting it to a [`Utf8PathBuf`]. |
| [`str_assume_utf8`](#str-assume-utf8) | fn |  |
| [`impl_cmp!`](#impl-cmp) | macro |  |
| [`impl_cmp_std_path!`](#impl-cmp-std-path) | macro |  |
| [`impl_cmp_str!`](#impl-cmp-str) | macro |  |
| [`impl_cmp_os_str!`](#impl-cmp-os-str) | macro |  |

## Modules

- [`serde_impls`](serde_impls/index.md) — Serde implementations for the types in this crate.

## Structs

### `Utf8PathBuf`

```rust
struct Utf8PathBuf(PathBuf);
```

*Defined in [`camino-1.2.1/src/lib.rs:112`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L112)*

An owned, mutable UTF-8 path (akin to [`String`](../cargo_platform/index.md)).

This type provides methods like `push` and `set_extension` that mutate
the path in place. It also implements [`Deref`](../gimli/index.md) to [`Utf8Path`](#utf8path), meaning that
all methods on [`Utf8Path`](#utf8path) slices are available on [`Utf8PathBuf`](#utf8pathbuf) values as well.


# Examples

You can use `push` to build up a [`Utf8PathBuf`](#utf8pathbuf) from
components:

```rust
use camino::Utf8PathBuf;

let mut path = Utf8PathBuf::new();

path.push(r"C:\");
path.push("windows");
path.push("system32");

path.set_extension("dll");
```

However, `push` is best used for dynamic situations. This is a better way
to do this when you know all of the components ahead of time:

```rust
use camino::Utf8PathBuf;

let path: Utf8PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
```

We can still do better than this! Since these are all strings, we can use

use camino::Utf8PathBuf;

let path = Utf8PathBuf::from(r"C:\windows\system32.dll");
```rust

Which method works best depends on what kind of situation you're in.

#### Implementations

- <span id="utf8pathbuf-new"></span>`fn new() -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8pathbuf-from-path-buf"></span>`fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8pathbuf-from-os-string"></span>`fn from_os_string(os_string: OsString) -> Result<Utf8PathBuf, OsString>` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8pathbuf-into-std-path-buf"></span>`fn into_std_path_buf(self) -> PathBuf`

- <span id="utf8pathbuf-with-capacity"></span>`fn with_capacity(capacity: usize) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8pathbuf-as-path"></span>`fn as_path(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8pathbuf-leak"></span>`fn leak<'a>(self) -> &'a mut Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8pathbuf-push"></span>`fn push(&mut self, path: impl AsRef<Utf8Path>)` — [`Utf8Path`](#utf8path)

- <span id="utf8pathbuf-pop"></span>`fn pop(&mut self) -> bool`

- <span id="utf8pathbuf-set-file-name"></span>`fn set_file_name(&mut self, file_name: impl AsRef<str>)`

- <span id="utf8pathbuf-set-extension"></span>`fn set_extension(&mut self, extension: impl AsRef<str>) -> bool`

- <span id="utf8pathbuf-into-string"></span>`fn into_string(self) -> String`

- <span id="utf8pathbuf-into-os-string"></span>`fn into_os_string(self) -> OsString`

- <span id="utf8pathbuf-into-boxed-path"></span>`fn into_boxed_path(self) -> Box<Utf8Path>` — [`Utf8Path`](#utf8path)

- <span id="utf8pathbuf-capacity"></span>`fn capacity(&self) -> usize`

- <span id="utf8pathbuf-clear"></span>`fn clear(&mut self)`

- <span id="utf8pathbuf-reserve"></span>`fn reserve(&mut self, additional: usize)`

- <span id="utf8pathbuf-try-reserve"></span>`fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>`

- <span id="utf8pathbuf-reserve-exact"></span>`fn reserve_exact(&mut self, additional: usize)`

- <span id="utf8pathbuf-try-reserve-exact"></span>`fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>`

- <span id="utf8pathbuf-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

- <span id="utf8pathbuf-shrink-to"></span>`fn shrink_to(&mut self, min_capacity: usize)`

#### Trait Implementations

##### `impl AsRef for Utf8PathBuf`

- <span id="utf8pathbuf-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Clone for Utf8PathBuf`

- <span id="utf8pathbuf-clone"></span>`fn clone(&self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl Debug for Utf8PathBuf`

- <span id="utf8pathbuf-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8PathBuf`

- <span id="utf8pathbuf-default"></span>`fn default() -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl Deref for Utf8PathBuf`

- <span id="utf8pathbuf-deref-type-target"></span>`type Target = Utf8Path`

- <span id="utf8pathbuf-deref"></span>`fn deref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl DerefMut for Utf8PathBuf`

- <span id="utf8pathbuf-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Deserialize for crate::Utf8PathBuf`

- <span id="crateutf8pathbuf-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Utf8PathBuf`

##### `impl Display for Utf8PathBuf`

- <span id="utf8pathbuf-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8PathBuf`

##### `impl<P: AsRef<Utf8Path>> Extend for Utf8PathBuf`

- <span id="utf8pathbuf-extend"></span>`fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I)`

##### `impl<P: AsRef<Utf8Path>> FromIterator for Utf8PathBuf`

- <span id="utf8pathbuf-from-iter"></span>`fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl FromStr for Utf8PathBuf`

- <span id="utf8pathbuf-fromstr-type-err"></span>`type Err = Infallible`

- <span id="utf8pathbuf-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Utf8PathBuf`

- <span id="utf8pathbuf-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IntoIterator for &'a Utf8PathBuf`

- <span id="a-utf8pathbuf-intoiterator-type-item"></span>`type Item = &'a str`

- <span id="a-utf8pathbuf-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-utf8pathbuf-into-iter"></span>`fn into_iter(self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Ord for Utf8PathBuf`

- <span id="utf8pathbuf-cmp"></span>`fn cmp(&self, other: &Utf8PathBuf) -> Ordering` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl PartialEq for Utf8PathBuf`

- <span id="utf8pathbuf-eq"></span>`fn eq(&self, other: &Utf8PathBuf) -> bool` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl PartialOrd for Utf8PathBuf`

- <span id="utf8pathbuf-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8PathBuf) -> Option<Ordering>` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl Receiver for Utf8PathBuf`

- <span id="utf8pathbuf-receiver-type-target"></span>`type Target = T`

##### `impl Serialize for crate::Utf8PathBuf`

- <span id="crateutf8pathbuf-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToString for Utf8PathBuf`

- <span id="utf8pathbuf-to-string"></span>`fn to_string(&self) -> String`

### `Utf8Path`

```rust
struct Utf8Path(Path);
```

*Defined in [`camino-1.2.1/src/lib.rs:600`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L600)*

A slice of a UTF-8 path (akin to [`str`](../clap_builder/builder/str/index.md)).

This type supports a number of operations for inspecting a path, including
breaking the path into its components (separated by `/` on Unix and by either
`/` or `\` on Windows), extracting the file name, determining whether the path
is absolute, and so on.

This is an *unsized* type, meaning that it must always be used behind a
pointer like `&` or [`Box`](../allocator_api2/stable/boxed/index.md). For an owned version of this type,
see [`Utf8PathBuf`](#utf8pathbuf).

# Examples

```rust
use camino::Utf8Path;

// Note: this example does work on Windows
let path = Utf8Path::new("./foo/bar.txt");

let parent = path.parent();
assert_eq!(parent, Some(Utf8Path::new("./foo")));

let file_stem = path.file_stem();
assert_eq!(file_stem, Some("bar"));

let extension = path.extension();
assert_eq!(extension, Some("txt"));
```

#### Implementations

- <span id="utf8path-new"></span>`fn new(s: &impl AsRef<str> + ?Sized) -> &Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8path-from-path"></span>`fn from_path(path: &Path) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

- <span id="utf8path-from-os-str"></span>`fn from_os_str(path: &OsStr) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

- <span id="utf8path-as-std-path"></span>`fn as_std_path(&self) -> &Path`

- <span id="utf8path-as-str"></span>`fn as_str(&self) -> &str`

- <span id="utf8path-as-os-str"></span>`fn as_os_str(&self) -> &OsStr`

- <span id="utf8path-to-path-buf"></span>`fn to_path_buf(&self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-is-absolute"></span>`fn is_absolute(&self) -> bool`

- <span id="utf8path-is-relative"></span>`fn is_relative(&self) -> bool`

- <span id="utf8path-has-root"></span>`fn has_root(&self) -> bool`

- <span id="utf8path-parent"></span>`fn parent(&self) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

- <span id="utf8path-ancestors"></span>`fn ancestors(&self) -> Utf8Ancestors<'_>` — [`Utf8Ancestors`](#utf8ancestors)

- <span id="utf8path-file-name"></span>`fn file_name(&self) -> Option<&str>`

- <span id="utf8path-strip-prefix"></span>`fn strip_prefix(&self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError>` — [`Utf8Path`](#utf8path)

- <span id="utf8path-starts-with"></span>`fn starts_with(&self, base: impl AsRef<Path>) -> bool`

- <span id="utf8path-ends-with"></span>`fn ends_with(&self, base: impl AsRef<Path>) -> bool`

- <span id="utf8path-file-stem"></span>`fn file_stem(&self) -> Option<&str>`

- <span id="utf8path-extension"></span>`fn extension(&self) -> Option<&str>`

- <span id="utf8path-join"></span>`fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf` — [`Utf8Path`](#utf8path), [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-join-os"></span>`fn join_os(&self, path: impl AsRef<Path>) -> PathBuf`

- <span id="utf8path-with-file-name"></span>`fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-with-extension"></span>`fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-components"></span>`fn components(&self) -> Utf8Components<'_>` — [`Utf8Components`](#utf8components)

- <span id="utf8path-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](#iter)

- <span id="utf8path-metadata"></span>`fn metadata(&self) -> io::Result<fs::Metadata>`

- <span id="utf8path-symlink-metadata"></span>`fn symlink_metadata(&self) -> io::Result<fs::Metadata>`

- <span id="utf8path-canonicalize"></span>`fn canonicalize(&self) -> io::Result<PathBuf>`

- <span id="utf8path-canonicalize-utf8"></span>`fn canonicalize_utf8(&self) -> io::Result<Utf8PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-read-link"></span>`fn read_link(&self) -> io::Result<PathBuf>`

- <span id="utf8path-read-link-utf8"></span>`fn read_link_utf8(&self) -> io::Result<Utf8PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-read-dir"></span>`fn read_dir(&self) -> io::Result<fs::ReadDir>`

- <span id="utf8path-read-dir-utf8"></span>`fn read_dir_utf8(&self) -> io::Result<ReadDirUtf8>` — [`ReadDirUtf8`](#readdirutf8)

- <span id="utf8path-exists"></span>`fn exists(&self) -> bool`

- <span id="utf8path-try-exists"></span>`fn try_exists(&self) -> io::Result<bool>`

- <span id="utf8path-is-file"></span>`fn is_file(&self) -> bool`

- <span id="utf8path-is-dir"></span>`fn is_dir(&self) -> bool`

- <span id="utf8path-is-symlink"></span>`fn is_symlink(&self) -> bool`

- <span id="utf8path-into-path-buf"></span>`fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf` — [`Utf8Path`](#utf8path), [`Utf8PathBuf`](#utf8pathbuf)

- <span id="utf8path-assume-utf8"></span>`unsafe fn assume_utf8(path: &Path) -> &Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8path-assume-utf8-mut"></span>`unsafe fn assume_utf8_mut(path: &mut Path) -> &mut Utf8Path` — [`Utf8Path`](#utf8path)

#### Trait Implementations

##### `impl AsRef for Utf8Components<'_>`

- <span id="utf8components-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Clone for Box<Utf8Path>`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Utf8Path`

- <span id="utf8path-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for &'a crate::Utf8Path`

- <span id="a-crateutf8path-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl Display for Utf8Path`

- <span id="utf8path-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Path`

##### `impl Hash for Utf8Path`

- <span id="utf8path-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IntoIterator for &'a Utf8Path`

- <span id="a-utf8path-intoiterator-type-item"></span>`type Item = &'a str`

- <span id="a-utf8path-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-utf8path-into-iter"></span>`fn into_iter(self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Ord for Utf8Path`

- <span id="utf8path-cmp"></span>`fn cmp(&self, other: &Utf8Path) -> Ordering` — [`Utf8Path`](#utf8path)

##### `impl PartialEq for Utf8Path`

- <span id="utf8path-eq"></span>`fn eq(&self, other: &Utf8Path) -> bool` — [`Utf8Path`](#utf8path)

##### `impl PartialOrd for Utf8Path`

- <span id="utf8path-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Path) -> Option<Ordering>` — [`Utf8Path`](#utf8path)

##### `impl Serialize for crate::Utf8Path`

- <span id="crateutf8path-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToString for Utf8Path`

- <span id="utf8path-to-string"></span>`fn to_string(&self) -> String`

### `Utf8Ancestors<'a>`

```rust
struct Utf8Ancestors<'a>(Ancestors<'a>);
```

*Defined in [`camino-1.2.1/src/lib.rs:1641`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L1641)*

An iterator over [`Utf8Path`](#utf8path) and its ancestors.

This `struct` is created by the `ancestors` method on [`Utf8Path`](#utf8path).
See its documentation for more.

# Examples

```rust
use camino::Utf8Path;

let path = Utf8Path::new("/foo/bar");

for ancestor in path.ancestors() {
    println!("{}", ancestor);
}
```


#### Trait Implementations

##### `impl Clone for Utf8Ancestors<'a>`

- <span id="utf8ancestors-clone"></span>`fn clone(&self) -> Utf8Ancestors<'a>` — [`Utf8Ancestors`](#utf8ancestors)

##### `impl Copy for Utf8Ancestors<'a>`

##### `impl Debug for Utf8Ancestors<'_>`

- <span id="utf8ancestors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Utf8Ancestors<'_>`

##### `impl IntoIterator for Utf8Ancestors<'a>`

- <span id="utf8ancestors-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="utf8ancestors-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="utf8ancestors-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Utf8Ancestors<'a>`

- <span id="utf8ancestors-iterator-type-item"></span>`type Item = &'a Utf8Path`

- <span id="utf8ancestors-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Utf8Components<'a>`

```rust
struct Utf8Components<'a>(Components<'a>);
```

*Defined in [`camino-1.2.1/src/lib.rs:1684`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L1684)*

An iterator over the [`Utf8Component`](#utf8component)s of a [`Utf8Path`](#utf8path).

This `struct` is created by the `components` method on [`Utf8Path`](#utf8path).
See its documentation for more.

# Examples

```rust
use camino::Utf8Path;

let path = Utf8Path::new("/tmp/foo/bar.txt");

for component in path.components() {
    println!("{:?}", component);
}
```


#### Implementations

- <span id="utf8components-as-path"></span>`fn as_path(&self) -> &'a Utf8Path` — [`Utf8Path`](#utf8path)

#### Trait Implementations

##### `impl AsRef for Utf8Components<'_>`

- <span id="utf8components-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Clone for Utf8Components<'a>`

- <span id="utf8components-clone"></span>`fn clone(&self) -> Utf8Components<'a>` — [`Utf8Components`](#utf8components)

##### `impl Debug for Utf8Components<'_>`

- <span id="utf8components-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Utf8Components<'_>`

- <span id="utf8components-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl Eq for Utf8Components<'a>`

##### `impl FusedIterator for Utf8Components<'_>`

##### `impl IntoIterator for Utf8Components<'a>`

- <span id="utf8components-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="utf8components-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="utf8components-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Utf8Components<'a>`

- <span id="utf8components-iterator-type-item"></span>`type Item = Utf8Component<'a>`

- <span id="utf8components-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Ord for Utf8Components<'a>`

- <span id="utf8components-cmp"></span>`fn cmp(&self, other: &Utf8Components<'a>) -> cmp::Ordering` — [`Utf8Components`](#utf8components)

##### `impl PartialEq for Utf8Components<'a>`

- <span id="utf8components-eq"></span>`fn eq(&self, other: &Utf8Components<'a>) -> bool` — [`Utf8Components`](#utf8components)

##### `impl PartialOrd for Utf8Components<'a>`

- <span id="utf8components-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Components<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Components`](#utf8components)

##### `impl StructuralPartialEq for Utf8Components<'a>`

### `Iter<'a>`

```rust
struct Iter<'a> {
    inner: Utf8Components<'a>,
}
```

*Defined in [`camino-1.2.1/src/lib.rs:1777-1779`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L1777-L1779)*

An iterator over the [`Utf8Component`](#utf8component)s of a [`Utf8Path`](#utf8path), as [`str`](../clap_builder/builder/str/index.md) slices.

This `struct` is created by the `iter` method on [`Utf8Path`](#utf8path).
See its documentation for more.


#### Implementations

- <span id="iter-as-path"></span>`fn as_path(&self) -> &'a Utf8Path` — [`Utf8Path`](#utf8path)

#### Trait Implementations

##### `impl AsRef for Iter<'_>`

- <span id="iter-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Clone for Iter<'a>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Debug for Iter<'_>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Iter<'a>`

- <span id="iter-next-back"></span>`fn next_back(&mut self) -> Option<&'a str>`

##### `impl FusedIterator for Iter<'_>`

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a str`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<&'a str>`

### `Utf8PrefixComponent<'a>`

```rust
struct Utf8PrefixComponent<'a>(PrefixComponent<'a>);
```

*Defined in [`camino-1.2.1/src/lib.rs:2132`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2132)*

A structure wrapping a Windows path prefix as well as its unparsed string
representation.

In addition to the parsed [`Utf8Prefix`](#utf8prefix) information returned by [`kind`](#kind),
[`Utf8PrefixComponent`](#utf8prefixcomponent) also holds the raw and unparsed [`str`](../clap_builder/builder/str/index.md) slice,
returned by [`as_str`](#as-str).

Instances of this `struct` can be obtained by matching against the
[`Prefix` variant] on [`Utf8Component`](#utf8component).

Does not occur on Unix.

# Examples

```rust
if cfg!(windows) {
use camino::{Utf8Component, Utf8Path, Utf8Prefix};
use std::ffi::OsStr;

let path = Utf8Path::new(r"C:\you\later\");
match path.components().next().unwrap() {
    Utf8Component::Prefix(prefix_component) => {
        assert_eq!(Utf8Prefix::Disk(b'C'), prefix_component.kind());
        assert_eq!("C:", prefix_component.as_str());
    }
    _ => unreachable!(),
}
}
```




#### Implementations

- <span id="utf8prefixcomponent-kind"></span>`fn kind(&self) -> Utf8Prefix<'a>` — [`Utf8Prefix`](#utf8prefix)

- <span id="utf8prefixcomponent-as-str"></span>`fn as_str(&self) -> &'a str`

- <span id="utf8prefixcomponent-as-os-str"></span>`fn as_os_str(&self) -> &'a OsStr`

#### Trait Implementations

##### `impl Clone for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-clone"></span>`fn clone(&self) -> Utf8PrefixComponent<'a>` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl Copy for Utf8PrefixComponent<'a>`

##### `impl Debug for Utf8PrefixComponent<'_>`

- <span id="utf8prefixcomponent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8PrefixComponent<'_>`

- <span id="utf8prefixcomponent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8PrefixComponent<'a>`

##### `impl Hash for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-cmp"></span>`fn cmp(&self, other: &Utf8PrefixComponent<'a>) -> cmp::Ordering` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl PartialEq for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-eq"></span>`fn eq(&self, other: &Utf8PrefixComponent<'a>) -> bool` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl PartialOrd for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8PrefixComponent<'a>) -> option::Option<cmp::Ordering>` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl StructuralPartialEq for Utf8PrefixComponent<'a>`

##### `impl ToString for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-to-string"></span>`fn to_string(&self) -> String`

### `ReadDirUtf8`

```rust
struct ReadDirUtf8 {
    inner: fs::ReadDir,
}
```

*Defined in [`camino-1.2.1/src/lib.rs:2212-2214`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2212-L2214)*

Iterator over the entries in a directory.

This iterator is returned from `Utf8Path::read_dir_utf8` and will yield instances of
<code>[io::Result]<[Utf8DirEntry]></code>. Through a [`Utf8DirEntry`](#utf8direntry) information like the entry's path
and possibly other metadata can be learned.

The order in which this iterator returns entries is platform and filesystem
dependent.

# Errors

This [`io::Result`](../cargo_metadata/errors/index.md) will be an `Err` if there's some sort of intermittent
IO error during iteration.

If a directory entry is not UTF-8, an [`io::Error`](../addr2line/index.md) is returned with the
[`ErrorKind`](io::ErrorKind) set to `InvalidData`
and the payload set to a [`FromPathBufError`](#frompathbuferror).

#### Trait Implementations

##### `impl Debug for ReadDirUtf8`

- <span id="readdirutf8-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ReadDirUtf8`

- <span id="readdirutf8-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="readdirutf8-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="readdirutf8-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ReadDirUtf8`

- <span id="readdirutf8-iterator-type-item"></span>`type Item = Result<Utf8DirEntry, Error>`

- <span id="readdirutf8-next"></span>`fn next(&mut self) -> Option<io::Result<Utf8DirEntry>>` — [`Utf8DirEntry`](#utf8direntry)

### `Utf8DirEntry`

```rust
struct Utf8DirEntry {
    inner: fs::DirEntry,
    path: Utf8PathBuf,
}
```

*Defined in [`camino-1.2.1/src/lib.rs:2231-2234`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2231-L2234)*

Entries returned by the [`ReadDirUtf8`](#readdirutf8) iterator.

An instance of [`Utf8DirEntry`](#utf8direntry) represents an entry inside of a directory on the filesystem. Each
entry can be inspected via methods to learn about the full path or possibly other metadata.

#### Implementations

- <span id="utf8direntry-new"></span>`fn new(inner: fs::DirEntry) -> io::Result<Self>`

- <span id="utf8direntry-path"></span>`fn path(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8direntry-metadata"></span>`fn metadata(&self) -> io::Result<Metadata>`

- <span id="utf8direntry-file-type"></span>`fn file_type(&self) -> io::Result<fs::FileType>`

- <span id="utf8direntry-file-name"></span>`fn file_name(&self) -> &str`

- <span id="utf8direntry-into-inner"></span>`fn into_inner(self) -> fs::DirEntry`

- <span id="utf8direntry-into-path"></span>`fn into_path(self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

#### Trait Implementations

##### `impl Debug for Utf8DirEntry`

- <span id="utf8direntry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FromPathBufError`

```rust
struct FromPathBufError {
    path: PathBuf,
    error: FromPathError,
}
```

*Defined in [`camino-1.2.1/src/lib.rs:2700-2703`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2700-L2703)*

A possible error value while converting a [`PathBuf`](../clap_builder/index.md) to a [`Utf8PathBuf`](#utf8pathbuf).

Produced by the [`TryFrom<&PathBuf>`][tryfrom] implementation for [`Utf8PathBuf`](#utf8pathbuf).

# Examples

```rust
use camino::{Utf8PathBuf, FromPathBufError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

let unicode_path = PathBuf::from("/valid/unicode");
let utf8_path_buf: Utf8PathBuf = unicode_path.try_into().expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
#[cfg(unix)]
let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
#[cfg(unix)]
let non_unicode_path = PathBuf::from(non_unicode_str);
#[cfg(unix)]
let err: FromPathBufError = Utf8PathBuf::try_from(non_unicode_path.clone())
    .expect_err("non-Unicode path failed");
#[cfg(unix)]
assert_eq!(err.as_path(), &non_unicode_path);
#[cfg(unix)]
assert_eq!(err.into_path_buf(), non_unicode_path);
```

#### Implementations

- <span id="frompathbuferror-as-path"></span>`fn as_path(&self) -> &Path`

- <span id="frompathbuferror-into-path-buf"></span>`fn into_path_buf(self) -> PathBuf`

- <span id="frompathbuferror-from-path-error"></span>`fn from_path_error(&self) -> FromPathError` — [`FromPathError`](#frompatherror)

- <span id="frompathbuferror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

#### Trait Implementations

##### `impl Clone for FromPathBufError`

- <span id="frompathbuferror-clone"></span>`fn clone(&self) -> FromPathBufError` — [`FromPathBufError`](#frompathbuferror)

##### `impl Debug for FromPathBufError`

- <span id="frompathbuferror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromPathBufError`

- <span id="frompathbuferror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromPathBufError`

##### `impl Error for FromPathBufError`

- <span id="frompathbuferror-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl PartialEq for FromPathBufError`

- <span id="frompathbuferror-eq"></span>`fn eq(&self, other: &FromPathBufError) -> bool` — [`FromPathBufError`](#frompathbuferror)

##### `impl StructuralPartialEq for FromPathBufError`

##### `impl ToString for FromPathBufError`

- <span id="frompathbuferror-to-string"></span>`fn to_string(&self) -> String`

### `FromPathError`

```rust
struct FromPathError(());
```

*Defined in [`camino-1.2.1/src/lib.rs:2781`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2781)*

A possible error value while converting a [`Path`](../rustdoc_types/index.md) to a [`Utf8Path`](#utf8path).

Produced by the [`TryFrom<&Path>`][tryfrom] implementation for [`&Utf8Path`](Utf8Path).

# Examples

```rust
use camino::{Utf8Path, FromPathError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

let unicode_path = Path::new("/valid/unicode");
let utf8_path: &Utf8Path = unicode_path.try_into().expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
#[cfg(unix)]
let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
#[cfg(unix)]
let non_unicode_path = Path::new(non_unicode_str);
#[cfg(unix)]
let err: FromPathError = <&Utf8Path>::try_from(non_unicode_path)
    .expect_err("non-Unicode path failed");
```

#### Implementations

- <span id="frompatherror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

#### Trait Implementations

##### `impl Clone for FromPathError`

- <span id="frompatherror-clone"></span>`fn clone(&self) -> FromPathError` — [`FromPathError`](#frompatherror)

##### `impl Copy for FromPathError`

##### `impl Debug for FromPathError`

- <span id="frompatherror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromPathError`

- <span id="frompatherror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromPathError`

##### `impl Error for FromPathError`

- <span id="frompatherror-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl PartialEq for FromPathError`

- <span id="frompatherror-eq"></span>`fn eq(&self, other: &FromPathError) -> bool` — [`FromPathError`](#frompatherror)

##### `impl StructuralPartialEq for FromPathError`

##### `impl ToString for FromPathError`

- <span id="frompatherror-to-string"></span>`fn to_string(&self) -> String`

### `FromOsStringError`

```rust
struct FromOsStringError {
    os_string: std::ffi::OsString,
    error: FromOsStrError,
}
```

*Defined in [`camino-1.2.1/src/lib.rs:2841-2844`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2841-L2844)*

A possible error value while converting a [`OsString`](../clap_builder/index.md) to a [`Utf8PathBuf`](#utf8pathbuf).

Produced by the `TryFrom<OsString>` implementation for [`Utf8PathBuf`](#utf8pathbuf).

# Examples

```rust
#[cfg(osstring_from_str)] {
use camino::{Utf8PathBuf, FromOsStringError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
use std::str::FromStr;
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

let unicode_string = OsString::from_str("/valid/unicode").unwrap();
let utf8_path_buf: Utf8PathBuf = unicode_string.try_into()
    .expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
#[cfg(unix)]
let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").to_owned();
#[cfg(unix)]
let err: FromOsStringError = Utf8PathBuf::try_from(non_unicode_string.clone())
    .expect_err("non-Unicode path failed");
#[cfg(unix)]
assert_eq!(err.as_os_str(), &non_unicode_string);
#[cfg(unix)]
assert_eq!(err.into_os_string(), non_unicode_string);
}
```

#### Implementations

- <span id="fromosstringerror-as-os-str"></span>`fn as_os_str(&self) -> &OsStr`

- <span id="fromosstringerror-into-os-string"></span>`fn into_os_string(self) -> OsString`

- <span id="fromosstringerror-from-os-str-error"></span>`fn from_os_str_error(&self) -> FromOsStrError` — [`FromOsStrError`](#fromosstrerror)

- <span id="fromosstringerror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

#### Trait Implementations

##### `impl Clone for FromOsStringError`

- <span id="fromosstringerror-clone"></span>`fn clone(&self) -> FromOsStringError` — [`FromOsStringError`](#fromosstringerror)

##### `impl Debug for FromOsStringError`

- <span id="fromosstringerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromOsStringError`

- <span id="fromosstringerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromOsStringError`

##### `impl Error for FromOsStringError`

- <span id="fromosstringerror-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl PartialEq for FromOsStringError`

- <span id="fromosstringerror-eq"></span>`fn eq(&self, other: &FromOsStringError) -> bool` — [`FromOsStringError`](#fromosstringerror)

##### `impl StructuralPartialEq for FromOsStringError`

##### `impl ToString for FromOsStringError`

- <span id="fromosstringerror-to-string"></span>`fn to_string(&self) -> String`

### `FromOsStrError`

```rust
struct FromOsStrError(());
```

*Defined in [`camino-1.2.1/src/lib.rs:2922`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2922)*

A possible error value while converting a [`OsStr`](../clap_builder/builder/os_str/index.md) to a [`Utf8Path`](#utf8path).

Produced by the `TryFrom<&OsStr>` implementation for [`&Utf8Path`](Utf8Path).


# Examples

```rust
use camino::{Utf8Path, FromOsStrError};
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

let unicode_str = OsStr::new("/valid/unicode");
let utf8_path: &Utf8Path = unicode_str.try_into().expect("valid Unicode path succeeded");

// Paths on Unix can be non-UTF-8.
#[cfg(unix)]
let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
#[cfg(unix)]
let err: FromOsStrError = <&Utf8Path>::try_from(non_unicode_str)
    .expect_err("non-Unicode path failed");
```

#### Implementations

- <span id="fromosstrerror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

#### Trait Implementations

##### `impl Clone for FromOsStrError`

- <span id="fromosstrerror-clone"></span>`fn clone(&self) -> FromOsStrError` — [`FromOsStrError`](#fromosstrerror)

##### `impl Copy for FromOsStrError`

##### `impl Debug for FromOsStrError`

- <span id="fromosstrerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromOsStrError`

- <span id="fromosstrerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromOsStrError`

##### `impl Error for FromOsStrError`

- <span id="fromosstrerror-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl PartialEq for FromOsStrError`

- <span id="fromosstrerror-eq"></span>`fn eq(&self, other: &FromOsStrError) -> bool` — [`FromOsStrError`](#fromosstrerror)

##### `impl StructuralPartialEq for FromOsStrError`

##### `impl ToString for FromOsStrError`

- <span id="fromosstrerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Utf8Component<'a>`

```rust
enum Utf8Component<'a> {
    Prefix(Utf8PrefixComponent<'a>),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a str),
}
```

*Defined in [`camino-1.2.1/src/lib.rs:1887-1912`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L1887-L1912)*

A single component of a path.

A [`Utf8Component`](#utf8component) roughly corresponds to a substring between path separators
(`/` or `\`).

This `enum` is created by iterating over [`Utf8Components`](#utf8components), which in turn is
created by the [`components`](Utf8Path::components) method on [`Utf8Path`](#utf8path).

# Examples

```rust
use camino::{Utf8Component, Utf8Path};

let path = Utf8Path::new("/tmp/foo/bar.txt");
let components = path.components().collect::<Vec<_>>();
assert_eq!(&components, &[
    Utf8Component::RootDir,
    Utf8Component::Normal("tmp"),
    Utf8Component::Normal("foo"),
    Utf8Component::Normal("bar.txt"),
]);
```

#### Variants

- **`Prefix`**

  A Windows path prefix, e.g., `C:` or `\\server\share`.
  
  There is a large variety of prefix types, see [`Utf8Prefix`](#utf8prefix)'s documentation
  for more.
  
  Does not occur on Unix.

- **`RootDir`**

  The root directory component, appears after any prefix and before anything else.
  
  It represents a separator that designates that a path starts from root.

- **`CurDir`**

  A reference to the current directory, i.e., `.`.

- **`ParentDir`**

  A reference to the parent directory, i.e., `..`.

- **`Normal`**

  A normal component, e.g., `a` and `b` in `a/b`.
  
  This variant is the most common one, it represents references to files
  or directories.

#### Implementations

- <span id="utf8component-new"></span>`unsafe fn new(component: Component<'a>) -> Utf8Component<'a>` — [`Utf8Component`](#utf8component)

- <span id="utf8component-as-str"></span>`fn as_str(&self) -> &'a str`

- <span id="utf8component-as-os-str"></span>`fn as_os_str(&self) -> &'a OsStr`

#### Trait Implementations

##### `impl AsRef for Utf8Component<'_>`

- <span id="utf8component-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Clone for Utf8Component<'a>`

- <span id="utf8component-clone"></span>`fn clone(&self) -> Utf8Component<'a>` — [`Utf8Component`](#utf8component)

##### `impl Copy for Utf8Component<'a>`

##### `impl Debug for Utf8Component<'_>`

- <span id="utf8component-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8Component<'_>`

- <span id="utf8component-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Component<'a>`

##### `impl Hash for Utf8Component<'a>`

- <span id="utf8component-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Utf8Component<'a>`

- <span id="utf8component-cmp"></span>`fn cmp(&self, other: &Utf8Component<'a>) -> cmp::Ordering` — [`Utf8Component`](#utf8component)

##### `impl PartialEq for Utf8Component<'a>`

- <span id="utf8component-eq"></span>`fn eq(&self, other: &Utf8Component<'a>) -> bool` — [`Utf8Component`](#utf8component)

##### `impl PartialOrd for Utf8Component<'a>`

- <span id="utf8component-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Component<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Component`](#utf8component)

##### `impl StructuralPartialEq for Utf8Component<'a>`

##### `impl ToString for Utf8Component<'a>`

- <span id="utf8component-to-string"></span>`fn to_string(&self) -> String`

### `Utf8Prefix<'a>`

```rust
enum Utf8Prefix<'a> {
    Verbatim(&'a str),
    VerbatimUNC(&'a str, &'a str),
    VerbatimDisk(u8),
    DeviceNS(&'a str),
    UNC(&'a str, &'a str),
    Disk(u8),
}
```

*Defined in [`camino-1.2.1/src/lib.rs:2039-2073`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L2039-L2073)*

Windows path prefixes, e.g., `C:` or `\\server\share`.

Windows uses a variety of path prefix styles, including references to drive
volumes (like `C:`), network shared folders (like `\\server\share`), and
others. In addition, some path prefixes are "verbatim" (i.e., prefixed with
`\\?\`), in which case `/` is *not* treated as a separator and essentially
no normalization is performed.

# Examples

```rust
use camino::{Utf8Component, Utf8Path, Utf8Prefix};
use camino::Utf8Prefix::*;

fn get_path_prefix(s: &str) -> Utf8Prefix {
    let path = Utf8Path::new(s);
    match path.components().next().unwrap() {
        Utf8Component::Prefix(prefix_component) => prefix_component.kind(),
        _ => panic!(),
    }
}

if cfg!(windows) {
assert_eq!(Verbatim("pictures"), get_path_prefix(r"\\?\pictures\kittens"));
assert_eq!(VerbatimUNC("server", "share"), get_path_prefix(r"\\?\UNC\server\share"));
assert_eq!(VerbatimDisk(b'C'), get_path_prefix(r"\\?\C:\"));
assert_eq!(DeviceNS("BrainInterface"), get_path_prefix(r"\\.\BrainInterface"));
assert_eq!(UNC("server", "share"), get_path_prefix(r"\\server\share"));
assert_eq!(Disk(b'C'), get_path_prefix(r"C:\Users\Rust\Pictures\Ferris"));
}
```

#### Variants

- **`Verbatim`**

  Verbatim prefix, e.g., `\\?\cat_pics`.
  
  Verbatim prefixes consist of `\\?\` immediately followed by the given
  component.

- **`VerbatimUNC`**

  Verbatim prefix using Windows' _**U**niform **N**aming **C**onvention_,
  e.g., `\\?\UNC\server\share`.
  
  Verbatim UNC prefixes consist of `\\?\UNC\` immediately followed by the
  server's hostname and a share name.

- **`VerbatimDisk`**

  Verbatim disk prefix, e.g., `\\?\C:`.
  
  Verbatim disk prefixes consist of `\\?\` immediately followed by the
  drive letter and `:`.

- **`DeviceNS`**

  Device namespace prefix, e.g., `\\.\COM42`.
  
  Device namespace prefixes consist of `\\.\` immediately followed by the
  device name.

- **`UNC`**

  Prefix using Windows' _**U**niform **N**aming **C**onvention_, e.g.
  `\\server\share`.
  
  UNC prefixes consist of the server's hostname and a share name.

- **`Disk`**

  Prefix `C:` for the given disk drive.

#### Implementations

- <span id="utf8prefix-is-verbatim"></span>`fn is_verbatim(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Prefix<'a>`

- <span id="utf8prefix-clone"></span>`fn clone(&self) -> Utf8Prefix<'a>` — [`Utf8Prefix`](#utf8prefix)

##### `impl Copy for Utf8Prefix<'a>`

##### `impl Debug for Utf8Prefix<'a>`

- <span id="utf8prefix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Prefix<'a>`

##### `impl Hash for Utf8Prefix<'a>`

- <span id="utf8prefix-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Utf8Prefix<'a>`

- <span id="utf8prefix-cmp"></span>`fn cmp(&self, other: &Utf8Prefix<'a>) -> cmp::Ordering` — [`Utf8Prefix`](#utf8prefix)

##### `impl PartialEq for Utf8Prefix<'a>`

- <span id="utf8prefix-eq"></span>`fn eq(&self, other: &Utf8Prefix<'a>) -> bool` — [`Utf8Prefix`](#utf8prefix)

##### `impl PartialOrd for Utf8Prefix<'a>`

- <span id="utf8prefix-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Prefix<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Prefix`](#utf8prefix)

##### `impl StructuralPartialEq for Utf8Prefix<'a>`

## Functions

### `absolute_utf8`

```rust
fn absolute_utf8<P: AsRef<Path>>(path: P) -> io::Result<Utf8PathBuf>
```

*Defined in [`camino-1.2.1/src/lib.rs:3395-3405`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3395-L3405)*

Makes the path absolute without accessing the filesystem, converting it to a [`Utf8PathBuf`](#utf8pathbuf).

If the path is relative, the current directory is used as the base directory. All intermediate
components will be resolved according to platform-specific rules, but unlike
`canonicalize` or [`canonicalize_utf8`](Utf8Path::canonicalize_utf8),
this does not resolve symlinks and may succeed even if the path does not exist.

*Requires Rust 1.79 or newer.*

# Errors

Errors if:

* The path is empty.
* The `current directory` cannot be determined.
* The path is not valid UTF-8.

# Examples

## POSIX paths

```rust
#[cfg(unix)]
fn main() -> std::io::Result<()> {
    use camino::Utf8Path;

    // Relative to absolute
    let absolute = camino::absolute_utf8("foo/./bar")?;
    assert!(absolute.ends_with("foo/bar"));

    // Absolute to absolute
    let absolute = camino::absolute_utf8("/foo//test/.././bar.rs")?;
    assert_eq!(absolute, Utf8Path::new("/foo/test/../bar.rs"));
    Ok(())
}
#[cfg(not(unix))]
fn main() {}
```

The path is resolved using [POSIX semantics][posix-semantics] except that it stops short of
resolving symlinks. This means it will keep `..` components and trailing slashes.

## Windows paths

```rust
#[cfg(windows)]
fn main() -> std::io::Result<()> {
    use camino::Utf8Path;

    // Relative to absolute
    let absolute = camino::absolute_utf8("foo/./bar")?;
    assert!(absolute.ends_with(r"foo\bar"));

    // Absolute to absolute
    let absolute = camino::absolute_utf8(r"C:\foo//test\..\./bar.rs")?;

    assert_eq!(absolute, Utf8Path::new(r"C:\foo\bar.rs"));
    Ok(())
}
#[cfg(not(windows))]
fn main() {}
```

For verbatim paths this will simply return the path as given. For other paths this is currently
equivalent to calling [`GetFullPathNameW`][windows-path].

Note that this [may change in the future][changes].




### `str_assume_utf8`

```rust
unsafe fn str_assume_utf8(string: &std::ffi::OsStr) -> &str
```

*Defined in [`camino-1.2.1/src/lib.rs:3409-3428`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3409-L3428)*

## Macros

### `impl_cmp!`

*Defined in [`camino-1.2.1/src/lib.rs:3132-3166`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3132-L3166)*

### `impl_cmp_std_path!`

*Defined in [`camino-1.2.1/src/lib.rs:3174-3208`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3174-L3208)*

### `impl_cmp_str!`

*Defined in [`camino-1.2.1/src/lib.rs:3223-3257`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3223-L3257)*

### `impl_cmp_os_str!`

*Defined in [`camino-1.2.1/src/lib.rs:3272-3306`](../../.source_1765521767/camino-1.2.1/src/lib.rs#L3272-L3306)*

