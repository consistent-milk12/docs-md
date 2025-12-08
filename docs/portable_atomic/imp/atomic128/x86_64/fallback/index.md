*[portable_atomic](../../../../index.md) / [imp](../../../index.md) / [atomic128](../../index.md) / [x86_64](../index.md) / [fallback](index.md)*

---

# Module `fallback`

## Functions

### `atomic_load`

```rust
unsafe fn atomic_load(src: *mut u128, order: core::sync::atomic::Ordering) -> u128
```

### `atomic_load_seqcst`

```rust
unsafe fn atomic_load_seqcst(src: *mut u128) -> u128
```

### `atomic_store`

```rust
unsafe fn atomic_store(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

### `atomic_store_non_seqcst`

```rust
unsafe fn atomic_store_non_seqcst(dst: *mut u128, val: u128)
```

### `atomic_store_seqcst`

```rust
unsafe fn atomic_store_seqcst(dst: *mut u128, val: u128)
```

### `atomic_compare_exchange`

```rust
unsafe fn atomic_compare_exchange(dst: *mut u128, old: u128, new: u128, success: core::sync::atomic::Ordering, failure: core::sync::atomic::Ordering) -> (u128, bool)
```

### `atomic_compare_exchange_seqcst`

```rust
unsafe fn atomic_compare_exchange_seqcst(dst: *mut u128, old: u128, new: u128) -> (u128, bool)
```

### `atomic_swap`

```rust
unsafe fn atomic_swap(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_swap_seqcst`

```rust
unsafe fn atomic_swap_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_add`

```rust
unsafe fn atomic_add(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_add_seqcst`

```rust
unsafe fn atomic_add_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_sub`

```rust
unsafe fn atomic_sub(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_sub_seqcst`

```rust
unsafe fn atomic_sub_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_and`

```rust
unsafe fn atomic_and(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_and_seqcst`

```rust
unsafe fn atomic_and_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_nand`

```rust
unsafe fn atomic_nand(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_nand_seqcst`

```rust
unsafe fn atomic_nand_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_or`

```rust
unsafe fn atomic_or(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_or_seqcst`

```rust
unsafe fn atomic_or_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_xor`

```rust
unsafe fn atomic_xor(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_xor_seqcst`

```rust
unsafe fn atomic_xor_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_max`

```rust
unsafe fn atomic_max(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_max_seqcst`

```rust
unsafe fn atomic_max_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_umax`

```rust
unsafe fn atomic_umax(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_umax_seqcst`

```rust
unsafe fn atomic_umax_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_min`

```rust
unsafe fn atomic_min(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_min_seqcst`

```rust
unsafe fn atomic_min_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_umin`

```rust
unsafe fn atomic_umin(dst: *mut u128, val: u128, order: Ordering) -> u128
```

### `atomic_umin_seqcst`

```rust
unsafe fn atomic_umin_seqcst(dst: *mut u128, val: u128) -> u128
```

### `atomic_not`

```rust
unsafe fn atomic_not(dst: *mut u128, order: Ordering) -> u128
```

### `atomic_not_seqcst`

```rust
unsafe fn atomic_not_seqcst(dst: *mut u128) -> u128
```

### `atomic_neg`

```rust
unsafe fn atomic_neg(dst: *mut u128, order: Ordering) -> u128
```

### `atomic_neg_seqcst`

```rust
unsafe fn atomic_neg_seqcst(dst: *mut u128) -> u128
```

## Type Aliases

### `Udw`

```rust
type Udw = u128;
```

### `AtomicUdw`

```rust
type AtomicUdw = super::super::super::fallback::AtomicU128;
```

### `AtomicIdw`

```rust
type AtomicIdw = super::super::super::fallback::AtomicI128;
```

## Macros

### `debug_assert_outline_atomics!`

### `atomic_rmw_3!`

### `atomic_rmw_2!`

