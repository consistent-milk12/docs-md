*[hashbrown](../index.md) / [hash_table](index.md)*

---

# Module `hash_table`

A hash table implemented with quadratic probing and SIMD lookup.

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

- `const fn new_in(alloc: A) -> Self`

- `fn with_capacity_in(capacity: usize, alloc: A) -> Self`

- `fn allocator(self: &Self) -> &A`

- `fn find(self: &Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&T>`

- `fn find_mut(self: &mut Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<&mut T>`

- `fn find_entry(self: &mut Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>>` — [`OccupiedEntry`](#occupiedentry), [`AbsentEntry`](#absententry)

- `fn find_bucket_index(self: &Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<usize>`

- `fn entry(self: &mut Self, hash: u64, eq: impl FnMut(&T) -> bool, hasher: impl Fn(&T) -> u64) -> Entry<'_, T, A>` — [`Entry`](#entry)

- `fn get_bucket_entry(self: &mut Self, index: usize) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>>` — [`OccupiedEntry`](#occupiedentry), [`AbsentEntry`](#absententry)

- `unsafe fn get_bucket_entry_unchecked(self: &mut Self, index: usize) -> OccupiedEntry<'_, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn get_bucket(self: &Self, index: usize) -> Option<&T>`

- `unsafe fn get_bucket_unchecked(self: &Self, index: usize) -> &T`

- `fn get_bucket_mut(self: &mut Self, index: usize) -> Option<&mut T>`

- `unsafe fn get_bucket_unchecked_mut(self: &mut Self, index: usize) -> &mut T`

- `fn insert_unique(self: &mut Self, hash: u64, value: T, hasher: impl Fn(&T) -> u64) -> OccupiedEntry<'_, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn clear(self: &mut Self)`

- `fn shrink_to_fit(self: &mut Self, hasher: impl Fn(&T) -> u64)`

- `fn shrink_to(self: &mut Self, min_capacity: usize, hasher: impl Fn(&T) -> u64)`

- `fn reserve(self: &mut Self, additional: usize, hasher: impl Fn(&T) -> u64)`

- `fn try_reserve(self: &mut Self, additional: usize, hasher: impl Fn(&T) -> u64) -> Result<(), TryReserveError>` — [`TryReserveError`](../index.md)

- `fn num_buckets(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_, T>` — [`Iter`](#iter)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>` — [`IterMut`](#itermut)

- `fn iter_buckets(self: &Self) -> IterBuckets<'_, T>` — [`IterBuckets`](#iterbuckets)

- `fn iter_hash(self: &Self, hash: u64) -> IterHash<'_, T>` — [`IterHash`](#iterhash)

- `fn iter_hash_mut(self: &mut Self, hash: u64) -> IterHashMut<'_, T>` — [`IterHashMut`](#iterhashmut)

- `fn iter_hash_buckets(self: &Self, hash: u64) -> IterHashBuckets<'_, T>` — [`IterHashBuckets`](#iterhashbuckets)

- `fn retain(self: &mut Self, f: impl FnMut(&mut T) -> bool)`

- `fn drain(self: &mut Self) -> Drain<'_, T, A>` — [`Drain`](#drain)

- `fn extract_if<F>(self: &mut Self, f: F) -> ExtractIf<'_, T, F, A>` — [`ExtractIf`](#extractif)

- `fn get_disjoint_mut<const N: usize>(self: &mut Self, hashes: [u64; N], eq: impl FnMut(usize, &T) -> bool) -> [Option<&mut T>; N]`

- `fn get_many_mut<const N: usize>(self: &mut Self, hashes: [u64; N], eq: impl FnMut(usize, &T) -> bool) -> [Option<&mut T>; N]`

- `unsafe fn get_disjoint_unchecked_mut<const N: usize>(self: &mut Self, hashes: [u64; N], eq: impl FnMut(usize, &T) -> bool) -> [Option<&mut T>; N]`

- `unsafe fn get_many_unchecked_mut<const N: usize>(self: &mut Self, hashes: [u64; N], eq: impl FnMut(usize, &T) -> bool) -> [Option<&mut T>; N]`

- `fn allocation_size(self: &Self) -> usize`

#### Trait Implementations

##### `impl<T, A> Clone for HashTable<T, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, A> Debug for HashTable<T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A> Default for HashTable<T, A>`

- `fn default() -> Self`

##### `impl<T, A> IntoIterator for HashTable<T, A>`

- `type Item = T`

- `type IntoIter = IntoIter<T, A>`

- `fn into_iter(self: Self) -> IntoIter<T, A>` — [`IntoIter`](#intoiter)

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
It is part of the [`Entry`](#entry) enum.

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

- `fn remove(self: Self) -> (T, VacantEntry<'a, T, A>)` — [`VacantEntry`](#vacantentry)

- `fn get(self: &Self) -> &T`

- `fn get_mut(self: &mut Self) -> &mut T`

- `fn into_mut(self: Self) -> &'a mut T`

- `fn into_table(self: Self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

- `fn bucket_index(self: &Self) -> usize`

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for OccupiedEntry<'_, T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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
It is part of the [`Entry`](#entry) enum.

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

- `fn insert(self: Self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn into_table(self: Self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for VacantEntry<'_, T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn into_table(self: Self) -> &'a mut HashTable<T, A>` — [`HashTable`](../index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for AbsentEntry<'_, T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn clone(self: &Self) -> Iter<'a, T>` — [`Iter`](#iter)

##### `impl<T: fmt::Debug> Debug for Iter<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Iter<'_, T>`

- `fn default() -> Self`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

- `fn len(self: &Self) -> usize`

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl<I> IntoIterator for Iter<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for Iter<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterMut<'_, T>`

- `fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterMut<'_, T>`

- `fn len(self: &Self) -> usize`

##### `impl<T> FusedIterator for IterMut<'_, T>`

##### `impl<I> IntoIterator for IterMut<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for IterMut<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `fn clone(self: &Self) -> Self`

##### `impl<T> Debug for IterBuckets<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterBuckets<'_, T>`

- `fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterBuckets<'_, T>`

- `fn len(self: &Self) -> usize`

##### `impl<T> FusedIterator for IterBuckets<'_, T>`

##### `impl<I> IntoIterator for IterBuckets<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for IterBuckets<'_, T>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

- `fn clone(self: &Self) -> IterHash<'a, T>` — [`IterHash`](#iterhash)

##### `impl<T> Debug for IterHash<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHash<'_, T>`

- `fn default() -> Self`

##### `impl<T> FusedIterator for IterHash<'_, T>`

##### `impl<I> IntoIterator for IterHash<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for IterHash<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashMut<'_, T>`

- `fn default() -> Self`

##### `impl<T> FusedIterator for IterHashMut<'_, T>`

##### `impl<I> IntoIterator for IterHashMut<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for IterHashMut<'a, T>`

- `type Item = &'a mut T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `fn clone(self: &Self) -> Self`

##### `impl<T> Debug for IterHashBuckets<'_, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashBuckets<'_, T>`

- `fn default() -> Self`

##### `impl<T> FusedIterator for IterHashBuckets<'_, T>`

##### `impl<I> IntoIterator for IterHashBuckets<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for IterHashBuckets<'_, T>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> Default for IntoIter<T, A>`

- `fn default() -> Self`

##### `impl<T, A> ExactSizeIterator for IntoIter<T, A>`

- `fn len(self: &Self) -> usize`

##### `impl<T, A> FusedIterator for IntoIter<T, A>`

##### `impl<I> IntoIterator for IntoIter<T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A> Iterator for IntoIter<T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

- `fn len(self: &Self) -> usize`

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<I> IntoIterator for Drain<'a, T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

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

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, F, A: Allocator> Iterator for ExtractIf<'_, T, F, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

- `fn insert(self: Self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn or_insert(self: Self, default: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn or_insert_with(self: Self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn and_modify(self: Self, f: impl FnOnce(&mut T)) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, A: Allocator> Debug for Entry<'_, T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

