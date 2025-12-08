*[ryu](../index.md) / [f2s](index.md)*

---

# Module `f2s`

## Contents

- [Structs](#structs)
  - [`FloatingDecimal32`](#floatingdecimal32)
- [Functions](#functions)
  - [`f2d`](#f2d)
- [Constants](#constants)
  - [`FLOAT_MANTISSA_BITS`](#float_mantissa_bits)
  - [`FLOAT_EXPONENT_BITS`](#float_exponent_bits)
  - [`FLOAT_BIAS`](#float_bias)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FloatingDecimal32`](#floatingdecimal32) | struct |  |
| [`f2d`](#f2d) | fn |  |
| [`FLOAT_MANTISSA_BITS`](#float_mantissa_bits) | const |  |
| [`FLOAT_EXPONENT_BITS`](#float_exponent_bits) | const |  |
| [`FLOAT_BIAS`](#float_bias) | const |  |
| [`unnamed`](#unnamed) | const |  |
| [`unnamed`](#unnamed) | const |  |

## Structs

### `FloatingDecimal32`

```rust
struct FloatingDecimal32 {
    pub mantissa: u32,
    pub exponent: i32,
}
```

## Functions

### `f2d`

```rust
fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32
```

## Constants

### `FLOAT_MANTISSA_BITS`

```rust
const FLOAT_MANTISSA_BITS: u32 = 23u32;
```

### `FLOAT_EXPONENT_BITS`

```rust
const FLOAT_EXPONENT_BITS: u32 = 8u32;
```

### `FLOAT_BIAS`

```rust
const FLOAT_BIAS: i32 = 127i32;
```

