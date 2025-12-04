*[ring](../index.md) / [digest](index.md)*

---

# Module `digest`

SHA-2 and the legacy SHA-1 digest algorithm.

If all the data is available in a single contiguous slice then the `digest`
function should be used. Otherwise, the digest can be calculated in
multiple steps using `Context`.

## Structs

### `Context`

```rust
struct Context {
}
```

A context for multi-step (Init-Update-Finish) digest calculations.

# Examples

```
use ring::digest;

let one_shot = digest::digest(&digest::SHA384, b"hello, world");

let mut ctx = digest::Context::new(&digest::SHA384);
ctx.update(b"hello");
ctx.update(b", ");
ctx.update(b"world");
let multi_part = ctx.finish();

assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());
```

#### Implementations

- `fn new(algorithm: &'static Algorithm) -> Self`
  Constructs a new context.

- `fn update(self: &mut Self, data: &[u8])`
  Updates the digest with all the data in `data`.

- `fn finish(self: Self) -> Digest`
  Finalizes the digest calculation and returns the digest value.

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The algorithm that this context is using.

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

- `fn clone(self: &Self) -> Context`

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

### `Digest`

```rust
struct Digest {
}
```

A calculated digest value.

Use [`Self::as_ref`](#as-ref) to get the value as a `&[u8](#u8)`.

#### Implementations

- `fn algorithm(self: &Self) -> &'static Algorithm`
  The algorithm that was used to calculate the digest value.

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

##### `impl Clone`

- `fn clone(self: &Self) -> Digest`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

- `fn fmt(self: &Self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Algorithm`

```rust
struct Algorithm {
}
```

A digest algorithm.

#### Implementations

- `fn block_len(self: &Self) -> usize`
  The internal block length.

- `fn chaining_len(self: &Self) -> usize`
  The size of the chaining value of the digest function, in bytes.

- `fn output_len(self: &Self) -> usize`
  The length of a finalized digest.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

## Functions

### `digest`

```rust
fn digest(algorithm: &'static Algorithm, data: &[u8]) -> Digest
```

Returns the digest of `data` using the given digest algorithm.

## Constants

### `MAX_BLOCK_LEN`

```rust
const MAX_BLOCK_LEN: usize = 128usize;
```

The maximum block length ([`Algorithm::block_len()`](#block-len)) of all the algorithms
in this module.

### `MAX_OUTPUT_LEN`

```rust
const MAX_OUTPUT_LEN: usize = 64usize;
```

The maximum output length ([`Algorithm::output_len()`](#output-len)) of all the
algorithms in this module.

### `MAX_CHAINING_LEN`

```rust
const MAX_CHAINING_LEN: usize = 64usize;
```

The maximum chaining length ([`Algorithm::chaining_len()`](#chaining-len)) of all the
algorithms in this module.

### `SHA1_OUTPUT_LEN`

```rust
const SHA1_OUTPUT_LEN: usize = 20usize;
```

The length of the output of SHA-1, in bytes.

### `SHA256_OUTPUT_LEN`

```rust
const SHA256_OUTPUT_LEN: usize = 32usize;
```

The length of the output of SHA-256, in bytes.

### `SHA384_OUTPUT_LEN`

```rust
const SHA384_OUTPUT_LEN: usize = 48usize;
```

The length of the output of SHA-384, in bytes.

### `SHA512_OUTPUT_LEN`

```rust
const SHA512_OUTPUT_LEN: usize = 64usize;
```

The length of the output of SHA-512, in bytes.

### `SHA512_256_OUTPUT_LEN`

```rust
const SHA512_256_OUTPUT_LEN: usize = 32usize;
```

The length of the output of SHA-512/256, in bytes.

