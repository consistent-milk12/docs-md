*[crossbeam_utils](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread synchronization primitives.

* [`Parker`](parker/index.md), a thread parking primitive.
* [`ShardedLock`](sharded_lock/index.md), a sharded reader-writer lock with fast concurrent reads.
* [`WaitGroup`](wait_group/index.md), for synchronizing the beginning or end of some computation.

## Structs

### `Parker`

```rust
struct Parker {
    unparker: Unparker,
    _marker: std::marker::PhantomData<*const ()>,
}
```

A thread parking primitive.

Conceptually, each `Parker` has an associated token which is initially not present:

* The [`park`](#park) method blocks the current thread unless or until the token is available, at
  which point it automatically consumes the token.

* The [`park_timeout`](#park-timeout) and [`park_deadline`](#park-deadline) methods work the same as [`park`](#park), but block for
  a specified maximum time.

* The [`unpark`](#unpark) method atomically makes the token available if it wasn't already. Because the
  token is initially absent, [`unpark`](#unpark) followed by [`park`](#park) will result in the second call
  returning immediately.

In other words, each `Parker` acts a bit like a spinlock that can be locked and unlocked using
[`park`](#park) and [`unpark`](#unpark).

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

- `fn new() -> Parker` — [`Parker`](../../sync/parker/index.md)

- `fn park(self: &Self)`

- `fn park_timeout(self: &Self, timeout: Duration)`

- `fn park_deadline(self: &Self, deadline: Instant)`

- `fn unparker(self: &Self) -> &Unparker` — [`Unparker`](../../sync/parker/index.md)

- `fn into_raw(this: Parker) -> *const ()` — [`Parker`](../../sync/parker/index.md)

- `unsafe fn from_raw(ptr: *const ()) -> Parker` — [`Parker`](../../sync/parker/index.md)

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl Send`

### `Unparker`

```rust
struct Unparker {
    inner: std::sync::Arc<Inner>,
}
```

Unparks a thread parked by the associated [`Parker`](parker/index.md).

#### Implementations

- `fn unpark(self: &Self)`

- `fn into_raw(this: Unparker) -> *const ()` — [`Unparker`](../../sync/parker/index.md)

- `unsafe fn from_raw(ptr: *const ()) -> Unparker` — [`Unparker`](../../sync/parker/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Unparker` — [`Unparker`](../../sync/parker/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Send`

##### `impl Sync`

### `ShardedLock<T: ?Sized>`

```rust
struct ShardedLock<T: ?Sized> {
    shards: std::boxed::Box<[crate::CachePadded<Shard>]>,
    value: std::cell::UnsafeCell<T>,
}
```

A sharded reader-writer lock.

This lock is equivalent to [`RwLock`](#rwlock), except read operations are faster and write operations
are slower.

A `ShardedLock` is internally made of a list of *shards*, each being a [`RwLock`](#rwlock) occupying a
single cache line. Read operations will pick one of the shards depending on the current thread
and lock it. Write operations need to lock all shards in succession.

By splitting the lock into shards, concurrent read operations will in most cases choose
different shards and thus update different cache lines, which is good for scalability. However,
write operations need to do more work and are therefore slower than usual.

The priority policy of the lock is dependent on the underlying operating system's
implementation, and this type does not guarantee that any particular policy will be used.

# Poisoning

A `ShardedLock`, like [`RwLock`](#rwlock), will become poisoned on a panic. Note that it may only be
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

- `fn is_poisoned(self: &Self) -> bool`

- `fn get_mut(self: &mut Self) -> LockResult<&mut T>`

- `fn try_read(self: &Self) -> TryLockResult<ShardedLockReadGuard<'_, T>>` — [`ShardedLockReadGuard`](../../sync/sharded_lock/index.md)

- `fn read(self: &Self) -> LockResult<ShardedLockReadGuard<'_, T>>` — [`ShardedLockReadGuard`](../../sync/sharded_lock/index.md)

- `fn try_write(self: &Self) -> TryLockResult<ShardedLockWriteGuard<'_, T>>` — [`ShardedLockWriteGuard`](../../sync/sharded_lock/index.md)

- `fn write(self: &Self) -> LockResult<ShardedLockWriteGuard<'_, T>>` — [`ShardedLockWriteGuard`](../../sync/sharded_lock/index.md)

#### Trait Implementations

##### `impl Debug<T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> ShardedLock<T>` — [`ShardedLock`](../../sync/sharded_lock/index.md)

##### `impl RefUnwindSafe<T: ?Sized>`

##### `impl Send<T: ?Sized + Send>`

##### `impl Sync<T: ?Sized + Send + Sync>`

##### `impl UnwindSafe<T: ?Sized>`

### `ShardedLockReadGuard<'a, T: ?Sized>`

```rust
struct ShardedLockReadGuard<'a, T: ?Sized> {
    lock: &'a ShardedLock<T>,
    _guard: std::sync::RwLockReadGuard<'a, ()>,
    _marker: std::marker::PhantomData<std::sync::RwLockReadGuard<'a, T>>,
}
```

A guard used to release the shared read access of a [`ShardedLock`](sharded_lock/index.md) when dropped.

#### Trait Implementations

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref<T: ?Sized>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl Display<T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sync<T: ?Sized + Sync>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `ShardedLockWriteGuard<'a, T: ?Sized>`

```rust
struct ShardedLockWriteGuard<'a, T: ?Sized> {
    lock: &'a ShardedLock<T>,
    _marker: std::marker::PhantomData<std::sync::RwLockWriteGuard<'a, T>>,
}
```

A guard used to release the exclusive write access of a [`ShardedLock`](sharded_lock/index.md) when dropped.

#### Trait Implementations

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref<T: ?Sized>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl DerefMut<T: ?Sized>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl Display<T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop<T: ?Sized>`

- `fn drop(self: &mut Self)`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Sync<T: ?Sized + Sync>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `WaitGroup`

```rust
struct WaitGroup {
    inner: std::sync::Arc<Inner>,
}
```

Enables threads to synchronize the beginning or end of some computation.

# Wait groups vs barriers

`WaitGroup` is very similar to [`Barrier`](#barrier), but there are a few differences:

* [`Barrier`](#barrier) needs to know the number of threads at construction, while `WaitGroup` is cloned to
  register more threads.

* A [`Barrier`](#barrier) can be reused even after all threads have synchronized, while a `WaitGroup`
  synchronizes threads only once.

* All threads wait for others to reach the [`Barrier`](#barrier). With `WaitGroup`, each thread can choose
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

- `fn new() -> Self`

- `fn wait(self: Self)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> WaitGroup` — [`WaitGroup`](../../sync/wait_group/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl Drop`

- `fn drop(self: &mut Self)`

