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

- [`utils`](utils/index.md)
- [`seq_lock`](seq_lock/index.md)

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: UnsafeCell<i128>,
}
```

#### Implementations

- <span id="atomici128-len"></span>`const LEN: usize`

- <span id="atomici128-chunks"></span>`unsafe fn chunks(&self) -> &[core::sync::atomic::AtomicUsize; 2]`

- <span id="atomici128-optimistic-read"></span>`fn optimistic_read(&self) -> i128`

- <span id="atomici128-read"></span>`fn read(&self, _guard: &SeqLockWriteGuard<'static>) -> i128` — [`SeqLockWriteGuard`](seq_lock/index.md)

- <span id="atomici128-write"></span>`fn write(&self, val: i128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md)

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: UnsafeCell<u128>,
}
```

#### Implementations

- <span id="atomicu128-new"></span>`const fn new(v: u128) -> Self`

- <span id="atomicu128-is-lock-free"></span>`fn is_lock_free() -> bool`

- <span id="atomicu128-is-always-lock-free"></span>`const IS_ALWAYS_LOCK_FREE: bool`

- <span id="atomicu128-load"></span>`fn load(&self, order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-store"></span>`fn store(&self, val: u128, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu128-swap"></span>`fn swap(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-compare-exchange"></span>`fn compare_exchange(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](../../index.md)

- <span id="atomicu128-compare-exchange-weak"></span>`fn compare_exchange_weak(&self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-add"></span>`fn fetch_add(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-sub"></span>`fn fetch_sub(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-and"></span>`fn fetch_and(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-nand"></span>`fn fetch_nand(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-or"></span>`fn fetch_or(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-xor"></span>`fn fetch_xor(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-max"></span>`fn fetch_max(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-min"></span>`fn fetch_min(&self, val: u128, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-not"></span>`fn fetch_not(&self, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-not"></span>`fn not(&self, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu128-fetch-neg"></span>`fn fetch_neg(&self, _order: Ordering) -> u128` — [`Ordering`](../../index.md)

- <span id="atomicu128-neg"></span>`fn neg(&self, order: Ordering)` — [`Ordering`](../../index.md)

- <span id="atomicu128-as-ptr"></span>`const fn as_ptr(&self) -> *mut u128`

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

## Macros

### `atomic!`

