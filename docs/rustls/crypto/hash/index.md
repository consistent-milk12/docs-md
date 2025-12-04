*[rustls](../../index.md) / [crypto](../index.md) / [hash](index.md)*

---

# Module `hash`

Hashing interfaces.

## Structs

### `Output`

```rust
struct Output {
    // [REDACTED: Private Fields]
}
```

A hash output, stored as a value.

#### Implementations

- `fn new(bytes: &[u8]) -> Self`
  Build a `hash::Output` from a slice of no more than `Output::MAX_LEN` bytes.

- `const MAX_LEN: usize`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `HashAlgorithm`

```rust
enum HashAlgorithm {
    NONE,
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    Unknown(u8),
}
```

The `HashAlgorithm` TLS protocol enum.  Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA.
The `Unknown` item is used when processing unrecognised ordinals.

#### Implementations

- `fn to_array(self: Self) -> [u8; 1]`

- `fn as_str(self: &Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(x: u8) -> Self`

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

- `fn clone(self: &Self) -> HashAlgorithm`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Codec`

- `fn encode(self: &Self, bytes: &mut alloc::vec::Vec<u8>)`

- `fn read(r: &mut Reader<'_>) -> Result<Self, crate::error::InvalidMessage>`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &HashAlgorithm) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Traits

### `Hash`

```rust
trait Hash: Send + Sync { ... }
```

Describes a single cryptographic hash function.

This interface can do both one-shot and incremental hashing, using
`Hash::hash()` and `Hash::start()` respectively.

#### Required Methods

- `fn start(self: &Self) -> Box<dyn Context>`

  Start an incremental hash computation.

- `fn hash(self: &Self, data: &[u8]) -> Output`

  Return the output of this hash function with input `data`.

- `fn output_len(self: &Self) -> usize`

  The length in bytes of this hash function's output.

- `fn algorithm(self: &Self) -> HashAlgorithm`

  Which hash function this is, eg, `HashAlgorithm::SHA256`.

- `fn fips(self: &Self) -> bool`

  Return `true` if this is backed by a FIPS-approved implementation.

### `Context`

```rust
trait Context: Send + Sync { ... }
```

How to incrementally compute a hash.

#### Required Methods

- `fn fork_finish(self: &Self) -> Output`

  Finish the computation, returning the resulting output.

- `fn fork(self: &Self) -> Box<dyn Context>`

  Fork the computation, producing another context that has the

- `fn finish(self: Box<Self>) -> Output`

  Terminate and finish the computation, returning the resulting output.

- `fn update(self: &mut Self, data: &[u8])`

  Add `data` to computation.

