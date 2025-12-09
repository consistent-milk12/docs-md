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

## Contents

- [Modules](#modules)
  - [`decompose`](#decompose)
  - [`lookups`](#lookups)
  - [`normalize`](#normalize)
  - [`perfect_hash`](#perfect_hash)
  - [`quick_check`](#quick_check)
  - [`recompose`](#recompose)
  - [`replace`](#replace)
  - [`stream_safe`](#stream_safe)
  - [`tables`](#tables)
  - [`char`](#char)
- [Structs](#structs)
  - [`Decompositions`](#decompositions)
  - [`Recompositions`](#recompositions)
  - [`Replacements`](#replacements)
  - [`StreamSafe`](#streamsafe)
- [Enums](#enums)
  - [`IsNormalized`](#isnormalized)
- [Traits](#traits)
  - [`UnicodeNormalization`](#unicodenormalization)
- [Functions](#functions)
  - [`is_nfc`](#is_nfc)
  - [`is_nfc_quick`](#is_nfc_quick)
  - [`is_nfc_stream_safe`](#is_nfc_stream_safe)
  - [`is_nfc_stream_safe_quick`](#is_nfc_stream_safe_quick)
  - [`is_nfd`](#is_nfd)
  - [`is_nfd_quick`](#is_nfd_quick)
  - [`is_nfd_stream_safe`](#is_nfd_stream_safe)
  - [`is_nfd_stream_safe_quick`](#is_nfd_stream_safe_quick)
  - [`is_nfkc`](#is_nfkc)
  - [`is_nfkc_quick`](#is_nfkc_quick)
  - [`is_nfkd`](#is_nfkd)
  - [`is_nfkd_quick`](#is_nfkd_quick)
- [Constants](#constants)
  - [`UNICODE_VERSION`](#unicode_version)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decompose`](#decompose) | mod |  |
| [`lookups`](#lookups) | mod | Lookups of unicode properties using minimal perfect hashing. |
| [`normalize`](#normalize) | mod | Functions for computing canonical and compatible decompositions for Unicode characters. |
| [`perfect_hash`](#perfect_hash) | mod | Support for lookups based on minimal perfect hashing. |
| [`quick_check`](#quick_check) | mod |  |
| [`recompose`](#recompose) | mod |  |
| [`replace`](#replace) | mod |  |
| [`stream_safe`](#stream_safe) | mod |  |
| [`tables`](#tables) | mod |  |
| [`char`](#char) | mod | Methods for composing and decomposing characters. |
| [`Decompositions`](#decompositions) | struct |  |
| [`Recompositions`](#recompositions) | struct |  |
| [`Replacements`](#replacements) | struct |  |
| [`StreamSafe`](#streamsafe) | struct |  |
| [`IsNormalized`](#isnormalized) | enum |  |
| [`UnicodeNormalization`](#unicodenormalization) | trait | Methods for iterating over strings while applying Unicode normalizations |
| [`is_nfc`](#is_nfc) | fn |  |
| [`is_nfc_quick`](#is_nfc_quick) | fn |  |
| [`is_nfc_stream_safe`](#is_nfc_stream_safe) | fn |  |
| [`is_nfc_stream_safe_quick`](#is_nfc_stream_safe_quick) | fn |  |
| [`is_nfd`](#is_nfd) | fn |  |
| [`is_nfd_quick`](#is_nfd_quick) | fn |  |
| [`is_nfd_stream_safe`](#is_nfd_stream_safe) | fn |  |
| [`is_nfd_stream_safe_quick`](#is_nfd_stream_safe_quick) | fn |  |
| [`is_nfkc`](#is_nfkc) | fn |  |
| [`is_nfkc_quick`](#is_nfkc_quick) | fn |  |
| [`is_nfkd`](#is_nfkd) | fn |  |
| [`is_nfkd_quick`](#is_nfkd_quick) | fn |  |
| [`UNICODE_VERSION`](#unicode_version) | const |  |

## Modules

- [`decompose`](decompose/index.md)
- [`lookups`](lookups/index.md) — Lookups of unicode properties using minimal perfect hashing.
- [`normalize`](normalize/index.md) — Functions for computing canonical and compatible decompositions for Unicode characters.
- [`perfect_hash`](perfect_hash/index.md) — Support for lookups based on minimal perfect hashing.
- [`quick_check`](quick_check/index.md)
- [`recompose`](recompose/index.md)
- [`replace`](replace/index.md)
- [`stream_safe`](stream_safe/index.md)
- [`tables`](tables/index.md)
- [`char`](char/index.md) — Methods for composing and decomposing characters.

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

- <span id="decompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](#decompositions)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Decompositions<I>`

- <span id="decompositions-clone"></span>`fn clone(&self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- <span id="decompositions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<I> IntoIterator for Decompositions<I>`

- <span id="decompositions-item"></span>`type Item = <I as Iterator>::Item`

- <span id="decompositions-intoiter"></span>`type IntoIter = I`

- <span id="decompositions-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- <span id="decompositions-item"></span>`type Item = char`

- <span id="decompositions-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="decompositions-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Decompositions<I>`

- <span id="decompositions-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- <span id="decompositions-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="decompositions-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="decompositions-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="decompositions-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](#replacements)

- <span id="decompositions-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

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

- <span id="recompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Self`

- <span id="recompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Recompositions<I>`

- <span id="recompositions-clone"></span>`fn clone(&self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- <span id="recompositions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<I> IntoIterator for Recompositions<I>`

- <span id="recompositions-item"></span>`type Item = <I as Iterator>::Item`

- <span id="recompositions-intoiter"></span>`type IntoIter = I`

- <span id="recompositions-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- <span id="recompositions-item"></span>`type Item = char`

- <span id="recompositions-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<T> ToString for Recompositions<I>`

- <span id="recompositions-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- <span id="recompositions-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="recompositions-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="recompositions-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="recompositions-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](#replacements)

- <span id="recompositions-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

### `Replacements<I>`

```rust
struct Replacements<I> {
    iter: I,
    buffer: Option<char>,
}
```

External iterator for replacements for a string's characters.

#### Implementations

- <span id="replacements-new-cjk-compat-variants"></span>`fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](#replacements)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Replacements<I>`

- <span id="replacements-clone"></span>`fn clone(&self) -> Replacements<I>` — [`Replacements`](#replacements)

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- <span id="replacements-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<I> IntoIterator for Replacements<I>`

- <span id="replacements-item"></span>`type Item = <I as Iterator>::Item`

- <span id="replacements-intoiter"></span>`type IntoIter = I`

- <span id="replacements-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- <span id="replacements-item"></span>`type Item = char`

- <span id="replacements-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="replacements-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToString for Replacements<I>`

- <span id="replacements-to-string"></span>`fn to_string(&self) -> String`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- <span id="replacements-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="replacements-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="replacements-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="replacements-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="replacements-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](#replacements)

- <span id="replacements-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

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

- <span id="streamsafe-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<I> IntoIterator for StreamSafe<I>`

- <span id="streamsafe-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamsafe-intoiter"></span>`type IntoIter = I`

- <span id="streamsafe-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- <span id="streamsafe-item"></span>`type Item = char`

- <span id="streamsafe-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- <span id="streamsafe-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="streamsafe-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](#decompositions)

- <span id="streamsafe-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="streamsafe-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](#recompositions)

- <span id="streamsafe-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](#replacements)

- <span id="streamsafe-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](#streamsafe)

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

- <span id="isnormalized-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsNormalized`

##### `impl PartialEq for IsNormalized`

- <span id="isnormalized-eq"></span>`fn eq(&self, other: &IsNormalized) -> bool` — [`IsNormalized`](#isnormalized)

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

- `fn nfd(self) -> Decompositions<I>`

  Returns an iterator over the string in Unicode Normalization Form D

- `fn nfkd(self) -> Decompositions<I>`

  Returns an iterator over the string in Unicode Normalization Form KD

- `fn nfc(self) -> Recompositions<I>`

  An Iterator over the string in Unicode Normalization Form C

- `fn nfkc(self) -> Recompositions<I>`

  An Iterator over the string in Unicode Normalization Form KC

- `fn cjk_compat_variants(self) -> Replacements<I>`

  A transformation which replaces [CJK Compatibility Ideograph] codepoints

- `fn stream_safe(self) -> StreamSafe<I>`

  An Iterator over the string with Conjoining Grapheme Joiner characters

#### Implementors

- [`Decompositions`](#decompositions)
- [`Recompositions`](#recompositions)
- [`Replacements`](#replacements)
- [`StreamSafe`](#streamsafe)
- `&'a str`
- `I`
- `char`

## Functions

## Constants

