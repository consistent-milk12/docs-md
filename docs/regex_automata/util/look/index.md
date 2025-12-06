*[regex_automata](../../index.md) / [util](../index.md) / [look](index.md)*

---

# Module `look`

Types and routines for working with look-around assertions.

This module principally defines two types:

* [`Look`](../../index.md) enumerates all of the assertions supported by this crate.
* [`LookSet`](#lookset) provides a way to efficiently store a set of [`Look`](../../index.md) values.
* [`LookMatcher`](#lookmatcher) provides routines for checking whether a `Look` or a
`LookSet` matches at a particular position in a haystack.

## Structs

### `LookSet`

```rust
struct LookSet {
    pub bits: u32,
}
```

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

- `fn empty() -> LookSet` — [`LookSet`](#lookset)

- `fn full() -> LookSet` — [`LookSet`](#lookset)

- `fn singleton(look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- `fn len(self: Self) -> usize`

- `fn is_empty(self: Self) -> bool`

- `fn contains(self: Self, look: Look) -> bool` — [`Look`](#look)

- `fn contains_anchor(self: &Self) -> bool`

- `fn contains_anchor_haystack(self: &Self) -> bool`

- `fn contains_anchor_line(self: &Self) -> bool`

- `fn contains_anchor_lf(self: &Self) -> bool`

- `fn contains_anchor_crlf(self: &Self) -> bool`

- `fn contains_word(self: Self) -> bool`

- `fn contains_word_unicode(self: Self) -> bool`

- `fn contains_word_ascii(self: Self) -> bool`

- `fn iter(self: Self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

- `fn insert(self: Self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- `fn set_insert(self: &mut Self, look: Look)` — [`Look`](#look)

- `fn remove(self: Self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

- `fn set_remove(self: &mut Self, look: Look)` — [`Look`](#look)

- `fn subtract(self: Self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- `fn set_subtract(self: &mut Self, other: LookSet)` — [`LookSet`](#lookset)

- `fn union(self: Self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- `fn set_union(self: &mut Self, other: LookSet)` — [`LookSet`](#lookset)

- `fn intersect(self: Self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

- `fn set_intersect(self: &mut Self, other: LookSet)` — [`LookSet`](#lookset)

- `fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](#lookset)

- `fn write_repr(self: Self, slice: &mut [u8])`

- `fn available(self: Self) -> Result<(), UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for LookSet`

- `fn clone(self: &Self) -> LookSet` — [`LookSet`](#lookset)

##### `impl Copy for LookSet`

##### `impl Debug for LookSet`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for LookSet`

- `fn default() -> LookSet` — [`LookSet`](#lookset)

##### `impl Eq for LookSet`

##### `impl PartialEq for LookSet`

- `fn eq(self: &Self, other: &LookSet) -> bool` — [`LookSet`](#lookset)

##### `impl StructuralPartialEq for LookSet`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Clone for LookSetIter`

- `fn clone(self: &Self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

##### `impl Debug for LookSetIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for LookSetIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for LookSetIter`

- `type Item = Look`

- `fn next(self: &mut Self) -> Option<Look>` — [`Look`](#look)

### `LookMatcher`

```rust
struct LookMatcher {
    lineterm: crate::util::escape::DebugByte,
}
```

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

- `fn new() -> LookMatcher` — [`LookMatcher`](#lookmatcher)

- `fn set_line_terminator(self: &mut Self, byte: u8) -> &mut LookMatcher` — [`LookMatcher`](#lookmatcher)

- `fn get_line_terminator(self: &Self) -> u8`

- `fn matches(self: &Self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

- `fn matches_inline(self: &Self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

- `fn matches_set(self: &Self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

- `fn matches_set_inline(self: &Self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

- `fn add_to_byteset(self: &Self, look: Look, set: &mut crate::util::alphabet::ByteClassSet)` — [`Look`](#look), [`ByteClassSet`](../alphabet/index.md)

- `fn is_start(self: &Self, _haystack: &[u8], at: usize) -> bool`

- `fn is_end(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_start_lf(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_end_lf(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_start_crlf(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_end_crlf(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_ascii_negate(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- `fn is_word_unicode_negate(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- `fn is_word_start_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_end_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_start_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- `fn is_word_end_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- `fn is_word_start_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_end_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn is_word_start_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

- `fn is_word_end_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for LookMatcher`

- `fn clone(self: &Self) -> LookMatcher` — [`LookMatcher`](#lookmatcher)

##### `impl Debug for LookMatcher`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LookMatcher`

- `fn default() -> LookMatcher` — [`LookMatcher`](#lookmatcher)

### `UnicodeWordBoundaryError`

```rust
struct UnicodeWordBoundaryError(());
```

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. The `\w` class is used to
determine whether a codepoint is considered a word character or not when
determining whether a Unicode aware `\b` (or `\B`) matches at a particular
position.

This error can only occur when the `unicode-word-boundary` feature is
disabled.

#### Implementations

- `fn check() -> Result<(), UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

#### Trait Implementations

##### `impl Clone for UnicodeWordBoundaryError`

- `fn clone(self: &Self) -> UnicodeWordBoundaryError` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

##### `impl Debug for UnicodeWordBoundaryError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for UnicodeWordBoundaryError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for UnicodeWordBoundaryError`

##### `impl<T> ToString for UnicodeWordBoundaryError`

- `fn to_string(self: &Self) -> String`

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

- `const fn reversed(self: Self) -> Look` — [`Look`](#look)

- `const fn as_repr(self: Self) -> u32`

- `const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](#look)

- `const fn as_char(self: Self) -> char`

#### Trait Implementations

##### `impl Clone for Look`

- `fn clone(self: &Self) -> Look` — [`Look`](#look)

##### `impl Copy for Look`

##### `impl Debug for Look`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Look`

##### `impl PartialEq for Look`

- `fn eq(self: &Self, other: &Look) -> bool` — [`Look`](#look)

##### `impl StructuralPartialEq for Look`

