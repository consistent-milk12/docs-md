# Crate `unicode_normalization`

Unicode character composition and decomposition utilities
as described in
[Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).

```rust
extern crate unicode_normalization;

use unicode_normalization::char::compose;
use unicode_normalization::UnicodeNormalization;

fn main() {
    assert_eq!(compose('A','\u{30a}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");
}
```

# crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
unicode-normalization = "0.1.20"
```

## Modules

- [`decompose`](decompose/index.md) - 
- [`lookups`](lookups/index.md) - Lookups of unicode properties using minimal perfect hashing.
- [`normalize`](normalize/index.md) - Functions for computing canonical and compatible decompositions for Unicode characters.
- [`perfect_hash`](perfect_hash/index.md) - Support for lookups based on minimal perfect hashing.
- [`quick_check`](quick_check/index.md) - 
- [`recompose`](recompose/index.md) - 
- [`replace`](replace/index.md) - 
- [`stream_safe`](stream_safe/index.md) - 
- [`tables`](tables/index.md) - 
- [`char`](char/index.md) - Methods for composing and decomposing characters.

## Structs

### `Decompositions<I>`

```rust
struct Decompositions<I> {
    kind: DecompositionType,
    iter: core::iter::Fuse<I>,
    buffer: tinyvec::TinyVec<[(u8, char); 4]>,
    ready: core::ops::Range<usize>,
}
```

External iterator for a string decomposition's characters.

#### Implementations

- `fn push_back(self: &mut Self, ch: char)`

- `fn sort_pending(self: &mut Self)`

- `fn reset_buffer(self: &mut Self)`

- `fn increment_next_ready(self: &mut Self)`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Decompositions<I>`

- `fn clone(self: &Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<I> IntoIterator for Decompositions<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Decompositions<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md)

### `Recompositions<I>`

```rust
struct Recompositions<I> {
    iter: crate::decompose::Decompositions<I>,
    state: RecompositionState,
    buffer: tinyvec::TinyVec<[char; 4]>,
    composee: Option<char>,
    last_ccc: Option<u8>,
}
```

External iterator for a string recomposition's characters.

#### Implementations

- `fn new_canonical(iter: I) -> Self`

- `fn new_compatible(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Recompositions<I>`

- `fn clone(self: &Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<I> IntoIterator for Recompositions<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl<T> ToString for Recompositions<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md)

### `Replacements<I>`

```rust
struct Replacements<I> {
    iter: I,
    buffer: Option<char>,
}
```

External iterator for replacements for a string's characters.

#### Implementations

- `fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](replace/index.md)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Replacements<I>`

- `fn clone(self: &Self) -> Replacements<I>` — [`Replacements`](replace/index.md)

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<I> IntoIterator for Replacements<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Replacements<I>`

- `fn to_string(self: &Self) -> String`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md)

### `StreamSafe<I>`

```rust
struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}
```

[UAX15-D4]: This iterator keeps track of how many non-starters there have been
since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.


#### Implementations

- `fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<I> IntoIterator for StreamSafe<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md)

## Enums

### `IsNormalized`

```rust
enum IsNormalized {
    Yes,
    No,
    Maybe,
}
```

QuickCheck quickly determines if a string is normalized, it can return
`Maybe`

The QuickCheck algorithm can quickly determine if a text is or isn't
normalized without any allocations in many cases, but it has to be able to
return `Maybe` when a full decomposition and recomposition is necessary.

#### Variants

- **`Yes`**

  The text is definitely normalized.

- **`No`**

  The text is definitely not normalized.

- **`Maybe`**

  The text may be normalized.

#### Trait Implementations

##### `impl Debug for IsNormalized`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for IsNormalized`

##### `impl PartialEq for IsNormalized`

- `fn eq(self: &Self, other: &IsNormalized) -> bool` — [`IsNormalized`](quick_check/index.md)

##### `impl StructuralPartialEq for IsNormalized`

## Traits

### `UnicodeNormalization<I: Iterator<Item = char>>`

```rust
trait UnicodeNormalization<I: Iterator<Item = char>> { ... }
```

Methods for iterating over strings while applying Unicode normalizations
as described in
[Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/).

#### Required Methods

- `fn nfd(self: Self) -> Decompositions<I>`

  Returns an iterator over the string in Unicode Normalization Form D

- `fn nfkd(self: Self) -> Decompositions<I>`

  Returns an iterator over the string in Unicode Normalization Form KD

- `fn nfc(self: Self) -> Recompositions<I>`

  An Iterator over the string in Unicode Normalization Form C

- `fn nfkc(self: Self) -> Recompositions<I>`

  An Iterator over the string in Unicode Normalization Form KC

- `fn cjk_compat_variants(self: Self) -> Replacements<I>`

  A transformation which replaces [CJK Compatibility Ideograph] codepoints

- `fn stream_safe(self: Self) -> StreamSafe<I>`

  An Iterator over the string with Conjoining Grapheme Joiner characters

## Functions

## Constants

