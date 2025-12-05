*[http](../index.md) / [version](index.md)*

---

# Module `version`

HTTP version

This module contains a definition of the `Version` type. The `Version`
type is intended to be accessed through the root of the crate
(`http::Version`) rather than this module.

The `Version` type contains constants that represent the various versions
of the HTTP protocol.

# Examples

```rust
use http::Version;

let http11 = Version::HTTP_11;
let http2 = Version::HTTP_2;
assert!(http11 != http2);

println!("{:?}", http2);
```

## Structs

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

