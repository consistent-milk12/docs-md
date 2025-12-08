*[portable_atomic](../../index.md) / [imp](../index.md) / [fallback](index.md)*

---

# Module `fallback`

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

- `const fn new(v: i128) -> Self`

- `fn is_lock_free() -> bool`

- `const IS_ALWAYS_LOCK_FREE: bool`

- `fn load(self: &Self, order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn store(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../index.md)

- `fn swap(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn compare_exchange(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](../../index.md)

- `fn compare_exchange_weak(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>` — [`Ordering`](../../index.md)

- `fn fetch_add(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_sub(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_and(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_nand(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_or(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_xor(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_max(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_min(self: &Self, val: i128, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn fetch_not(self: &Self, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn not(self: &Self, order: Ordering)` — [`Ordering`](../../index.md)

- `fn fetch_neg(self: &Self, _order: Ordering) -> i128` — [`Ordering`](../../index.md)

- `fn neg(self: &Self, order: Ordering)` — [`Ordering`](../../index.md)

- `const fn as_ptr(self: &Self) -> *mut i128`

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: UnsafeCell<u128>,
}
```

#### Implementations

- `const LEN: usize`

- `unsafe fn chunks(self: &Self) -> &[core::sync::atomic::AtomicUsize; 2]`

- `fn optimistic_read(self: &Self) -> u128`

- `fn read(self: &Self, _guard: &SeqLockWriteGuard<'static>) -> u128` — [`SeqLockWriteGuard`](seq_lock/index.md)

- `fn write(self: &Self, val: u128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md)

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

## Macros

### `atomic!`

