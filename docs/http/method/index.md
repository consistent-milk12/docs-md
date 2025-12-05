*[http](../index.md) / [method](index.md)*

---

# Module `method`

The HTTP request method

This module contains HTTP-method related structs and errors and such. The
main type of this module, `Method`, is also reexported at the root of the
crate as `http::Method` and is intended for import through that location
primarily.

# Examples

```rust
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

## Structs

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

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Method) -> bool`

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

### `InvalidMethod`

```rust
struct InvalidMethod {
    // [REDACTED: Private Fields]
}
```

A possible error value when converting `Method` from bytes.

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

