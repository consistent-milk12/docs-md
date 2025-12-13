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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:294-297`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L294-L297)*

An atomic pointer that can be safely shared between threads.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address. For example, the tag for a pointer to a sized type `T`
should be less than `(1 << mem::align_of::<T>().trailing_zeros())`.

Any method that loads the pointer must be passed a reference to a [`Guard`](guard/index.md).

Crossbeam supports dynamically sized types.  See [`Pointable`](atomic/index.md) for details.

#### Implementations

- <span id="atomic-new"></span>`fn new(init: T) -> Atomic<T>` — [`Atomic`](atomic/index.md#atomic)

  Allocates `value` on the heap and returns a new atomic pointer pointing to it.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::Atomic;

  

  let a = Atomic::new(1234);

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

#### Trait Implementations

##### `impl<T> Any for Atomic<T>`

- <span id="atomic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Atomic<T>`

- <span id="atomic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Atomic<T>`

- <span id="atomic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Clone for Atomic<T>`

- <span id="atomic-clone"></span>`fn clone(&self) -> Self`

  Returns a copy of the atomic value.

  

  Note that a `Relaxed` load is used here. If you need synchronization, use it with other

  atomics or fences.

##### `impl<T> CloneToUninit for Atomic<T>`

- <span id="atomic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: ?Sized + Pointable> Debug for Atomic<T>`

- <span id="atomic-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Atomic<T>`

- <span id="atomic-default"></span>`fn default() -> Self`

##### `impl<T> From for Atomic<T>`

- <span id="atomic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Atomic<T>`

- <span id="atomic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Atomic<T>`

- <span id="atomic-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomic-pointable-type-init"></span>`type Init = T`

- <span id="atomic-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="atomic-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomic-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomic-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Atomic<T>`

- <span id="atomic-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable + Send + Sync> Send for Atomic<T>`

##### `impl<T: ?Sized + Pointable + Send + Sync> Sync for Atomic<T>`

##### `impl<T> ToOwned for Atomic<T>`

- <span id="atomic-toowned-type-owned"></span>`type Owned = T`

- <span id="atomic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="atomic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Atomic<T>`

- <span id="atomic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Atomic<T>`

- <span id="atomic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>>`

```rust
struct CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>> {
    pub current: Shared<'g, T>,
    pub new: P,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:34-40`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L34-L40)*

The error returned on failed compare-and-swap operation.

#### Fields

- **`current`**: `Shared<'g, T>`

  The value in the atomic pointer at the time of the failed operation.

- **`new`**: `P`

  The new value, which the operation failed to store.

#### Trait Implementations

##### `impl<T> Any for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P: Pointer<T> + fmt::Debug> Debug for CompareExchangeError<'_, T, P>`

- <span id="compareexchangeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-pointable-const-align"></span>`const ALIGN: usize`

- <span id="compareexchangeerror-pointable-type-init"></span>`type Init = T`

- <span id="compareexchangeerror-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="compareexchangeerror-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="compareexchangeerror-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="compareexchangeerror-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compareexchangeerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CompareExchangeError<'g, T, P>`

- <span id="compareexchangeerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compareexchangeerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Owned<T: ?Sized + Pointable>`

```rust
struct Owned<T: ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<alloc::boxed::Box<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1048-1051`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L1048-L1051)*

An owned heap-allocated object.

This type is very similar to `Box<T>`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="owned-from-raw"></span>`unsafe fn from_raw(raw: *mut T) -> Owned<T>` — [`Owned`](atomic/index.md#owned)

  Returns a new owned pointer pointing to `raw`.

  

  This function is unsafe because improper use may lead to memory problems. Argument `raw`

  must be a valid pointer. Also, a double-free may occur if the function is called twice on

  the same raw pointer.

  

  # Panics

  

  Panics if `raw` is not properly aligned.

  

  # Safety

  

  The given `raw` should have been derived from `Owned`, and one `raw` should not be converted

  back by `Owned::from_raw()` multiple times.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::Owned;

  

  let o = unsafe { Owned::from_raw(Box::into_raw(Box::new(1234))) };

  ```

- <span id="owned-into-box"></span>`fn into_box(self) -> Box<T>`

  Converts the owned pointer into a `Box`.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::Owned;

  

  let o = Owned::new(1234);

  let b: Box<i32> = o.into_box();

  assert_eq!(*b, 1234);

  ```

- <span id="owned-new"></span>`fn new(init: T) -> Owned<T>` — [`Owned`](atomic/index.md#owned)

  Allocates `value` on the heap and returns a new owned pointer pointing to it.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::Owned;

  

  let o = Owned::new(1234);

  ```

#### Trait Implementations

##### `impl<T> Any for Owned<T>`

- <span id="owned-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: ?Sized + Pointable> AsMut for Owned<T>`

- <span id="owned-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> AsRef for Owned<T>`

- <span id="owned-asref-as-ref"></span>`fn as_ref(&self) -> &T`

##### `impl<T> Borrow for Owned<T>`

- <span id="owned-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Owned<T>`

- <span id="owned-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone> Clone for Owned<T>`

- <span id="owned-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Owned<T>`

- <span id="owned-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: ?Sized + Pointable> Debug for Owned<T>`

- <span id="owned-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Deref for Owned<T>`

- <span id="owned-deref-type-target"></span>`type Target = T`

- <span id="owned-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized + Pointable> DerefMut for Owned<T>`

- <span id="owned-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Drop for Owned<T>`

- <span id="owned-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Owned<T>`

- <span id="owned-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Owned<T>`

- <span id="owned-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for Owned<T>`

- <span id="owned-pointable-const-align"></span>`const ALIGN: usize`

- <span id="owned-pointable-type-init"></span>`type Init = T`

- <span id="owned-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="owned-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="owned-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="owned-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Owned<T>`

- <span id="owned-pointer-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="owned-pointer-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

  Returns a new pointer pointing to the tagged pointer `data`.

  

  # Panics

  

  Panics if the data is zero in debug mode.

##### `impl<T> Receiver for Owned<T>`

- <span id="owned-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToOwned for Owned<T>`

- <span id="owned-toowned-type-owned"></span>`type Owned = T`

- <span id="owned-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="owned-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Owned<T>`

- <span id="owned-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="owned-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Owned<T>`

- <span id="owned-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="owned-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Shared<'g, T: 'g + ?Sized + Pointable>`

```rust
struct Shared<'g, T: 'g + ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<(&'g (), *const T)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1297-1300`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L1297-L1300)*

A pointer to an object protected by the epoch GC.

The pointer is valid for use only during the lifetime `'g`.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address.

#### Implementations

- <span id="shared-as-raw"></span>`fn as_raw(&self) -> *const T`

  Converts the pointer to a raw pointer (without the tag).

  

  # Examples

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic, Owned};

  use std::sync::atomic::Ordering::SeqCst;

  

  let o = Owned::new(1234);

  let raw = &*o as *const _;

  let a = Atomic::from(o);

  

  let guard = &epoch::pin();

  let p = a.load(SeqCst, guard);

  assert_eq!(p.as_raw(), raw);

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

#### Trait Implementations

##### `impl<T> Any for Shared<'g, T>`

- <span id="shared-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Shared<'g, T>`

- <span id="shared-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Shared<'g, T>`

- <span id="shared-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Clone for Shared<'_, T>`

- <span id="shared-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Shared<'g, T>`

- <span id="shared-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: ?Sized + Pointable> Copy for Shared<'_, T>`

##### `impl<T: ?Sized + Pointable> Debug for Shared<'_, T>`

- <span id="shared-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Shared<'_, T>`

- <span id="shared-default"></span>`fn default() -> Self`

##### `impl<T: ?Sized + Pointable> Eq for Shared<'_, T>`

##### `impl<T> From for Shared<'g, T>`

- <span id="shared-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Shared<'g, T>`

- <span id="shared-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: ?Sized + Pointable> Ord for Shared<'_, T>`

- <span id="shared-ord-cmp"></span>`fn cmp(&self, other: &Self) -> cmp::Ordering`

##### `impl<T: ?Sized + Pointable> PartialEq for Shared<'g, T>`

- <span id="shared-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: ?Sized + Pointable> PartialOrd for Shared<'g, T>`

- <span id="shared-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T> Pointable for Shared<'g, T>`

- <span id="shared-pointable-const-align"></span>`const ALIGN: usize`

- <span id="shared-pointable-type-init"></span>`type Init = T`

- <span id="shared-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="shared-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="shared-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="shared-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Shared<'_, T>`

- <span id="shared-pointer-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="shared-pointer-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

##### `impl<T> ToOwned for Shared<'g, T>`

- <span id="shared-toowned-type-owned"></span>`type Owned = T`

- <span id="shared-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shared-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Shared<'g, T>`

- <span id="shared-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shared-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Shared<'g, T>`

- <span id="shared-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shared-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Collector`

```rust
struct Collector {
    global: alloc::sync::Arc<crate::internal::Global>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:22-24`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/collector.rs#L22-L24)*

An epoch-based garbage collector.

#### Implementations

- <span id="collector-new"></span>`fn new() -> Self`

  Creates a new collector.

- <span id="collector-register"></span>`fn register(&self) -> LocalHandle` — [`LocalHandle`](collector/index.md#localhandle)

  Registers a new handle for the collector.

#### Trait Implementations

##### `impl Any for Collector`

- <span id="collector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Collector`

- <span id="collector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Collector`

- <span id="collector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Collector`

- <span id="collector-clone"></span>`fn clone(&self) -> Self`

  Creates another reference to the same garbage collector.

##### `impl CloneToUninit for Collector`

- <span id="collector-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Collector`

- <span id="collector-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Collector`

- <span id="collector-default"></span>`fn default() -> Self`

##### `impl Eq for Collector`

##### `impl<T> From for Collector`

- <span id="collector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Collector`

- <span id="collector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Collector`

- <span id="collector-partialeq-eq"></span>`fn eq(&self, rhs: &Collector) -> bool` — [`Collector`](collector/index.md#collector)

  Checks if both handles point to the same collector.

##### `impl Pointable for Collector`

- <span id="collector-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collector-pointable-type-init"></span>`type Init = T`

- <span id="collector-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="collector-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collector-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collector-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Send for Collector`

##### `impl Sync for Collector`

##### `impl ToOwned for Collector`

- <span id="collector-toowned-type-owned"></span>`type Owned = T`

- <span id="collector-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="collector-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Collector`

- <span id="collector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Collector`

- <span id="collector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocalHandle`

```rust
struct LocalHandle {
    local: *const crate::internal::Local,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/collector.rs:73-75`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/collector.rs#L73-L75)*

A handle to a garbage collector.

#### Implementations

- <span id="localhandle-pin"></span>`fn pin(&self) -> Guard` — [`Guard`](guard/index.md#guard)

  Pins the handle.

- <span id="localhandle-is-pinned"></span>`fn is_pinned(&self) -> bool`

  Returns `true` if the handle is pinned.

- <span id="localhandle-collector"></span>`fn collector(&self) -> &Collector` — [`Collector`](collector/index.md#collector)

  Returns the `Collector` associated with this handle.

#### Trait Implementations

##### `impl Any for LocalHandle`

- <span id="localhandle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocalHandle`

- <span id="localhandle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalHandle`

- <span id="localhandle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LocalHandle`

- <span id="localhandle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for LocalHandle`

- <span id="localhandle-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for LocalHandle`

- <span id="localhandle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocalHandle`

- <span id="localhandle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for LocalHandle`

- <span id="localhandle-pointable-const-align"></span>`const ALIGN: usize`

- <span id="localhandle-pointable-type-init"></span>`type Init = T`

- <span id="localhandle-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="localhandle-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="localhandle-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="localhandle-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for LocalHandle`

- <span id="localhandle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localhandle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalHandle`

- <span id="localhandle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localhandle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Guard`

```rust
struct Guard {
    local: *const crate::internal::Local,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:69-71`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/guard.rs#L69-L71)*

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

  Stores a function so that it can be executed at some point after all currently pinned

  threads get unpinned.

  

  This method first stores `f` into the thread-local (or handle-local) cache. If this cache

  becomes full, some functions are moved into the global cache. At the same time, some

  functions from both local and global caches may get executed in order to incrementally

  clean up the caches as they fill up.

  

  There is no guarantee when exactly `f` will be executed. The only guarantee is that it

  won't be executed until all currently pinned threads get unpinned. In theory, `f` might

  never run, but the epoch-based garbage collection will make an effort to execute it

  reasonably soon.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, the function will simply be

  executed immediately.

- <span id="guard-defer-unchecked"></span>`unsafe fn defer_unchecked<F, R>(&self, f: F)`

  Stores a function so that it can be executed at some point after all currently pinned

  threads get unpinned.

  

  This method first stores `f` into the thread-local (or handle-local) cache. If this cache

  becomes full, some functions are moved into the global cache. At the same time, some

  functions from both local and global caches may get executed in order to incrementally

  clean up the caches as they fill up.

  

  There is no guarantee when exactly `f` will be executed. The only guarantee is that it

  won't be executed until all currently pinned threads get unpinned. In theory, `f` might

  never run, but the epoch-based garbage collection will make an effort to execute it

  reasonably soon.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, the function will simply be

  executed immediately.

  

  # Safety

  

  The given function must not hold reference onto the stack. It is highly recommended that

  the passed function is **always** marked with `move` in order to prevent accidental

  borrows.

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard = &epoch::pin();

  let message = "Hello!";

  unsafe {

      // ALWAYS use `move` when sending a closure into `defer_unchecked`.

      guard.defer_unchecked(move || {

          println!("{}", message);

      });

  }

  ```

  

  Apart from that, keep in mind that another thread may execute `f`, so anything accessed by

  the closure must be `Send`.

  

  We intentionally didn't require `F: Send`, because Rust's type systems usually cannot prove

  `F: Send` for typical use cases. For example, consider the following code snippet, which

  exemplifies the typical use case of deferring the deallocation of a shared reference:

  

  ```ignore

  let shared = Owned::new(7i32).into_shared(guard);

  guard.defer_unchecked(move || shared.into_owned()); // `Shared` is not `Send`!

  ```

  

  While `Shared` is not `Send`, it's safe for another thread to call the deferred function,

  because it's called only after the grace period and `shared` is no longer shared with other

  threads. But we don't expect type systems to prove this.

  

  # Examples

  

  When a heap-allocated object in a data structure becomes unreachable, it has to be

  deallocated. However, the current thread and other threads may be still holding references

  on the stack to that same object. Therefore it cannot be deallocated before those references

  get dropped. This method can defer deallocation until all those threads get unpinned and

  consequently drop all their references on the stack.

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic, Owned};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new("foo");

  

  // Now suppose that `a` is shared among multiple threads and concurrently

  // accessed and modified...

  

  // Pin the current thread.

  let guard = &epoch::pin();

  

  // Steal the object currently stored in `a` and swap it with another one.

  let p = a.swap(Owned::new("bar").into_shared(guard), SeqCst, guard);

  

  if !p.is_null() {

      // The object `p` is pointing to is now unreachable.

      // Defer its deallocation until all currently pinned threads get unpinned.

      unsafe {

          // ALWAYS use `move` when sending a closure into `defer_unchecked`.

          guard.defer_unchecked(move || {

              println!("{} is now being deallocated.", p.deref());

              // Now we have unique access to the object pointed to by `p` and can turn it

              // into an `Owned`. Dropping the `Owned` will deallocate the object.

              drop(p.into_owned());

          });

      }

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-defer-destroy"></span>`unsafe fn defer_destroy<T>(&self, ptr: Shared<'_, T>)` — [`Shared`](atomic/index.md#shared)

  Stores a destructor for an object so that it can be deallocated and dropped at some point

  after all currently pinned threads get unpinned.

  

  This method first stores the destructor into the thread-local (or handle-local) cache. If

  this cache becomes full, some destructors are moved into the global cache. At the same

  time, some destructors from both local and global caches may get executed in order to

  incrementally clean up the caches as they fill up.

  

  There is no guarantee when exactly the destructor will be executed. The only guarantee is

  that it won't be executed until all currently pinned threads get unpinned. In theory, the

  destructor might never run, but the epoch-based garbage collection will make an effort to

  execute it reasonably soon.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, the destructor will simply be

  executed immediately.

  

  # Safety

  

  The object must not be reachable by other threads anymore, otherwise it might be still in

  use when the destructor runs.

  

  Apart from that, keep in mind that another thread may execute the destructor, so the object

  must be sendable to other threads.

  

  We intentionally didn't require `T: Send`, because Rust's type systems usually cannot prove

  `T: Send` for typical use cases. For example, consider the following code snippet, which

  exemplifies the typical use case of deferring the deallocation of a shared reference:

  

  ```ignore

  let shared = Owned::new(7i32).into_shared(guard);

  guard.defer_destroy(shared); // `Shared` is not `Send`!

  ```

  

  While `Shared` is not `Send`, it's safe for another thread to call the destructor, because

  it's called only after the grace period and `shared` is no longer shared with other

  threads. But we don't expect type systems to prove this.

  

  # Examples

  

  When a heap-allocated object in a data structure becomes unreachable, it has to be

  deallocated. However, the current thread and other threads may be still holding references

  on the stack to that same object. Therefore it cannot be deallocated before those references

  get dropped. This method can defer deallocation until all those threads get unpinned and

  consequently drop all their references on the stack.

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic, Owned};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new("foo");

  

  // Now suppose that `a` is shared among multiple threads and concurrently

  // accessed and modified...

  

  // Pin the current thread.

  let guard = &epoch::pin();

  

  // Steal the object currently stored in `a` and swap it with another one.

  let p = a.swap(Owned::new("bar").into_shared(guard), SeqCst, guard);

  

  if !p.is_null() {

      // The object `p` is pointing to is now unreachable.

      // Defer its deallocation until all currently pinned threads get unpinned.

      unsafe {

          guard.defer_destroy(p);

      }

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-flush"></span>`fn flush(&self)`

  Clears up the thread-local cache of deferred functions by executing them or moving into the

  global cache.

  

  Call this method after deferring execution of a function if you want to get it executed as

  soon as possible. Flushing will make sure it is residing in in the global cache, so that

  any thread has a chance of taking the function and executing it.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, it is a no-op (nothing happens).

  

  # Examples

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard = &epoch::pin();

  guard.defer(move || {

      println!("This better be printed as soon as possible!");

  });

  guard.flush();

  ```

- <span id="guard-repin"></span>`fn repin(&mut self)`

  Unpins and then immediately re-pins the thread.

  

  This method is useful when you don't want delay the advancement of the global epoch by

  holding an old epoch. For safety, you should not maintain any guard-based reference across

  the call (the latter is enforced by `&mut self`). The thread will only be repinned if this

  is the only active guard for the current thread.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, then the call will be just no-op.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic};

  use std::sync::atomic::Ordering::SeqCst;

  

  let a = Atomic::new(777);

  let mut guard = epoch::pin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  guard.repin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-repin-after"></span>`fn repin_after<F, R>(&mut self, f: F) -> R`

  Temporarily unpins the thread, executes the given function and then re-pins the thread.

  

  This method is useful when you need to perform a long-running operation (e.g. sleeping)

  and don't need to maintain any guard-based reference across the call (the latter is enforced

  by `&mut self`). The thread will only be unpinned if this is the only active guard for the

  current thread.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, then the passed function is called

  directly without unpinning the thread.

  

  # Examples

  

  ```rust

  use crossbeam_epoch::{self as epoch, Atomic};

  use std::sync::atomic::Ordering::SeqCst;

  use std::thread;

  use std::time::Duration;

  

  let a = Atomic::new(777);

  let mut guard = epoch::pin();

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  guard.repin_after(|| thread::sleep(Duration::from_millis(50)));

  {

      let p = a.load(SeqCst, &guard);

      assert_eq!(unsafe { p.as_ref() }, Some(&777));

  }

  unsafe { drop(a.into_owned()); } // avoid leak

  ```

- <span id="guard-collector"></span>`fn collector(&self) -> Option<&Collector>` — [`Collector`](collector/index.md#collector)

  Returns the `Collector` associated with this guard.

  

  This method is useful when you need to ensure that all guards used with

  a data structure come from the same collector.

  

  If this method is called from an [`unprotected`](guard/index.md) guard, then `None` is returned.

  

  # Examples

  

  ```rust

  use crossbeam_epoch as epoch;

  

  let guard1 = epoch::pin();

  let guard2 = epoch::pin();

  assert!(guard1.collector() == guard2.collector());

  ```

#### Trait Implementations

##### `impl Any for Guard`

- <span id="guard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Guard`

- <span id="guard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Guard`

- <span id="guard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Guard`

- <span id="guard-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Guard`

- <span id="guard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Guard`

- <span id="guard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Guard`

- <span id="guard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Guard`

- <span id="guard-pointable-const-align"></span>`const ALIGN: usize`

- <span id="guard-pointable-type-init"></span>`type Init = T`

- <span id="guard-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](atomic/index.md#pointable)

- <span id="guard-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="guard-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="guard-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Guard`

- <span id="guard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="guard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Guard`

- <span id="guard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="guard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `CompareAndSetOrdering`

```rust
trait CompareAndSetOrdering { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:67-76`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L67-L76)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:150-192`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L150-L192)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1029-1040`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L1029-L1040)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/guard.rs:513-523`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/guard.rs#L513-L523)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:50-52`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L50-L52)*

Returns the default global collector.

### `is_pinned`

```rust
fn is_pinned() -> bool
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:45-47`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L45-L47)*

Returns `true` if the current thread is pinned.

### `pin`

```rust
fn pin() -> crate::guard::Guard
```

*Defined in [`crossbeam-epoch-0.9.18/src/default.rs:39-41`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/default.rs#L39-L41)*

Pins the current thread.

## Type Aliases

### `CompareAndSetError<'g, T, P>`

```rust
type CompareAndSetError<'g, T, P> = CompareExchangeError<'g, T, P>;
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:31`](../../.source_1765633015/crossbeam-epoch-0.9.18/src/atomic.rs#L31)*

The error returned on failed compare-and-set operation.

