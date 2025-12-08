*[gimli](../index.md) / [endianity](index.md)*

---

# Module `endianity`

Types for compile-time and run-time endianity.

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- `fn clone(self: &Self) -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LittleEndian`

- `fn default() -> LittleEndian` — [`LittleEndian`](../index.md)

##### `impl Endianity for LittleEndian`

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

Big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- `fn clone(self: &Self) -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BigEndian`

- `fn default() -> BigEndian` — [`BigEndian`](../index.md)

##### `impl Endianity for BigEndian`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- `fn eq(self: &Self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md)

##### `impl StructuralPartialEq for BigEndian`

## Enums

### `RunTimeEndian`

```rust
enum RunTimeEndian {
    Little,
    Big,
}
```

Byte order that is selectable at runtime.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for RunTimeEndian`

- `fn clone(self: &Self) -> RunTimeEndian` — [`RunTimeEndian`](../index.md)

##### `impl Copy for RunTimeEndian`

##### `impl Debug for RunTimeEndian`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RunTimeEndian`

- `fn default() -> RunTimeEndian` — [`RunTimeEndian`](../index.md)

##### `impl Endianity for RunTimeEndian`

- `fn is_big_endian(self: Self) -> bool`

##### `impl Eq for RunTimeEndian`

##### `impl Hash for RunTimeEndian`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for RunTimeEndian`

- `fn eq(self: &Self, other: &RunTimeEndian) -> bool` — [`RunTimeEndian`](../index.md)

##### `impl StructuralPartialEq for RunTimeEndian`

## Traits

### `Endianity`

```rust
trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq { ... }
```

A trait describing the endianity of some buffer.

#### Required Methods

- `fn is_big_endian(self: Self) -> bool`

  Return true for big endian byte order.

- `fn is_little_endian(self: Self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self: Self, buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(self: Self, buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(self: Self, buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_uint(self: &mut Self, buf: &[u8]) -> u64`

  Read an unsigned n-bytes integer u64.

- `fn read_i16(self: Self, buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i32(self: Self, buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i64(self: Self, buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_f32(self: Self, buf: &[u8]) -> f32`

  Reads a 32 bit floating point number from `buf`.

- `fn read_f64(self: Self, buf: &[u8]) -> f64`

  Reads a 32 bit floating point number from `buf`.

- `fn write_u16(self: Self, buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(self: Self, buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(self: Self, buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianity for the target platform.

