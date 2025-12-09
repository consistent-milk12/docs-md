*[crossbeam_epoch](../index.md) / [atomic](index.md)*

---

# Module `atomic`

## Contents

- [Structs](#structs)
  - [`CompareExchangeError`](#compareexchangeerror)
  - [`Array`](#array)
  - [`Atomic`](#atomic)
  - [`Owned`](#owned)
  - [`Shared`](#shared)
- [Traits](#traits)
  - [`CompareAndSetOrdering`](#compareandsetordering)
  - [`Pointable`](#pointable)
  - [`Pointer`](#pointer)
- [Functions](#functions)
  - [`strongest_failure_ordering`](#strongest_failure_ordering)
  - [`low_bits`](#low_bits)
  - [`ensure_aligned`](#ensure_aligned)
  - [`compose_tag`](#compose_tag)
  - [`decompose_tag`](#decompose_tag)
- [Type Aliases](#type-aliases)
  - [`CompareAndSetError`](#compareandseterror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CompareExchangeError`](#compareexchangeerror) | struct | The error returned on failed compare-and-swap operation. |
| [`Array`](#array) | struct | Array with size. |
| [`Atomic`](#atomic) | struct | An atomic pointer that can be safely shared between threads. |
| [`Owned`](#owned) | struct | An owned heap-allocated object. |
| [`Shared`](#shared) | struct | A pointer to an object protected by the epoch GC. |
| [`CompareAndSetOrdering`](#compareandsetordering) | trait | Memory orderings for compare-and-set operations. |
| [`Pointable`](#pointable) | trait | Types that are pointed to by a single word. |
| [`Pointer`](#pointer) | trait | A trait for either `Owned` or `Shared` pointers. |
| [`strongest_failure_ordering`](#strongest_failure_ordering) | fn | Given ordering for the success case in a compare-exchange operation, returns the strongest appropriate ordering for the failure case. |
| [`low_bits`](#low_bits) | fn | Returns a bitmask containing the unused least significant bits of an aligned pointer to `T`. |
| [`ensure_aligned`](#ensure_aligned) | fn | Panics if the pointer is not properly unaligned. |
| [`compose_tag`](#compose_tag) | fn | Given a tagged pointer `data`, returns the same pointer, but tagged with `tag`. |
| [`decompose_tag`](#decompose_tag) | fn | Decomposes a tagged pointer `data` into the pointer and the tag. |
| [`CompareAndSetError`](#compareandseterror) | type | The error returned on failed compare-and-set operation. |

## Structs

### `CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>>`

```rust
struct CompareExchangeError<'g, T: ?Sized + Pointable, P: Pointer<T>> {
    pub current: Shared<'g, T>,
    pub new: P,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:34-40`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L34-L40)*

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

- <span id="compareexchangeerror-const-align"></span>`const ALIGN: usize`

- <span id="compareexchangeerror-type-init"></span>`type Init = T`

- <span id="compareexchangeerror-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="compareexchangeerror-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="compareexchangeerror-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="compareexchangeerror-drop"></span>`unsafe fn drop(ptr: usize)`

### `Array<T>`

```rust
struct Array<T> {
    len: usize,
    elements: [core::mem::MaybeUninit<T>; 0],
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:237-241`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L237-L241)*

Array with size.

# Memory layout

An array consisting of size and elements:

```text
         elements
         |
         |
------------------------------------
| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
------------------------------------
```

Its memory layout is different from that of `Box<[T]>` in that size is in the allocation (not
along with pointer as in `Box<[T]>`).

Elements are not present in the type, but they will be in the allocation.
```rust

#### Fields

- **`len`**: `usize`

  The number of elements (not the number of bytes).

#### Implementations

- <span id="array-layout"></span>`fn layout(len: usize) -> Layout`

#### Trait Implementations

##### `impl<T> Pointable for Array<T>`

- <span id="array-const-align"></span>`const ALIGN: usize`

- <span id="array-type-init"></span>`type Init = T`

- <span id="array-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="array-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="array-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="array-drop"></span>`unsafe fn drop(ptr: usize)`

### `Atomic<T: ?Sized + Pointable>`

```rust
struct Atomic<T: ?Sized + Pointable> {
    data: core::sync::atomic::AtomicUsize,
    _marker: core::marker::PhantomData<*mut T>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:294-297`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L294-L297)*

An atomic pointer that can be safely shared between threads.

The pointer must be properly aligned. Since it is aligned, a tag can be stored into the unused
least significant bits of the address. For example, the tag for a pointer to a sized type `T`
should be less than `(1 << mem::align_of::<T>().trailing_zeros())`.

Any method that loads the pointer must be passed a reference to a [`Guard`](../guard/index.md).

Crossbeam supports dynamically sized types.  See [`Pointable`](#pointable) for details.

#### Implementations

- <span id="atomic-new"></span>`fn new(init: T) -> Atomic<T>` — [`Atomic`](#atomic)

#### Trait Implementations

##### `impl<T: ?Sized + Pointable> Clone for Atomic<T>`

- <span id="atomic-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: ?Sized + Pointable> Debug for Atomic<T>`

- <span id="atomic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable> Default for Atomic<T>`

- <span id="atomic-default"></span>`fn default() -> Self`

##### `impl<T> Pointable for Atomic<T>`

- <span id="atomic-const-align"></span>`const ALIGN: usize`

- <span id="atomic-type-init"></span>`type Init = T`

- <span id="atomic-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="atomic-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomic-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomic-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Atomic<T>`

- <span id="atomic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable + Send + Sync> Send for Atomic<T>`

##### `impl<T: ?Sized + Pointable + Send + Sync> Sync for Atomic<T>`

### `Owned<T: ?Sized + Pointable>`

```rust
struct Owned<T: ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<alloc::boxed::Box<T>>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1048-1051`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1048-L1051)*

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

- <span id="owned-type-target"></span>`type Target = T`

- <span id="owned-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: ?Sized + Pointable> DerefMut for Owned<T>`

- <span id="owned-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: ?Sized + Pointable> Drop for Owned<T>`

- <span id="owned-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for Owned<T>`

- <span id="owned-const-align"></span>`const ALIGN: usize`

- <span id="owned-type-init"></span>`type Init = T`

- <span id="owned-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="owned-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="owned-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="owned-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Owned<T>`

- <span id="owned-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="owned-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

##### `impl<P, T> Receiver for Owned<T>`

- <span id="owned-type-target"></span>`type Target = T`

### `Shared<'g, T: 'g + ?Sized + Pointable>`

```rust
struct Shared<'g, T: 'g + ?Sized + Pointable> {
    data: usize,
    _marker: core::marker::PhantomData<(&'g (), *const T)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1297-1300`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1297-L1300)*

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

##### `impl<'g, T: ?Sized + Pointable> PartialEq for Shared<'g, T>`

- <span id="shared-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<'g, T: ?Sized + Pointable> PartialOrd for Shared<'g, T>`

- <span id="shared-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>`

##### `impl<T> Pointable for Shared<'g, T>`

- <span id="shared-const-align"></span>`const ALIGN: usize`

- <span id="shared-type-init"></span>`type Init = T`

- <span id="shared-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](#pointable)

- <span id="shared-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="shared-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="shared-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Shared<'_, T>`

- <span id="shared-into-usize"></span>`fn into_usize(self) -> usize`

- <span id="shared-from-usize"></span>`unsafe fn from_usize(data: usize) -> Self`

## Traits

### `CompareAndSetOrdering`

```rust
trait CompareAndSetOrdering { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:67-76`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L67-L76)*

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

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:150-192`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L150-L192)*

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

- `T`
- `[core::mem::MaybeUninit<T>]`

### `Pointer<T: ?Sized + Pointable>`

```rust
trait Pointer<T: ?Sized + Pointable> { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:1029-1040`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L1029-L1040)*

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

### `strongest_failure_ordering`

```rust
fn strongest_failure_ordering(ord: core::sync::atomic::Ordering) -> core::sync::atomic::Ordering
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:19-26`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L19-L26)*

Given ordering for the success case in a compare-exchange operation, returns the strongest
appropriate ordering for the failure case.

### `low_bits`

```rust
fn low_bits<T: ?Sized + Pointable>() -> usize
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:106-108`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L106-L108)*

Returns a bitmask containing the unused least significant bits of an aligned pointer to `T`.

### `ensure_aligned`

```rust
fn ensure_aligned<T: ?Sized + Pointable>(raw: usize)
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:112-114`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L112-L114)*

Panics if the pointer is not properly unaligned.

### `compose_tag`

```rust
fn compose_tag<T: ?Sized + Pointable>(data: usize, tag: usize) -> usize
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:120-122`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L120-L122)*

Given a tagged pointer `data`, returns the same pointer, but tagged with `tag`.

`tag` is truncated to fit into the unused bits of the pointer to `T`.

### `decompose_tag`

```rust
fn decompose_tag<T: ?Sized + Pointable>(data: usize) -> (usize, usize)
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:126-128`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L126-L128)*

Decomposes a tagged pointer `data` into the pointer and the tag.

## Type Aliases

### `CompareAndSetError<'g, T, P>`

```rust
type CompareAndSetError<'g, T, P> = CompareExchangeError<'g, T, P>;
```

*Defined in [`crossbeam-epoch-0.9.18/src/atomic.rs:31`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/atomic.rs#L31)*

The error returned on failed compare-and-set operation.

