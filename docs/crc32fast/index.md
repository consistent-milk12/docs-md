# Crate `crc32fast`

Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.

## Usage

### Simple usage

For simple use-cases, you can call the [`hash()`](#hash) convenience function to
directly compute the CRC32 checksum for a given byte slice:

```rust
let checksum = crc32fast::hash(b"foo bar baz");
```

### Advanced usage

For use-cases that require more flexibility or performance, for example when
processing large amounts of data, you can create and manipulate a [`Hasher`](crc32fast/index.md):

```rust
use crc32fast::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"foo bar baz");
let checksum = hasher.finalize();
```

## Performance

This crate contains multiple CRC32 implementations:

- A fast baseline implementation which processes up to 16 bytes per iteration
- An optimized implementation for modern `x86` using `sse` and `pclmulqdq` instructions

Calling the [`Hasher::new`](#new) constructor at runtime will perform a feature detection to select the most
optimal implementation for the current CPU feature set.

## Structs

### `Hasher`

```rust
struct Hasher {
}
```

Represents an in-progress CRC32 computation.

#### Implementations

- `fn new() -> Self`
  Create a new `Hasher`.

- `fn new_with_initial(init: u32) -> Self`
  Create a new `Hasher` with an initial CRC32 state.

- `fn new_with_initial_len(init: u32, amount: u64) -> Self`
  Create a new `Hasher` with an initial CRC32 state.

- `fn update(self: &mut Self, buf: &[u8])`
  Process the given byte slice and update the hash state.

- `fn finalize(self: Self) -> u32`
  Finalize the hash state and return the computed CRC32 value.

- `fn reset(self: &mut Self)`
  Reset the hash state.

- `fn combine(self: &mut Self, other: &Self)`
  Combine the hash state with the hash state for the subsequent block of bytes.

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

- `fn clone(self: &Self) -> Hasher`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Hasher`

- `fn write(self: &mut Self, bytes: &[u8])`

- `fn finish(self: &Self) -> u64`

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

- `fn default() -> Self`

## Functions

### `hash`

```rust
fn hash(buf: &[u8]) -> u32
```

Computes the CRC32 hash of a byte slice.

Check out [`Hasher`](crc32fast/index.md) for more advanced use-cases.

