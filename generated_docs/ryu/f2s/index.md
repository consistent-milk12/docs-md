*[ryu](../index.md) / [f2s](index.md)*

---

# Module `f2s`

## Contents

- [Structs](#structs)
  - [`FloatingDecimal32`](#floatingdecimal32)
- [Functions](#functions)
  - [`f2d`](#f2d)
- [Constants](#constants)
  - [`FLOAT_MANTISSA_BITS`](#float-mantissa-bits)
  - [`FLOAT_EXPONENT_BITS`](#float-exponent-bits)
  - [`FLOAT_BIAS`](#float-bias)
  - [`FLOAT_POW5_BITCOUNT`](#float-pow5-bitcount)
  - [`FLOAT_POW5_INV_BITCOUNT`](#float-pow5-inv-bitcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FloatingDecimal32`](#floatingdecimal32) | struct |  |
| [`f2d`](#f2d) | fn |  |
| [`FLOAT_MANTISSA_BITS`](#float-mantissa-bits) | const |  |
| [`FLOAT_EXPONENT_BITS`](#float-exponent-bits) | const |  |
| [`FLOAT_BIAS`](#float-bias) | const |  |
| [`FLOAT_POW5_BITCOUNT`](#float-pow5-bitcount) | const |  |
| [`FLOAT_POW5_INV_BITCOUNT`](#float-pow5-inv-bitcount) | const |  |

## Structs

### `FloatingDecimal32`

```rust
struct FloatingDecimal32 {
    pub mantissa: u32,
    pub exponent: i32,
}
```

*Defined in [`ryu-1.0.20/src/f2s.rs:32-37`](../../../.source_1765521767/ryu-1.0.20/src/f2s.rs#L32-L37)*

#### Trait Implementations

##### `impl Any for FloatingDecimal32`

- <span id="floatingdecimal32-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FloatingDecimal32`

- <span id="floatingdecimal32-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FloatingDecimal32`

- <span id="floatingdecimal32-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FloatingDecimal32`

- <span id="floatingdecimal32-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FloatingDecimal32`

- <span id="floatingdecimal32-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FloatingDecimal32`

- <span id="floatingdecimal32-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="floatingdecimal32-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FloatingDecimal32`

- <span id="floatingdecimal32-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="floatingdecimal32-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `f2d`

```rust
fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32
```

*Defined in [`ryu-1.0.20/src/f2s.rs:40-178`](../../../.source_1765521767/ryu-1.0.20/src/f2s.rs#L40-L178)*

## Constants

### `FLOAT_MANTISSA_BITS`
```rust
const FLOAT_MANTISSA_BITS: u32 = 23u32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:26`](../../../.source_1765521767/ryu-1.0.20/src/f2s.rs#L26)*

### `FLOAT_EXPONENT_BITS`
```rust
const FLOAT_EXPONENT_BITS: u32 = 8u32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:27`](../../../.source_1765521767/ryu-1.0.20/src/f2s.rs#L27)*

### `FLOAT_BIAS`
```rust
const FLOAT_BIAS: i32 = 127i32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:28`](../../../.source_1765521767/ryu-1.0.20/src/f2s.rs#L28)*

### `FLOAT_POW5_BITCOUNT`
```rust
const FLOAT_POW5_BITCOUNT: i32 = 61i32;
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:24`](../../../.source_1765521767/ryu-1.0.20/src/f2s_intrinsics.rs#L24)*

### `FLOAT_POW5_INV_BITCOUNT`
```rust
const FLOAT_POW5_INV_BITCOUNT: i32 = 61i32;
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:23`](../../../.source_1765521767/ryu-1.0.20/src/f2s_intrinsics.rs#L23)*

