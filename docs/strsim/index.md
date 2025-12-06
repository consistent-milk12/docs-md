# Crate `strsim`

This library implements string similarity metrics.

## Enums

### `StrSimError`

```rust
enum StrSimError {
    DifferentLengthArgs,
}
```

#### Trait Implementations

##### `impl Debug for StrSimError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for StrSimError`

- `fn fmt(self: &Self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Error for StrSimError`

##### `impl PartialEq for StrSimError`

- `fn eq(self: &Self, other: &StrSimError) -> bool` — [`StrSimError`](#strsimerror)

##### `impl StructuralPartialEq for StrSimError`

##### `impl<T> ToString for StrSimError`

- `fn to_string(self: &Self) -> String`

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

