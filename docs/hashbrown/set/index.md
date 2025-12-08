*[hashbrown](../index.md) / [set](index.md)*

---

# Module `set`

## Structs

### `HashSet<T, S, A: Allocator>`

```rust
struct HashSet<T, S, A: Allocator> {
    map: super::map::HashMap<T, (), S, A>,
}
```

A hash set implemented as a `HashMap` where the value is `()`.

As with the [`HashMap`](../index.md) type, a `HashSet` requires that the elements
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

- `fn new() -> Self`

- `fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<T, S, A> BitAndAssign for HashSet<T, S, A>`

- `fn bitand_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../index.md)

##### `impl<T, S, A> BitOrAssign for HashSet<T, S, A>`

- `fn bitor_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../index.md)

##### `impl<T, S, A> BitXorAssign for HashSet<T, S, A>`

- `fn bitxor_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../index.md)

##### `impl<T: Clone, S: Clone, A: Allocator + Clone> Clone for HashSet<T, S, A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<T, S, A> Debug for HashSet<T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> Default for HashSet<T, S, A>`

- `fn default() -> Self`

##### `impl<T, S, A> Eq for HashSet<T, S, A>`

##### `impl<Q, K> Equivalent for HashSet<T, S, A>`

- `fn equivalent(self: &Self, key: &K) -> bool`

##### `impl<'a, T, S, A> Extend for HashSet<T, S, A>`

- `fn extend<I: IntoIterator<Item = &'a T>>(self: &mut Self, iter: I)`

##### `impl<T, S, A> FromIterator for HashSet<T, S, A>`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T, S, A: Allocator> IntoIterator for HashSet<T, S, A>`

- `type Item = T`

- `type IntoIter = IntoIter<T, A>`

- `fn into_iter(self: Self) -> IntoIter<T, A>` — [`IntoIter`](../hash_set/index.md)

##### `impl<T, S, A> PartialEq for HashSet<T, S, A>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T, S, A> SubAssign for HashSet<T, S, A>`

- `fn sub_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../index.md)

### `Iter<'a, K>`

```rust
struct Iter<'a, K> {
    iter: super::map::Keys<'a, K, ()>,
}
```

An iterator over the items of a `HashSet`.

This `struct` is created by the [`iter`](#iter) method on [`HashSet`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<K> Clone for Iter<'_, K>`

- `fn clone(self: &Self) -> Self`

##### `impl<K: fmt::Debug> Debug for Iter<'_, K>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K> Default for Iter<'_, K>`

- `fn default() -> Self`

##### `impl<K> ExactSizeIterator for Iter<'_, K>`

- `fn len(self: &Self) -> usize`

##### `impl<K> FusedIterator for Iter<'_, K>`

##### `impl<I> IntoIterator for Iter<'a, K>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K> Iterator for Iter<'a, K>`

- `type Item = &'a K`

- `fn next(self: &mut Self) -> Option<&'a K>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `IntoIter<K, A: Allocator>`

```rust
struct IntoIter<K, A: Allocator> {
    iter: map::IntoIter<K, (), A>,
}
```

An owning iterator over the items of a `HashSet`.

This `struct` is created by the `into_iter` method on [`HashSet`](../index.md)
(provided by the `IntoIterator` trait). See its documentation for more.



#### Trait Implementations

##### `impl<K: fmt::Debug, A: Allocator> Debug for IntoIter<K, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> Default for IntoIter<K, A>`

- `fn default() -> Self`

##### `impl<K, A: Allocator> ExactSizeIterator for IntoIter<K, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, A: Allocator> FusedIterator for IntoIter<K, A>`

##### `impl<I> IntoIterator for IntoIter<K, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, A: Allocator> Iterator for IntoIter<K, A>`

- `type Item = K`

- `fn next(self: &mut Self) -> Option<K>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Drain<'a, K, A: Allocator>`

```rust
struct Drain<'a, K, A: Allocator> {
    iter: map::Drain<'a, K, (), A>,
}
```

A draining iterator over the items of a `HashSet`.

This `struct` is created by the `drain` method on [`HashSet`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<K: fmt::Debug, A: Allocator> Debug for Drain<'_, K, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> ExactSizeIterator for Drain<'_, K, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, A: Allocator> FusedIterator for Drain<'_, K, A>`

##### `impl<I> IntoIterator for Drain<'a, K, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, A: Allocator> Iterator for Drain<'_, K, A>`

- `type Item = K`

- `fn next(self: &mut Self) -> Option<K>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `ExtractIf<'a, K, F, A: Allocator>`

```rust
struct ExtractIf<'a, K, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, (K, ()), A>,
}
```

A draining iterator over entries of a `HashSet` which don't satisfy the predicate `f`.

This `struct` is created by the `extract_if` method on [`HashSet`](../index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<K, F, A: Allocator> FusedIterator for ExtractIf<'_, K, F, A>`

##### `impl<I> IntoIterator for ExtractIf<'a, K, F, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, F, A: Allocator> Iterator for ExtractIf<'_, K, F, A>`

- `type Item = K`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Intersection<'a, T, S, A: Allocator>`

```rust
struct Intersection<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

A lazy iterator producing elements in the intersection of `HashSet`s.

This `struct` is created by the `intersection` method on [`HashSet`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Intersection<'_, T, S, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S, A> Debug for Intersection<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Intersection<'_, T, S, A>`

##### `impl<I> IntoIterator for Intersection<'a, T, S, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, S, A> Iterator for Intersection<'a, T, S, A>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<&'a T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Difference<'a, T, S, A: Allocator>`

```rust
struct Difference<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

A lazy iterator producing elements in the difference of `HashSet`s.

This `struct` is created by the `difference` method on [`HashSet`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Difference<'_, T, S, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S, A> Debug for Difference<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Difference<'_, T, S, A>`

##### `impl<I> IntoIterator for Difference<'a, T, S, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, S, A> Iterator for Difference<'a, T, S, A>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<&'a T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `SymmetricDifference<'a, T, S, A: Allocator>`

```rust
struct SymmetricDifference<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Difference<'a, T, S, A>, Difference<'a, T, S, A>>,
}
```

A lazy iterator producing elements in the symmetric difference of `HashSet`s.

This `struct` is created by the `symmetric_difference` method on
[`HashSet`](../index.md). See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for SymmetricDifference<'_, T, S, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S, A> Debug for SymmetricDifference<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for SymmetricDifference<'_, T, S, A>`

##### `impl<I> IntoIterator for SymmetricDifference<'a, T, S, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, S, A> Iterator for SymmetricDifference<'a, T, S, A>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<&'a T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Union<'a, T, S, A: Allocator>`

```rust
struct Union<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Iter<'a, T>, Difference<'a, T, S, A>>,
}
```

A lazy iterator producing elements in the union of `HashSet`s.

This `struct` is created by the `union` method on [`HashSet`](../index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T, S, A: Allocator> Clone for Union<'_, T, S, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T, S, A> Debug for Union<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> FusedIterator for Union<'_, T, S, A>`

##### `impl<I> IntoIterator for Union<'a, T, S, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T, S, A> Iterator for Union<'a, T, S, A>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<&'a T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `OccupiedEntry<'a, T, S, A: Allocator>`

```rust
struct OccupiedEntry<'a, T, S, A: Allocator> {
    inner: map::OccupiedEntry<'a, T, (), S, A>,
}
```

A view into an occupied entry in a `HashSet`.
It is part of the [`Entry`](../hash_table/index.md) enum.

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

- `fn get(self: &Self) -> &T`

- `fn remove(self: Self) -> T`

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for OccupiedEntry<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VacantEntry<'a, T, S, A: Allocator>`

```rust
struct VacantEntry<'a, T, S, A: Allocator> {
    inner: map::VacantEntry<'a, T, (), S, A>,
}
```

A view into a vacant entry in a `HashSet`.
It is part of the [`Entry`](../hash_table/index.md) enum.

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

- `fn get(self: &Self) -> &T`

- `fn into_value(self: Self) -> T`

- `fn insert(self: Self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](../hash_set/index.md)

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for VacantEntry<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

A view into a single entry in a set, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashSet`](../index.md).


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

- `fn insert(self: Self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](../hash_set/index.md)

- `fn or_insert(self: Self)`

- `fn get(self: &Self) -> &T`

#### Trait Implementations

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for Entry<'_, T, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `assert_covariance`

```rust
fn assert_covariance()
```

