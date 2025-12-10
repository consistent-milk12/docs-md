*[regex_automata](../../index.md) / [util](../index.md) / [look](index.md)*

---

# Module `look`

Types and routines for working with look-around assertions.

This module principally defines two types:

* [`Look`](#look) enumerates all of the assertions supported by this crate.
* [`LookSet`](#lookset) provides a way to efficiently store a set of [`Look`](#look) values.
* [`LookMatcher`](#lookmatcher) provides routines for checking whether a `Look` or a
`LookSet` matches at a particular position in a haystack.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`is_word_char`](#is_word_char) | mod | A module that looks for word codepoints using regex-syntax's data tables. |
| [`LookSet`](#lookset) | struct | LookSet is a memory-efficient set of look-around assertions. |
| [`LookSetIter`](#looksetiter) | struct | An iterator over all look-around assertions in a [`LookSet`]. |
| [`LookMatcher`](#lookmatcher) | struct | A matcher for look-around assertions. |
| [`UnicodeWordBoundaryError`](#unicodewordboundaryerror) | struct | An error that occurs when the Unicode-aware `\w` class is unavailable. |
| [`Look`](#look) | enum | A look-around assertion. |

## Modules

- [`is_word_char`](is_word_char/index.md) — A module that looks for word codepoints using regex-syntax's data tables.

## Structs

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:244-255`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/look.rs#L244-L255)*

LookSet is a memory-efficient set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, a [`thompson::NFA`](crate::nfa::thompson::NFA) provides properties
that return `LookSet`s.

#### Fields

- **`bits`**: `u32`

  The underlying representation this set is exposed to make it possible
  to store it somewhere efficiently. The representation is that
  of a bitset, where each assertion occupies bit `i` where
  `i = Look::as_repr()`.
  
  Note that users of this internal representation must permit the full
  range of `u16` values to be represented. For example, even if the
  current implementation only makes use of the 10 least significant bits,
  it may use more bits in a future semver compatible release.

#### Implementations

- <span id="lookset-empty"></span>`fn empty() -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-full"></span>`fn full() -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-singleton"></span>`fn singleton(look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-len"></span>`fn len(self) -> usize`

- <span id="lookset-is-empty"></span>`fn is_empty(self) -> bool`

- <span id="lookset-contains"></span>`fn contains(self, look: Look) -> bool` — [`Look`](#look)

- <span id="lookset-contains-anchor"></span>`fn contains_anchor(&self) -> bool`

- <span id="lookset-contains-anchor-haystack"></span>`fn contains_anchor_haystack(&self) -> bool`

- <span id="lookset-contains-anchor-line"></span>`fn contains_anchor_line(&self) -> bool`

- <span id="lookset-contains-anchor-lf"></span>`fn contains_anchor_lf(&self) -> bool`

- <span id="lookset-contains-anchor-crlf"></span>`fn contains_anchor_crlf(&self) -> bool`

- <span id="lookset-contains-word"></span>`fn contains_word(self) -> bool`

- <span id="lookset-contains-word-unicode"></span>`fn contains_word_unicode(self) -> bool`

- <span id="lookset-contains-word-ascii"></span>`fn contains_word_ascii(self) -> bool`

- <span id="lookset-iter"></span>`fn iter(self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

- <span id="lookset-insert"></span>`fn insert(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-set-insert"></span>`fn set_insert(&mut self, look: Look)` — [`Look`](#look)

- <span id="lookset-remove"></span>`fn remove(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- <span id="lookset-set-remove"></span>`fn set_remove(&mut self, look: Look)` — [`Look`](#look)

- <span id="lookset-subtract"></span>`fn subtract(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-subtract"></span>`fn set_subtract(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-union"></span>`fn union(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-union"></span>`fn set_union(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-intersect"></span>`fn intersect(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-set-intersect"></span>`fn set_intersect(&mut self, other: LookSet)` — [`LookSet`](#lookset)

- <span id="lookset-read-repr"></span>`fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](#lookset)

- <span id="lookset-write-repr"></span>`fn write_repr(self, slice: &mut [u8])`

- <span id="lookset-available"></span>`fn available(self) -> Result<(), UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for LookSet`

- <span id="lookset-clone"></span>`fn clone(&self) -> LookSet` — [`LookSet`](#lookset)

##### `impl Copy for LookSet`

##### `impl Debug for LookSet`

- <span id="lookset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for LookSet`

- <span id="lookset-default"></span>`fn default() -> LookSet` — [`LookSet`](#lookset)

##### `impl Eq for LookSet`

##### `impl PartialEq for LookSet`

- <span id="lookset-eq"></span>`fn eq(&self, other: &LookSet) -> bool` — [`LookSet`](#lookset)

##### `impl StructuralPartialEq for LookSet`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:517-519`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/look.rs#L517-L519)*

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Clone for LookSetIter`

- <span id="looksetiter-clone"></span>`fn clone(&self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

##### `impl Debug for LookSetIter`

- <span id="looksetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LookSetIter`

- <span id="looksetiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="looksetiter-type-intoiter"></span>`type IntoIter = I`

- <span id="looksetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LookSetIter`

- <span id="looksetiter-type-item"></span>`type Item = Look`

- <span id="looksetiter-next"></span>`fn next(&mut self) -> Option<Look>` — [`Look`](#look)

### `LookMatcher`

```rust
struct LookMatcher {
    lineterm: crate::util::escape::DebugByte,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:577-579`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/look.rs#L577-L579)*

A matcher for look-around assertions.

This matcher permits configuring aspects of how look-around assertions are
matched.

# Example

A `LookMatcher` can change the line terminator used for matching multi-line
anchors such as `(?m:^)` and `(?m:$)`.

```rust
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::look::LookMatcher,
    Match, Input,
};

let mut lookm = LookMatcher::new();
lookm.set_line_terminator(b'\x00');

let re = PikeVM::builder()
    .thompson(thompson::Config::new().look_matcher(lookm))
    .build(r"(?m)^[a-z]+$")?;
let mut cache = re.create_cache();

// Multi-line assertions now use NUL as a terminator.
assert_eq!(
    Some(Match::must(0, 1..4)),
    re.find(&mut cache, b"\x00abc\x00"),
);
// ... and \n is no longer recognized as a terminator.
assert_eq!(
    None,
    re.find(&mut cache, b"\nabc\n"),
);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="lookmatcher-new"></span>`fn new() -> LookMatcher` — [`LookMatcher`](#lookmatcher)

- <span id="lookmatcher-set-line-terminator"></span>`fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher` — [`LookMatcher`](#lookmatcher)

- <span id="lookmatcher-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

- <span id="lookmatcher-matches"></span>`fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

- <span id="lookmatcher-matches-inline"></span>`fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

- <span id="lookmatcher-matches-set"></span>`fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

- <span id="lookmatcher-matches-set-inline"></span>`fn matches_set_inline(&self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

- <span id="lookmatcher-add-to-byteset"></span>`fn add_to_byteset(&self, look: Look, set: &mut crate::util::alphabet::ByteClassSet)` — [`Look`](#look), [`ByteClassSet`](../alphabet/index.md#byteclassset)

- <span id="lookmatcher-is-start"></span>`fn is_start(&self, _haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-end"></span>`fn is_end(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-start-lf"></span>`fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-end-lf"></span>`fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-start-crlf"></span>`fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-end-crlf"></span>`fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-ascii"></span>`fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-ascii-negate"></span>`fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-unicode"></span>`fn is_word_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- <span id="lookmatcher-is-word-unicode-negate"></span>`fn is_word_unicode_negate(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- <span id="lookmatcher-is-word-start-ascii"></span>`fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-end-ascii"></span>`fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-start-unicode"></span>`fn is_word_start_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- <span id="lookmatcher-is-word-end-unicode"></span>`fn is_word_end_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- <span id="lookmatcher-is-word-start-half-ascii"></span>`fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-end-half-ascii"></span>`fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool`

- <span id="lookmatcher-is-word-start-half-unicode"></span>`fn is_word_start_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- <span id="lookmatcher-is-word-end-half-unicode"></span>`fn is_word_end_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for LookMatcher`

- <span id="lookmatcher-clone"></span>`fn clone(&self) -> LookMatcher` — [`LookMatcher`](#lookmatcher)

##### `impl Debug for LookMatcher`

- <span id="lookmatcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LookMatcher`

- <span id="lookmatcher-default"></span>`fn default() -> LookMatcher` — [`LookMatcher`](#lookmatcher)

### `UnicodeWordBoundaryError`

```rust
struct UnicodeWordBoundaryError(());
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:1280`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/look.rs#L1280)*

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. The `\w` class is used to
determine whether a codepoint is considered a word character or not when
determining whether a Unicode aware `\b` (or `\B`) matches at a particular
position.

This error can only occur when the `unicode-word-boundary` feature is
disabled.

#### Implementations

- <span id="unicodewordboundaryerror-check"></span>`fn check() -> Result<(), UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-clone"></span>`fn clone(&self) -> UnicodeWordBoundaryError` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

##### `impl Debug for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for UnicodeWordBoundaryError`

##### `impl ToString for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Look`

```rust
enum Look {
    Start,
    End,
    StartLF,
    EndLF,
    StartCRLF,
    EndCRLF,
    WordAscii,
    WordAsciiNegate,
    WordUnicode,
    WordUnicodeNegate,
    WordStartAscii,
    WordEndAscii,
    WordStartUnicode,
    WordEndUnicode,
    WordStartHalfAscii,
    WordEndHalfAscii,
    WordStartHalfUnicode,
    WordEndHalfUnicode,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:62-135`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/look.rs#L62-L135)*

A look-around assertion.

An assertion matches at a position between characters in a haystack.
Namely, it does not actually "consume" any input as most parts of a regular
expression do. Assertions are a way of stating that some property must be
true at a particular point during matching.

For example, `(?m)^[a-z]+$` is a pattern that:

* Scans the haystack for a position at which `(?m:^)` is satisfied. That
occurs at either the beginning of the haystack, or immediately following
a `\n` character.
* Looks for one or more occurrences of `[a-z]`.
* Once `[a-z]+` has matched as much as it can, an overall match is only
reported when `[a-z]+` stops just before a `\n`.

So in this case, `abc` and `\nabc\n` match, but `\nabc1\n` does not.

Assertions are also called "look-around," "look-behind" and "look-ahead."
Specifically, some assertions are look-behind (like `^`), other assertions
are look-ahead (like `$`) and yet other assertions are both look-ahead and
look-behind (like `\b`).

# Assertions in an NFA

An assertion in a [`thompson::NFA`](crate::nfa::thompson::NFA) can be
thought of as a conditional epsilon transition. That is, a matching engine
like the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) only permits
moving through conditional epsilon transitions when their condition
is satisfied at whatever position the `PikeVM` is currently at in the
haystack.

How assertions are handled in a `DFA` is trickier, since a DFA does not
have epsilon transitions at all. In this case, they are compiled into the
automaton itself, at the expense of more states than what would be required
without an assertion.

#### Variants

- **`Start`**

  Match the beginning of text. Specifically, this matches at the starting
  position of the input.

- **`End`**

  Match the end of text. Specifically, this matches at the ending
  position of the input.

- **`StartLF`**

  Match the beginning of a line or the beginning of text. Specifically,
  this matches at the starting position of the input, or at the position
  immediately following a `\n` character.

- **`EndLF`**

  Match the end of a line or the end of text. Specifically, this matches
  at the end position of the input, or at the position immediately
  preceding a `\n` character.

- **`StartCRLF`**

  Match the beginning of a line or the beginning of text. Specifically,
  this matches at the starting position of the input, or at the position
  immediately following either a `\r` or `\n` character, but never after
  a `\r` when a `\n` follows.

- **`EndCRLF`**

  Match the end of a line or the end of text. Specifically, this matches
  at the end position of the input, or at the position immediately
  preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
  precedes it.

- **`WordAscii`**

  Match an ASCII-only word boundary. That is, this matches a position
  where the left adjacent character and right adjacent character
  correspond to a word and non-word or a non-word and word character.

- **`WordAsciiNegate`**

  Match an ASCII-only negation of a word boundary.

- **`WordUnicode`**

  Match a Unicode-aware word boundary. That is, this matches a position
  where the left adjacent character and right adjacent character
  correspond to a word and non-word or a non-word and word character.

- **`WordUnicodeNegate`**

  Match a Unicode-aware negation of a word boundary.

- **`WordStartAscii`**

  Match the start of an ASCII-only word boundary. That is, this matches a
  position at either the beginning of the haystack or where the previous
  character is not a word character and the following character is a word
  character.

- **`WordEndAscii`**

  Match the end of an ASCII-only word boundary. That is, this matches
  a position at either the end of the haystack or where the previous
  character is a word character and the following character is not a word
  character.

- **`WordStartUnicode`**

  Match the start of a Unicode word boundary. That is, this matches a
  position at either the beginning of the haystack or where the previous
  character is not a word character and the following character is a word
  character.

- **`WordEndUnicode`**

  Match the end of a Unicode word boundary. That is, this matches a
  position at either the end of the haystack or where the previous
  character is a word character and the following character is not a word
  character.

- **`WordStartHalfAscii`**

  Match the start half of an ASCII-only word boundary. That is, this
  matches a position at either the beginning of the haystack or where the
  previous character is not a word character.

- **`WordEndHalfAscii`**

  Match the end half of an ASCII-only word boundary. That is, this
  matches a position at either the end of the haystack or where the
  following character is not a word character.

- **`WordStartHalfUnicode`**

  Match the start half of a Unicode word boundary. That is, this matches
  a position at either the beginning of the haystack or where the
  previous character is not a word character.

- **`WordEndHalfUnicode`**

  Match the end half of a Unicode word boundary. That is, this matches
  a position at either the end of the haystack or where the following
  character is not a word character.

#### Implementations

- <span id="look-reversed"></span>`const fn reversed(self) -> Look` — [`Look`](#look)

- <span id="look-as-repr"></span>`const fn as_repr(self) -> u32`

- <span id="look-from-repr"></span>`const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](#look)

- <span id="look-as-char"></span>`const fn as_char(self) -> char`

#### Trait Implementations

##### `impl Clone for Look`

- <span id="look-clone"></span>`fn clone(&self) -> Look` — [`Look`](#look)

##### `impl Copy for Look`

##### `impl Debug for Look`

- <span id="look-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Look`

##### `impl PartialEq for Look`

- <span id="look-eq"></span>`fn eq(&self, other: &Look) -> bool` — [`Look`](#look)

##### `impl StructuralPartialEq for Look`

