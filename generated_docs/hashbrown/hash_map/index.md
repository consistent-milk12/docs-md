*[hashbrown](../index.md) / [hash_map](index.md)*

---

# Module `hash_map`

A hash map implemented with quadratic probing and SIMD lookup.

## Structs

### `HashMap<K, V, S, A: Allocator>`

```rust
struct HashMap<K, V, S, A: Allocator> {
    hash_builder: S,
    table: crate::raw::RawTable<(K, V), A>,
}
```

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

- `fn allocator(self: &Self) -> &A`

- `const fn with_hasher_in(hash_builder: S, alloc: A) -> Self`

- `fn with_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self`

- `fn hasher(self: &Self) -> &S`

- `fn capacity(self: &Self) -> usize`

- `fn keys(self: &Self) -> Keys<'_, K, V>` — [`Keys`](#keys)

- `fn values(self: &Self) -> Values<'_, K, V>` — [`Values`](#values)

- `fn values_mut(self: &mut Self) -> ValuesMut<'_, K, V>` — [`ValuesMut`](#valuesmut)

- `fn iter(self: &Self) -> Iter<'_, K, V>` — [`Iter`](#iter)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, K, V>` — [`IterMut`](#itermut)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn drain(self: &mut Self) -> Drain<'_, K, V, A>` — [`Drain`](#drain)

- `fn retain<F>(self: &mut Self, f: F)`

- `fn extract_if<F>(self: &mut Self, f: F) -> ExtractIf<'_, K, V, F, A>` — [`ExtractIf`](#extractif)

- `fn clear(self: &mut Self)`

- `fn into_keys(self: Self) -> IntoKeys<K, V, A>` — [`IntoKeys`](#intokeys)

- `fn into_values(self: Self) -> IntoValues<K, V, A>` — [`IntoValues`](#intovalues)

#### Trait Implementations

##### `impl<K: Clone, V: Clone, S: Clone, A: Allocator + Clone> Clone for HashMap<K, V, S, A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<K, V, S, A> Debug for HashMap<K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S, A> Default for HashMap<K, V, S, A>`

- `fn default() -> Self`

##### `impl<K, V, S, A> Eq for HashMap<K, V, S, A>`

##### `impl<Q, K> Equivalent for HashMap<K, V, S, A>`

- `fn equivalent(self: &Self, key: &K) -> bool`

##### `impl<K, V, S, A> Extend for HashMap<K, V, S, A>`

- `fn extend<T: IntoIterator<Item = (K, V)>>(self: &mut Self, iter: T)`

##### `impl<K, V, S, A> FromIterator for HashMap<K, V, S, A>`

- `fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self`

##### `impl<K, Q, V, S, A> Index for HashMap<K, V, S, A>`

- `type Output = V`

- `fn index(self: &Self, key: &Q) -> &V`

##### `impl<K, V, S, A: Allocator> IntoIterator for HashMap<K, V, S, A>`

- `type Item = (K, V)`

- `type IntoIter = IntoIter<K, V, A>`

- `fn into_iter(self: Self) -> IntoIter<K, V, A>` — [`IntoIter`](#intoiter)

##### `impl<K, V, S, A> PartialEq for HashMap<K, V, S, A>`

- `fn eq(self: &Self, other: &Self) -> bool`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    inner: crate::raw::RawIter<(K, V)>,
    marker: core::marker::PhantomData<(&'a K, &'a V)>,
}
```

An iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(&'a K, &'a V)`.

This `struct` is created by the [`iter`](#iter) method on [`HashMap`](../index.md). See its
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

##### `impl<K, V> Clone for Iter<'_, K, V>`

- `fn clone(self: &Self) -> Self`

##### `impl<K: Debug, V: Debug> Debug for Iter<'_, K, V>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Iter<'_, K, V>`

- `fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V> FusedIterator for Iter<'_, K, V>`

##### `impl<I> IntoIterator for Iter<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for Iter<'a, K, V>`

- `type Item = (&'a K, &'a V)`

- `fn next(self: &mut Self) -> Option<(&'a K, &'a V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    inner: crate::raw::RawIter<(K, V)>,
    marker: core::marker::PhantomData<(&'a K, &'a mut V)>,
}
```

A mutable iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(&'a K, &'a mut V)`.

This `struct` is created by the `iter_mut` method on [`HashMap`](../index.md). See its
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

- `fn iter(self: &Self) -> Iter<'_, K, V>` — [`Iter`](#iter)

#### Trait Implementations

##### `impl<K, V> Debug for IterMut<'_, K, V>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for IterMut<'_, K, V>`

- `fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V> FusedIterator for IterMut<'_, K, V>`

##### `impl<I> IntoIterator for IterMut<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for IterMut<'a, K, V>`

- `type Item = (&'a K, &'a mut V)`

- `fn next(self: &mut Self) -> Option<(&'a K, &'a mut V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

##### `impl<K: Send, V: Send> Send for IterMut<'_, K, V>`

### `IntoIter<K, V, A: Allocator>`

```rust
struct IntoIter<K, V, A: Allocator> {
    inner: crate::raw::RawIntoIter<(K, V), A>,
}
```

An owning iterator over the entries of a `HashMap` in arbitrary order.
The iterator element type is `(K, V)`.

This `struct` is created by the `into_iter` method on [`HashMap`](../index.md)
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

- `fn iter(self: &Self) -> Iter<'_, K, V>` — [`Iter`](#iter)

#### Trait Implementations

##### `impl<K: Debug, V: Debug, A: Allocator> Debug for IntoIter<K, V, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoIter<K, V, A>`

- `fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A>`

##### `impl<I> IntoIterator for IntoIter<K, V, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A>`

- `type Item = (K, V)`

- `fn next(self: &mut Self) -> Option<(K, V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `IntoKeys<K, V, A: Allocator>`

```rust
struct IntoKeys<K, V, A: Allocator> {
    inner: IntoIter<K, V, A>,
}
```

An owning iterator over the keys of a `HashMap` in arbitrary order.
The iterator element type is `K`.

This `struct` is created by the `into_keys` method on [`HashMap`](../index.md).
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

##### `impl<K: Debug, V: Debug, A: Allocator> Debug for IntoKeys<K, V, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoKeys<K, V, A>`

- `fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoKeys<K, V, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V, A: Allocator> FusedIterator for IntoKeys<K, V, A>`

##### `impl<I> IntoIterator for IntoKeys<K, V, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A>`

- `type Item = K`

- `fn next(self: &mut Self) -> Option<K>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `IntoValues<K, V, A: Allocator>`

```rust
struct IntoValues<K, V, A: Allocator> {
    inner: IntoIter<K, V, A>,
}
```

An owning iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `V`.

This `struct` is created by the `into_values` method on [`HashMap`](../index.md).
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

##### `impl<K, V: Debug, A: Allocator> Debug for IntoValues<K, V, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> Default for IntoValues<K, V, A>`

- `fn default() -> Self`

##### `impl<K, V, A: Allocator> ExactSizeIterator for IntoValues<K, V, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V, A: Allocator> FusedIterator for IntoValues<K, V, A>`

##### `impl<I> IntoIterator for IntoValues<K, V, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A>`

- `type Item = V`

- `fn next(self: &mut Self) -> Option<V>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Keys<'a, K, V>`

```rust
struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>,
}
```

An iterator over the keys of a `HashMap` in arbitrary order.
The iterator element type is `&'a K`.

This `struct` is created by the `keys` method on [`HashMap`](../index.md). See its
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

##### `impl<K, V> Clone for Keys<'_, K, V>`

- `fn clone(self: &Self) -> Self`

##### `impl<K: Debug, V> Debug for Keys<'_, K, V>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Keys<'_, K, V>`

- `fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Keys<'_, K, V>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V> FusedIterator for Keys<'_, K, V>`

##### `impl<I> IntoIterator for Keys<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for Keys<'a, K, V>`

- `type Item = &'a K`

- `fn next(self: &mut Self) -> Option<&'a K>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Values<'a, K, V>`

```rust
struct Values<'a, K, V> {
    inner: Iter<'a, K, V>,
}
```

An iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `&'a V`.

This `struct` is created by the `values` method on [`HashMap`](../index.md). See its
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

##### `impl<K, V> Clone for Values<'_, K, V>`

- `fn clone(self: &Self) -> Self`

##### `impl<K, V: Debug> Debug for Values<'_, K, V>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for Values<'_, K, V>`

- `fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for Values<'_, K, V>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V> FusedIterator for Values<'_, K, V>`

##### `impl<I> IntoIterator for Values<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for Values<'a, K, V>`

- `type Item = &'a V`

- `fn next(self: &mut Self) -> Option<&'a V>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `Drain<'a, K, V, A: Allocator>`

```rust
struct Drain<'a, K, V, A: Allocator> {
    inner: crate::raw::RawDrain<'a, (K, V), A>,
}
```

A draining iterator over the entries of a `HashMap` in arbitrary
order. The iterator element type is `(K, V)`.

This `struct` is created by the `drain` method on [`HashMap`](../index.md). See its
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

- `fn iter(self: &Self) -> Iter<'_, K, V>` — [`Iter`](#iter)

#### Trait Implementations

##### `impl<K, V, A> Debug for Drain<'_, K, V, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, A: Allocator> ExactSizeIterator for Drain<'_, K, V, A>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V, A: Allocator> FusedIterator for Drain<'_, K, V, A>`

##### `impl<I> IntoIterator for Drain<'a, K, V, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A>`

- `type Item = (K, V)`

- `fn next(self: &mut Self) -> Option<(K, V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `ExtractIf<'a, K, V, F, A: Allocator>`

```rust
struct ExtractIf<'a, K, V, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, (K, V), A>,
}
```

A draining iterator over entries of a `HashMap` which don't satisfy the predicate
`f(&k, &mut v)` in arbitrary order. The iterator element type is `(K, V)`.

This `struct` is created by the `extract_if` method on [`HashMap`](../index.md). See its
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

##### `impl<K, V, F> FusedIterator for ExtractIf<'_, K, V, F>`

##### `impl<I> IntoIterator for ExtractIf<'a, K, V, F, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<K, V, F, A> Iterator for ExtractIf<'_, K, V, F, A>`

- `type Item = (K, V)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ValuesMut<'a, K, V>`

```rust
struct ValuesMut<'a, K, V> {
    inner: IterMut<'a, K, V>,
}
```

A mutable iterator over the values of a `HashMap` in arbitrary order.
The iterator element type is `&'a mut V`.

This `struct` is created by the `values_mut` method on [`HashMap`](../index.md). See its
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

##### `impl<K, V: Debug> Debug for ValuesMut<'_, K, V>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V> Default for ValuesMut<'_, K, V>`

- `fn default() -> Self`

##### `impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V>`

- `fn len(self: &Self) -> usize`

##### `impl<K, V> FusedIterator for ValuesMut<'_, K, V>`

##### `impl<I> IntoIterator for ValuesMut<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for ValuesMut<'a, K, V>`

- `type Item = &'a mut V`

- `fn next(self: &mut Self) -> Option<&'a mut V>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn fold<B, F>(self: Self, init: B, f: F) -> B`

### `OccupiedEntry<'a, K, V, S, A: Allocator>`

```rust
struct OccupiedEntry<'a, K, V, S, A: Allocator> {
    hash: u64,
    elem: crate::raw::Bucket<(K, V)>,
    table: &'a mut HashMap<K, V, S, A>,
}
```

A view into an occupied entry in a [`HashMap`](../index.md).
It is part of the [`Entry`](#entry) and [`EntryRef`](#entryref) enums.

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

- `fn key(self: &Self) -> &K`

- `fn remove_entry(self: Self) -> (K, V)`

- `fn get(self: &Self) -> &V`

- `fn get_mut(self: &mut Self) -> &mut V`

- `fn into_mut(self: Self) -> &'a mut V`

- `fn insert(self: &mut Self, value: V) -> V`

- `fn remove(self: Self) -> V`

- `fn replace_entry_with<F>(self: Self, f: F) -> Entry<'a, K, V, S, A>` — [`Entry`](#entry)

#### Trait Implementations

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedEntry<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S, A> Send for OccupiedEntry<'_, K, V, S, A>`

##### `impl<K, V, S, A> Sync for OccupiedEntry<'_, K, V, S, A>`

### `VacantEntry<'a, K, V, S, A: Allocator>`

```rust
struct VacantEntry<'a, K, V, S, A: Allocator> {
    hash: u64,
    key: K,
    table: &'a mut HashMap<K, V, S, A>,
}
```

A view into a vacant entry in a `HashMap`.
It is part of the [`Entry`](#entry) enum.

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

- `fn key(self: &Self) -> &K`

- `fn into_key(self: Self) -> K`

- `fn insert(self: Self, value: V) -> &'a mut V`

- `fn insert_entry(self: Self, value: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](#occupiedentry)

#### Trait Implementations

##### `impl<K: Debug, V, S, A: Allocator> Debug for VacantEntry<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `VacantEntryRef<'map, 'key, K, Q: ?Sized, V, S, A: Allocator>`

```rust
struct VacantEntryRef<'map, 'key, K, Q: ?Sized, V, S, A: Allocator> {
    hash: u64,
    key: &'key Q,
    table: &'map mut HashMap<K, V, S, A>,
}
```

A view into a vacant entry in a `HashMap`.
It is part of the [`EntryRef`](#entryref) enum.

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

- `fn key(self: &Self) -> &'key Q`

- `fn insert(self: Self, value: V) -> &'map mut V`

- `fn insert_with_key(self: Self, key: K, value: V) -> &'map mut V`

- `fn insert_entry(self: Self, value: V) -> OccupiedEntry<'map, K, V, S, A>` — [`OccupiedEntry`](#occupiedentry)

#### Trait Implementations

##### `impl<K, Q, V, S, A> Debug for VacantEntryRef<'_, '_, K, Q, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OccupiedError<'a, K, V, S, A: Allocator>`

```rust
struct OccupiedError<'a, K, V, S, A: Allocator> {
    pub entry: OccupiedEntry<'a, K, V, S, A>,
    pub value: V,
}
```

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

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for OccupiedError<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K: Debug, V: Debug, S, A: Allocator> Display for OccupiedError<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for OccupiedError<'a, K, V, S, A>`

- `fn to_string(self: &Self) -> String`

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

A view into a single entry in a map, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashMap`](../index.md).


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

- `fn or_default(self: Self) -> &'a mut V`

#### Trait Implementations

##### `impl<K: Debug, V: Debug, S, A: Allocator> Debug for Entry<'_, K, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EntryRef<'a, 'b, K, Q: ?Sized, V, S, A>`

```rust
enum EntryRef<'a, 'b, K, Q: ?Sized, V, S, A>
where
    A: Allocator {
    Occupied(OccupiedEntry<'a, K, V, S, A>),
    Vacant(VacantEntryRef<'a, 'b, K, Q, V, S, A>),
}
```

A view into a single entry in a map, which may either be vacant or occupied,
with any borrowed form of the map's key type.


This `enum` is constructed from the `entry_ref` method on [`HashMap`](../index.md).

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

- `fn insert(self: Self, value: V) -> OccupiedEntry<'a, K, V, S, A>` — [`OccupiedEntry`](#occupiedentry)

- `fn or_insert(self: Self, default: V) -> &'a mut V`

- `fn or_insert_with<F: FnOnce() -> V>(self: Self, default: F) -> &'a mut V`

- `fn or_insert_with_key<F: FnOnce(&Q) -> V>(self: Self, default: F) -> &'a mut V`

- `fn key(self: &Self) -> &Q`

- `fn and_modify<F>(self: Self, f: F) -> Self`

#### Trait Implementations

##### `impl<K, Q, V, S, A> Debug for EntryRef<'_, '_, K, Q, V, S, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `make_hasher`

```rust
fn make_hasher<Q, V, S>(hash_builder: &S) -> impl Fn(&(Q, V)) -> u64 + '_
where
    Q: Hash,
    S: BuildHasher
```

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `equivalent_key`

```rust
fn equivalent_key<Q, K, V>(k: &Q) -> impl Fn(&(K, V)) -> bool + '_
where
    Q: Equivalent<K> + ?Sized
```

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `equivalent`

```rust
fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized
```

Ensures that a single closure type across uses of this which, in turn prevents multiple
instances of any functions like `RawTable::reserve` from being generated

### `make_hash`

```rust
fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher
```

### `assert_covariance`

```rust
fn assert_covariance()
```

