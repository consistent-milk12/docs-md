*[portable_atomic](../../../../index.md) / [imp](../../../index.md) / [atomic128](../../index.md) / [x86_64](../index.md) / [fallback](index.md)*

---

# Module `fallback`

## Contents

- [Functions](#functions)
  - [`atomic_load`](#atomic_load)
  - [`atomic_load_seqcst`](#atomic_load_seqcst)
  - [`atomic_store`](#atomic_store)
  - [`atomic_store_non_seqcst`](#atomic_store_non_seqcst)
  - [`atomic_store_seqcst`](#atomic_store_seqcst)
  - [`atomic_compare_exchange`](#atomic_compare_exchange)
  - [`atomic_compare_exchange_seqcst`](#atomic_compare_exchange_seqcst)
  - [`atomic_swap`](#atomic_swap)
  - [`atomic_swap_seqcst`](#atomic_swap_seqcst)
  - [`atomic_add`](#atomic_add)
  - [`atomic_add_seqcst`](#atomic_add_seqcst)
  - [`atomic_sub`](#atomic_sub)
  - [`atomic_sub_seqcst`](#atomic_sub_seqcst)
  - [`atomic_and`](#atomic_and)
  - [`atomic_and_seqcst`](#atomic_and_seqcst)
  - [`atomic_nand`](#atomic_nand)
  - [`atomic_nand_seqcst`](#atomic_nand_seqcst)
  - [`atomic_or`](#atomic_or)
  - [`atomic_or_seqcst`](#atomic_or_seqcst)
  - [`atomic_xor`](#atomic_xor)
  - [`atomic_xor_seqcst`](#atomic_xor_seqcst)
  - [`atomic_max`](#atomic_max)
  - [`atomic_max_seqcst`](#atomic_max_seqcst)
  - [`atomic_umax`](#atomic_umax)
  - [`atomic_umax_seqcst`](#atomic_umax_seqcst)
  - [`atomic_min`](#atomic_min)
  - [`atomic_min_seqcst`](#atomic_min_seqcst)
  - [`atomic_umin`](#atomic_umin)
  - [`atomic_umin_seqcst`](#atomic_umin_seqcst)
  - [`atomic_not`](#atomic_not)
  - [`atomic_not_seqcst`](#atomic_not_seqcst)
  - [`atomic_neg`](#atomic_neg)
  - [`atomic_neg_seqcst`](#atomic_neg_seqcst)
- [Type Aliases](#type-aliases)
  - [`Udw`](#udw)
  - [`AtomicUdw`](#atomicudw)
  - [`AtomicIdw`](#atomicidw)
- [Macros](#macros)
  - [`debug_assert_outline_atomics!`](#debug_assert_outline_atomics)
  - [`atomic_rmw_3!`](#atomic_rmw_3)
  - [`atomic_rmw_2!`](#atomic_rmw_2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`atomic_load`](#atomic_load) | fn |  |
| [`atomic_load_seqcst`](#atomic_load_seqcst) | fn |  |
| [`atomic_store`](#atomic_store) | fn |  |
| [`atomic_store_non_seqcst`](#atomic_store_non_seqcst) | fn |  |
| [`atomic_store_seqcst`](#atomic_store_seqcst) | fn |  |
| [`atomic_compare_exchange`](#atomic_compare_exchange) | fn |  |
| [`atomic_compare_exchange_seqcst`](#atomic_compare_exchange_seqcst) | fn |  |
| [`atomic_swap`](#atomic_swap) | fn |  |
| [`atomic_swap_seqcst`](#atomic_swap_seqcst) | fn |  |
| [`atomic_add`](#atomic_add) | fn |  |
| [`atomic_add_seqcst`](#atomic_add_seqcst) | fn |  |
| [`atomic_sub`](#atomic_sub) | fn |  |
| [`atomic_sub_seqcst`](#atomic_sub_seqcst) | fn |  |
| [`atomic_and`](#atomic_and) | fn |  |
| [`atomic_and_seqcst`](#atomic_and_seqcst) | fn |  |
| [`atomic_nand`](#atomic_nand) | fn |  |
| [`atomic_nand_seqcst`](#atomic_nand_seqcst) | fn |  |
| [`atomic_or`](#atomic_or) | fn |  |
| [`atomic_or_seqcst`](#atomic_or_seqcst) | fn |  |
| [`atomic_xor`](#atomic_xor) | fn |  |
| [`atomic_xor_seqcst`](#atomic_xor_seqcst) | fn |  |
| [`atomic_max`](#atomic_max) | fn |  |
| [`atomic_max_seqcst`](#atomic_max_seqcst) | fn |  |
| [`atomic_umax`](#atomic_umax) | fn |  |
| [`atomic_umax_seqcst`](#atomic_umax_seqcst) | fn |  |
| [`atomic_min`](#atomic_min) | fn |  |
| [`atomic_min_seqcst`](#atomic_min_seqcst) | fn |  |
| [`atomic_umin`](#atomic_umin) | fn |  |
| [`atomic_umin_seqcst`](#atomic_umin_seqcst) | fn |  |
| [`atomic_not`](#atomic_not) | fn |  |
| [`atomic_not_seqcst`](#atomic_not_seqcst) | fn |  |
| [`atomic_neg`](#atomic_neg) | fn |  |
| [`atomic_neg_seqcst`](#atomic_neg_seqcst) | fn |  |
| [`Udw`](#udw) | type |  |
| [`AtomicUdw`](#atomicudw) | type |  |
| [`AtomicIdw`](#atomicidw) | type |  |
| [`debug_assert_outline_atomics!`](#debug_assert_outline_atomics) | macro |  |
| [`atomic_rmw_3!`](#atomic_rmw_3) | macro |  |
| [`atomic_rmw_2!`](#atomic_rmw_2) | macro |  |

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

