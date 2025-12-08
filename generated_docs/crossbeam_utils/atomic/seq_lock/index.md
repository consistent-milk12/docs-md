*[crossbeam_utils](../../index.md) / [atomic](../index.md) / [seq_lock](index.md)*

---

# Module `seq_lock`

## Structs

### `SeqLock`

```rust
struct SeqLock {
    state: core::sync::atomic::AtomicUsize,
}
```

A simple stamped lock.

#### Fields

- **`state`**: `core::sync::atomic::AtomicUsize`

  The current state of the lock.
  
  All bits except the least significant one hold the current stamp. When locked, the state
  equals 1 and doesn't contain a valid stamp.

#### Implementations

- `const fn new() -> Self`

- `fn optimistic_read(self: &Self) -> Option<usize>`

- `fn validate_read(self: &Self, stamp: usize) -> bool`

- `fn write(self: &'static Self) -> SeqLockWriteGuard` — [`SeqLockWriteGuard`](#seqlockwriteguard)

### `SeqLockWriteGuard`

```rust
struct SeqLockWriteGuard {
    lock: &'static SeqLock,
    state: usize,
}
```

An RAII guard that releases the lock and increments the stamp when dropped.

#### Fields

- **`lock`**: `&'static SeqLock`

  The parent lock.

- **`state`**: `usize`

  The stamp before locking.

#### Implementations

- `fn abort(self: Self)`

#### Trait Implementations

##### `impl Drop for SeqLockWriteGuard`

- `fn drop(self: &mut Self)`

