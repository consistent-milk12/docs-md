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

- `fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

#### Trait Implementations

##### `impl Clone<I: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

##### `impl Display<I: Iterator<Item = char> + Clone>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator<I: Iterator<Item = char> + FusedIterator>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<I: Iterator<Item = char>>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl UnicodeNormalization<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md)

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

##### `impl Clone<I: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

##### `impl Display<I: Iterator<Item = char> + Clone>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator<I: Iterator<Item = char> + FusedIterator>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<I: Iterator<Item = char>>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl UnicodeNormalization<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md)

### `Replacements<I>`

```rust
struct Replacements<I> {
    iter: I,
    buffer: Option<char>,
}
```

External iterator for replacements for a string's characters.

#### Implementations

- `fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](../replace/index.md)

#### Trait Implementations

##### `impl Clone<I: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Replacements<I>` — [`Replacements`](../replace/index.md)

##### `impl Display<I: Iterator<Item = char> + Clone>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator<I: Iterator<Item = char> + FusedIterator>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<I: Iterator<Item = char>>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl UnicodeNormalization<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md)

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

[UAX15-D4]: https://www.unicode.org/reports/tr15/#UAX15-D4

#### Implementations

- `fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl FusedIterator<I: Iterator<Item = char> + FusedIterator>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<I: Iterator<Item = char>>`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

##### `impl UnicodeNormalization<I>`

- `fn nfd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfkd(self: Self) -> Decompositions<I>` — [`Decompositions`](../decompose/index.md)

- `fn nfc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn nfkc(self: Self) -> Recompositions<I>` — [`Recompositions`](../recompose/index.md)

- `fn cjk_compat_variants(self: Self) -> Replacements<I>` — [`Replacements`](../replace/index.md)

- `fn stream_safe(self: Self) -> StreamSafe<I>` — [`StreamSafe`](../stream_safe/index.md)

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &IsNormalized) -> bool` — [`IsNormalized`](../quick_check/index.md)

##### `impl StructuralPartialEq`

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

