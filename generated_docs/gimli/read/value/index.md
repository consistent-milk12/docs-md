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

*Defined in [`gimli-0.32.3/src/read/value.rs:26-51`](../../../../.source_1765521767/gimli-0.32.3/src/read/value.rs#L26-L51)*

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

  The size in bits of a value for this type.

- <span id="valuetype-from-encoding"></span>`fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../../index.md#dwate), [`ValueType`](../index.md#valuetype)

  Construct a `ValueType` from the attributes of a base type DIE.

- <span id="valuetype-from-entry"></span>`fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<'_, '_, R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry), [`Result`](../../index.md#result), [`ValueType`](../index.md#valuetype)

  Construct a `ValueType` from a base type DIE.

#### Trait Implementations

##### `impl Any for ValueType`

- <span id="valuetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueType`

- <span id="valuetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueType`

- <span id="valuetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueType`

- <span id="valuetype-clone"></span>`fn clone(&self) -> ValueType` — [`ValueType`](../index.md#valuetype)

##### `impl CloneToUninit for ValueType`

- <span id="valuetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- <span id="valuetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueType`

##### `impl<T> From for ValueType`

- <span id="valuetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueType`

- <span id="valuetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ValueType`

- <span id="valuetype-partialeq-eq"></span>`fn eq(&self, other: &ValueType) -> bool` — [`ValueType`](../index.md#valuetype)

##### `impl StructuralPartialEq for ValueType`

##### `impl ToOwned for ValueType`

- <span id="valuetype-toowned-type-owned"></span>`type Owned = T`

- <span id="valuetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueType`

- <span id="valuetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueType`

- <span id="valuetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`gimli-0.32.3/src/read/value.rs:55-78`](../../../../.source_1765521767/gimli-0.32.3/src/read/value.rs#L55-L78)*

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

  Return the `ValueType` corresponding to this `Value`.

- <span id="value-parse"></span>`fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Read a `Value` with the given `value_type` from a `Reader`.

- <span id="value-to-u64"></span>`fn to_u64(self, addr_mask: u64) -> Result<u64>` — [`Result`](../../index.md#result)

  Convert a `Value` to a `u64`.

  

  The `ValueType` of `self` must be integral.

  Values are sign extended if the source value is signed.

- <span id="value-from-u64"></span>`fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Create a `Value` with the given `value_type` from a `u64` value.

  

  The `value_type` may be integral or floating point.

  The result is truncated if the `u64` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f32"></span>`fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Create a `Value` with the given `value_type` from a `f32` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f32` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f64"></span>`fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Create a `Value` with the given `value_type` from a `f64` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f64` value does

  not fit the bounds of the `value_type`.

- <span id="value-convert"></span>`fn convert(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Convert a `Value` to the given `value_type`.

  

  When converting between integral types, the result is truncated

  if the source value does not fit the bounds of the `value_type`.

  When converting from floating point types, the result is not defined

  if the source value does not fit the bounds of the `value_type`.

  

  This corresponds to the DWARF `DW_OP_convert` operation.

- <span id="value-reinterpret"></span>`fn reinterpret(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](../index.md#valuetype), [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Reinterpret the bits in a `Value` as the given `value_type`.

  

  The source and result value types must have equal sizes.

  

  This corresponds to the DWARF `DW_OP_reinterpret` operation.

- <span id="value-abs"></span>`fn abs(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Perform an absolute value operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_abs` operation.

- <span id="value-neg"></span>`fn neg(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Perform a negation operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_neg` operation.

- <span id="value-add"></span>`fn add(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform an addition operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_plus` operation.

- <span id="value-sub"></span>`fn sub(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a subtraction operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_minus` operation.

- <span id="value-mul"></span>`fn mul(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a multiplication operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_mul` operation.

- <span id="value-div"></span>`fn div(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a division operation.

  

  This operation requires matching types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_div` operation.

- <span id="value-rem"></span>`fn rem(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a remainder operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This corresponds to the DWARF `DW_OP_mod` operation.

- <span id="value-not"></span>`fn not(self, addr_mask: u64) -> Result<Value>` — [`Result`](../../index.md#result), [`Value`](../index.md#value)

  Perform a bitwise not operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_not` operation.

- <span id="value-and"></span>`fn and(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a bitwise and operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_and` operation.

- <span id="value-or"></span>`fn or(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a bitwise or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_or` operation.

- <span id="value-xor"></span>`fn xor(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a bitwise exclusive-or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_xor` operation.

- <span id="value-shift-length"></span>`fn shift_length(self) -> Result<u64>` — [`Result`](../../index.md#result)

  Convert value to bit length suitable for a shift operation.

  

  If the value is negative then an error is returned.

- <span id="value-shl"></span>`fn shl(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a shift left operation.

  

  This operation requires integral types.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shl` operation.

- <span id="value-shr"></span>`fn shr(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform a logical shift right operation.

  

  This operation requires an unsigned integral type for the value.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shr` operation.

- <span id="value-shra"></span>`fn shra(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform an arithmetic shift right operation.

  

  This operation requires a signed integral type for the value.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned for positive values,

  and -1 is returned for negative values.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shra` operation.

- <span id="value-eq"></span>`fn eq(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `==` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_eq` operation.

- <span id="value-ge"></span>`fn ge(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `>=` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ge` operation.

- <span id="value-gt"></span>`fn gt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `>` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_gt` operation.

- <span id="value-le"></span>`fn le(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `<= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_le` operation.

- <span id="value-lt"></span>`fn lt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `< relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_lt` operation.

- <span id="value-ne"></span>`fn ne(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](../index.md#value), [`Result`](../../index.md#result)

  Perform the `!= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ne` operation.

#### Trait Implementations

##### `impl Any for Value`

- <span id="value-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Value`

- <span id="value-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Value`

- <span id="value-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](../index.md#value)

##### `impl CloneToUninit for Value`

- <span id="value-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Value`

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Value`

- <span id="value-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Value`

- <span id="value-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Value`

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](../index.md#value)

##### `impl StructuralPartialEq for Value`

##### `impl ToOwned for Value`

- <span id="value-toowned-type-owned"></span>`type Owned = T`

- <span id="value-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="value-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Value`

- <span id="value-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="value-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Value`

- <span id="value-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="value-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

*Defined in [`gimli-0.32.3/src/read/value.rs:13-17`](../../../../.source_1765521767/gimli-0.32.3/src/read/value.rs#L13-L17)*

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

*Defined in [`gimli-0.32.3/src/read/value.rs:20-22`](../../../../.source_1765521767/gimli-0.32.3/src/read/value.rs#L20-L22)*

