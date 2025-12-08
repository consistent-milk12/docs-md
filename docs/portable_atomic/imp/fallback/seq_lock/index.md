*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [seq_lock](index.md)*

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

- `fn write(self: &Self) -> SeqLockWriteGuard<'_>` — [`SeqLockWriteGuard`](#seqlockwriteguard)

### `SeqLockWriteGuard<'a>`

```rust
struct SeqLockWriteGuard<'a> {
    lock: &'a SeqLock,
    state: usize,
}
```

An RAII guard that releases the lock and increments the stamp when dropped.

#### Fields

- **`lock`**: `&'a SeqLock`

  The parent lock.

- **`state`**: `usize`

  The stamp before locking.

#### Implementations

- `fn abort(self: Self)`

#### Trait Implementations

##### `impl Drop for SeqLockWriteGuard<'_>`

- `fn drop(self: &mut Self)`

## Type Aliases

### `Stamp`

```rust
type Stamp = usize;
```

### `AtomicChunk`

```rust
type AtomicChunk = core::sync::atomic::AtomicUsize;
```

### `Chunk`

```rust
type Chunk = usize;
```

