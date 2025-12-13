*[hashbrown](../index.md) / [set](index.md)*

---

# Module `set`

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

*Defined in [`hashbrown-0.16.1/src/set.rs:114-116`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L114-L116)*

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

  Creates an empty `HashSet`.

  

  The hash set is initially created with a capacity of 0, so it will not allocate until it

  is first inserted into.

  

  # HashDoS resistance

  

  The `hash_builder` normally use a fixed key by default and that does

  not allow the `HashSet` to be protected against attacks such as `HashDoS`.

  Users who require HashDoS resistance should explicitly use

  `std::collections::hash_map::RandomState`

  as the hasher when creating a [`HashSet`](../hash_set/index.md), for example with

  [`with_hasher`](HashSet::with_hasher) method.

  

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  let set: HashSet<i32> = HashSet::new();

  ```

- <span id="hashset-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Creates an empty `HashSet` with the specified capacity.

  

  The hash set will be able to hold at least `capacity` elements without

  reallocating. If `capacity` is 0, the hash set will not allocate.

  

  # HashDoS resistance

  

  The `hash_builder` normally use a fixed key by default and that does

  not allow the `HashSet` to be protected against attacks such as `HashDoS`.

  Users who require HashDoS resistance should explicitly use

  `std::collections::hash_map::RandomState`

  as the hasher when creating a [`HashSet`](../hash_set/index.md), for example with

  [`with_capacity_and_hasher`](HashSet::with_capacity_and_hasher) method.

  

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  let set: HashSet<i32> = HashSet::with_capacity(10);

  assert!(set.capacity() >= 10);

  ```

#### Trait Implementations

##### `impl<T> Any for HashSet<T, S, A>`

- <span id="hashset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, S, A> BitAnd for &HashSet<T, S, A>`

- <span id="hashset-bitand-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitand"></span>`fn bitand(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](../hash_set/index.md#hashset)

  Returns the intersection of `self` and `rhs` as a new `HashSet<T, S>`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

  

  let set = &a & &b;

  

  let mut i = 0;

  let expected = [2, 3];

  for x in &set {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> BitAndAssign for HashSet<T, S, A>`

- <span id="hashset-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../hash_set/index.md#hashset)

  Modifies this set to contain the intersection of `self` and `rhs`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

  

  a &= &b;

  

  let mut i = 0;

  let expected = [2, 3];

  for x in &a {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> BitOr for &HashSet<T, S, A>`

- <span id="hashset-bitor-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitor"></span>`fn bitor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](../hash_set/index.md#hashset)

  Returns the union of `self` and `rhs` as a new `HashSet<T, S>`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  let set = &a | &b;

  

  let mut i = 0;

  let expected = [1, 2, 3, 4, 5];

  for x in &set {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> BitOrAssign for HashSet<T, S, A>`

- <span id="hashset-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../hash_set/index.md#hashset)

  Modifies this set to contain the union of `self` and `rhs`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  a |= &b;

  

  let mut i = 0;

  let expected = [1, 2, 3, 4, 5];

  for x in &a {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> BitXor for &HashSet<T, S, A>`

- <span id="hashset-bitxor-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-bitxor"></span>`fn bitxor(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](../hash_set/index.md#hashset)

  Returns the symmetric difference of `self` and `rhs` as a new `HashSet<T, S>`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  let set = &a ^ &b;

  

  let mut i = 0;

  let expected = [1, 2, 4, 5];

  for x in &set {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> BitXorAssign for HashSet<T, S, A>`

- <span id="hashset-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../hash_set/index.md#hashset)

  Modifies this set to contain the symmetric difference of `self` and `rhs`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  a ^= &b;

  

  let mut i = 0;

  let expected = [1, 2, 4, 5];

  for x in &a {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T> Borrow for HashSet<T, S, A>`

- <span id="hashset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HashSet<T, S, A>`

- <span id="hashset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone, S: Clone, A: Allocator + Clone> Clone for HashSet<T, S, A>`

- <span id="hashset-clone"></span>`fn clone(&self) -> Self`

- <span id="hashset-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T> CloneToUninit for HashSet<T, S, A>`

- <span id="hashset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S, A> Debug for HashSet<T, S, A>`

- <span id="hashset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S, A> Default for HashSet<T, S, A>`

- <span id="hashset-default"></span>`fn default() -> Self`

  Creates an empty `HashSet<T, S>` with the `Default` value for the hasher.

##### `impl<T, S, A> Eq for HashSet<T, S, A>`

##### `impl<K> Equivalent for HashSet<T, S, A>`

- <span id="hashset-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T, S, A> Extend for HashSet<T, S, A>`

- <span id="hashset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> From for HashSet<T, S, A>`

- <span id="hashset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, S, A> FromIterator for HashSet<T, S, A>`

- <span id="hashset-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T, U> Into for HashSet<T, S, A>`

- <span id="hashset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, S, A: Allocator> IntoIterator for &'a HashSet<T, S, A>`

- <span id="a-hashset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-hashset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-hashset-intoiterator-into-iter"></span>`fn into_iter(self) -> Iter<'a, T>` — [`Iter`](../hash_set/index.md#iter)

##### `impl<T, S, A> PartialEq for HashSet<T, S, A>`

- <span id="hashset-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T, S, A> Sub for &HashSet<T, S, A>`

- <span id="hashset-sub-type-output"></span>`type Output = HashSet<T, S, A>`

- <span id="hashset-sub"></span>`fn sub(self, rhs: &HashSet<T, S, A>) -> HashSet<T, S, A>` — [`HashSet`](../hash_set/index.md#hashset)

  Returns the difference of `self` and `rhs` as a new `HashSet<T, S>`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  let set = &a - &b;

  

  let mut i = 0;

  let expected = [1, 2];

  for x in &set {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T, S, A> SubAssign for HashSet<T, S, A>`

- <span id="hashset-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: &HashSet<T, S, A>)` — [`HashSet`](../hash_set/index.md#hashset)

  Modifies this set to contain the difference of `self` and `rhs`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut a: HashSet<_> = vec![1, 2, 3].into_iter().collect();

  let b: HashSet<_> = vec![3, 4, 5].into_iter().collect();

  

  a -= &b;

  

  let mut i = 0;

  let expected = [1, 2];

  for x in &a {

      assert!(expected.contains(x));

      i += 1;

  }

  assert_eq!(i, expected.len());

  ```

##### `impl<T> ToOwned for HashSet<T, S, A>`

- <span id="hashset-toowned-type-owned"></span>`type Owned = T`

- <span id="hashset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hashset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for HashSet<T, S, A>`

- <span id="hashset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hashset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for HashSet<T, S, A>`

- <span id="hashset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hashset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, K>`

```rust
struct Iter<'a, K> {
    iter: super::map::Keys<'a, K, ()>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1647-1649`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1647-L1649)*

An iterator over the items of a `HashSet`.

This `struct` is created by the [`iter`](#iter) method on [`HashSet`](../hash_set/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl Any for Iter<'a, K>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, K>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, K>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K> Clone for Iter<'_, K>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Iter<'a, K>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K: fmt::Debug> Debug for Iter<'_, K>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K> Default for Iter<'_, K>`

- <span id="iter-default"></span>`fn default() -> Self`

##### `impl<K> ExactSizeIterator for Iter<'_, K>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Iter<'a, K>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K> FusedIterator for Iter<'_, K>`

##### `impl<U> Into for Iter<'a, K>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a, K>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K> Iterator for Iter<'a, K>`

- <span id="iter-iterator-type-item"></span>`type Item = &'a K`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<&'a K>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="iter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl ToOwned for Iter<'a, K>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iter<'a, K>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a, K>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoIter<K, A: Allocator>`

```rust
struct IntoIter<K, A: Allocator> {
    iter: map::IntoIter<K, (), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1658-1660`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1658-L1660)*

An owning iterator over the items of a `HashSet`.

This `struct` is created by the `into_iter` method on [`HashSet`](../hash_set/index.md)
(provided by the `IntoIterator` trait). See its documentation for more.



#### Trait Implementations

##### `impl Any for IntoIter<K, A>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<K, A>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<K, A>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, A: Allocator> Debug for IntoIter<K, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> Default for IntoIter<K, A>`

- <span id="intoiter-default"></span>`fn default() -> Self`

##### `impl<K, A: Allocator> ExactSizeIterator for IntoIter<K, A>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for IntoIter<K, A>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, A: Allocator> FusedIterator for IntoIter<K, A>`

##### `impl<U> Into for IntoIter<K, A>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter<K, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, A: Allocator> Iterator for IntoIter<K, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = K`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<K>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for IntoIter<K, A>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter<K, A>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, K, A: Allocator>`

```rust
struct Drain<'a, K, A: Allocator> {
    iter: map::Drain<'a, K, (), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1669-1671`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1669-L1671)*

A draining iterator over the items of a `HashSet`.

This `struct` is created by the `drain` method on [`HashSet`](../hash_set/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl Any for Drain<'a, K, A>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a, K, A>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, K, A>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, A: Allocator> Debug for Drain<'_, K, A>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, A: Allocator> ExactSizeIterator for Drain<'_, K, A>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for Drain<'a, K, A>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, A: Allocator> FusedIterator for Drain<'_, K, A>`

##### `impl<U> Into for Drain<'a, K, A>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a, K, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, A: Allocator> Iterator for Drain<'_, K, A>`

- <span id="drain-iterator-type-item"></span>`type Item = K`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<K>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<U> TryFrom for Drain<'a, K, A>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drain<'a, K, A>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExtractIf<'a, K, F, A: Allocator>`

```rust
struct ExtractIf<'a, K, F, A: Allocator> {
    f: F,
    inner: crate::raw::RawExtractIf<'a, (K, ()), A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1681-1684`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1681-L1684)*

A draining iterator over entries of a `HashSet` which don't satisfy the predicate `f`.

This `struct` is created by the `extract_if` method on [`HashSet`](../hash_set/index.md). See its
documentation for more.



#### Trait Implementations

##### `impl Any for ExtractIf<'a, K, F, A>`

- <span id="extractif-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExtractIf<'a, K, F, A>`

- <span id="extractif-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExtractIf<'a, K, F, A>`

- <span id="extractif-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ExtractIf<'a, K, F, A>`

- <span id="extractif-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<K, F, A: Allocator> FusedIterator for ExtractIf<'_, K, F, A>`

##### `impl<U> Into for ExtractIf<'a, K, F, A>`

- <span id="extractif-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ExtractIf<'a, K, F, A>`

- <span id="extractif-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="extractif-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="extractif-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, F, A: Allocator> Iterator for ExtractIf<'_, K, F, A>`

- <span id="extractif-iterator-type-item"></span>`type Item = K`

- <span id="extractif-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="extractif-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for ExtractIf<'a, K, F, A>`

- <span id="extractif-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extractif-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExtractIf<'a, K, F, A>`

- <span id="extractif-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extractif-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Intersection<'a, T, S, A: Allocator>`

```rust
struct Intersection<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1693-1698`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1693-L1698)*

A lazy iterator producing elements in the intersection of `HashSet`s.

This `struct` is created by the `intersection` method on [`HashSet`](../hash_set/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T> Any for Intersection<'a, T, S, A>`

- <span id="intersection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Intersection<'a, T, S, A>`

- <span id="intersection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Intersection<'a, T, S, A>`

- <span id="intersection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S, A: Allocator> Clone for Intersection<'_, T, S, A>`

- <span id="intersection-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Intersection<'a, T, S, A>`

- <span id="intersection-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S, A> Debug for Intersection<'_, T, S, A>`

- <span id="intersection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Intersection<'a, T, S, A>`

- <span id="intersection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, S, A> FusedIterator for Intersection<'_, T, S, A>`

##### `impl<T, U> Into for Intersection<'a, T, S, A>`

- <span id="intersection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Intersection<'a, T, S, A>`

- <span id="intersection-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intersection-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intersection-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Intersection<'a, T, S, A>`

- <span id="intersection-iterator-type-item"></span>`type Item = &'a T`

- <span id="intersection-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="intersection-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intersection-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for Intersection<'a, T, S, A>`

- <span id="intersection-toowned-type-owned"></span>`type Owned = T`

- <span id="intersection-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intersection-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Intersection<'a, T, S, A>`

- <span id="intersection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intersection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Intersection<'a, T, S, A>`

- <span id="intersection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intersection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Difference<'a, T, S, A: Allocator>`

```rust
struct Difference<'a, T, S, A: Allocator> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1707-1712`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1707-L1712)*

A lazy iterator producing elements in the difference of `HashSet`s.

This `struct` is created by the `difference` method on [`HashSet`](../hash_set/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T> Any for Difference<'a, T, S, A>`

- <span id="difference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Difference<'a, T, S, A>`

- <span id="difference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Difference<'a, T, S, A>`

- <span id="difference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S, A: Allocator> Clone for Difference<'_, T, S, A>`

- <span id="difference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Difference<'a, T, S, A>`

- <span id="difference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S, A> Debug for Difference<'_, T, S, A>`

- <span id="difference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Difference<'a, T, S, A>`

- <span id="difference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, S, A> FusedIterator for Difference<'_, T, S, A>`

##### `impl<T, U> Into for Difference<'a, T, S, A>`

- <span id="difference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Difference<'a, T, S, A>`

- <span id="difference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="difference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="difference-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Difference<'a, T, S, A>`

- <span id="difference-iterator-type-item"></span>`type Item = &'a T`

- <span id="difference-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="difference-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="difference-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for Difference<'a, T, S, A>`

- <span id="difference-toowned-type-owned"></span>`type Owned = T`

- <span id="difference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="difference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Difference<'a, T, S, A>`

- <span id="difference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="difference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Difference<'a, T, S, A>`

- <span id="difference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="difference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SymmetricDifference<'a, T, S, A: Allocator>`

```rust
struct SymmetricDifference<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Difference<'a, T, S, A>, Difference<'a, T, S, A>>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1721-1723`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1721-L1723)*

A lazy iterator producing elements in the symmetric difference of `HashSet`s.

This `struct` is created by the `symmetric_difference` method on
[`HashSet`](../hash_set/index.md). See its documentation for more.



#### Trait Implementations

##### `impl<T> Any for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S, A: Allocator> Clone for SymmetricDifference<'_, T, S, A>`

- <span id="symmetricdifference-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S, A> Debug for SymmetricDifference<'_, T, S, A>`

- <span id="symmetricdifference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, S, A> FusedIterator for SymmetricDifference<'_, T, S, A>`

##### `impl<T, U> Into for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="symmetricdifference-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="symmetricdifference-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-iterator-type-item"></span>`type Item = &'a T`

- <span id="symmetricdifference-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="symmetricdifference-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="symmetricdifference-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-toowned-type-owned"></span>`type Owned = T`

- <span id="symmetricdifference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="symmetricdifference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="symmetricdifference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SymmetricDifference<'a, T, S, A>`

- <span id="symmetricdifference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="symmetricdifference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Union<'a, T, S, A: Allocator>`

```rust
struct Union<'a, T, S, A: Allocator> {
    iter: core::iter::Chain<Iter<'a, T>, Difference<'a, T, S, A>>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:1732-1734`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L1732-L1734)*

A lazy iterator producing elements in the union of `HashSet`s.

This `struct` is created by the `union` method on [`HashSet`](../hash_set/index.md).
See its documentation for more.



#### Trait Implementations

##### `impl<T> Any for Union<'a, T, S, A>`

- <span id="union-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Union<'a, T, S, A>`

- <span id="union-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Union<'a, T, S, A>`

- <span id="union-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, S, A: Allocator> Clone for Union<'_, T, S, A>`

- <span id="union-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Union<'a, T, S, A>`

- <span id="union-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T, S, A> Debug for Union<'_, T, S, A>`

- <span id="union-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Union<'a, T, S, A>`

- <span id="union-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, S, A> FusedIterator for Union<'_, T, S, A>`

##### `impl<T, U> Into for Union<'a, T, S, A>`

- <span id="union-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Union<'a, T, S, A>`

- <span id="union-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="union-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="union-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, S, A> Iterator for Union<'a, T, S, A>`

- <span id="union-iterator-type-item"></span>`type Item = &'a T`

- <span id="union-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="union-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="union-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> ToOwned for Union<'a, T, S, A>`

- <span id="union-toowned-type-owned"></span>`type Owned = T`

- <span id="union-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="union-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Union<'a, T, S, A>`

- <span id="union-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="union-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Union<'a, T, S, A>`

- <span id="union-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="union-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedEntry<'a, T, S, A: Allocator>`

```rust
struct OccupiedEntry<'a, T, S, A: Allocator> {
    inner: map::OccupiedEntry<'a, T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2301-2303`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L2301-L2303)*

A view into an occupied entry in a `HashSet`.
It is part of the [`Entry`](../hash_set/index.md) enum.

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

  Gets a reference to the value in the entry.

  

  # Examples

  

  ```rust

  use hashbrown::hash_set::{Entry, HashSet};

  

  let mut set: HashSet<&str> = HashSet::new();

  set.entry("poneyland").or_insert();

  

  match set.entry("poneyland") {

      Entry::Vacant(_) => panic!(),

      Entry::Occupied(entry) => assert_eq!(entry.get(), &"poneyland"),

  }

  ```

- <span id="occupiedentry-remove"></span>`fn remove(self) -> T`

  Takes the value out of the entry, and returns it.

  Keeps the allocated memory for reuse.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  use hashbrown::hash_set::Entry;

  

  let mut set: HashSet<&str> = HashSet::new();

  // The set is empty

  assert!(set.is_empty() && set.capacity() == 0);

  

  set.entry("poneyland").or_insert();

  let capacity_before_remove = set.capacity();

  

  if let Entry::Occupied(o) = set.entry("poneyland") {

      assert_eq!(o.remove(), "poneyland");

  }

  

  assert_eq!(set.contains("poneyland"), false);

  // Now set hold none elements but capacity is equal to the old one

  assert!(set.len() == 0 && set.capacity() == capacity_before_remove);

  ```

#### Trait Implementations

##### `impl<T> Any for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for OccupiedEntry<'_, T, S, A>`

- <span id="occupiedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OccupiedEntry<'a, T, S, A>`

- <span id="occupiedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VacantEntry<'a, T, S, A: Allocator>`

```rust
struct VacantEntry<'a, T, S, A: Allocator> {
    inner: map::VacantEntry<'a, T, (), S, A>,
}
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2339-2341`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L2339-L2341)*

A view into a vacant entry in a `HashSet`.
It is part of the [`Entry`](../hash_set/index.md) enum.

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

  Gets a reference to the value that would be used when inserting

  through the `VacantEntry`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut set: HashSet<&str> = HashSet::new();

  assert_eq!(set.entry("poneyland").get(), &"poneyland");

  ```

- <span id="vacantentry-into-value"></span>`fn into_value(self) -> T`

  Take ownership of the value.

  

  # Examples

  

  ```rust

  use hashbrown::hash_set::{Entry, HashSet};

  

  let mut set: HashSet<&str> = HashSet::new();

  

  match set.entry("poneyland") {

      Entry::Occupied(_) => panic!(),

      Entry::Vacant(v) => assert_eq!(v.into_value(), "poneyland"),

  }

  ```

- <span id="vacantentry-insert"></span>`fn insert(self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](../hash_set/index.md#occupiedentry)

  Sets the value of the entry with the `VacantEntry`'s value.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  use hashbrown::hash_set::Entry;

  

  let mut set: HashSet<&str> = HashSet::new();

  

  if let Entry::Vacant(o) = set.entry("poneyland") {

      o.insert();

  }

  assert!(set.contains("poneyland"));

  ```

#### Trait Implementations

##### `impl<T> Any for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for VacantEntry<'_, T, S, A>`

- <span id="vacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for VacantEntry<'a, T, S, A>`

- <span id="vacantentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`hashbrown-0.16.1/src/set.rs:2221-2254`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L2221-L2254)*

A view into a single entry in a set, which may either be vacant or occupied.

This `enum` is constructed from the `entry` method on [`HashSet`](../hash_set/index.md).


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

- <span id="entry-insert"></span>`fn insert(self) -> OccupiedEntry<'a, T, S, A>` — [`OccupiedEntry`](../hash_set/index.md#occupiedentry)

  Sets the value of the entry, and returns an `OccupiedEntry`.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut set: HashSet<&str> = HashSet::new();

  let entry = set.entry("horseyland").insert();

  

  assert_eq!(entry.get(), &"horseyland");

  ```

- <span id="entry-or-insert"></span>`fn or_insert(self)`

  Ensures a value is in the entry by inserting if it was vacant.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut set: HashSet<&str> = HashSet::new();

  

  // nonexistent key

  set.entry("poneyland").or_insert();

  assert!(set.contains("poneyland"));

  

  // existing key

  set.entry("poneyland").or_insert();

  assert!(set.contains("poneyland"));

  assert_eq!(set.len(), 1);

  ```

- <span id="entry-get"></span>`fn get(&self) -> &T`

  Returns a reference to this entry's value.

  

  # Examples

  

  ```rust

  use hashbrown::HashSet;

  

  let mut set: HashSet<&str> = HashSet::new();

  set.entry("poneyland").or_insert();

  // existing key

  assert_eq!(set.entry("poneyland").get(), &"poneyland");

  // nonexistent key

  assert_eq!(set.entry("horseland").get(), &"horseland");

  ```

#### Trait Implementations

##### `impl<T> Any for Entry<'a, T, S, A>`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry<'a, T, S, A>`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry<'a, T, S, A>`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, S, A: Allocator> Debug for Entry<'_, T, S, A>`

- <span id="entry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Entry<'a, T, S, A>`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Entry<'a, T, S, A>`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for Entry<'a, T, S, A>`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Entry<'a, T, S, A>`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `assert_covariance`

```rust
fn assert_covariance()
```

*Defined in [`hashbrown-0.16.1/src/set.rs:2541-2574`](../../../.source_1765521767/hashbrown-0.16.1/src/set.rs#L2541-L2574)*

