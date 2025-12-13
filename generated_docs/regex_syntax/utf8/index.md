*[regex_syntax](../index.md) / [utf8](index.md)*

---

# Module `utf8`

Converts ranges of Unicode scalar values to equivalent ranges of UTF-8 bytes.

This is sub-module is useful for constructing byte based automatons that need
to embed UTF-8 decoding. The most common use of this module is in conjunction
with the [`hir::ClassUnicodeRange`](crate::hir::ClassUnicodeRange) type.

See the documentation on the `Utf8Sequences` iterator for more details and
an example.

# Wait, what is this?

This is simplest to explain with an example. Let's say you wanted to test
whether a particular byte sequence was a Cyrillic character. One possible
scalar value range is `[0400-04FF]`. The set of allowed bytes for this
range can be expressed as a sequence of byte ranges:

```text
[D0-D3][80-BF]
```

This is simple enough: simply encode the boundaries, `0400` encodes to
`D0 80` and `04FF` encodes to `D3 BF`, and create ranges from each
corresponding pair of bytes: `D0` to `D3` and `80` to `BF`.

However, what if you wanted to add the Cyrillic Supplementary characters to
your range? Your range might then become `[0400-052F]`. The same procedure
as above doesn't quite work because `052F` encodes to `D4 AF`. The byte ranges
you'd get from the previous transformation would be `[D0-D4][80-AF]`. However,
this isn't quite correct because this range doesn't capture many characters,
for example, `04FF` (because its last byte, `BF` isn't in the range `80-AF`).

Instead, you need multiple sequences of byte ranges:

```text
[D0-D3][80-BF]  # matches codepoints 0400-04FF
[D4][80-AF]     # matches codepoints 0500-052F
```

This gets even more complicated if you want bigger ranges, particularly if
they naively contain surrogate codepoints. For example, the sequence of byte
ranges for the basic multilingual plane (`[0000-FFFF]`) look like this:

```text
[0-7F]
[C2-DF][80-BF]
[E0][A0-BF][80-BF]
[E1-EC][80-BF][80-BF]
[ED][80-9F][80-BF]
[EE-EF][80-BF][80-BF]
```

Note that the byte ranges above will *not* match any erroneous encoding of
UTF-8, including encodings of surrogate codepoints.

And, of course, for all of Unicode (`[000000-10FFFF]`):

```text
[0-7F]
[C2-DF][80-BF]
[E0][A0-BF][80-BF]
[E1-EC][80-BF][80-BF]
[ED][80-9F][80-BF]
[EE-EF][80-BF][80-BF]
[F0][90-BF][80-BF][80-BF]
[F1-F3][80-BF][80-BF][80-BF]
[F4][80-8F][80-BF][80-BF]
```

This module automates the process of creating these byte ranges from ranges of
Unicode scalar values.

# Lineage

I got the idea and general implementation strategy from Russ Cox in his
[article on regexps](https://web.archive.org/web/20160404141123/https://swtch.com/~rsc/regexp/regexp3.html) and RE2.
Russ Cox got it from Ken Thompson's `grep` (no source, folk lore?).
I also got the idea from
[Lucene](https://github.com/apache/lucene-solr/blob/ae93f4e7ac6a3908046391de35d4f50a0d3c59ca/lucene/core/src/java/org/apache/lucene/util/automaton/UTF32ToUTF8.java),
which uses it for executing automata on their term index.

## Contents

- [Structs](#structs)
  - [`Utf8Range`](#utf8range)
  - [`Utf8Sequences`](#utf8sequences)
  - [`ScalarRange`](#scalarrange)
- [Enums](#enums)
  - [`Utf8Sequence`](#utf8sequence)
- [Functions](#functions)
  - [`max_scalar_value`](#max-scalar-value)
- [Constants](#constants)
  - [`MAX_UTF8_BYTES`](#max-utf8-bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8Range`](#utf8range) | struct | A single inclusive range of UTF-8 bytes. |
| [`Utf8Sequences`](#utf8sequences) | struct | An iterator over ranges of matching UTF-8 byte sequences. |
| [`ScalarRange`](#scalarrange) | struct |  |
| [`Utf8Sequence`](#utf8sequence) | enum | Utf8Sequence represents a sequence of byte ranges. |
| [`max_scalar_value`](#max-scalar-value) | fn |  |
| [`MAX_UTF8_BYTES`](#max-utf8-bytes) | const |  |

## Structs

### `Utf8Range`

```rust
struct Utf8Range {
    pub start: u8,
    pub end: u8,
}
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:218-223`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L218-L223)*

A single inclusive range of UTF-8 bytes.

#### Fields

- **`start`**: `u8`

  Start of byte range (inclusive).

- **`end`**: `u8`

  End of byte range (inclusive).

#### Implementations

- <span id="utf8range-new"></span>`fn new(start: u8, end: u8) -> Self`

- <span id="utf8range-matches"></span>`fn matches(&self, b: u8) -> bool`

  Returns true if and only if the given byte is in this range.

#### Trait Implementations

##### `impl Any for Utf8Range`

- <span id="utf8range-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Range`

- <span id="utf8range-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Range`

- <span id="utf8range-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Range`

- <span id="utf8range-clone"></span>`fn clone(&self) -> Utf8Range` — [`Utf8Range`](#utf8range)

##### `impl CloneToUninit for Utf8Range`

- <span id="utf8range-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8Range`

##### `impl Debug for Utf8Range`

- <span id="utf8range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Range`

##### `impl<T> From for Utf8Range`

- <span id="utf8range-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8Range`

- <span id="utf8range-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Utf8Range`

- <span id="utf8range-ord-cmp"></span>`fn cmp(&self, other: &Utf8Range) -> cmp::Ordering` — [`Utf8Range`](#utf8range)

##### `impl PartialEq for Utf8Range`

- <span id="utf8range-partialeq-eq"></span>`fn eq(&self, other: &Utf8Range) -> bool` — [`Utf8Range`](#utf8range)

##### `impl PartialOrd for Utf8Range`

- <span id="utf8range-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Range) -> option::Option<cmp::Ordering>` — [`Utf8Range`](#utf8range)

##### `impl StructuralPartialEq for Utf8Range`

##### `impl ToOwned for Utf8Range`

- <span id="utf8range-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8range-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8range-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Range`

- <span id="utf8range-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8range-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Range`

- <span id="utf8range-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8range-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8Sequences`

```rust
struct Utf8Sequences {
    range_stack: alloc::vec::Vec<ScalarRange>,
}
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:297-299`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L297-L299)*

An iterator over ranges of matching UTF-8 byte sequences.

The iteration represents an alternation of comprehensive byte sequences
that match precisely the set of UTF-8 encoded scalar values.

A byte sequence corresponds to one of the scalar values in the range given
if and only if it completely matches exactly one of the sequences of byte
ranges produced by this iterator.

Each sequence of byte ranges matches a unique set of bytes. That is, no two
sequences will match the same bytes.

# Example

This shows how to match an arbitrary byte sequence against a range of
scalar values.

```rust
use regex_syntax::utf8::{Utf8Sequences, Utf8Sequence};

fn matches(seqs: &[Utf8Sequence], bytes: &[u8]) -> bool {
    for range in seqs {
        if range.matches(bytes) {
            return true;
        }
    }
    false
}

// Test the basic multilingual plane.
let seqs: Vec<_> = Utf8Sequences::new('\u{0}', '\u{FFFF}').collect();

// UTF-8 encoding of 'a'.
assert!(matches(&seqs, &[0x61]));
// UTF-8 encoding of '☃' (`\u{2603}`).
assert!(matches(&seqs, &[0xE2, 0x98, 0x83]));
// UTF-8 encoding of `\u{10348}` (outside the BMP).
assert!(!matches(&seqs, &[0xF0, 0x90, 0x8D, 0x88]));
// Tries to match against a UTF-8 encoding of a surrogate codepoint,
// which is invalid UTF-8, and therefore fails, despite the fact that
// the corresponding codepoint (0xD800) falls in the range given.
assert!(!matches(&seqs, &[0xED, 0xA0, 0x80]));
// And fails against plain old invalid UTF-8.
assert!(!matches(&seqs, &[0xFF, 0xFF]));
```

If this example seems circuitous, that's because it is! It's meant to be
illustrative. In practice, you could just try to decode your byte sequence
and compare it with the scalar value range directly. However, this is not
always possible (for example, in a byte based automaton).

#### Implementations

- <span id="utf8sequences-new"></span>`fn new(start: char, end: char) -> Self`

  Create a new iterator over UTF-8 byte ranges for the scalar value range

  given.

- <span id="utf8sequences-push"></span>`fn push(&mut self, start: u32, end: u32)`

#### Trait Implementations

##### `impl Any for Utf8Sequences`

- <span id="utf8sequences-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Sequences`

- <span id="utf8sequences-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Sequences`

- <span id="utf8sequences-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Utf8Sequences`

- <span id="utf8sequences-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8Sequences`

- <span id="utf8sequences-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Utf8Sequences`

##### `impl<U> Into for Utf8Sequences`

- <span id="utf8sequences-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Utf8Sequences`

- <span id="utf8sequences-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="utf8sequences-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="utf8sequences-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Utf8Sequences`

- <span id="utf8sequences-iterator-type-item"></span>`type Item = Utf8Sequence`

- <span id="utf8sequences-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Utf8Sequences`

- <span id="utf8sequences-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8sequences-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Sequences`

- <span id="utf8sequences-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8sequences-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ScalarRange`

```rust
struct ScalarRange {
    start: u32,
    end: u32,
}
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:325-328`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L325-L328)*

#### Implementations

- <span id="scalarrange-split"></span>`fn split(&self) -> Option<(ScalarRange, ScalarRange)>` — [`ScalarRange`](#scalarrange)

  split splits this range if it overlaps with a surrogate codepoint.

  

  Either or both ranges may be invalid.

- <span id="scalarrange-is-valid"></span>`fn is_valid(&self) -> bool`

  is_valid returns true if and only if start <= end.

- <span id="scalarrange-as-ascii"></span>`fn as_ascii(&self) -> Option<Utf8Range>` — [`Utf8Range`](#utf8range)

  as_ascii returns this range as a Utf8Range if and only if all scalar

  values in this range can be encoded as a single byte.

- <span id="scalarrange-is-ascii"></span>`fn is_ascii(&self) -> bool`

  is_ascii returns true if the range is ASCII only (i.e., takes a single

  byte to encode any scalar value).

- <span id="scalarrange-encode"></span>`fn encode(&self, start: &mut [u8], end: &mut [u8]) -> usize`

  encode writes the UTF-8 encoding of the start and end of this range

  to the corresponding destination slices, and returns the number of

  bytes written.

  

  The slices should have room for at least `MAX_UTF8_BYTES`.

#### Trait Implementations

##### `impl Any for ScalarRange`

- <span id="scalarrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScalarRange`

- <span id="scalarrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScalarRange`

- <span id="scalarrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ScalarRange`

- <span id="scalarrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ScalarRange`

- <span id="scalarrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ScalarRange`

- <span id="scalarrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ScalarRange`

- <span id="scalarrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scalarrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ScalarRange`

- <span id="scalarrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scalarrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Utf8Sequence`

```rust
enum Utf8Sequence {
    One(Utf8Range),
    Two([Utf8Range; 2]),
    Three([Utf8Range; 3]),
    Four([Utf8Range; 4]),
}
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:97-106`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L97-L106)*

Utf8Sequence represents a sequence of byte ranges.

To match a Utf8Sequence, a candidate byte sequence must match each
successive range.

For example, if there are two ranges, `[C2-DF][80-BF]`, then the byte
sequence `\xDD\x61` would not match because `0x61 < 0x80`.

#### Variants

- **`One`**

  One byte range.

- **`Two`**

  Two successive byte ranges.

- **`Three`**

  Three successive byte ranges.

- **`Four`**

  Four successive byte ranges.

#### Implementations

- <span id="utf8sequence-from-encoded-range"></span>`fn from_encoded_range(start: &[u8], end: &[u8]) -> Self`

  Creates a new UTF-8 sequence from the encoded bytes of a scalar value

  range.

  

  This assumes that `start` and `end` have the same length.

- <span id="utf8sequence-as-slice"></span>`fn as_slice(&self) -> &[Utf8Range]` — [`Utf8Range`](#utf8range)

  Returns the underlying sequence of byte ranges as a slice.

- <span id="utf8sequence-len"></span>`fn len(&self) -> usize`

  Returns the number of byte ranges in this sequence.

  

  The length is guaranteed to be in the closed interval `[1, 4]`.

- <span id="utf8sequence-reverse"></span>`fn reverse(&mut self)`

  Reverses the ranges in this sequence.

  

  For example, if this corresponds to the following sequence:

  

  ```text

  [D0-D3][80-BF]

  ```

  

  Then after reversal, it will be

  

  ```text

  [80-BF][D0-D3]

  ```

  

  This is useful when one is constructing a UTF-8 automaton to match

  character classes in reverse.

- <span id="utf8sequence-matches"></span>`fn matches(&self, bytes: &[u8]) -> bool`

  Returns true if and only if a prefix of `bytes` matches this sequence

  of byte ranges.

#### Trait Implementations

##### `impl Any for Utf8Sequence`

- <span id="utf8sequence-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8Sequence`

- <span id="utf8sequence-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8Sequence`

- <span id="utf8sequence-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8Sequence`

- <span id="utf8sequence-clone"></span>`fn clone(&self) -> Utf8Sequence` — [`Utf8Sequence`](#utf8sequence)

##### `impl CloneToUninit for Utf8Sequence`

- <span id="utf8sequence-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Utf8Sequence`

##### `impl Debug for Utf8Sequence`

- <span id="utf8sequence-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Sequence`

##### `impl<T> From for Utf8Sequence`

- <span id="utf8sequence-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8Sequence`

- <span id="utf8sequence-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for &'a Utf8Sequence`

- <span id="a-utf8sequence-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, Utf8Range>`

- <span id="a-utf8sequence-intoiterator-type-item"></span>`type Item = &'a Utf8Range`

- <span id="a-utf8sequence-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl Ord for Utf8Sequence`

- <span id="utf8sequence-ord-cmp"></span>`fn cmp(&self, other: &Utf8Sequence) -> cmp::Ordering` — [`Utf8Sequence`](#utf8sequence)

##### `impl PartialEq for Utf8Sequence`

- <span id="utf8sequence-partialeq-eq"></span>`fn eq(&self, other: &Utf8Sequence) -> bool` — [`Utf8Sequence`](#utf8sequence)

##### `impl PartialOrd for Utf8Sequence`

- <span id="utf8sequence-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Utf8Sequence) -> option::Option<cmp::Ordering>` — [`Utf8Sequence`](#utf8sequence)

##### `impl StructuralPartialEq for Utf8Sequence`

##### `impl ToOwned for Utf8Sequence`

- <span id="utf8sequence-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8sequence-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8sequence-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8Sequence`

- <span id="utf8sequence-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8sequence-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8Sequence`

- <span id="utf8sequence-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8sequence-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `max_scalar_value`

```rust
fn max_scalar_value(nbytes: usize) -> u32
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:445-453`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L445-L453)*

## Constants

### `MAX_UTF8_BYTES`
```rust
const MAX_UTF8_BYTES: usize = 4usize;
```

*Defined in [`regex-syntax-0.8.8/src/utf8.rs:87`](../../../.source_1765521767/regex-syntax-0.8.8/src/utf8.rs#L87)*

