*[http](../index.md) / [request](index.md)*

---

# Module `request`

HTTP request types.

This module contains structs related to HTTP requests, notably the
`Request` type itself as well as a builder to create requests. Typically
you'll import the `http::Request` type rather than reaching into this
module itself.

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

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
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
# panic!()
}
```

## Structs

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

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
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
# panic!()
}
```

Deserialize a request of bytes via json:

```
use http::Request;
use serde::de;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
```

Or alternatively, serialize the body of a request to json

```
use http::Request;
use serde::ser;

fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
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

### `Parts`

```rust
struct Parts {
    pub method: crate::method::Method,
    pub uri: crate::Uri,
    pub version: crate::version::Version,
    pub headers: crate::header::HeaderMap<crate::header::HeaderValue>,
    pub extensions: crate::Extensions,
    // [REDACTED: Private Fields]
}
```

Component parts of an HTTP `Request`

The HTTP request head consists of a method, uri, version, and a set of
header fields.

#### Fields

- **`method`**: `crate::method::Method`

  The request's method

- **`uri`**: `crate::Uri`

  The request's URI

- **`version`**: `crate::version::Version`

  The request's version

- **`headers`**: `crate::header::HeaderMap<crate::header::HeaderValue>`

  The request's headers

- **`extensions`**: `crate::Extensions`

  The request's extensions

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

- `fn clone(self: &Self) -> Parts`

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

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

An HTTP request builder

This type can be used to construct an instance or `Request`
through a builder-like pattern.

#### Implementations

- `fn new() -> Builder`
  Creates a new default instance of `Builder` to construct a `Request`.

- `fn method<T>(self: Self, method: T) -> Builder`
  Set the HTTP method for this request.

- `fn method_ref(self: &Self) -> Option<&Method>`
  Get the HTTP Method for this request.

- `fn uri<T>(self: Self, uri: T) -> Builder`
  Set the URI for this request.

- `fn uri_ref(self: &Self) -> Option<&Uri>`
  Get the URI for this request

- `fn version(self: Self, version: Version) -> Builder`
  Set the HTTP version for this request.

- `fn version_ref(self: &Self) -> Option<&Version>`
  Get the HTTP version for this request

- `fn header<K, V>(self: Self, key: K, value: V) -> Builder`
  Appends a header to this request builder.

- `fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>>`
  Get header on this request builder.

- `fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>>`
  Get headers on this request builder.

- `fn extension<T>(self: Self, extension: T) -> Builder`
  Adds an extension to this builder

- `fn extensions_ref(self: &Self) -> Option<&Extensions>`
  Get a reference to the extensions for this request builder.

- `fn extensions_mut(self: &mut Self) -> Option<&mut Extensions>`
  Get a mutable reference to the extensions for this request builder.

- `fn body<T>(self: Self, body: T) -> Result<Request<T>>`
  "Consumes" this builder, using the provided `body` to return a

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

