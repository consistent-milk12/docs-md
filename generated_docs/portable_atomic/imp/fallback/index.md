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

- `const LEN: usize`

- `unsafe fn chunks(self: &Self) -> &[core::sync::atomic::AtomicUsize; 2]`

- `fn optimistic_read(self: &Self) -> i128`

- `fn read(self: &Self, _guard: &SeqLockWriteGuard<'static>) -> i128` — [`SeqLockWriteGuard`](seq_lock/index.md)

- `fn write(self: &Self, val: i128, _guard: &SeqLockWriteGuard<'static>)` — [`SeqLockWriteGuard`](seq_lock/index.md)

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

