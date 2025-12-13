*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [list](index.md)*

---

# Module `list`

Lock-free intrusive linked list.

Ideas from Michael.  High Performance Dynamic Lock-Free Hash Tables and List-Based Sets.  SPAA
2002.  <http://dl.acm.org/citation.cfm?id=564870.564881>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Entry`](#entry) | struct | An entry in a linked list. |
| [`List`](#list) | struct | A lock-free, intrusive linked list of type `T`. |
| [`Iter`](#iter) | struct | An iterator used for retrieving values from the list. |
| [`IterError`](#itererror) | enum | An error that occurs during iteration over the list. |
| [`IsElement`](#iselement) | trait | Implementing this trait asserts that the type `T` can be used as an element in the intrusive linked list defined in this module. |

## Structs

### `Entry`

```rust
struct Entry {
    next: crate::Atomic<Entry>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:16-20`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/list.rs#L16-L20)*

An entry in a linked list.

An Entry is accessed from multiple threads, so it would be beneficial to put it in a different
cache-line than thread-local data in terms of performance.

#### Fields

- **`next`**: `crate::Atomic<Entry>`

  The next entry in the linked list.
  If the tag is 1, this entry is marked as deleted.

#### Implementations

- <span id="entry-delete"></span>`unsafe fn delete(&self, guard: &Guard)` — [`Guard`](../../guard/index.md#guard)

  Marks this entry as deleted, deferring the actual deallocation to a later iteration.

  

  # Safety

  

  The entry should be a member of a linked list, and it should not have been deleted.

  It should be safe to call `C::finalize` on the entry after the `guard` is dropped, where `C`

  is the associated helper for the linked list.

#### Trait Implementations

##### `impl Any for Entry`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Entry`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Entry`

- <span id="entry-default"></span>`fn default() -> Self`

  Returns the empty entry.

##### `impl<T> From for Entry`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Entry`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for Entry`

- <span id="entry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="entry-pointable-type-init"></span>`type Init = T`

- <span id="entry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="entry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="entry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="entry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Entry`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Entry`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `List<T, C: IsElement<T>>`

```rust
struct List<T, C: IsElement<T>> {
    head: crate::Atomic<Entry>,
    _marker: core::marker::PhantomData<(T, C)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:96-102`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/list.rs#L96-L102)*

A lock-free, intrusive linked list of type `T`.

#### Fields

- **`head`**: `crate::Atomic<Entry>`

  The head of the linked list.

- **`_marker`**: `core::marker::PhantomData<(T, C)>`

  The phantom data for using `T` and `C`.

#### Implementations

- <span id="list-new"></span>`fn new() -> Self`

  Returns a new, empty linked list.

- <span id="list-insert"></span>`unsafe fn insert<'g>(self: &'g Self, container: Shared<'g, T>, guard: &'g Guard)` — [`Shared`](../../atomic/index.md#shared), [`Guard`](../../guard/index.md#guard)

  Inserts `entry` into the head of the list.

  

  # Safety

  

  You should guarantee that:

  

  - `container` is not null

  - `container` is immovable, e.g. inside an `Owned`

  - the same `Entry` is not inserted more than once

  - the inserted object will be removed before the list is dropped

- <span id="list-iter"></span>`fn iter<'g>(self: &'g Self, guard: &'g Guard) -> Iter<'g, T, C>` — [`Guard`](../../guard/index.md#guard), [`Iter`](#iter)

  Returns an iterator over all objects.

  

  # Caveat

  

  Every object that is inserted at the moment this function is called and persists at least

  until the end of iteration will be returned. Since this iterator traverses a lock-free

  linked list that may be concurrently modified, some additional caveats apply:

  

  1. If a new object is inserted during iteration, it may or may not be returned.

  2. If an object is deleted during iteration, it may or may not be returned.

  3. The iteration may be aborted when it lost in a race condition. In this case, the winning

     thread will continue to iterate over the same list.

#### Trait Implementations

##### `impl<T> Any for List<T, C>`

- <span id="list-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for List<T, C>`

- <span id="list-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for List<T, C>`

- <span id="list-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, C: fmt::Debug + IsElement<T>> Debug for List<T, C>`

- <span id="list-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, C: IsElement<T>> Drop for List<T, C>`

- <span id="list-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for List<T, C>`

- <span id="list-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for List<T, C>`

- <span id="list-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Pointable for List<T, C>`

- <span id="list-pointable-const-align"></span>`const ALIGN: usize`

- <span id="list-pointable-type-init"></span>`type Init = T`

- <span id="list-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="list-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="list-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="list-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for List<T, C>`

- <span id="list-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="list-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for List<T, C>`

- <span id="list-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="list-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'g, T, C: IsElement<T>>`

```rust
struct Iter<'g, T, C: IsElement<T>> {
    guard: &'g crate::Guard,
    pred: &'g crate::Atomic<Entry>,
    curr: crate::Shared<'g, Entry>,
    head: &'g crate::Atomic<Entry>,
    _marker: core::marker::PhantomData<(&'g T, C)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:105-121`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/list.rs#L105-L121)*

An iterator used for retrieving values from the list.

#### Fields

- **`guard`**: `&'g crate::Guard`

  The guard that protects the iteration.

- **`pred`**: `&'g crate::Atomic<Entry>`

  Pointer from the predecessor to the current entry.

- **`curr`**: `crate::Shared<'g, Entry>`

  The current entry.

- **`head`**: `&'g crate::Atomic<Entry>`

  The list head, needed for restarting iteration.

- **`_marker`**: `core::marker::PhantomData<(&'g T, C)>`

  Logically, we store a borrow of an instance of `T` and
  use the type information from `C`.

#### Trait Implementations

##### `impl<T> Any for Iter<'g, T, C>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'g, T, C>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'g, T, C>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Iter<'g, T, C>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Iter<'g, T, C>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'g, T, C>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'g, C: IsElement<T>> Iterator for Iter<'g, T, C>`

- <span id="iter-iterator-type-item"></span>`type Item = Result<&'g T, IterError>`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> Pointable for Iter<'g, T, C>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Iter<'g, T, C>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<'g, T, C>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `IterError`

```rust
enum IterError {
    Stalled,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:125-129`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/list.rs#L125-L129)*

An error that occurs during iteration over the list.

#### Variants

- **`Stalled`**

  A concurrent thread modified the state of the list at the same place that this iterator
  was inspecting. Subsequent iteration will restart from the beginning of the list.

#### Trait Implementations

##### `impl Any for IterError`

- <span id="itererror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterError`

- <span id="itererror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterError`

- <span id="itererror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IterError`

- <span id="itererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterError`

- <span id="itererror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterError`

- <span id="itererror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for IterError`

- <span id="itererror-partialeq-eq"></span>`fn eq(&self, other: &IterError) -> bool` — [`IterError`](#itererror)

##### `impl Pointable for IterError`

- <span id="itererror-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itererror-pointable-type-init"></span>`type Init = T`

- <span id="itererror-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="itererror-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itererror-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itererror-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for IterError`

##### `impl<U> TryFrom for IterError`

- <span id="itererror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itererror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterError`

- <span id="itererror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itererror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IsElement<T>`

```rust
trait IsElement<T> { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:67-92`](../../../../.source_1765633015/crossbeam-epoch-0.9.18/src/sync/list.rs#L67-L92)*

Implementing this trait asserts that the type `T` can be used as an element in the intrusive
linked list defined in this module. `T` has to contain (or otherwise be linked to) an instance
of `Entry`.

# Example

```ignore
struct A {
    entry: Entry,
    data: usize,
}

impl IsElement<A> for A {
    fn entry_of(a: &A) -> &Entry {
        let entry_ptr = ((a as usize) + offset_of!(A, entry)) as *const Entry;
        unsafe { &*entry_ptr }
    }

    unsafe fn element_of(entry: &Entry) -> &T {
        let elem_ptr = ((entry as usize) - offset_of!(A, entry)) as *const T;
        &*elem_ptr
    }

    unsafe fn finalize(entry: &Entry, guard: &Guard) {
        guard.defer_destroy(Shared::from(Self::element_of(entry) as *const _));
    }
}
```

This trait is implemented on a type separate from `T` (although it can be just `T`), because
one type might be placeable into multiple lists, in which case it would require multiple
implementations of `IsElement`. In such cases, each struct implementing `IsElement<T>`
represents a distinct `Entry` in `T`.

For example, we can insert the following struct into two lists using `entry1` for one
and `entry2` for the other:

```ignore
struct B {
    entry1: Entry,
    entry2: Entry,
    data: usize,
}
```


#### Required Methods

- `fn entry_of(_: &T) -> &Entry`

  Returns a reference to this element's `Entry`.

- `fn element_of(_: &Entry) -> &T`

  Given a reference to an element's entry, returns that element.

- `fn finalize(_: &Entry, _: &Guard)`

  The function that is called when an entry is unlinked from list.

#### Implementors

- [`Local`](../../internal/index.md#local)

