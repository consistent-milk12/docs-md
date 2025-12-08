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

## Structs

### `Utf8Range`

```rust
struct Utf8Range {
    pub start: u8,
    pub end: u8,
}
```

A single inclusive range of UTF-8 bytes.

#### Fields

- **`start`**: `u8`

  Start of byte range (inclusive).

- **`end`**: `u8`

  End of byte range (inclusive).

#### Implementations

- `fn new(start: u8, end: u8) -> Self`

- `fn matches(self: &Self, b: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Range`

- `fn clone(self: &Self) -> Utf8Range` — [`Utf8Range`](#utf8range)

##### `impl Copy for Utf8Range`

##### `impl Debug for Utf8Range`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Range`

##### `impl Ord for Utf8Range`

- `fn cmp(self: &Self, other: &Utf8Range) -> $crate::cmp::Ordering` — [`Utf8Range`](#utf8range)

##### `impl PartialEq for Utf8Range`

- `fn eq(self: &Self, other: &Utf8Range) -> bool` — [`Utf8Range`](#utf8range)

##### `impl PartialOrd for Utf8Range`

- `fn partial_cmp(self: &Self, other: &Utf8Range) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Utf8Range`](#utf8range)

##### `impl StructuralPartialEq for Utf8Range`

### `Utf8Sequences`

```rust
struct Utf8Sequences {
    range_stack: alloc::vec::Vec<ScalarRange>,
}
```

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

- `fn new(start: char, end: char) -> Self`

- `fn push(self: &mut Self, start: u32, end: u32)`

#### Trait Implementations

##### `impl Debug for Utf8Sequences`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FusedIterator for Utf8Sequences`

##### `impl<I> IntoIterator for Utf8Sequences`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for Utf8Sequences`

- `type Item = Utf8Sequence`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ScalarRange`

```rust
struct ScalarRange {
    start: u32,
    end: u32,
}
```

#### Implementations

- `fn split(self: &Self) -> Option<(ScalarRange, ScalarRange)>` — [`ScalarRange`](#scalarrange)

- `fn is_valid(self: &Self) -> bool`

- `fn as_ascii(self: &Self) -> Option<Utf8Range>` — [`Utf8Range`](#utf8range)

- `fn is_ascii(self: &Self) -> bool`

- `fn encode(self: &Self, start: &mut [u8], end: &mut [u8]) -> usize`

#### Trait Implementations

##### `impl Debug for ScalarRange`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn from_encoded_range(start: &[u8], end: &[u8]) -> Self`

- `fn as_slice(self: &Self) -> &[Utf8Range]` — [`Utf8Range`](#utf8range)

- `fn len(self: &Self) -> usize`

- `fn reverse(self: &mut Self)`

- `fn matches(self: &Self, bytes: &[u8]) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Sequence`

- `fn clone(self: &Self) -> Utf8Sequence` — [`Utf8Sequence`](#utf8sequence)

##### `impl Copy for Utf8Sequence`

##### `impl Debug for Utf8Sequence`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utf8Sequence`

##### `impl Ord for Utf8Sequence`

- `fn cmp(self: &Self, other: &Utf8Sequence) -> $crate::cmp::Ordering` — [`Utf8Sequence`](#utf8sequence)

##### `impl PartialEq for Utf8Sequence`

- `fn eq(self: &Self, other: &Utf8Sequence) -> bool` — [`Utf8Sequence`](#utf8sequence)

##### `impl PartialOrd for Utf8Sequence`

- `fn partial_cmp(self: &Self, other: &Utf8Sequence) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Utf8Sequence`](#utf8sequence)

##### `impl StructuralPartialEq for Utf8Sequence`

## Functions

### `max_scalar_value`

```rust
fn max_scalar_value(nbytes: usize) -> u32
```

## Constants

### `MAX_UTF8_BYTES`

```rust
const MAX_UTF8_BYTES: usize = 4usize;
```

