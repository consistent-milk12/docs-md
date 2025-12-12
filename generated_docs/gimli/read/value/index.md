*[gimli](../../index.md) / [read](../index.md) / [value](index.md)*

---

# Module `value`

Definitions for values used in DWARF expressions.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueType`](#valuetype) | enum | The type of an entry on the DWARF stack. |
| [`Value`](#value) | enum | The value of an entry on the DWARF stack. |
| [`sign_extend`](#sign-extend) | fn | Convert a u64 to an i64, with sign extension if required. |
| [`mask_bit_size`](#mask-bit-size) | fn |  |

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

*Defined in [`gimli-0.32.3/src/read/value.rs:26-51`](../../../../.source_1765210505/gimli-0.32.3/src/read/value.rs#L26-L51)*

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

- <span id="valuetype-bit-size"></span>`fn bit_size(self, addr_mask: u64) -> u32`

- <span id="valuetype-from-encoding"></span>`fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../../index.md#dwate), [`ValueType`](../index.md#valuetype)

- <span id="valuetype-from-entry"></span>`fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry), [`Result`](../../index.md#result), [`ValueType`](../index.md#valuetype)

#### Trait Implementations

##### `impl Clone for ValueType`

- <span id="valuetype-clone"></span>`fn clone(&self) -> ValueType` — [`ValueType`](../index.md#valuetype)

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- <span id="valuetype-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueType`

##### `impl PartialEq for ValueType`

- <span id="valuetype-eq"></span>`fn eq(&self, other: &ValueType) -> bool` — [`ValueType`](../index.md#valuetype)

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

*Defined in [`gimli-0.32.3/src/read/value.rs:55-78`](../../../../.source_1765210505/gimli-0.32.3/src/read/value.rs#L55-L78)*

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

- <span id="value-value-type"></span>`fn value_type(&self) -> ValueType` — [`ValueType`](../index.md#valuetype)

- <span id="value-parse"></span>`fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-to-u64"></span>`fn to_u64(self, addr_mask: u64) -> Result<u64>` — [`Result`](../../index.md#result)

- <span id="value-from-u64"></span>`fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-from-f32"></span>`fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-from-f64"></span>`fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-convert"></span>`fn convert(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-reinterpret"></span>`fn reinterpret(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-abs"></span>`fn abs(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-neg"></span>`fn neg(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-add"></span>`fn add(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-sub"></span>`fn sub(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-mul"></span>`fn mul(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-div"></span>`fn div(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-rem"></span>`fn rem(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-not"></span>`fn not(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

- <span id="value-and"></span>`fn and(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-or"></span>`fn or(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-xor"></span>`fn xor(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-shift-length"></span>`fn shift_length(self) -> Result<u64>` — [`Result`](../../index.md#result)

- <span id="value-shl"></span>`fn shl(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-shr"></span>`fn shr(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-shra"></span>`fn shra(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-eq"></span>`fn eq(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-ge"></span>`fn ge(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-gt"></span>`fn gt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-le"></span>`fn le(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-lt"></span>`fn lt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

- <span id="value-ne"></span>`fn ne(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](../index.md#value)

##### `impl Copy for Value`

##### `impl Debug for Value`

- <span id="value-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Value`

- <span id="value-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](../index.md#value)

##### `impl StructuralPartialEq for Value`

## Functions

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

*Defined in [`gimli-0.32.3/src/read/value.rs:13-17`](../../../../.source_1765210505/gimli-0.32.3/src/read/value.rs#L13-L17)*

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

*Defined in [`gimli-0.32.3/src/read/value.rs:20-22`](../../../../.source_1765210505/gimli-0.32.3/src/read/value.rs#L20-L22)*

