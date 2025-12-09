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

*Defined in [`ryu-1.0.20/src/d2s.rs:83-88`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L83-L88)*

## Functions

### `decimal_length17`

```rust
fn decimal_length17(v: u64) -> u32
```

*Defined in [`ryu-1.0.20/src/d2s.rs:38-80`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L38-L80)*

### `d2d`

```rust
fn d2d(ieee_mantissa: u64, ieee_exponent: u32) -> FloatingDecimal64
```

*Defined in [`ryu-1.0.20/src/d2s.rs:91-302`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L91-L302)*

## Constants

### `DOUBLE_MANTISSA_BITS`
```rust
const DOUBLE_MANTISSA_BITS: u32 = 52u32;
```

*Defined in [`ryu-1.0.20/src/d2s.rs:31`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L31)*

### `DOUBLE_EXPONENT_BITS`
```rust
const DOUBLE_EXPONENT_BITS: u32 = 11u32;
```

*Defined in [`ryu-1.0.20/src/d2s.rs:32`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L32)*

### `DOUBLE_BIAS`
```rust
const DOUBLE_BIAS: i32 = 1_023i32;
```

*Defined in [`ryu-1.0.20/src/d2s.rs:33`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L33)*

### `DOUBLE_POW5_INV_BITCOUNT`
```rust
const DOUBLE_POW5_INV_BITCOUNT: i32 = 125i32;
```

*Defined in [`ryu-1.0.20/src/d2s.rs:34`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L34)*

### `DOUBLE_POW5_BITCOUNT`
```rust
const DOUBLE_POW5_BITCOUNT: i32 = 125i32;
```

*Defined in [`ryu-1.0.20/src/d2s.rs:35`](../../../.source_1765210505/ryu-1.0.20/src/d2s.rs#L35)*

