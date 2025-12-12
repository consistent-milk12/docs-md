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

- <span id="sparsesets-resize"></span>`fn resize(&mut self, new_capacity: usize)`

- <span id="sparsesets-clear"></span>`fn clear(&mut self)`

- <span id="sparsesets-swap"></span>`fn swap(&mut self)`

- <span id="sparsesets-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for SparseSets`

- <span id="sparsesets-clone"></span>`fn clone(&self) -> SparseSets` — [`SparseSets`](#sparsesets)

##### `impl Debug for SparseSets`

- <span id="sparsesets-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sparseset-resize"></span>`fn resize(&mut self, new_capacity: usize)`

- <span id="sparseset-capacity"></span>`fn capacity(&self) -> usize`

- <span id="sparseset-len"></span>`fn len(&self) -> usize`

- <span id="sparseset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="sparseset-insert"></span>`fn insert(&mut self, id: StateID) -> bool` — [`StateID`](../primitives/index.md#stateid)

- <span id="sparseset-contains"></span>`fn contains(&self, id: StateID) -> bool` — [`StateID`](../primitives/index.md#stateid)

- <span id="sparseset-clear"></span>`fn clear(&mut self)`

- <span id="sparseset-iter"></span>`fn iter(&self) -> SparseSetIter<'_>` — [`SparseSetIter`](#sparsesetiter)

- <span id="sparseset-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for SparseSet`

- <span id="sparseset-clone"></span>`fn clone(&self) -> SparseSet` — [`SparseSet`](#sparseset)

##### `impl Debug for SparseSet`

- <span id="sparseset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SparseSetIter<'a>`

```rust
struct SparseSetIter<'a>(core::slice::Iter<'a, crate::util::primitives::StateID>);
```

*Defined in [`regex-automata-0.4.13/src/util/sparse_set.rs:230`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/sparse_set.rs#L230)*

An iterator over all elements in a sparse set.

The lifetime `'a` refers to the lifetime of the set being iterated over.

#### Trait Implementations

##### `impl Debug for SparseSetIter<'a>`

- <span id="sparsesetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SparseSetIter<'a>`

- <span id="sparsesetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sparsesetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="sparsesetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SparseSetIter<'a>`

- <span id="sparsesetiter-iterator-type-item"></span>`type Item = StateID`

- <span id="sparsesetiter-next"></span>`fn next(&mut self) -> Option<StateID>` — [`StateID`](../primitives/index.md#stateid)

