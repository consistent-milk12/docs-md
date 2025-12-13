*[ryu](../index.md) / [f2s_intrinsics](index.md)*

---

# Module `f2s_intrinsics`

## Contents

- [Functions](#functions)
  - [`pow5factor_32`](#pow5factor-32)
  - [`multiple_of_power_of_5_32`](#multiple-of-power-of-5-32)
  - [`multiple_of_power_of_2_32`](#multiple-of-power-of-2-32)
  - [`mul_shift_32`](#mul-shift-32)
  - [`mul_pow5_inv_div_pow2`](#mul-pow5-inv-div-pow2)
  - [`mul_pow5_div_pow2`](#mul-pow5-div-pow2)
- [Constants](#constants)
  - [`FLOAT_POW5_INV_BITCOUNT`](#float-pow5-inv-bitcount)
  - [`FLOAT_POW5_BITCOUNT`](#float-pow5-bitcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pow5factor_32`](#pow5factor-32) | fn |  |
| [`multiple_of_power_of_5_32`](#multiple-of-power-of-5-32) | fn |  |
| [`multiple_of_power_of_2_32`](#multiple-of-power-of-2-32) | fn |  |
| [`mul_shift_32`](#mul-shift-32) | fn |  |
| [`mul_pow5_inv_div_pow2`](#mul-pow5-inv-div-pow2) | fn |  |
| [`mul_pow5_div_pow2`](#mul-pow5-div-pow2) | fn |  |
| [`FLOAT_POW5_INV_BITCOUNT`](#float-pow5-inv-bitcount) | const |  |
| [`FLOAT_POW5_BITCOUNT`](#float-pow5-bitcount) | const |  |

## Functions

### `pow5factor_32`

```rust
fn pow5factor_32(value: u32) -> u32
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:27-40`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L27-L40)*

### `multiple_of_power_of_5_32`

```rust
fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:44-46`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L44-L46)*

### `multiple_of_power_of_2_32`

```rust
fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:50-53`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L50-L53)*

### `mul_shift_32`

```rust
fn mul_shift_32(m: u32, factor: u64, shift: i32) -> u32
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:58-72`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L58-L72)*

### `mul_pow5_inv_div_pow2`

```rust
fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:75-98`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L75-L98)*

### `mul_pow5_div_pow2`

```rust
fn mul_pow5_div_pow2(m: u32, i: u32, j: i32) -> u32
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:101-113`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L101-L113)*

## Constants

### `FLOAT_POW5_INV_BITCOUNT`
```rust
const FLOAT_POW5_INV_BITCOUNT: i32 = 61i32;
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:23`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L23)*

### `FLOAT_POW5_BITCOUNT`
```rust
const FLOAT_POW5_BITCOUNT: i32 = 61i32;
```

*Defined in [`ryu-1.0.20/src/f2s_intrinsics.rs:24`](../../../.source_1765633015/ryu-1.0.20/src/f2s_intrinsics.rs#L24)*

