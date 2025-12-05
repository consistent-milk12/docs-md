# Crate `ureq_proto`

Supporting crate for [ureq](https://crates.io/crates/ureq).

This crate contains types used to implement ureq.

# In scope:

* First class HTTP/1.1 protocol implementation
* Indication of connection states (such as when a connection must be closed)
* transfer-encoding: chunked
* 100-continue handling

# Out of scope:

* Opening/closing sockets
* TLS (https)
* Request routing
* Body data transformations (charset, compression etc)

# The http crate

Based on the [http crate](https://crates.io/crates/http) - a unified HTTP API for Rust.

## Modules

- [`client`](client/index.md) - HTTP/1.1 client protocol
- [`parser`](parser/index.md) - Low level HTTP parser

## Enums

### `Error`

```rust
enum Error {
    BadHeader(String),
    UnsupportedVersion,
    MethodVersionMismatch(http::Method, http::Version),
    TooManyHostHeaders,
    TooManyContentLengthHeaders,
    BadHostHeader,
    BadAuthorizationHeader,
    BadContentLengthHeader,
    OutputOverflow,
    ChunkLenNotAscii,
    ChunkLenNotANumber,
    ChunkExpectedCrLf,
    BodyContentAfterFinish,
    BodyLargerThanContentLength,
    BodyNotAllowed,
    HttpParseFail(String),
    HttpParseTooManyHeaders,
    NoLocationHeader,
    BadLocationHeader(String),
    HeadersWith100,
    BodyIsChunked,
    BadReject100Status(http::StatusCode),
}
```

Error type for ureq-proto

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: httparse::Error) -> Self`

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

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

##### `impl StructuralPartialEq`

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

### `BodyMode`

```rust
enum BodyMode {
    NoBody,
    LengthDelimited(u64),
    Chunked,
    CloseDelimited,
}
```

Kind of body

#### Variants

- **`NoBody`**

  No body is expected either due to the status or method.

- **`LengthDelimited`**

  Delimited by content-length.
  The value is what's left to receive.

- **`Chunked`**

  Chunked transfer encoding

- **`CloseDelimited`**

  Expect remote to close at end of body.

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

- `fn clone(self: &Self) -> BodyMode`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BodyMode) -> bool`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CloseReason`

```rust
enum CloseReason {
    ClientConnectionClose,
    ServerConnectionClose,
    Not100Continue,
    CloseDelimitedBody,
}
```

Reasons for closing an HTTP connection.

This enum represents the various reasons why an HTTP connection might need
to be closed after a request/response cycle is complete.

#### Variants

- **`ClientConnectionClose`**

  Client sent `connection: close`.

- **`ServerConnectionClose`**

  Server sent `connection: close`.

- **`Not100Continue`**

  When doing expect-100 the server sent _some other response_.
  
  For expect-100, the only options for a server response are:
  
  * 100 continue, in which case we continue to send the body.
  * do nothing, in which case we continue to send the body after a timeout.
  * a 4xx or 5xx response indicating the server cannot receive the body.

- **`CloseDelimitedBody`**

  Response body is close delimited.
  
  We do not know how much body data to receive. The socket will be closed
  when it's done. This is HTTP/1.0 semantics.

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

- `fn clone(self: &Self) -> CloseReason`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CloseReason) -> bool`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

