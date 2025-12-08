# Crate `hashbrown`

This crate is a Rust port of Google's high-performance [SwissTable] hash
map, adapted to make it a drop-in replacement for Rust's standard `HashMap`
and `HashSet` types.

The original C++ version of [SwissTable] can be found [here], and this
[CppCon talk] gives an overview of how the algorithm works.




## Modules

- [`macros`](macros/index.md) - 
- [`control`](control/index.md) - 
- [`hasher`](hasher/index.md) - 
- [`raw`](raw/index.md) - 
- [`util`](util/index.md) - 
- [`external_trait_impls`](external_trait_impls/index.md) - 
- [`map`](map/index.md) - 
- [`raw_entry`](raw_entry/index.md) - 
- [`scopeguard`](scopeguard/index.md) - 
- [`set`](set/index.md) - 
- [`table`](table/index.md) - 
- [`hash_map`](hash_map/index.md) - A hash map implemented with quadratic probing and SIMD lookup.
- [`hash_set`](hash_set/index.md) - A hash set implemented as a `HashMap` where the value is `()`.
- [`hash_table`](hash_table/index.md) - A hash table implemented with quadratic probing and SIMD lookup.

## Structs

### `DefaultHashBuilder`

```rust
struct DefaultHashBuilder {
    inner: foldhash::fast::RandomState,
}
```

Default hash builder for the `S` type parameter of
[`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

This only implements `BuildHasher` when the "default-hasher" crate feature
is enabled; otherwise it just serves as a placeholder, and a custom `S` type
must be used to have a fully functional `HashMap` or `HashSet`.

#### Trait Implementations

##### `impl BuildHasher for DefaultHashBuilder`

- `type Hasher = DefaultHasher`

- `fn build_hasher(self: &Self) -> <Self as >::Hasher`

##### `impl Clone for DefaultHashBuilder`

- `fn clone(self: &Self) -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

##### `impl Debug for DefaultHashBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DefaultHashBuilder`

- `fn default() -> DefaultHashBuilder` — [`DefaultHashBuilder`](#defaulthashbuilder)

### `DefaultHasher`

```rust
struct DefaultHasher {
    inner: <foldhash::fast::RandomState as BuildHasher>::Hasher,
}
```

Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

#### Trait Implementations

##### `impl Clone for DefaultHasher`

- `fn clone(self: &Self) -> DefaultHasher` — [`DefaultHasher`](#defaulthasher)

##### `impl Hasher for DefaultHasher`

- `fn write(self: &mut Self, arg: &[u8])`

- `fn write_u8(self: &mut Self, arg: u8)`

- `fn write_u16(self: &mut Self, arg: u16)`

- `fn write_u32(self: &mut Self, arg: u32)`

- `fn write_u64(self: &mut Self, arg: u64)`

- `fn write_u128(self: &mut Self, arg: u128)`

- `fn write_usize(self: &mut Self, arg: usize)`

- `fn write_i8(self: &mut Self, arg: i8)`

- `fn write_i16(self: &mut Self, arg: i16)`

- `fn write_i32(self: &mut Self, arg: i32)`

- `fn write_i64(self: &mut Self, arg: i64)`

- `fn write_i128(self: &mut Self, arg: i128)`

- `fn write_isize(self: &mut Self, arg: isize)`

- `fn finish(self: &Self) -> u64`

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

- `fn raw_entry_mut(self: &mut Self) -> RawEntryBuilderMut<'_, K, V, S, A>` — [`RawEntryBuilderMut`](raw_entry/index.md)

- `fn raw_entry(self: &Self) -> RawEntryBuilder<'_, K, V, S, A>` — [`RawEntryBuilder`](raw_entry/index.md)

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

- `fn into_iter(self: Self) -> IntoIter<K, V, A>` — [`IntoIter`](hash_map/index.md)

##### `impl<K, V, S, A> PartialEq for HashMap<K, V, S, A>`

- `fn eq(self: &Self, other: &Self) -> bool`

### `HashSet<T, S, A: Allocator>`

```rust
struct HashSet<T, S, A: Allocator> {
    map: super::map::HashMap<T, (), S, A>,
}
```

A hash set implemented as a `HashMap` where the value is `()`.

As with the [`HashMap`](#hashmap) type, a `HashSet` requires that the elements
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

- `fn allocator(self: &Self) -> &A`

- `const fn with_hasher_in(hasher: S, alloc: A) -> Self`

- `fn with_capacity_and_hasher_in(capacity: usize, hasher: S, alloc: A) -> Self`

- `fn hasher(self: &Self) -> &S`

#### Trait Implementations

##### `impl<T, S, A> BitAndAssign for HashSet<T, S, A>`

- `fn bitand_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitOrAssign for HashSet<T, S, A>`

- `fn bitor_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

##### `impl<T, S, A> BitXorAssign for HashSet<T, S, A>`

- `fn bitxor_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

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

##### `impl<T, S, A> Extend for HashSet<T, S, A>`

- `fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I)`

##### `impl<T, S, A> FromIterator for HashSet<T, S, A>`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T, S, A: Allocator> IntoIterator for HashSet<T, S, A>`

- `type Item = T`

- `type IntoIter = IntoIter<T, A>`

- `fn into_iter(self: Self) -> IntoIter<T, A>` — [`IntoIter`](hash_set/index.md)

##### `impl<T, S, A> PartialEq for HashSet<T, S, A>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T, S, A> SubAssign for HashSet<T, S, A>`

- `fn sub_assign(self: &mut Self, rhs: &HashSet<T, S, A>)` — [`HashSet`](#hashset)

### `HashTable<T, A>`

```rust
struct HashTable<T, A>
where
    A: Allocator {
    raw: crate::raw::RawTable<T, A>,
}
```

Low-level hash table with explicit hashing.

The primary use case for this type over [`HashMap`](#hashmap) or [`HashSet`](#hashset) is to
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
[`HashMap`](#hashmap) and [`HashSet`](#hashset). Specifically, the API allows you to shoot
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

- `fn find_entry(self: &mut Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>>` — [`OccupiedEntry`](hash_table/index.md), [`AbsentEntry`](hash_table/index.md)

- `fn find_bucket_index(self: &Self, hash: u64, eq: impl FnMut(&T) -> bool) -> Option<usize>`

- `fn entry(self: &mut Self, hash: u64, eq: impl FnMut(&T) -> bool, hasher: impl Fn(&T) -> u64) -> Entry<'_, T, A>` — [`Entry`](hash_table/index.md)

- `fn get_bucket_entry(self: &mut Self, index: usize) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>>` — [`OccupiedEntry`](hash_table/index.md), [`AbsentEntry`](hash_table/index.md)

- `unsafe fn get_bucket_entry_unchecked(self: &mut Self, index: usize) -> OccupiedEntry<'_, T, A>` — [`OccupiedEntry`](hash_table/index.md)

- `fn get_bucket(self: &Self, index: usize) -> Option<&T>`

- `unsafe fn get_bucket_unchecked(self: &Self, index: usize) -> &T`

- `fn get_bucket_mut(self: &mut Self, index: usize) -> Option<&mut T>`

- `unsafe fn get_bucket_unchecked_mut(self: &mut Self, index: usize) -> &mut T`

- `fn insert_unique(self: &mut Self, hash: u64, value: T, hasher: impl Fn(&T) -> u64) -> OccupiedEntry<'_, T, A>` — [`OccupiedEntry`](hash_table/index.md)

- `fn clear(self: &mut Self)`

- `fn shrink_to_fit(self: &mut Self, hasher: impl Fn(&T) -> u64)`

- `fn shrink_to(self: &mut Self, min_capacity: usize, hasher: impl Fn(&T) -> u64)`

- `fn reserve(self: &mut Self, additional: usize, hasher: impl Fn(&T) -> u64)`

- `fn try_reserve(self: &mut Self, additional: usize, hasher: impl Fn(&T) -> u64) -> Result<(), TryReserveError>` — [`TryReserveError`](#tryreserveerror)

- `fn num_buckets(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> Iter<'_, T>` — [`Iter`](hash_table/index.md)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, T>` — [`IterMut`](hash_table/index.md)

- `fn iter_buckets(self: &Self) -> IterBuckets<'_, T>` — [`IterBuckets`](hash_table/index.md)

- `fn iter_hash(self: &Self, hash: u64) -> IterHash<'_, T>` — [`IterHash`](hash_table/index.md)

- `fn iter_hash_mut(self: &mut Self, hash: u64) -> IterHashMut<'_, T>` — [`IterHashMut`](hash_table/index.md)

- `fn iter_hash_buckets(self: &Self, hash: u64) -> IterHashBuckets<'_, T>` — [`IterHashBuckets`](hash_table/index.md)

- `fn retain(self: &mut Self, f: impl FnMut(&mut T) -> bool)`

- `fn drain(self: &mut Self) -> Drain<'_, T, A>` — [`Drain`](hash_table/index.md)

- `fn extract_if<F>(self: &mut Self, f: F) -> ExtractIf<'_, T, F, A>` — [`ExtractIf`](hash_table/index.md)

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

- `fn into_iter(self: Self) -> IntoIter<T, A>` — [`IntoIter`](hash_table/index.md)

## Enums

### `TryReserveError`

```rust
enum TryReserveError {
    CapacityOverflow,
    AllocError {
        layout: alloc::alloc::Layout,
    },
}
```

The error type for `try_reserve` methods.

#### Variants

- **`CapacityOverflow`**

  Error due to the computed capacity exceeding the collection's maximum
  (usually `isize::MAX` bytes).

- **`AllocError`**

  The memory allocator returned an error

#### Trait Implementations

##### `impl Clone for TryReserveError`

- `fn clone(self: &Self) -> TryReserveError` — [`TryReserveError`](#tryreserveerror)

##### `impl Debug for TryReserveError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TryReserveError`

##### `impl<Q, K> Equivalent for TryReserveError`

- `fn equivalent(self: &Self, key: &K) -> bool`

##### `impl PartialEq for TryReserveError`

- `fn eq(self: &Self, other: &TryReserveError) -> bool` — [`TryReserveError`](#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

