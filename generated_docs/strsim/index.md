# Crate `strsim`

This library implements string similarity metrics.

## Contents

- [Structs](#structs)
  - [`StringWrapper`](#stringwrapper)
  - [`RowId`](#rowid)
  - [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)
  - [`GrowingHashmapChar`](#growinghashmapchar)
  - [`HybridGrowingHashmapChar`](#hybridgrowinghashmapchar)
- [Enums](#enums)
  - [`StrSimError`](#strsimerror)
- [Functions](#functions)
  - [`generic_hamming`](#generic-hamming)
  - [`hamming`](#hamming)
  - [`generic_jaro`](#generic-jaro)
  - [`jaro`](#jaro)
  - [`generic_jaro_winkler`](#generic-jaro-winkler)
  - [`jaro_winkler`](#jaro-winkler)
  - [`generic_levenshtein`](#generic-levenshtein)
  - [`levenshtein`](#levenshtein)
  - [`normalized_levenshtein`](#normalized-levenshtein)
  - [`osa_distance`](#osa-distance)
  - [`flat_index`](#flat-index)
  - [`generic_damerau_levenshtein`](#generic-damerau-levenshtein)
  - [`damerau_levenshtein_impl`](#damerau-levenshtein-impl)
  - [`damerau_levenshtein`](#damerau-levenshtein)
  - [`normalized_damerau_levenshtein`](#normalized-damerau-levenshtein)
  - [`bigrams`](#bigrams)
  - [`sorensen_dice`](#sorensen-dice)
- [Type Aliases](#type-aliases)
  - [`HammingResult`](#hammingresult)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StringWrapper`](#stringwrapper) | struct |  |
| [`RowId`](#rowid) | struct |  |
| [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar) | struct |  |
| [`GrowingHashmapChar`](#growinghashmapchar) | struct | specialized hashmap to store user provided types this implementation relies on a couple of base assumptions in order to simplify the implementation - the hashmap does not have an upper limit of included items - the default value for the `ValueType` can be used as a dummy value to indicate an empty cell - elements can't be removed - only allocates memory on first write access. |
| [`HybridGrowingHashmapChar`](#hybridgrowinghashmapchar) | struct |  |
| [`StrSimError`](#strsimerror) | enum |  |
| [`generic_hamming`](#generic-hamming) | fn | Calculates the number of positions in the two sequences where the elements differ. |
| [`hamming`](#hamming) | fn | Calculates the number of positions in the two strings where the characters differ. |
| [`generic_jaro`](#generic-jaro) | fn | Calculates the Jaro similarity between two sequences. |
| [`jaro`](#jaro) | fn | Calculates the Jaro similarity between two strings. |
| [`generic_jaro_winkler`](#generic-jaro-winkler) | fn | Like Jaro but gives a boost to sequences that have a common prefix. |
| [`jaro_winkler`](#jaro-winkler) | fn | Like Jaro but gives a boost to strings that have a common prefix. |
| [`generic_levenshtein`](#generic-levenshtein) | fn | Calculates the minimum number of insertions, deletions, and substitutions required to change one sequence into the other. |
| [`levenshtein`](#levenshtein) | fn | Calculates the minimum number of insertions, deletions, and substitutions required to change one string into the other. |
| [`normalized_levenshtein`](#normalized-levenshtein) | fn | Calculates a normalized score of the Levenshtein algorithm between 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same. |
| [`osa_distance`](#osa-distance) | fn | Like Levenshtein but allows for adjacent transpositions. |
| [`flat_index`](#flat-index) | fn |  |
| [`generic_damerau_levenshtein`](#generic-damerau-levenshtein) | fn | Like optimal string alignment, but substrings can be edited an unlimited number of times, and the triangle inequality holds. |
| [`damerau_levenshtein_impl`](#damerau-levenshtein-impl) | fn |  |
| [`damerau_levenshtein`](#damerau-levenshtein) | fn | Like optimal string alignment, but substrings can be edited an unlimited number of times, and the triangle inequality holds. |
| [`normalized_damerau_levenshtein`](#normalized-damerau-levenshtein) | fn | Calculates a normalized score of the Damerau–Levenshtein algorithm between 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same. |
| [`bigrams`](#bigrams) | fn | Returns an Iterator of char tuples. |
| [`sorensen_dice`](#sorensen-dice) | fn | Calculates a Sørensen-Dice similarity distance using bigrams. |
| [`HammingResult`](#hammingresult) | type |  |

## Structs

### `StringWrapper<'a>`

```rust
struct StringWrapper<'a>(&'a str);
```

*Defined in [`strsim-0.11.1/src/lib.rs:166`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L166)*

#### Trait Implementations

##### `impl Any for StringWrapper<'a>`

- <span id="stringwrapper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringWrapper<'a>`

- <span id="stringwrapper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringWrapper<'a>`

- <span id="stringwrapper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StringWrapper<'a>`

- <span id="stringwrapper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringWrapper<'a>`

- <span id="stringwrapper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for &'a StringWrapper<'b>`

- <span id="a-stringwrapper-intoiterator-type-item"></span>`type Item = char`

- <span id="a-stringwrapper-intoiterator-type-intoiter"></span>`type IntoIter = Chars<'b>`

- <span id="a-stringwrapper-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<U> TryFrom for StringWrapper<'a>`

- <span id="stringwrapper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringwrapper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringWrapper<'a>`

- <span id="stringwrapper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringwrapper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RowId`

```rust
struct RowId {
    val: isize,
}
```

*Defined in [`strsim-0.11.1/src/lib.rs:417-419`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L417-L419)*

#### Trait Implementations

##### `impl Any for RowId`

- <span id="rowid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RowId`

- <span id="rowid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RowId`

- <span id="rowid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RowId`

- <span id="rowid-clone"></span>`fn clone(&self) -> RowId` — [`RowId`](#rowid)

##### `impl CloneToUninit for RowId`

- <span id="rowid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RowId`

##### `impl Default for RowId`

- <span id="rowid-default"></span>`fn default() -> Self`

##### `impl Eq for RowId`

##### `impl<T> From for RowId`

- <span id="rowid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RowId`

- <span id="rowid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RowId`

- <span id="rowid-partialeq-eq"></span>`fn eq(&self, other: &RowId) -> bool` — [`RowId`](#rowid)

##### `impl StructuralPartialEq for RowId`

##### `impl ToOwned for RowId`

- <span id="rowid-toowned-type-owned"></span>`type Owned = T`

- <span id="rowid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rowid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RowId`

- <span id="rowid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rowid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RowId`

- <span id="rowid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rowid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GrowingHashmapMapElemChar<ValueType>`

```rust
struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}
```

*Defined in [`strsim-0.11.1/src/lib.rs:428-431`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L428-L431)*

#### Trait Implementations

##### `impl Any for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<ValueType: clone::Clone> Clone for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-clone"></span>`fn clone(&self) -> GrowingHashmapMapElemChar<ValueType>` — [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)

##### `impl CloneToUninit for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<ValueType: default::Default> Default for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-default"></span>`fn default() -> GrowingHashmapMapElemChar<ValueType>` — [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)

##### `impl<T> From for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-toowned-type-owned"></span>`type Owned = T`

- <span id="growinghashmapmapelemchar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="growinghashmapmapelemchar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="growinghashmapmapelemchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="growinghashmapmapelemchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GrowingHashmapChar<ValueType>`

```rust
struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}
```

*Defined in [`strsim-0.11.1/src/lib.rs:440-445`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L440-L445)*

specialized hashmap to store user provided types
this implementation relies on a couple of base assumptions in order to simplify the implementation
- the hashmap does not have an upper limit of included items
- the default value for the `ValueType` can be used as a dummy value to indicate an empty cell
- elements can't be removed
- only allocates memory on first write access.
  This improves performance for hashmaps that are never written to

#### Implementations

- <span id="growinghashmapchar-get"></span>`fn get(&self, key: u32) -> ValueType`

- <span id="growinghashmapchar-get-mut"></span>`fn get_mut(&mut self, key: u32) -> &mut ValueType`

- <span id="growinghashmapchar-allocate"></span>`fn allocate(&mut self)`

- <span id="growinghashmapchar-lookup"></span>`fn lookup(&self, key: u32) -> usize`

  lookup key inside the hashmap using a similar collision resolution

  strategy to `CPython` and `Ruby`

- <span id="growinghashmapchar-grow"></span>`fn grow(&mut self, min_used: i32)`

#### Trait Implementations

##### `impl Any for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<ValueType> Default for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-default"></span>`fn default() -> Self`

##### `impl<T> From for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="growinghashmapchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="growinghashmapchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HybridGrowingHashmapChar<ValueType>`

```rust
struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}
```

*Defined in [`strsim-0.11.1/src/lib.rs:567-570`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L567-L570)*

#### Implementations

- <span id="hybridgrowinghashmapchar-get"></span>`fn get(&self, key: char) -> ValueType`

- <span id="hybridgrowinghashmapchar-get-mut"></span>`fn get_mut(&mut self, key: char) -> &mut ValueType`

#### Trait Implementations

##### `impl Any for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<ValueType> Default for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-default"></span>`fn default() -> Self`

##### `impl<T> From for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hybridgrowinghashmapchar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hybridgrowinghashmapchar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `StrSimError`

```rust
enum StrSimError {
    DifferentLengthArgs,
}
```

*Defined in [`strsim-0.11.1/src/lib.rs:33-35`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L33-L35)*

#### Trait Implementations

##### `impl Any for StrSimError`

- <span id="strsimerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrSimError`

- <span id="strsimerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrSimError`

- <span id="strsimerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for StrSimError`

- <span id="strsimerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StrSimError`

- <span id="strsimerror-display-fmt"></span>`fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Error for StrSimError`

##### `impl<T> From for StrSimError`

- <span id="strsimerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StrSimError`

- <span id="strsimerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StrSimError`

- <span id="strsimerror-partialeq-eq"></span>`fn eq(&self, other: &StrSimError) -> bool` — [`StrSimError`](#strsimerror)

##### `impl StructuralPartialEq for StrSimError`

##### `impl ToString for StrSimError`

- <span id="strsimerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StrSimError`

- <span id="strsimerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strsimerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrSimError`

- <span id="strsimerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strsimerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `generic_hamming`

```rust
fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>
```

*Defined in [`strsim-0.11.1/src/lib.rs:53-72`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L53-L72)*

Calculates the number of positions in the two sequences where the elements
differ. Returns an error if the sequences have different lengths.

### `hamming`

```rust
fn hamming(a: &str, b: &str) -> HammingResult
```

*Defined in [`strsim-0.11.1/src/lib.rs:84-86`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L84-L86)*

Calculates the number of positions in the two strings where the characters
differ. Returns an error if the strings have different lengths.

```rust
use strsim::{hamming, StrSimError::DifferentLengthArgs};

assert_eq!(Ok(3), hamming("hamming", "hammers"));

assert_eq!(Err(DifferentLengthArgs), hamming("hamming", "ham"));
```

### `generic_jaro`

```rust
fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>
```

*Defined in [`strsim-0.11.1/src/lib.rs:90-164`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L90-L164)*

Calculates the Jaro similarity between two sequences. The returned value
is between 0.0 and 1.0 (higher value means more similar).

### `jaro`

```rust
fn jaro(a: &str, b: &str) -> f64
```

*Defined in [`strsim-0.11.1/src/lib.rs:186-188`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L186-L188)*

Calculates the Jaro similarity between two strings. The returned value
is between 0.0 and 1.0 (higher value means more similar).

```rust
use strsim::jaro;

assert!((0.392 - jaro("Friedrich Nietzsche", "Jean-Paul Sartre")).abs() <
        0.001);
```

### `generic_jaro_winkler`

```rust
fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>
```

*Defined in [`strsim-0.11.1/src/lib.rs:191-211`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L191-L211)*

Like Jaro but gives a boost to sequences that have a common prefix.

### `jaro_winkler`

```rust
fn jaro_winkler(a: &str, b: &str) -> f64
```

*Defined in [`strsim-0.11.1/src/lib.rs:221-223`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L221-L223)*

Like Jaro but gives a boost to strings that have a common prefix.

```rust
use strsim::jaro_winkler;

assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
        0.001);
```

### `generic_levenshtein`

```rust
fn generic_levenshtein<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> usize
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>
```

*Defined in [`strsim-0.11.1/src/lib.rs:233-259`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L233-L259)*

Calculates the minimum number of insertions, deletions, and substitutions
required to change one sequence into the other.

```rust
use strsim::generic_levenshtein;

assert_eq!(3, generic_levenshtein(&[1,2,3], &[1,2,3,4,5,6]));
```

### `levenshtein`

```rust
fn levenshtein(a: &str, b: &str) -> usize
```

*Defined in [`strsim-0.11.1/src/lib.rs:269-271`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L269-L271)*

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```rust
use strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```

### `normalized_levenshtein`

```rust
fn normalized_levenshtein(a: &str, b: &str) -> f64
```

*Defined in [`strsim-0.11.1/src/lib.rs:285-290`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L285-L290)*

Calculates a normalized score of the Levenshtein algorithm between 0.0 and
1.0 (inclusive), where 1.0 means the strings are the same.

```rust
use strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);
assert!((normalized_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_levenshtein("", "second").abs() < 0.00001);
assert!(normalized_levenshtein("first", "").abs() < 0.00001);
assert!((normalized_levenshtein("string", "string") - 1.0).abs() < 0.00001);
```

### `osa_distance`

```rust
fn osa_distance(a: &str, b: &str) -> usize
```

*Defined in [`strsim-0.11.1/src/lib.rs:300-337`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L300-L337)*

Like Levenshtein but allows for adjacent transpositions. Each substring can
only be edited once.

```rust
use strsim::osa_distance;

assert_eq!(3, osa_distance("ab", "bca"));
```

### `flat_index`

```rust
fn flat_index(i: usize, j: usize, width: usize) -> usize
```

*Defined in [`strsim-0.11.1/src/lib.rs:341-343`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L341-L343)*

### `generic_damerau_levenshtein`

```rust
fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone
```

*Defined in [`strsim-0.11.1/src/lib.rs:353-414`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L353-L414)*

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```rust
use strsim::generic_damerau_levenshtein;

assert_eq!(2, generic_damerau_levenshtein(&[1,2], &[2,3,1]));
```

### `damerau_levenshtein_impl`

```rust
fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
where
    Iter1: Iterator<Item = char> + Clone,
    Iter2: Iterator<Item = char> + Clone
```

*Defined in [`strsim-0.11.1/src/lib.rs:609-667`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L609-L667)*

### `damerau_levenshtein`

```rust
fn damerau_levenshtein(a: &str, b: &str) -> usize
```

*Defined in [`strsim-0.11.1/src/lib.rs:677-679`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L677-L679)*

Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

```rust
use strsim::damerau_levenshtein;

assert_eq!(2, damerau_levenshtein("ab", "bca"));
```

### `normalized_damerau_levenshtein`

```rust
fn normalized_damerau_levenshtein(a: &str, b: &str) -> f64
```

*Defined in [`strsim-0.11.1/src/lib.rs:693-702`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L693-L702)*

Calculates a normalized score of the Damerau–Levenshtein algorithm between
0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.

```rust
use strsim::normalized_damerau_levenshtein;

assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.27272).abs() < 0.00001);
assert!((normalized_damerau_levenshtein("", "") - 1.0).abs() < 0.00001);
assert!(normalized_damerau_levenshtein("", "flower").abs() < 0.00001);
assert!(normalized_damerau_levenshtein("tree", "").abs() < 0.00001);
assert!((normalized_damerau_levenshtein("sunglasses", "sunglasses") - 1.0).abs() < 0.00001);
```

### `bigrams`

```rust
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_
```

*Defined in [`strsim-0.11.1/src/lib.rs:705-707`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L705-L707)*

Returns an Iterator of char tuples.

### `sorensen_dice`

```rust
fn sorensen_dice(a: &str, b: &str) -> f64
```

*Defined in [`strsim-0.11.1/src/lib.rs:721-754`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L721-L754)*

Calculates a Sørensen-Dice similarity distance using bigrams.
See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.

```rust
use strsim::sorensen_dice;

assert_eq!(1.0, sorensen_dice("", ""));
assert_eq!(0.0, sorensen_dice("", "a"));
assert_eq!(0.0, sorensen_dice("french", "quebec"));
assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
```

## Type Aliases

### `HammingResult`

```rust
type HammingResult = Result<usize, StrSimError>;
```

*Defined in [`strsim-0.11.1/src/lib.rs:49`](../../.source_1765521767/strsim-0.11.1/src/lib.rs#L49)*

