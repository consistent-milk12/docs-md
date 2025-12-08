*[ryu](../index.md) / [d2s_intrinsics](index.md)*

---

# Module `d2s_intrinsics`

## Functions

### `div5`

```rust
fn div5(x: u64) -> u64
```

### `div10`

```rust
fn div10(x: u64) -> u64
```

### `div100`

```rust
fn div100(x: u64) -> u64
```

### `pow5_factor`

```rust
fn pow5_factor(value: u64) -> u32
```

### `multiple_of_power_of_5`

```rust
fn multiple_of_power_of_5(value: u64, p: u32) -> bool
```

### `multiple_of_power_of_2`

```rust
fn multiple_of_power_of_2(value: u64, p: u32) -> bool
```

### `mul_shift_64`

```rust
fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64
```

### `mul_shift_all_64`

```rust
unsafe fn mul_shift_all_64(m: u64, mul: &(u64, u64), j: u32, vp: *mut u64, vm: *mut u64, mm_shift: u32) -> u64
```

