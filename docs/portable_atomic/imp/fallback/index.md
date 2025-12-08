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

- `fn load(self: &Self, order: Ordering) -> i128`

- `fn store(self: &Self, val: i128, order: Ordering)`

- `fn swap(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn compare_exchange(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>`

- `fn compare_exchange_weak(self: &Self, current: i128, new: i128, success: Ordering, failure: Ordering) -> Result<i128, i128>`

- `fn fetch_add(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_sub(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_and(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_nand(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_or(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_xor(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_max(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_min(self: &Self, val: i128, _order: Ordering) -> i128`

- `fn fetch_not(self: &Self, _order: Ordering) -> i128`

- `fn not(self: &Self, order: Ordering)`

- `fn fetch_neg(self: &Self, _order: Ordering) -> i128`

- `fn neg(self: &Self, order: Ordering)`

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

- `fn add(self: &Self, val: u128, order: Ordering)`

- `fn sub(self: &Self, val: u128, order: Ordering)`

- `fn and(self: &Self, val: u128, order: Ordering)`

- `fn or(self: &Self, val: u128, order: Ordering)`

- `fn xor(self: &Self, val: u128, order: Ordering)`

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `lock`

```rust
fn lock(addr: usize) -> &'static self::seq_lock::SeqLock
```

## Macros

### `atomic!`

