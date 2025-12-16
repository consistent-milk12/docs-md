*[itertools](../index.md) / [groupbylazy](index.md)*

---

# Module `groupbylazy`

## Contents

- [Structs](#structs)
  - [`ChunkIndex`](#chunkindex)
  - [`GroupInner`](#groupinner)
  - [`ChunkBy`](#chunkby)
  - [`Groups`](#groups)
  - [`Group`](#group)
  - [`IntoChunks`](#intochunks)
  - [`Chunks`](#chunks)
  - [`Chunk`](#chunk)
- [Traits](#traits)
  - [`KeyFunction`](#keyfunction)
- [Functions](#functions)
  - [`new`](#new)
  - [`new_chunks`](#new-chunks)
- [Type Aliases](#type-aliases)
  - [`GroupBy`](#groupby)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ChunkIndex`](#chunkindex) | struct | `ChunkIndex` acts like the grouping key function for `IntoChunks` |
| [`GroupInner`](#groupinner) | struct |  |
| [`ChunkBy`](#chunkby) | struct | `ChunkBy` is the storage for the lazy grouping operation. |
| [`Groups`](#groups) | struct | An iterator that yields the Group iterators. |
| [`Group`](#group) | struct | An iterator for the elements in a single group. |
| [`IntoChunks`](#intochunks) | struct | `ChunkLazy` is the storage for a lazy chunking operation. |
| [`Chunks`](#chunks) | struct | An iterator that yields the Chunk iterators. |
| [`Chunk`](#chunk) | struct | An iterator for the elements in a single chunk. |
| [`KeyFunction`](#keyfunction) | trait | A trait to unify `FnMut` for `ChunkBy` with the chunk key in `IntoChunks` |
| [`new`](#new) | fn | Create a new |
| [`new_chunks`](#new-chunks) | fn | Create a new |
| [`GroupBy`](#groupby) | type | See [`ChunkBy`](crate::structs::ChunkBy). |

## Structs

### `ChunkIndex`

```rust
struct ChunkIndex {
    size: usize,
    index: usize,
    key: usize,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:23-27`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L23-L27)*

`ChunkIndex` acts like the grouping key function for `IntoChunks`

#### Implementations

- <span id="chunkindex-new"></span>`fn new(size: usize) -> Self`

#### Trait Implementations

##### `impl Any for ChunkIndex`

- <span id="chunkindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkIndex`

- <span id="chunkindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkIndex`

- <span id="chunkindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ChunkIndex`

- <span id="chunkindex-clone"></span>`fn clone(&self) -> ChunkIndex` — [`ChunkIndex`](#chunkindex)

##### `impl CloneToUninit for ChunkIndex`

- <span id="chunkindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ChunkIndex`

- <span id="chunkindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunkIndex`

- <span id="chunkindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChunkIndex`

- <span id="chunkindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChunkIndex`

##### `impl<A> KeyFunction for ChunkIndex`

- <span id="chunkindex-keyfunction-type-key"></span>`type Key = usize`

- <span id="chunkindex-keyfunction-call-mut"></span>`fn call_mut(&mut self, _arg: A) -> <Self as >::Key` — [`KeyFunction`](#keyfunction)

##### `impl ToOwned for ChunkIndex`

- <span id="chunkindex-toowned-type-owned"></span>`type Owned = T`

- <span id="chunkindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunkindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ChunkIndex`

- <span id="chunkindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChunkIndex`

- <span id="chunkindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GroupInner<K, I, F>`

```rust
struct GroupInner<K, I, F>
where
    I: Iterator {
    key: F,
    iter: I,
    current_key: Option<K>,
    current_elt: Option<<I as >::Item>,
    done: bool,
    top_group: usize,
    oldest_buffered_group: usize,
    bottom_group: usize,
    buffer: alloc::vec::Vec<vec::IntoIter<<I as >::Item>>,
    dropped_group: usize,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:54-77`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L54-L77)*

#### Fields

- **`done`**: `bool`

  flag set if iterator is exhausted

- **`top_group`**: `usize`

  Index of group we are currently buffering or visiting

- **`oldest_buffered_group`**: `usize`

  Least index for which we still have elements buffered

- **`bottom_group`**: `usize`

  Group index for `buffer[0]` -- the slots
  `bottom_group..oldest_buffered_group` are unused and will be erased when
  that range is large enough.

- **`buffer`**: `alloc::vec::Vec<vec::IntoIter<<I as >::Item>>`

  Buffered groups, from `bottom_group` (index 0) to `top_group`.

- **`dropped_group`**: `usize`

  index of last group iter that was dropped,
  `usize::MAX` initially when no group was dropped

#### Implementations

- <span id="groupinner-step"></span>`fn step(&mut self, client: usize) -> Option<<I as >::Item>`

  `client`: Index of group that requests next element

- <span id="groupinner-lookup-buffer"></span>`fn lookup_buffer(&mut self, client: usize) -> Option<<I as >::Item>`

- <span id="groupinner-next-element"></span>`fn next_element(&mut self) -> Option<<I as >::Item>`

  Take the next element from the iterator, and set the done
  flag if exhausted. Must not be called after done.

- <span id="groupinner-step-buffering"></span>`fn step_buffering(&mut self, client: usize) -> Option<<I as >::Item>`

- <span id="groupinner-push-next-group"></span>`fn push_next_group(&mut self, group: Vec<<I as >::Item>)`

- <span id="groupinner-step-current"></span>`fn step_current(&mut self) -> Option<<I as >::Item>`

  This is the immediate case, where we use no buffering

- <span id="groupinner-group-key"></span>`fn group_key(&mut self, client: usize) -> K`

  Request the just started groups' key.
  
  `client`: Index of group
  
  **Panics** if no group key is available.

#### Trait Implementations

##### `impl Any for GroupInner<K, I, F>`

- <span id="groupinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupInner<K, I, F>`

- <span id="groupinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupInner<K, I, F>`

- <span id="groupinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: clone::Clone, I, F: clone::Clone> Clone for GroupInner<K, I, F>`

- <span id="groupinner-clone"></span>`fn clone(&self) -> GroupInner<K, I, F>` — [`GroupInner`](#groupinner)

##### `impl CloneToUninit for GroupInner<K, I, F>`

- <span id="groupinner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for GroupInner<K, I, F>`

- <span id="groupinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupInner<K, I, F>`

- <span id="groupinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GroupInner<K, I, F>`

##### `impl ToOwned for GroupInner<K, I, F>`

- <span id="groupinner-toowned-type-owned"></span>`type Owned = T`

- <span id="groupinner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupinner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupInner<K, I, F>`

- <span id="groupinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupInner<K, I, F>`

- <span id="groupinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkBy<K, I, F>`

```rust
struct ChunkBy<K, I, F>
where
    I: Iterator {
    inner: std::cell::RefCell<GroupInner<K, I, F>>,
    index: std::cell::Cell<usize>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:304-312`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L304-L312)*

`ChunkBy` is the storage for the lazy grouping operation.

If the groups are consumed in their original order, or if each
group is dropped without keeping it around, then `ChunkBy` uses
no allocations. It needs allocations only if several group iterators
are alive at the same time.

This type implements `IntoIterator` (it is **not** an iterator
itself), because the group iterators need to borrow from this
value. It should be stored in a local variable or temporary and
iterated.

See [`.chunk_by()`](crate::Itertools::chunk_by) for more information.

#### Implementations

- <span id="chunkby-step"></span>`fn step(&self, client: usize) -> Option<<I as >::Item>`

  `client`: Index of group that requests next element

- <span id="chunkby-drop-group"></span>`fn drop_group(&self, client: usize)`

  `client`: Index of group

#### Trait Implementations

##### `impl Any for ChunkBy<K, I, F>`

- <span id="chunkby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkBy<K, I, F>`

- <span id="chunkby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkBy<K, I, F>`

- <span id="chunkby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunkBy<K, I, F>`

- <span id="chunkby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChunkBy<K, I, F>`

- <span id="chunkby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChunkBy<K, I, F>`

##### `impl<K, I, F> IntoIterator for &'a ChunkBy<K, I, F>`

- <span id="a-chunkby-intoiterator-type-item"></span>`type Item = (K, Group<'a, K, I, F>)`

- <span id="a-chunkby-intoiterator-type-intoiter"></span>`type IntoIter = Groups<'a, K, I, F>`

- <span id="a-chunkby-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<U> TryFrom for ChunkBy<K, I, F>`

- <span id="chunkby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChunkBy<K, I, F>`

- <span id="chunkby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Groups<'a, K, I, F>`

```rust
struct Groups<'a, K, I, F>
where
    I: Iterator + 'a,
    <I as >::Item: 'a,
    K: 'a,
    F: 'a {
    parent: &'a ChunkBy<K, I, F>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:378-386`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L378-L386)*

An iterator that yields the Group iterators.

Iterator element type is `(K, Group)`:
the group's key `K` and the group's iterator.

See [`.chunk_by()`](crate::Itertools::chunk_by) for more information.

#### Trait Implementations

##### `impl Any for Groups<'a, K, I, F>`

- <span id="groups-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Groups<'a, K, I, F>`

- <span id="groups-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Groups<'a, K, I, F>`

- <span id="groups-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Groups<'a, K, I, F>`

- <span id="groups-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Groups<'a, K, I, F>`

- <span id="groups-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Groups<'a, K, I, F>`

##### `impl<I> IntoIterator for Groups<'a, K, I, F>`

- <span id="groups-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="groups-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="groups-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, I, F> Iterator for Groups<'a, K, I, F>`

- <span id="groups-iterator-type-item"></span>`type Item = (K, Group<'a, K, I, F>)`

- <span id="groups-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Groups<'a, K, I, F>`

##### `impl<FromA, FromB> MultiUnzip for Groups<'a, K, I, F>`

- <span id="groups-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl<U> TryFrom for Groups<'a, K, I, F>`

- <span id="groups-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groups-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Groups<'a, K, I, F>`

- <span id="groups-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groups-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Group<'a, K, I, F>`

```rust
struct Group<'a, K, I, F>
where
    I: Iterator + 'a,
    <I as >::Item: 'a,
    K: 'a,
    F: 'a {
    parent: &'a ChunkBy<K, I, F>,
    index: usize,
    first: Option<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:419-429`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L419-L429)*

An iterator for the elements in a single group.

Iterator element type is `I::Item`.

#### Trait Implementations

##### `impl Any for Group<'a, K, I, F>`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group<'a, K, I, F>`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group<'a, K, I, F>`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, I, F> Drop for Group<'a, K, I, F>`

- <span id="group-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Group<'a, K, I, F>`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Group<'a, K, I, F>`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Group<'a, K, I, F>`

##### `impl<I> IntoIterator for Group<'a, K, I, F>`

- <span id="group-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="group-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="group-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, I, F> Iterator for Group<'a, K, I, F>`

- <span id="group-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="group-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Group<'a, K, I, F>`

##### `impl MultiUnzip for Group<'a, K, I, F>`

- <span id="group-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Group<'a, K, I, F>`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group<'a, K, I, F>`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoChunks<I>`

```rust
struct IntoChunks<I>
where
    I: Iterator {
    inner: std::cell::RefCell<GroupInner<usize, I, ChunkIndex>>,
    index: std::cell::Cell<usize>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:496-504`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L496-L504)*

`ChunkLazy` is the storage for a lazy chunking operation.

`IntoChunks` behaves just like `ChunkBy`: it is iterable, and
it only buffers if several chunk iterators are alive at the same time.

This type implements `IntoIterator` (it is **not** an iterator
itself), because the chunk iterators need to borrow from this
value. It should be stored in a local variable or temporary and
iterated.

Iterator element type is `Chunk`, each chunk's iterator.

See [`.chunks()`](crate::Itertools::chunks) for more information.

#### Implementations

- <span id="intochunks-step"></span>`fn step(&self, client: usize) -> Option<<I as >::Item>`

  `client`: Index of chunk that requests next element

- <span id="intochunks-drop-group"></span>`fn drop_group(&self, client: usize)`

  `client`: Index of chunk

#### Trait Implementations

##### `impl Any for IntoChunks<I>`

- <span id="intochunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoChunks<I>`

- <span id="intochunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoChunks<I>`

- <span id="intochunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for IntoChunks<I>`

- <span id="intochunks-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for IntoChunks<I>`

- <span id="intochunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for IntoChunks<I>`

- <span id="intochunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntoChunks<I>`

- <span id="intochunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntoChunks<I>`

##### `impl<I> IntoIterator for &'a IntoChunks<I>`

- <span id="a-intochunks-intoiterator-type-item"></span>`type Item = Chunk<'a, I>`

- <span id="a-intochunks-intoiterator-type-intoiter"></span>`type IntoIter = Chunks<'a, I>`

- <span id="a-intochunks-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToOwned for IntoChunks<I>`

- <span id="intochunks-toowned-type-owned"></span>`type Owned = T`

- <span id="intochunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intochunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntoChunks<I>`

- <span id="intochunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intochunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoChunks<I>`

- <span id="intochunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intochunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunks<'a, I>`

```rust
struct Chunks<'a, I>
where
    I: Iterator + 'a,
    <I as >::Item: 'a {
    parent: &'a IntoChunks<I>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:549-555`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L549-L555)*

An iterator that yields the Chunk iterators.

Iterator element type is `Chunk`.

See [`.chunks()`](crate::Itertools::chunks) for more information.

#### Trait Implementations

##### `impl Any for Chunks<'a, I>`

- <span id="chunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunks<'a, I>`

- <span id="chunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunks<'a, I>`

- <span id="chunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Chunks<'a, I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<'a, I>` — [`Chunks`](#chunks)

##### `impl CloneToUninit for Chunks<'a, I>`

- <span id="chunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Chunks<'a, I>`

- <span id="chunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chunks<'a, I>`

- <span id="chunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunks<'a, I>`

##### `impl<I> IntoIterator for Chunks<'a, I>`

- <span id="chunks-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunks-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chunks-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Chunks<'a, I>`

- <span id="chunks-iterator-type-item"></span>`type Item = Chunk<'a, I>`

- <span id="chunks-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Chunks<'a, I>`

##### `impl ToOwned for Chunks<'a, I>`

- <span id="chunks-toowned-type-owned"></span>`type Owned = T`

- <span id="chunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chunks<'a, I>`

- <span id="chunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunks<'a, I>`

- <span id="chunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunk<'a, I>`

```rust
struct Chunk<'a, I>
where
    I: Iterator + 'a,
    <I as >::Item: 'a {
    parent: &'a IntoChunks<I>,
    index: usize,
    first: Option<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:580-588`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L580-L588)*

An iterator for the elements in a single chunk.

Iterator element type is `I::Item`.

#### Trait Implementations

##### `impl Any for Chunk<'a, I>`

- <span id="chunk-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunk<'a, I>`

- <span id="chunk-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunk<'a, I>`

- <span id="chunk-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Drop for Chunk<'a, I>`

- <span id="chunk-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Chunk<'a, I>`

- <span id="chunk-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chunk<'a, I>`

- <span id="chunk-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunk<'a, I>`

##### `impl<I> IntoIterator for Chunk<'a, I>`

- <span id="chunk-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunk-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chunk-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Chunk<'a, I>`

- <span id="chunk-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunk-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Chunk<'a, I>`

##### `impl MultiUnzip for Chunk<'a, I>`

- <span id="chunk-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Chunk<'a, I>`

- <span id="chunk-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunk-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunk<'a, I>`

- <span id="chunk-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunk-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `KeyFunction<A>`

```rust
trait KeyFunction<A> { ... }
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:5-8`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L5-L8)*

A trait to unify `FnMut` for `ChunkBy` with the chunk key in `IntoChunks`

#### Associated Types

- `type Key`

#### Required Methods

- `fn KeyFunction::call_mut(&mut self, arg: A) -> <Self as >::Key`

#### Implementors

- [`ChunkIndex`](#chunkindex)
- `F`

## Functions

### `new`

```rust
fn new<K, J, F>(iter: J, f: F) -> ChunkBy<K, <J as >::IntoIter, F>
where
    J: IntoIterator,
    F: FnMut(&<J as >::Item) -> K
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:315-335`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L315-L335)*

Create a new

### `new_chunks`

```rust
fn new_chunks<J>(iter: J, size: usize) -> IntoChunks<<J as >::IntoIter>
where
    J: IntoIterator
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:461-480`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L461-L480)*

Create a new

## Type Aliases

### `GroupBy<K, I, F>`

```rust
type GroupBy<K, I, F> = ChunkBy<K, I, F>;
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:288`](../../../.source_1765900590/itertools-0.14.0/src/groupbylazy.rs#L288)*

See [`ChunkBy`](crate::structs::ChunkBy).

