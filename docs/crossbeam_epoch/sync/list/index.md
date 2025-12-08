*[crossbeam_epoch](../../index.md) / [sync](../index.md) / [list](index.md)*

---

# Module `list`

Lock-free intrusive linked list.

Ideas from Michael.  High Performance Dynamic Lock-Free Hash Tables and List-Based Sets.  SPAA
2002.  <http://dl.acm.org/citation.cfm?id=564870.564881>

## Structs

### `Entry`

```rust
struct Entry {
    next: crate::Atomic<Entry>,
}
```

An entry in a linked list.

An Entry is accessed from multiple threads, so it would be beneficial to put it in a different
cache-line than thread-local data in terms of performance.

#### Fields

- **`next`**: `crate::Atomic<Entry>`

  The next entry in the linked list.
  If the tag is 1, this entry is marked as deleted.

#### Implementations

- `unsafe fn delete(self: &Self, guard: &Guard)` — [`Guard`](../../index.md)

#### Trait Implementations

##### `impl Debug for Entry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Entry`

- `fn default() -> Self`

##### `impl<T> Pointable for Entry`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `List<T, C: IsElement<T>>`

```rust
struct List<T, C: IsElement<T>> {
    head: crate::Atomic<Entry>,
    _marker: core::marker::PhantomData<(T, C)>,
}
```

A lock-free, intrusive linked list of type `T`.

#### Fields

- **`head`**: `crate::Atomic<Entry>`

  The head of the linked list.

- **`_marker`**: `core::marker::PhantomData<(T, C)>`

  The phantom data for using `T` and `C`.

#### Implementations

- `fn new() -> Self`

- `unsafe fn insert<'g>(self: &'g Self, container: Shared<'g, T>, guard: &'g Guard)` — [`Shared`](../../index.md), [`Guard`](../../index.md)

- `fn iter<'g>(self: &'g Self, guard: &'g Guard) -> Iter<'g, T, C>` — [`Guard`](../../index.md), [`Iter`](#iter)

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug, C: $crate::fmt::Debug + IsElement<T>> Debug for List<T, C>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T, C: IsElement<T>> Drop for List<T, C>`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for List<T, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

##### `impl<I> IntoIterator for Iter<'g, T, C>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'g, T: 'g, C: IsElement<T>> Iterator for Iter<'g, T, C>`

- `type Item = Result<&'g T, IterError>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> Pointable for Iter<'g, T, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Enums

### `IterError`

```rust
enum IterError {
    Stalled,
}
```

An error that occurs during iteration over the list.

#### Variants

- **`Stalled`**

  A concurrent thread modified the state of the list at the same place that this iterator
  was inspecting. Subsequent iteration will restart from the beginning of the list.

#### Trait Implementations

##### `impl Debug for IterError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for IterError`

- `fn eq(self: &Self, other: &IterError) -> bool` — [`IterError`](#itererror)

##### `impl<T> Pointable for IterError`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for IterError`

## Traits

### `IsElement<T>`

```rust
trait IsElement<T> { ... }
```

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

