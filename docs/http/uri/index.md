*[http](../index.md) / [uri](index.md)*

---

# Module `uri`

URI component of request and response lines

This module primarily contains the `Uri` type which is a component of all
HTTP requests and also reexports this type at the root of the crate. A URI
is not always a "full URL" in the sense of something you'd type into a web
browser, but HTTP requests may only have paths on servers but may have full
schemes and hostnames on clients.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_str(), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

## Structs

### `Authority`

```rust
struct Authority {
    // [REDACTED: Private Fields]
}
```

Represents the authority component of a URI.

#### Implementations

- `const fn from_static(src: &'static str) -> Self`
  Attempt to convert an `Authority` from a static string.

- `fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>`
  Attempt to convert a `Bytes` buffer to a `Authority`.

- `fn host(self: &Self) -> &str`
  Get the host of this `Authority`.

- `fn port(self: &Self) -> Option<Port<&str>>`
  Get the port part of this `Authority`.

- `fn port_u16(self: &Self) -> Option<u16>`
  Get the port of this `Authority` as a `u16`.

- `fn as_str(self: &Self) -> &str`
  Return a str representation of the authority

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = InvalidUri`

- `fn from_str(s: &str) -> Result<Self, InvalidUri>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Authority`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Authority) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &&'a str) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Authority) -> Option<cmp::Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(t: String) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

A builder for `Uri`s.

This type can be used to construct an instance of `Uri`
through a builder pattern.

#### Implementations

- `fn new() -> Builder`
  Creates a new default instance of `Builder` to construct a `Uri`.

- `fn scheme<T>(self: Self, scheme: T) -> Self`
  Set the `Scheme` for this URI.

- `fn authority<T>(self: Self, auth: T) -> Self`
  Set the `Authority` for this URI.

- `fn path_and_query<T>(self: Self, p_and_q: T) -> Self`
  Set the `PathAndQuery` for this URI.

- `fn build(self: Self) -> Result<Uri, crate::Error>`
  Consumes this builder, and tries to construct a valid `Uri` from

#### Trait Implementations

##### `impl From`

- `fn from(uri: Uri) -> Self`

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Builder`

### `PathAndQuery`

```rust
struct PathAndQuery {
    // [REDACTED: Private Fields]
}
```

Represents the path component of a URI

#### Implementations

- `const fn from_static(src: &'static str) -> Self`
  Convert a `PathAndQuery` from a static string.

- `fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>`
  Attempt to convert a `Bytes` buffer to a `PathAndQuery`.

- `fn path(self: &Self) -> &str`
  Returns the path component

- `fn query(self: &Self) -> Option<&str>`
  Returns the query string component

- `fn as_str(self: &Self) -> &str`
  Returns the path and query as a string component.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = InvalidUri`

- `fn from_str(s: &str) -> Result<Self, InvalidUri>`

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

- `fn clone(self: &Self) -> PathAndQuery`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: hash::Hasher>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PathAndQuery) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &PathAndQuery) -> Option<cmp::Ordering>`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &&'a str) -> Option<cmp::Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(s: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(s: &String) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Port<T>`

```rust
struct Port<T> {
    // [REDACTED: Private Fields]
}
```

The port component of a URI.

#### Implementations

- `const fn as_u16(self: &Self) -> u16`
  Returns the port number as a `u16`.

- `fn as_str(self: &Self) -> &str`
  Returns the port number as a `str`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef<T>`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq<T>`

- `fn eq(self: &Self, other: &u16) -> bool`

##### `impl PartialEq<T, U>`

- `fn eq(self: &Self, other: &Port<U>) -> bool`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Scheme`

```rust
struct Scheme {
    // [REDACTED: Private Fields]
}
```

Represents the scheme component of a URI

#### Implementations

- `const HTTP: Scheme`

- `const HTTPS: Scheme`

- `fn as_str(self: &Self) -> &str`
  Return a str representation of the scheme

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = InvalidUri`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Scheme`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Scheme) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Uri`

```rust
struct Uri {
    // [REDACTED: Private Fields]
}
```

The URI component of a request.

For HTTP 1, this is included as part of the request line. From Section 5.3,
Request Target:

> Once an inbound connection is obtained, the client sends an HTTP
> request message (Section 3) with a request-target derived from the
> target URI.  There are four distinct formats for the request-target,
> depending on both the method being requested and whether the request
> is to a proxy.
>
> ```notrust
> request-target = origin-form
>                / absolute-form
>                / authority-form
>                / asterisk-form
> ```

The URI is structured as follows:

```notrust
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
|-|   |-------------------------------||--------| |-------------------| |-----|
 |                  |                       |               |              |
scheme          authority                 path            query         fragment
```

For HTTP 2.0, the URI is encoded using pseudoheaders.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_str(), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

#### Implementations

- `fn builder() -> Builder`
  Creates a new builder-style object to manufacture a `Uri`.

- `fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts>`
  Attempt to convert a `Parts` into a `Uri`.

- `fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>`
  Attempt to convert a `Bytes` buffer to a `Uri`.

- `fn from_static(src: &'static str) -> Self`
  Convert a `Uri` from a static string.

- `fn into_parts(self: Self) -> Parts`
  Convert a `Uri` into `Parts`.

- `fn path_and_query(self: &Self) -> Option<&PathAndQuery>`
  Returns the path & query components of the Uri

- `fn path(self: &Self) -> &str`
  Get the path of this `Uri`.

- `fn scheme(self: &Self) -> Option<&Scheme>`
  Get the scheme of this `Uri`.

- `fn scheme_str(self: &Self) -> Option<&str>`
  Get the scheme of this `Uri` as a `&str`.

- `fn authority(self: &Self) -> Option<&Authority>`
  Get the authority of this `Uri`.

- `fn host(self: &Self) -> Option<&str>`
  Get the host of this `Uri`.

- `fn port(self: &Self) -> Option<Port<&str>>`
  Get the port part of this `Uri`.

- `fn port_u16(self: &Self) -> Option<u16>`
  Get the port of this `Uri` as a `u16`.

- `fn query(self: &Self) -> Option<&str>`
  Get the query string of this `Uri`, starting after the `?`.

#### Trait Implementations

##### `impl From`

- `fn from(path_and_query: PathAndQuery) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(authority: Authority) -> Self`

##### `impl FromStr`

- `type Err = InvalidUri`

- `fn from_str(s: &str) -> Result<Uri, InvalidUri>`

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

- `fn clone(self: &Self) -> Uri`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Uri) -> bool`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(t: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUriParts`

- `fn try_from(src: Parts) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = Error`

- `fn try_from(src: &'a Uri) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Uri`

### `Parts`

```rust
struct Parts {
    pub scheme: Option<Scheme>,
    pub authority: Option<Authority>,
    pub path_and_query: Option<PathAndQuery>,
    // [REDACTED: Private Fields]
}
```

The various parts of a URI.

This struct is used to provide to and retrieve from a URI.

#### Fields

- **`scheme`**: `Option<Scheme>`

  The scheme component of a URI

- **`authority`**: `Option<Authority>`

  The authority component of a URI

- **`path_and_query`**: `Option<PathAndQuery>`

  The origin-form component of a URI

#### Trait Implementations

##### `impl From`

- `fn from(src: Uri) -> Self`

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Parts`

### `InvalidUri`

```rust
struct InvalidUri();
```

An error resulting from a failed attempt to construct a URI.

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

### `InvalidUriParts`

```rust
struct InvalidUriParts();
```

An error resulting from a failed attempt to construct a URI.

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

