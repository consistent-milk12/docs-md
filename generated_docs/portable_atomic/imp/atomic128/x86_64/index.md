*[portable_atomic](../../../index.md) / [imp](../../index.md) / [atomic128](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Contents

- [Modules](#modules)
  - [`fallback`](#fallback)
  - [`detect`](#detect)
- [Structs](#structs)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`cmpxchg16b`](#cmpxchg16b)
  - [`atomic_load_vmovdqa`](#atomic_load_vmovdqa)
  - [`atomic_store_vmovdqa`](#atomic_store_vmovdqa)
  - [`atomic_load`](#atomic_load)
  - [`atomic_load_cmpxchg16b`](#atomic_load_cmpxchg16b)
  - [`atomic_store`](#atomic_store)
  - [`atomic_store_cmpxchg16b`](#atomic_store_cmpxchg16b)
  - [`atomic_compare_exchange`](#atomic_compare_exchange)
  - [`atomic_swap_cmpxchg16b`](#atomic_swap_cmpxchg16b)
  - [`atomic_add_cmpxchg16b`](#atomic_add_cmpxchg16b)
  - [`atomic_sub_cmpxchg16b`](#atomic_sub_cmpxchg16b)
  - [`atomic_and_cmpxchg16b`](#atomic_and_cmpxchg16b)
  - [`atomic_nand_cmpxchg16b`](#atomic_nand_cmpxchg16b)
  - [`atomic_or_cmpxchg16b`](#atomic_or_cmpxchg16b)
  - [`atomic_xor_cmpxchg16b`](#atomic_xor_cmpxchg16b)
  - [`atomic_not_cmpxchg16b`](#atomic_not_cmpxchg16b)
  - [`atomic_neg_cmpxchg16b`](#atomic_neg_cmpxchg16b)
  - [`atomic_max_cmpxchg16b`](#atomic_max_cmpxchg16b)
  - [`atomic_umax_cmpxchg16b`](#atomic_umax_cmpxchg16b)
  - [`atomic_min_cmpxchg16b`](#atomic_min_cmpxchg16b)
  - [`atomic_umin_cmpxchg16b`](#atomic_umin_cmpxchg16b)
  - [`atomic_swap`](#atomic_swap)
  - [`atomic_add`](#atomic_add)
  - [`atomic_sub`](#atomic_sub)
  - [`atomic_and`](#atomic_and)
  - [`atomic_nand`](#atomic_nand)
  - [`atomic_or`](#atomic_or)
  - [`atomic_xor`](#atomic_xor)
  - [`atomic_max`](#atomic_max)
  - [`atomic_umax`](#atomic_umax)
  - [`atomic_min`](#atomic_min)
  - [`atomic_umin`](#atomic_umin)
  - [`atomic_not`](#atomic_not)
  - [`atomic_neg`](#atomic_neg)
  - [`is_lock_free`](#is_lock_free)
- [Constants](#constants)
  - [`IS_ALWAYS_LOCK_FREE`](#is_always_lock_free)
- [Macros](#macros)
  - [`atomic128!`](#atomic128)
  - [`atomic_rmw_by_atomic_update!`](#atomic_rmw_by_atomic_update)
  - [`debug_assert_cmpxchg16b!`](#debug_assert_cmpxchg16b)
  - [`debug_assert_vmovdqa_atomic!`](#debug_assert_vmovdqa_atomic)
  - [`ptr_modifier!`](#ptr_modifier)
  - [`load_store_detect!`](#load_store_detect)
  - [`atomic_rmw_cas_3!`](#atomic_rmw_cas_3)
  - [`atomic_rmw_cas_2!`](#atomic_rmw_cas_2)
  - [`select_atomic_rmw!`](#select_atomic_rmw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fallback`](#fallback) | mod |  |
| [`detect`](#detect) | mod |  |
| [`AtomicI128`](#atomici128) | struct |  |
| [`AtomicU128`](#atomicu128) | struct |  |
| [`cmpxchg16b`](#cmpxchg16b) | fn |  |
| [`atomic_load_vmovdqa`](#atomic_load_vmovdqa) | fn |  |
| [`atomic_store_vmovdqa`](#atomic_store_vmovdqa) | fn |  |
| [`atomic_load`](#atomic_load) | fn |  |
| [`atomic_load_cmpxchg16b`](#atomic_load_cmpxchg16b) | fn |  |
| [`atomic_store`](#atomic_store) | fn |  |
| [`atomic_store_cmpxchg16b`](#atomic_store_cmpxchg16b) | fn |  |
| [`atomic_compare_exchange`](#atomic_compare_exchange) | fn |  |
| [`atomic_swap_cmpxchg16b`](#atomic_swap_cmpxchg16b) | fn |  |
| [`atomic_add_cmpxchg16b`](#atomic_add_cmpxchg16b) | fn |  |
| [`atomic_sub_cmpxchg16b`](#atomic_sub_cmpxchg16b) | fn |  |
| [`atomic_and_cmpxchg16b`](#atomic_and_cmpxchg16b) | fn |  |
| [`atomic_nand_cmpxchg16b`](#atomic_nand_cmpxchg16b) | fn |  |
| [`atomic_or_cmpxchg16b`](#atomic_or_cmpxchg16b) | fn |  |
| [`atomic_xor_cmpxchg16b`](#atomic_xor_cmpxchg16b) | fn |  |
| [`atomic_not_cmpxchg16b`](#atomic_not_cmpxchg16b) | fn |  |
| [`atomic_neg_cmpxchg16b`](#atomic_neg_cmpxchg16b) | fn |  |
| [`atomic_max_cmpxchg16b`](#atomic_max_cmpxchg16b) | fn |  |
| [`atomic_umax_cmpxchg16b`](#atomic_umax_cmpxchg16b) | fn |  |
| [`atomic_min_cmpxchg16b`](#atomic_min_cmpxchg16b) | fn |  |
| [`atomic_umin_cmpxchg16b`](#atomic_umin_cmpxchg16b) | fn |  |
| [`atomic_swap`](#atomic_swap) | fn |  |
| [`atomic_add`](#atomic_add) | fn |  |
| [`atomic_sub`](#atomic_sub) | fn |  |
| [`atomic_and`](#atomic_and) | fn |  |
| [`atomic_nand`](#atomic_nand) | fn |  |
| [`atomic_or`](#atomic_or) | fn |  |
| [`atomic_xor`](#atomic_xor) | fn |  |
| [`atomic_max`](#atomic_max) | fn |  |
| [`atomic_umax`](#atomic_umax) | fn |  |
| [`atomic_min`](#atomic_min) | fn |  |
| [`atomic_umin`](#atomic_umin) | fn |  |
| [`atomic_not`](#atomic_not) | fn |  |
| [`atomic_neg`](#atomic_neg) | fn |  |
| [`is_lock_free`](#is_lock_free) | fn |  |
| [`IS_ALWAYS_LOCK_FREE`](#is_always_lock_free) | const |  |
| [`atomic128!`](#atomic128) | macro |  |
| [`atomic_rmw_by_atomic_update!`](#atomic_rmw_by_atomic_update) | macro |  |
| [`debug_assert_cmpxchg16b!`](#debug_assert_cmpxchg16b) | macro |  |
| [`debug_assert_vmovdqa_atomic!`](#debug_assert_vmovdqa_atomic) | macro |  |
| [`ptr_modifier!`](#ptr_modifier) | macro |  |
| [`load_store_detect!`](#load_store_detect) | macro |  |
| [`atomic_rmw_cas_3!`](#atomic_rmw_cas_3) | macro | Atomic RMW by CAS loop (3 arguments) |
| [`atomic_rmw_cas_2!`](#atomic_rmw_cas_2) | macro | Atomic RMW by CAS loop (2 arguments) |
| [`select_atomic_rmw!`](#select_atomic_rmw) | macro |  |

## Modules

- [`fallback`](fallback/index.md) - 
- [`detect`](detect/index.md) - 

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: core::cell::UnsafeCell<i128>,
}
```

#### Implementations

- <span id="atomici128-add"></span>`fn add(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- <span id="atomici128-sub"></span>`fn sub(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- <span id="atomici128-and"></span>`fn and(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- <span id="atomici128-or"></span>`fn or(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- <span id="atomici128-xor"></span>`fn xor(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: core::cell::UnsafeCell<u128>,
}
```

#### Implementations

- <span id="atomicu128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../../index.md)

- <span id="atomicu128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../../index.md)

- <span id="atomicu128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../../index.md)

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `cmpxchg16b`

```rust
unsafe fn cmpxchg16b(dst: *mut u128, old: u128, new: u128) -> (u128, bool)
```

### `atomic_load_vmovdqa`

```rust
unsafe fn atomic_load_vmovdqa(src: *mut u128) -> u128
```

### `atomic_store_vmovdqa`

```rust
unsafe fn atomic_store_vmovdqa(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

### `atomic_load`

```rust
unsafe fn atomic_load(src: *mut u128, _order: core::sync::atomic::Ordering) -> u128
```

### `atomic_load_cmpxchg16b`

```rust
unsafe fn atomic_load_cmpxchg16b(src: *mut u128) -> u128
```

### `atomic_store`

```rust
unsafe fn atomic_store(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

### `atomic_store_cmpxchg16b`

```rust
unsafe fn atomic_store_cmpxchg16b(dst: *mut u128, val: u128)
```

### `atomic_compare_exchange`

```rust
unsafe fn atomic_compare_exchange(dst: *mut u128, old: u128, new: u128, _success: core::sync::atomic::Ordering, _failure: core::sync::atomic::Ordering) -> Result<u128, u128>
```

### `atomic_swap_cmpxchg16b`

```rust
unsafe fn atomic_swap_cmpxchg16b(dst: *mut u128, val: u128, _order: core::sync::atomic::Ordering) -> u128
```

### `atomic_add_cmpxchg16b`

```rust
unsafe fn atomic_add_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_sub_cmpxchg16b`

```rust
unsafe fn atomic_sub_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_and_cmpxchg16b`

```rust
unsafe fn atomic_and_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_nand_cmpxchg16b`

```rust
unsafe fn atomic_nand_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_or_cmpxchg16b`

```rust
unsafe fn atomic_or_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_xor_cmpxchg16b`

```rust
unsafe fn atomic_xor_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_not_cmpxchg16b`

```rust
unsafe fn atomic_not_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_neg_cmpxchg16b`

```rust
unsafe fn atomic_neg_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_max_cmpxchg16b`

```rust
unsafe fn atomic_max_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umax_cmpxchg16b`

```rust
unsafe fn atomic_umax_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_min_cmpxchg16b`

```rust
unsafe fn atomic_min_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umin_cmpxchg16b`

```rust
unsafe fn atomic_umin_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_swap`

```rust
unsafe fn atomic_swap(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_add`

```rust
unsafe fn atomic_add(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_sub`

```rust
unsafe fn atomic_sub(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_and`

```rust
unsafe fn atomic_and(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_nand`

```rust
unsafe fn atomic_nand(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_or`

```rust
unsafe fn atomic_or(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_xor`

```rust
unsafe fn atomic_xor(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_max`

```rust
unsafe fn atomic_max(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umax`

```rust
unsafe fn atomic_umax(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_min`

```rust
unsafe fn atomic_min(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umin`

```rust
unsafe fn atomic_umin(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_not`

```rust
unsafe fn atomic_not(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_neg`

```rust
unsafe fn atomic_neg(dst: *mut u128, _order: Ordering) -> u128
```

### `is_lock_free`

```rust
fn is_lock_free() -> bool
```

## Constants

### `IS_ALWAYS_LOCK_FREE`

```rust
const IS_ALWAYS_LOCK_FREE: bool = false;
```

## Macros

### `atomic128!`

### `atomic_rmw_by_atomic_update!`

### `debug_assert_cmpxchg16b!`

### `debug_assert_vmovdqa_atomic!`

### `ptr_modifier!`

### `load_store_detect!`

### `atomic_rmw_cas_3!`

Atomic RMW by CAS loop (3 arguments)
`unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rsi/r8 pair: val argument (read-only for `$op`)
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `atomic_rmw_cas_2!`

Atomic RMW by CAS loop (2 arguments)
`unsafe fn(dst: *mut u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `select_atomic_rmw!`

