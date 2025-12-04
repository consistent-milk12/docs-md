*[miniz_oxide](../index.md) / [deflate](index.md)*

---

# Module `deflate`

This module contains functionality for compression.

## Modules

- [`core`](core/index.md) - Streaming compression functionality.
- [`stream`](stream/index.md) - Extra streaming compression functionality.

## Enums

### `CompressionLevel`

```rust
enum CompressionLevel {
    NoCompression,
    BestSpeed,
    BestCompression,
    UberCompression,
    DefaultLevel,
    DefaultCompression,
}
```

How much processing the compressor should do to compress the data.
`NoCompression` and `Bestspeed` have special meanings, the other levels determine the number
of checks for matches in the hash chains and whether to use lazy or greedy parsing.

#### Variants

- **`NoCompression`**

  Don't do any compression, only output uncompressed blocks.

- **`BestSpeed`**

  Fast compression. Uses a special compression routine that is optimized for speed.

- **`BestCompression`**

  Slow/high compression. Do a lot of checks to try to find good matches.

- **`UberCompression`**

  Even more checks, can be very slow.

- **`DefaultLevel`**

  Default compromise between speed and compression.

- **`DefaultCompression`**

  Use the default compression level.

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

- `fn clone(self: &Self) -> CompressionLevel`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CompressionLevel) -> bool`

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

## Functions

### `compress_to_vec`

```rust
fn compress_to_vec(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```

Compress the input data to a vector, using the specified compression level (0-10).

### `compress_to_vec_zlib`

```rust
fn compress_to_vec_zlib(input: &[u8], level: u8) -> crate::alloc::vec::Vec<u8>
```

Compress the input data to a vector, using the specified compression level (0-10), and with a
zlib wrapper.

