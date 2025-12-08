*[object](../index.md) / [endian](index.md)*

---

# Module `endian`

Types for compile-time and run-time endianness.

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Compile-time little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- `fn clone(self: &Self) -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LittleEndian`

- `fn default() -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Endian for LittleEndian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- `fn eq(self: &Self, other: &LittleEndian) -> bool` — [`LittleEndian`](../index.md)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- `fn clone(self: &Self) -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BigEndian`

- `fn default() -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Endian for BigEndian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- `fn eq(self: &Self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md)

##### `impl StructuralPartialEq for BigEndian`

### `U16Bytes<E: Endian>`

```rust
struct U16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `u16` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 2]) -> Self`

- `fn new(e: E, n: u16) -> Self`

- `fn get(self: Self, e: E) -> u16`

- `fn set(self: &mut Self, e: E, n: u16)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U16Bytes<E>`

- `fn clone(self: &Self) -> U16Bytes<E>` — [`U16Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U16Bytes<E>`

##### `impl<E: Endian> Debug for U16Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U16Bytes<E>`

- `fn default() -> U16Bytes<E>` — [`U16Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U16Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U16Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U16Bytes<E>`

- `fn cmp(self: &Self, other: &U16Bytes<E>) -> $crate::cmp::Ordering` — [`U16Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U16Bytes<E>`

- `fn eq(self: &Self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U16Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U16Bytes`](../index.md)

##### `impl<E: Endian> Pod for U16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U16Bytes<E>`

### `U32Bytes<E: Endian>`

```rust
struct U32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `u32` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 4]) -> Self`

- `fn new(e: E, n: u32) -> Self`

- `fn get(self: Self, e: E) -> u32`

- `fn set(self: &mut Self, e: E, n: u32)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U32Bytes<E>`

- `fn clone(self: &Self) -> U32Bytes<E>` — [`U32Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U32Bytes<E>`

##### `impl<E: Endian> Debug for U32Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U32Bytes<E>`

- `fn default() -> U32Bytes<E>` — [`U32Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U32Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U32Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U32Bytes<E>`

- `fn cmp(self: &Self, other: &U32Bytes<E>) -> $crate::cmp::Ordering` — [`U32Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U32Bytes<E>`

- `fn eq(self: &Self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U32Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U32Bytes`](../index.md)

##### `impl<E: Endian> Pod for U32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U32Bytes<E>`

### `U64Bytes<E: Endian>`

```rust
struct U64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `u64` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 8]) -> Self`

- `fn new(e: E, n: u64) -> Self`

- `fn get(self: Self, e: E) -> u64`

- `fn set(self: &mut Self, e: E, n: u64)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for U64Bytes<E>`

- `fn clone(self: &Self) -> U64Bytes<E>` — [`U64Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for U64Bytes<E>`

##### `impl<E: Endian> Debug for U64Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for U64Bytes<E>`

- `fn default() -> U64Bytes<E>` — [`U64Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for U64Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for U64Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for U64Bytes<E>`

- `fn cmp(self: &Self, other: &U64Bytes<E>) -> $crate::cmp::Ordering` — [`U64Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for U64Bytes<E>`

- `fn eq(self: &Self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for U64Bytes<E>`

- `fn partial_cmp(self: &Self, other: &U64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U64Bytes`](../index.md)

##### `impl<E: Endian> Pod for U64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U64Bytes<E>`

### `I16Bytes<E: Endian>`

```rust
struct I16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `i16` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 2]) -> Self`

- `fn new(e: E, n: i16) -> Self`

- `fn get(self: Self, e: E) -> i16`

- `fn set(self: &mut Self, e: E, n: i16)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I16Bytes<E>`

- `fn clone(self: &Self) -> I16Bytes<E>` — [`I16Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I16Bytes<E>`

##### `impl<E: Endian> Debug for I16Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I16Bytes<E>`

- `fn default() -> I16Bytes<E>` — [`I16Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I16Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I16Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I16Bytes<E>`

- `fn cmp(self: &Self, other: &I16Bytes<E>) -> $crate::cmp::Ordering` — [`I16Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I16Bytes<E>`

- `fn eq(self: &Self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I16Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I16Bytes`](../index.md)

##### `impl<E: Endian> Pod for I16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I16Bytes<E>`

### `I32Bytes<E: Endian>`

```rust
struct I32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `i32` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 4]) -> Self`

- `fn new(e: E, n: i32) -> Self`

- `fn get(self: Self, e: E) -> i32`

- `fn set(self: &mut Self, e: E, n: i32)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I32Bytes<E>`

- `fn clone(self: &Self) -> I32Bytes<E>` — [`I32Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I32Bytes<E>`

##### `impl<E: Endian> Debug for I32Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I32Bytes<E>`

- `fn default() -> I32Bytes<E>` — [`I32Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I32Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I32Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I32Bytes<E>`

- `fn cmp(self: &Self, other: &I32Bytes<E>) -> $crate::cmp::Ordering` — [`I32Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I32Bytes<E>`

- `fn eq(self: &Self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I32Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I32Bytes`](../index.md)

##### `impl<E: Endian> Pod for I32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I32Bytes<E>`

### `I64Bytes<E: Endian>`

```rust
struct I64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `i64` value with an externally specified endianness of type `E`.

#### Implementations

- `const fn from_bytes(n: [u8; 8]) -> Self`

- `fn new(e: E, n: i64) -> Self`

- `fn get(self: Self, e: E) -> i64`

- `fn set(self: &mut Self, e: E, n: i64)`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + Endian> Clone for I64Bytes<E>`

- `fn clone(self: &Self) -> I64Bytes<E>` — [`I64Bytes`](../index.md)

##### `impl<E: $crate::marker::Copy + Endian> Copy for I64Bytes<E>`

##### `impl<E: Endian> Debug for I64Bytes<E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: $crate::default::Default + Endian> Default for I64Bytes<E>`

- `fn default() -> I64Bytes<E>` — [`I64Bytes`](../index.md)

##### `impl<E: $crate::cmp::Eq + Endian> Eq for I64Bytes<E>`

##### `impl<E: $crate::hash::Hash + Endian> Hash for I64Bytes<E>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<E: $crate::cmp::Ord + Endian> Ord for I64Bytes<E>`

- `fn cmp(self: &Self, other: &I64Bytes<E>) -> $crate::cmp::Ordering` — [`I64Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialEq + Endian> PartialEq for I64Bytes<E>`

- `fn eq(self: &Self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](../index.md)

##### `impl<E: $crate::cmp::PartialOrd + Endian> PartialOrd for I64Bytes<E>`

- `fn partial_cmp(self: &Self, other: &I64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I64Bytes`](../index.md)

##### `impl<E: Endian> Pod for I64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I64Bytes<E>`

## Enums

### `Endianness`

```rust
enum Endianness {
    Little,
    Big,
}
```

An endianness that is selectable at run-time.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for Endianness`

- `fn clone(self: &Self) -> Endianness` — [`Endianness`](../index.md)

##### `impl Copy for Endianness`

##### `impl Debug for Endianness`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Endianness`

- `fn default() -> Endianness` — [`Endianness`](../index.md)

##### `impl Endian for Endianness`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for Endianness`

##### `impl Hash for Endianness`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Endianness`

- `fn eq(self: &Self, other: &Endianness) -> bool` — [`Endianness`](../index.md)

##### `impl StructuralPartialEq for Endianness`

## Traits

### `Endian`

```rust
trait Endian: Debug + Default + Clone + Copy + PartialEq + Eq + 'static { ... }
```

A trait for using an endianness specification.

Provides methods for converting between the specified endianness and
the native endianness of the target machine.

This trait does not require that the endianness is known at compile time.

#### Required Methods

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn from_little_endian(little_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_big_endian(self: Self) -> bool`

  Return true for big endian byte order.

- `fn is_little_endian(self: Self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self: Self, n: u16) -> u16`

  Converts an unsigned 16 bit integer to native endian.

- `fn read_u32(self: Self, n: u32) -> u32`

  Converts an unsigned 32 bit integer to native endian.

- `fn read_u64(self: Self, n: u64) -> u64`

  Converts an unsigned 64 bit integer to native endian.

- `fn read_i16(self: Self, n: i16) -> i16`

  Converts a signed 16 bit integer to native endian.

- `fn read_i32(self: Self, n: i32) -> i32`

  Converts a signed 32 bit integer to native endian.

- `fn read_i64(self: Self, n: i64) -> i64`

  Converts a signed 64 bit integer to native endian.

- `fn read_u16_bytes(self: Self, n: [u8; 2]) -> u16`

  Converts an unaligned unsigned 16 bit integer to native endian.

- `fn read_u32_bytes(self: Self, n: [u8; 4]) -> u32`

  Converts an unaligned unsigned 32 bit integer to native endian.

- `fn read_u64_bytes(self: Self, n: [u8; 8]) -> u64`

  Converts an unaligned unsigned 64 bit integer to native endian.

- `fn read_i16_bytes(self: Self, n: [u8; 2]) -> i16`

  Converts an unaligned signed 16 bit integer to native endian.

- `fn read_i32_bytes(self: Self, n: [u8; 4]) -> i32`

  Converts an unaligned signed 32 bit integer to native endian.

- `fn read_i64_bytes(self: Self, n: [u8; 8]) -> i64`

  Converts an unaligned signed 64 bit integer to native endian.

- `fn write_u16(self: Self, n: u16) -> u16`

  Converts an unsigned 16 bit integer from native endian.

- `fn write_u32(self: Self, n: u32) -> u32`

  Converts an unsigned 32 bit integer from native endian.

- `fn write_u64(self: Self, n: u64) -> u64`

  Converts an unsigned 64 bit integer from native endian.

- `fn write_i16(self: Self, n: i16) -> i16`

  Converts a signed 16 bit integer from native endian.

- `fn write_i32(self: Self, n: i32) -> i32`

  Converts a signed 32 bit integer from native endian.

- `fn write_i64(self: Self, n: i64) -> i64`

  Converts a signed 64 bit integer from native endian.

- `fn write_u16_bytes(self: Self, n: u16) -> [u8; 2]`

  Converts an unaligned unsigned 16 bit integer from native endian.

- `fn write_u32_bytes(self: Self, n: u32) -> [u8; 4]`

  Converts an unaligned unsigned 32 bit integer from native endian.

- `fn write_u64_bytes(self: Self, n: u64) -> [u8; 8]`

  Converts an unaligned unsigned 64 bit integer from native endian.

- `fn write_i16_bytes(self: Self, n: i16) -> [u8; 2]`

  Converts an unaligned signed 16 bit integer from native endian.

- `fn write_i32_bytes(self: Self, n: i32) -> [u8; 4]`

  Converts an unaligned signed 32 bit integer from native endian.

- `fn write_i64_bytes(self: Self, n: i64) -> [u8; 8]`

  Converts an unaligned signed 64 bit integer from native endian.

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianness for the target platform.

### `U16<E>`

```rust
type U16<E> = U16Bytes<E>;
```

A `u16` value with an externally specified endianness of type `E`.

### `U32<E>`

```rust
type U32<E> = U32Bytes<E>;
```

A `u32` value with an externally specified endianness of type `E`.

### `U64<E>`

```rust
type U64<E> = U64Bytes<E>;
```

A `u64` value with an externally specified endianness of type `E`.

### `I16<E>`

```rust
type I16<E> = I16Bytes<E>;
```

An `i16` value with an externally specified endianness of type `E`.

### `I32<E>`

```rust
type I32<E> = I32Bytes<E>;
```

An `i32` value with an externally specified endianness of type `E`.

### `I64<E>`

```rust
type I64<E> = I64Bytes<E>;
```

An `i64` value with an externally specified endianness of type `E`.

## Macros

### `unsafe_impl_endian_pod!`

