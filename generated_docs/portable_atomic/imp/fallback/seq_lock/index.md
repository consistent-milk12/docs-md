*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [seq_lock](index.md)*

---

# Module `seq_lock`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SeqLock`](#seqlock) | struct | A simple stamped lock. |
| [`SeqLockWriteGuard`](#seqlockwriteguard) | struct | An RAII guard that releases the lock and increments the stamp when dropped. |
| [`Stamp`](#stamp) | type |  |
| [`AtomicChunk`](#atomicchunk) | type |  |
| [`Chunk`](#chunk) | type |  |

## Structs

### `SeqLock`

```rust
struct SeqLock {
    state: core::sync::atomic::AtomicUsize,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs:27-33`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs#L27-L33)*

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

- <span id="seqlock-write"></span>`fn write(&self) -> SeqLockWriteGuard<'_>` — [`SeqLockWriteGuard`](#seqlockwriteguard)

### `SeqLockWriteGuard<'a>`

```rust
struct SeqLockWriteGuard<'a> {
    lock: &'a SeqLock,
    state: usize,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs:82-88`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs#L82-L88)*

An RAII guard that releases the lock and increments the stamp when dropped.

#### Fields

- **`lock`**: `&'a SeqLock`

  The parent lock.

- **`state`**: `usize`

  The stamp before locking.

#### Implementations

- <span id="seqlockwriteguard-abort"></span>`fn abort(self)`

#### Trait Implementations

##### `impl Drop for SeqLockWriteGuard<'_>`

- <span id="seqlockwriteguard-drop"></span>`fn drop(&mut self)`

## Type Aliases

### `Stamp`

```rust
type Stamp = usize;
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs:18`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs#L18)*

### `AtomicChunk`

```rust
type AtomicChunk = core::sync::atomic::AtomicUsize;
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs:23`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs#L23)*

### `Chunk`

```rust
type Chunk = usize;
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs:24`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/fallback/seq_lock.rs#L24)*

