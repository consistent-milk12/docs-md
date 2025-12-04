*[regex_automata](../../index.md) / [util](../index.md) / [look](index.md)*

---

# Module `look`

Types and routines for working with look-around assertions.

This module principally defines two types:

* [`Look`](regex_automata/util/look/index.md) enumerates all of the assertions supported by this crate.
* [`LookSet`](regex_automata/util/look/index.md) provides a way to efficiently store a set of [`Look`](regex_automata/util/look/index.md) values.
* [`LookMatcher`](regex_automata/util/look/index.md) provides routines for checking whether a `Look` or a
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

- `fn empty() -> LookSet`
  Create an empty set of look-around assertions.

- `fn full() -> LookSet`
  Create a full set of look-around assertions.

- `fn singleton(look: Look) -> LookSet`
  Create a look-around set containing the look-around assertion given.

- `fn len(self: Self) -> usize`
  Returns the total number of look-around assertions in this set.

- `fn is_empty(self: Self) -> bool`
  Returns true if and only if this set is empty.

- `fn contains(self: Self, look: Look) -> bool`
  Returns true if and only if the given look-around assertion is in this

- `fn contains_anchor(self: &Self) -> bool`
  Returns true if and only if this set contains any anchor assertions.

- `fn contains_anchor_haystack(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of

- `fn contains_anchor_line(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_anchor_lf(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_anchor_crlf(self: &Self) -> bool`
  Returns true if and only if this set contains any "start/end of line"

- `fn contains_word(self: Self) -> bool`
  Returns true if and only if this set contains any word boundary or

- `fn contains_word_unicode(self: Self) -> bool`
  Returns true if and only if this set contains any Unicode word boundary

- `fn contains_word_ascii(self: Self) -> bool`
  Returns true if and only if this set contains any ASCII word boundary

- `fn iter(self: Self) -> LookSetIter`
  Returns an iterator over all of the look-around assertions in this set.

- `fn insert(self: Self, look: Look) -> LookSet`
  Return a new set that is equivalent to the original, but with the given

- `fn set_insert(self: &mut Self, look: Look)`
  Updates this set in place with the result of inserting the given

- `fn remove(self: Self, look: Look) -> LookSet`
  Return a new set that is equivalent to the original, but with the given

- `fn set_remove(self: &mut Self, look: Look)`
  Updates this set in place with the result of removing the given

- `fn subtract(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the result of subtracting the given set from

- `fn set_subtract(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of subtracting the given set

- `fn union(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the union of this and the one given.

- `fn set_union(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of unioning it with the one

- `fn intersect(self: Self, other: LookSet) -> LookSet`
  Returns a new set that is the intersection of this and the one given.

- `fn set_intersect(self: &mut Self, other: LookSet)`
  Updates this set in place with the result of intersecting it with the

- `fn read_repr(slice: &[u8]) -> LookSet`
  Return a `LookSet` from the slice given as a native endian 32-bit

- `fn write_repr(self: Self, slice: &mut [u8])`
  Write a `LookSet` as a native endian 32-bit integer to the beginning

- `fn available(self: Self) -> Result<(), UnicodeWordBoundaryError>`
  Checks that all assertions in this set can be matched.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> LookSet`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LookSet) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> LookSet`

### `LookSetIter`

```rust
struct LookSetIter {
}
```

An iterator over all look-around assertions in a [`LookSet`](regex_automata/util/look/index.md).

This iterator is created by [`LookSet::iter`](#iter).

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> LookSetIter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator`

- `type Item = Look`

- `fn next(self: &mut Self) -> Option<Look>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LookMatcher`

```rust
struct LookMatcher {
}
```

A matcher for look-around assertions.

This matcher permits configuring aspects of how look-around assertions are
matched.

# Example

A `LookMatcher` can change the line terminator used for matching multi-line
anchors such as `(?m:^)` and `(?m:$)`.

```
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

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> LookMatcher`
  Creates a new default matcher for look-around assertions.

- `fn set_line_terminator(self: &mut Self, byte: u8) -> &mut LookMatcher`
  Sets the line terminator for use with `(?m:^)` and `(?m:$)`.

- `fn get_line_terminator(self: &Self) -> u8`
  Returns the line terminator that was configured for this matcher.

- `fn matches(self: &Self, look: Look, haystack: &[u8], at: usize) -> bool`
  Returns true when the position `at` in `haystack` satisfies the given

- `fn matches_set(self: &Self, set: LookSet, haystack: &[u8], at: usize) -> bool`
  Returns true when _all_ of the assertions in the given set match at the

- `fn is_start(self: &Self, _haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::Start`] is satisfied `at` the given position

- `fn is_end(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::End`] is satisfied `at` the given position in

- `fn is_start_lf(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::StartLF`] is satisfied `at` the given

- `fn is_end_lf(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::EndLF`] is satisfied `at` the given position

- `fn is_start_crlf(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::StartCRLF`] is satisfied `at` the given

- `fn is_end_crlf(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::EndCRLF`] is satisfied `at` the given

- `fn is_word_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordAscii`] is satisfied `at` the given

- `fn is_word_ascii_negate(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordAsciiNegate`] is satisfied `at` the given

- `fn is_word_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordUnicode`] is satisfied `at` the given

- `fn is_word_unicode_negate(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordUnicodeNegate`] is satisfied `at` the

- `fn is_word_start_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordStartAscii`] is satisfied `at` the given

- `fn is_word_end_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordEndAscii`] is satisfied `at` the given

- `fn is_word_start_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordStartUnicode`] is satisfied `at` the

- `fn is_word_end_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordEndUnicode`] is satisfied `at` the

- `fn is_word_start_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordStartHalfAscii`] is satisfied `at` the

- `fn is_word_end_half_ascii(self: &Self, haystack: &[u8], at: usize) -> bool`
  Returns true when [`Look::WordEndHalfAscii`] is satisfied `at` the

- `fn is_word_start_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordStartHalfUnicode`] is satisfied `at` the

- `fn is_word_end_half_unicode(self: &Self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>`
  Returns true when [`Look::WordEndHalfUnicode`] is satisfied `at` the

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> LookMatcher`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> LookMatcher`

### `UnicodeWordBoundaryError`

```rust
struct UnicodeWordBoundaryError();
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

- `fn check() -> Result<(), UnicodeWordBoundaryError>`
  Returns an error if and only if Unicode word boundary data is

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> UnicodeWordBoundaryError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `const fn reversed(self: Self) -> Look`
  Flip the look-around assertion to its equivalent for reverse searches.

- `const fn as_repr(self: Self) -> u32`
  Return the underlying representation of this look-around enumeration

- `const fn from_repr(repr: u32) -> Option<Look>`
  Given the underlying representation of a `Look` value, return the

- `const fn as_char(self: Self) -> char`
  Returns a convenient single codepoint representation of this

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Look`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Look) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

