*[crossbeam_utils](../../index.md) / [atomic](../index.md) / [seq_lock](index.md)*

---

# Module `seq_lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SeqLock`](#seqlock) | struct | A simple stamped lock. |
| [`SeqLockWriteGuard`](#seqlockwriteguard) | struct | An RAII guard that releases the lock and increments the stamp when dropped. |

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

- <span id="seqlock-new"></span>`const fn new() -> Self`

- <span id="seqlock-optimistic-read"></span>`fn optimistic_read(&self) -> Option<usize>`

- <span id="seqlock-validate-read"></span>`fn validate_read(&self, stamp: usize) -> bool`

- <span id="seqlock-write"></span>`fn write(self: &'static Self) -> SeqLockWriteGuard` — [`SeqLockWriteGuard`](#seqlockwriteguard)

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

- <span id="seqlockwriteguard-abort"></span>`fn abort(self)`

#### Trait Implementations

##### `impl Drop for SeqLockWriteGuard`

- <span id="seqlockwriteguard-drop"></span>`fn drop(&mut self)`

