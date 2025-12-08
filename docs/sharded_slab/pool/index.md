*[sharded_slab](../index.md) / [pool](index.md)*

---

# Module `pool`

A lock-free concurrent object pool.

See the [`Pool` type's documentation][pool](#pool) for details on the object pool API and how
it differs from the [`Slab`](../index.md) API.



## Structs

### `Pool<T, C>`

```rust
struct Pool<T, C>
where
    T: Clear + Default,
    C: cfg::Config {
    shards: shard::Array<T, C>,
    _cfg: std::marker::PhantomData<C>,
}
```

A lock-free concurrent object pool.

Slabs provide pre-allocated storage for many instances of a single type. But, when working with
heap allocated objects, the advantages of a slab are lost, as the memory allocated for the
object is freed when the object is removed from the slab. With a pool, we can instead reuse
this memory for objects being added to the pool in the future, therefore reducing memory
fragmentation and avoiding additional allocations.

This type implements a lock-free concurrent pool, indexed by `usize`s. The items stored in this
type need to implement [`Clear`](../clear/index.md) and `Default`.

The `Pool` type shares similar semantics to [`Slab`](../index.md) when it comes to sharing across threads
and storing mutable shared data. The biggest difference is there are no `Slab::insert` and
`Slab::take` analouges for the `Pool` type. Instead new items are added to the pool by using
the `Pool::create` method, and marked for clearing by the `Pool::clear` method.

# Examples

Add an entry to the pool, returning an index:
```rust
use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let key = pool.create_with(|item| item.push_str("hello world")).unwrap();
assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Create a new pooled item, returning a guard that allows mutable access:
```rust
use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let mut guard = pool.create().unwrap();
let key = guard.key();
guard.push_str("hello world");

drop(guard); // release the guard, allowing immutable access.
assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Pool entries can be cleared by calling `Pool::clear`. This marks the entry to
be cleared when the guards referencing to it are dropped.
```rust
use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Mark this entry to be cleared.
pool.clear(key);

// The cleared entry is no longer available in the pool
assert!(pool.get(key).is_none());
```
# Configuration

Both `Pool` and [`Slab`](../index.md) share the same configuration mechanism. See [crate level documentation][config-doc]
for more details.








#### Implementations

- `const USED_BITS: usize`

- `fn create(self: &Self) -> Option<RefMut<'_, T, C>>` — [`RefMut`](#refmut)

- `fn create_owned(self: Arc<Self>) -> Option<OwnedRefMut<T, C>>` — [`OwnedRefMut`](#ownedrefmut)

- `fn create_with(self: &Self, init: impl FnOnce(&mut T)) -> Option<usize>`

- `fn get(self: &Self, key: usize) -> Option<Ref<'_, T, C>>` — [`Ref`](#ref)

- `fn get_owned(self: Arc<Self>, key: usize) -> Option<OwnedRef<T, C>>` — [`OwnedRef`](#ownedref)

- `fn clear(self: &Self, key: usize) -> bool`

#### Trait Implementations

##### `impl<T, C> Debug for Pool<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Pool<T>`

- `fn default() -> Self`

##### `impl<T, C> Send for Pool<T, C>`

##### `impl<T, C> Sync for Pool<T, C>`

### `Ref<'a, T, C>`

```rust
struct Ref<'a, T, C>
where
    T: Clear + Default,
    C: cfg::Config {
    inner: page::slot::Guard<T, C>,
    shard: &'a shard::Shard<T, C>,
    key: usize,
}
```

A guard that allows access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard references is
currently being accessed. If the item is removed from the pool while the guard exists, the
removal will be deferred until all guards are dropped.

#### Implementations

- `fn key(self: &Self) -> usize`

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<'a, T, C> Debug for Ref<'a, T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, C> Deref for Ref<'a, T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'a, T, C> Drop for Ref<'a, T, C>`

- `fn drop(self: &mut Self)`

##### `impl<'a, T, C> PartialEq for Ref<'a, T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for Ref<'a, T, C>`

- `type Target = T`

### `RefMut<'a, T, C>`

```rust
struct RefMut<'a, T, C>
where
    T: Clear + Default,
    C: cfg::Config {
    inner: page::slot::InitGuard<T, C>,
    shard: &'a shard::Shard<T, C>,
    key: usize,
}
```

A guard that allows exclusive mutable access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard
references is currently being accessed. If the item is removed from the pool
while a guard exists, the removal will be deferred until the guard is
dropped. The slot cannot be accessed by other threads while it is accessed
mutably.

#### Implementations

- `fn key(self: &Self) -> usize`

- `fn downgrade(self: Self) -> Ref<'a, T, C>` — [`Ref`](#ref)

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<'a, T, C> Debug for RefMut<'a, T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, C> Deref for RefMut<'a, T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'a, T, C> DerefMut for RefMut<'a, T, C>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<'a, T, C> Drop for RefMut<'a, T, C>`

- `fn drop(self: &mut Self)`

##### `impl<'a, T, C> PartialEq for RefMut<'a, T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for RefMut<'a, T, C>`

- `type Target = T`

### `OwnedRef<T, C>`

```rust
struct OwnedRef<T, C>
where
    T: Clear + Default,
    C: cfg::Config {
    inner: page::slot::Guard<T, C>,
    pool: std::sync::Arc<Pool<T, C>>,
    key: usize,
}
```

An owned guard that allows shared immutable access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard references is
currently being accessed. If the item is removed from the pool while the guard exists, the
removal will be deferred until all guards are dropped.

Unlike [`Ref`](#ref), which borrows the pool, an `OwnedRef` clones the `Arc`
around the pool. Therefore, it keeps the pool from being dropped until all
such guards have been dropped. This means that an `OwnedRef` may be held for
an arbitrary lifetime.


# Examples

```rust
use sharded_slab::Pool;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let value = pool.clone().get_owned(key).unwrap();

// Now, the original `Arc` clone of the pool may be dropped, but the
// returned `OwnedRef` can still access the value.
assert_eq!(value, String::from("hello world"));
```

Unlike [`Ref`](#ref), an `OwnedRef` may be stored in a struct which must live
for the `'static` lifetime:

```rust
use sharded_slab::Pool;
use sharded_slab::pool::OwnedRef;
use std::sync::Arc;

pub struct MyStruct {
    pool_ref: OwnedRef<String>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let pool_ref = pool.clone().get_owned(key).unwrap();
let my_struct = MyStruct {
    pool_ref,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedRef`s may be sent between threads:

```rust
use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let value = pool.clone().get_owned(key).unwrap();

thread::spawn(move || {
    assert_eq!(value, String::from("hello world"));
    // ...
}).join().unwrap();
```


#### Implementations

- `fn key(self: &Self) -> usize`

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T, C> Debug for OwnedRef<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, C> Deref for OwnedRef<T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<T, C> Drop for OwnedRef<T, C>`

- `fn drop(self: &mut Self)`

##### `impl<T, C> PartialEq for OwnedRef<T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for OwnedRef<T, C>`

- `type Target = T`

##### `impl<T, C> Send for OwnedRef<T, C>`

##### `impl<T, C> Sync for OwnedRef<T, C>`

### `OwnedRefMut<T, C>`

```rust
struct OwnedRefMut<T, C>
where
    T: Clear + Default,
    C: cfg::Config {
    inner: page::slot::InitGuard<T, C>,
    pool: std::sync::Arc<Pool<T, C>>,
    key: usize,
}
```

An owned guard that allows exclusive, mutable access to an object in a pool.

An `OwnedRefMut<T>` functions more or less identically to an owned
`Box<T>`: it can be passed to functions, stored in structure fields, and
borrowed mutably or immutably, and can be owned for arbitrary lifetimes.
The difference is that, unlike a `Box<T>`, the memory allocation for the
`T` lives in the `Pool`; when an `OwnedRefMut` is created, it may reuse
memory that was allocated for a previous pooled object that has been
cleared. Additionally, the `OwnedRefMut` may be [downgraded] to an
[`OwnedRef`](#ownedref) which may be shared freely, essentially turning the `Box`
into an `Arc`.

This is returned by `Pool::create_owned`.

While the guard exists, it indicates to the pool that the item the guard
references is currently being accessed. If the item is removed from the pool
while the guard exists, theremoval will be deferred until all guards are
dropped.

Unlike [`RefMut`](#refmut), which borrows the pool, an `OwnedRefMut` clones the `Arc`
around the pool. Therefore, it keeps the pool from being dropped until all
such guards have been dropped. This means that an `OwnedRefMut` may be held for
an arbitrary lifetime.

# Examples

```rust
use sharded_slab::Pool;
use std::thread;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new pooled item, returning an owned guard that allows mutable
// access to the new item.
let mut item = pool.clone().create_owned().unwrap();
// Return a key that allows indexing the created item once the guard
// has been dropped.
let key = item.key();

// Mutate the item.
item.push_str("Hello");
// Drop the guard, releasing mutable access to the new item.
drop(item);

/// Other threads may now (immutably) access the item using the returned key.
thread::spawn(move || {
   assert_eq!(pool.get(key).unwrap(), String::from("Hello"));
}).join().unwrap();
```

```rust
use sharded_slab::Pool;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new item, returning an owned, mutable guard.
let mut value = pool.clone().create_owned().unwrap();

// Now, the original `Arc` clone of the pool may be dropped, but the
// returned `OwnedRefMut` can still access the value.
drop(pool);

value.push_str("hello world");
assert_eq!(value, String::from("hello world"));
```

Unlike [`RefMut`](#refmut), an `OwnedRefMut` may be stored in a struct which must live
for the `'static` lifetime:

```rust
use sharded_slab::Pool;
use sharded_slab::pool::OwnedRefMut;
use std::sync::Arc;

pub struct MyStruct {
    pool_ref: OwnedRefMut<String>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new item, returning a mutable owned reference.
let pool_ref = pool.clone().create_owned().unwrap();

let my_struct = MyStruct {
    pool_ref,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedRefMut`s may be sent between threads:

```rust
use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

let mut value = pool.clone().create_owned().unwrap();
let key = value.key();

thread::spawn(move || {
    value.push_str("hello world");
    // ...
}).join().unwrap();

// Once the `OwnedRefMut` has been dropped by the other thread, we may
// now access the value immutably on this thread.

assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Downgrading from a mutable to an immutable reference:

```rust
use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

let mut value = pool.clone().create_owned().unwrap();
let key = value.key();
value.push_str("hello world");

// Downgrade the mutable owned ref to an immutable owned ref.
let value = value.downgrade();

// Once the `OwnedRefMut` has been downgraded, other threads may
// immutably access the pooled value:
thread::spawn(move || {
    assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
}).join().unwrap();

// This thread can still access the pooled value through the
// immutable owned ref:
assert_eq!(value, String::from("hello world"));
```





#### Implementations

- `fn key(self: &Self) -> usize`

- `fn downgrade(self: Self) -> OwnedRef<T, C>` — [`OwnedRef`](#ownedref)

- `fn shard(self: &Self) -> Option<&Shard<T, C>>` — [`Shard`](../shard/index.md)

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T, C> Debug for OwnedRefMut<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, C> Deref for OwnedRefMut<T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<T, C> DerefMut for OwnedRefMut<T, C>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<T, C> Drop for OwnedRefMut<T, C>`

- `fn drop(self: &mut Self)`

##### `impl<T, C> PartialEq for OwnedRefMut<T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for OwnedRefMut<T, C>`

- `type Target = T`

##### `impl<T, C> Send for OwnedRefMut<T, C>`

##### `impl<T, C> Sync for OwnedRefMut<T, C>`

