*[ryu](../index.md) / [d2s_intrinsics](index.md)*

---

# Module `d2s_intrinsics`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`div5`](#div5) | fn |  |
| [`div10`](#div10) | fn |  |
| [`div100`](#div100) | fn |  |
| [`pow5_factor`](#pow5_factor) | fn |  |
| [`multiple_of_power_of_5`](#multiple_of_power_of_5) | fn |  |
| [`multiple_of_power_of_2`](#multiple_of_power_of_2) | fn |  |
| [`mul_shift_64`](#mul_shift_64) | fn |  |
| [`mul_shift_all_64`](#mul_shift_all_64) | fn |  |

## Functions

### `div5`

```rust
fn div5(x: u64) -> u64
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:24-26`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L24-L26)*

### `div10`

```rust
fn div10(x: u64) -> u64
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:29-31`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L29-L31)*

### `div100`

```rust
fn div100(x: u64) -> u64
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:34-36`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L34-L36)*

### `pow5_factor`

```rust
fn pow5_factor(value: u64) -> u32
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:39-52`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L39-L52)*

### `multiple_of_power_of_5`

```rust
fn multiple_of_power_of_5(value: u64, p: u32) -> bool
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:56-59`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L56-L59)*

### `multiple_of_power_of_2`

```rust
fn multiple_of_power_of_2(value: u64, p: u32) -> bool
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:63-68`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L63-L68)*

### `mul_shift_64`

```rust
fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:71-75`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L71-L75)*

### `mul_shift_all_64`

```rust
unsafe fn mul_shift_all_64(m: u64, mul: &(u64, u64), j: u32, vp: *mut u64, vm: *mut u64, mm_shift: u32) -> u64
```

*Defined in [`ryu-1.0.20/src/d2s_intrinsics.rs:78-89`](../../../.source_1765210505/ryu-1.0.20/src/d2s_intrinsics.rs#L78-L89)*

