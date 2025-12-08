# Crate `sharded_slab`

 A lock-free concurrent slab.

 Slabs provide pre-allocated storage for many instances of a single data
 type. When a large number of values of a single type are required,
 this can be more efficient than allocating each item individually. Since the
 allocated items are the same size, memory fragmentation is reduced, and
 creating and removing new items can be very cheap.

 This crate implements a lock-free concurrent slab, indexed by `usize`s.

 ## Usage

 First, add this to your `Cargo.toml`:

 ```toml
 sharded-slab = "0.1.1"
 ```

 This crate provides two  types, [`Slab`](#slab) and [`Pool`](pool/index.md), which provide
 slightly different APIs for using a sharded slab.

 [`Slab`](#slab) implements a slab for _storing_ small types, sharing them between
 threads, and accessing them by index. New entries are allocated by
 [inserting] data, moving it in by value. Similarly, entries may be
 deallocated by [taking] from the slab, moving the value out. This API is
 similar to a `Vec<Option<T>>`, but allowing lock-free concurrent insertion
 and removal.

 In contrast, the [`Pool`](pool/index.md) type provides an [object pool] style API for
 _reusing storage_. Rather than constructing values and moving them into the
 pool, as with [`Slab`](#slab), [allocating an entry][`create`](../fs_err/file/index.md) from the pool takes a
 closure that's provided with a mutable reference to initialize the entry in
 place. When entries are deallocated, they are [cleared] in place. Types
 which own a heap allocation can be cleared by dropping any _data_ they
 store, but retaining any previously-allocated capacity. This means that a
 [`Pool`](pool/index.md) may be used to reuse a set of existing heap allocations, reducing
 allocator load.





 # Examples

 Inserting an item into the slab, returning an index:
 ```rust
 use sharded_slab::Slab;
 let slab = Slab::new();

 let key = slab.insert("hello world").unwrap();
 assert_eq!(slab.get(key).unwrap(), "hello world");
 ```

 To share a slab across threads, it may be wrapped in an `Arc`:
 ```rust
 use sharded_slab::Slab;
 use std::sync::Arc;
 let slab = Arc::new(Slab::new());

 let slab2 = slab.clone();
 let thread2 = std::thread::spawn(move || {
     let key = slab2.insert("hello from thread two").unwrap();
     assert_eq!(slab2.get(key).unwrap(), "hello from thread two");
     key
 });

 let key1 = slab.insert("hello from thread one").unwrap();
 assert_eq!(slab.get(key1).unwrap(), "hello from thread one");

 // Wait for thread 2 to complete.
 let key2 = thread2.join().unwrap();

 // The item inserted by thread 2 remains in the slab.
 assert_eq!(slab.get(key2).unwrap(), "hello from thread two");
```

 If items in the slab must be mutated, a `Mutex` or `RwLock` may be used for
 each item, providing granular locking of items rather than of the slab:

 ```rust
 use sharded_slab::Slab;
 use std::sync::{Arc, Mutex};
 let slab = Arc::new(Slab::new());

 let key = slab.insert(Mutex::new(String::from("hello world"))).unwrap();

 let slab2 = slab.clone();
 let thread2 = std::thread::spawn(move || {
     let hello = slab2.get(key).expect("item missing");
     let mut hello = hello.lock().expect("mutex poisoned");
     *hello = String::from("hello everyone!");
 });

 thread2.join().unwrap();

 let hello = slab.get(key).expect("item missing");
 let mut hello = hello.lock().expect("mutex poisoned");
 assert_eq!(hello.as_str(), "hello everyone!");
 ```

 # Configuration

 For performance reasons, several values used by the slab are calculated as
 constants. In order to allow users to tune the slab's parameters, we provide
 a [`Config`](cfg/index.md) trait which defines these parameters as associated `consts`.
 The `Slab` type is generic over a `C: Config` parameter.

 # Comparison with Similar Crates

 - `slab`: Carl Lerche's `slab` crate provides a slab implementation with a
   similar API, implemented by storing all data in a single vector.

   Unlike `sharded_slab`, inserting and removing elements from the slab
   requires  mutable access. This means that if the slab is accessed
   concurrently by multiple threads, it is necessary for it to be protected
   by a `Mutex` or `RwLock`. Items may not be inserted or removed (or
   accessed, if a `Mutex` is used) concurrently, even when they are
   unrelated. In many cases, the lock can become a significant bottleneck. On
   the other hand, this crate allows separate indices in the slab to be
   accessed, inserted, and removed concurrently without requiring a global
   lock. Therefore, when the slab is shared across multiple threads, this
   crate offers significantly better performance than `slab`.

   However, the lock free slab introduces some additional constant-factor
   overhead. This means that in use-cases where a slab is _not_ shared by
   multiple threads and locking is not required, this crate will likely offer
   slightly worse performance.

   In summary: `sharded-slab` offers significantly improved performance in
   concurrent use-cases, while `slab` should be preferred in single-threaded
   use-cases.

 # Safety and Correctness

 Most implementations of lock-free data structures in Rust require some
 amount of unsafe code, and this crate is not an exception. In order to catch
 potential bugs in this unsafe code, we make use of `loom`, a
 permutation-testing tool for concurrent Rust programs. All `unsafe` blocks
 this crate occur in accesses to `loom` `UnsafeCell`s. This means that when
 those accesses occur in this crate's tests, `loom` will assert that they are
 valid under the C11 memory model across multiple permutations of concurrent
 executions of those tests.

 In order to guard against the [ABA problem][aba], this crate makes use of
 _generational indices_. Each slot in the slab tracks a generation counter
 which is incremented every time a value is inserted into that slot, and the
 indices returned by `Slab::insert` include the generation of the slot when
 the value was inserted, packed into the high-order bits of the index. This
 ensures that if a value is inserted, removed,  and a new value is inserted
 into the same slot in the slab, the key returned by the first call to
 `insert` will not map to the new value.

 Since a fixed number of bits are set aside to use for storing the generation
 counter, the counter will wrap  around after being incremented a number of
 times. To avoid situations where a returned index lives long enough to see the
 generation counter wrap around to the same value, it is good to be fairly
 generous when configuring the allocation of index bits.



 # Performance

 These graphs were produced by [benchmarks] of the sharded slab implementation,
 using the `criterion` crate.

 The first shows the results of a benchmark where an increasing number of
 items are inserted and then removed into a slab concurrently by five
 threads. It compares the performance of the sharded slab implementation
 with a `RwLock<slab::Slab>`:

 <img width="1124" alt="Screen Shot 2019-10-01 at 5 09 49 PM" src="https://user-images.githubusercontent.com/2796466/66078398-cd6c9f80-e516-11e9-9923-0ed6292e8498.png">

 The second graph shows the results of a benchmark where an increasing
 number of items are inserted and then removed by a _single_ thread. It
 compares the performance of the sharded slab implementation with an
 `RwLock<slab::Slab>` and a `mut slab::Slab`.

 <img width="925" alt="Screen Shot 2019-10-01 at 5 13 45 PM" src="https://user-images.githubusercontent.com/2796466/66078469-f0974f00-e516-11e9-95b5-f65f0aa7e494.png">

 These benchmarks demonstrate that, while the sharded approach introduces
 a small constant-factor overhead, it offers significantly better
 performance across concurrent accesses.


 # Implementation Notes

 See [this page](crate::implementation) for details on this crate's design
 and implementation.


## Modules

- [`implementation`](implementation/index.md) - Notes on `sharded-slab`'s implementation and design.
- [`pool`](pool/index.md) - A lock-free concurrent object pool.

## Structs

### `DefaultConfig`

```rust
struct DefaultConfig {
    _p: (),
}
```

Default slab configuration values.

#### Trait Implementations

##### `impl Clone for DefaultConfig`

- `fn clone(self: &Self) -> DefaultConfig` — [`DefaultConfig`](cfg/index.md)

##### `impl Config for DefaultConfig`

- `const INITIAL_PAGE_SIZE: usize`

- `const MAX_THREADS: usize`

- `const MAX_PAGES: usize`

##### `impl Copy for DefaultConfig`

##### `impl Debug for DefaultConfig`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UniqueIter<'a, T, C: cfg::Config>`

```rust
struct UniqueIter<'a, T, C: cfg::Config> {
    shards: shard::IterMut<'a, Option<T>, C>,
    pages: slice::Iter<'a, page::Shared<Option<T>, C>>,
    slots: Option<std::iter::FilterMap<std::slice::Iter<'a, self::slot::Slot<Option<T>, C>>, fn(&'a self::slot::Slot<Option<T>, C>) -> Option<&'a T>>>,
}
```

An exclusive fused iterator over the items in a [`Slab`](crate::Slab).

#### Trait Implementations

##### `impl<'a, T: $crate::fmt::Debug, C: $crate::fmt::Debug + cfg::Config> Debug for UniqueIter<'a, T, C>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T, C: cfg::Config> FusedIterator for UniqueIter<'_, T, C>`

##### `impl<I> IntoIterator for UniqueIter<'a, T, C>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, C: cfg::Config> Iterator for UniqueIter<'a, T, C>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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
type need to implement [`Clear`](clear/index.md) and `Default`.

The `Pool` type shares similar semantics to [`Slab`](#slab) when it comes to sharing across threads
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

Both `Pool` and [`Slab`](#slab) share the same configuration mechanism. See [crate level documentation][config-doc]
for more details.








#### Implementations

- `const USED_BITS: usize`

- `fn create(self: &Self) -> Option<RefMut<'_, T, C>>` — [`RefMut`](pool/index.md)

- `fn create_owned(self: Arc<Self>) -> Option<OwnedRefMut<T, C>>` — [`OwnedRefMut`](pool/index.md)

- `fn create_with(self: &Self, init: impl FnOnce(&mut T)) -> Option<usize>`

- `fn get(self: &Self, key: usize) -> Option<Ref<'_, T, C>>` — [`Ref`](pool/index.md)

- `fn get_owned(self: Arc<Self>, key: usize) -> Option<OwnedRef<T, C>>` — [`OwnedRef`](pool/index.md)

- `fn clear(self: &Self, key: usize) -> bool`

#### Trait Implementations

##### `impl<T, C> Debug for Pool<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Pool<T>`

- `fn default() -> Self`

##### `impl<T, C> Send for Pool<T, C>`

##### `impl<T, C> Sync for Pool<T, C>`

### `Slab<T, C: cfg::Config>`

```rust
struct Slab<T, C: cfg::Config> {
    shards: shard::Array<Option<T>, C>,
    _cfg: std::marker::PhantomData<C>,
}
```

A sharded slab.

See the [crate-level documentation](crate) for details on using this type.

#### Implementations

- `fn new() -> Self`

- `fn new_with_config<C: cfg::Config>() -> Slab<T, C>` — [`Slab`](#slab)

#### Trait Implementations

##### `impl<T: fmt::Debug, C: cfg::Config> Debug for Slab<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Slab<T>`

- `fn default() -> Self`

##### `impl<T: Send, C: cfg::Config> Send for Slab<T, C>`

##### `impl<T: Sync, C: cfg::Config> Sync for Slab<T, C>`

### `Entry<'a, T, C: cfg::Config>`

```rust
struct Entry<'a, T, C: cfg::Config> {
    inner: page::slot::Guard<Option<T>, C>,
    value: ptr::NonNull<T>,
    shard: &'a shard::Shard<Option<T>, C>,
    key: usize,
}
```

A handle that allows access to an occupied entry in a [`Slab`](#slab).

While the guard exists, it indicates to the slab that the item the guard
references is currently being accessed. If the item is removed from the slab
while a guard exists, the removal will be deferred until all guards are
dropped.

#### Implementations

- `fn key(self: &Self) -> usize`

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<'a, T, C> Debug for Entry<'a, T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, C: cfg::Config> Deref for Entry<'a, T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'a, T, C: cfg::Config> Drop for Entry<'a, T, C>`

- `fn drop(self: &mut Self)`

##### `impl<'a, T, C> PartialEq for Entry<'a, T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for Entry<'a, T, C>`

- `type Target = T`

### `VacantEntry<'a, T, C: cfg::Config>`

```rust
struct VacantEntry<'a, T, C: cfg::Config> {
    inner: page::slot::InitGuard<Option<T>, C>,
    key: usize,
    _lt: std::marker::PhantomData<&'a ()>,
}
```

A handle to a vacant entry in a [`Slab`](#slab).

`VacantEntry` allows constructing values with the key that they will be
assigned to.

# Examples

```rust
use sharded_slab::Slab;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry().unwrap();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab.get(hello).unwrap().0);
assert_eq!("hello", slab.get(hello).unwrap().1);
```

#### Implementations

- `fn insert(self: Self, val: T)`

- `fn key(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'a, T: $crate::fmt::Debug, C: $crate::fmt::Debug + cfg::Config> Debug for VacantEntry<'a, T, C>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OwnedEntry<T, C>`

```rust
struct OwnedEntry<T, C>
where
    C: cfg::Config {
    inner: page::slot::Guard<Option<T>, C>,
    value: ptr::NonNull<T>,
    slab: std::sync::Arc<Slab<T, C>>,
    key: usize,
}
```

An owned reference to an occupied entry in a [`Slab`](#slab).

While the guard exists, it indicates to the slab that the item the guard
references is currently being accessed. If the item is removed from the slab
while the guard exists, the  removal will be deferred until all guards are
dropped.

Unlike [`Entry`](#entry), which borrows the slab, an `OwnedEntry` clones the `Arc`
around the slab. Therefore, it keeps the slab from being dropped until all
such guards have been dropped. This means that an `OwnedEntry` may be held for
an arbitrary lifetime.

# Examples

```rust
use sharded_slab::Slab;
use std::sync::Arc;

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let value = slab.clone().get_owned(key).unwrap();

// Now, the original `Arc` clone of the slab may be dropped, but the
// returned `OwnedEntry` can still access the value.
assert_eq!(value, "hello world");
```

Unlike [`Entry`](#entry), an `OwnedEntry` may be stored in a struct which must live
for the `'static` lifetime:

```rust
use sharded_slab::Slab;
use sharded_slab::OwnedEntry;
use std::sync::Arc;

pub struct MyStruct {
    entry: OwnedEntry<&'static str>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let entry = slab.clone().get_owned(key).unwrap();
let my_struct = MyStruct {
    entry,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedEntry`s may be sent between threads:

```rust
use sharded_slab::Slab;
use std::{thread, sync::Arc};

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let value = slab.clone().get_owned(key).unwrap();

thread::spawn(move || {
    assert_eq!(value, "hello world");
    // ...
}).join().unwrap();
```



#### Implementations

- `fn key(self: &Self) -> usize`

- `fn value(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T, C> Debug for OwnedEntry<T, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, C> Deref for OwnedEntry<T, C>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<T, C> Drop for OwnedEntry<T, C>`

- `fn drop(self: &mut Self)`

##### `impl<T, C> PartialEq for OwnedEntry<T, C>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl<P, T> Receiver for OwnedEntry<T, C>`

- `type Target = T`

##### `impl<T, C> Send for OwnedEntry<T, C>`

##### `impl<T, C> Sync for OwnedEntry<T, C>`

## Traits

