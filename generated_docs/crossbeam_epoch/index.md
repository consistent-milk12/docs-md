# Crate `crossbeam_epoch`

Epoch-based memory reclamation.

An interesting problem concurrent collections deal with comes from the remove operation.
Suppose that a thread removes an element from a lock-free map, while another thread is reading
that same element at the same time. The first thread must wait until the second thread stops
reading the element. Only then it is safe to destruct it.

Programming languages that come with garbage collectors solve this problem trivially. The
garbage collector will destruct the removed element when no thread can hold a reference to it
anymore.

This crate implements a basic memory reclamation mechanism, which is based on epochs. When an
element gets removed from a concurrent collection, it is inserted into a pile of garbage and
marked with the current epoch. Every time a thread accesses a collection, it checks the current
epoch, attempts to increment it, and destructs some garbage that became so old that no thread
can be referencing it anymore.

That is the general mechanism behind epoch-based memory reclamation, but the details are a bit
more complicated. Anyhow, memory reclamation is designed to be fully automatic and something
users of concurrent collections don't have to worry much about.

# Pointers

Concurrent collections are built using atomic pointers. This module provides [`Atomic`](#atomic), which
is just a shared atomic pointer to a heap-allocated object. Loading an [`Atomic`](#atomic) yields a
[`Shared`](#shared), which is an epoch-protected pointer through which the loaded object can be safely
read.

# Pinning

Before an [`Atomic`](#atomic) can be loaded, a participant must be [`pin`](#pin)ned. By pinning a participant
we declare that any object that gets removed from now on must not be destructed just
yet. Garbage collection of newly removed objects is suspended until the participant gets
unpinned.

# Garbage

Objects that get removed from concurrent collections must be stashed away until all currently
pinned participants get unpinned. Such objects can be stored into a thread-local or global
storage, where they are kept until the right time for their destruction comes.

There is a global shared instance of garbage queue. You can [`defer`](Guard::defer) the execution of an
arbitrary function until the global epoch is advanced enough. Most notably, concurrent data
structures may defer the deallocation of an object.

# APIs

For majority of use cases, just use the default garbage collector by invoking [`pin`](#pin). If you
want to create your own garbage collector, use the [`Collector`](#collector) API.

## Contents

- [Modules](#modules)
  - [`primitive`](#primitive)
  - [`atomic`](#atomic)
  - [`collector`](#collector)
  - [`deferred`](#deferred)
  - [`epoch`](#epoch)
  - [`guard`](#guard)
  - [`internal`](#internal)
  - [`sync`](#sync)
  - [`default`](#default)
- [Structs](#structs)
  - [`Atomic`](#atomic)
  - [`CompareExchangeError`](#compareexchangeerror)
  - [`Owned`](#owned)
  - [`Shared`](#shared)
  - [`Collector`](#collector)
  - [`LocalHandle`](#localhandle)
  - [`Guard`](#guard)
- [Traits](#traits)
  - [`CompareAndSetOrdering`](#compareandsetordering)
  - [`Pointable`](#pointable)
  - [`Pointer`](#pointer)
- [Functions](#functions)
  - [`unprotected`](#unprotected)
  - [`default_collector`](#default_collector)
  - [`is_pinned`](#is_pinned)
  - [`pin`](#pin)
- [Type Aliases](#type-aliases)
  - [`CompareAndSetError`](#compareandseterror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`primitive`](#primitive) | mod |  |
| [`atomic`](#atomic) | mod |  |
| [`collector`](#collector) | mod |  |
| [`deferred`](#deferred) | mod |  |
| [`epoch`](#epoch) | mod | The global epoch |
| [`guard`](#guard) | mod |  |
| [`internal`](#internal) | mod | The global data and participant for garbage collection. |
| [`sync`](#sync) | mod | Synchronization primitives. |
| [`default`](#default) | mod | The default garbage collector. |
| [`Atomic`](#atomic) | struct |  |
| [`CompareExchangeError`](#compareexchangeerror) | struct |  |
| [`Owned`](#owned) | struct |  |
| [`Shared`](#shared) | struct |  |
| [`Collector`](#collector) | struct |  |
| [`LocalHandle`](#localhandle) | struct |  |
| [`Guard`](#guard) | struct |  |
| [`CompareAndSetOrdering`](#compareandsetordering) | trait |  |
| [`Pointable`](#pointable) | trait |  |
| [`Pointer`](#pointer) | trait |  |
| [`unprotected`](#unprotected) | fn |  |
| [`default_collector`](#default_collector) | fn |  |
| [`is_pinned`](#is_pinned) | fn |  |
| [`pin`](#pin) | fn |  |
| [`CompareAndSetError`](#compareandseterror) | type |  |

## Modules

- [`primitive`](primitive/index.md)
- [`atomic`](atomic/index.md)
- [`collector`](collector/index.md)
- [`deferred`](deferred/index.md)
- [`epoch`](epoch/index.md) — The global epoch
- [`guard`](guard/index.md)
- [`internal`](internal/index.md) — The global data and participant for garbage collection.
- [`sync`](sync/index.md) — Synchronization primitives.
- [`default`](default/index.md) — The default garbage collector.

## Structs

### `Atomic<T: ?Sized + Pointable>`

```rust
struct Atomic<T: ?Sized + Pointable> {
    data: core::sync::atomic::AtomicUsize,
    _marker: core::marker::PhantomData<*mut T>,
}
```

An atomic pointer that can be safely shared between threads.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address. For example, the tag for a pointer to a sized type `T`
should be less than `(1 << mem::align_of::<T>().trailing_zeros())`.

Any method that loads the pointer must be passed a reference to a [`Guard`](#guard).

Crossbeam supports dynamically sized types.  See [`Pointable`](#pointable) for details.

#### Implementations

- <span id="atomic-init"></span>`fn init(init: <T as >::Init) -> Atomic<T>` — [`Pointable`](#pointable), [`Atomic`](#atomic)

- <span id="atomic-from-usize"></span>`fn from_usize(data: usize) -> Self`

- <span id="atomic-null"></span>`const fn null() -> Atomic<T>` — [`Atomic`](#atomic)

- <span id="atomic-load"></span>`fn load<'g>(&self, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-load-consume"></span>`fn load_consume<'g>(&self, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-store"></span>`fn store<P: Pointer<T>>(&self, new: P, ord: Ordering)`

- <span id="atomic-swap"></span>`fn swap<'g, P: Pointer<T>>(&self, new: P, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-compare-exchange"></span>`fn compare_exchange<'g, P>(&self, current: Shared<'_, T>, new: P, success: Ordering, failure: Ordering, _: &'g Guard) -> Result<Shared<'g, T>, CompareExchangeError<'g, T, P>>` — [`Shared`](#shared), [`Guard`](#guard), [`CompareExchangeError`](#compareexchangeerror)

- <span id="atomic-compare-exchange-weak"></span>`fn compare_exchange_weak<'g, P>(&self, current: Shared<'_, T>, new: P, success: Ordering, failure: Ordering, _: &'g Guard) -> Result<Shared<'g, T>, CompareExchangeError<'g, T, P>>` — [`Shared`](#shared), [`Guard`](#guard), [`CompareExchangeError`](#compareexchangeerror)

- <span id="atomic-fetch-update"></span>`fn fetch_update<'g, F>(&self, set_order: Ordering, fail_order: Ordering, guard: &'g Guard, func: F) -> Result<Shared<'g, T>, Shared<'g, T>>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-compare-and-set"></span>`fn compare_and_set<'g, O, P>(&self, current: Shared<'_, T>, new: P, ord: O, guard: &'g Guard) -> Result<Shared<'g, T>, CompareAndSetError<'g, T, P>>` — [`Shared`](#shared), [`Guard`](#guard), [`CompareAndSetError`](#compareandseterror)

- <span id="atomic-compare-and-set-weak"></span>`fn compare_and_set_weak<'g, O, P>(&self, current: Shared<'_, T>, new: P, ord: O, guard: &'g Guard) -> Result<Shared<'g, T>, CompareAndSetError<'g, T, P>>` — [`Shared`](#shared), [`Guard`](#guard), [`CompareAndSetError`](#compareandseterror)

- <span id="atomic-fetch-and"></span>`fn fetch_and<'g>(&self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-fetch-or"></span>`fn fetch_or<'g>(&self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-fetch-xor"></span>`fn fetch_xor<'g>(&self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](#guard), [`Shared`](#shared)

- <span id="atomic-into-owned"></span>`unsafe fn into_owned(self) -> Owned<T>` — [`Owned`](#owned)

- <span id="atomic-try-into-owned"></span>`unsafe fn try_into_owned(self) -> Option<Owned<T>>` — [`Owned`](#owned)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Atomic<T>`

- <span id="atomic-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Atomic<T>`

- <span id="atomic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Atomic<T>`

- <span id="atomic-default"></span>`fn default() -> Self`

##### `impl<T> Pointable for Atomic<T>`

- <span id="atomic-align"></span>`const ALIGN: usize`

- <span id="atomic-init"></span>`type Init = T`

- <span id="atomic-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="atomic-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomic-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomic-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Atomic<T>`

- <span id="atomic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable + Send + Sync> Send for Atomic<T>`

##### `impl<T: ?Sized + Pointable + Send + Sync> Sync for Atomic<T>`

### `CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>>`

```rust
struct CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>> {
    pub current: Shared<'g, T>,
    pub new: P,
}
```

The error returned on failed compare-and-swap operation.

#### Fields

- **`current`**: `Shared<'g, T>`

  The value in the atomic pointer at the time of the failed operation.

- **`new`**: `P`

  The new value, which the operation failed to store.

#### Trait Implementations

##### `impl<T, P: Pointer<T> + fmt::Debug> Debug for CompareExchangeError<'_, T, P>`

- <span id="compareexchangeerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-align"></span>`const ALIGN: usize`

- <span id="compareexchangeerror-init"></span>`type Init = T`

- <span id="compareexchangeerror-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="compareexchangeerror-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="compareexchangeerror-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="compareexchangeerror-drop"></span>`unsafe fn drop(ptr: usize)`

### `Owned<T: ?Sized + Pointable>`

```rust
struct Owned<T: ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<alloc::boxed::Box<T>>,
}
```

An owned heap-allocated object.

This type is very similar to `Box<T>`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="owned-from-raw"></span>`unsafe fn from_raw(raw: *mut T) -> Owned<T>` — [`Owned`](#owned)

- <span id="owned-into-box"></span>`fn into_box(self) -> Box<T>`

- <span id="owned-new"></span>`fn new(init: T) -> Owned<T>` — [`Owned`](#owned)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> AsMut for Owned<T>`

- <span id="owned-as-mut"></span>`fn as_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> AsRef for Owned<T>`

- <span id="owned-as-ref"></span>`fn as_ref(&self) -> &T`

##### `impl<T: Clone> Clone for Owned<T>`

- <span id="owned-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Owned<T>`

- <span id="owned-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Deref for Owned<T>`

- <span id="owned-target"></span>`type Target = T`

- <span id="owned-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized + Pointable> DerefMut for Owned<T>`

- <span id="owned-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Drop for Owned<T>`

- <span id="owned-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Owned<T>`

- <span id="owned-align"></span>`const ALIGN: usize`

- <span id="owned-init"></span>`type Init = T`

- <span id="owned-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="owned-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="owned-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="owned-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Owned<T>`

- <span id="owned-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="owned-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

##### `impl<P, T> Receiver for Owned<T>`

- <span id="owned-target"></span>`type Target = T`

### `Shared<'g, T: 'g + ?Sized + Pointable>`

```rust
struct Shared<'g, T: 'g + ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<(&'g (), *const T)>,
}
```

A pointer to an object protected by the epoch GC.

The pointer is valid for use only during the lifetime `'g`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="shared-null"></span>`fn null() -> Shared<'g, T>` — [`Shared`](#shared)

- <span id="shared-is-null"></span>`fn is_null(&self) -> bool`

- <span id="shared-deref"></span>`unsafe fn deref(&self) -> &'g T`

- <span id="shared-deref-mut"></span>`unsafe fn deref_mut(&mut self) -> &'g mut T`

- <span id="shared-as-ref"></span>`unsafe fn as_ref(&self) -> Option<&'g T>`

- <span id="shared-into-owned"></span>`unsafe fn into_owned(self) -> Owned<T>` — [`Owned`](#owned)

- <span id="shared-try-into-owned"></span>`unsafe fn try_into_owned(self) -> Option<Owned<T>>` — [`Owned`](#owned)

- <span id="shared-tag"></span>`fn tag(&self) -> usize`

- <span id="shared-with-tag"></span>`fn with_tag(&self, tag: usize) -> Shared<'g, T>` — [`Shared`](#shared)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Shared<'_, T>`

- <span id="shared-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized + Pointable> Copy for Shared<'_, T>`

##### `impl<T: ?Sized + Pointable> Debug for Shared<'_, T>`

- <span id="shared-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Shared<'_, T>`

- <span id="shared-default"></span>`fn default() -> Self`

##### `impl<T: ?Sized + Pointable> Eq for Shared<'_, T>`

##### `impl<T: ?Sized + Pointable> Ord for Shared<'_, T>`

- <span id="shared-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl<'g, T: ?Sized + Pointable> PartialEq for Shared<'g, T>`

- <span id="shared-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<'g, T: ?Sized + Pointable> PartialOrd for Shared<'g, T>`

- <span id="shared-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T> Pointable for Shared<'g, T>`

- <span id="shared-align"></span>`const ALIGN: usize`

- <span id="shared-init"></span>`type Init = T`

- <span id="shared-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="shared-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="shared-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="shared-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Shared<'_, T>`

- <span id="shared-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="shared-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

### `Collector`

```rust
struct Collector {
    global: alloc::sync::Arc<crate::internal::Global>,
}
```

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](#localhandle)

#### Trait Implementations

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Collector`

- <span id="collector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- <span id="collector-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](#collector)

##### `impl<T> Pointable for Collector`

- <span id="collector-align"></span>`const ALIGN: usize`

- <span id="collector-init"></span>`type Init = T`

- <span id="collector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="collector-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Collector`

##### `impl Sync for Collector`

### `LocalHandle`

```rust
struct LocalHandle {
    local: *const crate::internal::Local,
}
```

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](#guard)

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](#collector)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- <span id="localhandle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for LocalHandle`

- <span id="localhandle-align"></span>`const ALIGN: usize`

- <span id="localhandle-init"></span>`type Init = T`

- <span id="localhandle-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="localhandle-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-drop"></span>`unsafe fn drop(ptr: usize)`

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

A guard that keeps the current thread pinned.

# Pinning

The current thread is pinned by calling [`pin`](#pin), which returns a new guard:

```rust
use crossbeam_epoch as epoch;

// It is often convenient to prefix a call to `pin` with a `&` in order to create a reference.
// This is not really necessary, but makes passing references to the guard a bit easier.
let guard = &epoch::pin();
```

When a guard gets dropped, the current thread is automatically unpinned.

# Pointers on the stack

Having a guard allows us to create pointers on the stack to heap-allocated objects.
For example:

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::sync::atomic::Ordering::SeqCst;

// Create a heap-allocated number.
let a = Atomic::new(777);

// Pin the current thread.
let guard = &epoch::pin();

// Load the heap-allocated object and create pointer `p` on the stack.
let p = a.load(SeqCst, guard);

// Dereference the pointer and print the value:
if let Some(num) = unsafe { p.as_ref() } {
    println!("The number is {}.", num);
}
unsafe { drop(a.into_owned()); } // avoid leak
```

# Multiple guards

Pinning is reentrant and it is perfectly legal to create multiple guards. In that case, the
thread will actually be pinned only when the first guard is created and unpinned when the last
one is dropped:

```rust
use crossbeam_epoch as epoch;

let guard1 = epoch::pin();
let guard2 = epoch::pin();
assert!(epoch::is_pinned());
drop(guard1);
assert!(epoch::is_pinned());
drop(guard2);
assert!(!epoch::is_pinned());
```


#### Implementations

- <span id="guard-defer"></span>`fn defer<F, R>(&self, f: F)`

- <span id="guard-defer-unchecked"></span>`unsafe fn defer_unchecked<F, R>(&self, f: F)`

- <span id="guard-defer-destroy"></span>`unsafe fn defer_destroy<T>(&self, ptr: Shared<'_, T>)` — [`Shared`](#shared)

- <span id="guard-flush"></span>`fn flush(&self)`

- <span id="guard-repin"></span>`fn repin(&mut self)`

- <span id="guard-repin-after"></span>`fn repin_after<F, R>(&mut self, f: F) -> R`

- <span id="guard-collector"></span>`fn collector(&self) -> Option<&Collector>` — [`Collector`](#collector)

#### Trait Implementations

##### `impl Debug for Guard`

- <span id="guard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- <span id="guard-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Guard`

- <span id="guard-align"></span>`const ALIGN: usize`

- <span id="guard-init"></span>`type Init = T`

- <span id="guard-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="guard-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="guard-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="guard-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `CompareAndSetOrdering`

```rust
trait CompareAndSetOrdering { ... }
```

Memory orderings for compare-and-set operations.

A compare-and-set operation can have different memory orderings depending on whether it
succeeds or fails. This trait generalizes different ways of specifying memory orderings.

The two ways of specifying orderings for compare-and-set are:

1. Just one `Ordering` for the success case. In case of failure, the strongest appropriate
   ordering is chosen.
2. A pair of `Ordering`s. The first one is for the success case, while the second one is
   for the failure case.

#### Required Methods

- `fn success(&self) -> Ordering`

  The ordering of the operation when it succeeds.

- `fn failure(&self) -> Ordering`

  The ordering of the operation when it fails.

#### Implementors

- `(core::sync::atomic::Ordering, core::sync::atomic::Ordering)`
- `core::sync::atomic::Ordering`

### `Pointable`

```rust
trait Pointable { ... }
```

Types that are pointed to by a single word.

In concurrent programming, it is necessary to represent an object within a word because atomic
operations (e.g., reads, writes, read-modify-writes) support only single words.  This trait
qualifies such types that are pointed to by a single word.

The trait generalizes `Box<T>` for a sized type `T`.  In a box, an object of type `T` is
allocated in heap and it is owned by a single-word pointer.  This trait is also implemented for
`[MaybeUninit<T>]` by storing its size along with its elements and pointing to the pair of array
size and elements.

Pointers to `Pointable` types can be stored in [`Atomic`](#atomic), [`Owned`](#owned), and [`Shared`](#shared).  In
particular, Crossbeam supports dynamically sized slices as follows.

```rust
use std::mem::MaybeUninit;
use crossbeam_epoch::Owned;

let o = Owned::<[MaybeUninit<i32>]>::init(10); // allocating [i32; 10]
```

#### Associated Types

- `type Init`

#### Associated Constants

- `const ALIGN: usize`

#### Required Methods

- `fn init(init: <Self as >::Init) -> usize`

  Initializes a with the given initializer.

- `fn deref<'a>(ptr: usize) -> &'a Self`

  Dereferences the given pointer.

- `fn deref_mut<'a>(ptr: usize) -> &'a mut Self`

  Mutably dereferences the given pointer.

- `fn drop(ptr: usize)`

  Drops the object pointed to by the given pointer.

#### Implementors

- [`Array`](atomic/index.md)
- [`AtomicEpoch`](epoch/index.md)
- [`Atomic`](#atomic)
- [`Bag`](internal/index.md)
- [`Collector`](#collector)
- [`CompareExchangeError`](#compareexchangeerror)
- [`Deferred`](deferred/index.md)
- [`Entry`](sync/list/index.md)
- [`Epoch`](epoch/index.md)
- [`Global`](internal/index.md)
- [`Guard`](#guard)
- [`IterError`](sync/list/index.md)
- [`Iter`](sync/list/index.md)
- [`List`](sync/list/index.md)
- [`LocalHandle`](#localhandle)
- [`Local`](internal/index.md)
- [`Node`](sync/queue/index.md)
- [`OnceLock`](sync/once_lock/index.md)
- [`Owned`](#owned)
- [`Queue`](sync/queue/index.md)
- [`SealedBag`](internal/index.md)
- [`Shared`](#shared)
- [`UnsafeCell`](primitive/cell/index.md)
- `T`
- `[core::mem::MaybeUninit<T>]`

### `Pointer<T: ?Sized + Pointable>`

```rust
trait Pointer<T: ?Sized + Pointable> { ... }
```

A trait for either `Owned` or `Shared` pointers.

#### Required Methods

- `fn into_usize(self) -> usize`

  Returns the machine representation of the pointer.

- `fn from_usize(data: usize) -> Self`

  Returns a new pointer pointing to the tagged pointer `data`.

#### Implementors

- [`Owned`](#owned)
- [`Shared`](#shared)

## Functions

## Type Aliases

