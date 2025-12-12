*[hashbrown](../index.md) / [hash_set](index.md)*

---

# Module `hash_set`

A hash set implemented as a `HashMap` where the value is `()`.

## Contents

- [Structs](#structs)
  - [`HashSet`](#hashset)
  - [`Iter`](#iter)
  - [`IntoIter`](#intoiter)
  - [`Drain`](#drain)
  - [`ExtractIf`](#extractif)
  - [`Intersection`](#intersection)
  - [`Difference`](#difference)
  - [`SymmetricDifference`](#symmetricdifference)
  - [`Union`](#union)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
- [Enums](#enums)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`assert_covariance`](#assert-covariance)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashSet`](#hashset) | struct | A hash set implemented as a `HashMap` where the value is `()`. |
| [`Iter`](#iter) | struct | An iterator over the items of a `HashSet`. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the items of a `HashSet`. |
| [`Drain`](#drain) | struct | A draining iterator over the items of a `HashSet`. |
| [`ExtractIf`](#extractif) | struct | A draining iterator over entries of a `HashSet` which don't satisfy the predicate `f`. |
| [`Intersection`](#intersection) | struct | A lazy iterator producing elements in the intersection of `HashSet`s. |
| [`Difference`](#difference) | struct | A lazy iterator producing elements in the difference of `HashSet`s. |
| [`SymmetricDifference`](#symmetricdifference) | struct | A lazy iterator producing elements in the symmetric difference of `HashSet`s. |
| [`Union`](#union) | struct | A lazy iterator producing elements in the union of `HashSet`s. |
| [`OccupiedEntry`](#occupiedentry) | struct | A view into an occupied entry in a `HashSet`. |
| [`VacantEntry`](#vacantentry) | struct | A view into a vacant entry in a `HashSet`. |
| [`Entry`](#entry) | enum | A view into a single entry in a set, which may either be vacant or occupied. |
| [`assert_covariance`](#assert-covariance) | fn |  |

## Structs

### `HashSet<T, S, A: Allocator>`

```rust
struct HashSet<T, S, A: Allocator> {
    map: super::map::HashMap<T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:114-116`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L114-L116)*

A hash set implemented as a `HashMap` where the value is `()`.

As with the [`HashMap`](../hash_map/index.md) type, a `HashSet` requires that the elements
implement the `Eq` and `Hash` traits. This can frequently be achieved by
using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
it is important that the following property holds:

```text
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must be equal.


It is a logic error for an item to be modified in such a way that the
item's hash, as determined by the `Hash` trait, or its equality, as
determined by the `Eq` trait, changes while it is in the set. This is
normally only possible through [`Cell`](#cell), [`RefCell`](#refcell), global state, I/O, or
unsafe code.

It is also a logic error for the `Hash` implementation of a key to panic.
This is generally only possible if the trait is implemented manually. If a
panic does occur then the contents of the `HashSet` may become corrupted and
some items may be dropped from the table.

# Examples

```rust
use hashbrown::HashSet;
// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).
let mut books = HashSet::new();

// Add some books.
books.insert("A Dance With Dragons".to_string());
books.insert("To Kill a Mockingbird".to_string());
books.insert("The Odyssey".to_string());
books.insert("The Great Gatsby".to_string());

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!("We have {} books, but The Winds of Winter ain't one.",
             books.len());
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{}", book);
}
```

The easiest way to use `HashSet` with a custom type is to derive
`Eq` and `Hash`. We must also derive `PartialEq`. This will in the
future be implied by `Eq`.

```rust
use hashbrown::HashSet;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    power: usize,
}

let mut vikings = HashSet::new();

vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
vikings.insert(Viking { name: "Harald".to_string(), power: 8 });

// Use derived implementation to print the vikings.
for x in &vikings {
    println!("{:?}", x);
}
```

A `HashSet` with fixed list of elements can be initialized from an array:

```rust
use hashbrown::HashSet;

let viking_names: HashSet<&'static str> =
    [ "Einar", "Olaf", "Harald" ].into_iter().collect();
// use the values stored in the set
```







#### Implementations

- <span id="hashset-new"></span>`fn new() -> Self`

- <span id="hashset-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T, S, A> BitAnd for &HashSet<T, S, A>`

- <span id="hashset-bitand-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitand"></span>`fn bitand(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitAndAssign for HashSet<T, S, A>`

- <span id="hashset-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitOr for &HashSet<T, S, A>`

- <span id="hashset-bitor-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitor"></span>`fn bitor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitOrAssign for HashSet<T, S, A>`

- <span id="hashset-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitXor for &HashSet<T, S, A>`

- <span id="hashset-bitxor-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitxor"></span>`fn bitxor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitXorAssign for HashSet<T, S, A>`

- <span id="hashset-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

##### `impl<T: Clone, S: Clone, A: Allocator + Clone> Clone for HashSet<T, S, A>`

- <span id="hashset-clone"></span>`fn clone(&self) -> Self`

- <span id="hashset-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T, S, A> Debug for HashSet<T, S, A>`

- <span id="hashset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> Default for HashSet<T, S, A>`

- <span id="hashset-default"></span>`fn default() -> Self`

##### `impl<T, S, A> Eq for HashSet<T, S, A>`

##### `impl<K> Equivalent for HashSet<T, S, A>`

- <span id="hashset-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T, S, A> Extend for HashSet<T, S, A>`

- <span id="hashset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T, S, A> FromIterator for HashSet<T, S, A>`

- <span id="hashset-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T, S, A: Allocator> IntoIterator for &'a HashSet<T, S, A>`

- <span id="a-hashset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-hashset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-hashset-into-iter"></span>`fn into_iter(self) -> Iter<'a, T>` — [`Iter`](#iter)

##### `impl<T, S, A> PartialEq for HashSet<T, S, A>`

- <span id="hashset-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T, S, A> Sub for &HashSet<T, S, A>`

- <span id="hashset-sub-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-sub"></span>`fn sub(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](#hashset)

##### `impl<T, S, A> SubAssign for HashSet<T, S, A>`

- <span id="hashset-sub-assign"></span>`fn sub_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

### `Iter<'a, K>`

```rust
struct Iter<'a, K> {
    iter: super::map::Keys<'a, K, ()>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1647-1649`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1647-L1649)*

An iterator over the items of a `HashSet`.

This `struct` is created by the [`iter`](#iter) method on [`HashSet`](#hashset).
See its documentation for more.



#### Trait Implementations

##### `impl<K> Clone for Iter<'_, K>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<K: fmt::Debug> Debug for Iter<'_, K>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K> Default for Iter<'_, K>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<K> ExactSizeIterator for Iter<'_, K>`

- <span id="iter-len"></span>`fn len(&self) -> usize`

##### `impl<K> FusedIterator for Iter<'_, K>`

##### `impl IntoIterator for Iter<'a, K>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K> Iterator for Iter<'a, K>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a K`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<&'a K>`

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `IntoIter<K, A: Allocator>`

```rust
struct IntoIter<K, A: Allocator> {
    iter: map::IntoIter<K, (), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1658-1660`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1658-L1660)*

An owning iterator over the items of a `HashSet`.

This `struct` is created by the `into_iter` method on [`HashSet`](#hashset)
(provided by the `IntoIterator` trait). See its documentation for more.



#### Trait Implementations

##### `impl<K: fmt::Debug, A: Allocator> Debug for IntoIter<K, A>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> Default for IntoIter<K, A>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<K, A: Allocator> ExactSizeIterator for IntoIter<K, A>`

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

##### `impl<K, A: Allocator> FusedIterator for IntoIter<K, A>`

##### `impl IntoIterator for IntoIter<K, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, A: Allocator> Iterator for IntoIter<K, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = K`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<K>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Drain<'a, K, A: Allocator>`

```rust
struct Drain<'a, K, A: Allocator> {
    iter: map::Drain<'a, K, (), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1669-1671`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1669-L1671)*

A draining iterator over the items of a `HashSet`.

This `struct` is created by the `drain` method on [`HashSet`](#hashset).
See its documentation for more.



#### Trait Implementations

##### `impl<K: fmt::Debug, A: Allocator> Debug for Drain<'_, K, A>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> ExactSizeIterator for Drain<'_, K, A>`

- <span id="drain-len"></span>`fn len(&self) -> usize`

##### `impl<K, A: Allocator> FusedIterator for Drain<'_, K, A>`

##### `impl IntoIterator for Drain<'a, K, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, A: Allocator> Iterator for Drain<'_, K, A>`

- <span id="drain-iterator-type-item"></span>`type Item = K`

- <span id="drain-next"></span>`fn next(&mut self) -> Option<K>`

- <span id="drain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `ExtractIf<'a, K, F, A: Allocator>`

```rust
struct ExtractIf<'a, K, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, (K, ()), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1681-1684`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1681-L1684)*

A draining iterator over entries of a `HashSet` which don't satisfy the predicate `f`.

This `struct` is created by the `extract_if` method on [`HashSet`](#hashset). See its
documentation for more.



#### Trait Implementations

##### `impl<K, F, A: Allocator> FusedIterator for ExtractIf<'_, K, F, A>`

##### `impl IntoIterator for ExtractIf<'a, K, F, A>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, F, A: Allocator> Iterator for ExtractIf<'_, K, F, A>`

- <span id="extractif-iterator-type-item"></span>`type Item = K`

- <span id="extractif-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Intersection<'a, T, S, A: Allocator>`

```rust
struct Intersection<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1693-1698`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1693-L1698)*

A lazy iterator producing elements in the intersection of `HashSet`s.

This `struct` is created by the `intersection` method on [`HashSet`](#hashset).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Intersection<'_, T, S, A>`

- <span id="intersection-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S, A> Debug for Intersection<'_, T, S, A>`

- <span id="intersection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Intersection<'_, T, S, A>`

##### `impl IntoIterator for Intersection<'a, T, S, A>`

- <span id="intersection-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersection-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intersection-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Intersection<'a, T, S, A>`

- <span id="intersection-iterator-type-item"></span>`type Item = &'a T`

- <span id="intersection-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="intersection-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intersection-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Difference<'a, T, S, A: Allocator>`

```rust
struct Difference<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1707-1712`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1707-L1712)*

A lazy iterator producing elements in the difference of `HashSet`s.

This `struct` is created by the `difference` method on [`HashSet`](#hashset).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Difference<'_, T, S, A>`

- <span id="difference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S, A> Debug for Difference<'_, T, S, A>`

- <span id="difference-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Difference<'_, T, S, A>`

##### `impl IntoIterator for Difference<'a, T, S, A>`

- <span id="difference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="difference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="difference-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Difference<'a, T, S, A>`

- <span id="difference-iterator-type-item"></span>`type Item = &'a T`

- <span id="difference-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="difference-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="difference-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `SymmetricDifference<'a, T, S, A: Allocator>`

```rust
struct SymmetricDifference<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Difference<'a, T, S, A>, Difference<'a, T, S, A>>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1721-1723`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1721-L1723)*

A lazy iterator producing elements in the symmetric difference of `HashSet`s.

This `struct` is created by the `symmetric_difference` method on
[`HashSet`](#hashset). See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for SymmetricDifference<'_, T, S, A>`

- <span id="symmetricdifference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S, A> Debug for SymmetricDifference<'_, T, S, A>`

- <span id="symmetricdifference-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for SymmetricDifference<'_, T, S, A>`

##### `impl IntoIterator for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symmetricdifference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symmetricdifference-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-iterator-type-item"></span>`type Item = &'a T`

- <span id="symmetricdifference-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="symmetricdifference-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="symmetricdifference-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `Union<'a, T, S, A: Allocator>`

```rust
struct Union<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Iter<'a, T>, Difference<'a, T, S, A>>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1732-1734`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L1732-L1734)*

A lazy iterator producing elements in the union of `HashSet`s.

This `struct` is created by the `union` method on [`HashSet`](#hashset).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Union<'_, T, S, A>`

- <span id="union-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S, A> Debug for Union<'_, T, S, A>`

- <span id="union-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Union<'_, T, S, A>`

##### `impl IntoIterator for Union<'a, T, S, A>`

- <span id="union-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="union-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="union-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Union<'a, T, S, A>`

- <span id="union-iterator-type-item"></span>`type Item = &'a T`

- <span id="union-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="union-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="union-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `OccupiedEntry<'a, T, S, A: Allocator>`

```rust
struct OccupiedEntry<'a, T, S, A: Allocator> {
    inner: map::OccupiedEntry<'a, T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2301-2303`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L2301-L2303)*

A view into an occupied entry in a `HashSet`.
It is part of the [`Entry`](../hash_map/index.md) enum.

# Examples

```rust
use hashbrown::hash_set::{Entry, HashSet, OccupiedEntry};

let mut set = HashSet::new();
set.extend(["a", "b", "c"]);

let _entry_o: OccupiedEntry<_, _> = set.entry("a").insert();
assert_eq!(set.len(), 3);

// Existing key
match set.entry("a") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(view) => {
        assert_eq!(view.get(), &"a");
    }
}

assert_eq!(set.len(), 3);

// Existing key (take)
match set.entry("c") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(view) => {
        assert_eq!(view.remove(), "c");
    }
}
assert_eq!(set.get(&"c"), None);
assert_eq!(set.len(), 2);
```

#### Implementations

- <span id="occupiedentry-get"></span>`fn get(&self) -> &T`

- <span id="occupiedentry-remove"></span>`fn remove(self) -> T`

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for OccupiedEntry<'_, T, S, A>`

- <span id="occupiedentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VacantEntry<'a, T, S, A: Allocator>`

```rust
struct VacantEntry<'a, T, S, A: Allocator> {
    inner: map::VacantEntry<'a, T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2339-2341`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L2339-L2341)*

A view into a vacant entry in a `HashSet`.
It is part of the [`Entry`](../hash_map/index.md) enum.

# Examples

```rust
use hashbrown::hash_set::{Entry, HashSet, VacantEntry};

let mut set = HashSet::<&str>::new();

let entry_v: VacantEntry<_, _> = match set.entry("a") {
    Entry::Vacant(view) => view,
    Entry::Occupied(_) => unreachable!(),
};
entry_v.insert();
assert!(set.contains("a") && set.len() == 1);

// Nonexistent key (insert)
match set.entry("b") {
    Entry::Vacant(view) => { view.insert(); },
    Entry::Occupied(_) => unreachable!(),
}
assert!(set.contains("b") && set.len() == 2);
```

#### Implementations

- <span id="vacantentry-get"></span>`fn get(&self) -> &T`

- <span id="vacantentry-into-value"></span>`fn into_value(self) -> T`

- <span id="vacantentry-insert"></span>`fn insert(self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](#occupiedentry)

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for VacantEntry<'_, T, S, A>`

- <span id="vacantentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Entry<'a, T, S, A>`

```rust
enum Entry<'a, T, S, A>
where
    A: Allocator {
    Occupied(OccupiedEntry<'a, T, S, A>),
    Vacant(VacantEntry<'a, T, S, A>),
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2221-2254`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L2221-L2254)*

A view into a single entry in a set, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashSet`](#hashset).


# Examples

```rust
use hashbrown::hash_set::{Entry, HashSet, OccupiedEntry};

let mut set = HashSet::new();
set.extend(["a", "b", "c"]);
assert_eq!(set.len(), 3);

// Existing value (insert)
let entry: Entry<_, _> = set.entry("a");
let _raw_o: OccupiedEntry<_, _> = entry.insert();
assert_eq!(set.len(), 3);
// Nonexistent value (insert)
set.entry("d").insert();

// Existing value (or_insert)
set.entry("b").or_insert();
// Nonexistent value (or_insert)
set.entry("e").or_insert();

println!("Our HashSet: {:?}", set);

let mut vec: Vec<_> = set.iter().copied().collect();
// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, ["a", "b", "c", "d", "e"]);
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_set::{Entry, HashSet};
  let mut set: HashSet<_> = ["a", "b"].into();
  
  match set.entry("a") {
      Entry::Vacant(_) => unreachable!(),
      Entry::Occupied(_) => { }
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_set::{Entry, HashSet};
  let mut set: HashSet<&str> = HashSet::new();
  
  match set.entry("a") {
      Entry::Occupied(_) => unreachable!(),
      Entry::Vacant(_) => { }
  }
  ```

#### Implementations

- <span id="entry-insert"></span>`fn insert(self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](#occupiedentry)

- <span id="entry-or-insert"></span>`fn or_insert(self)`

- <span id="entry-get"></span>`fn get(&self) -> &T`

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for Entry<'_, T, S, A>`

- <span id="entry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `assert_covariance`

```rust
fn assert_covariance()
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2541-2574`](../../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L2541-L2574)*

