*[crossbeam_utils](../../index.md) / [sync](../index.md) / [sharded_lock](index.md)*

---

# Module `sharded_lock`

## Contents

- [Structs](#structs)
  - [`Shard`](#shard)
  - [`ShardedLock`](#shardedlock)
  - [`ShardedLockReadGuard`](#shardedlockreadguard)
  - [`ShardedLockWriteGuard`](#shardedlockwriteguard)
  - [`ThreadIndices`](#threadindices)
  - [`Registration`](#registration)
- [Functions](#functions)
  - [`current_index`](#current-index)
  - [`thread_indices`](#thread-indices)
- [Constants](#constants)
  - [`NUM_SHARDS`](#num-shards)
  - [`REGISTRATION`](#registration)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Shard`](#shard) | struct | A shard containing a single reader-writer lock. |
| [`ShardedLock`](#shardedlock) | struct | A sharded reader-writer lock. |
| [`ShardedLockReadGuard`](#shardedlockreadguard) | struct | A guard used to release the shared read access of a [`ShardedLock`] when dropped. |
| [`ShardedLockWriteGuard`](#shardedlockwriteguard) | struct | A guard used to release the exclusive write access of a [`ShardedLock`] when dropped. |
| [`ThreadIndices`](#threadindices) | struct | The global registry keeping track of registered threads and indices. |
| [`Registration`](#registration) | struct | A registration of a thread with an index. |
| [`current_index`](#current-index) | fn | Returns a `usize` that identifies the current thread. |
| [`thread_indices`](#thread-indices) | fn |  |
| [`NUM_SHARDS`](#num-shards) | const | The number of shards per sharded lock. |
| [`REGISTRATION`](#registration) | const |  |

## Structs

### `Shard`

```rust
struct Shard {
    lock: std::sync::RwLock<()>,
    write_guard: std::cell::UnsafeCell<Option<std::sync::RwLockWriteGuard<'static, ()>>>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:21-30`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L21-L30)*

A shard containing a single reader-writer lock.

#### Fields

- **`lock`**: `std::sync::RwLock<()>`

  The inner reader-writer lock.

- **`write_guard`**: `std::cell::UnsafeCell<Option<std::sync::RwLockWriteGuard<'static, ()>>>`

  The write-guard keeping this shard locked.
  
  Write operations will lock each shard and store the guard here. These guards get dropped at
  the same time the big guard is dropped.

### `ShardedLock<T: ?Sized>`

```rust
struct ShardedLock<T: ?Sized> {
    shards: std::boxed::Box<[crate::CachePadded<Shard>]>,
    value: std::cell::UnsafeCell<T>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:78-84`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L78-L84)*

A sharded reader-writer lock.

This lock is equivalent to `RwLock`, except read operations are faster and write operations
are slower.

A `ShardedLock` is internally made of a list of *shards*, each being a `RwLock` occupying a
single cache line. Read operations will pick one of the shards depending on the current thread
and lock it. Write operations need to lock all shards in succession.

By splitting the lock into shards, concurrent read operations will in most cases choose
different shards and thus update different cache lines, which is good for scalability. However,
write operations need to do more work and are therefore slower than usual.

The priority policy of the lock is dependent on the underlying operating system's
implementation, and this type does not guarantee that any particular policy will be used.

# Poisoning

A `ShardedLock`, like `RwLock`, will become poisoned on a panic. Note that it may only be
poisoned if a panic occurs while a write operation is in progress. If a panic occurs in any
read operation, the lock will not be poisoned.

# Examples

```rust
use crossbeam_utils::sync::ShardedLock;

let lock = ShardedLock::new(5);

// Any number of read locks can be held at once.
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // Read locks are dropped at this point.

// However, only one write lock may be held.
{
    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);
} // Write lock is dropped here.
```


#### Fields

- **`shards`**: `std::boxed::Box<[crate::CachePadded<Shard>]>`

  A list of locks protecting the internal data.

- **`value`**: `std::cell::UnsafeCell<T>`

  The internal data.

#### Implementations

- <span id="shardedlock-new"></span>`fn new(value: T) -> ShardedLock<T>` — [`ShardedLock`](#shardedlock)

- <span id="shardedlock-into-inner"></span>`fn into_inner(self) -> LockResult<T>`

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Debug> Debug for ShardedLock<T>`

- <span id="shardedlock-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for ShardedLock<T>`

- <span id="shardedlock-default"></span>`fn default() -> ShardedLock<T>` — [`ShardedLock`](#shardedlock)

##### `impl<T: ?Sized> RefUnwindSafe for ShardedLock<T>`

##### `impl<T: ?Sized + Send> Send for ShardedLock<T>`

##### `impl<T: ?Sized + Send + Sync> Sync for ShardedLock<T>`

##### `impl<T: ?Sized> UnwindSafe for ShardedLock<T>`

### `ShardedLockReadGuard<'a, T: ?Sized>`

```rust
struct ShardedLockReadGuard<'a, T: ?Sized> {
    lock: &'a ShardedLock<T>,
    _guard: std::sync::RwLockReadGuard<'a, ()>,
    _marker: std::marker::PhantomData<std::sync::RwLockReadGuard<'a, T>>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:486-490`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L486-L490)*

A guard used to release the shared read access of a [`ShardedLock`](#shardedlock) when dropped.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for ShardedLockReadGuard<'_, T>`

- <span id="shardedlockreadguard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized> Deref for ShardedLockReadGuard<'_, T>`

- <span id="shardedlockreadguard-deref-type-target"></span>`type Target = T`

- <span id="shardedlockreadguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized + fmt::Display> Display for ShardedLockReadGuard<'_, T>`

- <span id="shardedlockreadguard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Receiver for ShardedLockReadGuard<'a, T>`

- <span id="shardedlockreadguard-receiver-type-target"></span>`type Target = T`

##### `impl<T: ?Sized + Sync> Sync for ShardedLockReadGuard<'_, T>`

##### `impl<T> ToString for ShardedLockReadGuard<'a, T>`

- <span id="shardedlockreadguard-to-string"></span>`fn to_string(&self) -> String`

### `ShardedLockWriteGuard<'a, T: ?Sized>`

```rust
struct ShardedLockWriteGuard<'a, T: ?Sized> {
    lock: &'a ShardedLock<T>,
    _marker: std::marker::PhantomData<std::sync::RwLockWriteGuard<'a, T>>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:518-521`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L518-L521)*

A guard used to release the exclusive write access of a [`ShardedLock`](#shardedlock) when dropped.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for ShardedLockWriteGuard<'_, T>`

- <span id="shardedlockwriteguard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized> Deref for ShardedLockWriteGuard<'_, T>`

- <span id="shardedlockwriteguard-deref-type-target"></span>`type Target = T`

- <span id="shardedlockwriteguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized> DerefMut for ShardedLockWriteGuard<'_, T>`

- <span id="shardedlockwriteguard-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + fmt::Display> Display for ShardedLockWriteGuard<'_, T>`

- <span id="shardedlockwriteguard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized> Drop for ShardedLockWriteGuard<'_, T>`

- <span id="shardedlockwriteguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Receiver for ShardedLockWriteGuard<'a, T>`

- <span id="shardedlockwriteguard-receiver-type-target"></span>`type Target = T`

##### `impl<T: ?Sized + Sync> Sync for ShardedLockWriteGuard<'_, T>`

##### `impl<T> ToString for ShardedLockWriteGuard<'a, T>`

- <span id="shardedlockwriteguard-to-string"></span>`fn to_string(&self) -> String`

### `ThreadIndices`

```rust
struct ThreadIndices {
    mapping: std::collections::HashMap<std::thread::ThreadId, usize>,
    free_list: std::vec::Vec<usize>,
    next_index: usize,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:579-588`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L579-L588)*

The global registry keeping track of registered threads and indices.

#### Fields

- **`mapping`**: `std::collections::HashMap<std::thread::ThreadId, usize>`

  Mapping from `ThreadId` to thread index.

- **`free_list`**: `std::vec::Vec<usize>`

  A list of free indices.

- **`next_index`**: `usize`

  The next index to allocate if the free list is empty.

### `Registration`

```rust
struct Registration {
    index: usize,
    thread_id: std::thread::ThreadId,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:605-608`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L605-L608)*

A registration of a thread with an index.

When dropped, unregisters the thread and frees the reserved index.

#### Trait Implementations

##### `impl Drop for Registration`

- <span id="registration-drop"></span>`fn drop(&mut self)`

## Functions

### `current_index`

```rust
fn current_index() -> Option<usize>
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:574-576`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L574-L576)*

Returns a `usize` that identifies the current thread.

Each thread is associated with an 'index'. While there are no particular guarantees, indices
usually tend to be consecutive numbers between 0 and the number of running threads.

Since this function accesses TLS, `None` might be returned if the current thread's TLS is
tearing down.

### `thread_indices`

```rust
fn thread_indices() -> &'static std::sync::Mutex<ThreadIndices>
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:590-600`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L590-L600)*

## Constants

### `NUM_SHARDS`
```rust
const NUM_SHARDS: usize = 8usize;
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:18`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L18)*

The number of shards per sharded lock. Must be a power of two.

### `REGISTRATION`
```rust
const REGISTRATION: thread::LocalKey<Registration>;
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:618-638`](../../../../.source_1765210505/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L618-L638)*

