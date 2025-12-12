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

Concurrent collections are built using atomic pointers. This module provides [`Atomic`](atomic/index.md), which
is just a shared atomic pointer to a heap-allocated object. Loading an [`Atomic`](atomic/index.md) yields a
[`Shared`](atomic/index.md), which is an epoch-protected pointer through which the loaded object can be safely
read.

# Pinning

Before an [`Atomic`](atomic/index.md) can be loaded, a participant must be [`pin`](default/index.md)ned. By pinning a participant
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

For majority of use cases, just use the default garbage collector by invoking [`pin`](default/index.md). If you
want to create your own garbage collector, use the [`Collector`](collector/index.md) API.

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
  - [`default_collector`](#default-collector)
  - [`is_pinned`](#is-pinned)
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
| [`default_collector`](#default-collector) | fn |  |
| [`is_pinned`](#is-pinned) | fn |  |
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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:294-297`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L294-L297)*

An atomic pointer that can be safely shared between threads.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address. For example, the tag for a pointer to a sized type `T`
should be less than `(1 << mem::align_of::<T>().trailing_zeros())`.

Any method that loads the pointer must be passed a reference to a [`Guard`](guard/index.md).

Crossbeam supports dynamically sized types.  See [`Pointable`](atomic/index.md) for details.

#### Implementations

- <span id="atomic-new"></span>`fn new(init: T) -> Atomic<T>` — [`Atomic`](atomic/index.md#atomic)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Atomic<T>`

- <span id="atomic-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Atomic<T>`

- <span id="atomic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Atomic<T>`

- <span id="atomic-default"></span>`fn default() -> Self`

##### `impl<T> Pointable for Atomic<T>`

- <span id="atomic-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomic-pointable-type-init"></span>`type Init = T`

- <span id="atomic-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:34-40`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L34-L40)*

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

- <span id="compareexchangeerror-pointable-const-align"></span>`const ALIGN: usize`

- <span id="compareexchangeerror-pointable-type-init"></span>`type Init = T`

- <span id="compareexchangeerror-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1048-1051`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1048-L1051)*

An owned heap-allocated object.

This type is very similar to `Box<T>`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="owned-from-raw"></span>`unsafe fn from_raw(raw: *mut T) -> Owned<T>` — [`Owned`](atomic/index.md#owned)

- <span id="owned-into-box"></span>`fn into_box(self) -> Box<T>`

- <span id="owned-new"></span>`fn new(init: T) -> Owned<T>` — [`Owned`](atomic/index.md#owned)

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

- <span id="owned-deref-type-target"></span>`type Target = T`

- <span id="owned-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized + Pointable> DerefMut for Owned<T>`

- <span id="owned-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Drop for Owned<T>`

- <span id="owned-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Owned<T>`

- <span id="owned-pointable-const-align"></span>`const ALIGN: usize`

- <span id="owned-pointable-type-init"></span>`type Init = T`

- <span id="owned-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="owned-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="owned-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="owned-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Owned<T>`

- <span id="owned-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="owned-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

##### `impl<T> Receiver for Owned<T>`

- <span id="owned-receiver-type-target"></span>`type Target = T`

### `Shared<'g, T: 'g + ?Sized + Pointable>`

```rust
struct Shared<'g, T: 'g + ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<(&'g (), *const T)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1297-1300`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1297-L1300)*

A pointer to an object protected by the epoch GC.

The pointer is valid for use only during the lifetime `'g`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="shared-as-raw"></span>`fn as_raw(&self) -> *const T`

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

##### `impl<T: ?Sized + Pointable> PartialEq for Shared<'g, T>`

- <span id="shared-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: ?Sized + Pointable> PartialOrd for Shared<'g, T>`

- <span id="shared-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T> Pointable for Shared<'g, T>`

- <span id="shared-pointable-const-align"></span>`const ALIGN: usize`

- <span id="shared-pointable-type-init"></span>`type Init = T`

- <span id="shared-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

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

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:22-24`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/collector.rs#L22-L24)*

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](collector/index.md#localhandle)

#### Trait Implementations

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Collector`

- <span id="collector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- <span id="collector-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](collector/index.md#collector)

##### `impl Pointable for Collector`

- <span id="collector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collector-pointable-type-init"></span>`type Init = T`

- <span id="collector-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

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

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:73-75`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/collector.rs#L73-L75)*

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](guard/index.md#guard)

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](collector/index.md#collector)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- <span id="localhandle-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for LocalHandle`

- <span id="localhandle-pointable-const-align"></span>`const ALIGN: usize`

- <span id="localhandle-pointable-type-init"></span>`type Init = T`

- <span id="localhandle-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="localhandle-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-drop"></span>`unsafe fn drop(ptr: usize)`

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:69-71`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/guard.rs#L69-L71)*

A guard that keeps the current thread pinned.

# Pinning

The current thread is pinned by calling [`pin`](default/index.md), which returns a new guard:

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

- <span id="guard-defer-destroy"></span>`unsafe fn defer_destroy<T>(&self, ptr: Shared<'_, T>)` — [`Shared`](atomic/index.md#shared)

- <span id="guard-flush"></span>`fn flush(&self)`

- <span id="guard-repin"></span>`fn repin(&mut self)`

- <span id="guard-repin-after"></span>`fn repin_after<F, R>(&mut self, f: F) -> R`

- <span id="guard-collector"></span>`fn collector(&self) -> Option<&Collector>` — [`Collector`](collector/index.md#collector)

#### Trait Implementations

##### `impl Debug for Guard`

- <span id="guard-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- <span id="guard-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for Guard`

- <span id="guard-pointable-const-align"></span>`const ALIGN: usize`

- <span id="guard-pointable-type-init"></span>`type Init = T`

- <span id="guard-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="guard-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="guard-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="guard-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `CompareAndSetOrdering`

```rust
trait CompareAndSetOrdering { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:67-76`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L67-L76)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:150-192`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L150-L192)*

Types that are pointed to by a single word.

In concurrent programming, it is necessary to represent an object within a word because atomic
operations (e.g., reads, writes, read-modify-writes) support only single words.  This trait
qualifies such types that are pointed to by a single word.

The trait generalizes `Box<T>` for a sized type `T`.  In a box, an object of type `T` is
allocated in heap and it is owned by a single-word pointer.  This trait is also implemented for
`[MaybeUninit<T>]` by storing its size along with its elements and pointing to the pair of array
size and elements.

Pointers to `Pointable` types can be stored in [`Atomic`](atomic/index.md), [`Owned`](atomic/index.md), and [`Shared`](atomic/index.md).  In
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

- `T`
- `[core::mem::MaybeUninit<T>]`

### `Pointer<T: ?Sized + Pointable>`

```rust
trait Pointer<T: ?Sized + Pointable> { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1029-1040`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1029-L1040)*

A trait for either `Owned` or `Shared` pointers.

#### Required Methods

- `fn into_usize(self) -> usize`

  Returns the machine representation of the pointer.

- `fn from_usize(data: usize) -> Self`

  Returns a new pointer pointing to the tagged pointer `data`.

#### Implementors

- [`Owned`](atomic/index.md#owned)
- [`Shared`](atomic/index.md#shared)

## Functions

### `unprotected`

```rust
unsafe fn unprotected() -> &'static Guard
```

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:513-523`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/guard.rs#L513-L523)*

Returns a reference to a dummy guard that allows unprotected access to [`Atomic`](atomic/index.md)s.

This guard should be used in special occasions only. Note that it doesn't actually keep any
thread pinned - it's just a fake guard that allows loading from [`Atomic`](atomic/index.md)s unsafely.

Note that calling `defer` with a dummy guard will not defer the function - it will just
execute the function immediately.

If necessary, it's possible to create more dummy guards by cloning: `unprotected().clone()`.

# Safety

Loading and dereferencing data from an [`Atomic`](atomic/index.md) using this guard is safe only if the
[`Atomic`](atomic/index.md) is not being concurrently modified by other threads.

# Examples

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::sync::atomic::Ordering::Relaxed;

let a = Atomic::new(7);

unsafe {
    // Load `a` without pinning the current thread.
    a.load(Relaxed, epoch::unprotected());

    // It's possible to create more dummy guards.
    let dummy = epoch::unprotected();

    dummy.defer(move || {
        println!("This gets executed immediately.");
    });

    // Dropping `dummy` doesn't affect the current thread - it's just a noop.
}
unsafe { drop(a.into_owned()); } // avoid leak
```

The most common use of this function is when constructing or destructing a data structure.

For example, we can use a dummy guard in the destructor of a Treiber stack because at that
point no other thread could concurrently modify the [`Atomic`](atomic/index.md)s we are accessing.

If we were to actually pin the current thread during destruction, that would just unnecessarily
delay garbage collection and incur some performance cost, so in cases like these `unprotected`
is very helpful.

```rust
use crossbeam_epoch::{self as epoch, Atomic};
use std::mem::ManuallyDrop;
use std::sync::atomic::Ordering::Relaxed;

struct Stack<T> {
    head: Atomic<Node<T>>,
}

struct Node<T> {
    data: ManuallyDrop<T>,
    next: Atomic<Node<T>>,
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            // Unprotected load.
            let mut node = self.head.load(Relaxed, epoch::unprotected());

            while let Some(n) = node.as_ref() {
                // Unprotected load.
                let next = n.next.load(Relaxed, epoch::unprotected());

                // Take ownership of the node, then drop its data and deallocate it.
                let mut o = node.into_owned();
                ManuallyDrop::drop(&mut o.data);
                drop(o);

                node = next;
            }
        }
    }
}
```



### `default_collector`

```rust
fn default_collector() -> &'static crate::collector::Collector
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:50-52`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/default.rs#L50-L52)*

Returns the default global collector.

### `is_pinned`

```rust
fn is_pinned() -> bool
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:45-47`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/default.rs#L45-L47)*

Returns `true` if the current thread is pinned.

### `pin`

```rust
fn pin() -> crate::guard::Guard
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:39-41`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/default.rs#L39-L41)*

Pins the current thread.

## Type Aliases

### `CompareAndSetError<'g, T, P>`

```rust
type CompareAndSetError<'g, T, P> = CompareExchangeError<'g, T, P>;
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:31`](../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L31)*

The error returned on failed compare-and-set operation.

