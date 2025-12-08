*[portable_atomic](../../index.md) / [imp](../index.md) / [fallback](index.md)*

---

# Module `fallback`

## Contents

- [Modules](#modules)
  - [`utils`](#utils)
  - [`seq_lock`](#seq_lock)
- [Structs](#structs)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`lock`](#lock)
- [Macros](#macros)
  - [`atomic!`](#atomic)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`utils`](#utils) | mod |  |
| [`seq_lock`](#seq_lock) | mod |  |
| [`AtomicI128`](#atomici128) | struct |  |
| [`AtomicU128`](#atomicu128) | struct |  |
| [`lock`](#lock) | fn |  |
| [`atomic!`](#atomic) | macro |  |

## Modules

- [`utils`](utils/index.md) - 
- [`seq_lock`](seq_lock/index.md) - 

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: UnsafeCell<i128>,
}
```

#### Implementations

- <span id="atomici128-new"></span>`const fn new(v: i128) -> Self`

- <span id="atomici128-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomici128-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomici128-load"></span>`fn load(&self, order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-store"></span>`fn store(&self, val: i128, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici128-swap"></span>`fn swap(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-compare-exchange"></span>`fn compare_exchange(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](../../index.md)

- <span id="atomici128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-add"></span>`fn fetch_add(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-sub"></span>`fn fetch_sub(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-and"></span>`fn fetch_and(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-nand"></span>`fn fetch_nand(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-or"></span>`fn fetch_or(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-xor"></span>`fn fetch_xor(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-max"></span>`fn fetch_max(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-min"></span>`fn fetch_min(&self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-not"></span>`fn fetch_not(&self, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici128-fetch-neg"></span>`fn fetch_neg(&self, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- <span id="atomici128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomici128-as-ptr"></span>`const fn as_ptr(&self) -> *mut i128`

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: UnsafeCell<u128>,
}
```

#### Implementations

- <span id="atomicu128-bit-set"></span>`fn bit_set(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="atomicu128-bit-clear"></span>`fn bit_clear(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

- <span id="atomicu128-bit-toggle"></span>`fn bit_toggle(&self, bit: u32, order: Ordering) -> bool` — [`Ordering`](../../index.md)

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

## Macros

### `atomic!`

