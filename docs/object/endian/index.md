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

##### `impl Clone`

- `fn clone(self: &Self) -> LittleEndian` — [`LittleEndian`](../../endian/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> LittleEndian` — [`LittleEndian`](../../endian/index.md)

##### `impl Endian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LittleEndian) -> bool` — [`LittleEndian`](../../endian/index.md)

##### `impl StructuralPartialEq`

### `BigEndian`

```rust
struct BigEndian;
```

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> BigEndian` — [`BigEndian`](../../endian/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> BigEndian` — [`BigEndian`](../../endian/index.md)

##### `impl Endian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &BigEndian) -> bool` — [`BigEndian`](../../endian/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> U16Bytes<E>` — [`U16Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> U16Bytes<E>` — [`U16Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &U16Bytes<E>) -> $crate::cmp::Ordering` — [`U16Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &U16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U16Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> U32Bytes<E>` — [`U32Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> U32Bytes<E>` — [`U32Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &U32Bytes<E>) -> $crate::cmp::Ordering` — [`U32Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &U32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U32Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> U64Bytes<E>` — [`U64Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> U64Bytes<E>` — [`U64Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &U64Bytes<E>) -> $crate::cmp::Ordering` — [`U64Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &U64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`U64Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> I16Bytes<E>` — [`I16Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> I16Bytes<E>` — [`I16Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &I16Bytes<E>) -> $crate::cmp::Ordering` — [`I16Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &I16Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I16Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> I32Bytes<E>` — [`I32Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> I32Bytes<E>` — [`I32Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &I32Bytes<E>) -> $crate::cmp::Ordering` — [`I32Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &I32Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I32Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone<E: $crate::clone::Clone + Endian>`

- `fn clone(self: &Self) -> I64Bytes<E>` — [`I64Bytes`](../../endian/index.md)

##### `impl Copy<E: $crate::marker::Copy + Endian>`

##### `impl Debug<E: Endian>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<E: $crate::default::Default + Endian>`

- `fn default() -> I64Bytes<E>` — [`I64Bytes`](../../endian/index.md)

##### `impl Eq<E: $crate::cmp::Eq + Endian>`

##### `impl Hash<E: $crate::hash::Hash + Endian>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<E: $crate::cmp::Ord + Endian>`

- `fn cmp(self: &Self, other: &I64Bytes<E>) -> $crate::cmp::Ordering` — [`I64Bytes`](../../endian/index.md)

##### `impl PartialEq<E: $crate::cmp::PartialEq + Endian>`

- `fn eq(self: &Self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](../../endian/index.md)

##### `impl PartialOrd<E: $crate::cmp::PartialOrd + Endian>`

- `fn partial_cmp(self: &Self, other: &I64Bytes<E>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`I64Bytes`](../../endian/index.md)

##### `impl Pod<E: Endian>`

##### `impl StructuralPartialEq<E: Endian>`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Endianness` — [`Endianness`](../../endian/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Endianness` — [`Endianness`](../../endian/index.md)

##### `impl Endian`

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Endianness) -> bool` — [`Endianness`](../../endian/index.md)

##### `impl StructuralPartialEq`

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

