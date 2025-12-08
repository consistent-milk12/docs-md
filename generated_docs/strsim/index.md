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
  - [`generic_hamming`](#generic_hamming)
  - [`hamming`](#hamming)
  - [`generic_jaro`](#generic_jaro)
  - [`jaro`](#jaro)
  - [`generic_jaro_winkler`](#generic_jaro_winkler)
  - [`jaro_winkler`](#jaro_winkler)
  - [`generic_levenshtein`](#generic_levenshtein)
  - [`levenshtein`](#levenshtein)
  - [`normalized_levenshtein`](#normalized_levenshtein)
  - [`osa_distance`](#osa_distance)
  - [`flat_index`](#flat_index)
  - [`generic_damerau_levenshtein`](#generic_damerau_levenshtein)
  - [`damerau_levenshtein_impl`](#damerau_levenshtein_impl)
  - [`damerau_levenshtein`](#damerau_levenshtein)
  - [`normalized_damerau_levenshtein`](#normalized_damerau_levenshtein)
  - [`bigrams`](#bigrams)
  - [`sorensen_dice`](#sorensen_dice)
- [Type Aliases](#type-aliases)
  - [`HammingResult`](#hammingresult)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StringWrapper`](#stringwrapper) | struct |  |
| [`RowId`](#rowid) | struct |  |
| [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar) | struct |  |
| [`GrowingHashmapChar`](#growinghashmapchar) | struct | specialized hashmap to store user provided types |
| [`HybridGrowingHashmapChar`](#hybridgrowinghashmapchar) | struct |  |
| [`StrSimError`](#strsimerror) | enum |  |
| [`generic_hamming`](#generic_hamming) | fn | Calculates the number of positions in the two sequences where the elements |
| [`hamming`](#hamming) | fn | Calculates the number of positions in the two strings where the characters |
| [`generic_jaro`](#generic_jaro) | fn | Calculates the Jaro similarity between two sequences. |
| [`jaro`](#jaro) | fn | Calculates the Jaro similarity between two strings. |
| [`generic_jaro_winkler`](#generic_jaro_winkler) | fn | Like Jaro but gives a boost to sequences that have a common prefix. |
| [`jaro_winkler`](#jaro_winkler) | fn | Like Jaro but gives a boost to strings that have a common prefix. |
| [`generic_levenshtein`](#generic_levenshtein) | fn | Calculates the minimum number of insertions, deletions, and substitutions |
| [`levenshtein`](#levenshtein) | fn | Calculates the minimum number of insertions, deletions, and substitutions |
| [`normalized_levenshtein`](#normalized_levenshtein) | fn | Calculates a normalized score of the Levenshtein algorithm between 0.0 and |
| [`osa_distance`](#osa_distance) | fn | Like Levenshtein but allows for adjacent transpositions. |
| [`flat_index`](#flat_index) | fn |  |
| [`generic_damerau_levenshtein`](#generic_damerau_levenshtein) | fn | Like optimal string alignment, but substrings can be edited an unlimited |
| [`damerau_levenshtein_impl`](#damerau_levenshtein_impl) | fn |  |
| [`damerau_levenshtein`](#damerau_levenshtein) | fn | Like optimal string alignment, but substrings can be edited an unlimited |
| [`normalized_damerau_levenshtein`](#normalized_damerau_levenshtein) | fn | Calculates a normalized score of the Damerau–Levenshtein algorithm between |
| [`bigrams`](#bigrams) | fn | Returns an Iterator of char tuples. |
| [`sorensen_dice`](#sorensen_dice) | fn | Calculates a Sørensen-Dice similarity distance using bigrams. |
| [`HammingResult`](#hammingresult) | type |  |

## Structs

### `StringWrapper<'a>`

```rust
struct StringWrapper<'a>(&'a str);
```

### `RowId`

```rust
struct RowId {
    val: isize,
}
```

#### Trait Implementations

##### `impl Clone for RowId`

- <span id="rowid-clone"></span>`fn clone(&self) -> RowId` — [`RowId`](#rowid)

##### `impl Copy for RowId`

##### `impl Default for RowId`

- <span id="rowid-default"></span>`fn default() -> Self`

##### `impl Eq for RowId`

##### `impl PartialEq for RowId`

- <span id="rowid-eq"></span>`fn eq(&self, other: &RowId) -> bool` — [`RowId`](#rowid)

##### `impl StructuralPartialEq for RowId`

### `GrowingHashmapMapElemChar<ValueType>`

```rust
struct GrowingHashmapMapElemChar<ValueType> {
    key: u32,
    value: ValueType,
}
```

#### Trait Implementations

##### `impl<ValueType: clone::Clone> Clone for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-clone"></span>`fn clone(&self) -> GrowingHashmapMapElemChar<ValueType>` — [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)

##### `impl<ValueType: default::Default> Default for GrowingHashmapMapElemChar<ValueType>`

- <span id="growinghashmapmapelemchar-default"></span>`fn default() -> GrowingHashmapMapElemChar<ValueType>` — [`GrowingHashmapMapElemChar`](#growinghashmapmapelemchar)

### `GrowingHashmapChar<ValueType>`

```rust
struct GrowingHashmapChar<ValueType> {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
}
```

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

- <span id="growinghashmapchar-grow"></span>`fn grow(&mut self, min_used: i32)`

#### Trait Implementations

##### `impl<ValueType> Default for GrowingHashmapChar<ValueType>`

- <span id="growinghashmapchar-default"></span>`fn default() -> Self`

### `HybridGrowingHashmapChar<ValueType>`

```rust
struct HybridGrowingHashmapChar<ValueType> {
    map: GrowingHashmapChar<ValueType>,
    extended_ascii: [ValueType; 256],
}
```

#### Implementations

- <span id="hybridgrowinghashmapchar-get"></span>`fn get(&self, key: char) -> ValueType`

- <span id="hybridgrowinghashmapchar-get-mut"></span>`fn get_mut(&mut self, key: char) -> &mut ValueType`

#### Trait Implementations

##### `impl<ValueType> Default for HybridGrowingHashmapChar<ValueType>`

- <span id="hybridgrowinghashmapchar-default"></span>`fn default() -> Self`

## Enums

### `StrSimError`

```rust
enum StrSimError {
    DifferentLengthArgs,
}
```

#### Trait Implementations

##### `impl Debug for StrSimError`

- <span id="strsimerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StrSimError`

- <span id="strsimerror-fmt"></span>`fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Error for StrSimError`

##### `impl PartialEq for StrSimError`

- <span id="strsimerror-eq"></span>`fn eq(&self, other: &StrSimError) -> bool` — [`StrSimError`](#strsimerror)

##### `impl StructuralPartialEq for StrSimError`

##### `impl<T> ToString for StrSimError`

- <span id="strsimerror-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `generic_hamming`

```rust
fn generic_hamming<Iter1, Iter2, Elem1, Elem2>(a: Iter1, b: Iter2) -> HammingResult
where
    Iter1: IntoIterator<Item = Elem1>,
    Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>
```

Calculates the number of positions in the two sequences where the elements
differ. Returns an error if the sequences have different lengths.

### `hamming`

```rust
fn hamming(a: &str, b: &str) -> HammingResult
```

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

Calculates the Jaro similarity between two sequences. The returned value
is between 0.0 and 1.0 (higher value means more similar).

### `jaro`

```rust
fn jaro(a: &str, b: &str) -> f64
```

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

Like Jaro but gives a boost to sequences that have a common prefix.

### `jaro_winkler`

```rust
fn jaro_winkler(a: &str, b: &str) -> f64
```

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

### `generic_damerau_levenshtein`

```rust
fn generic_damerau_levenshtein<Elem>(a_elems: &[Elem], b_elems: &[Elem]) -> usize
where
    Elem: Eq + Hash + Clone
```

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

### `damerau_levenshtein`

```rust
fn damerau_levenshtein(a: &str, b: &str) -> usize
```

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

Returns an Iterator of char tuples.

### `sorensen_dice`

```rust
fn sorensen_dice(a: &str, b: &str) -> f64
```

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

