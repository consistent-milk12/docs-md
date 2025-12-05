*[http](../index.md) / [response](index.md)*

---

# Module `response`

HTTP response types.

This module contains structs related to HTTP responses, notably the
`Response` type itself as well as a builder to create responses. Typically
you'll import the `http::Response` type rather than reaching into this
module itself.

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

## Structs

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

- `fn builder() -> Builder`
  Creates a new builder-style object to manufacture a `Response`

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

##### `impl AsSendBody`

##### `impl ResponseExt`

### `Parts`

```rust
struct Parts {
    pub status: crate::status::StatusCode,
    pub version: crate::version::Version,
    pub headers: crate::header::HeaderMap<crate::header::HeaderValue>,
    pub extensions: crate::Extensions,
    // [REDACTED: Private Fields]
}
```

Component parts of an HTTP `Response`

The HTTP response head consists of a status, version, and a set of
header fields.

#### Fields

- **`status`**: `crate::status::StatusCode`

  The response's status

- **`version`**: `crate::version::Version`

  The response's version

- **`headers`**: `crate::header::HeaderMap<crate::header::HeaderValue>`

  The response's headers

- **`extensions`**: `crate::Extensions`

  The response's extensions

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

An HTTP response builder

This type can be used to construct an instance of `Response` through a
builder-like pattern.

#### Implementations

- `fn new() -> Builder`
  Creates a new default instance of `Builder` to construct either a

- `fn status<T>(self: Self, status: T) -> Builder`
  Set the HTTP status for this response.

- `fn version(self: Self, version: Version) -> Builder`
  Set the HTTP version for this response.

- `fn header<K, V>(self: Self, key: K, value: V) -> Builder`
  Appends a header to this response builder.

- `fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>>`
  Get header on this response builder.

- `fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>>`
  Get header on this response builder.

- `fn extension<T>(self: Self, extension: T) -> Builder`
  Adds an extension to this builder

- `fn extensions_ref(self: &Self) -> Option<&Extensions>`
  Get a reference to the extensions for this response builder.

- `fn extensions_mut(self: &mut Self) -> Option<&mut Extensions>`
  Get a mutable reference to the extensions for this response builder.

- `fn body<T>(self: Self, body: T) -> Result<Response<T>>`
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

