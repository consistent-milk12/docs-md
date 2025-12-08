*[object](../index.md) / [endian](index.md)*

---

# Module `endian`

Types for compile-time and run-time endianness.

## Contents

- [Structs](#structs)
  - [`LittleEndian`](#littleendian)
  - [`BigEndian`](#bigendian)
  - [`U16Bytes`](#u16bytes)
  - [`U32Bytes`](#u32bytes)
  - [`U64Bytes`](#u64bytes)
  - [`I16Bytes`](#i16bytes)
  - [`I32Bytes`](#i32bytes)
  - [`I64Bytes`](#i64bytes)
- [Enums](#enums)
  - [`Endianness`](#endianness)
- [Traits](#traits)
  - [`Endian`](#endian)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`I16`](#i16)
  - [`I32`](#i32)
  - [`I64`](#i64)
- [Macros](#macros)
  - [`unsafe_impl_endian_pod!`](#unsafe_impl_endian_pod)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LittleEndian`](#littleendian) | struct | Compile-time little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Compile-time big endian byte order. |
| [`U16Bytes`](#u16bytes) | struct | An unaligned `u16` value with an externally specified endianness of type `E`. |
| [`U32Bytes`](#u32bytes) | struct | An unaligned `u32` value with an externally specified endianness of type `E`. |
| [`U64Bytes`](#u64bytes) | struct | An unaligned `u64` value with an externally specified endianness of type `E`. |
| [`I16Bytes`](#i16bytes) | struct | An unaligned `i16` value with an externally specified endianness of type `E`. |
| [`I32Bytes`](#i32bytes) | struct | An unaligned `i32` value with an externally specified endianness of type `E`. |
| [`I64Bytes`](#i64bytes) | struct | An unaligned `i64` value with an externally specified endianness of type `E`. |
| [`Endianness`](#endianness) | enum | An endianness that is selectable at run-time. |
| [`Endian`](#endian) | trait | A trait for using an endianness specification. |
| [`NativeEndian`](#nativeendian) | type | The native endianness for the target platform. |
| [`U16`](#u16) | type | A `u16` value with an externally specified endianness of type `E`. |
| [`U32`](#u32) | type | A `u32` value with an externally specified endianness of type `E`. |
| [`U64`](#u64) | type | A `u64` value with an externally specified endianness of type `E`. |
| [`I16`](#i16) | type | An `i16` value with an externally specified endianness of type `E`. |
| [`I32`](#i32) | type | An `i32` value with an externally specified endianness of type `E`. |
| [`I64`](#i64) | type | An `i64` value with an externally specified endianness of type `E`. |
| [`unsafe_impl_endian_pod!`](#unsafe_impl_endian_pod) | macro |  |

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Compile-time little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Endian for LittleEndian`

- <span id="littleendian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="littleendian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](../index.md)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Endian for BigEndian`

- <span id="bigendian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="bigendian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- <span id="bigendian-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md)

##### `impl StructuralPartialEq for BigEndian`

### `U16Bytes<E: Endian>`

```rust
struct U16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `u16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

- <span id="u16bytes-new"></span>`fn new(e: E, n: u16) -> Self`

- <span id="u16bytes-get"></span>`fn get(self, e: E) -> u16`

- <span id="u16bytes-set"></span>`fn set(&mut self, e: E, n: u16)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U16Bytes<E>`

- <span id="u16bytes-clone"></span>`fn clone(&self) -> U16Bytes<E>` — [`U16Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for U16Bytes<E>`

##### `impl<E: Endian> Debug for U16Bytes<E>`

- <span id="u16bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U16Bytes<E>`

- <span id="u16bytes-default"></span>`fn default() -> U16Bytes<E>` — [`U16Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for U16Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for U16Bytes<E>`

- <span id="u16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U16Bytes<E>`

- <span id="u16bytes-cmp"></span>`fn cmp(&self, other: &U16Bytes<E>) -> cmp::Ordering` — [`U16Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U16Bytes<E>`

- <span id="u16bytes-eq"></span>`fn eq(&self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U16Bytes<E>`

- <span id="u16bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &U16Bytes<E>) -> option::Option<cmp::Ordering>` — [`U16Bytes`](../index.md)

##### `impl<E: Endian> Pod for U16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U16Bytes<E>`

### `U32Bytes<E: Endian>`

```rust
struct U32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `u32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

- <span id="u32bytes-new"></span>`fn new(e: E, n: u32) -> Self`

- <span id="u32bytes-get"></span>`fn get(self, e: E) -> u32`

- <span id="u32bytes-set"></span>`fn set(&mut self, e: E, n: u32)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U32Bytes<E>`

- <span id="u32bytes-clone"></span>`fn clone(&self) -> U32Bytes<E>` — [`U32Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for U32Bytes<E>`

##### `impl<E: Endian> Debug for U32Bytes<E>`

- <span id="u32bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U32Bytes<E>`

- <span id="u32bytes-default"></span>`fn default() -> U32Bytes<E>` — [`U32Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for U32Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for U32Bytes<E>`

- <span id="u32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U32Bytes<E>`

- <span id="u32bytes-cmp"></span>`fn cmp(&self, other: &U32Bytes<E>) -> cmp::Ordering` — [`U32Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U32Bytes<E>`

- <span id="u32bytes-eq"></span>`fn eq(&self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U32Bytes<E>`

- <span id="u32bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &U32Bytes<E>) -> option::Option<cmp::Ordering>` — [`U32Bytes`](../index.md)

##### `impl<E: Endian> Pod for U32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U32Bytes<E>`

### `U64Bytes<E: Endian>`

```rust
struct U64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `u64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

- <span id="u64bytes-new"></span>`fn new(e: E, n: u64) -> Self`

- <span id="u64bytes-get"></span>`fn get(self, e: E) -> u64`

- <span id="u64bytes-set"></span>`fn set(&mut self, e: E, n: u64)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U64Bytes<E>`

- <span id="u64bytes-clone"></span>`fn clone(&self) -> U64Bytes<E>` — [`U64Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for U64Bytes<E>`

##### `impl<E: Endian> Debug for U64Bytes<E>`

- <span id="u64bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U64Bytes<E>`

- <span id="u64bytes-default"></span>`fn default() -> U64Bytes<E>` — [`U64Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for U64Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for U64Bytes<E>`

- <span id="u64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U64Bytes<E>`

- <span id="u64bytes-cmp"></span>`fn cmp(&self, other: &U64Bytes<E>) -> cmp::Ordering` — [`U64Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U64Bytes<E>`

- <span id="u64bytes-eq"></span>`fn eq(&self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U64Bytes<E>`

- <span id="u64bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &U64Bytes<E>) -> option::Option<cmp::Ordering>` — [`U64Bytes`](../index.md)

##### `impl<E: Endian> Pod for U64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U64Bytes<E>`

### `I16Bytes<E: Endian>`

```rust
struct I16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `i16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

- <span id="i16bytes-new"></span>`fn new(e: E, n: i16) -> Self`

- <span id="i16bytes-get"></span>`fn get(self, e: E) -> i16`

- <span id="i16bytes-set"></span>`fn set(&mut self, e: E, n: i16)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I16Bytes<E>`

- <span id="i16bytes-clone"></span>`fn clone(&self) -> I16Bytes<E>` — [`I16Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for I16Bytes<E>`

##### `impl<E: Endian> Debug for I16Bytes<E>`

- <span id="i16bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I16Bytes<E>`

- <span id="i16bytes-default"></span>`fn default() -> I16Bytes<E>` — [`I16Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for I16Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for I16Bytes<E>`

- <span id="i16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I16Bytes<E>`

- <span id="i16bytes-cmp"></span>`fn cmp(&self, other: &I16Bytes<E>) -> cmp::Ordering` — [`I16Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I16Bytes<E>`

- <span id="i16bytes-eq"></span>`fn eq(&self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I16Bytes<E>`

- <span id="i16bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &I16Bytes<E>) -> option::Option<cmp::Ordering>` — [`I16Bytes`](../index.md)

##### `impl<E: Endian> Pod for I16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I16Bytes<E>`

### `I32Bytes<E: Endian>`

```rust
struct I32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `i32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

- <span id="i32bytes-new"></span>`fn new(e: E, n: i32) -> Self`

- <span id="i32bytes-get"></span>`fn get(self, e: E) -> i32`

- <span id="i32bytes-set"></span>`fn set(&mut self, e: E, n: i32)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I32Bytes<E>`

- <span id="i32bytes-clone"></span>`fn clone(&self) -> I32Bytes<E>` — [`I32Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for I32Bytes<E>`

##### `impl<E: Endian> Debug for I32Bytes<E>`

- <span id="i32bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I32Bytes<E>`

- <span id="i32bytes-default"></span>`fn default() -> I32Bytes<E>` — [`I32Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for I32Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for I32Bytes<E>`

- <span id="i32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I32Bytes<E>`

- <span id="i32bytes-cmp"></span>`fn cmp(&self, other: &I32Bytes<E>) -> cmp::Ordering` — [`I32Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I32Bytes<E>`

- <span id="i32bytes-eq"></span>`fn eq(&self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I32Bytes<E>`

- <span id="i32bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &I32Bytes<E>) -> option::Option<cmp::Ordering>` — [`I32Bytes`](../index.md)

##### `impl<E: Endian> Pod for I32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I32Bytes<E>`

### `I64Bytes<E: Endian>`

```rust
struct I64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `i64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

- <span id="i64bytes-new"></span>`fn new(e: E, n: i64) -> Self`

- <span id="i64bytes-get"></span>`fn get(self, e: E) -> i64`

- <span id="i64bytes-set"></span>`fn set(&mut self, e: E, n: i64)`

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I64Bytes<E>`

- <span id="i64bytes-clone"></span>`fn clone(&self) -> I64Bytes<E>` — [`I64Bytes`](../index.md)

##### `impl<E: marker::Copy + Endian> Copy for I64Bytes<E>`

##### `impl<E: Endian> Debug for I64Bytes<E>`

- <span id="i64bytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I64Bytes<E>`

- <span id="i64bytes-default"></span>`fn default() -> I64Bytes<E>` — [`I64Bytes`](../index.md)

##### `impl<E: cmp::Eq + Endian> Eq for I64Bytes<E>`

##### `impl<E: hash::Hash + Endian> Hash for I64Bytes<E>`

- <span id="i64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I64Bytes<E>`

- <span id="i64bytes-cmp"></span>`fn cmp(&self, other: &I64Bytes<E>) -> cmp::Ordering` — [`I64Bytes`](../index.md)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I64Bytes<E>`

- <span id="i64bytes-eq"></span>`fn eq(&self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](../index.md)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I64Bytes<E>`

- <span id="i64bytes-partial-cmp"></span>`fn partial_cmp(&self, other: &I64Bytes<E>) -> option::Option<cmp::Ordering>` — [`I64Bytes`](../index.md)

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

- <span id="endianness-clone"></span>`fn clone(&self) -> Endianness` — [`Endianness`](../index.md)

##### `impl Copy for Endianness`

##### `impl Debug for Endianness`

- <span id="endianness-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Endianness`

- <span id="endianness-default"></span>`fn default() -> Endianness` — [`Endianness`](../index.md)

##### `impl Endian for Endianness`

- <span id="endianness-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="endianness-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for Endianness`

##### `impl Hash for Endianness`

- <span id="endianness-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Endianness`

- <span id="endianness-eq"></span>`fn eq(&self, other: &Endianness) -> bool` — [`Endianness`](../index.md)

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

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer to native endian.

- `fn read_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer to native endian.

- `fn read_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer to native endian.

- `fn read_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer to native endian.

- `fn read_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer to native endian.

- `fn read_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer to native endian.

- `fn read_u16_bytes(self, n: [u8; 2]) -> u16`

  Converts an unaligned unsigned 16 bit integer to native endian.

- `fn read_u32_bytes(self, n: [u8; 4]) -> u32`

  Converts an unaligned unsigned 32 bit integer to native endian.

- `fn read_u64_bytes(self, n: [u8; 8]) -> u64`

  Converts an unaligned unsigned 64 bit integer to native endian.

- `fn read_i16_bytes(self, n: [u8; 2]) -> i16`

  Converts an unaligned signed 16 bit integer to native endian.

- `fn read_i32_bytes(self, n: [u8; 4]) -> i32`

  Converts an unaligned signed 32 bit integer to native endian.

- `fn read_i64_bytes(self, n: [u8; 8]) -> i64`

  Converts an unaligned signed 64 bit integer to native endian.

- `fn write_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer from native endian.

- `fn write_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer from native endian.

- `fn write_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer from native endian.

- `fn write_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer from native endian.

- `fn write_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer from native endian.

- `fn write_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer from native endian.

- `fn write_u16_bytes(self, n: u16) -> [u8; 2]`

  Converts an unaligned unsigned 16 bit integer from native endian.

- `fn write_u32_bytes(self, n: u32) -> [u8; 4]`

  Converts an unaligned unsigned 32 bit integer from native endian.

- `fn write_u64_bytes(self, n: u64) -> [u8; 8]`

  Converts an unaligned unsigned 64 bit integer from native endian.

- `fn write_i16_bytes(self, n: i16) -> [u8; 2]`

  Converts an unaligned signed 16 bit integer from native endian.

- `fn write_i32_bytes(self, n: i32) -> [u8; 4]`

  Converts an unaligned signed 32 bit integer from native endian.

- `fn write_i64_bytes(self, n: i64) -> [u8; 8]`

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

