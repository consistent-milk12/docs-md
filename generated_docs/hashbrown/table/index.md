*[hashbrown](../index.md) / [table](index.md)*

---

# Module `table`

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
| [`AbsentEntry`](#absententry) | struct | Type representing the absence of an entry, as returned by [`HashTable::find_entry`] and [`HashTable::get_bucket_entry`]. |
| [`Iter`](#iter) | struct | An iterator over the entries of a `HashTable` in arbitrary order. |
| [`IterMut`](#itermut) | struct | A mutable iterator over the entries of a `HashTable` in arbitrary order. |
| [`IterBuckets`](#iterbuckets) | struct | An iterator producing the `usize` indices of all occupied buckets, within the range `0..table.num_buckets()`. |
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

*Defined in [`hashbrown-0.16.1/src/table.rs:48-53`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L48-L53)*

Low-level hash table with explicit hashing.

The primary use case for this type over [`HashMap`](../hash_map/index.md) or [`HashSet`](../hash_set/index.md) is to
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
[`HashMap`](../hash_map/index.md) and [`HashSet`](../hash_set/index.md). Specifically, the API allows you to shoot
yourself in the foot by having multiple elements with identical keys in the
table. The table itself will still function correctly and lookups will
arbitrarily return one of the matching elements. However you should avoid
doing this because it changes the runtime of hash table operations from
`O(1)` to `O(k)` where `k` is the number of duplicate entries.





#### Implementations

- <span id="hashtable-new"></span>`const fn new() -> Self`

  Creates an empty `HashTable`.

  

  The hash table is initially created with a capacity of 0, so it will not allocate until it

  is first inserted into.

  

  # Examples

  

  ```rust

  use hashbrown::HashTable;

  let mut table: HashTable<&str> = HashTable::new();

  assert_eq!(table.len(), 0);

  assert_eq!(table.capacity(), 0);

  ```

- <span id="hashtable-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Creates an empty `HashTable` with the specified capacity.

  

  The hash table will be able to hold at least `capacity` elements without

  reallocating. If `capacity` is 0, the hash table will not allocate.

  

  # Examples

  

  ```rust

  use hashbrown::HashTable;

  let mut table: HashTable<&str> = HashTable::with_capacity(10);

  assert_eq!(table.len(), 0);

  assert!(table.capacity() >= 10);

  ```

#### Trait Implementations

##### `impl<T> Any for HashTable<T, A>`

- <span id="hashtable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HashTable<T, A>`

- <span id="hashtable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HashTable<T, A>`

- <span id="hashtable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, A> Clone for HashTable<T, A>`

- <span id="hashtable-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for HashTable<T, A>`

- <span id="hashtable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, A> Debug for HashTable<T, A>`

- <span id="hashtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A> Default for HashTable<T, A>`

- <span id="hashtable-default"></span>`fn default() -> Self`

##### `impl<T> From for HashTable<T, A>`

- <span id="hashtable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for HashTable<T, A>`

- <span id="hashtable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, A> IntoIterator for HashTable<T, A>`

- <span id="hashtable-intoiterator-type-item"></span>`type Item = T`

- <span id="hashtable-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<T, A>`

- <span id="hashtable-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter<T, A>` — [`IntoIter`](../hash_table/index.md#intoiter)

##### `impl<T> ToOwned for HashTable<T, A>`

- <span id="hashtable-toowned-type-owned"></span>`type Owned = T`

- <span id="hashtable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hashtable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for HashTable<T, A>`

- <span id="hashtable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hashtable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for HashTable<T, A>`

- <span id="hashtable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hashtable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedEntry<'a, T, A>`

```rust
struct OccupiedEntry<'a, T, A>
where
    A: Allocator {
    bucket: crate::raw::Bucket<T>,
    table: &'a mut HashTable<T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:1975-1981`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L1975-L1981)*

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

- <span id="occupiedentry-remove"></span>`fn remove(self) -> (T, VacantEntry<'a, T, A>)` — [`VacantEntry`](../hash_table/index.md#vacantentry)

  Takes the value out of the entry, and returns it along with a

  `VacantEntry` that can be used to insert another value with the same

  hash as the one that was just removed.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::hash_table::Entry;

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<&str> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  // The table is empty

  assert!(table.is_empty() && table.capacity() == 0);

  

  table.insert_unique(hasher(&"poneyland"), "poneyland", hasher);

  let capacity_before_remove = table.capacity();

  

  if let Entry::Occupied(o) = table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {

      assert_eq!(o.remove().0, "poneyland");

  }

  

  assert!(table

      .find(hasher(&"poneyland"), |&x| x == "poneyland")

      .is_none());

  // Now table hold none elements but capacity is equal to the old one

  assert!(table.len() == 0 && table.capacity() == capacity_before_remove);

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="occupiedentry-get"></span>`fn get(&self) -> &T`

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::hash_table::Entry;

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<&str> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  table.insert_unique(hasher(&"poneyland"), "poneyland", hasher);

  

  match table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {

      Entry::Vacant(_) => panic!(),

      Entry::Occupied(entry) => assert_eq!(entry.get(), &"poneyland"),

  }

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="occupiedentry-get-mut"></span>`fn get_mut(&mut self) -> &mut T`

  Gets a mutable reference to the value in the entry.

  

  If you need a reference to the `OccupiedEntry` which may outlive the

  destruction of the `Entry` value, see `into_mut`.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::hash_table::Entry;

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<(&str, u32)> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

  

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),

      Some(&("poneyland", 12))

  );

  

  if let Entry::Occupied(mut o) = table.entry(

      hasher(&"poneyland"),

      |&(x, _)| x == "poneyland",

      |(k, _)| hasher(&k),

  ) {

      o.get_mut().1 += 10;

      assert_eq!(o.get().1, 22);

  

      // We can use the same Entry multiple times.

      o.get_mut().1 += 2;

  }

  

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),

      Some(&("poneyland", 24))

  );

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="occupiedentry-into-mut"></span>`fn into_mut(self) -> &'a mut T`

  Converts the `OccupiedEntry` into a mutable reference to the value in the entry

  with a lifetime bound to the table itself.

  

  If you need multiple references to the `OccupiedEntry`, see `get_mut`.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::hash_table::Entry;

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<(&str, u32)> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  table.insert_unique(hasher(&"poneyland"), ("poneyland", 12), |(k, _)| hasher(&k));

  

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),

      Some(&("poneyland", 12))

  );

  

  let value: &mut (&str, u32);

  match table.entry(

      hasher(&"poneyland"),

      |&(x, _)| x == "poneyland",

      |(k, _)| hasher(&k),

  ) {

      Entry::Occupied(entry) => value = entry.into_mut(),

      Entry::Vacant(_) => panic!(),

  }

  value.1 += 10;

  

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(x, _)| x == "poneyland",),

      Some(&("poneyland", 22))

  );

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="occupiedentry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../hash_table/index.md#hashtable)

  Converts the `OccupiedEntry` into a mutable reference to the underlying

  table.

- <span id="occupiedentry-bucket-index"></span>`fn bucket_index(&self) -> usize`

  Returns the bucket index in the table for this entry.

  

  This can be used to store a borrow-free "reference" to the entry, later using

  `HashTable::get_bucket`, `HashTable::get_bucket_mut`, or

  `HashTable::get_bucket_entry` to access it again without hash probing.

  

  The index is only meaningful as long as the table is not resized and no entries are added

  or removed. After such changes, it may end up pointing to a different entry or none at all.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  table.insert_unique(hasher(&1), (1, 1), |val| hasher(&val.0));

  table.insert_unique(hasher(&2), (2, 2), |val| hasher(&val.0));

  table.insert_unique(hasher(&3), (3, 3), |val| hasher(&val.0));

  

  let index = table

      .entry(hasher(&2), |val| val.0 == 2, |val| hasher(&val.0))

      .or_insert((2, -2))

      .bucket_index();

  assert_eq!(table.get_bucket(index), Some(&(2, 2)));

  

  // Full mutation would invalidate any normal reference

  for (_key, value) in &mut table {

      *value *= 11;

  }

  

  // The index still reaches the same key with the updated value

  assert_eq!(table.get_bucket(index), Some(&(2, 22)));

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

#### Trait Implementations

##### `impl<T> Any for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for OccupiedEntry<'_, T, A>`

- <span id="occupiedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, A> Send for OccupiedEntry<'_, T, A>`

##### `impl<T, A> Sync for OccupiedEntry<'_, T, A>`

##### `impl<T, U> TryFrom for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OccupiedEntry<'a, T, A>`

- <span id="occupiedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`hashbrown-0.16.1/src/table.rs:2286-2293`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2286-L2293)*

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

- <span id="vacantentry-insert"></span>`fn insert(self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](../hash_table/index.md#occupiedentry)

  Inserts a new element into the table with the hash that was used to

  obtain the `VacantEntry`.

  

  An `OccupiedEntry` is returned for the newly inserted element.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::hash_table::Entry;

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<&str> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  

  if let Entry::Vacant(o) = table.entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher) {

      o.insert("poneyland");

  }

  assert_eq!(

      table.find(hasher(&"poneyland"), |&x| x == "poneyland"),

      Some(&"poneyland")

  );

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="vacantentry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../hash_table/index.md#hashtable)

  Converts the `VacantEntry` into a mutable reference to the underlying

  table.

#### Trait Implementations

##### `impl<T> Any for VacantEntry<'a, T, A>`

- <span id="vacantentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntry<'a, T, A>`

- <span id="vacantentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntry<'a, T, A>`

- <span id="vacantentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for VacantEntry<'_, T, A>`

- <span id="vacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VacantEntry<'a, T, A>`

- <span id="vacantentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for VacantEntry<'a, T, A>`

- <span id="vacantentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for VacantEntry<'a, T, A>`

- <span id="vacantentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for VacantEntry<'a, T, A>`

- <span id="vacantentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AbsentEntry<'a, T, A>`

```rust
struct AbsentEntry<'a, T, A>
where
    A: Allocator {
    table: &'a mut HashTable<T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2398-2403`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2398-L2403)*

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

- <span id="absententry-into-table"></span>`fn into_table(self) -> &'a mut HashTable<T, A>` — [`HashTable`](../hash_table/index.md#hashtable)

  Converts the `AbsentEntry` into a mutable reference to the underlying

  table.

#### Trait Implementations

##### `impl<T> Any for AbsentEntry<'a, T, A>`

- <span id="absententry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AbsentEntry<'a, T, A>`

- <span id="absententry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AbsentEntry<'a, T, A>`

- <span id="absententry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for AbsentEntry<'_, T, A>`

- <span id="absententry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AbsentEntry<'a, T, A>`

- <span id="absententry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for AbsentEntry<'a, T, A>`

- <span id="absententry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for AbsentEntry<'a, T, A>`

- <span id="absententry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="absententry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for AbsentEntry<'a, T, A>`

- <span id="absententry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="absententry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: crate::raw::RawIter<T>,
    marker: core::marker::PhantomData<&'a T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2430-2433`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2430-L2433)*

An iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `&'a T`.

This `struct` is created by the [`iter`](#iter) method on [`HashTable`](../hash_table/index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Any for Iter<'a, T>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, T>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, T>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Iter<'a, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'a, T>` — [`Iter`](../hash_table/index.md#iter)

##### `impl<T> CloneToUninit for Iter<'a, T>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Iter<'_, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Iter<'_, T>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Iter<'a, T>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl<T, U> Into for Iter<'a, T>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a T`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for Iter<'a, T>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Iter<'a, T>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<'a, T>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: crate::raw::RawIter<T>,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2503-2506`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2503-L2506)*

A mutable iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `&'a mut T`.

This `struct` is created by the `iter_mut` method on [`HashTable`](../hash_table/index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Any for IterMut<'a, T>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a, T>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a, T>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for IterMut<'_, T>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterMut<'_, T>`

- <span id="itermut-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterMut<'_, T>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IterMut<'a, T>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for IterMut<'_, T>`

##### `impl<T, U> Into for IterMut<'a, T>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterMut<'a, T>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterMut<'a, T>`

- <span id="itermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="itermut-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> TryFrom for IterMut<'a, T>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterMut<'a, T>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterBuckets<'a, T>`

```rust
struct IterBuckets<'a, T> {
    inner: crate::raw::FullBucketsIndices,
    marker: core::marker::PhantomData<&'a T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2572-2575`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2572-L2575)*

An iterator producing the `usize` indices of all occupied buckets,
within the range `0..table.num_buckets()`.

The order in which the iterator yields indices is unspecified
and may change in the future.

This `struct` is created by the `HashTable::iter_buckets` method. See its
documentation for more.

#### Trait Implementations

##### `impl<T> Any for IterBuckets<'a, T>`

- <span id="iterbuckets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterBuckets<'a, T>`

- <span id="iterbuckets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterBuckets<'a, T>`

- <span id="iterbuckets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for IterBuckets<'_, T>`

- <span id="iterbuckets-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for IterBuckets<'a, T>`

- <span id="iterbuckets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for IterBuckets<'_, T>`

- <span id="iterbuckets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterBuckets<'_, T>`

- <span id="iterbuckets-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for IterBuckets<'_, T>`

- <span id="iterbuckets-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IterBuckets<'a, T>`

- <span id="iterbuckets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for IterBuckets<'_, T>`

##### `impl<T, U> Into for IterBuckets<'a, T>`

- <span id="iterbuckets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterBuckets<'a, T>`

- <span id="iterbuckets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterbuckets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterbuckets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterBuckets<'_, T>`

- <span id="iterbuckets-iterator-type-item"></span>`type Item = usize`

- <span id="iterbuckets-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="iterbuckets-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToOwned for IterBuckets<'a, T>`

- <span id="iterbuckets-toowned-type-owned"></span>`type Owned = T`

- <span id="iterbuckets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterbuckets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IterBuckets<'a, T>`

- <span id="iterbuckets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterbuckets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterBuckets<'a, T>`

- <span id="iterbuckets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterbuckets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterHash<'a, T>`

```rust
struct IterHash<'a, T> {
    inner: crate::raw::RawIterHash<T>,
    marker: core::marker::PhantomData<&'a T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2634-2637`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2634-L2637)*

An iterator over the entries of a `HashTable` that could match a given hash.
The iterator element type is `&'a T`.

This `struct` is created by the `iter_hash` method on [`HashTable`](../hash_table/index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Any for IterHash<'a, T>`

- <span id="iterhash-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterHash<'a, T>`

- <span id="iterhash-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterHash<'a, T>`

- <span id="iterhash-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for IterHash<'a, T>`

- <span id="iterhash-clone"></span>`fn clone(&self) -> IterHash<'a, T>` — [`IterHash`](../hash_table/index.md#iterhash)

##### `impl<T> CloneToUninit for IterHash<'a, T>`

- <span id="iterhash-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for IterHash<'_, T>`

- <span id="iterhash-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHash<'_, T>`

- <span id="iterhash-default"></span>`fn default() -> Self`

##### `impl<T> From for IterHash<'a, T>`

- <span id="iterhash-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for IterHash<'_, T>`

##### `impl<T, U> Into for IterHash<'a, T>`

- <span id="iterhash-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterHash<'a, T>`

- <span id="iterhash-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhash-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterhash-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterHash<'a, T>`

- <span id="iterhash-iterator-type-item"></span>`type Item = &'a T`

- <span id="iterhash-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterhash-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for IterHash<'a, T>`

- <span id="iterhash-toowned-type-owned"></span>`type Owned = T`

- <span id="iterhash-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterhash-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IterHash<'a, T>`

- <span id="iterhash-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterhash-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterHash<'a, T>`

- <span id="iterhash-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterhash-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterHashMut<'a, T>`

```rust
struct IterHashMut<'a, T> {
    inner: crate::raw::RawIterHash<T>,
    marker: core::marker::PhantomData<&'a mut T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2700-2703`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2700-L2703)*

A mutable iterator over the entries of a `HashTable` that could match a given hash.
The iterator element type is `&'a mut T`.

This `struct` is created by the `iter_hash_mut` method on [`HashTable`](../hash_table/index.md). See its
documentation for more.



#### Trait Implementations

##### `impl<T> Any for IterHashMut<'a, T>`

- <span id="iterhashmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterHashMut<'a, T>`

- <span id="iterhashmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterHashMut<'a, T>`

- <span id="iterhashmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for IterHashMut<'_, T>`

- <span id="iterhashmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashMut<'_, T>`

- <span id="iterhashmut-default"></span>`fn default() -> Self`

##### `impl<T> From for IterHashMut<'a, T>`

- <span id="iterhashmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for IterHashMut<'_, T>`

##### `impl<T, U> Into for IterHashMut<'a, T>`

- <span id="iterhashmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterHashMut<'a, T>`

- <span id="iterhashmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhashmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterhashmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterHashMut<'a, T>`

- <span id="iterhashmut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="iterhashmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterhashmut-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> TryFrom for IterHashMut<'a, T>`

- <span id="iterhashmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterhashmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterHashMut<'a, T>`

- <span id="iterhashmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterhashmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterHashBuckets<'a, T>`

```rust
struct IterHashBuckets<'a, T> {
    inner: crate::raw::RawIterHashIndices,
    marker: core::marker::PhantomData<&'a T>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2756-2759`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2756-L2759)*

An iterator producing the `usize` indices of all buckets which may match a hash.

This `struct` is created by the `HashTable::iter_hash_buckets` method. See its
documentation for more.

#### Trait Implementations

##### `impl<T> Any for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-default"></span>`fn default() -> Self`

##### `impl<T> From for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> FusedIterator for IterHashBuckets<'_, T>`

##### `impl<T, U> Into for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterhashbuckets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterhashbuckets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterHashBuckets<'_, T>`

- <span id="iterhashbuckets-iterator-type-item"></span>`type Item = usize`

- <span id="iterhashbuckets-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ToOwned for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-toowned-type-owned"></span>`type Owned = T`

- <span id="iterhashbuckets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterhashbuckets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterhashbuckets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterHashBuckets<'a, T>`

- <span id="iterhashbuckets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterhashbuckets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter<T, A>`

```rust
struct IntoIter<T, A>
where
    A: Allocator {
    inner: crate::raw::RawIntoIter<T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2808-2813`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2808-L2813)*

An owning iterator over the entries of a `HashTable` in arbitrary order.
The iterator element type is `T`.

This `struct` is created by the `into_iter` method on [`HashTable`](../hash_table/index.md)
(provided by the `IntoIterator` trait). See its documentation for more.
The table cannot be used after calling that method.




#### Trait Implementations

##### `impl<T> Any for IntoIter<T, A>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<T, A>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T, A>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, A> Debug for IntoIter<T, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> Default for IntoIter<T, A>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<T, A> ExactSizeIterator for IntoIter<T, A>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoIter<T, A>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A> FusedIterator for IntoIter<T, A>`

##### `impl<T, U> Into for IntoIter<T, A>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter<T, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A> Iterator for IntoIter<T, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> TryFrom for IntoIter<T, A>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T, A>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, T, A: Allocator>`

```rust
struct Drain<'a, T, A: Allocator> {
    inner: crate::raw::RawDrain<'a, T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2880-2882`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2880-L2882)*

A draining iterator over the items of a `HashTable`.

This `struct` is created by the `drain` method on [`HashTable`](../hash_table/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T> Any for Drain<'a, T, A>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a, T, A>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, T, A>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Drain<'a, T, A>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<T, U> Into for Drain<'a, T, A>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a, T, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> TryFrom for Drain<'a, T, A>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Drain<'a, T, A>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExtractIf<'a, T, F, A: Allocator>`

```rust
struct ExtractIf<'a, T, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, T, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/table.rs:2928-2931`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L2928-L2931)*

A draining iterator over entries of a `HashTable` which don't satisfy the predicate `f`.

This `struct` is created by `HashTable::extract_if`. See its
documentation for more.

#### Trait Implementations

##### `impl<T> Any for ExtractIf<'a, T, F, A>`

- <span id="extractif-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExtractIf<'a, T, F, A>`

- <span id="extractif-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExtractIf<'a, T, F, A>`

- <span id="extractif-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ExtractIf<'a, T, F, A>`

- <span id="extractif-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, F, A: Allocator> FusedIterator for ExtractIf<'_, T, F, A>`

##### `impl<T, U> Into for ExtractIf<'a, T, F, A>`

- <span id="extractif-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ExtractIf<'a, T, F, A>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, F, A: Allocator> Iterator for ExtractIf<'_, T, F, A>`

- <span id="extractif-iterator-type-item"></span>`type Item = T`

- <span id="extractif-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T, U> TryFrom for ExtractIf<'a, T, F, A>`

- <span id="extractif-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extractif-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ExtractIf<'a, T, F, A>`

- <span id="extractif-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extractif-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`hashbrown-0.16.1/src/table.rs:1676-1736`](../../../.source_1765633015/hashbrown-0.16.1/src/table.rs#L1676-L1736)*

A view into a single entry in a table, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashTable`](../hash_table/index.md).


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

- <span id="entry-insert"></span>`fn insert(self, value: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](../hash_table/index.md#occupiedentry)

  Sets the value of the entry, replacing any existing value if there is

  one, and returns an [`OccupiedEntry`](../hash_table/index.md).

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<&str> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  

  let entry = table

      .entry(hasher(&"horseyland"), |&x| x == "horseyland", hasher)

      .insert("horseyland");

  

  assert_eq!(entry.get(), &"horseyland");

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="entry-or-insert"></span>`fn or_insert(self, default: T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](../hash_table/index.md#occupiedentry)

  Ensures a value is in the entry by inserting if it was vacant.

  

  Returns an [`OccupiedEntry`](../hash_table/index.md) pointing to the now-occupied entry.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<&str> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  

  // nonexistent key

  table

      .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)

      .or_insert("poneyland");

  assert!(table

      .find(hasher(&"poneyland"), |&x| x == "poneyland")

      .is_some());

  

  // existing key

  table

      .entry(hasher(&"poneyland"), |&x| x == "poneyland", hasher)

      .or_insert("poneyland");

  assert!(table

      .find(hasher(&"poneyland"), |&x| x == "poneyland")

      .is_some());

  assert_eq!(table.len(), 1);

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="entry-or-insert-with"></span>`fn or_insert_with(self, default: impl FnOnce() -> T) -> OccupiedEntry<'a, T, A>` — [`OccupiedEntry`](../hash_table/index.md#occupiedentry)

  Ensures a value is in the entry by inserting the result of the default function if empty..

  

  Returns an [`OccupiedEntry`](../hash_table/index.md) pointing to the now-occupied entry.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<String> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  

  table

      .entry(hasher("poneyland"), |x| x == "poneyland", |val| hasher(val))

      .or_insert_with(|| "poneyland".to_string());

  

  assert!(table

      .find(hasher(&"poneyland"), |x| x == "poneyland")

      .is_some());

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

- <span id="entry-and-modify"></span>`fn and_modify(self, f: impl FnOnce(&mut T)) -> Self`

  Provides in-place mutable access to an occupied entry before any

  potential inserts into the table.

  

  # Examples

  

  ```rust

  #[cfg(feature = "nightly")]

  fn test() {

  use hashbrown::{HashTable, DefaultHashBuilder};

  use std::hash::BuildHasher;

  

  let mut table: HashTable<(&str, u32)> = HashTable::new();

  let hasher = DefaultHashBuilder::default();

  let hasher = |val: &_| hasher.hash_one(val);

  

  table

      .entry(

          hasher(&"poneyland"),

          |&(x, _)| x == "poneyland",

          |(k, _)| hasher(&k),

      )

      .and_modify(|(_, v)| *v += 1)

      .or_insert(("poneyland", 42));

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(k, _)| k == "poneyland"),

      Some(&("poneyland", 42))

  );

  

  table

      .entry(

          hasher(&"poneyland"),

          |&(x, _)| x == "poneyland",

          |(k, _)| hasher(&k),

      )

      .and_modify(|(_, v)| *v += 1)

      .or_insert(("poneyland", 42));

  assert_eq!(

      table.find(hasher(&"poneyland"), |&(k, _)| k == "poneyland"),

      Some(&("poneyland", 43))

  );

  }

  fn main() {

      #[cfg(feature = "nightly")]

      test()

  }

  ```

#### Trait Implementations

##### `impl<T> Any for Entry<'a, T, A>`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry<'a, T, A>`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry<'a, T, A>`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Entry<'_, T, A>`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Entry<'a, T, A>`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Entry<'a, T, A>`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Entry<'a, T, A>`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Entry<'a, T, A>`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

