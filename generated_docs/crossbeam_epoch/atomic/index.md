*[crossbeam_epoch](../index.md) / [atomic](index.md)*

---

# Module `atomic`

## Structs

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

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Array<T>`

```rust
struct Array<T> {
    len: usize,
    elements: [core::mem::MaybeUninit<T>; 0],
}
```

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

- `fn layout(len: usize) -> Layout`

#### Trait Implementations

##### `impl<T> Pointable for Array<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

Any method that loads the pointer must be passed a reference to a [`Guard`](../index.md).

Crossbeam supports dynamically sized types.  See [`Pointable`](../index.md) for details.

#### Implementations

- `fn init(init: <T as >::Init) -> Atomic<T>` — [`Pointable`](../index.md), [`Atomic`](../index.md)

- `fn from_usize(data: usize) -> Self`

- `const fn null() -> Atomic<T>` — [`Atomic`](../index.md)

- `fn load<'g>(self: &Self, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn load_consume<'g>(self: &Self, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn store<P: Pointer<T>>(self: &Self, new: P, ord: Ordering)`

- `fn swap<'g, P: Pointer<T>>(self: &Self, new: P, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn compare_exchange<'g, P>(self: &Self, current: Shared<'_, T>, new: P, success: Ordering, failure: Ordering, _: &'g Guard) -> Result<Shared<'g, T>, CompareExchangeError<'g, T, P>>` — [`Shared`](../index.md), [`Guard`](../index.md), [`CompareExchangeError`](../index.md)

- `fn compare_exchange_weak<'g, P>(self: &Self, current: Shared<'_, T>, new: P, success: Ordering, failure: Ordering, _: &'g Guard) -> Result<Shared<'g, T>, CompareExchangeError<'g, T, P>>` — [`Shared`](../index.md), [`Guard`](../index.md), [`CompareExchangeError`](../index.md)

- `fn fetch_update<'g, F>(self: &Self, set_order: Ordering, fail_order: Ordering, guard: &'g Guard, func: F) -> Result<Shared<'g, T>, Shared<'g, T>>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn compare_and_set<'g, O, P>(self: &Self, current: Shared<'_, T>, new: P, ord: O, guard: &'g Guard) -> Result<Shared<'g, T>, CompareAndSetError<'g, T, P>>` — [`Shared`](../index.md), [`Guard`](../index.md), [`CompareAndSetError`](../index.md)

- `fn compare_and_set_weak<'g, O, P>(self: &Self, current: Shared<'_, T>, new: P, ord: O, guard: &'g Guard) -> Result<Shared<'g, T>, CompareAndSetError<'g, T, P>>` — [`Shared`](../index.md), [`Guard`](../index.md), [`CompareAndSetError`](../index.md)

- `fn fetch_and<'g>(self: &Self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn fetch_or<'g>(self: &Self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn fetch_xor<'g>(self: &Self, val: usize, ord: Ordering, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `unsafe fn into_owned(self: Self) -> Owned<T>` — [`Owned`](../index.md)

- `unsafe fn try_into_owned(self: Self) -> Option<Owned<T>>` — [`Owned`](../index.md)

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

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Atomic<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ?Sized + Pointable + Send + Sync> Send for Atomic<T>`

##### `impl<T: ?Sized + Pointable + Send + Sync> Sync for Atomic<T>`

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

- `fn init(init: <T as >::Init) -> Owned<T>` — [`Pointable`](../index.md), [`Owned`](../index.md)

- `fn into_shared<'g>(self: Self, _: &'g Guard) -> Shared<'g, T>` — [`Guard`](../index.md), [`Shared`](../index.md)

- `fn tag(self: &Self) -> usize`

- `fn with_tag(self: Self, tag: usize) -> Owned<T>` — [`Owned`](../index.md)

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

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

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

- `fn as_raw(self: &Self) -> *const T`

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

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: ?Sized + Pointable> Pointer for Shared<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn success(self: &Self) -> Ordering`

  The ordering of the operation when it succeeds.

- `fn failure(self: &Self) -> Ordering`

  The ordering of the operation when it fails.

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

Pointers to `Pointable` types can be stored in [`Atomic`](../index.md), [`Owned`](../index.md), and [`Shared`](../index.md).  In
particular, Crossbeam supports dynamically sized slices as follows.

```rust
use std::mem::MaybeUninit;
use crossbeam_epoch::Owned;

let o = Owned::<[MaybeUninit<i32>]>::init(10); // allocating [i32; 10]
```

#### Required Methods

- `const ALIGN: usize`

- `type Init`

- `fn init(init: <Self as >::Init) -> usize`

  Initializes a with the given initializer.

- `fn deref<'a>(ptr: usize) -> &'a Self`

  Dereferences the given pointer.

- `fn deref_mut<'a>(ptr: usize) -> &'a mut Self`

  Mutably dereferences the given pointer.

- `fn drop(ptr: usize)`

  Drops the object pointed to by the given pointer.

### `Pointer<T: ?Sized + Pointable>`

```rust
trait Pointer<T: ?Sized + Pointable> { ... }
```

A trait for either `Owned` or `Shared` pointers.

#### Required Methods

- `fn into_usize(self: Self) -> usize`

  Returns the machine representation of the pointer.

- `fn from_usize(data: usize) -> Self`

  Returns a new pointer pointing to the tagged pointer `data`.

## Functions

### `strongest_failure_ordering`

```rust
fn strongest_failure_ordering(ord: core::sync::atomic::Ordering) -> core::sync::atomic::Ordering
```

Given ordering for the success case in a compare-exchange operation, returns the strongest
appropriate ordering for the failure case.

### `low_bits`

```rust
fn low_bits<T: ?Sized + Pointable>() -> usize
```

Returns a bitmask containing the unused least significant bits of an aligned pointer to `T`.

### `ensure_aligned`

```rust
fn ensure_aligned<T: ?Sized + Pointable>(raw: usize)
```

Panics if the pointer is not properly unaligned.

### `compose_tag`

```rust
fn compose_tag<T: ?Sized + Pointable>(data: usize, tag: usize) -> usize
```

Given a tagged pointer `data`, returns the same pointer, but tagged with `tag`.

`tag` is truncated to fit into the unused bits of the pointer to `T`.

### `decompose_tag`

```rust
fn decompose_tag<T: ?Sized + Pointable>(data: usize) -> (usize, usize)
```

Decomposes a tagged pointer `data` into the pointer and the tag.

## Type Aliases

### `CompareAndSetError<'g, T, P>`

```rust
type CompareAndSetError<'g, T, P> = CompareExchangeError<'g, T, P>;
```

The error returned on failed compare-and-set operation.

