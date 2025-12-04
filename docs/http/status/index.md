*[http](../index.md) / [status](index.md)*

---

# Module `status`

HTTP status codes

This module contains HTTP-status code related structs and errors. The main
type in this module is `StatusCode` which is not intended to be used through
this module but rather the `http::StatusCode` type.

# Examples

```
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND, 404);
assert!(StatusCode::OK.is_success());
```

## Structs

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
significant digit. See [`StatusCode::is_success`](#is-success), etc. Values above 599
are unclassified but allowed for legacy compatibility, though their use is
discouraged. Applications may interpret such values as protocol errors.

# Examples

```
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

- `fn eq(self: &Self, other: &u16) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StatusCode) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StatusCode) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

- `type Error = InvalidStatusCode`

- `fn try_from(t: u16) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidStatusCode`

- `fn try_from(t: &'a [u8]) -> Result<Self, <Self as >::Error>`

##### `impl TryFrom<'a>`

- `type Error = InvalidStatusCode`

- `fn try_from(t: &'a str) -> Result<Self, <Self as >::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> StatusCode`

### `InvalidStatusCode`

```rust
struct InvalidStatusCode {
}
```

A possible error value when converting a `StatusCode` from a `u16` or `&str`.

This error indicates that the supplied input was not a valid number, was less
than 100, or was greater than 999.

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

