# Crate `hashbrown`

This crate is a Rust port of Google's high-performance [SwissTable] hash
map, adapted to make it a drop-in replacement for Rust's standard `HashMap`
and `HashSet` types.

The original C++ version of [SwissTable] can be found [here], and this
[CppCon talk] gives an overview of how the algorithm works.




## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`control`](#control)
  - [`hasher`](#hasher)
  - [`raw`](#raw)
  - [`util`](#util)
  - [`external_trait_impls`](#external_trait_impls)
  - [`map`](#map)
  - [`raw_entry`](#raw_entry)
  - [`scopeguard`](#scopeguard)
  - [`set`](#set)
  - [`table`](#table)
  - [`hash_map`](#hash_map)
  - [`hash_set`](#hash_set)
  - [`hash_table`](#hash_table)
- [Structs](#structs)
  - [`DefaultHashBuilder`](#defaulthashbuilder)
  - [`DefaultHasher`](#defaulthasher)
  - [`HashMap`](#hashmap)
  - [`HashSet`](#hashset)
  - [`HashTable`](#hashtable)
- [Enums](#enums)
  - [`TryReserveError`](#tryreserveerror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`control`](#control) | mod |  |
| [`hasher`](#hasher) | mod |  |
| [`raw`](#raw) | mod |  |
| [`util`](#util) | mod |  |
| [`external_trait_impls`](#external_trait_impls) | mod |  |
| [`map`](#map) | mod |  |
| [`raw_entry`](#raw_entry) | mod |  |
| [`scopeguard`](#scopeguard) | mod |  |
| [`set`](#set) | mod |  |
| [`table`](#table) | mod |  |
| [`hash_map`](#hash_map) | mod | A hash map implemented with quadratic probing and SIMD lookup. |
| [`hash_set`](#hash_set) | mod | A hash set implemented as a `HashMap` where the value is `()`. |
| [`hash_table`](#hash_table) | mod | A hash table implemented with quadratic probing and SIMD lookup. |
| [`DefaultHashBuilder`](#defaulthashbuilder) | struct |  |
| [`DefaultHasher`](#defaulthasher) | struct |  |
| [`HashMap`](#hashmap) | struct |  |
| [`HashSet`](#hashset) | struct |  |
| [`HashTable`](#hashtable) | struct |  |
| [`TryReserveError`](#tryreserveerror) | enum | The error type for `try_reserve` methods. |

## Modules

- [`macros`](macros/index.md)
- [`control`](control/index.md)
- [`hasher`](hasher/index.md)
- [`raw`](raw/index.md)
- [`util`](util/index.md)
- [`external_trait_impls`](external_trait_impls/index.md)
- [`map`](map/index.md)
- [`raw_entry`](raw_entry/index.md)
- [`scopeguard`](scopeguard/index.md)
- [`set`](set/index.md)
- [`table`](table/index.md)
- [`hash_map`](hash_map/index.md) — A hash map implemented with quadratic probing and SIMD lookup.
- [`hash_set`](hash_set/index.md) — A hash set implemented as a `HashMap` where the value is `()`.
- [`hash_table`](hash_table/index.md) — A hash table implemented with quadratic probing and SIMD lookup.

## Structs

### `DefaultHashBuilder`

```rust
struct DefaultHashBuilder {
    inner: foldhash::fast::RandomState,
}
```

*Defined in [`hashbrown-0.16.1/src/hasher.rs:14-17`](../../.source_1765210505/hashbrown-0.16.1/src/hasher.rs#L14-L17)*

Default hash builder for the `S` type parameter of
[`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

This only implements `BuildHasher` when the "default-hasher" crate feature
is enabled; otherwise it just serves as a placeholder, and a custom `S` type
must be used to have a fully functional `HashMap` or `HashSet`.

#### Trait Implementations

##### `impl BuildHasher for DefaultHashBuilder`

- <span id="defaulthashbuilder-type-hasher"></span>`type Hasher = DefaultHasher`

- <span id="defaulthashbuilder-build-hasher"></span>`fn build_hasher(&self) -> <Self as >::Hasher`

##### `impl Clone for DefaultHashBuilder`

- <span id="defaulthashbuilder-clone"></span>`fn clone(&self) -> DefaultHashBuilder` — [`DefaultHashBuilder`](hasher/index.md#defaulthashbuilder)

##### `impl Debug for DefaultHashBuilder`

- <span id="defaulthashbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DefaultHashBuilder`

- <span id="defaulthashbuilder-default"></span>`fn default() -> DefaultHashBuilder` — [`DefaultHashBuilder`](hasher/index.md#defaulthashbuilder)

### `DefaultHasher`

```rust
struct DefaultHasher {
    inner: <foldhash::fast::RandomState as BuildHasher>::Hasher,
}
```

*Defined in [`hashbrown-0.16.1/src/hasher.rs:34-36`](../../.source_1765210505/hashbrown-0.16.1/src/hasher.rs#L34-L36)*

Default hasher for [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet).

#### Trait Implementations

##### `impl Clone for DefaultHasher`

- <span id="defaulthasher-clone"></span>`fn clone(&self) -> DefaultHasher` — [`DefaultHasher`](hasher/index.md#defaulthasher)

##### `impl Hasher for DefaultHasher`

- <span id="defaulthasher-write"></span>`fn write(&mut self, arg: &[u8])`

- <span id="defaulthasher-write-u8"></span>`fn write_u8(&mut self, arg: u8)`

- <span id="defaulthasher-write-u16"></span>`fn write_u16(&mut self, arg: u16)`

- <span id="defaulthasher-write-u32"></span>`fn write_u32(&mut self, arg: u32)`

- <span id="defaulthasher-write-u64"></span>`fn write_u64(&mut self, arg: u64)`

- <span id="defaulthasher-write-u128"></span>`fn write_u128(&mut self, arg: u128)`

- <span id="defaulthasher-write-usize"></span>`fn write_usize(&mut self, arg: usize)`

- <span id="defaulthasher-write-i8"></span>`fn write_i8(&mut self, arg: i8)`

- <span id="defaulthasher-write-i16"></span>`fn write_i16(&mut self, arg: i16)`

- <span id="defaulthasher-write-i32"></span>`fn write_i32(&mut self, arg: i32)`

- <span id="defaulthasher-write-i64"></span>`fn write_i64(&mut self, arg: i64)`

- <span id="defaulthasher-write-i128"></span>`fn write_i128(&mut self, arg: i128)`

- <span id="defaulthasher-write-isize"></span>`fn write_isize(&mut self, arg: isize)`

- <span id="defaulthasher-finish"></span>`fn finish(&self) -> u64`

### `HashMap<K, V, S, A: Allocator>`

```rust
struct HashMap<K, V, S, A: Allocator> {
    hash_builder: S,
    table: crate::raw::RawTable<(K, V), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/map.rs:185-188`](../../.source_1765210505/hashbrown-0.16.1/src/map.rs#L185-L188)*

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

- <span id="hashmap-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

#### Trait Implementations

##### `impl<K: Clone, V: Clone, S: Clone, A: Allocator + Clone> Clone for HashMap<K, V, S, A>`

- <span id="hashmap-clone"></span>`fn clone(&self) -> Self`

- <span id="hashmap-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<K, V, S, A> Debug for HashMap<K, V, S, A>`

- <span id="hashmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S, A> Default for HashMap<K, V, S, A>`

- <span id="hashmap-default"></span>`fn default() -> Self`

##### `impl<K, V, S, A> Eq for HashMap<K, V, S, A>`

##### `impl<Q, K> Equivalent for HashMap<K, V, S, A>`

- <span id="hashmap-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<K, V, S, A> Extend for HashMap<K, V, S, A>`

- <span id="hashmap-extend"></span>`fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T)`

##### `impl<K, V, S, A> FromIterator for HashMap<K, V, S, A>`

- <span id="hashmap-from-iter"></span>`fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self`

##### `impl<K, Q, V, S, A> Index for HashMap<K, V, S, A>`

- <span id="hashmap-type-output"></span>`type Output = V`

- <span id="hashmap-index"></span>`fn index(&self, key: &Q) -> &V`

##### `impl<'a, K, V, S, A: Allocator> IntoIterator for &'a HashMap<K, V, S, A>`

- <span id="a-hashmap-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="a-hashmap-type-intoiter"></span>`type IntoIter = Iter<'a, K, V>`

- <span id="a-hashmap-into-iter"></span>`fn into_iter(self) -> Iter<'a, K, V>` — [`Iter`](hash_map/index.md#iter)

##### `impl<K, V, S, A> PartialEq for HashMap<K, V, S, A>`

- <span id="hashmap-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `HashSet<T, S, A: Allocator>`

```rust
struct HashSet<T, S, A: Allocator> {
    map: super::map::HashMap<T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:114-116`](../../.source_1765210505/hashbrown-0.16.1/src/set.rs#L114-L116)*

A hash set implemented as a `HashMap` where the value is `()`.

As with the [`HashMap`](hash_map/index.md) type, a `HashSet` requires that the elements
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

- <span id="hashset-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitand"></span>`fn bitand(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> BitAndAssign for HashSet<T, S, A>`

- <span id="hashset-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> BitOr for &HashSet<T, S, A>`

- <span id="hashset-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitor"></span>`fn bitor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> BitOrAssign for HashSet<T, S, A>`

- <span id="hashset-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> BitXor for &HashSet<T, S, A>`

- <span id="hashset-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitxor"></span>`fn bitxor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> BitXorAssign for HashSet<T, S, A>`

- <span id="hashset-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T: Clone, S: Clone, A: Allocator + Clone> Clone for HashSet<T, S, A>`

- <span id="hashset-clone"></span>`fn clone(&self) -> Self`

- <span id="hashset-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T, S, A> Debug for HashSet<T, S, A>`

- <span id="hashset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> Default for HashSet<T, S, A>`

- <span id="hashset-default"></span>`fn default() -> Self`

##### `impl<T, S, A> Eq for HashSet<T, S, A>`

##### `impl<Q, K> Equivalent for HashSet<T, S, A>`

- <span id="hashset-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T, S, A> Extend for HashSet<T, S, A>`

- <span id="hashset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T, S, A> FromIterator for HashSet<T, S, A>`

- <span id="hashset-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<'a, T, S, A: Allocator> IntoIterator for &'a HashSet<T, S, A>`

- <span id="a-hashset-type-item"></span>`type Item = &'a T`

- <span id="a-hashset-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-hashset-into-iter"></span>`fn into_iter(self) -> Iter<'a, T>` — [`Iter`](hash_set/index.md#iter)

##### `impl<T, S, A> PartialEq for HashSet<T, S, A>`

- <span id="hashset-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T, S, A> Sub for &HashSet<T, S, A>`

- <span id="hashset-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-sub"></span>`fn sub(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](hash_set/index.md#hashset)

##### `impl<T, S, A> SubAssign for HashSet<T, S, A>`

- <span id="hashset-sub-assign"></span>`fn sub_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](hash_set/index.md#hashset)

### `HashTable<T, A>`

```rust
struct HashTable<T, A>
where
    A: Allocator {
    raw: crate::raw::RawTable<T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:48-53`](../../.source_1765210505/hashbrown-0.16.1/src/table.rs#L48-L53)*

Low-level hash table with explicit hashing.

The primary use case for this type over [`HashMap`](hash_map/index.md) or [`HashSet`](hash_set/index.md) is to
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
[`HashMap`](hash_map/index.md) and [`HashSet`](hash_set/index.md). Specifically, the API allows you to shoot
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

- <span id="hashtable-type-item"></span>`type Item = T`

- <span id="hashtable-type-intoiter"></span>`type IntoIter = IntoIter<T, A>`

- <span id="hashtable-into-iter"></span>`fn into_iter(self) -> IntoIter<T, A>` — [`IntoIter`](hash_table/index.md#intoiter)

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

*Defined in [`hashbrown-0.16.1/src/lib.rs:180-190`](../../.source_1765210505/hashbrown-0.16.1/src/lib.rs#L180-L190)*

The error type for `try_reserve` methods.

#### Variants

- **`CapacityOverflow`**

  Error due to the computed capacity exceeding the collection's maximum
  (usually `isize::MAX` bytes).

- **`AllocError`**

  The memory allocator returned an error

#### Trait Implementations

##### `impl Clone for TryReserveError`

- <span id="tryreserveerror-clone"></span>`fn clone(&self) -> TryReserveError` — [`TryReserveError`](#tryreserveerror)

##### `impl Debug for TryReserveError`

- <span id="tryreserveerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryReserveError`

##### `impl Equivalent for TryReserveError`

- <span id="tryreserveerror-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for TryReserveError`

- <span id="tryreserveerror-eq"></span>`fn eq(&self, other: &TryReserveError) -> bool` — [`TryReserveError`](#tryreserveerror)

##### `impl StructuralPartialEq for TryReserveError`

