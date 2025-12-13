# Crate `camino`

UTF-8 encoded paths.

`camino` is an extension of the [`std::path`](../fs_err/path/index.md) module that adds new [`Utf8PathBuf`](#utf8pathbuf) and [`Utf8Path`](#utf8path)
types. These are like the standard library's [`PathBuf`](../clap_builder/index.md) and [`Path`](../rustdoc_types/index.md) types, except they are
guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
contents as strings, they implement [`Display`](../miette_derive/index.md), etc.

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

  Allocates an empty [`Utf8PathBuf`](#utf8pathbuf).

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  

  let path = Utf8PathBuf::new();

  ```

- <span id="utf8pathbuf-from-path-buf"></span>`fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

  Creates a new [`Utf8PathBuf`](#utf8pathbuf) from a [`PathBuf`](../clap_builder/index.md) containing valid UTF-8 characters.

  

  Errors with the original [`PathBuf`](../clap_builder/index.md) if it is not valid UTF-8.

  

  For a version that returns a type that implements [`std::error::Error`](../addr2line/index.md),

  see [`TryFrom<&PathBuf>`][tryfrom].

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  use std::ffi::OsStr;

  #[cfg(unix)]

  use std::os::unix::ffi::OsStrExt;

  use std::path::PathBuf;

  

  let unicode_path = PathBuf::from("/valid/unicode");

  Utf8PathBuf::from_path_buf(unicode_path).expect("valid Unicode path succeeded");

  

  // Paths on Unix can be non-UTF-8.

  #[cfg(unix)]

  let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");

  #[cfg(unix)]

  let non_unicode_path = PathBuf::from(non_unicode_str);

  #[cfg(unix)]

  Utf8PathBuf::from_path_buf(non_unicode_path).expect_err("non-Unicode path failed");

  ```

- <span id="utf8pathbuf-from-os-string"></span>`fn from_os_string(os_string: OsString) -> Result<Utf8PathBuf, OsString>` — [`Utf8PathBuf`](#utf8pathbuf)

  Creates a new [`Utf8PathBuf`](#utf8pathbuf) from an [`OsString`](../clap_builder/index.md) containing valid UTF-8 characters.

  

  Errors with the original [`OsString`](../clap_builder/index.md) if it is not valid UTF-8.

  

  For a version that returns a type that implements [`std::error::Error`](../addr2line/index.md), use the

  `TryFrom<OsString>` impl.

  

  # Examples

  

  ```rust

  #[cfg(osstring_from_str)] {

  use camino::Utf8PathBuf;

  use std::ffi::OsStr;

  use std::ffi::OsString;

  use std::convert::TryFrom;

  use std::str::FromStr;

  #[cfg(unix)]

  use std::os::unix::ffi::OsStrExt;

  

  let unicode_string = OsString::from_str("/valid/unicode").unwrap();

  Utf8PathBuf::from_os_string(unicode_string).expect("valid Unicode path succeeded");

  

  // Paths on Unix can be non-UTF-8.

  #[cfg(unix)]

  let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").into();

  #[cfg(unix)]

  Utf8PathBuf::from_os_string(non_unicode_string).expect_err("non-Unicode path failed");

  }

  ```

- <span id="utf8pathbuf-into-std-path-buf"></span>`fn into_std_path_buf(self) -> PathBuf`

  Converts a [`Utf8PathBuf`](#utf8pathbuf) to a [`PathBuf`](../clap_builder/index.md).

  

  This is equivalent to the [`From<Utf8PathBuf> for PathBuf`][`from`](../serde_json/value/from/index.md) implementation,

  but may aid in type inference.

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  use std::path::PathBuf;

  

  let utf8_path_buf = Utf8PathBuf::from("foo.txt");

  let std_path_buf = utf8_path_buf.into_std_path_buf();

  assert_eq!(std_path_buf.to_str(), Some("foo.txt"));

  

  // Convert back to a Utf8PathBuf.

  let new_utf8_path_buf = Utf8PathBuf::from_path_buf(std_path_buf).unwrap();

  assert_eq!(new_utf8_path_buf, "foo.txt");

  ```

- <span id="utf8pathbuf-with-capacity"></span>`fn with_capacity(capacity: usize) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

  Creates a new [`Utf8PathBuf`](#utf8pathbuf) with a given capacity used to create the internal [`PathBuf`](../clap_builder/index.md).

  See `with_capacity` defined on [`PathBuf`](../clap_builder/index.md).

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  

  let mut path = Utf8PathBuf::with_capacity(10);

  let capacity = path.capacity();

  

  // This push is done without reallocating

  path.push(r"C:\");

  

  assert_eq!(capacity, path.capacity());

  ```

- <span id="utf8pathbuf-as-path"></span>`fn as_path(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

  Coerces to a [`Utf8Path`](#utf8path) slice.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let p = Utf8PathBuf::from("/test");

  assert_eq!(Utf8Path::new("/test"), p.as_path());

  ```

- <span id="utf8pathbuf-leak"></span>`fn leak<'a>(self) -> &'a mut Utf8Path` — [`Utf8Path`](#utf8path)

  Consumes and leaks the [`Utf8PathBuf`](#utf8pathbuf), returning a mutable reference to the contents,

  `&'a mut Utf8Path`.

  

  The caller has free choice over the returned lifetime, including 'static.

  Indeed, this function is ideally used for data that lives for the remainder of

  the program’s life, as dropping the returned reference will cause a memory leak.

  

  It does not reallocate or shrink the [`Utf8PathBuf`](#utf8pathbuf), so the leaked allocation may include

  unused capacity that is not part of the returned slice. If you want to discard excess

  capacity, call `into_boxed_path`, and then `Box::leak` instead.

  However, keep in mind that trimming the capacity may result in a reallocation and copy.

  

  *Requires Rust 1.89 or newer.*

- <span id="utf8pathbuf-push"></span>`fn push(&mut self, path: impl AsRef<Utf8Path>)` — [`Utf8Path`](#utf8path)

  Extends `self` with `path`.

  

  If `path` is absolute, it replaces the current path.

  

  On Windows:

  

  * if `path` has a root but no prefix (e.g., `\windows`), it

    replaces everything except for the prefix (if any) of `self`.

  * if `path` has a prefix but no root, it replaces `self`.

  

  # Examples

  

  Pushing a relative path extends the existing path:

  

  ```rust

  use camino::Utf8PathBuf;

  

  let mut path = Utf8PathBuf::from("/tmp");

  path.push("file.bk");

  assert_eq!(path, Utf8PathBuf::from("/tmp/file.bk"));

  ```

  

  Pushing an absolute path replaces the existing path:

  

  ```rust

  use camino::Utf8PathBuf;

  

  let mut path = Utf8PathBuf::from("/tmp");

  path.push("/etc");

  assert_eq!(path, Utf8PathBuf::from("/etc"));

  ```

- <span id="utf8pathbuf-pop"></span>`fn pop(&mut self) -> bool`

  Truncates `self` to `self.parent`.

  

  Returns `false` and does nothing if `self.parent` is [`None`](#none).

  Otherwise, returns `true`.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let mut p = Utf8PathBuf::from("/spirited/away.rs");

  

  p.pop();

  assert_eq!(Utf8Path::new("/spirited"), p);

  p.pop();

  assert_eq!(Utf8Path::new("/"), p);

  ```

- <span id="utf8pathbuf-set-file-name"></span>`fn set_file_name(&mut self, file_name: impl AsRef<str>)`

  Updates `self.file_name` to `file_name`.

  

  If `self.file_name` was [`None`](#none), this is equivalent to pushing

  `file_name`.

  

  Otherwise it is equivalent to calling `pop` and then pushing

  `file_name`. The new path will be a sibling of the original path.

  (That is, it will have the same parent.)

  

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  

  let mut buf = Utf8PathBuf::from("/");

  assert_eq!(buf.file_name(), None);

  buf.set_file_name("bar");

  assert_eq!(buf, Utf8PathBuf::from("/bar"));

  assert!(buf.file_name().is_some());

  buf.set_file_name("baz.txt");

  assert_eq!(buf, Utf8PathBuf::from("/baz.txt"));

  ```

- <span id="utf8pathbuf-set-extension"></span>`fn set_extension(&mut self, extension: impl AsRef<str>) -> bool`

  Updates `self.extension` to `extension`.

  

  Returns `false` and does nothing if `self.file_name` is [`None`](#none),

  returns `true` and updates the extension otherwise.

  

  If `self.extension` is [`None`](#none), the extension is added; otherwise

  it is replaced.

  

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let mut p = Utf8PathBuf::from("/feel/the");

  

  p.set_extension("force");

  assert_eq!(Utf8Path::new("/feel/the.force"), p.as_path());

  

  p.set_extension("dark_side");

  assert_eq!(Utf8Path::new("/feel/the.dark_side"), p.as_path());

  ```

- <span id="utf8pathbuf-into-string"></span>`fn into_string(self) -> String`

  Consumes the [`Utf8PathBuf`](#utf8pathbuf), yielding its internal [`String`](../cargo_platform/index.md) storage.

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  

  let p = Utf8PathBuf::from("/the/head");

  let s = p.into_string();

  assert_eq!(s, "/the/head");

  ```

- <span id="utf8pathbuf-into-os-string"></span>`fn into_os_string(self) -> OsString`

  Consumes the [`Utf8PathBuf`](#utf8pathbuf), yielding its internal [`OsString`](../clap_builder/index.md) storage.

  

  # Examples

  

  ```rust

  use camino::Utf8PathBuf;

  use std::ffi::OsStr;

  

  let p = Utf8PathBuf::from("/the/head");

  let s = p.into_os_string();

  assert_eq!(s, OsStr::new("/the/head"));

  ```

- <span id="utf8pathbuf-into-boxed-path"></span>`fn into_boxed_path(self) -> Box<Utf8Path>` — [`Utf8Path`](#utf8path)

  Converts this [`Utf8PathBuf`](#utf8pathbuf) into a [boxed](Box) [`Utf8Path`](#utf8path).

- <span id="utf8pathbuf-capacity"></span>`fn capacity(&self) -> usize`

  Invokes `capacity` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

- <span id="utf8pathbuf-clear"></span>`fn clear(&mut self)`

  Invokes `clear` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

- <span id="utf8pathbuf-reserve"></span>`fn reserve(&mut self, additional: usize)`

  Invokes `reserve` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

- <span id="utf8pathbuf-try-reserve"></span>`fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>`

  Invokes `try_reserve` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

  

  *Requires Rust 1.63 or newer.*

- <span id="utf8pathbuf-reserve-exact"></span>`fn reserve_exact(&mut self, additional: usize)`

  Invokes `reserve_exact` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

- <span id="utf8pathbuf-try-reserve-exact"></span>`fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError>`

  Invokes `try_reserve_exact` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

  

  *Requires Rust 1.63 or newer.*

- <span id="utf8pathbuf-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Invokes `shrink_to_fit` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

- <span id="utf8pathbuf-shrink-to"></span>`fn shrink_to(&mut self, min_capacity: usize)`

  Invokes `shrink_to` on the underlying instance of [`PathBuf`](../clap_builder/index.md).

#### Trait Implementations

##### `impl Any for Utf8PathBuf`

- <span id="utf8pathbuf-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Utf8PathBuf`

- <span id="utf8pathbuf-asref-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl<T> Borrow for Utf8PathBuf`

- <span id="utf8pathbuf-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8PathBuf`

- <span id="utf8pathbuf-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8PathBuf`

- <span id="utf8pathbuf-clone"></span>`fn clone(&self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl CloneToUninit for Utf8PathBuf`

- <span id="utf8pathbuf-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8PathBuf`

- <span id="utf8pathbuf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8PathBuf`

- <span id="utf8pathbuf-default"></span>`fn default() -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl Deref for Utf8PathBuf`

- <span id="utf8pathbuf-deref-type-target"></span>`type Target = Utf8Path`

- <span id="utf8pathbuf-deref"></span>`fn deref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl DerefMut for Utf8PathBuf`

- <span id="utf8pathbuf-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Deserialize for crate::Utf8PathBuf`

- <span id="crateutf8pathbuf-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Utf8PathBuf`

##### `impl Display for Utf8PathBuf`

- <span id="utf8pathbuf-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8PathBuf`

##### `impl<P: AsRef<Utf8Path>> Extend for Utf8PathBuf`

- <span id="utf8pathbuf-extend"></span>`fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I)`

##### `impl<T> From for Utf8PathBuf`

- <span id="utf8pathbuf-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<P: AsRef<Utf8Path>> FromIterator for Utf8PathBuf`

- <span id="utf8pathbuf-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl FromStr for Utf8PathBuf`

- <span id="utf8pathbuf-fromstr-type-err"></span>`type Err = Infallible`

- <span id="utf8pathbuf-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Utf8PathBuf`

- <span id="utf8pathbuf-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Utf8PathBuf`

- <span id="utf8pathbuf-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for &'a Utf8PathBuf`

- <span id="a-utf8pathbuf-intoiterator-type-item"></span>`type Item = &'a str`

- <span id="a-utf8pathbuf-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-utf8pathbuf-intoiterator-into-iter"></span>`fn into_iter(self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Ord for Utf8PathBuf`

- <span id="utf8pathbuf-ord-cmp"></span>`fn cmp(&self, other: &Utf8PathBuf) -> Ordering` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl PartialEq for Utf8PathBuf`

- <span id="utf8pathbuf-partialeq-eq"></span>`fn eq(&self, other: &Utf8PathBuf) -> bool` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl PartialOrd for Utf8PathBuf`

- <span id="utf8pathbuf-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8PathBuf) -> Option<Ordering>` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl Receiver for Utf8PathBuf`

- <span id="utf8pathbuf-receiver-type-target"></span>`type Target = T`

##### `impl Serialize for crate::Utf8PathBuf`

- <span id="crateutf8pathbuf-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToOwned for Utf8PathBuf`

- <span id="utf8pathbuf-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8pathbuf-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8pathbuf-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Utf8PathBuf`

- <span id="utf8pathbuf-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Utf8PathBuf`

- <span id="utf8pathbuf-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8pathbuf-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8PathBuf`

- <span id="utf8pathbuf-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8pathbuf-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Directly wraps a string slice as a [`Utf8Path`](#utf8path) slice.

  

  This is a cost-free conversion.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  Utf8Path::new("foo.txt");

  ```

  

  You can create [`Utf8Path`](#utf8path)s from [`String`](../cargo_platform/index.md)s, or even other [`Utf8Path`](#utf8path)s:

  

  ```rust

  use camino::Utf8Path;

  

  let string = String::from("foo.txt");

  let from_string = Utf8Path::new(&string);

  let from_path = Utf8Path::new(&from_string);

  assert_eq!(from_string, from_path);

  ```

- <span id="utf8path-from-path"></span>`fn from_path(path: &Path) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

  Converts a [`Path`](../rustdoc_types/index.md) to a [`Utf8Path`](#utf8path).

  

  Returns [`None`](#none) if the path is not valid UTF-8.

  

  For a version that returns a type that implements [`std::error::Error`](../addr2line/index.md),

  see [`TryFrom<&Path>`][tryfrom].

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  use std::ffi::OsStr;

  #[cfg(unix)]

  use std::os::unix::ffi::OsStrExt;

  use std::path::Path;

  

  let unicode_path = Path::new("/valid/unicode");

  Utf8Path::from_path(unicode_path).expect("valid Unicode path succeeded");

  

  // Paths on Unix can be non-UTF-8.

  #[cfg(unix)]

  let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");

  #[cfg(unix)]

  let non_unicode_path = Path::new(non_unicode_str);

  #[cfg(unix)]

  assert!(Utf8Path::from_path(non_unicode_path).is_none(), "non-Unicode path failed");

  ```

- <span id="utf8path-from-os-str"></span>`fn from_os_str(path: &OsStr) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

  Converts an [`OsStr`](../clap_builder/builder/os_str/index.md) to a [`Utf8Path`](#utf8path).

  

  Returns [`None`](#none) if the path is not valid UTF-8.

  

  For a version that returns a type that implements [`std::error::Error`](../addr2line/index.md), use the

  [`TryFrom<&OsStr>`][tryfrom] impl.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  use std::ffi::OsStr;

  #[cfg(unix)]

  use std::os::unix::ffi::OsStrExt;

  use std::path::Path;

  

  let unicode_string = OsStr::new("/valid/unicode");

  Utf8Path::from_os_str(unicode_string).expect("valid Unicode string succeeded");

  

  // Paths on Unix can be non-UTF-8.

  #[cfg(unix)]

  let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");

  #[cfg(unix)]

  assert!(Utf8Path::from_os_str(non_unicode_str).is_none(), "non-Unicode string failed");

  ```

- <span id="utf8path-as-std-path"></span>`fn as_std_path(&self) -> &Path`

  Converts a [`Utf8Path`](#utf8path) to a [`Path`](../rustdoc_types/index.md).

  

  This is equivalent to the [`AsRef<Path> for Utf8PathBuf`][asref] implementation,

  but may aid in type inference.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  use std::path::Path;

  

  let utf8_path = Utf8Path::new("foo.txt");

  let std_path: &Path = utf8_path.as_std_path();

  assert_eq!(std_path.to_str(), Some("foo.txt"));

  

  // Convert back to a Utf8Path.

  let new_utf8_path = Utf8Path::from_path(std_path).unwrap();

  assert_eq!(new_utf8_path, "foo.txt");

  ```

- <span id="utf8path-as-str"></span>`fn as_str(&self) -> &str`

  Yields the underlying [`str`](../clap_builder/builder/str/index.md) slice.

  

  Unlike `Path::to_str`, this always returns a slice because the contents

  of a [`Utf8Path`](#utf8path) are guaranteed to be valid UTF-8.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let s = Utf8Path::new("foo.txt").as_str();

  assert_eq!(s, "foo.txt");

  ```

- <span id="utf8path-as-os-str"></span>`fn as_os_str(&self) -> &OsStr`

  Yields the underlying [`OsStr`](../clap_builder/builder/os_str/index.md) slice.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let os_str = Utf8Path::new("foo.txt").as_os_str();

  assert_eq!(os_str, std::ffi::OsStr::new("foo.txt"));

  ```

- <span id="utf8path-to-path-buf"></span>`fn to_path_buf(&self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

  Converts a [`Utf8Path`](#utf8path) to an owned [`Utf8PathBuf`](#utf8pathbuf).

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let path_buf = Utf8Path::new("foo.txt").to_path_buf();

  assert_eq!(path_buf, Utf8PathBuf::from("foo.txt"));

  ```

- <span id="utf8path-is-absolute"></span>`fn is_absolute(&self) -> bool`

  Returns `true` if the [`Utf8Path`](#utf8path) is absolute, i.e., if it is independent of

  the current directory.

  

  * On Unix, a path is absolute if it starts with the root, so

    `is_absolute` and `has_root` are equivalent.

  

  * On Windows, a path is absolute if it has a prefix and starts with the

    root: `C:\windows` is absolute, while `C:temp` and `\temp` are not.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert!(!Utf8Path::new("foo.txt").is_absolute());

  ```

- <span id="utf8path-is-relative"></span>`fn is_relative(&self) -> bool`

  Returns `true` if the [`Utf8Path`](#utf8path) is relative, i.e., not absolute.

  

  See `is_absolute`'s documentation for more details.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert!(Utf8Path::new("foo.txt").is_relative());

  ```

- <span id="utf8path-has-root"></span>`fn has_root(&self) -> bool`

  Returns `true` if the [`Utf8Path`](#utf8path) has a root.

  

  * On Unix, a path has a root if it begins with `/`.

  

  * On Windows, a path has a root if it:

      * has no prefix and begins with a separator, e.g., `\windows`

      * has a prefix followed by a separator, e.g., `C:\windows` but not `C:windows`

      * has any non-disk prefix, e.g., `\\server\share`

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert!(Utf8Path::new("/etc/passwd").has_root());

  ```

- <span id="utf8path-parent"></span>`fn parent(&self) -> Option<&Utf8Path>` — [`Utf8Path`](#utf8path)

  Returns the [`Path`](../rustdoc_types/index.md) without its final component, if there is one.

  

  Returns [`None`](#none) if the path terminates in a root or prefix.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/foo/bar");

  let parent = path.parent().unwrap();

  assert_eq!(parent, Utf8Path::new("/foo"));

  

  let grand_parent = parent.parent().unwrap();

  assert_eq!(grand_parent, Utf8Path::new("/"));

  assert_eq!(grand_parent.parent(), None);

  ```

- <span id="utf8path-ancestors"></span>`fn ancestors(&self) -> Utf8Ancestors<'_>` — [`Utf8Ancestors`](#utf8ancestors)

  Produces an iterator over [`Utf8Path`](#utf8path) and its ancestors.

  

  The iterator will yield the [`Utf8Path`](#utf8path) that is returned if the [`parent`](#parent) method is used zero

  or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,

  `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`](#parent) method returns

  [`None`](#none), the iterator will do likewise. The iterator will always yield at least one value,

  namely `&self`.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let mut ancestors = Utf8Path::new("/foo/bar").ancestors();

  assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo/bar")));

  assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo")));

  assert_eq!(ancestors.next(), Some(Utf8Path::new("/")));

  assert_eq!(ancestors.next(), None);

  

  let mut ancestors = Utf8Path::new("../foo/bar").ancestors();

  assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo/bar")));

  assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo")));

  assert_eq!(ancestors.next(), Some(Utf8Path::new("..")));

  assert_eq!(ancestors.next(), Some(Utf8Path::new("")));

  assert_eq!(ancestors.next(), None);

  ```

- <span id="utf8path-file-name"></span>`fn file_name(&self) -> Option<&str>`

  Returns the final component of the [`Utf8Path`](#utf8path), if there is one.

  

  If the path is a normal file, this is the file name. If it's the path of a directory, this

  is the directory name.

  

  Returns [`None`](#none) if the path terminates in `..`.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert_eq!(Some("bin"), Utf8Path::new("/usr/bin/").file_name());

  assert_eq!(Some("foo.txt"), Utf8Path::new("tmp/foo.txt").file_name());

  assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.").file_name());

  assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.//").file_name());

  assert_eq!(None, Utf8Path::new("foo.txt/..").file_name());

  assert_eq!(None, Utf8Path::new("/").file_name());

  ```

- <span id="utf8path-strip-prefix"></span>`fn strip_prefix(&self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError>` — [`Utf8Path`](#utf8path)

  Returns a path that, when joined onto `base`, yields `self`.

  

  # Errors

  

  If `base` is not a prefix of `self` (i.e., `starts_with`

  returns `false`), returns `Err`.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let path = Utf8Path::new("/test/haha/foo.txt");

  

  assert_eq!(path.strip_prefix("/"), Ok(Utf8Path::new("test/haha/foo.txt")));

  assert_eq!(path.strip_prefix("/test"), Ok(Utf8Path::new("haha/foo.txt")));

  assert_eq!(path.strip_prefix("/test/"), Ok(Utf8Path::new("haha/foo.txt")));

  assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Utf8Path::new("")));

  assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Utf8Path::new("")));

  

  assert!(path.strip_prefix("test").is_err());

  assert!(path.strip_prefix("/haha").is_err());

  

  let prefix = Utf8PathBuf::from("/test/");

  assert_eq!(path.strip_prefix(prefix), Ok(Utf8Path::new("haha/foo.txt")));

  ```

- <span id="utf8path-starts-with"></span>`fn starts_with(&self, base: impl AsRef<Path>) -> bool`

  Determines whether `base` is a prefix of `self`.

  

  Only considers whole path components to match.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/etc/passwd");

  

  assert!(path.starts_with("/etc"));

  assert!(path.starts_with("/etc/"));

  assert!(path.starts_with("/etc/passwd"));

  assert!(path.starts_with("/etc/passwd/")); // extra slash is okay

  assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay

  

  assert!(!path.starts_with("/e"));

  assert!(!path.starts_with("/etc/passwd.txt"));

  

  assert!(!Utf8Path::new("/etc/foo.rs").starts_with("/etc/foo"));

  ```

- <span id="utf8path-ends-with"></span>`fn ends_with(&self, base: impl AsRef<Path>) -> bool`

  Determines whether `child` is a suffix of `self`.

  

  Only considers whole path components to match.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/etc/resolv.conf");

  

  assert!(path.ends_with("resolv.conf"));

  assert!(path.ends_with("etc/resolv.conf"));

  assert!(path.ends_with("/etc/resolv.conf"));

  

  assert!(!path.ends_with("/resolv.conf"));

  assert!(!path.ends_with("conf")); // use .extension() instead

  ```

- <span id="utf8path-file-stem"></span>`fn file_stem(&self) -> Option<&str>`

  Extracts the stem (non-extension) portion of `self.file_name`.

  

  The stem is:

  

  * [`None`](#none), if there is no file name;

  * The entire file name if there is no embedded `.`;

  * The entire file name if the file name begins with `.` and has no other `.`s within;

  * Otherwise, the portion of the file name before the final `.`

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert_eq!("foo", Utf8Path::new("foo.rs").file_stem().unwrap());

  assert_eq!("foo.tar", Utf8Path::new("foo.tar.gz").file_stem().unwrap());

  ```

- <span id="utf8path-extension"></span>`fn extension(&self) -> Option<&str>`

  Extracts the extension of `self.file_name`, if possible.

  

  The extension is:

  

  * [`None`](#none), if there is no file name;

  * [`None`](#none), if there is no embedded `.`;

  * [`None`](#none), if the file name begins with `.` and has no other `.`s within;

  * Otherwise, the portion of the file name after the final `.`

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  assert_eq!("rs", Utf8Path::new("foo.rs").extension().unwrap());

  assert_eq!("gz", Utf8Path::new("foo.tar.gz").extension().unwrap());

  ```

- <span id="utf8path-join"></span>`fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf` — [`Utf8Path`](#utf8path), [`Utf8PathBuf`](#utf8pathbuf)

  Creates an owned [`Utf8PathBuf`](#utf8pathbuf) with `path` adjoined to `self`.

  

  See `Utf8PathBuf::push` for more details on what it means to adjoin a path.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  assert_eq!(Utf8Path::new("/etc").join("passwd"), Utf8PathBuf::from("/etc/passwd"));

  ```

- <span id="utf8path-join-os"></span>`fn join_os(&self, path: impl AsRef<Path>) -> PathBuf`

  Creates an owned [`PathBuf`](../clap_builder/index.md) with `path` adjoined to `self`.

  

  See `PathBuf::push` for more details on what it means to adjoin a path.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  use std::path::PathBuf;

  

  assert_eq!(Utf8Path::new("/etc").join_os("passwd"), PathBuf::from("/etc/passwd"));

  ```

- <span id="utf8path-with-file-name"></span>`fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

  Creates an owned [`Utf8PathBuf`](#utf8pathbuf) like `self` but with the given file name.

  

  See `Utf8PathBuf::set_file_name` for more details.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let path = Utf8Path::new("/tmp/foo.txt");

  assert_eq!(path.with_file_name("bar.txt"), Utf8PathBuf::from("/tmp/bar.txt"));

  

  let path = Utf8Path::new("/tmp");

  assert_eq!(path.with_file_name("var"), Utf8PathBuf::from("/var"));

  ```

- <span id="utf8path-with-extension"></span>`fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

  Creates an owned [`Utf8PathBuf`](#utf8pathbuf) like `self` but with the given extension.

  

  See `Utf8PathBuf::set_extension` for more details.

  

  # Examples

  

  ```rust

  use camino::{Utf8Path, Utf8PathBuf};

  

  let path = Utf8Path::new("foo.rs");

  assert_eq!(path.with_extension("txt"), Utf8PathBuf::from("foo.txt"));

  

  let path = Utf8Path::new("foo.tar.gz");

  assert_eq!(path.with_extension(""), Utf8PathBuf::from("foo.tar"));

  assert_eq!(path.with_extension("xz"), Utf8PathBuf::from("foo.tar.xz"));

  assert_eq!(path.with_extension("").with_extension("txt"), Utf8PathBuf::from("foo.txt"));

  ```

- <span id="utf8path-components"></span>`fn components(&self) -> Utf8Components<'_>` — [`Utf8Components`](#utf8components)

  Produces an iterator over the [`Utf8Component`](#utf8component)s of the path.

  

  When parsing the path, there is a small amount of normalization:

  

  * Repeated separators are ignored, so `a/b` and `a//b` both have

    `a` and `b` as components.

  

  * Occurrences of `.` are normalized away, except if they are at the

    beginning of the path. For example, `a/./b`, `a/b/`, `a/b/.` and

    `a/b` all have `a` and `b` as components, but `./a/b` starts with

    an additional [`CurDir`](#curdir) component.

  

  * A trailing slash is normalized away, `/a/b` and `/a/b/` are equivalent.

  

  Note that no other normalization takes place; in particular, `a/c`

  and `a/b/../c` are distinct, to account for the possibility that `b`

  is a symbolic link (so its parent isn't `a`).

  

  # Examples

  

  ```rust

  use camino::{Utf8Component, Utf8Path};

  

  let mut components = Utf8Path::new("/tmp/foo.txt").components();

  

  assert_eq!(components.next(), Some(Utf8Component::RootDir));

  assert_eq!(components.next(), Some(Utf8Component::Normal("tmp")));

  assert_eq!(components.next(), Some(Utf8Component::Normal("foo.txt")));

  assert_eq!(components.next(), None)

  ```

- <span id="utf8path-iter"></span>`fn iter(&self) -> Iter<'_>` — [`Iter`](#iter)

  Produces an iterator over the path's components viewed as [`str`](../clap_builder/builder/str/index.md)

  slices.

  

  For more information about the particulars of how the path is separated

  into components, see `components`.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let mut it = Utf8Path::new("/tmp/foo.txt").iter();

  assert_eq!(it.next(), Some(std::path::MAIN_SEPARATOR.to_string().as_str()));

  assert_eq!(it.next(), Some("tmp"));

  assert_eq!(it.next(), Some("foo.txt"));

  assert_eq!(it.next(), None)

  ```

- <span id="utf8path-metadata"></span>`fn metadata(&self) -> io::Result<fs::Metadata>`

  Queries the file system to get information about a file, directory, etc.

  

  This function will traverse symbolic links to query information about the

  destination file.

  

  This is an alias to [`fs::metadata`](../tracing_core/metadata/index.md).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/Minas/tirith");

  let metadata = path.metadata().expect("metadata call failed");

  println!("{:?}", metadata.file_type());

  ```

- <span id="utf8path-symlink-metadata"></span>`fn symlink_metadata(&self) -> io::Result<fs::Metadata>`

  Queries the metadata about a file without following symlinks.

  

  This is an alias to [`fs::symlink_metadata`](../fs_err/index.md).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/Minas/tirith");

  let metadata = path.symlink_metadata().expect("symlink_metadata call failed");

  println!("{:?}", metadata.file_type());

  ```

- <span id="utf8path-canonicalize"></span>`fn canonicalize(&self) -> io::Result<PathBuf>`

  Returns the canonical, absolute form of the path with all intermediate

  components normalized and symbolic links resolved.

  

  This returns a [`PathBuf`](../clap_builder/index.md) because even if a symlink is valid Unicode, its target may not

  be. For a version that returns a [`Utf8PathBuf`](#utf8pathbuf), see

  [`canonicalize_utf8`](Self::canonicalize_utf8).

  

  This is an alias to [`fs::canonicalize`](../fs_err/index.md).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  use std::path::PathBuf;

  

  let path = Utf8Path::new("/foo/test/../test/bar.rs");

  assert_eq!(path.canonicalize().unwrap(), PathBuf::from("/foo/test/bar.rs"));

  ```

- <span id="utf8path-canonicalize-utf8"></span>`fn canonicalize_utf8(&self) -> io::Result<Utf8PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

  Returns the canonical, absolute form of the path with all intermediate

  components normalized and symbolic links resolved.

  

  This method attempts to convert the resulting [`PathBuf`](../clap_builder/index.md) into a [`Utf8PathBuf`](#utf8pathbuf). For a

  version that does not attempt to do this conversion, see

  [`canonicalize`](Self::canonicalize).

  

  # Errors

  

  The I/O operation may return an error: see the [`fs::canonicalize`](../fs_err/index.md)

  documentation for more.

  

  If the resulting path is not UTF-8, an [`io::Error`](../addr2line/index.md) is returned with the

  [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)

  and the payload set to a [`FromPathBufError`](#frompathbuferror).

  

  # Examples

  

  ```no_run

  use camino::{Utf8Path, Utf8PathBuf};

  

  let path = Utf8Path::new("/foo/test/../test/bar.rs");

  assert_eq!(path.canonicalize_utf8().unwrap(), Utf8PathBuf::from("/foo/test/bar.rs"));

  ```

- <span id="utf8path-read-link"></span>`fn read_link(&self) -> io::Result<PathBuf>`

  Reads a symbolic link, returning the file that the link points to.

  

  This returns a [`PathBuf`](../clap_builder/index.md) because even if a symlink is valid Unicode, its target may not

  be. For a version that returns a [`Utf8PathBuf`](#utf8pathbuf), see

  [`read_link_utf8`](Self::read_link_utf8).

  

  This is an alias to [`fs::read_link`](../fs_err/index.md).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/laputa/sky_castle.rs");

  let path_link = path.read_link().expect("read_link call failed");

  ```

- <span id="utf8path-read-link-utf8"></span>`fn read_link_utf8(&self) -> io::Result<Utf8PathBuf>` — [`Utf8PathBuf`](#utf8pathbuf)

  Reads a symbolic link, returning the file that the link points to.

  

  This method attempts to convert the resulting [`PathBuf`](../clap_builder/index.md) into a [`Utf8PathBuf`](#utf8pathbuf). For a

  version that does not attempt to do this conversion, see [`read_link`](Self::read_link).

  

  # Errors

  

  The I/O operation may return an error: see the [`fs::read_link`](../fs_err/index.md)

  documentation for more.

  

  If the resulting path is not UTF-8, an [`io::Error`](../addr2line/index.md) is returned with the

  [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)

  and the payload set to a [`FromPathBufError`](#frompathbuferror).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/laputa/sky_castle.rs");

  let path_link = path.read_link_utf8().expect("read_link call failed");

  ```

- <span id="utf8path-read-dir"></span>`fn read_dir(&self) -> io::Result<fs::ReadDir>`

  Returns an iterator over the entries within a directory.

  

  The iterator will yield instances of [`io::Result`](../cargo_metadata/errors/index.md)`<`[`fs::DirEntry`](../fs_err/index.md)`>`. New

  errors may be encountered after an iterator is initially constructed.

  

  This is an alias to [`fs::read_dir`](../fs_err/index.md).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/laputa");

  for entry in path.read_dir().expect("read_dir call failed") {

      if let Ok(entry) = entry {

          println!("{:?}", entry.path());

      }

  }

  ```

- <span id="utf8path-read-dir-utf8"></span>`fn read_dir_utf8(&self) -> io::Result<ReadDirUtf8>` — [`ReadDirUtf8`](#readdirutf8)

  Returns an iterator over the entries within a directory.

  

  The iterator will yield instances of [`io::Result`](../cargo_metadata/errors/index.md)`<`[`Utf8DirEntry`](#utf8direntry)`>`. New

  errors may be encountered after an iterator is initially constructed.

  

  # Errors

  

  The I/O operation may return an error: see the [`fs::read_dir`](../fs_err/index.md)

  documentation for more.

  

  If a directory entry is not UTF-8, an [`io::Error`](../addr2line/index.md) is returned with the

  [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)

  and the payload set to a [`FromPathBufError`](#frompathbuferror).

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  let path = Utf8Path::new("/laputa");

  for entry in path.read_dir_utf8().expect("read_dir call failed") {

      if let Ok(entry) = entry {

          println!("{}", entry.path());

      }

  }

  ```

- <span id="utf8path-exists"></span>`fn exists(&self) -> bool`

  Returns `true` if the path points at an existing entity.

  

  Warning: this method may be error-prone, consider using `try_exists()` instead!

  It also has a risk of introducing time-of-check to time-of-use (TOCTOU) bugs.

  

  This function will traverse symbolic links to query information about the

  destination file. In case of broken symbolic links this will return `false`.

  

  If you cannot access the directory containing the file, e.g., because of a

  permission error, this will return `false`.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  assert!(!Utf8Path::new("does_not_exist.txt").exists());

  ```

  

  # See Also

  

  This is a convenience function that coerces errors to false. If you want to

  check errors, call [`fs::metadata`](../tracing_core/metadata/index.md).

- <span id="utf8path-try-exists"></span>`fn try_exists(&self) -> io::Result<bool>`

  Returns `Ok(true)` if the path points at an existing entity.

  

  This function will traverse symbolic links to query information about the

  destination file. In case of broken symbolic links this will return `Ok(false)`.

  

  As opposed to the `exists()` method, this one doesn't silently ignore errors

  unrelated to the path not existing. (E.g. it will return `Err` in case of permission

  denied on some of the parent directories.)

  

  Note that while this avoids some pitfalls of the `exists()` method, it still can not

  prevent time-of-check to time-of-use (TOCTOU) bugs. You should only use it in scenarios

  where those bugs are not an issue.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  assert!(

      !Utf8Path::new("does_not_exist.txt")

          .try_exists()

          .expect("Can't check existence of file does_not_exist.txt")

  );

  assert!(Utf8Path::new("/root/secret_file.txt").try_exists().is_err());

  ```

- <span id="utf8path-is-file"></span>`fn is_file(&self) -> bool`

  Returns `true` if the path exists on disk and is pointing at a regular file.

  

  This function will traverse symbolic links to query information about the

  destination file. In case of broken symbolic links this will return `false`.

  

  If you cannot access the directory containing the file, e.g., because of a

  permission error, this will return `false`.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  assert_eq!(Utf8Path::new("./is_a_directory/").is_file(), false);

  assert_eq!(Utf8Path::new("a_file.txt").is_file(), true);

  ```

  

  # See Also

  

  This is a convenience function that coerces errors to false. If you want to

  check errors, call [`fs::metadata`](../tracing_core/metadata/index.md) and handle its [`Result`](../cargo_metadata/errors/index.md). Then call

  `fs::Metadata::is_file` if it was `Ok`.

  

  When the goal is simply to read from (or write to) the source, the most

  reliable way to test the source can be read (or written to) is to open

  it. Only using `is_file` can break workflows like `diff <( prog_a )` on

  a Unix-like system for example. See `fs::File::open` or

  `fs::OpenOptions::open` for more information.

- <span id="utf8path-is-dir"></span>`fn is_dir(&self) -> bool`

  Returns `true` if the path exists on disk and is pointing at a directory.

  

  This function will traverse symbolic links to query information about the

  destination file. In case of broken symbolic links this will return `false`.

  

  If you cannot access the directory containing the file, e.g., because of a

  permission error, this will return `false`.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  assert_eq!(Utf8Path::new("./is_a_directory/").is_dir(), true);

  assert_eq!(Utf8Path::new("a_file.txt").is_dir(), false);

  ```

  

  # See Also

  

  This is a convenience function that coerces errors to false. If you want to

  check errors, call [`fs::metadata`](../tracing_core/metadata/index.md) and handle its [`Result`](../cargo_metadata/errors/index.md). Then call

  `fs::Metadata::is_dir` if it was `Ok`.

- <span id="utf8path-is-symlink"></span>`fn is_symlink(&self) -> bool`

  Returns `true` if the path exists on disk and is pointing at a symbolic link.

  

  This function will not traverse symbolic links.

  In case of a broken symbolic link this will also return true.

  

  If you cannot access the directory containing the file, e.g., because of a

  permission error, this will return false.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  use std::os::unix::fs::symlink;

  

  let link_path = Utf8Path::new("link");

  symlink("/origin_does_not_exist/", link_path).unwrap();

  assert_eq!(link_path.is_symlink(), true);

  assert_eq!(link_path.exists(), false);

  ```

  

  # See Also

  

  This is a convenience function that coerces errors to false. If you want to

  check errors, call `Utf8Path::symlink_metadata` and handle its [`Result`](../cargo_metadata/errors/index.md). Then call

  `fs::Metadata::is_symlink` if it was `Ok`.

- <span id="utf8path-into-path-buf"></span>`fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf` — [`Utf8Path`](#utf8path), [`Utf8PathBuf`](#utf8pathbuf)

  Converts a [`Box<Utf8Path>`](../allocator_api2/stable/boxed/index.md) into a [`Utf8PathBuf`](#utf8pathbuf) without copying or allocating.

- <span id="utf8path-assume-utf8"></span>`unsafe fn assume_utf8(path: &Path) -> &Utf8Path` — [`Utf8Path`](#utf8path)

- <span id="utf8path-assume-utf8-mut"></span>`unsafe fn assume_utf8_mut(path: &mut Path) -> &mut Utf8Path` — [`Utf8Path`](#utf8path)

#### Trait Implementations

##### `impl Any for Utf8Path`

- <span id="utf8path-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Utf8Components<'_>`

- <span id="utf8components-asref-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl<T> Borrow for Utf8Path`

- <span id="utf8path-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Path`

- <span id="utf8path-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Box<Utf8Path>`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Utf8Path`

- <span id="utf8path-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for &'a crate::Utf8Path`

- <span id="a-crateutf8path-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl Display for Utf8Path`

- <span id="utf8path-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Path`

##### `impl From for &'a Utf8Path`

- <span id="a-utf8path-from"></span>`fn from(s: &'a str) -> &'a Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl Hash for Utf8Path`

- <span id="utf8path-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IntoIterator for &'a Utf8Path`

- <span id="a-utf8path-intoiterator-type-item"></span>`type Item = &'a str`

- <span id="a-utf8path-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a>`

- <span id="a-utf8path-intoiterator-into-iter"></span>`fn into_iter(self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl Ord for Utf8Path`

- <span id="utf8path-ord-cmp"></span>`fn cmp(&self, other: &Utf8Path) -> Ordering` — [`Utf8Path`](#utf8path)

##### `impl PartialEq for Utf8Path`

- <span id="utf8path-partialeq-eq"></span>`fn eq(&self, other: &Utf8Path) -> bool` — [`Utf8Path`](#utf8path)

##### `impl PartialOrd for Utf8Path`

- <span id="utf8path-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Path) -> Option<Ordering>` — [`Utf8Path`](#utf8path)

##### `impl Serialize for crate::Utf8Path`

- <span id="crateutf8path-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl ToOwned for Utf8Path`

- <span id="utf8path-toowned-type-owned"></span>`type Owned = Utf8PathBuf`

- <span id="utf8path-toowned-to-owned"></span>`fn to_owned(&self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

##### `impl ToString for Utf8Path`

- <span id="utf8path-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TryFrom for &'a Utf8Path`

- <span id="a-utf8path-tryfrom-type-error"></span>`type Error = FromPathError`

- <span id="a-utf8path-tryfrom-try-from"></span>`fn try_from(path: &'a Path) -> Result<&'a Utf8Path, <Self as >::Error>` — [`Utf8Path`](#utf8path)

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

##### `impl Any for Utf8Ancestors<'a>`

- <span id="utf8ancestors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Ancestors<'a>`

- <span id="utf8ancestors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Ancestors<'a>`

- <span id="utf8ancestors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Ancestors<'a>`

- <span id="utf8ancestors-clone"></span>`fn clone(&self) -> Utf8Ancestors<'a>` — [`Utf8Ancestors`](#utf8ancestors)

##### `impl CloneToUninit for Utf8Ancestors<'a>`

- <span id="utf8ancestors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8Ancestors<'a>`

##### `impl Debug for Utf8Ancestors<'_>`

- <span id="utf8ancestors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8Ancestors<'a>`

- <span id="utf8ancestors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Utf8Ancestors<'_>`

##### `impl<U> Into for Utf8Ancestors<'a>`

- <span id="utf8ancestors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Utf8Ancestors<'a>`

- <span id="utf8ancestors-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="utf8ancestors-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="utf8ancestors-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Utf8Ancestors<'a>`

- <span id="utf8ancestors-iterator-type-item"></span>`type Item = &'a Utf8Path`

- <span id="utf8ancestors-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for Utf8Ancestors<'a>`

- <span id="utf8ancestors-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8ancestors-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8ancestors-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Ancestors<'a>`

- <span id="utf8ancestors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8ancestors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Ancestors<'a>`

- <span id="utf8ancestors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8ancestors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Extracts a slice corresponding to the portion of the path remaining for iteration.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let mut components = Utf8Path::new("/tmp/foo/bar.txt").components();

  components.next();

  components.next();

  

  assert_eq!(Utf8Path::new("foo/bar.txt"), components.as_path());

  ```

#### Trait Implementations

##### `impl Any for Utf8Components<'a>`

- <span id="utf8components-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Utf8Components<'_>`

- <span id="utf8components-asref-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl<T> Borrow for Utf8Components<'a>`

- <span id="utf8components-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Components<'a>`

- <span id="utf8components-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Components<'a>`

- <span id="utf8components-clone"></span>`fn clone(&self) -> Utf8Components<'a>` — [`Utf8Components`](#utf8components)

##### `impl CloneToUninit for Utf8Components<'a>`

- <span id="utf8components-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8Components<'_>`

- <span id="utf8components-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Utf8Components<'_>`

- <span id="utf8components-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl Eq for Utf8Components<'a>`

##### `impl<T> From for Utf8Components<'a>`

- <span id="utf8components-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Utf8Components<'_>`

##### `impl<U> Into for Utf8Components<'a>`

- <span id="utf8components-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Utf8Components<'a>`

- <span id="utf8components-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="utf8components-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="utf8components-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Utf8Components<'a>`

- <span id="utf8components-iterator-type-item"></span>`type Item = Utf8Component<'a>`

- <span id="utf8components-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Ord for Utf8Components<'a>`

- <span id="utf8components-ord-cmp"></span>`fn cmp(&self, other: &Utf8Components<'a>) -> cmp::Ordering` — [`Utf8Components`](#utf8components)

##### `impl PartialEq for Utf8Components<'a>`

- <span id="utf8components-partialeq-eq"></span>`fn eq(&self, other: &Utf8Components<'a>) -> bool` — [`Utf8Components`](#utf8components)

##### `impl PartialOrd for Utf8Components<'a>`

- <span id="utf8components-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Components<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Components`](#utf8components)

##### `impl StructuralPartialEq for Utf8Components<'a>`

##### `impl ToOwned for Utf8Components<'a>`

- <span id="utf8components-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8components-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8components-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Components<'a>`

- <span id="utf8components-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8components-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Components<'a>`

- <span id="utf8components-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8components-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Extracts a slice corresponding to the portion of the path remaining for iteration.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let mut iter = Utf8Path::new("/tmp/foo/bar.txt").iter();

  iter.next();

  iter.next();

  

  assert_eq!(Utf8Path::new("foo/bar.txt"), iter.as_path());

  ```

#### Trait Implementations

##### `impl Any for Iter<'a>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Iter<'_>`

- <span id="iter-asref-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl<T> Borrow for Iter<'a>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Iter<'a>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a>` — [`Iter`](#iter)

##### `impl CloneToUninit for Iter<'a>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Iter<'_>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Iter<'a>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<&'a str>`

##### `impl<T> From for Iter<'a>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Iter<'_>`

##### `impl<U> Into for Iter<'a>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter<'a>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a str`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<&'a str>`

##### `impl ToOwned for Iter<'a>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iter<'a>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Returns the parsed prefix data.

  

  See [`Utf8Prefix`](#utf8prefix)'s documentation for more information on the different

  kinds of prefixes.

- <span id="utf8prefixcomponent-as-str"></span>`fn as_str(&self) -> &'a str`

  Returns the [`str`](../clap_builder/builder/str/index.md) slice for this prefix.

- <span id="utf8prefixcomponent-as-os-str"></span>`fn as_os_str(&self) -> &'a OsStr`

  Returns the raw [`OsStr`](../clap_builder/builder/os_str/index.md) slice for this prefix.

#### Trait Implementations

##### `impl Any for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-clone"></span>`fn clone(&self) -> Utf8PrefixComponent<'a>` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl CloneToUninit for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8PrefixComponent<'a>`

##### `impl Debug for Utf8PrefixComponent<'_>`

- <span id="utf8prefixcomponent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8PrefixComponent<'_>`

- <span id="utf8prefixcomponent-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8PrefixComponent<'a>`

##### `impl<T> From for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-ord-cmp"></span>`fn cmp(&self, other: &Utf8PrefixComponent<'a>) -> cmp::Ordering` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl PartialEq for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-partialeq-eq"></span>`fn eq(&self, other: &Utf8PrefixComponent<'a>) -> bool` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl PartialOrd for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8PrefixComponent<'a>) -> option::Option<cmp::Ordering>` — [`Utf8PrefixComponent`](#utf8prefixcomponent)

##### `impl StructuralPartialEq for Utf8PrefixComponent<'a>`

##### `impl ToOwned for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8prefixcomponent-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8prefixcomponent-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8prefixcomponent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8PrefixComponent<'a>`

- <span id="utf8prefixcomponent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8prefixcomponent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ReadDirUtf8`

- <span id="readdirutf8-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReadDirUtf8`

- <span id="readdirutf8-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReadDirUtf8`

- <span id="readdirutf8-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReadDirUtf8`

- <span id="readdirutf8-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReadDirUtf8`

- <span id="readdirutf8-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReadDirUtf8`

- <span id="readdirutf8-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ReadDirUtf8`

- <span id="readdirutf8-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="readdirutf8-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="readdirutf8-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ReadDirUtf8`

- <span id="readdirutf8-iterator-type-item"></span>`type Item = Result<Utf8DirEntry, Error>`

- <span id="readdirutf8-iterator-next"></span>`fn next(&mut self) -> Option<io::Result<Utf8DirEntry>>` — [`Utf8DirEntry`](#utf8direntry)

##### `impl<U> TryFrom for ReadDirUtf8`

- <span id="readdirutf8-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="readdirutf8-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReadDirUtf8`

- <span id="readdirutf8-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="readdirutf8-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Returns the full path to the file that this entry represents.

  

  The full path is created by joining the original path to `read_dir`

  with the filename of this entry.

  

  # Examples

  

  ```no_run

  use camino::Utf8Path;

  

  fn main() -> std::io::Result<()> {

      for entry in Utf8Path::new(".").read_dir_utf8()? {

          let dir = entry?;

          println!("{}", dir.path());

      }

      Ok(())

  }

  ```

  

  This prints output like:

  

  ```text

  ./whatever.txt

  ./foo.html

  ./hello_world.rs

  ```

  

  The exact text, of course, depends on what files you have in `.`.

- <span id="utf8direntry-metadata"></span>`fn metadata(&self) -> io::Result<Metadata>`

  Returns the metadata for the file that this entry points at.

  

  This function will not traverse symlinks if this entry points at a symlink. To traverse

  symlinks use `Utf8Path::metadata` or `fs::File::metadata`.

  

  # Platform-specific behavior

  

  On Windows this function is cheap to call (no extra system calls

  needed), but on Unix platforms this function is the equivalent of

  calling `symlink_metadata` on the path.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {

      for entry in entries {

          if let Ok(entry) = entry {

              // Here, `entry` is a `Utf8DirEntry`.

              if let Ok(metadata) = entry.metadata() {

                  // Now let's show our entry's permissions!

                  println!("{}: {:?}", entry.path(), metadata.permissions());

              } else {

                  println!("Couldn't get metadata for {}", entry.path());

              }

          }

      }

  }

  ```

- <span id="utf8direntry-file-type"></span>`fn file_type(&self) -> io::Result<fs::FileType>`

  Returns the file type for the file that this entry points at.

  

  This function will not traverse symlinks if this entry points at a

  symlink.

  

  # Platform-specific behavior

  

  On Windows and most Unix platforms this function is free (no extra

  system calls needed), but some Unix platforms may require the equivalent

  call to `symlink_metadata` to learn about the target file type.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {

      for entry in entries {

          if let Ok(entry) = entry {

              // Here, `entry` is a `DirEntry`.

              if let Ok(file_type) = entry.file_type() {

                  // Now let's show our entry's file type!

                  println!("{}: {:?}", entry.path(), file_type);

              } else {

                  println!("Couldn't get file type for {}", entry.path());

              }

          }

      }

  }

  ```

- <span id="utf8direntry-file-name"></span>`fn file_name(&self) -> &str`

  Returns the bare file name of this directory entry without any other

  leading path component.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {

      for entry in entries {

          if let Ok(entry) = entry {

              // Here, `entry` is a `DirEntry`.

              println!("{}", entry.file_name());

          }

      }

  }

  ```

- <span id="utf8direntry-into-inner"></span>`fn into_inner(self) -> fs::DirEntry`

  Returns the original [`fs::DirEntry`](../fs_err/index.md) within this [`Utf8DirEntry`](#utf8direntry).

- <span id="utf8direntry-into-path"></span>`fn into_path(self) -> Utf8PathBuf` — [`Utf8PathBuf`](#utf8pathbuf)

  Returns the full path to the file that this entry represents.

  

  This is analogous to [`path`](#path), but moves ownership of the path.

#### Trait Implementations

##### `impl Any for Utf8DirEntry`

- <span id="utf8direntry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8DirEntry`

- <span id="utf8direntry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8DirEntry`

- <span id="utf8direntry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Utf8DirEntry`

- <span id="utf8direntry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8DirEntry`

- <span id="utf8direntry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8DirEntry`

- <span id="utf8direntry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Utf8DirEntry`

- <span id="utf8direntry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8direntry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8DirEntry`

- <span id="utf8direntry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8direntry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Returns the [`Path`](../rustdoc_types/index.md) slice that was attempted to be converted to [`Utf8PathBuf`](#utf8pathbuf).

- <span id="frompathbuferror-into-path-buf"></span>`fn into_path_buf(self) -> PathBuf`

  Returns the [`PathBuf`](../clap_builder/index.md) that was attempted to be converted to [`Utf8PathBuf`](#utf8pathbuf).

- <span id="frompathbuferror-from-path-error"></span>`fn from_path_error(&self) -> FromPathError` — [`FromPathError`](#frompatherror)

  Fetches a [`FromPathError`](#frompatherror) for more about the conversion failure.

  

  At the moment this struct does not contain any additional information, but is provided for

  completeness.

- <span id="frompathbuferror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

  Converts self into a [`std::io::Error`](../addr2line/index.md) with kind

  [`InvalidData`](io::ErrorKind::InvalidData).

  

  Many users of [`FromPathBufError`](#frompathbuferror) will want to convert it into an [`io::Error`](../addr2line/index.md). This is a

  convenience method to do that.

#### Trait Implementations

##### `impl Any for FromPathBufError`

- <span id="frompathbuferror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FromPathBufError`

- <span id="frompathbuferror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FromPathBufError`

- <span id="frompathbuferror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FromPathBufError`

- <span id="frompathbuferror-clone"></span>`fn clone(&self) -> FromPathBufError` — [`FromPathBufError`](#frompathbuferror)

##### `impl CloneToUninit for FromPathBufError`

- <span id="frompathbuferror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FromPathBufError`

- <span id="frompathbuferror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromPathBufError`

- <span id="frompathbuferror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromPathBufError`

##### `impl Error for FromPathBufError`

- <span id="frompathbuferror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for FromPathBufError`

- <span id="frompathbuferror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FromPathBufError`

- <span id="frompathbuferror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FromPathBufError`

- <span id="frompathbuferror-partialeq-eq"></span>`fn eq(&self, other: &FromPathBufError) -> bool` — [`FromPathBufError`](#frompathbuferror)

##### `impl StructuralPartialEq for FromPathBufError`

##### `impl ToOwned for FromPathBufError`

- <span id="frompathbuferror-toowned-type-owned"></span>`type Owned = T`

- <span id="frompathbuferror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="frompathbuferror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FromPathBufError`

- <span id="frompathbuferror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FromPathBufError`

- <span id="frompathbuferror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frompathbuferror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FromPathBufError`

- <span id="frompathbuferror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frompathbuferror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Converts self into a [`std::io::Error`](../addr2line/index.md) with kind

  [`InvalidData`](io::ErrorKind::InvalidData).

  

  Many users of [`FromPathError`](#frompatherror) will want to convert it into an [`io::Error`](../addr2line/index.md). This is a

  convenience method to do that.

#### Trait Implementations

##### `impl Any for FromPathError`

- <span id="frompatherror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FromPathError`

- <span id="frompatherror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FromPathError`

- <span id="frompatherror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FromPathError`

- <span id="frompatherror-clone"></span>`fn clone(&self) -> FromPathError` — [`FromPathError`](#frompatherror)

##### `impl CloneToUninit for FromPathError`

- <span id="frompatherror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FromPathError`

##### `impl Debug for FromPathError`

- <span id="frompatherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromPathError`

- <span id="frompatherror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromPathError`

##### `impl Error for FromPathError`

- <span id="frompatherror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for FromPathError`

- <span id="frompatherror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FromPathError`

- <span id="frompatherror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FromPathError`

- <span id="frompatherror-partialeq-eq"></span>`fn eq(&self, other: &FromPathError) -> bool` — [`FromPathError`](#frompatherror)

##### `impl StructuralPartialEq for FromPathError`

##### `impl ToOwned for FromPathError`

- <span id="frompatherror-toowned-type-owned"></span>`type Owned = T`

- <span id="frompatherror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="frompatherror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FromPathError`

- <span id="frompatherror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FromPathError`

- <span id="frompatherror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frompatherror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FromPathError`

- <span id="frompatherror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frompatherror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Returns the [`OsStr`](../clap_builder/builder/os_str/index.md) slice that was attempted to be converted to [`Utf8PathBuf`](#utf8pathbuf).

- <span id="fromosstringerror-into-os-string"></span>`fn into_os_string(self) -> OsString`

  Returns the [`OsString`](../clap_builder/index.md) that was attempted to be converted to [`Utf8PathBuf`](#utf8pathbuf).

- <span id="fromosstringerror-from-os-str-error"></span>`fn from_os_str_error(&self) -> FromOsStrError` — [`FromOsStrError`](#fromosstrerror)

  Fetches a [`FromOsStrError`](#fromosstrerror) for more about the conversion failure.

  

  At the moment this struct does not contain any additional information, but is provided for

  completeness.

- <span id="fromosstringerror-into-io-error"></span>`fn into_io_error(self) -> io::Error`

  Converts self into a [`std::io::Error`](../addr2line/index.md) with kind

  [`InvalidData`](io::ErrorKind::InvalidData).

  

  Many users of [`FromOsStringError`](#fromosstringerror) will want to convert it into an [`io::Error`](../addr2line/index.md).

  This is a convenience method to do that.

#### Trait Implementations

##### `impl Any for FromOsStringError`

- <span id="fromosstringerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FromOsStringError`

- <span id="fromosstringerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FromOsStringError`

- <span id="fromosstringerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FromOsStringError`

- <span id="fromosstringerror-clone"></span>`fn clone(&self) -> FromOsStringError` — [`FromOsStringError`](#fromosstringerror)

##### `impl CloneToUninit for FromOsStringError`

- <span id="fromosstringerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FromOsStringError`

- <span id="fromosstringerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromOsStringError`

- <span id="fromosstringerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromOsStringError`

##### `impl Error for FromOsStringError`

- <span id="fromosstringerror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for FromOsStringError`

- <span id="fromosstringerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FromOsStringError`

- <span id="fromosstringerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FromOsStringError`

- <span id="fromosstringerror-partialeq-eq"></span>`fn eq(&self, other: &FromOsStringError) -> bool` — [`FromOsStringError`](#fromosstringerror)

##### `impl StructuralPartialEq for FromOsStringError`

##### `impl ToOwned for FromOsStringError`

- <span id="fromosstringerror-toowned-type-owned"></span>`type Owned = T`

- <span id="fromosstringerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fromosstringerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FromOsStringError`

- <span id="fromosstringerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FromOsStringError`

- <span id="fromosstringerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fromosstringerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FromOsStringError`

- <span id="fromosstringerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fromosstringerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Converts self into a [`std::io::Error`](../addr2line/index.md) with kind

  [`InvalidData`](io::ErrorKind::InvalidData).

  

  Many users of [`FromOsStrError`](#fromosstrerror) will want to convert it into an [`io::Error`](../addr2line/index.md). This is a

  convenience method to do that.

#### Trait Implementations

##### `impl Any for FromOsStrError`

- <span id="fromosstrerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FromOsStrError`

- <span id="fromosstrerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FromOsStrError`

- <span id="fromosstrerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FromOsStrError`

- <span id="fromosstrerror-clone"></span>`fn clone(&self) -> FromOsStrError` — [`FromOsStrError`](#fromosstrerror)

##### `impl CloneToUninit for FromOsStrError`

- <span id="fromosstrerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FromOsStrError`

##### `impl Debug for FromOsStrError`

- <span id="fromosstrerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromOsStrError`

- <span id="fromosstrerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FromOsStrError`

##### `impl Error for FromOsStrError`

- <span id="fromosstrerror-error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl<T> From for FromOsStrError`

- <span id="fromosstrerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FromOsStrError`

- <span id="fromosstrerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for FromOsStrError`

- <span id="fromosstrerror-partialeq-eq"></span>`fn eq(&self, other: &FromOsStrError) -> bool` — [`FromOsStrError`](#fromosstrerror)

##### `impl StructuralPartialEq for FromOsStrError`

##### `impl ToOwned for FromOsStrError`

- <span id="fromosstrerror-toowned-type-owned"></span>`type Owned = T`

- <span id="fromosstrerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fromosstrerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FromOsStrError`

- <span id="fromosstrerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FromOsStrError`

- <span id="fromosstrerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fromosstrerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FromOsStrError`

- <span id="fromosstrerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fromosstrerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Extracts the underlying [`str`](../clap_builder/builder/str/index.md) slice.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let path = Utf8Path::new("./tmp/foo/bar.txt");

  let components: Vec<_> = path.components().map(|comp| comp.as_str()).collect();

  assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);

  ```

- <span id="utf8component-as-os-str"></span>`fn as_os_str(&self) -> &'a OsStr`

  Extracts the underlying [`OsStr`](../clap_builder/builder/os_str/index.md) slice.

  

  # Examples

  

  ```rust

  use camino::Utf8Path;

  

  let path = Utf8Path::new("./tmp/foo/bar.txt");

  let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();

  assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);

  ```

#### Trait Implementations

##### `impl Any for Utf8Component<'a>`

- <span id="utf8component-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Utf8Component<'_>`

- <span id="utf8component-asref-as-ref"></span>`fn as_ref(&self) -> &Utf8Path` — [`Utf8Path`](#utf8path)

##### `impl<T> Borrow for Utf8Component<'a>`

- <span id="utf8component-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Component<'a>`

- <span id="utf8component-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Component<'a>`

- <span id="utf8component-clone"></span>`fn clone(&self) -> Utf8Component<'a>` — [`Utf8Component`](#utf8component)

##### `impl CloneToUninit for Utf8Component<'a>`

- <span id="utf8component-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8Component<'a>`

##### `impl Debug for Utf8Component<'_>`

- <span id="utf8component-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf8Component<'_>`

- <span id="utf8component-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Component<'a>`

##### `impl<T> From for Utf8Component<'a>`

- <span id="utf8component-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Utf8Component<'a>`

- <span id="utf8component-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Utf8Component<'a>`

- <span id="utf8component-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Utf8Component<'a>`

- <span id="utf8component-ord-cmp"></span>`fn cmp(&self, other: &Utf8Component<'a>) -> cmp::Ordering` — [`Utf8Component`](#utf8component)

##### `impl PartialEq for Utf8Component<'a>`

- <span id="utf8component-partialeq-eq"></span>`fn eq(&self, other: &Utf8Component<'a>) -> bool` — [`Utf8Component`](#utf8component)

##### `impl PartialOrd for Utf8Component<'a>`

- <span id="utf8component-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Component<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Component`](#utf8component)

##### `impl StructuralPartialEq for Utf8Component<'a>`

##### `impl ToOwned for Utf8Component<'a>`

- <span id="utf8component-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8component-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8component-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Utf8Component<'a>`

- <span id="utf8component-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Utf8Component<'a>`

- <span id="utf8component-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8component-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Component<'a>`

- <span id="utf8component-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8component-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Determines if the prefix is verbatim, i.e., begins with `\\?\`.

  

  # Examples

  

  ```rust

  use camino::Utf8Prefix::*;

  

  assert!(Verbatim("pictures").is_verbatim());

  assert!(VerbatimUNC("server", "share").is_verbatim());

  assert!(VerbatimDisk(b'C').is_verbatim());

  assert!(!DeviceNS("BrainInterface").is_verbatim());

  assert!(!UNC("server", "share").is_verbatim());

  assert!(!Disk(b'C').is_verbatim());

  ```

#### Trait Implementations

##### `impl Any for Utf8Prefix<'a>`

- <span id="utf8prefix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Prefix<'a>`

- <span id="utf8prefix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Prefix<'a>`

- <span id="utf8prefix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Prefix<'a>`

- <span id="utf8prefix-clone"></span>`fn clone(&self) -> Utf8Prefix<'a>` — [`Utf8Prefix`](#utf8prefix)

##### `impl CloneToUninit for Utf8Prefix<'a>`

- <span id="utf8prefix-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8Prefix<'a>`

##### `impl Debug for Utf8Prefix<'a>`

- <span id="utf8prefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Prefix<'a>`

##### `impl<T> From for Utf8Prefix<'a>`

- <span id="utf8prefix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Utf8Prefix<'a>`

- <span id="utf8prefix-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Utf8Prefix<'a>`

- <span id="utf8prefix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Utf8Prefix<'a>`

- <span id="utf8prefix-ord-cmp"></span>`fn cmp(&self, other: &Utf8Prefix<'a>) -> cmp::Ordering` — [`Utf8Prefix`](#utf8prefix)

##### `impl PartialEq for Utf8Prefix<'a>`

- <span id="utf8prefix-partialeq-eq"></span>`fn eq(&self, other: &Utf8Prefix<'a>) -> bool` — [`Utf8Prefix`](#utf8prefix)

##### `impl PartialOrd for Utf8Prefix<'a>`

- <span id="utf8prefix-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Prefix<'a>) -> option::Option<cmp::Ordering>` — [`Utf8Prefix`](#utf8prefix)

##### `impl StructuralPartialEq for Utf8Prefix<'a>`

##### `impl ToOwned for Utf8Prefix<'a>`

- <span id="utf8prefix-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8prefix-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8prefix-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Prefix<'a>`

- <span id="utf8prefix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8prefix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Prefix<'a>`

- <span id="utf8prefix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8prefix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

