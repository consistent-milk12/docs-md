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
  - [`FLOAT_POW5_BITCOUNT`](#float_pow5_bitcount)
  - [`FLOAT_POW5_INV_BITCOUNT`](#float_pow5_inv_bitcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FloatingDecimal32`](#floatingdecimal32) | struct |  |
| [`f2d`](#f2d) | fn |  |
| [`FLOAT_MANTISSA_BITS`](#float_mantissa_bits) | const |  |
| [`FLOAT_EXPONENT_BITS`](#float_exponent_bits) | const |  |
| [`FLOAT_BIAS`](#float_bias) | const |  |
| [`FLOAT_POW5_BITCOUNT`](#float_pow5_bitcount) | const |  |
| [`FLOAT_POW5_INV_BITCOUNT`](#float_pow5_inv_bitcount) | const |  |

## Structs

### `FloatingDecimal32`

```rust
struct FloatingDecimal32 {
    pub mantissa: u32,
    pub exponent: i32,
}
```

*Defined in [`ryu-1.0.20/src/f2s.rs:32-37`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L32-L37)*

## Functions

### `f2d`

```rust
fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32
```

*Defined in [`ryu-1.0.20/src/f2s.rs:40-178`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L40-L178)*

## Constants

### `FLOAT_MANTISSA_BITS`
```rust
const FLOAT_MANTISSA_BITS: u32 = 23u32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:26`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L26)*

### `FLOAT_EXPONENT_BITS`
```rust
const FLOAT_EXPONENT_BITS: u32 = 8u32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:27`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L27)*

### `FLOAT_BIAS`
```rust
const FLOAT_BIAS: i32 = 127i32;
```

*Defined in [`ryu-1.0.20/src/f2s.rs:28`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L28)*

*Defined in [`ryu-1.0.20/src/f2s.rs:29`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L29)*

*Defined in [`ryu-1.0.20/src/f2s.rs:29`](../../../.source_1765210505/ryu-1.0.20/src/f2s.rs#L29)*

