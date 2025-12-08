*[hashbrown](../index.md) / [hash_table](index.md)*

---

# Module `hash_table`

A hash table implemented with quadratic probing and SIMD lookup.

## Contents

- [Structs](#structs)
  - [`HashTable`](#hashtable)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
  - [`AbsentEntry`](#absententry)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IterBuckets`](#iterbuckets)
  - [`IterHash`](#iterhash)
  - [`IterHashMut`](#iterhashmut)
  - [`IterHashBuckets`](#iterhashbuckets)
  - [`IntoIter`](#intoiter)
  - [`Drain`](#drain)
  - [`ExtractIf`](#extractif)
- [Enums](#enums)
  - [`Entry`](#entry)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashTable`](#hashtable) | struct | Low-level hash table with explicit hashing. |
| [`OccupiedEntry`](#occupiedentry) | struct | A view into an occupied entry in a `HashTable`. |
| [`VacantEntry`](#vacantentry) | struct | A view into a vacant entry in a `HashTable`. |
| [`AbsentEntry`](#absententry) | struct | Type representing the absence of an entry, as returned by [`HashTable::find_entry`] |
| [`Iter`](#iter) | struct | An iterator over the entries of a `HashTable` in arbitrary order. |
| [`IterMut`](#itermut) | struct | A mutable iterator over the entries of a `HashTable` in arbitrary order. |
| [`IterBuckets`](#iterbuckets) | struct | An iterator producing the `usize` indices of all occupied buckets |
| [`IterHash`](#iterhash) | struct | An iterator over the entries of a `HashTable` that could match a given hash. |
| [`IterHashMut`](#iterhashmut) | struct | A mutable iterator over the entries of a `HashTable` that could match a given hash. |
| [`IterHashBuckets`](#iterhashbuckets) | struct | An iterator producing the `usize` indices of all buckets which may match a hash. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the entries of a `HashTable` in arbitrary order. |
| [`Drain`](#drain) | struct | A draining iterator over the items of a `HashTable`. |
| [`ExtractIf`](#extractif) | struct | A draining iterator over entries of a `HashTable` which don't satisfy the predicate `f`. |
| [`Entry`](#entry) | enum | A view into a single entry in a table, which may either be vacant or occupied. |

## Structs

### `HashTable<T, A>`

```rust
struct HashTable<T, A>
where
    A: Allocator {
    raw: crate::raw::RawTable<T, A>,
}
```

Low-level hash table with explicit hashing.

The primary use case for this type over [`HashMap`](../index.md) or [`HashSet`](../index.md) is to
support types that do not implement the `Hash` and `Eq` traits, but
instead require additional data not contained in the key itself to compute a
hash and compare two elements for equality.

Examples of when this can be useful include:
- An `IndexMap` implementation where indices into a `Vec` are stored as
  elements in a `HashTable<usize>`. Hashing and comparing the elements
  requires indexing the associated `Vec` to get the actual value referred to
  by the index.
- Avoiding re-computing a hash when it is already known.
- Mutating the key of an element in a way that doesn't affect its hash.

To achieve this, `HashTable` methods that search for an element in the table
require a hash value and equality function to be explicitly passed in as
arguments. The method will then iterate over the elements with the given
hash and call the equality function on each of them, until a match is found.

In most cases, a `HashTable` will not be exposed directly in an API. It will
instead be wrapped in a helper type which handles the work of calculating
hash values and comparing elements.

Due to its low-level nature, this type provides fewer guarantees than
[`HashMap`](../index.md) and [`HashSet`](../index.md). Specifically, the API allows you to shoot
yourself in the foot by having multiple elements with identical keys in the
table. The table itself will still function correctly and lookups will
arbitrarily return one of the matching elements. However you should avoid
doing this because it changes the runtime of hash table operations from
`O(1)` to `O(k)` where `k` is the number of duplicate entries.





#### Implementations

- <span id="hashtable-new"></span>`const fn new() -> Self`

- <span id="hashtable-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T, A> Clone for HashTable<T, A>`

- <span id="hashtable-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, A> Debug for HashTable<T, A>`

- <span id="hashtable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A> Default for HashTable<T, A>`

- <span id="hashtable-default"></span>`fn default() -> Self`

##### `impl<T, A> IntoIterator for HashTable<T, A>`

- <span id="hashtable-item"></span>`type Item = T`

- <span id="hashtable-intoiter"></span>`type IntoIter = IntoIter<T, A>`

- <span id="hashtable-into-iter"></span>`fn into_iter(self) -> IntoIter<T, A>` — [`IntoIter`](#intoiter)

### `OccupiedEntry<'a, T, A>`

```rust
struct OccupiedEntry<'a, T, A>
where
    A: Allocator {
    bucket: crate::raw::Bucket<T>,
    table: &'a mut HashTable<T, A>,
}
```

A view into an occupied entry in a `HashTable`.
It is part of the [`Entry`](../hash_set/index.md) enum.

# Examples

```rust
#[cfg(feature = "nightly")]
fn test() {
use hashbrown::hash_table::{Entry, OccupiedEntry};
use hashbrown::{HashTable, DefaultHashBuilder};
use std::hash::BuildHasher;

let mut table = HashTable::new();
let hasher = DefaultHashBuilder::default();
let hasher = |val: &_| hasher.hash_one(val);
for x in ["a", "b", "c"] {
    table.insert_unique(hasher(&x), x, hasher);
}
assert_eq!(table.len(), 3);

let _entry_o: OccupiedEntry<_, _> = table.find_entry(hasher(&"a"), |&x| x == "a").unwrap();
assert_eq!(table.len(), 3);

// Existing key
match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(view) => {
        assert_eq!(view.get(), &"a");
    }
}

assert_eq!(table.len(), 3);

// Existing key (take)
match table.entry(hasher(&"c"), |&x| x == "c", hasher) {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(view) => {
        assert_eq!(view.remove().0, "c");
    }
}
assert_eq!(table.find(hasher(&"c"), |&x| x == "c"), None);
assert_eq!(table.len(), 2);
}
fn main() {
    #[cfg(feature = "nightly")]
    test()
}
```

#### Implementations

- <span id="occupiedentry-remove"></span>`fn remove(self) -> (T, VacantEntry<'a, T, A>)` — [`VacantEntry`](#vacantentry)

- <span id="occupiedentry-get"></span>`fn get(&self) -> &T`

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut T`

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut T`

- <span id="occupiedentry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

- <span id="occupiedentry-bucket-index"></span>`fn bucket_index(&self) -> usize`

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for OccupiedEntry<'_, T, A>`

- <span id="occupiedentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A> Send for OccupiedEntry<'_, T, A>`

##### `impl<T, A> Sync for OccupiedEntry<'_, T, A>`

### `VacantEntry<'a, T, A>`

```rust
struct VacantEntry<'a, T, A>
where
    A: Allocator {
    tag: self::tag::Tag,
    index: usize,
    table: &'a mut HashTable<T, A>,
}
```

A view into a vacant entry in a `HashTable`.
It is part of the [`Entry`](../hash_set/index.md) enum.

# Examples

```rust
#[cfg(feature = "nightly")]
fn test() {
use hashbrown::hash_table::{Entry, VacantEntry};
use hashbrown::{HashTable, DefaultHashBuilder};
use std::hash::BuildHasher;

let mut table: HashTable<&str> = HashTable::new();
let hasher = DefaultHashBuilder::default();
let hasher = |val: &_| hasher.hash_one(val);

let entry_v: VacantEntry<_, _> = match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
    Entry::Vacant(view) => view,
    Entry::Occupied(_) => unreachable!(),
};
entry_v.insert("a");
assert!(table.find(hasher(&"a"), |&x| x == "a").is_some() && table.len() == 1);

// Nonexistent key (insert)
match table.entry(hasher(&"b"), |&x| x == "b", hasher) {
    Entry::Vacant(view) => {
        view.insert("b");
    }
    Entry::Occupied(_) => unreachable!(),
}
assert!(table.find(hasher(&"b"), |&x| x == "b").is_some() && table.len() == 2);
}
fn main() {
    #[cfg(feature = "nightly")]
    test()
}
```

#### Implementations

- <span id="vacantentry-insert"></span>`fn insert(self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- <span id="vacantentry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for VacantEntry<'_, T, A>`

- <span id="vacantentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `AbsentEntry<'a, T, A>`

```rust
struct AbsentEntry<'a, T, A>
where
    A: Allocator {
    table: &'a mut HashTable<T, A>,
}
```

Type representing the absence of an entry, as returned by `HashTable::find_entry`
and `HashTable::get_bucket_entry`.

This type only exists due to [limitations] in Rust's NLL borrow checker. In
the future, those methods will return an `Option<OccupiedEntry>` and this
type will be removed.

# Examples

```rust
#[cfg(feature = "nightly")]
fn test() {
use hashbrown::hash_table::{AbsentEntry, Entry};
use hashbrown::{HashTable, DefaultHashBuilder};
use std::hash::BuildHasher;

let mut table: HashTable<&str> = HashTable::new();
let hasher = DefaultHashBuilder::default();
let hasher = |val: &_| hasher.hash_one(val);

let entry_v: AbsentEntry<_, _> = table.find_entry(hasher(&"a"), |&x| x == "a").unwrap_err();
entry_v
    .into_table()
    .insert_unique(hasher(&"a"), "a", hasher);
assert!(table.find(hasher(&"a"), |&x| x == "a").is_some() && table.len() == 1);

// Nonexistent key (insert)
match table.entry(hasher(&"b"), |&x| x == "b", hasher) {
    Entry::Vacant(view) => {
        view.insert("b");
    }
    Entry::Occupied(_) => unreachable!(),
}
assert!(table.find(hasher(&"b"), |&x| x == "b").is_some() && table.len() == 2);
}
fn main() {
    #[cfg(feature = "nightly")]
    test()
}
```

#### Implementations

- <span id="absententry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for AbsentEntry<'_, T, A>`

- <span id="absententry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: crate::raw::RawIter<T>,
    marker: core::marker::PhantomData<&'a T>,
}
```

An iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `&'a T`.

This `struct` is created by the [`iter`](#iter) method on [`HashTable`](../index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<'a, T> Clone for Iter<'a, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a, T>` — [`Iter`](#iter)

##### `impl<T: fmt::Debug> Debug for Iter<'_, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Iter<'_, T>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

- <span id="iter-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl<I> IntoIterator for Iter<'a, T>`

- <span id="iter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for Iter<'a, T>`

- <span id="iter-item"></span>`type Item = &'a T`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: crate::raw::RawIter<T>,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

A mutable iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `&'a mut T`.

This `struct` is created by the `iter_mut` method on [`HashTable`](../index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Debug for IterMut<'_, T>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterMut<'_, T>`

- <span id="itermut-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterMut<'_, T>`

- <span id="itermut-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for IterMut<'_, T>`

##### `impl<I> IntoIterator for IterMut<'a, T>`

- <span id="itermut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiter"></span>`type IntoIter = I`

- <span id="itermut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for IterMut<'a, T>`

- <span id="itermut-item"></span>`type Item = &'a mut T`

- <span id="itermut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itermut-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `IterBuckets<'a, T>`

```rust
struct IterBuckets<'a, T> {
    inner: crate::raw::FullBucketsIndices,
    marker: core::marker::PhantomData<&'a T>,
}
```

An iterator producing the `usize` indices of all occupied buckets,
within the range `0..table.num_buckets()`.

The order in which the iterator yields indices is unspecified
and may change in the future.

This `struct` is created by the `HashTable::iter_buckets` method. See its
documentation for more.

#### Trait Implementations

##### `impl<T> Clone for IterBuckets<'_, T>`

- <span id="iterbuckets-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for IterBuckets<'_, T>`

- <span id="iterbuckets-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterBuckets<'_, T>`

- <span id="iterbuckets-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterBuckets<'_, T>`

- <span id="iterbuckets-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for IterBuckets<'_, T>`

##### `impl<I> IntoIterator for IterBuckets<'a, T>`

- <span id="iterbuckets-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterbuckets-intoiter"></span>`type IntoIter = I`

- <span id="iterbuckets-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterBuckets<'_, T>`

- <span id="iterbuckets-item"></span>`type Item = usize`

- <span id="iterbuckets-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="iterbuckets-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterHash<'a, T>`

```rust
struct IterHash<'a, T> {
    inner: crate::raw::RawIterHash<T>,
    marker: core::marker::PhantomData<&'a T>,
}
```

An iterator over the entries of a `HashTable` that could match a given hash.
The iterator element type is `&'a T`.

This `struct` is created by the `iter_hash` method on [`HashTable`](../index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<'a, T> Clone for IterHash<'a, T>`

- <span id="iterhash-clone"></span>`fn clone(&self) -> IterHash<'a, T>` — [`IterHash`](#iterhash)

##### `impl<T> Debug for IterHash<'_, T>`

- <span id="iterhash-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHash<'_, T>`

- <span id="iterhash-default"></span>`fn default() -> Self`

##### `impl<T> FusedIterator for IterHash<'_, T>`

##### `impl<I> IntoIterator for IterHash<'a, T>`

- <span id="iterhash-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhash-intoiter"></span>`type IntoIter = I`

- <span id="iterhash-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for IterHash<'a, T>`

- <span id="iterhash-item"></span>`type Item = &'a T`

- <span id="iterhash-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterhash-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `IterHashMut<'a, T>`

```rust
struct IterHashMut<'a, T> {
    inner: crate::raw::RawIterHash<T>,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

A mutable iterator over the entries of a `HashTable` that could match a given hash.
The iterator element type is `&'a mut T`.

This `struct` is created by the `iter_hash_mut` method on [`HashTable`](../index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Debug for IterHashMut<'_, T>`

- <span id="iterhashmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashMut<'_, T>`

- <span id="iterhashmut-default"></span>`fn default() -> Self`

##### `impl<T> FusedIterator for IterHashMut<'_, T>`

##### `impl<I> IntoIterator for IterHashMut<'a, T>`

- <span id="iterhashmut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhashmut-intoiter"></span>`type IntoIter = I`

- <span id="iterhashmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for IterHashMut<'a, T>`

- <span id="iterhashmut-item"></span>`type Item = &'a mut T`

- <span id="iterhashmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterhashmut-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `IterHashBuckets<'a, T>`

```rust
struct IterHashBuckets<'a, T> {
    inner: crate::raw::RawIterHashIndices,
    marker: core::marker::PhantomData<&'a T>,
}
```

An iterator producing the `usize` indices of all buckets which may match a hash.

This `struct` is created by the `HashTable::iter_hash_buckets` method. See its
documentation for more.

#### Trait Implementations

##### `impl<T> Clone for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-default"></span>`fn default() -> Self`

##### `impl<T> FusedIterator for IterHashBuckets<'_, T>`

##### `impl<I> IntoIterator for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhashbuckets-intoiter"></span>`type IntoIter = I`

- <span id="iterhashbuckets-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-item"></span>`type Item = usize`

- <span id="iterhashbuckets-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `IntoIter<T, A>`

```rust
struct IntoIter<T, A>
where
    A: Allocator {
    inner: crate::raw::RawIntoIter<T, A>,
}
```

An owning iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `T`.

This `struct` is created by the `into_iter` method on [`HashTable`](../index.md)
(provided by the `IntoIterator` trait). See its documentation for more.
The table cannot be used after calling that method.




#### Trait Implementations

##### `impl<T, A> Debug for IntoIter<T, A>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> Default for IntoIter<T, A>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<T, A> ExactSizeIterator for IntoIter<T, A>`

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

##### `impl<T, A> FusedIterator for IntoIter<T, A>`

##### `impl<I> IntoIterator for IntoIter<T, A>`

- <span id="intoiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A> Iterator for IntoIter<T, A>`

- <span id="intoiter-item"></span>`type Item = T`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Drain<'a, T, A: Allocator>`

```rust
struct Drain<'a, T, A: Allocator> {
    inner: crate::raw::RawDrain<'a, T, A>,
}
```

A draining iterator over the items of a `HashTable`.

This `struct` is created by the `drain` method on [`HashTable`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

- <span id="drain-len"></span>`fn len(&self) -> usize`

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<I> IntoIterator for Drain<'a, T, A>`

- <span id="drain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiter"></span>`type IntoIter = I`

- <span id="drain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-item"></span>`type Item = T`

- <span id="drain-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `ExtractIf<'a, T, F, A: Allocator>`

```rust
struct ExtractIf<'a, T, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, T, A>,
}
```

A draining iterator over entries of a `HashTable` which don't satisfy the predicate `f`.

This `struct` is created by `HashTable::extract_if`. See its
documentation for more.

#### Trait Implementations

##### `impl<T, F, A: Allocator> FusedIterator for ExtractIf<'_, T, F, A>`

##### `impl<I> IntoIterator for ExtractIf<'a, T, F, A>`

- <span id="extractif-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiter"></span>`type IntoIter = I`

- <span id="extractif-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, F, A: Allocator> Iterator for ExtractIf<'_, T, F, A>`

- <span id="extractif-item"></span>`type Item = T`

- <span id="extractif-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `Entry<'a, T, A>`

```rust
enum Entry<'a, T, A>
where
    A: Allocator {
    Occupied(OccupiedEntry<'a, T, A>),
    Vacant(VacantEntry<'a, T, A>),
}
```

A view into a single entry in a table, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashTable`](../index.md).


# Examples

```rust
#[cfg(feature = "nightly")]
fn test() {
use hashbrown::hash_table::{Entry, OccupiedEntry};
use hashbrown::{HashTable, DefaultHashBuilder};
use std::hash::BuildHasher;

let mut table = HashTable::new();
let hasher = DefaultHashBuilder::default();
let hasher = |val: &_| hasher.hash_one(val);
for x in ["a", "b", "c"] {
    table.insert_unique(hasher(&x), x, hasher);
}
assert_eq!(table.len(), 3);

// Existing value (insert)
let entry: Entry<_> = table.entry(hasher(&"a"), |&x| x == "a", hasher);
let _raw_o: OccupiedEntry<_, _> = entry.insert("a");
assert_eq!(table.len(), 3);
// Nonexistent value (insert)
table.entry(hasher(&"d"), |&x| x == "d", hasher).insert("d");

// Existing value (or_insert)
table
    .entry(hasher(&"b"), |&x| x == "b", hasher)
    .or_insert("b");
// Nonexistent value (or_insert)
table
    .entry(hasher(&"e"), |&x| x == "e", hasher)
    .or_insert("e");

println!("Our HashTable: {:?}", table);

let mut vec: Vec<_> = table.iter().copied().collect();
// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, ["a", "b", "c", "d", "e"]);
}
fn main() {
    #[cfg(feature = "nightly")]
    test()
}
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  #[cfg(feature = "nightly")]
  fn test() {
  use hashbrown::hash_table::{Entry, OccupiedEntry};
  use hashbrown::{HashTable, DefaultHashBuilder};
  use std::hash::BuildHasher;
  
  let mut table = HashTable::new();
  let hasher = DefaultHashBuilder::default();
  let hasher = |val: &_| hasher.hash_one(val);
  for x in ["a", "b"] {
      table.insert_unique(hasher(&x), x, hasher);
  }
  
  match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
      Entry::Vacant(_) => unreachable!(),
      Entry::Occupied(_) => {}
  }
  }
  fn main() {
      #[cfg(feature = "nightly")]
      test()
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  #[cfg(feature = "nightly")]
  fn test() {
  use hashbrown::hash_table::{Entry, OccupiedEntry};
  use hashbrown::{HashTable, DefaultHashBuilder};
  use std::hash::BuildHasher;
  
  let mut table = HashTable::<&str>::new();
  let hasher = DefaultHashBuilder::default();
  let hasher = |val: &_| hasher.hash_one(val);
  
  match table.entry(hasher(&"a"), |&x| x == "a", hasher) {
      Entry::Vacant(_) => {}
      Entry::Occupied(_) => unreachable!(),
  }
  }
  fn main() {
      #[cfg(feature = "nightly")]
      test()
  }
  ```

#### Implementations

- <span id="entry-insert"></span>`fn insert(self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- <span id="entry-or-insert"></span>`fn or_insert(self, default: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- <span id="entry-or-insert-with"></span>`fn or_insert_with(self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- <span id="entry-and-modify"></span>`fn and_modify(self, f: impl FnOnce(&mut T)) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for Entry<'_, T, A>`

- <span id="entry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

