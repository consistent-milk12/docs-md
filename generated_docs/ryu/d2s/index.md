*[ryu](../index.md) / [d2s](index.md)*

---

# Module `d2s`

## Contents

- [Structs](#structs)
  - [`FloatingDecimal64`](#floatingdecimal64)
- [Functions](#functions)
  - [`decimal_length17`](#decimal_length17)
  - [`d2d`](#d2d)
- [Constants](#constants)
  - [`DOUBLE_MANTISSA_BITS`](#double_mantissa_bits)
  - [`DOUBLE_EXPONENT_BITS`](#double_exponent_bits)
  - [`DOUBLE_BIAS`](#double_bias)
  - [`DOUBLE_POW5_INV_BITCOUNT`](#double_pow5_inv_bitcount)
  - [`DOUBLE_POW5_BITCOUNT`](#double_pow5_bitcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FloatingDecimal64`](#floatingdecimal64) | struct |  |
| [`decimal_length17`](#decimal_length17) | fn |  |
| [`d2d`](#d2d) | fn |  |
| [`DOUBLE_MANTISSA_BITS`](#double_mantissa_bits) | const |  |
| [`DOUBLE_EXPONENT_BITS`](#double_exponent_bits) | const |  |
| [`DOUBLE_BIAS`](#double_bias) | const |  |
| [`DOUBLE_POW5_INV_BITCOUNT`](#double_pow5_inv_bitcount) | const |  |
| [`DOUBLE_POW5_BITCOUNT`](#double_pow5_bitcount) | const |  |

## Structs

### `FloatingDecimal64`

```rust
struct FloatingDecimal64 {
    pub mantissa: u64,
    pub exponent: i32,
}
```

## Functions

### `decimal_length17`

```rust
fn decimal_length17(v: u64) -> u32
```

### `d2d`

```rust
fn d2d(ieee_mantissa: u64, ieee_exponent: u32) -> FloatingDecimal64
```

## Constants

### `DOUBLE_MANTISSA_BITS`

```rust
const DOUBLE_MANTISSA_BITS: u32 = 52u32;
```

### `DOUBLE_EXPONENT_BITS`

```rust
const DOUBLE_EXPONENT_BITS: u32 = 11u32;
```

### `DOUBLE_BIAS`

```rust
const DOUBLE_BIAS: i32 = 1_023i32;
```

### `DOUBLE_POW5_INV_BITCOUNT`

```rust
const DOUBLE_POW5_INV_BITCOUNT: i32 = 125i32;
```

### `DOUBLE_POW5_BITCOUNT`

```rust
const DOUBLE_POW5_BITCOUNT: i32 = 125i32;
```

