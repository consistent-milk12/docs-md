*[gimli](../../index.md) / [read](../index.md) / [value](index.md)*

---

# Module `value`

Definitions for values used in DWARF expressions.

## Enums

### `ValueType`

```rust
enum ValueType {
    Generic,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}
```

The type of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  The generic type, which is address-sized and of unspecified sign,
  as specified in the DWARF 5 standard, section 2.5.1.
  This type is also used to represent address base types.

- **`I8`**

  Signed 8-bit integer type.

- **`U8`**

  Unsigned 8-bit integer type.

- **`I16`**

  Signed 16-bit integer type.

- **`U16`**

  Unsigned 16-bit integer type.

- **`I32`**

  Signed 32-bit integer type.

- **`U32`**

  Unsigned 32-bit integer type.

- **`I64`**

  Signed 64-bit integer type.

- **`U64`**

  Unsigned 64-bit integer type.

- **`F32`**

  32-bit floating point type.

- **`F64`**

  64-bit floating point type.

#### Implementations

- `fn bit_size(self: Self, addr_mask: u64) -> u32`

- `fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../../index.md), [`ValueType`](../index.md)

- `fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](../index.md), [`Result`](../../index.md), [`ValueType`](../index.md)

#### Trait Implementations

##### `impl Clone for ValueType`

- `fn clone(self: &Self) -> ValueType` — [`ValueType`](../index.md)

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ValueType`

##### `impl PartialEq for ValueType`

- `fn eq(self: &Self, other: &ValueType) -> bool` — [`ValueType`](../index.md)

##### `impl StructuralPartialEq for ValueType`

### `Value`

```rust
enum Value {
    Generic(u64),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}
```

The value of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  A generic value, which is address-sized and of unspecified sign.

- **`I8`**

  A signed 8-bit integer value.

- **`U8`**

  An unsigned 8-bit integer value.

- **`I16`**

  A signed 16-bit integer value.

- **`U16`**

  An unsigned 16-bit integer value.

- **`I32`**

  A signed 32-bit integer value.

- **`U32`**

  An unsigned 32-bit integer value.

- **`I64`**

  A signed 64-bit integer value.

- **`U64`**

  An unsigned 64-bit integer value.

- **`F32`**

  A 32-bit floating point value.

- **`F64`**

  A 64-bit floating point value.

#### Implementations

- `fn value_type(self: &Self) -> ValueType` — [`ValueType`](../index.md)

- `fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn to_u64(self: Self, addr_mask: u64) -> Result<u64>` — [`Result`](../../index.md)

- `fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn convert(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn reinterpret(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md), [`Result`](../../index.md), [`Value`](../index.md)

- `fn abs(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../index.md)

- `fn neg(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../index.md)

- `fn add(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn sub(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn mul(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn div(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn rem(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn not(self: Self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md), [`Value`](../index.md)

- `fn and(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn or(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn xor(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn shift_length(self: Self) -> Result<u64>` — [`Result`](../../index.md)

- `fn shl(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn shr(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn shra(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn eq(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn ge(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn gt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn le(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn lt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

- `fn ne(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md), [`Result`](../../index.md)

#### Trait Implementations

##### `impl Clone for Value`

- `fn clone(self: &Self) -> Value` — [`Value`](../index.md)

##### `impl Copy for Value`

##### `impl Debug for Value`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for Value`

- `fn eq(self: &Self, other: &Value) -> bool` — [`Value`](../index.md)

##### `impl StructuralPartialEq for Value`

## Functions

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

