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

## Structs

### `SparseSets`

```rust
struct SparseSets {
    set1: SparseSet,
    set2: SparseSet,
}
```

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

- `fn new(capacity: usize) -> SparseSets` — [`SparseSets`](#sparsesets)

- `fn resize(self: &mut Self, new_capacity: usize)`

- `fn clear(self: &mut Self)`

- `fn swap(self: &mut Self)`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for SparseSets`

- `fn clone(self: &Self) -> SparseSets` — [`SparseSets`](#sparsesets)

##### `impl Debug for SparseSets`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SparseSet`

```rust
struct SparseSet {
    len: usize,
    dense: alloc::vec::Vec<crate::util::primitives::StateID>,
    sparse: alloc::vec::Vec<crate::util::primitives::StateID>,
}
```

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

- `fn new(capacity: usize) -> SparseSet` — [`SparseSet`](#sparseset)

- `fn resize(self: &mut Self, new_capacity: usize)`

- `fn capacity(self: &Self) -> usize`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn insert(self: &mut Self, id: StateID) -> bool` — [`StateID`](../primitives/index.md)

- `fn contains(self: &Self, id: StateID) -> bool` — [`StateID`](../primitives/index.md)

- `fn clear(self: &mut Self)`

- `fn iter(self: &Self) -> SparseSetIter<'_>` — [`SparseSetIter`](#sparsesetiter)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for SparseSet`

- `fn clone(self: &Self) -> SparseSet` — [`SparseSet`](#sparseset)

##### `impl Debug for SparseSet`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SparseSetIter<'a>`

```rust
struct SparseSetIter<'a>(core::slice::Iter<'a, crate::util::primitives::StateID>);
```

An iterator over all elements in a sparse set.

The lifetime `'a` refers to the lifetime of the set being iterated over.

#### Trait Implementations

##### `impl<'a> Debug for SparseSetIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SparseSetIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for SparseSetIter<'a>`

- `type Item = StateID`

- `fn next(self: &mut Self) -> Option<StateID>` — [`StateID`](../primitives/index.md)

