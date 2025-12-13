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
  - [`perfect_hash`](#perfect-hash)
  - [`quick_check`](#quick-check)
  - [`recompose`](#recompose)
  - [`replace`](#replace)
  - [`stream_safe`](#stream-safe)
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
  - [`is_nfc`](#is-nfc)
  - [`is_nfc_quick`](#is-nfc-quick)
  - [`is_nfc_stream_safe`](#is-nfc-stream-safe)
  - [`is_nfc_stream_safe_quick`](#is-nfc-stream-safe-quick)
  - [`is_nfd`](#is-nfd)
  - [`is_nfd_quick`](#is-nfd-quick)
  - [`is_nfd_stream_safe`](#is-nfd-stream-safe)
  - [`is_nfd_stream_safe_quick`](#is-nfd-stream-safe-quick)
  - [`is_nfkc`](#is-nfkc)
  - [`is_nfkc_quick`](#is-nfkc-quick)
  - [`is_nfkd`](#is-nfkd)
  - [`is_nfkd_quick`](#is-nfkd-quick)
- [Constants](#constants)
  - [`UNICODE_VERSION`](#unicode-version)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`decompose`](#decompose) | mod |  |
| [`lookups`](#lookups) | mod | Lookups of unicode properties using minimal perfect hashing. |
| [`normalize`](#normalize) | mod | Functions for computing canonical and compatible decompositions for Unicode characters. |
| [`perfect_hash`](#perfect-hash) | mod | Support for lookups based on minimal perfect hashing. |
| [`quick_check`](#quick-check) | mod |  |
| [`recompose`](#recompose) | mod |  |
| [`replace`](#replace) | mod |  |
| [`stream_safe`](#stream-safe) | mod |  |
| [`tables`](#tables) | mod |  |
| [`char`](#char) | mod | Methods for composing and decomposing characters. |
| [`Decompositions`](#decompositions) | struct |  |
| [`Recompositions`](#recompositions) | struct |  |
| [`Replacements`](#replacements) | struct |  |
| [`StreamSafe`](#streamsafe) | struct |  |
| [`IsNormalized`](#isnormalized) | enum |  |
| [`UnicodeNormalization`](#unicodenormalization) | trait | Methods for iterating over strings while applying Unicode normalizations as described in [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/). |
| [`is_nfc`](#is-nfc) | fn |  |
| [`is_nfc_quick`](#is-nfc-quick) | fn |  |
| [`is_nfc_stream_safe`](#is-nfc-stream-safe) | fn |  |
| [`is_nfc_stream_safe_quick`](#is-nfc-stream-safe-quick) | fn |  |
| [`is_nfd`](#is-nfd) | fn |  |
| [`is_nfd_quick`](#is-nfd-quick) | fn |  |
| [`is_nfd_stream_safe`](#is-nfd-stream-safe) | fn |  |
| [`is_nfd_stream_safe_quick`](#is-nfd-stream-safe-quick) | fn |  |
| [`is_nfkc`](#is-nfkc) | fn |  |
| [`is_nfkc_quick`](#is-nfkc-quick) | fn |  |
| [`is_nfkd`](#is-nfkd) | fn |  |
| [`is_nfkd_quick`](#is-nfkd-quick) | fn |  |
| [`UNICODE_VERSION`](#unicode-version) | const |  |

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

*Defined in [`unicode-normalization-0.1.25/src/decompose.rs:23-37`](../../.source_1765521767/unicode-normalization-0.1.25/src/decompose.rs#L23-L37)*

External iterator for a string decomposition's characters.

#### Implementations

- <span id="decompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

  Create a new decomposition iterator for canonical decompositions (NFD)

  

  Note that this iterator can also be obtained by directly calling [`.nfd()`](crate::UnicodeNormalization::nfd)

  on the iterator.

- <span id="decompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

  Create a new decomposition iterator for compatability decompositions (NFkD)

  

  Note that this iterator can also be obtained by directly calling [`.nfkd()`](crate::UnicodeNormalization::nfkd)

  on the iterator.

#### Trait Implementations

##### `impl Any for Decompositions<I>`

- <span id="decompositions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Decompositions<I>`

- <span id="decompositions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Decompositions<I>`

- <span id="decompositions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Decompositions<I>`

- <span id="decompositions-clone"></span>`fn clone(&self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

##### `impl CloneToUninit for Decompositions<I>`

- <span id="decompositions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Decompositions<I>`

- <span id="decompositions-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Decompositions<I>`

- <span id="decompositions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Decompositions<I>`

##### `impl<U> Into for Decompositions<I>`

- <span id="decompositions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Decompositions<I>`

- <span id="decompositions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="decompositions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="decompositions-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Decompositions<I>`

- <span id="decompositions-iterator-type-item"></span>`type Item = char`

- <span id="decompositions-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="decompositions-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Decompositions<I>`

- <span id="decompositions-toowned-type-owned"></span>`type Owned = T`

- <span id="decompositions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="decompositions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Decompositions<I>`

- <span id="decompositions-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Decompositions<I>`

- <span id="decompositions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decompositions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Decompositions<I>`

- <span id="decompositions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decompositions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Decompositions<I>`

- <span id="decompositions-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="decompositions-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="decompositions-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="decompositions-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="decompositions-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

- <span id="decompositions-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md#streamsafe)

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

*Defined in [`unicode-normalization-0.1.25/src/recompose.rs:27-33`](../../.source_1765521767/unicode-normalization-0.1.25/src/recompose.rs#L27-L33)*

External iterator for a string recomposition's characters.

#### Implementations

- <span id="recompositions-new-canonical"></span>`fn new_canonical(iter: I) -> Self`

  Create a new recomposition iterator for canonical compositions (NFC)

  

  Note that this iterator can also be obtained by directly calling [`.nfc()`](crate::UnicodeNormalization::nfc)

  on the iterator.

- <span id="recompositions-new-compatible"></span>`fn new_compatible(iter: I) -> Self`

  Create a new recomposition iterator for compatability compositions (NFkC)

  

  Note that this iterator can also be obtained by directly calling [`.nfkc()`](crate::UnicodeNormalization::nfkc)

  on the iterator.

#### Trait Implementations

##### `impl Any for Recompositions<I>`

- <span id="recompositions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Recompositions<I>`

- <span id="recompositions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Recompositions<I>`

- <span id="recompositions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Recompositions<I>`

- <span id="recompositions-clone"></span>`fn clone(&self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

##### `impl CloneToUninit for Recompositions<I>`

- <span id="recompositions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Recompositions<I>`

- <span id="recompositions-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Recompositions<I>`

- <span id="recompositions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Recompositions<I>`

##### `impl<U> Into for Recompositions<I>`

- <span id="recompositions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Recompositions<I>`

- <span id="recompositions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="recompositions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="recompositions-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Recompositions<I>`

- <span id="recompositions-iterator-type-item"></span>`type Item = char`

- <span id="recompositions-iterator-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl ToOwned for Recompositions<I>`

- <span id="recompositions-toowned-type-owned"></span>`type Owned = T`

- <span id="recompositions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="recompositions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Recompositions<I>`

- <span id="recompositions-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Recompositions<I>`

- <span id="recompositions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="recompositions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Recompositions<I>`

- <span id="recompositions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="recompositions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Recompositions<I>`

- <span id="recompositions-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="recompositions-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="recompositions-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="recompositions-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="recompositions-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

- <span id="recompositions-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md#streamsafe)

### `Replacements<I>`

```rust
struct Replacements<I> {
    iter: I,
    buffer: Option<char>,
}
```

*Defined in [`unicode-normalization-0.1.25/src/replace.rs:18-23`](../../.source_1765521767/unicode-normalization-0.1.25/src/replace.rs#L18-L23)*

External iterator for replacements for a string's characters.

#### Implementations

- <span id="replacements-new-cjk-compat-variants"></span>`fn new_cjk_compat_variants(iter: I) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

  Create a new iterator that replaces [CJK Compatibility Ideograph] codepoints with normal forms using [Standardized Variation Sequences].

  

  Note that this iterator can also be obtained by directly calling `.cjk_compat_variants()` on the iterator.

  

  

#### Trait Implementations

##### `impl Any for Replacements<I>`

- <span id="replacements-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Replacements<I>`

- <span id="replacements-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Replacements<I>`

- <span id="replacements-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Replacements<I>`

- <span id="replacements-clone"></span>`fn clone(&self) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

##### `impl CloneToUninit for Replacements<I>`

- <span id="replacements-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Iterator<Item = char> + Clone> Display for Replacements<I>`

- <span id="replacements-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Replacements<I>`

- <span id="replacements-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for Replacements<I>`

##### `impl<U> Into for Replacements<I>`

- <span id="replacements-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Replacements<I>`

- <span id="replacements-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="replacements-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="replacements-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for Replacements<I>`

- <span id="replacements-iterator-type-item"></span>`type Item = char`

- <span id="replacements-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="replacements-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Replacements<I>`

- <span id="replacements-toowned-type-owned"></span>`type Owned = T`

- <span id="replacements-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="replacements-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Replacements<I>`

- <span id="replacements-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Replacements<I>`

- <span id="replacements-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="replacements-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Replacements<I>`

- <span id="replacements-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="replacements-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for Replacements<I>`

- <span id="replacements-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="replacements-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="replacements-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="replacements-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="replacements-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

- <span id="replacements-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md#streamsafe)

### `StreamSafe<I>`

```rust
struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}
```

*Defined in [`unicode-normalization-0.1.25/src/stream_safe.rs:18-22`](../../.source_1765521767/unicode-normalization-0.1.25/src/stream_safe.rs#L18-L22)*

[UAX15-D4]: This iterator keeps track of how many non-starters there have been
since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
(U+034F) if the count exceeds 30.


#### Implementations

- <span id="streamsafe-new"></span>`fn new(iter: I) -> Self`

  Create a new stream safe iterator.

  

  Note that this iterator can also be obtained by directly calling [`.stream_safe()`](crate::UnicodeNormalization::stream_safe)

  on the iterator.

#### Trait Implementations

##### `impl Any for StreamSafe<I>`

- <span id="streamsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamSafe<I>`

- <span id="streamsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamSafe<I>`

- <span id="streamsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StreamSafe<I>`

- <span id="streamsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator<Item = char> + FusedIterator> FusedIterator for StreamSafe<I>`

##### `impl<U> Into for StreamSafe<I>`

- <span id="streamsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for StreamSafe<I>`

- <span id="streamsafe-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamsafe-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamsafe-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator<Item = char>> Iterator for StreamSafe<I>`

- <span id="streamsafe-iterator-type-item"></span>`type Item = char`

- <span id="streamsafe-iterator-next"></span>`fn next(&mut self) -> Option<char>`

##### `impl<U> TryFrom for StreamSafe<I>`

- <span id="streamsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamSafe<I>`

- <span id="streamsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UnicodeNormalization for StreamSafe<I>`

- <span id="streamsafe-unicodenormalization-nfd"></span>`fn nfd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="streamsafe-unicodenormalization-nfkd"></span>`fn nfkd(self) -> Decompositions<I>` — [`Decompositions`](decompose/index.md#decompositions)

- <span id="streamsafe-unicodenormalization-nfc"></span>`fn nfc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="streamsafe-unicodenormalization-nfkc"></span>`fn nfkc(self) -> Recompositions<I>` — [`Recompositions`](recompose/index.md#recompositions)

- <span id="streamsafe-unicodenormalization-cjk-compat-variants"></span>`fn cjk_compat_variants(self) -> Replacements<I>` — [`Replacements`](replace/index.md#replacements)

- <span id="streamsafe-unicodenormalization-stream-safe"></span>`fn stream_safe(self) -> StreamSafe<I>` — [`StreamSafe`](stream_safe/index.md#streamsafe)

## Enums

### `IsNormalized`

```rust
enum IsNormalized {
    Yes,
    No,
    Maybe,
}
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:13-20`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L13-L20)*

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

##### `impl Any for IsNormalized`

- <span id="isnormalized-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IsNormalized`

- <span id="isnormalized-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IsNormalized`

- <span id="isnormalized-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IsNormalized`

- <span id="isnormalized-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IsNormalized`

##### `impl<T> From for IsNormalized`

- <span id="isnormalized-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IsNormalized`

- <span id="isnormalized-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for IsNormalized`

- <span id="isnormalized-partialeq-eq"></span>`fn eq(&self, other: &IsNormalized) -> bool` — [`IsNormalized`](quick_check/index.md#isnormalized)

##### `impl StructuralPartialEq for IsNormalized`

##### `impl<U> TryFrom for IsNormalized`

- <span id="isnormalized-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="isnormalized-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IsNormalized`

- <span id="isnormalized-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="isnormalized-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `UnicodeNormalization<I: Iterator<Item = char>>`

```rust
trait UnicodeNormalization<I: Iterator<Item = char>> { ... }
```

*Defined in [`unicode-normalization-0.1.25/src/lib.rs:99-136`](../../.source_1765521767/unicode-normalization-0.1.25/src/lib.rs#L99-L136)*

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

- `&'a str`
- `I`
- `char`

## Functions

### `is_nfc`

```rust
fn is_nfc(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:111-117`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L111-L117)*

Authoritatively check if a string is in NFC.

### `is_nfc_quick`

```rust
fn is_nfc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:75-77`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L75-L77)*

Quickly check if a string is in NFC, potentially returning
`IsNormalized::Maybe` if further checks are necessary.  In this case a check
like `s.chars().nfc().eq(s.chars())` should suffice.

### `is_nfc_stream_safe`

```rust
fn is_nfc_stream_safe(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:151-157`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L151-L157)*

Authoritatively check if a string is Stream-Safe NFC.

### `is_nfc_stream_safe_quick`

```rust
fn is_nfc_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:99-101`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L99-L101)*

Quickly check if a string is Stream-Safe NFC.

### `is_nfd`

```rust
fn is_nfd(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:131-137`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L131-L137)*

Authoritatively check if a string is in NFD.

### `is_nfd_quick`

```rust
fn is_nfd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:87-89`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L87-L89)*

Quickly check if a string is in NFD.

### `is_nfd_stream_safe`

```rust
fn is_nfd_stream_safe(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:161-167`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L161-L167)*

Authoritatively check if a string is Stream-Safe NFD.

### `is_nfd_stream_safe_quick`

```rust
fn is_nfd_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:105-107`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L105-L107)*

Quickly check if a string is Stream-Safe NFD.

### `is_nfkc`

```rust
fn is_nfkc(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:121-127`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L121-L127)*

Authoritatively check if a string is in NFKC.

### `is_nfkc_quick`

```rust
fn is_nfkc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:81-83`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L81-L83)*

Quickly check if a string is in NFKC.

### `is_nfkd`

```rust
fn is_nfkd(s: &str) -> bool
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:141-147`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L141-L147)*

Authoritatively check if a string is in NFKD.

### `is_nfkd_quick`

```rust
fn is_nfkd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized
```

*Defined in [`unicode-normalization-0.1.25/src/quick_check.rs:93-95`](../../.source_1765521767/unicode-normalization-0.1.25/src/quick_check.rs#L93-L95)*

Quickly check if a string is in NFKD.

## Constants

### `UNICODE_VERSION`
```rust
const UNICODE_VERSION: (u8, u8, u8);
```

*Defined in [`unicode-normalization-0.1.25/src/tables.rs:18`](../../.source_1765521767/unicode-normalization-0.1.25/src/tables.rs#L18)*

