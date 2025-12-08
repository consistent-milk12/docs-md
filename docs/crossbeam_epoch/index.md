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

Any method that loads the pointer must be passed a reference to a [`Guard`](guard/index.md).

Crossbeam supports dynamically sized types.  See [`Pointable`](atomic/index.md) for details.

#### Implementations

- `fn new(init: T) -> Atomic<T>` — [`Atomic`](atomic/index.md)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Atomic<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Atomic<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Atomic<T>`

- `fn default() -> Self`

##### `impl<T> Pointable for Atomic<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Atomic<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Pointable for CompareExchangeError<'g, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `unsafe fn from_raw(raw: *mut T) -> Owned<T>` — [`Owned`](atomic/index.md)

- `fn into_box(self: Self) -> Box<T>`

- `fn new(init: T) -> Owned<T>` — [`Owned`](atomic/index.md)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> AsMut for Owned<T>`

- `fn as_mut(self: &mut Self) -> &mut T`

##### `impl<T: ?Sized + Pointable> AsRef for Owned<T>`

- `fn as_ref(self: &Self) -> &T`

##### `impl<T: Clone> Clone for Owned<T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Owned<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Deref for Owned<T>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl<T: ?Sized + Pointable> DerefMut for Owned<T>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Drop for Owned<T>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Owned<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Owned<T>`

- `fn into_usize(self: Self) -> usize`

- `unsafe fn from_usize(data: usize) -> Self`

##### `impl<P, T> Receiver for Owned<T>`

- `type Target = T`

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

- `fn null() -> Shared<'g, T>` — [`Shared`](atomic/index.md)

- `fn is_null(self: &Self) -> bool`

- `unsafe fn deref(self: &Self) -> &'g T`

- `unsafe fn deref_mut(self: &mut Self) -> &'g mut T`

- `unsafe fn as_ref(self: &Self) -> Option<&'g T>`

- `unsafe fn into_owned(self: Self) -> Owned<T>` — [`Owned`](atomic/index.md)

- `unsafe fn try_into_owned(self: Self) -> Option<Owned<T>>` — [`Owned`](atomic/index.md)

- `fn tag(self: &Self) -> usize`

- `fn with_tag(self: &Self, tag: usize) -> Shared<'g, T>` — [`Shared`](atomic/index.md)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Shared<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: ?Sized + Pointable> Copy for Shared<'_, T>`

##### `impl<T: ?Sized + Pointable> Debug for Shared<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Shared<'_, T>`

- `fn default() -> Self`

##### `impl<T: ?Sized + Pointable> Eq for Shared<'_, T>`

##### `impl<T: ?Sized + Pointable> Ord for Shared<'_, T>`

- `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`

##### `impl<'g, T: ?Sized + Pointable> PartialEq for Shared<'g, T>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<'g, T: ?Sized + Pointable> PartialOrd for Shared<'g, T>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T> Pointable for Shared<'g, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Shared<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Collector`

```rust
struct Collector {
    global: alloc::sync::Arc<crate::internal::Global>,
}
```

An epoch-based garbage collector.

#### Implementations

- `fn new() -> Self`

- `fn register(self: &Self) -> LocalHandle` — [`LocalHandle`](collector/index.md)

#### Trait Implementations

##### `impl Clone for Collector`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for Collector`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- `fn default() -> Self`

##### `impl Eq for Collector`

##### `impl PartialEq for Collector`

- `fn eq(self: &Self, rhs: &Collector) -> bool` — [`Collector`](collector/index.md)

##### `impl<T> Pointable for Collector`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn pin(self: &Self) -> Guard` — [`Guard`](guard/index.md)

- `fn is_pinned(self: &Self) -> bool`

- `fn collector(self: &Self) -> &Collector` — [`Collector`](collector/index.md)

#### Trait Implementations

##### `impl Debug for LocalHandle`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for LocalHandle`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

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

- `fn defer<F, R>(self: &Self, f: F)`

- `unsafe fn defer_unchecked<F, R>(self: &Self, f: F)`

- `unsafe fn defer_destroy<T>(self: &Self, ptr: Shared<'_, T>)` — [`Shared`](atomic/index.md)

- `fn flush(self: &Self)`

- `fn repin(self: &mut Self)`

- `fn repin_after<F, R>(self: &mut Self, f: F) -> R`

- `fn collector(self: &Self) -> Option<&Collector>` — [`Collector`](collector/index.md)

#### Trait Implementations

##### `impl Debug for Guard`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for Guard`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

## Functions

## Type Aliases

