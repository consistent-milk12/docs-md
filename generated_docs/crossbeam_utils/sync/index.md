*[crossbeam_utils](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread synchronization primitives.

* [`Parker`](parker/index.md), a thread parking primitive.
* [`ShardedLock`](sharded_lock/index.md), a sharded reader-writer lock with fast concurrent reads.
* [`WaitGroup`](wait_group/index.md), for synchronizing the beginning or end of some computation.

## Contents

- [Modules](#modules)
  - [`once_lock`](#once-lock)
  - [`parker`](#parker)
  - [`sharded_lock`](#sharded-lock)
  - [`wait_group`](#wait-group)
- [Structs](#structs)
  - [`Parker`](#parker)
  - [`Unparker`](#unparker)
  - [`ShardedLock`](#shardedlock)
  - [`ShardedLockReadGuard`](#shardedlockreadguard)
  - [`ShardedLockWriteGuard`](#shardedlockwriteguard)
  - [`WaitGroup`](#waitgroup)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`once_lock`](#once-lock) | mod |  |
| [`parker`](#parker) | mod |  |
| [`sharded_lock`](#sharded-lock) | mod |  |
| [`wait_group`](#wait-group) | mod |  |
| [`Parker`](#parker) | struct |  |
| [`Unparker`](#unparker) | struct |  |
| [`ShardedLock`](#shardedlock) | struct |  |
| [`ShardedLockReadGuard`](#shardedlockreadguard) | struct |  |
| [`ShardedLockWriteGuard`](#shardedlockwriteguard) | struct |  |
| [`WaitGroup`](#waitgroup) | struct |  |

## Modules

- [`once_lock`](once_lock/index.md)
- [`parker`](parker/index.md)
- [`sharded_lock`](sharded_lock/index.md)
- [`wait_group`](wait_group/index.md)

## Structs

### `Parker`

```rust
struct Parker {
    unparker: Unparker,
    _marker: std::marker::PhantomData<*const ()>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:53-56`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L53-L56)*

A thread parking primitive.

Conceptually, each `Parker` has an associated token which is initially not present:

* The `park` method blocks the current thread unless or until the token is available, at
  which point it automatically consumes the token.

* The `park_timeout` and `park_deadline` methods work the same as `park`, but block for
  a specified maximum time.

* The `unpark` method atomically makes the token available if it wasn't already. Because the
  token is initially absent, `unpark` followed by `park` will result in the second call
  returning immediately.

In other words, each `Parker` acts a bit like a spinlock that can be locked and unlocked using
`park` and `unpark`.

# Examples

```rust
use std::thread;
use std::time::Duration;
use crossbeam_utils::sync::Parker;

let p = Parker::new();
let u = p.unparker().clone();

// Make the token available.
u.unpark();
// Wakes up immediately and consumes the token.
p.park();

thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    u.unpark();
});

// Wakes up when `u.unpark()` provides the token.
p.park();
std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
```





#### Implementations

- <span id="parker-new"></span>`fn new() -> Parker` — [`Parker`](parker/index.md#parker)

- <span id="parker-park"></span>`fn park(&self)`

- <span id="parker-park-timeout"></span>`fn park_timeout(&self, timeout: Duration)`

- <span id="parker-park-deadline"></span>`fn park_deadline(&self, deadline: Instant)`

- <span id="parker-unparker"></span>`fn unparker(&self) -> &Unparker` — [`Unparker`](parker/index.md#unparker)

- <span id="parker-into-raw"></span>`fn into_raw(this: Parker) -> *const ()` — [`Parker`](parker/index.md#parker)

- <span id="parker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Parker` — [`Parker`](parker/index.md#parker)

#### Trait Implementations

##### `impl Debug for Parker`

- <span id="parker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parker`

- <span id="parker-default"></span>`fn default() -> Self`

##### `impl Send for Parker`

### `Unparker`

```rust
struct Unparker {
    inner: std::sync::Arc<Inner>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/parker.rs:217-219`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/parker.rs#L217-L219)*

Unparks a thread parked by the associated [`Parker`](parker/index.md).

#### Implementations

- <span id="unparker-unpark"></span>`fn unpark(&self)`

- <span id="unparker-into-raw"></span>`fn into_raw(this: Unparker) -> *const ()` — [`Unparker`](parker/index.md#unparker)

- <span id="unparker-from-raw"></span>`unsafe fn from_raw(ptr: *const ()) -> Unparker` — [`Unparker`](parker/index.md#unparker)

#### Trait Implementations

##### `impl Clone for Unparker`

- <span id="unparker-clone"></span>`fn clone(&self) -> Unparker` — [`Unparker`](parker/index.md#unparker)

##### `impl Debug for Unparker`

- <span id="unparker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Send for Unparker`

##### `impl Sync for Unparker`

### `ShardedLock<T: ?Sized>`

```rust
struct ShardedLock<T: ?Sized> {
    shards: std::boxed::Box<[crate::CachePadded<Shard>]>,
    value: std::cell::UnsafeCell<T>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:78-84`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L78-L84)*

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

- <span id="shardedlock-new"></span>`fn new(value: T) -> ShardedLock<T>` — [`ShardedLock`](sharded_lock/index.md#shardedlock)

- <span id="shardedlock-into-inner"></span>`fn into_inner(self) -> LockResult<T>`

#### Trait Implementations

##### `impl<T: ?Sized + fmt::Debug> Debug for ShardedLock<T>`

- <span id="shardedlock-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for ShardedLock<T>`

- <span id="shardedlock-default"></span>`fn default() -> ShardedLock<T>` — [`ShardedLock`](sharded_lock/index.md#shardedlock)

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

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:486-490`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L486-L490)*

A guard used to release the shared read access of a [`ShardedLock`](sharded_lock/index.md) when dropped.

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

*Defined in [`crossbeam-utils-0.8.21/src/sync/sharded_lock.rs:518-521`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs#L518-L521)*

A guard used to release the exclusive write access of a [`ShardedLock`](sharded_lock/index.md) when dropped.

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

### `WaitGroup`

```rust
struct WaitGroup {
    inner: std::sync::Arc<Inner>,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/sync/wait_group.rs:46-48`](../../../.source_1765521767/crossbeam-utils-0.8.21/src/sync/wait_group.rs#L46-L48)*

Enables threads to synchronize the beginning or end of some computation.

# Wait groups vs barriers

`WaitGroup` is very similar to `Barrier`, but there are a few differences:

* `Barrier` needs to know the number of threads at construction, while `WaitGroup` is cloned to
  register more threads.

* A `Barrier` can be reused even after all threads have synchronized, while a `WaitGroup`
  synchronizes threads only once.

* All threads wait for others to reach the `Barrier`. With `WaitGroup`, each thread can choose
  to either wait for other threads or to continue without blocking.

# Examples

```rust
use crossbeam_utils::sync::WaitGroup;
use std::thread;

// Create a new wait group.
let wg = WaitGroup::new();

for _ in 0..4 {
    // Create another reference to the wait group.
    let wg = wg.clone();

    thread::spawn(move || {
        // Do some work.

        // Drop the reference to the wait group.
        drop(wg);
    });
}

// Block until all threads have finished their work.
wg.wait();
std::thread::sleep(std::time::Duration::from_millis(500)); // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371
```


#### Implementations

- <span id="waitgroup-new"></span>`fn new() -> Self`

- <span id="waitgroup-wait"></span>`fn wait(self)`

#### Trait Implementations

##### `impl Clone for WaitGroup`

- <span id="waitgroup-clone"></span>`fn clone(&self) -> WaitGroup` — [`WaitGroup`](wait_group/index.md#waitgroup)

##### `impl Debug for WaitGroup`

- <span id="waitgroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WaitGroup`

- <span id="waitgroup-default"></span>`fn default() -> Self`

##### `impl Drop for WaitGroup`

- <span id="waitgroup-drop"></span>`fn drop(&mut self)`

