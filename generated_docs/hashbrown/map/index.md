*[hashbrown](../index.md) / [map](index.md)*

---

# Module `map`

## Contents

- [Structs](#structs)
  - [`HashMap`](#hashmap)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`IntoIter`](#intoiter)
  - [`IntoKeys`](#intokeys)
  - [`IntoValues`](#intovalues)
  - [`Keys`](#keys)
  - [`Values`](#values)
  - [`Drain`](#drain)
  - [`ExtractIf`](#extractif)
  - [`ValuesMut`](#valuesmut)
  - [`OccupiedEntry`](#occupiedentry)
  - [`VacantEntry`](#vacantentry)
  - [`VacantEntryRef`](#vacantentryref)
  - [`OccupiedError`](#occupiederror)
  - [`RawEntryBuilderMut`](#rawentrybuildermut)
  - [`RawOccupiedEntryMut`](#rawoccupiedentrymut)
  - [`RawVacantEntryMut`](#rawvacantentrymut)
  - [`RawEntryBuilder`](#rawentrybuilder)
- [Enums](#enums)
  - [`Entry`](#entry)
  - [`EntryRef`](#entryref)
  - [`RawEntryMut`](#rawentrymut)
- [Functions](#functions)
  - [`make_hasher`](#make-hasher)
  - [`equivalent_key`](#equivalent-key)
  - [`equivalent`](#equivalent)
  - [`make_hash`](#make-hash)
  - [`assert_covariance`](#assert-covariance)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HashMap`](#hashmap) | struct | A hash map implemented with quadratic probing and SIMD lookup. |
| [`Iter`](#iter) | struct | An iterator over the entries of a `HashMap` in arbitrary order. |
| [`IterMut`](#itermut) | struct | A mutable iterator over the entries of a `HashMap` in arbitrary order. |
| [`IntoIter`](#intoiter) | struct | An owning iterator over the entries of a `HashMap` in arbitrary order. |
| [`IntoKeys`](#intokeys) | struct | An owning iterator over the keys of a `HashMap` in arbitrary order. |
| [`IntoValues`](#intovalues) | struct | An owning iterator over the values of a `HashMap` in arbitrary order. |
| [`Keys`](#keys) | struct | An iterator over the keys of a `HashMap` in arbitrary order. |
| [`Values`](#values) | struct | An iterator over the values of a `HashMap` in arbitrary order. |
| [`Drain`](#drain) | struct | A draining iterator over the entries of a `HashMap` in arbitrary order. |
| [`ExtractIf`](#extractif) | struct | A draining iterator over entries of a `HashMap` which don't satisfy the predicate `f(&k, &mut v)` in arbitrary order. |
| [`ValuesMut`](#valuesmut) | struct | A mutable iterator over the values of a `HashMap` in arbitrary order. |
| [`OccupiedEntry`](#occupiedentry) | struct | A view into an occupied entry in a [`HashMap`]. |
| [`VacantEntry`](#vacantentry) | struct | A view into a vacant entry in a `HashMap`. |
| [`VacantEntryRef`](#vacantentryref) | struct | A view into a vacant entry in a `HashMap`. |
| [`OccupiedError`](#occupiederror) | struct | The error returned by [`try_insert`](HashMap::try_insert) when the key already exists. |
| [`RawEntryBuilderMut`](#rawentrybuildermut) | struct | A builder for computing where in a [`HashMap`] a key-value pair would be stored. |
| [`RawOccupiedEntryMut`](#rawoccupiedentrymut) | struct | A view into an occupied entry in a `HashMap`. |
| [`RawVacantEntryMut`](#rawvacantentrymut) | struct | A view into a vacant entry in a `HashMap`. |
| [`RawEntryBuilder`](#rawentrybuilder) | struct | A builder for computing where in a [`HashMap`] a key-value pair would be stored. |
| [`Entry`](#entry) | enum | A view into a single entry in a map, which may either be vacant or occupied. |
| [`EntryRef`](#entryref) | enum | A view into a single entry in a map, which may either be vacant or occupied, with any borrowed form of the map's key type. |
| [`RawEntryMut`](#rawentrymut) | enum | A view into a single entry in a map, which may either be vacant or occupied. |
| [`make_hasher`](#make-hasher) | fn | Ensures that a single closure type across uses of this which, in turn prevents multiple instances of any functions like `RawTable::reserve` from being generated |
| [`equivalent_key`](#equivalent-key) | fn | Ensures that a single closure type across uses of this which, in turn prevents multiple instances of any functions like `RawTable::reserve` from being generated |
| [`equivalent`](#equivalent) | fn | Ensures that a single closure type across uses of this which, in turn prevents multiple instances of any functions like `RawTable::reserve` from being generated |
| [`make_hash`](#make-hash) | fn |  |
| [`assert_covariance`](#assert-covariance) | fn |  |

## Structs

### `HashMap<K, V, S, A: Allocator>`

```rust
struct HashMap<K, V, S, A: Allocator> {
    hash_builder: S,
    table: crate::raw::RawTable<(K, V), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:185-188`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L185-L188)*

A hash map implemented with quadratic probing and SIMD lookup.

The default hashing algorithm is currently `foldhash`, though this is
subject to change at any point in the future. This hash function is very
fast for all types of keys, but this algorithm will typically *not* protect
against attacks such as HashDoS.

The hashing algorithm can be replaced on a per-`HashMap` basis using the
`default`, `with_hasher`, and `with_capacity_and_hasher` methods. Many
alternative algorithms are available on crates.io, such as the `fnv` crate.

It is required that the keys implement the `Eq` and `Hash` traits, although
this can frequently be achieved by using `#[derive(PartialEq, Eq, Hash)]`.
If you implement these yourself, it is important that the following
property holds:

```text
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must be equal.

It is a logic error for a key to be modified in such a way that the key's
hash, as determined by the `Hash` trait, or its equality, as determined by
the `Eq` trait, changes while it is in the map. This is normally only
possible through [`Cell`](#cell), [`RefCell`](#refcell), global state, I/O, or unsafe code.

It is also a logic error for the `Hash` implementation of a key to panic.
This is generally only possible if the trait is implemented manually. If a
panic does occur then the contents of the `HashMap` may become corrupted and
some items may be dropped from the table.

# Examples

```rust
use hashbrown::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
let mut book_reviews = HashMap::new();

// Review some books.
book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
book_reviews.insert(
    "Grimms' Fairy Tales".to_string(),
    "Masterpiece.".to_string(),
);
book_reviews.insert(
    "Pride and Prejudice".to_string(),
    "Very enjoyable.".to_string(),
);
book_reviews.insert(
    "The Adventures of Sherlock Holmes".to_string(),
    "Eye lyked it alot.".to_string(),
);

// Check for a specific one.
// When collections store owned values (String), they can still be
// queried using references (&str).
if !book_reviews.contains_key("Les Misérables") {
    println!("We've got {} reviews, but Les Misérables ain't one.",
             book_reviews.len());
}

// oops, this review has a lot of spelling mistakes, let's delete it.
book_reviews.remove("The Adventures of Sherlock Holmes");

// Look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for &book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{}: {}", book, review),
        None => println!("{} is unreviewed.", book)
    }
}

// Look up the value for a key (will panic if the key is not found).
println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

// Iterate over everything.
for (book, review) in &book_reviews {
    println!("{}: \"{}\"", book, review);
}
```

`HashMap` also implements an [`Entry API`](#method.entry), which allows
for more complex methods of getting, setting, updating and removing keys and
their values:

```rust
use hashbrown::HashMap;

// type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, u8>` in this example).
let mut player_stats = HashMap::new();

fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
}

// insert a key only if it doesn't already exist
player_stats.entry("health").or_insert(100);

// insert a key using a function that provides a new value only if it
// doesn't already exist
player_stats.entry("defence").or_insert_with(random_stat_buff);

// update a key, guarding against the key possibly not being set
let stat = player_stats.entry("attack").or_insert(100);
*stat += random_stat_buff();
```

The easiest way to use `HashMap` with a custom key type is to derive `Eq` and `Hash`.
We must also derive `PartialEq`.










```rust
use hashbrown::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

// Use a HashMap to store the vikings' health points.
let mut vikings = HashMap::new();

vikings.insert(Viking::new("Einar", "Norway"), 25);
vikings.insert(Viking::new("Olaf", "Denmark"), 24);
vikings.insert(Viking::new("Harald", "Iceland"), 12);

// Use derived implementation to print the status of the vikings.
for (viking, health) in &vikings {
    println!("{:?} has {} hp", viking, health);
}
```

A `HashMap` with fixed list of elements can be initialized from an array:

```rust
use hashbrown::HashMap;

let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    .into_iter().collect();
// use the values stored in map
```

#### Implementations

- <span id="hashmap-new"></span>`fn new() -> Self`

  Creates an empty `HashMap`.

  

  The hash map is initially created with a capacity of 0, so it will not allocate until it

  is first inserted into.

  

  # HashDoS resistance

  

  The `hash_builder` normally use a fixed key by default and that does

  not allow the `HashMap` to be protected against attacks such as `HashDoS`.

  Users who require HashDoS resistance should explicitly use

  `std::collections::hash_map::RandomState`

  as the hasher when creating a [`HashMap`](../hash_map/index.md), for example with

  [`with_hasher`](HashMap::with_hasher) method.

  

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  let mut map: HashMap<&str, i32> = HashMap::new();

  assert_eq!(map.len(), 0);

  assert_eq!(map.capacity(), 0);

  ```

- <span id="hashmap-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Creates an empty `HashMap` with the specified capacity.

  

  The hash map will be able to hold at least `capacity` elements without

  reallocating. If `capacity` is 0, the hash map will not allocate.

  

  # HashDoS resistance

  

  The `hash_builder` normally use a fixed key by default and that does

  not allow the `HashMap` to be protected against attacks such as `HashDoS`.

  Users who require HashDoS resistance should explicitly use

  `std::collections::hash_map::RandomState`

  as the hasher when creating a [`HashMap`](../hash_map/index.md), for example with

  [`with_capacity_and_hasher`](HashMap::with_capacity_and_hasher) method.

  

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);

  assert_eq!(map.len(), 0);

  assert!(map.capacity() >= 10);

  ```

#### Trait Implementations

##### `impl Any for HashMap<K, V, S, A>`

- <span id="hashmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HashMap<K, V, S, A>`

- <span id="hashmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HashMap<K, V, S, A>`

- <span id="hashmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Clone, V: Clone, S: Clone, A: Allocator + Clone> Clone for HashMap<K, V, S, A>`

- <span id="hashmap-clone"></span>`fn clone(&self) -> Self`

- <span id="hashmap-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl CloneToUninit for HashMap<K, V, S, A>`

- <span id="hashmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K, V, S, A> Debug for HashMap<K, V, S, A>`

- <span id="hashmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S, A> Default for HashMap<K, V, S, A>`

- <span id="hashmap-default"></span>`fn default() -> Self`

  Creates an empty `HashMap<K, V, S, A>`, with the `Default` value for the hasher and allocator.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use std::collections::hash_map::RandomState;

  

  // You can specify all types of HashMap, including hasher and allocator.

  // Created map is empty and don't allocate memory

  let map: HashMap<u32, String> = Default::default();

  assert_eq!(map.capacity(), 0);

  let map: HashMap<u32, String, RandomState> = HashMap::default();

  assert_eq!(map.capacity(), 0);

  ```

##### `impl<K, V, S, A> Eq for HashMap<K, V, S, A>`

##### `impl<K> Equivalent for HashMap<K, V, S, A>`

- <span id="hashmap-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K, V, S, A> Extend for HashMap<K, V, S, A>`

- <span id="hashmap-extend"></span>`fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T)`

  Inserts all new key-values from the iterator to existing `HashMap<K, V, S, A>`.

  Replace values with existing keys with new values returned from the iterator.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::HashMap;

  

  let mut map = HashMap::new();

  map.insert(1, 100);

  

  let some_iter = [(1, 1), (2, 2)].into_iter();

  map.extend(some_iter);

  // Replace values with existing keys with new values returned from the iterator.

  // So that the map.get(&1) doesn't return Some(&100).

  assert_eq!(map.get(&1), Some(&1));

  

  let some_vec: Vec<_> = vec![(3, 3), (4, 4)];

  map.extend(some_vec);

  

  let some_arr = [(5, 5), (6, 6)];

  map.extend(some_arr);

  let old_map_len = map.len();

  

  // You can also extend from another HashMap

  let mut new_map = HashMap::new();

  new_map.extend(map);

  assert_eq!(new_map.len(), old_map_len);

  

  let mut vec: Vec<_> = new_map.into_iter().collect();

  // The `IntoIter` iterator produces items in arbitrary order, so the

  // items must be sorted to test them against a sorted array.

  vec.sort_unstable();

  assert_eq!(vec, [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]);

  ```

##### `impl<T> From for HashMap<K, V, S, A>`

- <span id="hashmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, S, A> FromIterator for HashMap<K, V, S, A>`

- <span id="hashmap-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self`

##### `impl<K, Q, V, S, A> Index for HashMap<K, V, S, A>`

- <span id="hashmap-index-type-output"></span>`type Output = V`

- <span id="hashmap-index"></span>`fn index(&self, key: &Q) -> &V`

  Returns a reference to the value corresponding to the supplied key.

  

  # Panics

  

  Panics if the key is not present in the `HashMap`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let map: HashMap<_, _> = [("a", "One"), ("b", "Two")].into();

  

  assert_eq!(map[&"a"], "One");

  assert_eq!(map[&"b"], "Two");

  ```

##### `impl<U> Into for HashMap<K, V, S, A>`

- <span id="hashmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<K, V, S, A: Allocator> IntoIterator for &'a HashMap<K, V, S, A>`

- <span id="a-hashmap-intoiterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-hashmap-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-hashmap-intoiterator-into-iter"></span>`fn into_iter(self) -> Iter<'a, K, V>` — [`Iter`](../hash_map/index.md#iter)

  Creates an iterator over the entries of a `HashMap` in arbitrary order.

  The iterator element type is `(&'a K, &'a V)`.

  

  Return the same `Iter` struct as by the [`iter`](#iter) method on [`HashMap`](../hash_map/index.md).

  

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  let map_one: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

  let mut map_two = HashMap::new();

  

  for (key, value) in &map_one {

      println!("Key: {}, Value: {}", key, value);

      map_two.insert(*key, *value);

  }

  

  assert_eq!(map_one, map_two);

  ```

##### `impl<K, V, S, A> PartialEq for HashMap<K, V, S, A>`

- <span id="hashmap-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for HashMap<K, V, S, A>`

- <span id="hashmap-toowned-type-owned"></span>`type Owned = T`

- <span id="hashmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hashmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HashMap<K, V, S, A>`

- <span id="hashmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hashmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HashMap<K, V, S, A>`

- <span id="hashmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hashmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    inner: crate::raw::RawIter<(K, V)>,
    marker: core::marker::PhantomData<(&'a K, &'a V)>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2206-2209`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2206-L2209)*

An iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(&'a K, &'a V)`.

This `struct` is created by the [`iter`](#iter) method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut iter = map.iter();
let mut vec = vec![iter.next(), iter.next(), iter.next()];

// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some((&1, &"a")), Some((&2, &"b")), Some((&3, &"c"))]);

// It is fused iterator
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
```

#### Trait Implementations

##### `impl Any for Iter<'a, K, V>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, K, V>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, K, V>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Iter<'a, K, V>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K: Debug, V: Debug> Debug for Iter<'_, K, V>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Iter<'_, K, V>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Iter<'a, K, V>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V> FusedIterator for Iter<'_, K, V>`

##### `impl<U> Into for Iter<'a, K, V>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a V)>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl ToOwned for Iter<'a, K, V>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iter<'a, K, V>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a, K, V>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    inner: crate::raw::RawIter<(K, V)>,
    marker: core::marker::PhantomData<(&'a K, &'a mut V)>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2255-2259`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2255-L2259)*

A mutable iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(&'a K, &'a mut V)`.

This `struct` is created by the `iter_mut` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<_, _> = [(1, "One".to_owned()), (2, "Two".into())].into();

let mut iter = map.iter_mut();
iter.next().map(|(_, v)| v.push_str(" Mississippi"));
iter.next().map(|(_, v)| v.push_str(" Mississippi"));

// It is fused iterator
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);

assert_eq!(map.get(&1).unwrap(), &"One Mississippi".to_owned());
assert_eq!(map.get(&2).unwrap(), &"Two Mississippi".to_owned());
```

#### Implementations

- <span id="itermut-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](../hash_map/index.md#iter)

  Returns a iterator of references over the remaining items.

#### Trait Implementations

##### `impl Any for IterMut<'a, K, V>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a, K, V>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a, K, V>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> Debug for IterMut<'_, K, V>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IterMut<'_, K, V>`

- <span id="itermut-default"></span>`fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IterMut<'a, K, V>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V> FusedIterator for IterMut<'_, K, V>`

##### `impl<U> Into for IterMut<'a, K, V>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a mut V)>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itermut-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<K: Send, V: Send> Send for IterMut<'_, K, V>`

##### `impl<U> TryFrom for IterMut<'a, K, V>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterMut<'a, K, V>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter<K, V, A: Allocator>`

```rust
struct IntoIter<K, V, A: Allocator> {
    inner: crate::raw::RawIntoIter<(K, V), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2307-2309`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2307-L2309)*

An owning iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(K, V)`.

This `struct` is created by the `into_iter` method on [`HashMap`](../hash_map/index.md)
(provided by the `IntoIterator` trait). See its documentation for more.
The map cannot be used after calling that method.



# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut iter = map.into_iter();
let mut vec = vec![iter.next(), iter.next(), iter.next()];

// The `IntoIter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some((1, "a")), Some((2, "b")), Some((3, "c"))]);

// It is fused iterator
assert_eq!(iter.next(), None);
assert_eq!(iter.next(), None);
```

#### Implementations

- <span id="intoiter-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](../hash_map/index.md#iter)

  Returns a iterator of references over the remaining items.

#### Trait Implementations

##### `impl Any for IntoIter<K, V, A>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<K, V, A>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<K, V, A>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, A: Allocator> Debug for IntoIter<K, V, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoIter<K, V, A>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoIter<K, V, A>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A>`

##### `impl<U> Into for IntoIter<K, V, A>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter<K, V, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = (K, V)`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<(K, V)>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for IntoIter<K, V, A>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter<K, V, A>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoKeys<K, V, A: Allocator>`

```rust
struct IntoKeys<K, V, A: Allocator> {
    inner: IntoIter<K, V, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2351-2353`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2351-L2353)*

An owning iterator over the keys of a `HashMap` in arbitrary order.
The iterator element type is `K`.

This `struct` is created by the `into_keys` method on [`HashMap`](../hash_map/index.md).
See its documentation for more.
The map cannot be used after calling that method.


# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut keys = map.into_keys();
let mut vec = vec![keys.next(), keys.next(), keys.next()];

// The `IntoKeys` iterator produces keys in arbitrary order, so the
// keys must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some(1), Some(2), Some(3)]);

// It is fused iterator
assert_eq!(keys.next(), None);
assert_eq!(keys.next(), None);
```

#### Trait Implementations

##### `impl Any for IntoKeys<K, V, A>`

- <span id="intokeys-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoKeys<K, V, A>`

- <span id="intokeys-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoKeys<K, V, A>`

- <span id="intokeys-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, A: Allocator> Debug for IntoKeys<K, V, A>`

- <span id="intokeys-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoKeys<K, V, A>`

- <span id="intokeys-default"></span>`fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoKeys<K, V, A>`

- <span id="intokeys-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoKeys<K, V, A>`

- <span id="intokeys-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, A: Allocator> FusedIterator for IntoKeys<K, V, A>`

##### `impl<U> Into for IntoKeys<K, V, A>`

- <span id="intokeys-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoKeys<K, V, A>`

- <span id="intokeys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intokeys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intokeys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A>`

- <span id="intokeys-iterator-type-item"></span>`type Item = K`

- <span id="intokeys-iterator-next"></span>`fn next(&mut self) -> Option<K>`

- <span id="intokeys-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intokeys-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for IntoKeys<K, V, A>`

- <span id="intokeys-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intokeys-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoKeys<K, V, A>`

- <span id="intokeys-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intokeys-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoValues<K, V, A: Allocator>`

```rust
struct IntoValues<K, V, A: Allocator> {
    inner: IntoIter<K, V, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2429-2431`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2429-L2431)*

An owning iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `V`.

This `struct` is created by the `into_values` method on [`HashMap`](../hash_map/index.md).
See its documentation for more. The map cannot be used after calling that method.


# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut values = map.into_values();
let mut vec = vec![values.next(), values.next(), values.next()];

// The `IntoValues` iterator produces values in arbitrary order, so
// the values must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some("a"), Some("b"), Some("c")]);

// It is fused iterator
assert_eq!(values.next(), None);
assert_eq!(values.next(), None);
```

#### Trait Implementations

##### `impl Any for IntoValues<K, V, A>`

- <span id="intovalues-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoValues<K, V, A>`

- <span id="intovalues-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoValues<K, V, A>`

- <span id="intovalues-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V: Debug, A: Allocator> Debug for IntoValues<K, V, A>`

- <span id="intovalues-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoValues<K, V, A>`

- <span id="intovalues-default"></span>`fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoValues<K, V, A>`

- <span id="intovalues-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoValues<K, V, A>`

- <span id="intovalues-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, A: Allocator> FusedIterator for IntoValues<K, V, A>`

##### `impl<U> Into for IntoValues<K, V, A>`

- <span id="intovalues-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoValues<K, V, A>`

- <span id="intovalues-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intovalues-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intovalues-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A>`

- <span id="intovalues-iterator-type-item"></span>`type Item = V`

- <span id="intovalues-iterator-next"></span>`fn next(&mut self) -> Option<V>`

- <span id="intovalues-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intovalues-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for IntoValues<K, V, A>`

- <span id="intovalues-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intovalues-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoValues<K, V, A>`

- <span id="intovalues-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intovalues-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Keys<'a, K, V>`

```rust
struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2507-2509`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2507-L2509)*

An iterator over the keys of a `HashMap` in arbitrary order.
The iterator element type is `&'a K`.

This `struct` is created by the `keys` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut keys = map.keys();
let mut vec = vec![keys.next(), keys.next(), keys.next()];

// The `Keys` iterator produces keys in arbitrary order, so the
// keys must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some(&1), Some(&2), Some(&3)]);

// It is fused iterator
assert_eq!(keys.next(), None);
assert_eq!(keys.next(), None);
```

#### Trait Implementations

##### `impl Any for Keys<'a, K, V>`

- <span id="keys-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Keys<'a, K, V>`

- <span id="keys-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Keys<'a, K, V>`

- <span id="keys-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> Clone for Keys<'_, K, V>`

- <span id="keys-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Keys<'a, K, V>`

- <span id="keys-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K: Debug, V> Debug for Keys<'_, K, V>`

- <span id="keys-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Keys<'_, K, V>`

- <span id="keys-default"></span>`fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Keys<'_, K, V>`

- <span id="keys-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Keys<'a, K, V>`

- <span id="keys-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V> FusedIterator for Keys<'_, K, V>`

##### `impl<U> Into for Keys<'a, K, V>`

- <span id="keys-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Keys<'a, K, V>`

- <span id="keys-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="keys-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="keys-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Keys<'a, K, V>`

- <span id="keys-iterator-type-item"></span>`type Item = &'a K`

- <span id="keys-iterator-next"></span>`fn next(&mut self) -> Option<&'a K>`

- <span id="keys-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="keys-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl ToOwned for Keys<'a, K, V>`

- <span id="keys-toowned-type-owned"></span>`type Owned = T`

- <span id="keys-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="keys-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Keys<'a, K, V>`

- <span id="keys-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="keys-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Keys<'a, K, V>`

- <span id="keys-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="keys-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Values<'a, K, V>`

```rust
struct Values<'a, K, V> {
    inner: Iter<'a, K, V>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2555-2557`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2555-L2557)*

An iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `&'a V`.

This `struct` is created by the `values` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut values = map.values();
let mut vec = vec![values.next(), values.next(), values.next()];

// The `Values` iterator produces values in arbitrary order, so the
// values must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some(&"a"), Some(&"b"), Some(&"c")]);

// It is fused iterator
assert_eq!(values.next(), None);
assert_eq!(values.next(), None);
```

#### Trait Implementations

##### `impl Any for Values<'a, K, V>`

- <span id="values-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Values<'a, K, V>`

- <span id="values-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Values<'a, K, V>`

- <span id="values-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> Clone for Values<'_, K, V>`

- <span id="values-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Values<'a, K, V>`

- <span id="values-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K, V: Debug> Debug for Values<'_, K, V>`

- <span id="values-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Values<'_, K, V>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Values<'_, K, V>`

- <span id="values-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Values<'a, K, V>`

- <span id="values-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V> FusedIterator for Values<'_, K, V>`

##### `impl<U> Into for Values<'a, K, V>`

- <span id="values-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Values<'a, K, V>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Values<'a, K, V>`

- <span id="values-iterator-type-item"></span>`type Item = &'a V`

- <span id="values-iterator-next"></span>`fn next(&mut self) -> Option<&'a V>`

- <span id="values-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="values-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl ToOwned for Values<'a, K, V>`

- <span id="values-toowned-type-owned"></span>`type Owned = T`

- <span id="values-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="values-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Values<'a, K, V>`

- <span id="values-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="values-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Values<'a, K, V>`

- <span id="values-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="values-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, K, V, A: Allocator>`

```rust
struct Drain<'a, K, V, A: Allocator> {
    inner: crate::raw::RawDrain<'a, (K, V), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2603-2605`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2603-L2605)*

A draining iterator over the entries of a `HashMap` in arbitrary
order. The iterator element type is `(K, V)`.

This `struct` is created by the `drain` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<_, _> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut drain_iter = map.drain();
let mut vec = vec![drain_iter.next(), drain_iter.next(), drain_iter.next()];

// The `Drain` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some((1, "a")), Some((2, "b")), Some((3, "c"))]);

// It is fused iterator
assert_eq!(drain_iter.next(), None);
assert_eq!(drain_iter.next(), None);
```

#### Implementations

- <span id="drain-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](../hash_map/index.md#iter)

  Returns a iterator of references over the remaining items.

#### Trait Implementations

##### `impl Any for Drain<'a, K, V, A>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a, K, V, A>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, K, V, A>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, A> Debug for Drain<'_, K, V, A>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> ExactSizeIterator for Drain<'_, K, V, A>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Drain<'a, K, V, A>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, A: Allocator> FusedIterator for Drain<'_, K, V, A>`

##### `impl<U> Into for Drain<'a, K, V, A>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a, K, V, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A>`

- <span id="drain-iterator-type-item"></span>`type Item = (K, V)`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<(K, V)>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for Drain<'a, K, V, A>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drain<'a, K, V, A>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExtractIf<'a, K, V, F, A: Allocator>`

```rust
struct ExtractIf<'a, K, V, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, (K, V), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2650-2653`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2650-L2653)*

A draining iterator over entries of a `HashMap` which don't satisfy the predicate
`f(&k, &mut v)` in arbitrary order. The iterator element type is `(K, V)`.

This `struct` is created by the `extract_if` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<i32, &str> = [(1, "a"), (2, "b"), (3, "c")].into();

let mut extract_if = map.extract_if(|k, _v| k % 2 != 0);
let mut vec = vec![extract_if.next(), extract_if.next()];

// The `ExtractIf` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [Some((1, "a")),Some((3, "c"))]);

// It is fused iterator
assert_eq!(extract_if.next(), None);
assert_eq!(extract_if.next(), None);
drop(extract_if);

assert_eq!(map.len(), 1);
```

#### Trait Implementations

##### `impl Any for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V, F> FusedIterator for ExtractIf<'_, K, V, F>`

##### `impl<U> Into for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V, F, A> Iterator for ExtractIf<'_, K, V, F, A>`

- <span id="extractif-iterator-type-item"></span>`type Item = (K, V)`

- <span id="extractif-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extractif-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExtractIf<'a, K, V, F, A>`

- <span id="extractif-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extractif-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ValuesMut<'a, K, V>`

```rust
struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2702-2704`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2702-L2704)*

A mutable iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `&'a mut V`.

This `struct` is created by the `values_mut` method on [`HashMap`](../hash_map/index.md). See its
documentation for more.


# Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<_, _> = [(1, "One".to_owned()), (2, "Two".into())].into();

let mut values = map.values_mut();
values.next().map(|v| v.push_str(" Mississippi"));
values.next().map(|v| v.push_str(" Mississippi"));

// It is fused iterator
assert_eq!(values.next(), None);
assert_eq!(values.next(), None);

assert_eq!(map.get(&1).unwrap(), &"One Mississippi".to_owned());
assert_eq!(map.get(&2).unwrap(), &"Two Mississippi".to_owned());
```

#### Trait Implementations

##### `impl Any for ValuesMut<'a, K, V>`

- <span id="valuesmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValuesMut<'a, K, V>`

- <span id="valuesmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValuesMut<'a, K, V>`

- <span id="valuesmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V: Debug> Debug for ValuesMut<'_, K, V>`

- <span id="valuesmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for ValuesMut<'_, K, V>`

- <span id="valuesmut-default"></span>`fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V>`

- <span id="valuesmut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ValuesMut<'a, K, V>`

- <span id="valuesmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, V> FusedIterator for ValuesMut<'_, K, V>`

##### `impl<U> Into for ValuesMut<'a, K, V>`

- <span id="valuesmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for ValuesMut<'a, K, V>`

- <span id="valuesmut-iterator-type-item"></span>`type Item = &'a mut V`

- <span id="valuesmut-iterator-next"></span>`fn next(&mut self) -> Option<&'a mut V>`

- <span id="valuesmut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="valuesmut-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for ValuesMut<'a, K, V>`

- <span id="valuesmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuesmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValuesMut<'a, K, V>`

- <span id="valuesmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuesmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedEntry<'a, K, V, S, A: Allocator>`

```rust
struct OccupiedEntry<'a, K, V, S, A: Allocator> {
    hash: u64,
    elem: crate::raw::Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2831-2835`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2831-L2835)*

A view into an occupied entry in a [`HashMap`](../hash_map/index.md).
It is part of the [`Entry`](../hash_map/index.md) and [`EntryRef`](../hash_map/index.md) enums.

# Examples

```rust
use hashbrown::hash_map::{Entry, HashMap, OccupiedEntry};

let mut map = HashMap::new();
map.extend([("a", 10), ("b", 20), ("c", 30)]);

let _entry_o: OccupiedEntry<_, _, _> = map.entry("a").insert(100);
assert_eq!(map.len(), 3);

// Existing key (insert and update)
match map.entry("a") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(mut view) => {
        assert_eq!(view.get(), &100);
        let v = view.get_mut();
        *v *= 10;
        assert_eq!(view.insert(1111), 1000);
    }
}

assert_eq!(map[&"a"], 1111);
assert_eq!(map.len(), 3);

// Existing key (take)
match map.entry("c") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("c", 30));
    }
}
assert_eq!(map.get(&"c"), None);
assert_eq!(map.len(), 2);
```

#### Implementations

- <span id="occupiedentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{Entry, HashMap};

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(12);

  

  match map.entry("poneyland") {

      Entry::Vacant(_) => panic!(),

      Entry::Occupied(entry) => assert_eq!(entry.key(), &"poneyland"),

  }

  ```

- <span id="occupiedentry-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Take the ownership of the key and value from the map.

  Keeps the allocated memory for reuse.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  // The map is empty

  assert!(map.is_empty() && map.capacity() == 0);

  

  map.entry("poneyland").or_insert(12);

  

  if let Entry::Occupied(o) = map.entry("poneyland") {

      // We delete the entry from the map.

      assert_eq!(o.remove_entry(), ("poneyland", 12));

  }

  

  assert_eq!(map.contains_key("poneyland"), false);

  // Now map hold none elements

  assert!(map.is_empty());

  ```

- <span id="occupiedentry-get"></span>`fn get(&self) -> &V`

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(12);

  

  match map.entry("poneyland") {

      Entry::Vacant(_) => panic!(),

      Entry::Occupied(entry) => assert_eq!(entry.get(), &12),

  }

  ```

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the value in the entry.

  

  If you need a reference to the `OccupiedEntry` which may outlive the

  destruction of the `Entry` value, see `into_mut`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(12);

  

  assert_eq!(map["poneyland"], 12);

  if let Entry::Occupied(mut o) = map.entry("poneyland") {

      *o.get_mut() += 10;

      assert_eq!(*o.get(), 22);

  

      // We can use the same Entry multiple times.

      *o.get_mut() += 2;

  }

  

  assert_eq!(map["poneyland"], 24);

  ```

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts the `OccupiedEntry` into a mutable reference to the value in the entry

  with a lifetime bound to the map itself.

  

  If you need multiple references to the `OccupiedEntry`, see `get_mut`.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{Entry, HashMap};

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(12);

  

  assert_eq!(map["poneyland"], 12);

  

  let value: &mut u32;

  match map.entry("poneyland") {

      Entry::Occupied(entry) => value = entry.into_mut(),

      Entry::Vacant(_) => panic!(),

  }

  *value += 10;

  

  assert_eq!(map["poneyland"], 22);

  ```

- <span id="occupiedentry-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry, and returns the entry's old value.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(12);

  

  if let Entry::Occupied(mut o) = map.entry("poneyland") {

      assert_eq!(o.insert(15), 12);

  }

  

  assert_eq!(map["poneyland"], 15);

  ```

- <span id="occupiedentry-remove"></span>`fn remove(self) -> V`

  Takes the value out of the entry, and returns it.

  Keeps the allocated memory for reuse.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  // The map is empty

  assert!(map.is_empty() && map.capacity() == 0);

  

  map.entry("poneyland").or_insert(12);

  

  if let Entry::Occupied(o) = map.entry("poneyland") {

      assert_eq!(o.remove(), 12);

  }

  

  assert_eq!(map.contains_key("poneyland"), false);

  // Now map hold none elements

  assert!(map.is_empty());

  ```

- <span id="occupiedentry-replace-entry-with"></span>`fn replace_entry_with<F>(self, f: F) -> Entry<'a, K, V, S, A>` — [`Entry`](../hash_map/index.md#entry)

  Provides shared access to the key and owned access to the value of

  the entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.insert("poneyland", 42);

  

  let entry = match map.entry("poneyland") {

      Entry::Occupied(e) => {

          e.replace_entry_with(|k, v| {

              assert_eq!(k, &"poneyland");

              assert_eq!(v, 42);

              Some(v + 1)

          })

      }

      Entry::Vacant(_) => panic!(),

  };

  

  match entry {

      Entry::Occupied(e) => {

          assert_eq!(e.key(), &"poneyland");

          assert_eq!(e.get(), &43);

      }

      Entry::Vacant(_) => panic!(),

  }

  

  assert_eq!(map["poneyland"], 43);

  

  let entry = match map.entry("poneyland") {

      Entry::Occupied(e) => e.replace_entry_with(|_k, _v| None),

      Entry::Vacant(_) => panic!(),

  };

  

  match entry {

      Entry::Vacant(e) => {

          assert_eq!(e.key(), &"poneyland");

      }

      Entry::Occupied(_) => panic!(),

  }

  

  assert!(!map.contains_key("poneyland"));

  ```

#### Trait Implementations

##### `impl Any for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedEntry<'_, K, V, S, A>`

- <span id="occupiedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<K, V, S, A> Send for OccupiedEntry<'_, K, V, S, A>`

##### `impl<K, V, S, A> Sync for OccupiedEntry<'_, K, V, S, A>`

##### `impl<U> TryFrom for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OccupiedEntry<'a, K, V, S, A>`

- <span id="occupiedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VacantEntry<'a, K, V, S, A: Allocator>`

```rust
struct VacantEntry<'a, K, V, S, A: Allocator> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2893-2897`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2893-L2897)*

A view into a vacant entry in a `HashMap`.
It is part of the [`Entry`](../hash_set/index.md) enum.

# Examples

```rust
use hashbrown::hash_map::{Entry, HashMap, VacantEntry};

let mut map = HashMap::<&str, i32>::new();

let entry_v: VacantEntry<_, _, _> = match map.entry("a") {
    Entry::Vacant(view) => view,
    Entry::Occupied(_) => unreachable!(),
};
entry_v.insert(10);
assert!(map[&"a"] == 10 && map.len() == 1);

// Nonexistent key (insert and update)
match map.entry("b") {
    Entry::Occupied(_) => unreachable!(),
    Entry::Vacant(view) => {
        let value = view.insert(2);
        assert_eq!(*value, 2);
        *value = 20;
    }
}
assert!(map[&"b"] == 20 && map.len() == 2);
```

#### Implementations

- <span id="vacantentry-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key that would be used when inserting a value

  through the `VacantEntry`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  assert_eq!(map.entry("poneyland").key(), &"poneyland");

  ```

- <span id="vacantentry-into-key"></span>`fn into_key(self) -> K`

  Take ownership of the key.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{Entry, HashMap};

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  match map.entry("poneyland") {

      Entry::Occupied(_) => panic!(),

      Entry::Vacant(v) => assert_eq!(v.into_key(), "poneyland"),

  }

  ```

- <span id="vacantentry-insert"></span>`fn insert(self, value: V) -> &'a mut V`

  Sets the value of the entry with the [`VacantEntry`](../hash_map/index.md)'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  if let Entry::Vacant(o) = map.entry("poneyland") {

      o.insert(37);

  }

  assert_eq!(map["poneyland"], 37);

  ```

- <span id="vacantentry-insert-entry"></span>`fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](../hash_map/index.md#occupiedentry)

  Sets the value of the entry with the [`VacantEntry`](../hash_map/index.md)'s key,

  and returns an [`OccupiedEntry`](../hash_map/index.md).

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  if let Entry::Vacant(v) = map.entry("poneyland") {

      let o = v.insert_entry(37);

      assert_eq!(o.get(), &37);

  }

  ```

#### Trait Implementations

##### `impl Any for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V, S, A: Allocator> Debug for VacantEntry<'_, K, V, S, A>`

- <span id="vacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VacantEntry<'a, K, V, S, A>`

- <span id="vacantentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VacantEntryRef<'map, 'key, K, Q: ?Sized, V, S, A: Allocator>`

```rust
struct VacantEntryRef<'map, 'key, K, Q: ?Sized, V, S, A: Allocator> {
    hash: u64,
    key: &'key Q,
    table: &'map mut HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:3037-3041`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L3037-L3041)*

A view into a vacant entry in a `HashMap`.
It is part of the [`EntryRef`](../hash_map/index.md) enum.

# Examples

```rust
use hashbrown::hash_map::{EntryRef, HashMap, VacantEntryRef};

let mut map = HashMap::<String, i32>::new();

let entry_v: VacantEntryRef<_, _, _, _> = match map.entry_ref("a") {
    EntryRef::Vacant(view) => view,
    EntryRef::Occupied(_) => unreachable!(),
};
entry_v.insert(10);
assert!(map["a"] == 10 && map.len() == 1);

// Nonexistent key (insert and update)
match map.entry_ref("b") {
    EntryRef::Occupied(_) => unreachable!(),
    EntryRef::Vacant(view) => {
        let value = view.insert(2);
        assert_eq!(*value, 2);
        *value = 20;
    }
}
assert!(map["b"] == 20 && map.len() == 2);
```

#### Implementations

- <span id="vacantentryref-key"></span>`fn key(&self) -> &'key Q`

  Gets a reference to the key that would be used when inserting a value

  through the `VacantEntryRef`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  let key: &str = "poneyland";

  assert_eq!(map.entry_ref(key).key(), "poneyland");

  ```

- <span id="vacantentryref-insert"></span>`fn insert(self, value: V) -> &'map mut V`

  Sets the value of the entry with the `VacantEntryRef`'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::EntryRef;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  let key: &str = "poneyland";

  

  if let EntryRef::Vacant(o) = map.entry_ref(key) {

      o.insert(37);

  }

  assert_eq!(map["poneyland"], 37);

  ```

- <span id="vacantentryref-insert-with-key"></span>`fn insert_with_key(self, key: K, value: V) -> &'map mut V`

  Sets the key and value of the entry and returns a mutable reference to

  the inserted value.

  

  Unlike `VacantEntryRef::insert`, this method allows the key to be

  explicitly specified, which is useful for key types that don't implement

  `K: From<&Q>`.

  

  # Panics

  

  This method panics if `key` is not equivalent to the key used to create

  the `VacantEntryRef`.

  

  # Example

  

  ```rust

  use hashbrown::hash_map::EntryRef;

  use hashbrown::HashMap;

  

  let mut map = HashMap::<(String, String), char>::new();

  let k = ("c".to_string(), "C".to_string());

  let v =  match map.entry_ref(&k) {

    // Insert cannot be used here because tuples do not implement From.

    // However this works because we can manually clone instead.

    EntryRef::Vacant(r) => r.insert_with_key(k.clone(), 'c'),

    // In this branch we avoid the clone.

    EntryRef::Occupied(r) => r.into_mut(),

  };

  assert_eq!(*v, 'c');

  ```

- <span id="vacantentryref-insert-entry"></span>`fn insert_entry(self, value: V) -> OccupiedEntry<'map, K, V, S, A>` — [`OccupiedEntry`](../hash_map/index.md#occupiedentry)

  Sets the value of the entry with the [`VacantEntryRef`](../hash_map/index.md)'s key,

  and returns an [`OccupiedEntry`](../hash_map/index.md).

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::EntryRef;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  if let EntryRef::Vacant(v) = map.entry_ref("poneyland") {

      let o = v.insert_entry(37);

      assert_eq!(o.get(), &37);

  }

  ```

#### Trait Implementations

##### `impl Any for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, Q, V, S, A> Debug for VacantEntryRef<'_, '_, K, Q, V, S, A>`

- <span id="vacantentryref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentryref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VacantEntryRef<'map, 'key, K, Q, V, S, A>`

- <span id="vacantentryref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentryref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedError<'a, K, V, S, A: Allocator>`

```rust
struct OccupiedError<'a, K, V, S, A: Allocator> {
    pub entry: OccupiedEntry<'a, K, V, S, A>,
    pub value: V,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:3078-3083`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L3078-L3083)*

The error returned by [`try_insert`](HashMap::try_insert) when the key already exists.

Contains the occupied entry, and the value that was not inserted.

# Examples

```rust
use hashbrown::hash_map::{HashMap, OccupiedError};

let mut map: HashMap<_, _> = [("a", 10), ("b", 20)].into();

// try_insert method returns mutable reference to the value if keys are vacant,
// but if the map did have key present, nothing is updated, and the provided
// value is returned inside `Err(_)` variant
match map.try_insert("a", 100) {
    Err(OccupiedError { mut entry, value }) => {
        assert_eq!(entry.key(), &"a");
        assert_eq!(value, 100);
        assert_eq!(entry.insert(100), 10)
    }
    _ => unreachable!(),
}
assert_eq!(map[&"a"], 100);
```

#### Fields

- **`entry`**: `OccupiedEntry<'a, K, V, S, A>`

  The entry in the map that was already occupied.

- **`value`**: `V`

  The value which was not inserted, because the entry was already occupied.

#### Trait Implementations

##### `impl Any for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedError<'_, K, V, S, A>`

- <span id="occupiederror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Display for OccupiedError<'_, K, V, S, A>`

- <span id="occupiederror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiederror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OccupiedError<'a, K, V, S, A>`

- <span id="occupiederror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiederror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawEntryBuilderMut<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilderMut<'a, K, V, S, A: Allocator> {
    map: &'a mut crate::HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:216-218`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L216-L218)*

A builder for computing where in a [`HashMap`](../hash_map/index.md) a key-value pair would be stored.

See the `HashMap::raw_entry_mut` docs for usage examples.

# Examples

```rust
use hashbrown::hash_map::{RawEntryBuilderMut, RawEntryMut::Vacant, RawEntryMut::Occupied};
use hashbrown::HashMap;
use core::hash::{BuildHasher, Hash};

let mut map = HashMap::new();
map.extend([(1, 11), (2, 12), (3, 13), (4, 14), (5, 15), (6, 16)]);
assert_eq!(map.len(), 6);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let builder: RawEntryBuilderMut<_, _, _> = map.raw_entry_mut();

// Existing key
match builder.from_key(&6) {
    Vacant(_) => unreachable!(),
    Occupied(view) => assert_eq!(view.get(), &16),
}

for key in 0..12 {
    let hash = compute_hash(map.hasher(), &key);
    let value = map.get(&key).cloned();
    let key_value = value.as_ref().map(|v| (&key, v));

    println!("Key: {} and value: {:?}", key, value);

    match map.raw_entry_mut().from_key(&key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
    match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
    match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
        Occupied(mut o) => assert_eq!(Some(o.get_key_value()), key_value),
        Vacant(_) => assert_eq!(value, None),
    }
}

assert_eq!(map.len(), 6);
```

#### Implementations

- <span id="rawentrybuildermut-from-key"></span>`fn from_key<Q>(self, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](../raw_entry/index.md#rawentrymut)

  Creates a `RawEntryMut` from the given key.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);

  entry.insert(key, 100);

  assert_eq!(map[&"a"], 100);

  ```

- <span id="rawentrybuildermut-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](../raw_entry/index.md#rawentrymut)

  Creates a `RawEntryMut` from the given key and its hash.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);

  entry.insert(key, 100);

  assert_eq!(map[&"a"], 100);

  ```

#### Trait Implementations

##### `impl Any for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilderMut<'_, K, V, S, A>`

- <span id="rawentrybuildermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrybuildermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryBuilderMut<'a, K, V, S, A>`

- <span id="rawentrybuildermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrybuildermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawOccupiedEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawOccupiedEntryMut<'a, K, V, S, A: Allocator> {
    elem: crate::raw::Bucket<(K, V)>,
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:395-399`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L395-L399)*

A view into an occupied entry in a `HashMap`.
It is part of the [`RawEntryMut`](../raw_entry/index.md) enum.

# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawOccupiedEntryMut};

let mut map = HashMap::new();
map.extend([("a", 10), ("b", 20), ("c", 30)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let _raw_o: RawOccupiedEntryMut<_, _, _> = map.raw_entry_mut().from_key(&"a").insert("a", 100);
assert_eq!(map.len(), 3);

// Existing key (insert and update)
match map.raw_entry_mut().from_key(&"a") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(mut view) => {
        assert_eq!(view.get(), &100);
        let v = view.get_mut();
        let new_v = (*v) * 10;
        *v = new_v;
        assert_eq!(view.insert(1111), 1000);
    }
}

assert_eq!(map[&"a"], 1111);
assert_eq!(map.len(), 3);

// Existing key (take)
let hash = compute_hash(map.hasher(), &"c");
match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"c") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("c", 30));
    }
}
assert_eq!(map.raw_entry().from_key(&"c"), None);
assert_eq!(map.len(), 2);

let hash = compute_hash(map.hasher(), &"b");
match map.raw_entry_mut().from_hash(hash, |q| *q == "b") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("b", 20));
    }
}
assert_eq!(map.get(&"b"), None);
assert_eq!(map.len(), 1);
```

#### Implementations

- <span id="rawoccupiedentrymut-key"></span>`fn key(&self) -> &K`

  Gets a reference to the key in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.key(), &"a")

  }

  ```

- <span id="rawoccupiedentrymut-key-mut"></span>`fn key_mut(&mut self) -> &mut K`

  Gets a mutable reference to the key in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          *o.key_mut() = key_two.clone();

      }

  }

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-into-key"></span>`fn into_key(self) -> &'a mut K`

  Converts the entry into a mutable reference to the key in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  let inside_key: &mut Rc<&str>;

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => inside_key = o.into_key(),

  }

  *inside_key = key_two.clone();

  

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-get"></span>`fn get(&self) -> &V`

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.get(), &100),

  }

  ```

- <span id="rawoccupiedentrymut-into-mut"></span>`fn into_mut(self) -> &'a mut V`

  Converts the `OccupiedEntry` into a mutable reference to the value in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  let value: &mut u32;

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => value = o.into_mut(),

  }

  *value += 900;

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-get-mut"></span>`fn get_mut(&mut self) -> &mut V`

  Gets a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => *o.get_mut() += 900,

  }

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-get-key-value"></span>`fn get_key_value(&self) -> (&K, &V)`

  Gets a reference to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.get_key_value(), (&"a", &100)),

  }

  ```

- <span id="rawoccupiedentrymut-get-key-value-mut"></span>`fn get_key_value_mut(&mut self) -> (&mut K, &mut V)`

  Gets a mutable reference to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          let (inside_key, inside_value) = o.get_key_value_mut();

          *inside_key = key_two.clone();

          *inside_value = 100;

      }

  }

  assert_eq!(map[&key_two], 100);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-into-key-value"></span>`fn into_key_value(self) -> (&'a mut K, &'a mut V)`

  Converts the `OccupiedEntry` into a mutable reference to the key and value in the entry

  with a lifetime bound to the map itself.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  let inside_key: &mut Rc<&str>;

  let inside_value: &mut u32;

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => {

          let tuple = o.into_key_value();

          inside_key = tuple.0;

          inside_value = tuple.1;

      }

  }

  *inside_key = key_two.clone();

  *inside_value = 100;

  assert_eq!(map[&key_two], 100);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-insert"></span>`fn insert(&mut self, value: V) -> V`

  Sets the value of the entry, and returns the entry's old value.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => assert_eq!(o.insert(1000), 100),

  }

  

  assert_eq!(map[&"a"], 1000);

  ```

- <span id="rawoccupiedentrymut-insert-key"></span>`fn insert_key(&mut self, key: K) -> K`

  Sets the value of the entry, and returns the entry's old value.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  use std::rc::Rc;

  

  let key_one = Rc::new("a");

  let key_two = Rc::new("a");

  

  let mut map: HashMap<Rc<&str>, u32> = HashMap::new();

  map.insert(key_one.clone(), 10);

  

  assert_eq!(map[&key_one], 10);

  assert!(Rc::strong_count(&key_one) == 2 && Rc::strong_count(&key_two) == 1);

  

  match map.raw_entry_mut().from_key(&key_one) {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(mut o) => {

          let old_key = o.insert_key(key_two.clone());

          assert!(Rc::ptr_eq(&old_key, &key_one));

      }

  }

  assert_eq!(map[&key_two], 10);

  assert!(Rc::strong_count(&key_one) == 1 && Rc::strong_count(&key_two) == 2);

  ```

- <span id="rawoccupiedentrymut-remove"></span>`fn remove(self) -> V`

  Takes the value out of the entry, and returns it.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.remove(), 100),

  }

  assert_eq!(map.get(&"a"), None);

  ```

- <span id="rawoccupiedentrymut-remove-entry"></span>`fn remove_entry(self) -> (K, V)`

  Take the ownership of the key and value from the map.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => assert_eq!(o.remove_entry(), ("a", 100)),

  }

  assert_eq!(map.get(&"a"), None);

  ```

- <span id="rawoccupiedentrymut-replace-entry-with"></span>`fn replace_entry_with<F>(self, f: F) -> RawEntryMut<'a, K, V, S, A>` — [`RawEntryMut`](../raw_entry/index.md#rawentrymut)

  Provides shared access to the key and owned access to the value of

  the entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  let raw_entry = match map.raw_entry_mut().from_key(&"a") {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {

          assert_eq!(k, &"a");

          assert_eq!(v, 100);

          Some(v + 900)

      }),

  };

  let raw_entry = match raw_entry {

      RawEntryMut::Vacant(_) => panic!(),

      RawEntryMut::Occupied(o) => o.replace_entry_with(|k, v| {

          assert_eq!(k, &"a");

          assert_eq!(v, 1000);

          None

      }),

  };

  match raw_entry {

      RawEntryMut::Vacant(_) => { },

      RawEntryMut::Occupied(_) => panic!(),

  };

  assert_eq!(map.get(&"a"), None);

  ```

#### Trait Implementations

##### `impl Any for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawOccupiedEntryMut<'_, K, V, S, A>`

- <span id="rawoccupiedentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<K, V, S, A> Send for RawOccupiedEntryMut<'_, K, V, S, A>`

##### `impl<K, V, S, A> Sync for RawOccupiedEntryMut<'_, K, V, S, A>`

##### `impl<U> TryFrom for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawoccupiedentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawOccupiedEntryMut<'a, K, V, S, A>`

- <span id="rawoccupiedentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawoccupiedentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawVacantEntryMut<'a, K, V, S, A: Allocator>`

```rust
struct RawVacantEntryMut<'a, K, V, S, A: Allocator> {
    table: &'a mut crate::raw::RawTable<(K, V), A>,
    hash_builder: &'a S,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:466-469`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L466-L469)*

A view into a vacant entry in a `HashMap`.
It is part of the [`RawEntryMut`](../raw_entry/index.md) enum.

# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawVacantEntryMut};

let mut map = HashMap::<&str, i32>::new();

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

let raw_v: RawVacantEntryMut<_, _, _> = match map.raw_entry_mut().from_key(&"a") {
    RawEntryMut::Vacant(view) => view,
    RawEntryMut::Occupied(_) => unreachable!(),
};
raw_v.insert("a", 10);
assert!(map[&"a"] == 10 && map.len() == 1);

// Nonexistent key (insert and update)
let hash = compute_hash(map.hasher(), &"b");
match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"b") {
    RawEntryMut::Occupied(_) => unreachable!(),
    RawEntryMut::Vacant(view) => {
        let (k, value) = view.insert("b", 2);
        assert_eq!((*k, *value), ("b", 2));
        *value = 20;
    }
}
assert!(map[&"b"] == 20 && map.len() == 2);

let hash = compute_hash(map.hasher(), &"c");
match map.raw_entry_mut().from_hash(hash, |q| *q == "c") {
    RawEntryMut::Occupied(_) => unreachable!(),
    RawEntryMut::Vacant(view) => {
        assert_eq!(view.insert("c", 30), (&mut "c", &mut 30));
    }
}
assert!(map[&"c"] == 30 && map.len() == 3);
```

#### Implementations

- <span id="rawvacantentrymut-insert"></span>`fn insert(self, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Sets the value of the entry with the `VacantEntry`'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  

  match map.raw_entry_mut().from_key(&"c") {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(v.insert("c", 300), (&mut "c", &mut 300)),

  }

  

  assert_eq!(map[&"c"], 300);

  ```

- <span id="rawvacantentrymut-insert-hashed-nocheck"></span>`fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'a mut K, &'a mut V)`

  Sets the value of the entry with the `VacantEntry`'s key,

  and returns a mutable reference to it.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "c";

  let hash = compute_hash(map.hasher(), &key);

  

  match map.raw_entry_mut().from_key_hashed_nocheck(hash, &key) {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(

          v.insert_hashed_nocheck(hash, key, 300),

          (&mut "c", &mut 300)

      ),

  }

  

  assert_eq!(map[&"c"], 300);

  ```

- <span id="rawvacantentrymut-insert-with-hasher"></span>`fn insert_with_hasher<H>(self, hash: u64, key: K, value: V, hasher: H) -> (&'a mut K, &'a mut V)`

  Set the value of an entry with a custom hasher function.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::hash_map::{HashMap, RawEntryMut};

  

  fn make_hasher<K, S>(hash_builder: &S) -> impl Fn(&K) -> u64 + '_

  where

      K: Hash + ?Sized,

      S: BuildHasher,

  {

      move |key: &K| {

          use core::hash::Hasher;

          let mut state = hash_builder.build_hasher();

          key.hash(&mut state);

          state.finish()

      }

  }

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let key = "a";

  let hash_builder = map.hasher().clone();

  let hash = make_hasher(&hash_builder)(&key);

  

  match map.raw_entry_mut().from_hash(hash, |q| q == &key) {

      RawEntryMut::Occupied(_) => panic!(),

      RawEntryMut::Vacant(v) => assert_eq!(

          v.insert_with_hasher(hash, key, 100, make_hasher(&hash_builder)),

          (&mut "a", &mut 100)

      ),

  }

  map.extend([("b", 200), ("c", 300), ("d", 400), ("e", 500), ("f", 600)]);

  assert_eq!(map[&"a"], 100);

  ```

- <span id="rawvacantentrymut-insert-entry"></span>`fn insert_entry(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](../raw_entry/index.md#rawoccupiedentrymut)

#### Trait Implementations

##### `impl Any for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawVacantEntryMut<'_, K, V, S, A>`

- <span id="rawvacantentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawvacantentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawVacantEntryMut<'a, K, V, S, A>`

- <span id="rawvacantentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawvacantentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawEntryBuilder<'a, K, V, S, A: Allocator>`

```rust
struct RawEntryBuilder<'a, K, V, S, A: Allocator> {
    map: &'a crate::HashMap<K, V, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:505-507`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L505-L507)*

A builder for computing where in a [`HashMap`](../hash_map/index.md) a key-value pair would be stored.

See the `HashMap::raw_entry` docs for usage examples.

# Examples

```rust
use hashbrown::hash_map::{HashMap, RawEntryBuilder};
use core::hash::{BuildHasher, Hash};

let mut map = HashMap::new();
map.extend([(1, 10), (2, 20), (3, 30)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

for k in 0..6 {
    let hash = compute_hash(map.hasher(), &k);
    let v = map.get(&k).cloned();
    let kv = v.as_ref().map(|v| (&k, v));

    println!("Key: {} and value: {:?}", k, v);
    let builder: RawEntryBuilder<_, _, _> = map.raw_entry();
    assert_eq!(builder.from_key(&k), kv);
    assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);
}
```

#### Implementations

- <span id="rawentrybuilder-from-key"></span>`fn from_key<Q>(self, k: &Q) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by key.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  assert_eq!(map.raw_entry().from_key(&key), Some((&"a", &100)));

  ```

- <span id="rawentrybuilder-from-key-hashed-nocheck"></span>`fn from_key_hashed_nocheck<Q>(self, hash: u64, k: &Q) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by a key and its hash.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::HashMap;

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &key), Some((&"a", &100)));

  ```

- <span id="rawentrybuilder-search"></span>`fn search<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

- <span id="rawentrybuilder-from-hash"></span>`fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>`

  Access an immutable entry by hash and matching function.

  

  # Examples

  

  ```rust

  use core::hash::{BuildHasher, Hash};

  use hashbrown::HashMap;

  

  fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {

      use core::hash::Hasher;

      let mut state = hash_builder.build_hasher();

      key.hash(&mut state);

      state.finish()

  }

  

  let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();

  let key = "a";

  let hash = compute_hash(map.hasher(), &key);

  assert_eq!(map.raw_entry().from_hash(hash, |k| k == &key), Some((&"a", &100)));

  ```

#### Trait Implementations

##### `impl Any for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V, S, A: Allocator> Debug for RawEntryBuilder<'_, K, V, S, A>`

- <span id="rawentrybuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrybuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryBuilder<'a, K, V, S, A>`

- <span id="rawentrybuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrybuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Entry<'a, K, V, S, A>`

```rust
enum Entry<'a, K, V, S, A>
where
    A: Allocator {
    Occupied(OccupiedEntry<'a, K, V, S, A>),
    Vacant(VacantEntry<'a, K, V, S, A>),
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2749-2782`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2749-L2782)*

A view into a single entry in a map, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashMap`](../hash_map/index.md).


# Examples

```rust
use hashbrown::hash_map::{Entry, HashMap, OccupiedEntry};

let mut map = HashMap::new();
map.extend([("a", 10), ("b", 20), ("c", 30)]);
assert_eq!(map.len(), 3);

// Existing key (insert)
let entry: Entry<_, _, _> = map.entry("a");
let _raw_o: OccupiedEntry<_, _, _> = entry.insert(1);
assert_eq!(map.len(), 3);
// Nonexistent key (insert)
map.entry("d").insert(4);

// Existing key (or_insert)
let v = map.entry("b").or_insert(2);
assert_eq!(std::mem::replace(v, 2), 20);
// Nonexistent key (or_insert)
map.entry("e").or_insert(5);

// Existing key (or_insert_with)
let v = map.entry("c").or_insert_with(|| 3);
assert_eq!(std::mem::replace(v, 3), 30);
// Nonexistent key (or_insert_with)
map.entry("f").or_insert_with(|| 6);

println!("Our HashMap: {:?}", map);

let mut vec: Vec<_> = map.iter().map(|(&k, &v)| (k, v)).collect();
// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3), ("d", 4), ("e", 5), ("f", 6)]);
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_map::{Entry, HashMap};
  let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
  
  match map.entry("a") {
      Entry::Vacant(_) => unreachable!(),
      Entry::Occupied(_) => { }
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_map::{Entry, HashMap};
  let mut map: HashMap<&str, i32> = HashMap::new();
  
  match map.entry("a") {
      Entry::Occupied(_) => unreachable!(),
      Entry::Vacant(_) => { }
  }
  ```

#### Implementations

- <span id="entry-insert"></span>`fn insert(self, value: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](../hash_map/index.md#occupiedentry)

  Sets the value of the entry, and returns an `OccupiedEntry`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let entry = map.entry("horseyland").insert(37);

  

  assert_eq!(entry.key(), &"horseyland");

  ```

- <span id="entry-or-insert"></span>`fn or_insert(self, default: V) -> &'a mut V`

  Ensures a value is in the entry by inserting the default if empty, and returns

  a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  // nonexistent key

  map.entry("poneyland").or_insert(3);

  assert_eq!(map["poneyland"], 3);

  

  // existing key

  *map.entry("poneyland").or_insert(10) *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="entry-or-insert-entry"></span>`fn or_insert_entry(self, default: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](../hash_map/index.md#occupiedentry)

  Ensures a value is in the entry by inserting the default if empty,

  and returns an [`OccupiedEntry`](../hash_map/index.md).

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  // nonexistent key

  let entry = map.entry("poneyland").or_insert_entry(3);

  assert_eq!(entry.key(), &"poneyland");

  assert_eq!(entry.get(), &3);

  

  // existing key

  let mut entry = map.entry("poneyland").or_insert_entry(10);

  assert_eq!(entry.key(), &"poneyland");

  assert_eq!(entry.get(), &3);

  ```

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V`

  Ensures a value is in the entry by inserting the result of the default function if empty,

  and returns a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  // nonexistent key

  map.entry("poneyland").or_insert_with(|| 3);

  assert_eq!(map["poneyland"], 3);

  

  // existing key

  *map.entry("poneyland").or_insert_with(|| 10) *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="entry-or-insert-with-key"></span>`fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V`

  Ensures a value is in the entry by inserting, if empty, the result of the default function.

  This method allows for generating key-derived values for insertion by providing the default

  function a reference to the key that was moved during the `.entry(key)` method call.

  

  The reference to the moved key is provided so that cloning or copying the key is

  unnecessary, unlike with `.or_insert_with(|| ... )`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, usize> = HashMap::new();

  

  // nonexistent key

  map.entry("poneyland").or_insert_with_key(|key| key.chars().count());

  assert_eq!(map["poneyland"], 9);

  

  // existing key

  *map.entry("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;

  assert_eq!(map["poneyland"], 18);

  ```

- <span id="entry-key"></span>`fn key(&self) -> &K`

  Returns a reference to this entry's key.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  map.entry("poneyland").or_insert(3);

  // existing key

  assert_eq!(map.entry("poneyland").key(), &"poneyland");

  // nonexistent key

  assert_eq!(map.entry("horseland").key(), &"horseland");

  ```

- <span id="entry-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  map.entry("poneyland")

     .and_modify(|e| { *e += 1 })

     .or_insert(42);

  assert_eq!(map["poneyland"], 42);

  

  map.entry("poneyland")

     .and_modify(|e| { *e += 1 })

     .or_insert(42);

  assert_eq!(map["poneyland"], 43);

  ```

- <span id="entry-and-replace-entry-with"></span>`fn and_replace_entry_with<F>(self, f: F) -> Self`

  Provides shared access to the key and owned access to the value of

  an occupied entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::Entry;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  let entry = map

      .entry("poneyland")

      .and_replace_entry_with(|_k, _v| panic!());

  

  match entry {

      Entry::Vacant(e) => {

          assert_eq!(e.key(), &"poneyland");

      }

      Entry::Occupied(_) => panic!(),

  }

  

  map.insert("poneyland", 42);

  

  let entry = map

      .entry("poneyland")

      .and_replace_entry_with(|k, v| {

          assert_eq!(k, &"poneyland");

          assert_eq!(v, 42);

          Some(v + 1)

      });

  

  match entry {

      Entry::Occupied(e) => {

          assert_eq!(e.key(), &"poneyland");

          assert_eq!(e.get(), &43);

      }

      Entry::Vacant(_) => panic!(),

  }

  

  assert_eq!(map["poneyland"], 43);

  

  let entry = map

      .entry("poneyland")

      .and_replace_entry_with(|_k, _v| None);

  

  match entry {

      Entry::Vacant(e) => assert_eq!(e.key(), &"poneyland"),

      Entry::Occupied(_) => panic!(),

  }

  

  assert!(!map.contains_key("poneyland"));

  ```

#### Trait Implementations

##### `impl Any for Entry<'a, K, V, S, A>`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry<'a, K, V, S, A>`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry<'a, K, V, S, A>`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for Entry<'_, K, V, S, A>`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Entry<'a, K, V, S, A>`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Entry<'a, K, V, S, A>`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Entry<'a, K, V, S, A>`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Entry<'a, K, V, S, A>`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EntryRef<'a, 'b, K, Q: ?Sized, V, S, A>`

```rust
enum EntryRef<'a, 'b, K, Q: ?Sized, V, S, A>
where
    A: Allocator {
    Occupied(OccupiedEntry<'a, K, V, S, A>),
    Vacant(VacantEntryRef<'a, 'b, K, Q, V, S, A>),
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:2957-2990`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L2957-L2990)*

A view into a single entry in a map, which may either be vacant or occupied,
with any borrowed form of the map's key type.


This `enum` is constructed from the `entry_ref` method on [`HashMap`](../hash_map/index.md).

`Hash` and `Eq` on the borrowed form of the map's key type *must* match those
for the key type. It also require that key may be constructed from the borrowed
form through the `From` trait.





# Examples

```rust
use hashbrown::hash_map::{EntryRef, HashMap, OccupiedEntry};

let mut map = HashMap::new();
map.extend([("a".to_owned(), 10), ("b".into(), 20), ("c".into(), 30)]);
assert_eq!(map.len(), 3);

// Existing key (insert)
let key = String::from("a");
let entry: EntryRef<_, _, _, _> = map.entry_ref(&key);
let _raw_o: OccupiedEntry<_, _, _, _> = entry.insert(1);
assert_eq!(map.len(), 3);
// Nonexistent key (insert)
map.entry_ref("d").insert(4);

// Existing key (or_insert)
let v = map.entry_ref("b").or_insert(2);
assert_eq!(std::mem::replace(v, 2), 20);
// Nonexistent key (or_insert)
map.entry_ref("e").or_insert(5);

// Existing key (or_insert_with)
let v = map.entry_ref("c").or_insert_with(|| 3);
assert_eq!(std::mem::replace(v, 3), 30);
// Nonexistent key (or_insert_with)
map.entry_ref("f").or_insert_with(|| 6);

println!("Our HashMap: {:?}", map);

for (key, value) in ["a", "b", "c", "d", "e", "f"].into_iter().zip(1..=6) {
    assert_eq!(map[key], value)
}
assert_eq!(map.len(), 6);
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_map::{EntryRef, HashMap};
  let mut map: HashMap<_, _> = [("a".to_owned(), 100), ("b".into(), 200)].into();
  
  match map.entry_ref("a") {
      EntryRef::Vacant(_) => unreachable!(),
      EntryRef::Occupied(_) => { }
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  use hashbrown::hash_map::{EntryRef, HashMap};
  let mut map: HashMap<String, i32> = HashMap::new();
  
  match map.entry_ref("a") {
      EntryRef::Occupied(_) => unreachable!(),
      EntryRef::Vacant(_) => { }
  }
  ```

#### Implementations

- <span id="entryref-insert"></span>`fn insert(self, value: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](../hash_map/index.md#occupiedentry)

  Sets the value of the entry, and returns an `OccupiedEntry`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  let entry = map.entry_ref("horseyland").insert(37);

  

  assert_eq!(entry.key(), "horseyland");

  ```

- <span id="entryref-or-insert"></span>`fn or_insert(self, default: V) -> &'a mut V`

  Ensures a value is in the entry by inserting the default if empty, and returns

  a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  

  // nonexistent key

  map.entry_ref("poneyland").or_insert(3);

  assert_eq!(map["poneyland"], 3);

  

  // existing key

  *map.entry_ref("poneyland").or_insert(10) *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="entryref-or-insert-with"></span>`fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V`

  Ensures a value is in the entry by inserting the result of the default function if empty,

  and returns a mutable reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  

  // nonexistent key

  map.entry_ref("poneyland").or_insert_with(|| 3);

  assert_eq!(map["poneyland"], 3);

  

  // existing key

  *map.entry_ref("poneyland").or_insert_with(|| 10) *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="entryref-or-insert-with-key"></span>`fn or_insert_with_key<F: FnOnce(&Q) -> V>(self, default: F) -> &'a mut V`

  Ensures a value is in the entry by inserting, if empty, the result of the default function.

  This method allows for generating key-derived values for insertion by providing the default

  function an access to the borrower form of the key.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, usize> = HashMap::new();

  

  // nonexistent key

  map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count());

  assert_eq!(map["poneyland"], 9);

  

  // existing key

  *map.entry_ref("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;

  assert_eq!(map["poneyland"], 18);

  ```

- <span id="entryref-key"></span>`fn key(&self) -> &Q`

  Returns a reference to this entry's key.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  map.entry_ref("poneyland").or_insert(3);

  // existing key

  assert_eq!(map.entry_ref("poneyland").key(), "poneyland");

  // nonexistent key

  assert_eq!(map.entry_ref("horseland").key(), "horseland");

  ```

- <span id="entryref-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<String, u32> = HashMap::new();

  

  map.entry_ref("poneyland")

     .and_modify(|e| { *e += 1 })

     .or_insert(42);

  assert_eq!(map["poneyland"], 42);

  

  map.entry_ref("poneyland")

     .and_modify(|e| { *e += 1 })

     .or_insert(42);

  assert_eq!(map["poneyland"], 43);

  ```

#### Trait Implementations

##### `impl Any for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, Q, V, S, A> Debug for EntryRef<'_, '_, K, Q, V, S, A>`

- <span id="entryref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entryref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EntryRef<'a, 'b, K, Q, V, S, A>`

- <span id="entryref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entryref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawEntryMut<'a, K, V, S, A: Allocator>`

```rust
enum RawEntryMut<'a, K, V, S, A: Allocator> {
    Occupied(RawOccupiedEntryMut<'a, K, V, S, A>),
    Vacant(RawVacantEntryMut<'a, K, V, S, A>),
}
```

*Defined in [`hashbrown-0.16.1/src/raw_entry.rs:304-333`](../../../.source_1765521767/hashbrown-0.16.1/src/raw_entry.rs#L304-L333)*

A view into a single entry in a map, which may either be vacant or occupied.

This is a lower-level version of [`Entry`](../hash_set/index.md).

This `enum` is constructed through the `raw_entry_mut` method on [`HashMap`](../hash_map/index.md),
then calling one of the methods of that [`RawEntryBuilderMut`](../raw_entry/index.md).




# Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut, RawOccupiedEntryMut};

let mut map = HashMap::new();
map.extend([('a', 1), ('b', 2), ('c', 3)]);
assert_eq!(map.len(), 3);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

// Existing key (insert)
let raw: RawEntryMut<_, _, _> = map.raw_entry_mut().from_key(&'a');
let _raw_o: RawOccupiedEntryMut<_, _, _> = raw.insert('a', 10);
assert_eq!(map.len(), 3);

// Nonexistent key (insert)
map.raw_entry_mut().from_key(&'d').insert('d', 40);
assert_eq!(map.len(), 4);

// Existing key (or_insert)
let hash = compute_hash(map.hasher(), &'b');
let kv = map
    .raw_entry_mut()
    .from_key_hashed_nocheck(hash, &'b')
    .or_insert('b', 20);
assert_eq!(kv, (&mut 'b', &mut 2));
*kv.1 = 20;
assert_eq!(map.len(), 4);

// Nonexistent key (or_insert)
let hash = compute_hash(map.hasher(), &'e');
let kv = map
    .raw_entry_mut()
    .from_key_hashed_nocheck(hash, &'e')
    .or_insert('e', 50);
assert_eq!(kv, (&mut 'e', &mut 50));
assert_eq!(map.len(), 5);

// Existing key (or_insert_with)
let hash = compute_hash(map.hasher(), &'c');
let kv = map
    .raw_entry_mut()
    .from_hash(hash, |q| q == &'c')
    .or_insert_with(|| ('c', 30));
assert_eq!(kv, (&mut 'c', &mut 3));
*kv.1 = 30;
assert_eq!(map.len(), 5);

// Nonexistent key (or_insert_with)
let hash = compute_hash(map.hasher(), &'f');
let kv = map
    .raw_entry_mut()
    .from_hash(hash, |q| q == &'f')
    .or_insert_with(|| ('f', 60));
assert_eq!(kv, (&mut 'f', &mut 60));
assert_eq!(map.len(), 6);

println!("Our HashMap: {:?}", map);

let mut vec: Vec<_> = map.iter().map(|(&k, &v)| (k, v)).collect();
// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [('a', 10), ('b', 20), ('c', 30), ('d', 40), ('e', 50), ('f', 60)]);
```

#### Variants

- **`Occupied`**

  An occupied entry.
  
  # Examples
  
  ```rust
  use hashbrown::{hash_map::RawEntryMut, HashMap};
  let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();
  
  match map.raw_entry_mut().from_key(&"a") {
      RawEntryMut::Vacant(_) => unreachable!(),
      RawEntryMut::Occupied(_) => { }
  }
  ```

- **`Vacant`**

  A vacant entry.
  
  # Examples
  
  ```rust
  use hashbrown::{hash_map::RawEntryMut, HashMap};
  let mut map: HashMap<&str, i32> = HashMap::new();
  
  match map.raw_entry_mut().from_key("a") {
      RawEntryMut::Occupied(_) => unreachable!(),
      RawEntryMut::Vacant(_) => { }
  }
  ```

#### Implementations

- <span id="rawentrymut-insert"></span>`fn insert(self, key: K, value: V) -> RawOccupiedEntryMut<'a, K, V, S, A>` — [`RawOccupiedEntryMut`](../raw_entry/index.md#rawoccupiedentrymut)

  Sets the value of the entry, and returns a `RawOccupiedEntryMut`.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  let entry = map.raw_entry_mut().from_key("horseyland").insert("horseyland", 37);

  

  assert_eq!(entry.remove_entry(), ("horseyland", 37));

  ```

- <span id="rawentrymut-or-insert"></span>`fn or_insert(self, default_key: K, default_val: V) -> (&'a mut K, &'a mut V)`

  Ensures a value is in the entry by inserting the default if empty, and returns

  mutable references to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);

  assert_eq!(map["poneyland"], 3);

  

  *map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10).1 *= 2;

  assert_eq!(map["poneyland"], 6);

  ```

- <span id="rawentrymut-or-insert-with"></span>`fn or_insert_with<F>(self, default: F) -> (&'a mut K, &'a mut V)`

  Ensures a value is in the entry by inserting the result of the default function if empty,

  and returns mutable references to the key and value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, String> = HashMap::new();

  

  map.raw_entry_mut().from_key("poneyland").or_insert_with(|| {

      ("poneyland", "hoho".to_string())

  });

  

  assert_eq!(map["poneyland"], "hoho".to_string());

  ```

- <span id="rawentrymut-and-modify"></span>`fn and_modify<F>(self, f: F) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the map.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  map.raw_entry_mut()

     .from_key("poneyland")

     .and_modify(|_k, v| { *v += 1 })

     .or_insert("poneyland", 42);

  assert_eq!(map["poneyland"], 42);

  

  map.raw_entry_mut()

     .from_key("poneyland")

     .and_modify(|_k, v| { *v += 1 })

     .or_insert("poneyland", 0);

  assert_eq!(map["poneyland"], 43);

  ```

- <span id="rawentrymut-and-replace-entry-with"></span>`fn and_replace_entry_with<F>(self, f: F) -> Self`

  Provides shared access to the key and owned access to the value of

  an occupied entry and allows to replace or remove it based on the

  value of the returned option.

  

  # Examples

  

  ```rust

  use hashbrown::HashMap;

  use hashbrown::hash_map::RawEntryMut;

  

  let mut map: HashMap<&str, u32> = HashMap::new();

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|_k, _v| panic!());

  

  match entry {

      RawEntryMut::Vacant(_) => {},

      RawEntryMut::Occupied(_) => panic!(),

  }

  

  map.insert("poneyland", 42);

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|k, v| {

          assert_eq!(k, &"poneyland");

          assert_eq!(v, 42);

          Some(v + 1)

      });

  

  match entry {

      RawEntryMut::Occupied(e) => {

          assert_eq!(e.key(), &"poneyland");

          assert_eq!(e.get(), &43);

      },

      RawEntryMut::Vacant(_) => panic!(),

  }

  

  assert_eq!(map["poneyland"], 43);

  

  let entry = map

      .raw_entry_mut()

      .from_key("poneyland")

      .and_replace_entry_with(|_k, _v| None);

  

  match entry {

      RawEntryMut::Vacant(_) => {},

      RawEntryMut::Occupied(_) => panic!(),

  }

  

  assert!(!map.contains_key("poneyland"));

  ```

#### Trait Implementations

##### `impl Any for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for RawEntryMut<'_, K, V, S, A>`

- <span id="rawentrymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawentrymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawEntryMut<'a, K, V, S, A>`

- <span id="rawentrymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawentrymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `make_hasher`

```rust
fn make_hasher<Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    Q: Hash,
    S: BuildHasher
```

*Defined in [`hashbrown-0.16.1/src/map.rs:209-215`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L209-L215)*

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `equivalent_key`

```rust
fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    Q: Equivalent<K> + ?Sized
```

*Defined in [`hashbrown-0.16.1/src/map.rs:220-225`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L220-L225)*

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `equivalent`

```rust
fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized
```

*Defined in [`hashbrown-0.16.1/src/map.rs:231-236`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L231-L236)*

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `make_hash`

```rust
fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher
```

*Defined in [`hashbrown-0.16.1/src/map.rs:240-249`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L240-L249)*

### `assert_covariance`

```rust
fn assert_covariance()
```

*Defined in [`hashbrown-0.16.1/src/map.rs:4814-4854`](../../../.source_1765521767/hashbrown-0.16.1/src/map.rs#L4814-L4854)*

