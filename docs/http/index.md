# Crate `http`

A general purpose library of common HTTP types

This crate is a general purpose library for common types found when working
with the HTTP protocol. You'll find [`Request`](request/index.md) and [`Response`](response/index.md) types for
working as either a client or a server as well as all of their components.
Notably you'll find `Uri` for what a [`Request`](request/index.md) is requesting, a [`Method`](method/index.md)
for how it's being requested, a [`StatusCode`](status/index.md) for what sort of response came
back, a [`Version`](version/index.md) for how this was communicated, and
[`HeaderName`](#headername)/[`HeaderValue`](#headervalue) definitions to get grouped in a [`HeaderMap`](#headermap) to
work with request/response headers.

You will notably *not* find an implementation of sending requests or
spinning up a server in this crate. It's intended that this crate is the
"standard library" for HTTP clients and servers without dictating any
particular implementation.

## Requests and Responses

Perhaps the main two types in this crate are the [`Request`](request/index.md) and [`Response`](response/index.md)
types. A [`Request`](request/index.md) could either be constructed to get sent off as a client
or it can also be received to generate a [`Response`](response/index.md) for a server. Similarly
as a client a [`Response`](response/index.md) is what you get after sending a [`Request`](request/index.md), whereas
on a server you'll be manufacturing a [`Response`](response/index.md) to send back to the client.

Each type has a number of accessors for the component fields. For as a
server you might want to inspect a requests URI to dispatch it:

```rust
use http::{Request, Response};

fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}
fn index(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
fn foo(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
fn bar(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
fn not_found(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
```

On a [`Request`](request/index.md) you'll also find accessors like `method` to return a
[`Method`](method/index.md) and `headers` to inspect the various headers. A [`Response`](response/index.md)
has similar methods for headers, the status code, etc.

In addition to getters, request/response types also have mutable accessors
to edit the request/response:

```rust
use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;
}
```

And finally, one of the most important aspects of requests/responses, the
body! The [`Request`](request/index.md) and [`Response`](response/index.md) types in this crate are *generic* in
what their body is. This allows downstream libraries to use different
representations such as `Request<Vec<u8>>`, `Response<impl Read>`,
`Request<impl Stream<Item = Vec<u8>, Error = _>>`, or even
`Response<MyCustomType>` where the custom type was deserialized from JSON.

The body representation is intentionally flexible to give downstream
libraries maximal flexibility in implementing the body as appropriate.

## HTTP Headers

Another major piece of functionality in this library is HTTP header
interpretation and generation. The `HeaderName` type serves as a way to
define header *names*, or what's to the left of the colon. A `HeaderValue`
conversely is the header *value*, or what's to the right of a colon.

For example, if you have an HTTP request that looks like:

```http
GET /foo HTTP/1.1
Accept: text/html
```

Then `"Accept"` is a [`HeaderName`](#headername) while `"text/html"` is a [`HeaderValue`](#headervalue).
Each of these is a dedicated type to allow for a number of interesting
optimizations and to also encode the static guarantees of each type. For
example a [`HeaderName`](#headername) is always a valid `&str`, but a [`HeaderValue`](#headervalue) may
not be valid UTF-8.

The most common header names are already defined for you as constant values
in the [`header`](header/index.md) module of this crate. For example:

```rust
use http::header::{self, HeaderName};

let name: HeaderName = header::ACCEPT;
assert_eq!(name.as_str(), "accept");
```

You can, however, also parse header names from strings:

```rust
use http::header::{self, HeaderName};

let name = "Accept".parse::<HeaderName>().unwrap();
assert_eq!(name, header::ACCEPT);
```

Header values can be created from string literals through the `from_static`
function:

```rust
use http::HeaderValue;

let value = HeaderValue::from_static("text/html");
assert_eq!(value.as_bytes(), b"text/html");
```

And header values can also be parsed like names:

```rust
use http::HeaderValue;

let value = "text/html";
let value = value.parse::<HeaderValue>().unwrap();
```

Most HTTP requests and responses tend to come with more than one header, so
it's not too useful to just work with names and values only! This crate also
provides a [`HeaderMap`](#headermap) type which is a specialized hash map for keys as
[`HeaderName`](#headername) and generic values. This type, like header names, is optimized
for common usage but should continue to scale with your needs over time.

# URIs

Each HTTP [`Request`](request/index.md) has an associated URI with it. This may just be a path
like `/index.html` but it could also be an absolute URL such as
`https://www.rust-lang.org/index.html`. A `URI` has a number of accessors to
interpret it:

```rust
use http::Uri;
use http::uri::Scheme;

let uri = "https://www.rust-lang.org/index.html".parse::<Uri>().unwrap();

assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/index.html");
assert_eq!(uri.query(), None);
```

## Modules

- [`header`](header/index.md) - HTTP header types
- [`method`](method/index.md) - The HTTP request method
- [`request`](request/index.md) - HTTP request types.
- [`response`](response/index.md) - HTTP response types.
- [`status`](status/index.md) - HTTP status codes
- [`uri`](uri/index.md) - URI component of request and response lines
- [`version`](version/index.md) - HTTP version

## Structs

### `Error`

```rust
struct Error {
    // [REDACTED: Private Fields]
}
```

A generic "error" for HTTP connections

This error type is less specific than the error returned from other
functions in this crate, but all other errors can be converted to this
error. Consumers of this crate can typically consume and work with this form
of error for conversions with the `?` operator.

#### Implementations

- `fn is<T: error::Error + 'static>(self: &Self) -> bool`
  Return true if the underlying error has the same type as T.

- `fn get_ref(self: &Self) -> &dyn error::Error`
  Return a reference to the lower level, inner error.

#### Trait Implementations

##### `impl From`

- `fn from(err: header::InvalidHeaderValue) -> Error`

##### `impl From`

- `fn from(err: status::InvalidStatusCode) -> Error`

##### `impl From`

- `fn from(err: uri::InvalidUriParts) -> Error`

##### `impl From`

- `fn from(err: header::InvalidHeaderName) -> Error`

##### `impl From`

- `fn from(err: MaxSizeReached) -> Error`

##### `impl From`

- `fn from(err: std::convert::Infallible) -> Error`

##### `impl From`

- `fn from(err: uri::InvalidUri) -> Error`

##### `impl From`

- `fn from(err: method::InvalidMethod) -> Error`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Extensions`

```rust
struct Extensions {
    // [REDACTED: Private Fields]
}
```

A type map of protocol extensions.

`Extensions` can be used by `Request` and `Response` to store
extra data derived from the underlying protocol.

#### Implementations

- `fn new() -> Extensions`
  Create an empty `Extensions`.

- `fn insert<T: Clone + Send + Sync + 'static>(self: &mut Self, val: T) -> Option<T>`
  Insert a type into this `Extensions`.

- `fn get<T: Send + Sync + 'static>(self: &Self) -> Option<&T>`
  Get a reference to a type previously inserted on this `Extensions`.

- `fn get_mut<T: Send + Sync + 'static>(self: &mut Self) -> Option<&mut T>`
  Get a mutable reference to a type previously inserted on this `Extensions`.

- `fn get_or_insert<T: Clone + Send + Sync + 'static>(self: &mut Self, value: T) -> &mut T`
  Get a mutable reference to a type, inserting `value` if not already present on this

- `fn get_or_insert_with<T: Clone + Send + Sync + 'static, F: FnOnce() -> T>(self: &mut Self, f: F) -> &mut T`
  Get a mutable reference to a type, inserting the value created by `f` if not already present

- `fn get_or_insert_default<T: Default + Clone + Send + Sync + 'static>(self: &mut Self) -> &mut T`
  Get a mutable reference to a type, inserting the type's default value if not already present

- `fn remove<T: Send + Sync + 'static>(self: &mut Self) -> Option<T>`
  Remove a type from this `Extensions`.

- `fn clear(self: &mut Self)`
  Clear the `Extensions` of all inserted extensions.

- `fn is_empty(self: &Self) -> bool`
  Check whether the extension set is empty or not.

- `fn len(self: &Self) -> usize`
  Get the number of extensions available.

- `fn extend(self: &mut Self, other: Self)`
  Extends `self` with another `Extensions`.

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

- `fn clone(self: &Self) -> Extensions`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Default`

- `fn default() -> Extensions`

### `HeaderMap<T>`

```rust
struct HeaderMap<T> {
    // [REDACTED: Private Fields]
}
```

A specialized [multimap](<https://en.wikipedia.org/wiki/Multimap>) for
header names and values.

# Overview

`HeaderMap` is designed specifically for efficient manipulation of HTTP
headers. It supports multiple values per header name and provides
specialized APIs for insertion, retrieval, and iteration.

The internal implementation is optimized for common usage patterns in HTTP,
and may change across versions. For example, the current implementation uses
[Robin Hood
hashing](<https://en.wikipedia.org/wiki/Hash_table#Robin_Hood_hashing>) to
store entries compactly and enable high load factors with good performance.
However, the collision resolution strategy and storage mechanism are not
part of the public API and may be altered in future releases.

# Iteration order

Unless otherwise specified, the order in which items are returned by
iterators from `HeaderMap` methods is arbitrary; there is no guaranteed
ordering among the elements yielded by such an iterator. Changes to the
iteration order are not considered breaking changes, so users must not rely
on any incidental order produced by such an iterator. However, for a given
crate version, the iteration order will be consistent across all platforms.

# Adaptive hashing

`HeaderMap` uses an adaptive strategy for hashing to maintain fast lookups
while resisting hash collision attacks. The default hash function
prioritizes performance. In scenarios where high collision rates are
detected—typically indicative of denial-of-service attacks—the
implementation switches to a more secure, collision-resistant hash function.

# Limitations

A `HeaderMap` can store at most 32,768 entries \(header name/value pairs\).
Attempting to exceed this limit will result in a panic.


# Examples

Basic usage

```rust
use http::HeaderMap;
use http::header::{CONTENT_LENGTH, HOST, LOCATION};
let mut headers = HeaderMap::new();

headers.insert(HOST, "example.com".parse().unwrap());
headers.insert(CONTENT_LENGTH, "123".parse().unwrap());

assert!(headers.contains_key(HOST));
assert!(!headers.contains_key(LOCATION));

assert_eq!(headers[HOST], "example.com");

headers.remove(HOST);

assert!(!headers.contains_key(HOST));
```

#### Implementations

- `fn with_capacity(capacity: usize) -> HeaderMap<T>`
  Create an empty `HeaderMap` with the specified capacity.

- `fn try_with_capacity(capacity: usize) -> Result<HeaderMap<T>, MaxSizeReached>`
  Create an empty `HeaderMap` with the specified capacity.

- `fn len(self: &Self) -> usize`
  Returns the number of headers stored in the map.

- `fn keys_len(self: &Self) -> usize`
  Returns the number of keys stored in the map.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the map contains no elements.

- `fn clear(self: &mut Self)`
  Clears the map, removing all key-value pairs. Keeps the allocated memory

- `fn capacity(self: &Self) -> usize`
  Returns the number of headers the map can hold without reallocating.

- `fn reserve(self: &mut Self, additional: usize)`
  Reserves capacity for at least `additional` more headers to be inserted

- `fn try_reserve(self: &mut Self, additional: usize) -> Result<(), MaxSizeReached>`
  Reserves capacity for at least `additional` more headers to be inserted

- `fn get<K>(self: &Self, key: K) -> Option<&T>`
  Returns a reference to the value associated with the key.

- `fn get_mut<K>(self: &mut Self, key: K) -> Option<&mut T>`
  Returns a mutable reference to the value associated with the key.

- `fn get_all<K>(self: &Self, key: K) -> GetAll<'_, T>`
  Returns a view of all values associated with a key.

- `fn contains_key<K>(self: &Self, key: K) -> bool`
  Returns true if the map contains a value for the specified key.

- `fn iter(self: &Self) -> Iter<'_, T>`
  An iterator visiting all key-value pairs.

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>`
  An iterator visiting all key-value pairs, with mutable value references.

- `fn keys(self: &Self) -> Keys<'_, T>`
  An iterator visiting all keys.

- `fn values(self: &Self) -> Values<'_, T>`
  An iterator visiting all values.

- `fn values_mut(self: &mut Self) -> ValuesMut<'_, T>`
  An iterator visiting all values mutably.

- `fn drain(self: &mut Self) -> Drain<'_, T>`
  Clears the map, returning all entries as an iterator.

- `fn entry<K>(self: &mut Self, key: K) -> Entry<'_, T>`
  Gets the given key's corresponding entry in the map for in-place

- `fn try_entry<K>(self: &mut Self, key: K) -> Result<Entry<'_, T>, InvalidHeaderName>`
  Gets the given key's corresponding entry in the map for in-place

- `fn insert<K>(self: &mut Self, key: K, val: T) -> Option<T>`
  Inserts a key-value pair into the map.

- `fn try_insert<K>(self: &mut Self, key: K, val: T) -> Result<Option<T>, MaxSizeReached>`
  Inserts a key-value pair into the map.

- `fn append<K>(self: &mut Self, key: K, value: T) -> bool`
  Inserts a key-value pair into the map.

- `fn try_append<K>(self: &mut Self, key: K, value: T) -> Result<bool, MaxSizeReached>`
  Inserts a key-value pair into the map.

- `fn remove<K>(self: &mut Self, key: K) -> Option<T>`
  Removes a key from the map, returning the value associated with the key.

- `fn new() -> Self`
  Create an empty `HeaderMap`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromIterator<T>`

- `fn from_iter<I>(iter: I) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<T>`

- `type Item = (Option<HeaderName>, T)`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> IntoIter<T>`
  Creates a consuming iterator, that is, one that moves keys and values

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> HeaderMap<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<T: Eq>`

##### `impl Extend<T>`

- `fn extend<I: IntoIterator<Item = (Option<HeaderName>, T)>>(self: &mut Self, iter: I)`
  Extend a `HeaderMap` with the contents of another `HeaderMap`.

##### `impl Extend<T>`

- `fn extend<I: IntoIterator<Item = (HeaderName, T)>>(self: &mut Self, iter: I)`

##### `impl Index<K, T>`

- `type Output = T`

- `fn index(self: &Self, index: K) -> &T`
  # Panics

##### `impl PartialEq<T: PartialEq>`

- `fn eq(self: &Self, other: &HeaderMap<T>) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<'a, K, V, S, T>`

- `type Error = Error`

- `fn try_from(c: &'a HashMap<K, V, S>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

### `HeaderName`

```rust
struct HeaderName {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP header field name

Header field names identify the header. Header sets may include multiple
headers with the same name. The HTTP specification defines a number of
standard headers, but HTTP messages may include non-standard header names as
well as long as they adhere to the specification.

`HeaderName` is used as the [`HeaderMap`](#headermap) key. Constants are available for
all standard header names in the [`header`](header/index.md) module.

# Representation

`HeaderName` represents standard header names using an `enum`, as such they
will not require an allocation for storage. All custom header names are
lower cased upon conversion to a `HeaderName` value. This avoids the
overhead of dynamically doing lower case conversion during the hash code
computation and the comparison operation.



#### Implementations

- `fn from_bytes(src: &[u8]) -> Result<HeaderName, InvalidHeaderName>`
  Converts a slice of bytes to an HTTP header name.

- `fn from_lowercase(src: &[u8]) -> Result<HeaderName, InvalidHeaderName>`
  Converts a slice of bytes to an HTTP header name.

- `const fn from_static(src: &'static str) -> HeaderName`
  Converts a static string to a HTTP header name.

- `fn as_str(self: &Self) -> &str`
  Returns a `str` representation of the header.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(src: &'a HeaderName) -> HeaderName`

##### `impl FromStr`

- `type Err = InvalidHeaderName`

- `fn from_str(s: &str) -> Result<HeaderName, InvalidHeaderName>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoHeaderName`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsHeaderName`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> HeaderName`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`
  Performs a case-insensitive comparison of the string against the header

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a HeaderName) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HeaderName) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`
  Performs a case-insensitive comparison of the string against the header

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom`

- `type Error = InvalidHeaderName`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderName`

- `fn try_from(s: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderName`

- `fn try_from(s: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HeaderValue`

```rust
struct HeaderValue {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP header field value.

In practice, HTTP header field values are usually valid ASCII. However, the
HTTP spec allows for a header value to contain opaque bytes as well. In this
case, the header field value is not able to be represented as a string.

To handle this, the `HeaderValue` is usable as a type and can be compared
with strings and implements `Debug`. A `to_str` fn is provided that returns
an `Err` if the header value contains non visible ascii characters.

#### Implementations

- `const fn from_static(src: &'static str) -> HeaderValue`
  Convert a static string to a `HeaderValue`.

- `fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a string to a `HeaderValue`.

- `fn from_name(name: HeaderName) -> HeaderValue`
  Converts a HeaderName into a HeaderValue

- `fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a byte slice to a `HeaderValue`.

- `fn from_maybe_shared<T>(src: T) -> Result<HeaderValue, InvalidHeaderValue>`
  Attempt to convert a `Bytes` buffer to a `HeaderValue`.

- `unsafe fn from_maybe_shared_unchecked<T>(src: T) -> HeaderValue`
  Convert a `Bytes` directly into a `HeaderValue` without validating.

- `fn to_str(self: &Self) -> Result<&str, ToStrError>`
  Yields a `&str` slice if the `HeaderValue` only contains visible ASCII

- `fn len(self: &Self) -> usize`
  Returns the length of `self`.

- `fn is_empty(self: &Self) -> bool`
  Returns true if the `HeaderValue` has a length of zero bytes.

- `fn as_bytes(self: &Self) -> &[u8]`
  Converts a `HeaderValue` to a byte slice.

- `fn set_sensitive(self: &mut Self, val: bool)`
  Mark that the header value represents sensitive information.

- `fn is_sensitive(self: &Self) -> bool`
  Returns `true` if the value represents sensitive data.

#### Trait Implementations

##### `impl From`

- `fn from(num: i32) -> HeaderValue`

##### `impl From`

- `fn from(num: usize) -> HeaderValue`

##### `impl From`

- `fn from(num: u32) -> HeaderValue`

##### `impl From`

- `fn from(num: i64) -> HeaderValue`

##### `impl From`

- `fn from(num: u16) -> HeaderValue`

##### `impl From`

- `fn from(h: HeaderName) -> HeaderValue`

##### `impl From<'a>`

- `fn from(t: &'a HeaderValue) -> Self`

##### `impl From`

- `fn from(num: u64) -> HeaderValue`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(num: i16) -> HeaderValue`

##### `impl From`

- `fn from(num: isize) -> HeaderValue`

##### `impl FromStr`

- `type Err = InvalidHeaderValue`

- `fn from_str(s: &str) -> Result<HeaderValue, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> HeaderValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HeaderValue) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &[u8]) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq<'a, T: ?Sized>`

- `fn eq(self: &Self, other: &&'a T) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering>`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`

##### `impl PartialOrd<'a, T: ?Sized>`

- `fn partial_cmp(self: &Self, other: &&'a T) -> Option<cmp::Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderValue`

- `fn try_from(t: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidHeaderValue`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidHeaderValue`

- `fn try_from(s: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Method`

```rust
struct Method();
```

The Request Method (VERB)

This type also contains constants for a number of common HTTP methods such
as GET, POST, etc.

Currently includes 8 variants representing the 8 methods defined in
[RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH,
and an Extension variant for all extensions.

# Examples

```rust
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

#### Implementations

- `const GET: Method`

- `const POST: Method`

- `const PUT: Method`

- `const DELETE: Method`

- `const HEAD: Method`

- `const OPTIONS: Method`

- `const CONNECT: Method`

- `const PATCH: Method`

- `const TRACE: Method`

- `fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod>`
  Converts a slice of bytes to an HTTP method.

- `fn is_safe(self: &Self) -> bool`
  Whether a method is considered "safe", meaning the request is

- `fn is_idempotent(self: &Self) -> bool`
  Whether a method is considered "idempotent", meaning the request has

- `fn as_str(self: &Self) -> &str`
  Return a &str representation of the HTTP method

#### Trait Implementations

##### `impl From<'a>`

- `fn from(t: &'a Method) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = InvalidMethod`

- `fn from_str(t: &str) -> Result<Self, <Self as >::Err>`

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

- `fn clone(self: &Self) -> Method`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a Method) -> bool`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Method) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<'a>`

- `type Error = InvalidMethod`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidMethod`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Method`

### `Request<T>`

```rust
struct Request<T> {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP request.

An HTTP request consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

# Examples

Creating a `Request` to send

```no_run
use http::{Request, Response};

let mut request = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request = request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

fn needs_awesome_header() -> bool {
    true
}

fn send(req: Request<()>) -> Response<()> {
    // ...
panic!()
}
```

Inspecting a request to see what was sent.

```rust
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
panic!()
}
```

Deserialize a request of bytes via json:

```rust
use http::Request;
use serde::de;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}

fn main() {}
```

Or alternatively, serialize the body of a request to json

```rust
use http::Request;
use serde::ser;

fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}

fn main() {}
```

#### Implementations

- `fn builder() -> Builder`
  Creates a new builder-style object to manufacture a `Request`

- `fn get<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a GET method and the given URI.

- `fn put<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a PUT method and the given URI.

- `fn post<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a POST method and the given URI.

- `fn delete<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a DELETE method and the given URI.

- `fn options<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with an OPTIONS method and the given URI.

- `fn head<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a HEAD method and the given URI.

- `fn connect<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a CONNECT method and the given URI.

- `fn patch<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a PATCH method and the given URI.

- `fn trace<T>(uri: T) -> Builder`
  Creates a new `Builder` initialized with a TRACE method and the given URI.

- `fn new(body: T) -> Request<T>`
  Creates a new blank `Request` with the body

- `fn from_parts(parts: Parts, body: T) -> Request<T>`
  Creates a new `Request` with the given components parts and body.

- `fn method(self: &Self) -> &Method`
  Returns a reference to the associated HTTP method.

- `fn method_mut(self: &mut Self) -> &mut Method`
  Returns a mutable reference to the associated HTTP method.

- `fn uri(self: &Self) -> &Uri`
  Returns a reference to the associated URI.

- `fn uri_mut(self: &mut Self) -> &mut Uri`
  Returns a mutable reference to the associated URI.

- `fn version(self: &Self) -> Version`
  Returns the associated version.

- `fn version_mut(self: &mut Self) -> &mut Version`
  Returns a mutable reference to the associated version.

- `fn headers(self: &Self) -> &HeaderMap<HeaderValue>`
  Returns a reference to the associated header field map.

- `fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue>`
  Returns a mutable reference to the associated header field map.

- `fn extensions(self: &Self) -> &Extensions`
  Returns a reference to the associated extensions.

- `fn extensions_mut(self: &mut Self) -> &mut Extensions`
  Returns a mutable reference to the associated extensions.

- `fn body(self: &Self) -> &T`
  Returns a reference to the associated HTTP body.

- `fn body_mut(self: &mut Self) -> &mut T`
  Returns a mutable reference to the associated HTTP body.

- `fn into_body(self: Self) -> T`
  Consumes the request, returning just the body.

- `fn into_parts(self: Self) -> (Parts, T)`
  Consumes the request returning the head and body parts.

- `fn map<F, U>(self: Self, f: F) -> Request<U>`
  Consumes the request returning a new request with body mapped to the

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

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Request<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> Request<T>`

##### `impl RequestExt<S: AsSendBody>`

### `Response<T>`

```rust
struct Response<T> {
    // [REDACTED: Private Fields]
}
```

Represents an HTTP response

An HTTP response consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

Typically you'll work with responses on the client side as the result of
sending a `Request` and on the server you'll be generating a `Response` to
send back to the client.

# Examples

Creating a `Response` to return

```rust
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}
```

A simple 404 handler

```rust
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}
```

Or otherwise inspecting the result of a request:

```no_run
use http::{Request, Response};

fn get(url: &str) -> http::Result<Response<()>> {
    // ...
panic!()
}

let response = get("https://www.rust-lang.org/").unwrap();

if !response.status().is_success() {
    panic!("failed to get a successful response status!");
}

if let Some(date) = response.headers().get("Date") {
    // we've got a `Date` header!
}

let body = response.body();
// ...
```

Deserialize a response of bytes via json:

```rust
use http::Response;
use serde::de;

fn deserialize<T>(res: Response<Vec<u8>>) -> serde_json::Result<Response<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = res.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Response::from_parts(parts, body))
}

fn main() {}
```

Or alternatively, serialize the body of a response to json

```rust
use http::Response;
use serde::ser;

fn serialize<T>(res: Response<T>) -> serde_json::Result<Response<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = res.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Response::from_parts(parts, body))
}

fn main() {}
```

#### Implementations

- `fn new(body: T) -> Response<T>`
  Creates a new blank `Response` with the body

- `fn from_parts(parts: Parts, body: T) -> Response<T>`
  Creates a new `Response` with the given head and body

- `fn status(self: &Self) -> StatusCode`
  Returns the `StatusCode`.

- `fn status_mut(self: &mut Self) -> &mut StatusCode`
  Returns a mutable reference to the associated `StatusCode`.

- `fn version(self: &Self) -> Version`
  Returns a reference to the associated version.

- `fn version_mut(self: &mut Self) -> &mut Version`
  Returns a mutable reference to the associated version.

- `fn headers(self: &Self) -> &HeaderMap<HeaderValue>`
  Returns a reference to the associated header field map.

- `fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue>`
  Returns a mutable reference to the associated header field map.

- `fn extensions(self: &Self) -> &Extensions`
  Returns a reference to the associated extensions.

- `fn extensions_mut(self: &mut Self) -> &mut Extensions`
  Returns a mutable reference to the associated extensions.

- `fn body(self: &Self) -> &T`
  Returns a reference to the associated HTTP body.

- `fn body_mut(self: &mut Self) -> &mut T`
  Returns a mutable reference to the associated HTTP body.

- `fn into_body(self: Self) -> T`
  Consumes the response, returning just the body.

- `fn into_parts(self: Self) -> (Parts, T)`
  Consumes the response returning the head and body parts.

- `fn map<F, U>(self: Self, f: F) -> Response<U>`
  Consumes the response returning a new response with body mapped to the

- `fn builder() -> Builder`
  Creates a new builder-style object to manufacture a `Response`

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

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Response<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> Response<T>`

##### `impl ResponseExt`

##### `impl AsSendBody`

### `StatusCode`

```rust
struct StatusCode();
```

An HTTP status code (`status-code` in RFC 9110 et al.).

Constants are provided for known status codes, including those in the IANA
[HTTP Status Code Registry](
https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml).

Status code values in the range 100-999 (inclusive) are supported by this
type. Values in the range 100-599 are semantically classified by the most
significant digit. See `StatusCode::is_success`, etc. Values above 599
are unclassified but allowed for legacy compatibility, though their use is
discouraged. Applications may interpret such values as protocol errors.

# Examples

```rust
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
assert!(StatusCode::OK.is_success());
```

#### Implementations

- `const CONTINUE: StatusCode`

- `const SWITCHING_PROTOCOLS: StatusCode`

- `const PROCESSING: StatusCode`

- `const EARLY_HINTS: StatusCode`

- `const OK: StatusCode`

- `const CREATED: StatusCode`

- `const ACCEPTED: StatusCode`

- `const NON_AUTHORITATIVE_INFORMATION: StatusCode`

- `const NO_CONTENT: StatusCode`

- `const RESET_CONTENT: StatusCode`

- `const PARTIAL_CONTENT: StatusCode`

- `const MULTI_STATUS: StatusCode`

- `const ALREADY_REPORTED: StatusCode`

- `const IM_USED: StatusCode`

- `const MULTIPLE_CHOICES: StatusCode`

- `const MOVED_PERMANENTLY: StatusCode`

- `const FOUND: StatusCode`

- `const SEE_OTHER: StatusCode`

- `const NOT_MODIFIED: StatusCode`

- `const USE_PROXY: StatusCode`

- `const TEMPORARY_REDIRECT: StatusCode`

- `const PERMANENT_REDIRECT: StatusCode`

- `const BAD_REQUEST: StatusCode`

- `const UNAUTHORIZED: StatusCode`

- `const PAYMENT_REQUIRED: StatusCode`

- `const FORBIDDEN: StatusCode`

- `const NOT_FOUND: StatusCode`

- `const METHOD_NOT_ALLOWED: StatusCode`

- `const NOT_ACCEPTABLE: StatusCode`

- `const PROXY_AUTHENTICATION_REQUIRED: StatusCode`

- `const REQUEST_TIMEOUT: StatusCode`

- `const CONFLICT: StatusCode`

- `const GONE: StatusCode`

- `const LENGTH_REQUIRED: StatusCode`

- `const PRECONDITION_FAILED: StatusCode`

- `const PAYLOAD_TOO_LARGE: StatusCode`

- `const URI_TOO_LONG: StatusCode`

- `const UNSUPPORTED_MEDIA_TYPE: StatusCode`

- `const RANGE_NOT_SATISFIABLE: StatusCode`

- `const EXPECTATION_FAILED: StatusCode`

- `const IM_A_TEAPOT: StatusCode`

- `const MISDIRECTED_REQUEST: StatusCode`

- `const UNPROCESSABLE_ENTITY: StatusCode`

- `const LOCKED: StatusCode`

- `const FAILED_DEPENDENCY: StatusCode`

- `const TOO_EARLY: StatusCode`

- `const UPGRADE_REQUIRED: StatusCode`

- `const PRECONDITION_REQUIRED: StatusCode`

- `const TOO_MANY_REQUESTS: StatusCode`

- `const REQUEST_HEADER_FIELDS_TOO_LARGE: StatusCode`

- `const UNAVAILABLE_FOR_LEGAL_REASONS: StatusCode`

- `const INTERNAL_SERVER_ERROR: StatusCode`

- `const NOT_IMPLEMENTED: StatusCode`

- `const BAD_GATEWAY: StatusCode`

- `const SERVICE_UNAVAILABLE: StatusCode`

- `const GATEWAY_TIMEOUT: StatusCode`

- `const HTTP_VERSION_NOT_SUPPORTED: StatusCode`

- `const VARIANT_ALSO_NEGOTIATES: StatusCode`

- `const INSUFFICIENT_STORAGE: StatusCode`

- `const LOOP_DETECTED: StatusCode`

- `const NOT_EXTENDED: StatusCode`

- `const NETWORK_AUTHENTICATION_REQUIRED: StatusCode`

- `const fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode>`
  Converts a u16 to a status code.

- `fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode>`
  Converts a `&[u8]` to a status code.

- `const fn as_u16(self: &Self) -> u16`
  Returns the `u16` corresponding to this `StatusCode`.

- `fn as_str(self: &Self) -> &str`
  Returns a &str representation of the `StatusCode`

- `fn canonical_reason(self: &Self) -> Option<&'static str>`
  Get the standardised `reason-phrase` for this status code.

- `fn is_informational(self: &Self) -> bool`
  Check if status is within 100-199.

- `fn is_success(self: &Self) -> bool`
  Check if status is within 200-299.

- `fn is_redirection(self: &Self) -> bool`
  Check if status is within 300-399.

- `fn is_client_error(self: &Self) -> bool`
  Check if status is within 400-499.

- `fn is_server_error(self: &Self) -> bool`
  Check if status is within 500-599.

#### Trait Implementations

##### `impl From<'a>`

- `fn from(t: &'a StatusCode) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = InvalidStatusCode`

- `fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode>`

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

- `fn clone(self: &Self) -> StatusCode`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &StatusCode) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StatusCode) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &u16) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StatusCode) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom`

- `type Error = InvalidStatusCode`

- `fn try_from(t: u16) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidStatusCode`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidStatusCode`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> StatusCode`

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

```rust
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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(path_and_query: PathAndQuery) -> Self`

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

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &&'a str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Uri) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(vec: Vec<u8>) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUri`

- `fn try_from(t: String) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = Error`

- `fn try_from(src: &'a Uri) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom`

- `type Error = InvalidUriParts`

- `fn try_from(src: Parts) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidUri`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Uri`

### `Version`

```rust
struct Version();
```

Represents a version of the HTTP spec.

#### Implementations

- `const HTTP_09: Version`

- `const HTTP_10: Version`

- `const HTTP_11: Version`

- `const HTTP_2: Version`

- `const HTTP_3: Version`

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

- `fn clone(self: &Self) -> Version`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Version) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

##### `impl Default`

- `fn default() -> Version`

## Type Aliases

