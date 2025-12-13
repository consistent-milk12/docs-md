*[regex_automata](../../index.md) / [util](../index.md) / [sparse_set](index.md)*

---

# Module `sparse_set`

This module defines a sparse set data structure. Its most interesting
properties are:

* They preserve insertion order.
* Set membership testing is done in constant time.
* Set insertion is done in constant time.
* Clearing the set is done in constant time.

The cost for doing this is that the capacity of the set needs to be known up
front, and the elements in the set are limited to state identifiers.

These sets are principally used when traversing an NFA state graph. This
happens at search time, for example, in the PikeVM. It also happens during DFA
determinization.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SparseSets`](#sparsesets) | struct | A pair of sparse sets. |
| [`SparseSet`](#sparseset) | struct | A sparse set used for representing ordered NFA states. |
| [`SparseSetIter`](#sparsesetiter) | struct | An iterator over all elements in a sparse set. |

## Structs

### `SparseSets`

```rust
struct SparseSets {
    set1: SparseSet,
    set2: SparseSet,
}
```

*Defined in [`regex-automata-0.4.13/src/util/sparse_set.rs:36-39`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/sparse_set.rs#L36-L39)*

A pair of sparse sets.

This is useful when one needs to compute NFA epsilon closures from a
previous set of states derived from an epsilon closure. One set can be the
starting states where as the other set can be the destination states after
following the transitions for a particular byte of input.

There is no significance to 'set1' or 'set2'. They are both sparse sets of
the same size.

The members of this struct are exposed so that callers may borrow 'set1'
and 'set2' individually without being force to borrow both at the same
time.

#### Implementations

- <span id="sparsesets-new"></span>`fn new(capacity: usize) -> SparseSets` — [`SparseSets`](#sparsesets)

  Create a new pair of sparse sets where each set has the given capacity.

  

  This panics if the capacity given is bigger than `StateID::LIMIT`.

- <span id="sparsesets-resize"></span>`fn resize(&mut self, new_capacity: usize)`

  Resizes these sparse sets to have the new capacity given.

  

  The sets are automatically cleared.

  

  This panics if the capacity given is bigger than `StateID::LIMIT`.

- <span id="sparsesets-clear"></span>`fn clear(&mut self)`

  Clear both sparse sets.

- <span id="sparsesets-swap"></span>`fn swap(&mut self)`

  Swap set1 with set2.

- <span id="sparsesets-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the memory usage, in bytes, used by this pair of sparse sets.

#### Trait Implementations

##### `impl Any for SparseSets`

- <span id="sparsesets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseSets`

- <span id="sparsesets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseSets`

- <span id="sparsesets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SparseSets`

- <span id="sparsesets-clone"></span>`fn clone(&self) -> SparseSets` — [`SparseSets`](#sparsesets)

##### `impl CloneToUninit for SparseSets`

- <span id="sparsesets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SparseSets`

- <span id="sparsesets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SparseSets`

- <span id="sparsesets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseSets`

- <span id="sparsesets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SparseSets`

- <span id="sparsesets-toowned-type-owned"></span>`type Owned = T`

- <span id="sparsesets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sparsesets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SparseSets`

- <span id="sparsesets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparsesets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseSets`

- <span id="sparsesets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparsesets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SparseSet`

```rust
struct SparseSet {
    len: usize,
    dense: alloc::vec::Vec<crate::util::primitives::StateID>,
    sparse: alloc::vec::Vec<crate::util::primitives::StateID>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/sparse_set.rs:91-106`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/sparse_set.rs#L91-L106)*

A sparse set used for representing ordered NFA states.

This supports constant time addition and membership testing. Clearing an
entire set can also be done in constant time. Iteration yields elements
in the order in which they were inserted.

The data structure is based on: https://research.swtch.com/sparse
Note though that we don't actually use uninitialized memory. We generally
reuse sparse sets, so the initial allocation cost is bearable. However, its
other properties listed above are extremely useful.

#### Fields

- **`len`**: `usize`

  The number of elements currently in this set.

- **`dense`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  Dense contains the ids in the order in which they were inserted.

- **`sparse`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  Sparse maps ids to their location in dense.
  
  A state ID is in the set if and only if
  sparse[`id`](../../hybrid/id/index.md) < len && id == dense[sparse[`id`](../../hybrid/id/index.md)].
  
  Note that these are indices into 'dense'. It's a little weird to use
  StateID here, but we know our length can never exceed the bounds of
  StateID (enforced by 'resize') and StateID will be at most 4 bytes
  where as a usize is likely double that in most cases.

#### Implementations

- <span id="sparseset-new"></span>`fn new(capacity: usize) -> SparseSet` — [`SparseSet`](#sparseset)

  Create a new sparse set with the given capacity.

  

  Sparse sets have a fixed size and they cannot grow. Attempting to

  insert more distinct elements than the total capacity of the set will

  result in a panic.

  

  This panics if the capacity given is bigger than `StateID::LIMIT`.

- <span id="sparseset-resize"></span>`fn resize(&mut self, new_capacity: usize)`

  Resizes this sparse set to have the new capacity given.

  

  This set is automatically cleared.

  

  This panics if the capacity given is bigger than `StateID::LIMIT`.

- <span id="sparseset-capacity"></span>`fn capacity(&self) -> usize`

  Returns the capacity of this set.

  

  The capacity represents a fixed limit on the number of distinct

  elements that are allowed in this set. The capacity cannot be changed.

- <span id="sparseset-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in this set.

- <span id="sparseset-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this set is empty.

- <span id="sparseset-insert"></span>`fn insert(&mut self, id: StateID) -> bool` — [`StateID`](../primitives/index.md#stateid)

  Insert the state ID value into this set and return true if the given

  state ID was not previously in this set.

  

  This operation is idempotent. If the given value is already in this

  set, then this is a no-op.

  

  If more than `capacity` ids are inserted, then this panics.

  

  This is marked as inline(always) since the compiler won't inline it

  otherwise, and it's a fairly hot piece of code in DFA determinization.

- <span id="sparseset-contains"></span>`fn contains(&self, id: StateID) -> bool` — [`StateID`](../primitives/index.md#stateid)

  Returns true if and only if this set contains the given value.

- <span id="sparseset-clear"></span>`fn clear(&mut self)`

  Clear this set such that it has no members.

- <span id="sparseset-iter"></span>`fn iter(&self) -> SparseSetIter<'_>` — [`SparseSetIter`](#sparsesetiter)

- <span id="sparseset-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, used by this sparse set.

#### Trait Implementations

##### `impl Any for SparseSet`

- <span id="sparseset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseSet`

- <span id="sparseset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseSet`

- <span id="sparseset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SparseSet`

- <span id="sparseset-clone"></span>`fn clone(&self) -> SparseSet` — [`SparseSet`](#sparseset)

##### `impl CloneToUninit for SparseSet`

- <span id="sparseset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SparseSet`

- <span id="sparseset-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for SparseSet`

- <span id="sparseset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseSet`

- <span id="sparseset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SparseSet`

- <span id="sparseset-toowned-type-owned"></span>`type Owned = T`

- <span id="sparseset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sparseset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SparseSet`

- <span id="sparseset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparseset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseSet`

- <span id="sparseset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparseset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SparseSetIter<'a>`

```rust
struct SparseSetIter<'a>(core::slice::Iter<'a, crate::util::primitives::StateID>);
```

*Defined in [`regex-automata-0.4.13/src/util/sparse_set.rs:230`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/sparse_set.rs#L230)*

An iterator over all elements in a sparse set.

The lifetime `'a` refers to the lifetime of the set being iterated over.

#### Trait Implementations

##### `impl Any for SparseSetIter<'a>`

- <span id="sparsesetiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseSetIter<'a>`

- <span id="sparsesetiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseSetIter<'a>`

- <span id="sparsesetiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SparseSetIter<'a>`

- <span id="sparsesetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SparseSetIter<'a>`

- <span id="sparsesetiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseSetIter<'a>`

- <span id="sparsesetiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SparseSetIter<'a>`

- <span id="sparsesetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sparsesetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sparsesetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SparseSetIter<'a>`

- <span id="sparsesetiter-iterator-type-item"></span>`type Item = StateID`

- <span id="sparsesetiter-iterator-next"></span>`fn next(&mut self) -> Option<StateID>` — [`StateID`](../primitives/index.md#stateid)

##### `impl<U> TryFrom for SparseSetIter<'a>`

- <span id="sparsesetiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparsesetiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseSetIter<'a>`

- <span id="sparsesetiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparsesetiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

