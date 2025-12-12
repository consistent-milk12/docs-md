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

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:16-20`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/list.rs#L16-L20)*

An entry in a linked list.

An Entry is accessed from multiple threads, so it would be beneficial to put it in a different
cache-line than thread-local data in terms of performance.

#### Fields

- **`next`**: `crate::Atomic<Entry>`

  The next entry in the linked list.
  If the tag is 1, this entry is marked as deleted.

#### Implementations

- <span id="entry-delete"></span>`unsafe fn delete(&self, guard: &Guard)` — [`Guard`](../../guard/index.md#guard)

#### Trait Implementations

##### `impl Debug for Entry`

- <span id="entry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Entry`

- <span id="entry-default"></span>`fn default() -> Self`

##### `impl Pointable for Entry`

- <span id="entry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="entry-pointable-type-init"></span>`type Init = T`

- <span id="entry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="entry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="entry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="entry-drop"></span>`unsafe fn drop(ptr: usize)`

### `List<T, C: IsElement<T>>`

```rust
struct List<T, C: IsElement<T>> {
    head: crate::Atomic<Entry>,
    _marker: core::marker::PhantomData<(T, C)>,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:96-102`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/list.rs#L96-L102)*

A lock-free, intrusive linked list of type `T`.

#### Fields

- **`head`**: `crate::Atomic<Entry>`

  The head of the linked list.

- **`_marker`**: `core::marker::PhantomData<(T, C)>`

  The phantom data for using `T` and `C`.

#### Implementations

- <span id="list-new"></span>`fn new() -> Self`

- <span id="list-insert"></span>`unsafe fn insert<'g>(self: &'g Self, container: Shared<'g, T>, guard: &'g Guard)` — [`Shared`](../../atomic/index.md#shared), [`Guard`](../../guard/index.md#guard)

- <span id="list-iter"></span>`fn iter<'g>(self: &'g Self, guard: &'g Guard) -> Iter<'g, T, C>` — [`Guard`](../../guard/index.md#guard), [`Iter`](#iter)

#### Trait Implementations

##### `impl<T: fmt::Debug, C: fmt::Debug + IsElement<T>> Debug for List<T, C>`

- <span id="list-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, C: IsElement<T>> Drop for List<T, C>`

- <span id="list-drop"></span>`fn drop(&mut self)`

##### `impl<T> Pointable for List<T, C>`

- <span id="list-pointable-const-align"></span>`const ALIGN: usize`

- <span id="list-pointable-type-init"></span>`type Init = T`

- <span id="list-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="list-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="list-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="list-drop"></span>`unsafe fn drop(ptr: usize)`

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

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:105-121`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/list.rs#L105-L121)*

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

##### `impl IntoIterator for Iter<'g, T, C>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'g, C: IsElement<T>> Iterator for Iter<'g, T, C>`

- <span id="iter-iterator-type-item"></span>`type Item = Result<&'g T, IterError>`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> Pointable for Iter<'g, T, C>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

## Enums

### `IterError`

```rust
enum IterError {
    Stalled,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:125-129`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/list.rs#L125-L129)*

An error that occurs during iteration over the list.

#### Variants

- **`Stalled`**

  A concurrent thread modified the state of the list at the same place that this iterator
  was inspecting. Subsequent iteration will restart from the beginning of the list.

#### Trait Implementations

##### `impl Debug for IterError`

- <span id="itererror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for IterError`

- <span id="itererror-eq"></span>`fn eq(&self, other: &IterError) -> bool` — [`IterError`](#itererror)

##### `impl Pointable for IterError`

- <span id="itererror-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itererror-pointable-type-init"></span>`type Init = T`

- <span id="itererror-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../atomic/index.md#pointable)

- <span id="itererror-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itererror-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itererror-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for IterError`

## Traits

### `IsElement<T>`

```rust
trait IsElement<T> { ... }
```

*Defined in [`crossbeam-epoch-0.9.18/src/sync/list.rs:67-92`](../../../../.source_1765521767/crossbeam-epoch-0.9.18/src/sync/list.rs#L67-L92)*

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

