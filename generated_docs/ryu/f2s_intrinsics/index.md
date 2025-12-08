*[ryu](../index.md) / [f2s_intrinsics](index.md)*

---

# Module `f2s_intrinsics`

## Contents

- [Functions](#functions)
  - [`pow5factor_32`](#pow5factor_32)
  - [`multiple_of_power_of_5_32`](#multiple_of_power_of_5_32)
  - [`multiple_of_power_of_2_32`](#multiple_of_power_of_2_32)
  - [`mul_shift_32`](#mul_shift_32)
  - [`mul_pow5_inv_div_pow2`](#mul_pow5_inv_div_pow2)
  - [`mul_pow5_div_pow2`](#mul_pow5_div_pow2)
- [Constants](#constants)
  - [`FLOAT_POW5_INV_BITCOUNT`](#float_pow5_inv_bitcount)
  - [`FLOAT_POW5_BITCOUNT`](#float_pow5_bitcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pow5factor_32`](#pow5factor_32) | fn |  |
| [`multiple_of_power_of_5_32`](#multiple_of_power_of_5_32) | fn |  |
| [`multiple_of_power_of_2_32`](#multiple_of_power_of_2_32) | fn |  |
| [`mul_shift_32`](#mul_shift_32) | fn |  |
| [`mul_pow5_inv_div_pow2`](#mul_pow5_inv_div_pow2) | fn |  |
| [`mul_pow5_div_pow2`](#mul_pow5_div_pow2) | fn |  |
| [`FLOAT_POW5_INV_BITCOUNT`](#float_pow5_inv_bitcount) | const |  |
| [`FLOAT_POW5_BITCOUNT`](#float_pow5_bitcount) | const |  |

## Functions

### `pow5factor_32`

```rust
fn pow5factor_32(value: u32) -> u32
```

### `multiple_of_power_of_5_32`

```rust
fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool
```

### `multiple_of_power_of_2_32`

```rust
fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool
```

### `mul_shift_32`

```rust
fn mul_shift_32(m: u32, factor: u64, shift: i32) -> u32
```

### `mul_pow5_inv_div_pow2`

```rust
fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32
```

### `mul_pow5_div_pow2`

```rust
fn mul_pow5_div_pow2(m: u32, i: u32, j: i32) -> u32
```

## Constants

### `FLOAT_POW5_INV_BITCOUNT`

```rust
const FLOAT_POW5_INV_BITCOUNT: i32 = 61i32;
```

### `FLOAT_POW5_BITCOUNT`

```rust
const FLOAT_POW5_BITCOUNT: i32 = 61i32;
```

