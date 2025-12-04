# Crate `adler2`

Adler-32 checksum implementation.

This implementation features:

- Permissively licensed (0BSD) clean-room implementation.
- Zero dependencies.
- Zero `unsafe`.
- Decent performance (3-4 GB/s).
- `#![no_std](#no-std)
` support (with `default-features = false`).

## Structs

### `Adler32`

```rust
struct Adler32 {
    // [REDACTED: Private Fields]
}
```

Adler-32 checksum calculator.

An instance of this type is equivalent to an Adler-32 checksum: It can be created in the default
state via [`new`](#new) (or the provided `Default` impl), or from a precalculated checksum via
[`from_checksum`](#from-checksum), and the currently stored checksum can be fetched via [`checksum`](#checksum).

This type also implements `Hasher`, which makes it easy to calculate Adler-32 checksums of any
type that implements or derives `Hash`. This also allows using Adler-32 in a `HashMap`, although
that is not recommended (while every checksum is a hash function, they are not necessarily a
good one).

# Examples

Basic, piecewise checksum calculation:

```
use adler2::Adler32;

let mut adler = Adler32::new();

adler.write_slice(&[0, 1, 2]);
adler.write_slice(&[3, 4, 5]);

assert_eq!(adler.checksum(), 0x00290010);
```

Using `Hash` to process structures:

```
use std::hash::Hash;
use adler2::Adler32;

#[derive(Hash)]
struct Data {
    byte: u8,
    word: u16,
    big: u64,
}

let mut adler = Adler32::new();

let data = Data { byte: 0x1F, word: 0xABCD, big: !0 };
data.hash(&mut adler);

// hash value depends on architecture endianness
if cfg!(target_endian = "little") {
    assert_eq!(adler.checksum(), 0x33410990);
}
if cfg!(target_endian = "big") {
    assert_eq!(adler.checksum(), 0x331F0990);
}

```




#### Implementations

- `fn new() -> Self`
  Creates a new Adler-32 instance with default state.

- `const fn from_checksum(sum: u32) -> Self`
  Creates an `Adler32` instance from a precomputed Adler-32 checksum.

- `fn checksum(self: &Self) -> u32`
  Returns the calculated checksum at this point in time.

- `fn write_slice(self: &mut Self, bytes: &[u8])`
  Adds `bytes` to the checksum calculation.

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

- `fn clone(self: &Self) -> Adler32`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Hasher`

- `fn finish(self: &Self) -> u64`

- `fn write(self: &mut Self, bytes: &[u8])`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

## Functions

### `adler32_slice`

```rust
fn adler32_slice(data: &[u8]) -> u32
```

Calculates the Adler-32 checksum of a byte slice.

This is a convenience function around the [`Adler32`](#adler32) type.


