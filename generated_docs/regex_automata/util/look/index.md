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
| [`is_word_char`](#is-word-char) | mod | A module that looks for word codepoints using regex-syntax's data tables. |
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

*Defined in [`regex-automata-0.4.13/src/util/look.rs:244-255`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/look.rs#L244-L255)*

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

  Create an empty set of look-around assertions.

- <span id="lookset-full"></span>`fn full() -> LookSet` — [`LookSet`](#lookset)

  Create a full set of look-around assertions.

  

  This set contains all possible look-around assertions.

- <span id="lookset-singleton"></span>`fn singleton(look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Create a look-around set containing the look-around assertion given.

  

  This is a convenience routine for creating an empty set and inserting

  one look-around assertions.

- <span id="lookset-len"></span>`fn len(self) -> usize`

  Returns the total number of look-around assertions in this set.

- <span id="lookset-is-empty"></span>`fn is_empty(self) -> bool`

  Returns true if and only if this set is empty.

- <span id="lookset-contains"></span>`fn contains(self, look: Look) -> bool` — [`Look`](#look)

  Returns true if and only if the given look-around assertion is in this

  set.

- <span id="lookset-contains-anchor"></span>`fn contains_anchor(&self) -> bool`

  Returns true if and only if this set contains any anchor assertions.

  This includes both "start/end of haystack" and "start/end of line."

- <span id="lookset-contains-anchor-haystack"></span>`fn contains_anchor_haystack(&self) -> bool`

  Returns true if and only if this set contains any "start/end of

  haystack" anchors. This doesn't include "start/end of line" anchors.

- <span id="lookset-contains-anchor-line"></span>`fn contains_anchor_line(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors. This doesn't include "start/end of haystack" anchors. This

  includes both `\n` line anchors and CRLF (`\r\n`) aware line anchors.

- <span id="lookset-contains-anchor-lf"></span>`fn contains_anchor_lf(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors that only treat `\n` as line terminators. This does not include

  haystack anchors or CRLF aware line anchors.

- <span id="lookset-contains-anchor-crlf"></span>`fn contains_anchor_crlf(&self) -> bool`

  Returns true if and only if this set contains any "start/end of line"

  anchors that are CRLF-aware. This doesn't include "start/end of

  haystack" or "start/end of line-feed" anchors.

- <span id="lookset-contains-word"></span>`fn contains_word(self) -> bool`

  Returns true if and only if this set contains any word boundary or

  negated word boundary assertions. This include both Unicode and ASCII

  word boundaries.

- <span id="lookset-contains-word-unicode"></span>`fn contains_word_unicode(self) -> bool`

  Returns true if and only if this set contains any Unicode word boundary

  or negated Unicode word boundary assertions.

- <span id="lookset-contains-word-ascii"></span>`fn contains_word_ascii(self) -> bool`

  Returns true if and only if this set contains any ASCII word boundary

  or negated ASCII word boundary assertions.

- <span id="lookset-iter"></span>`fn iter(self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

  Returns an iterator over all of the look-around assertions in this set.

- <span id="lookset-insert"></span>`fn insert(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Return a new set that is equivalent to the original, but with the given

  assertion added to it. If the assertion is already in the set, then the

  returned set is equivalent to the original.

- <span id="lookset-set-insert"></span>`fn set_insert(&mut self, look: Look)` — [`Look`](#look)

  Updates this set in place with the result of inserting the given

  assertion into this set.

- <span id="lookset-remove"></span>`fn remove(self, look: Look) -> LookSet` — [`Look`](#look), [`LookSet`](#lookset)

  Return a new set that is equivalent to the original, but with the given

  assertion removed from it. If the assertion is not in the set, then the

  returned set is equivalent to the original.

- <span id="lookset-set-remove"></span>`fn set_remove(&mut self, look: Look)` — [`Look`](#look)

  Updates this set in place with the result of removing the given

  assertion from this set.

- <span id="lookset-subtract"></span>`fn subtract(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the result of subtracting the given set from

  this set.

- <span id="lookset-set-subtract"></span>`fn set_subtract(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of subtracting the given set

  from this set.

- <span id="lookset-union"></span>`fn union(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the union of this and the one given.

- <span id="lookset-set-union"></span>`fn set_union(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of unioning it with the one

  given.

- <span id="lookset-intersect"></span>`fn intersect(self, other: LookSet) -> LookSet` — [`LookSet`](#lookset)

  Returns a new set that is the intersection of this and the one given.

- <span id="lookset-set-intersect"></span>`fn set_intersect(&mut self, other: LookSet)` — [`LookSet`](#lookset)

  Updates this set in place with the result of intersecting it with the

  one given.

- <span id="lookset-read-repr"></span>`fn read_repr(slice: &[u8]) -> LookSet` — [`LookSet`](#lookset)

  Return a `LookSet` from the slice given as a native endian 32-bit

  integer.

  

  # Panics

  

  This panics if `slice.len() < 4`.

- <span id="lookset-write-repr"></span>`fn write_repr(self, slice: &mut [u8])`

  Write a `LookSet` as a native endian 32-bit integer to the beginning

  of the slice given.

  

  # Panics

  

  This panics if `slice.len() < 4`.

- <span id="lookset-available"></span>`fn available(self) -> Result<(), UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Checks that all assertions in this set can be matched.

  

  Some assertions, such as Unicode word boundaries, require optional (but

  enabled by default) tables that may not be available. If there are

  assertions in this set that require tables that are not available, then

  this will return an error.

  

  Specifically, this returns an error when the

  `unicode-word-boundary` feature is _not_ enabled _and_ this set

  contains a Unicode word boundary assertion.

  

  It can be useful to use this on the result of

  [`NFA::look_set_any`](crate::nfa::thompson::NFA::look_set_any)

  when building a matcher engine to ensure methods like

  `LookMatcher::matches_set` do not panic at search time.

#### Trait Implementations

##### `impl Any for LookSet`

- <span id="lookset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookSet`

- <span id="lookset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookSet`

- <span id="lookset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LookSet`

- <span id="lookset-clone"></span>`fn clone(&self) -> LookSet` — [`LookSet`](#lookset)

##### `impl CloneToUninit for LookSet`

- <span id="lookset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LookSet`

##### `impl Debug for LookSet`

- <span id="lookset-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for LookSet`

- <span id="lookset-default"></span>`fn default() -> LookSet` — [`LookSet`](#lookset)

##### `impl Eq for LookSet`

##### `impl<T> From for LookSet`

- <span id="lookset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookSet`

- <span id="lookset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for LookSet`

- <span id="lookset-partialeq-eq"></span>`fn eq(&self, other: &LookSet) -> bool` — [`LookSet`](#lookset)

##### `impl StructuralPartialEq for LookSet`

##### `impl ToOwned for LookSet`

- <span id="lookset-toowned-type-owned"></span>`type Owned = T`

- <span id="lookset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lookset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookSet`

- <span id="lookset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookSet`

- <span id="lookset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LookSetIter`

```rust
struct LookSetIter {
    set: LookSet,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:517-519`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/look.rs#L517-L519)*

An iterator over all look-around assertions in a [`LookSet`](#lookset).

This iterator is created by `LookSet::iter`.

#### Trait Implementations

##### `impl Any for LookSetIter`

- <span id="looksetiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookSetIter`

- <span id="looksetiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookSetIter`

- <span id="looksetiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LookSetIter`

- <span id="looksetiter-clone"></span>`fn clone(&self) -> LookSetIter` — [`LookSetIter`](#looksetiter)

##### `impl CloneToUninit for LookSetIter`

- <span id="looksetiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LookSetIter`

- <span id="looksetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LookSetIter`

- <span id="looksetiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookSetIter`

- <span id="looksetiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LookSetIter`

- <span id="looksetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="looksetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="looksetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LookSetIter`

- <span id="looksetiter-iterator-type-item"></span>`type Item = Look`

- <span id="looksetiter-iterator-next"></span>`fn next(&mut self) -> Option<Look>` — [`Look`](#look)

##### `impl ToOwned for LookSetIter`

- <span id="looksetiter-toowned-type-owned"></span>`type Owned = T`

- <span id="looksetiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="looksetiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookSetIter`

- <span id="looksetiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="looksetiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookSetIter`

- <span id="looksetiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="looksetiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LookMatcher`

```rust
struct LookMatcher {
    lineterm: crate::util::escape::DebugByte,
}
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:577-579`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/look.rs#L577-L579)*

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

  Creates a new default matcher for look-around assertions.

- <span id="lookmatcher-set-line-terminator"></span>`fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher` — [`LookMatcher`](#lookmatcher)

  Sets the line terminator for use with `(?m:^)` and `(?m:$)`.

  

  Namely, instead of `^` matching after `\n` and `$` matching immediately

  before a `\n`, this will cause it to match after and before the byte

  given.

  

  It can occasionally be useful to use this to configure the line

  terminator to the NUL byte when searching binary data.

  

  Note that this does not apply to CRLF-aware line anchors such as

  `(?Rm:^)` and `(?Rm:$)`. CRLF-aware line anchors are hard-coded to

  use `\r` and `\n`.

- <span id="lookmatcher-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

  Returns the line terminator that was configured for this matcher.

  

  If no line terminator was configured, then this returns `\n`.

  

  Note that the line terminator should only be used for matching `(?m:^)`

  and `(?m:$)` assertions. It specifically should _not_ be used for

  matching the CRLF aware assertions `(?Rm:^)` and `(?Rm:$)`.

- <span id="lookmatcher-matches"></span>`fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

  Returns true when the position `at` in `haystack` satisfies the given

  look-around assertion.

  

  # Panics

  

  This panics when testing any Unicode word boundary assertion in this

  set and when the Unicode word data is not available. Specifically, this

  only occurs when the `unicode-word-boundary` feature is not enabled.

  

  Since it's generally expected that this routine is called inside of

  a matching engine, callers should check the error condition when

  building the matching engine. If there is a Unicode word boundary

  in the matcher and the data isn't available, then the matcher should

  fail to build.

  

  Callers can check the error condition with `LookSet::available`.

  

  This also may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-matches-inline"></span>`fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool` — [`Look`](#look)

  Like `matches`, but forcefully inlined.

  

  # Panics

  

  This panics when testing any Unicode word boundary assertion in this

  set and when the Unicode word data is not available. Specifically, this

  only occurs when the `unicode-word-boundary` feature is not enabled.

  

  Since it's generally expected that this routine is called inside of

  a matching engine, callers should check the error condition when

  building the matching engine. If there is a Unicode word boundary

  in the matcher and the data isn't available, then the matcher should

  fail to build.

  

  Callers can check the error condition with `LookSet::available`.

  

  This also may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-matches-set"></span>`fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

  Returns true when _all_ of the assertions in the given set match at the

  given position in the haystack.

  

  # Panics

  

  This panics when testing any Unicode word boundary assertion in this

  set and when the Unicode word data is not available. Specifically, this

  only occurs when the `unicode-word-boundary` feature is not enabled.

  

  Since it's generally expected that this routine is called inside of

  a matching engine, callers should check the error condition when

  building the matching engine. If there is a Unicode word boundary

  in the matcher and the data isn't available, then the matcher should

  fail to build.

  

  Callers can check the error condition with `LookSet::available`.

  

  This also may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-matches-set-inline"></span>`fn matches_set_inline(&self, set: LookSet, haystack: &[u8], at: usize) -> bool` — [`LookSet`](#lookset)

  Like `LookSet::matches`, but forcefully inlined for perf.

- <span id="lookmatcher-add-to-byteset"></span>`fn add_to_byteset(&self, look: Look, set: &mut crate::util::alphabet::ByteClassSet)` — [`Look`](#look), [`ByteClassSet`](../alphabet/index.md#byteclassset)

  Split up the given byte classes into equivalence classes in a way that

  is consistent with this look-around assertion.

- <span id="lookmatcher-is-start"></span>`fn is_start(&self, _haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::Start`](../../index.md) is satisfied `at` the given position

  in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-end"></span>`fn is_end(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::End`](../../index.md) is satisfied `at` the given position in

  `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-start-lf"></span>`fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::StartLF`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-end-lf"></span>`fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::EndLF`](../../index.md) is satisfied `at` the given position

  in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-start-crlf"></span>`fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::StartCRLF`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-end-crlf"></span>`fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::EndCRLF`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-ascii"></span>`fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordAscii`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-ascii-negate"></span>`fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordAsciiNegate`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-unicode"></span>`fn is_word_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordUnicode`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

- <span id="lookmatcher-is-word-unicode-negate"></span>`fn is_word_unicode_negate(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordUnicodeNegate`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

- <span id="lookmatcher-is-word-start-ascii"></span>`fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordStartAscii`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-end-ascii"></span>`fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordEndAscii`](../../index.md) is satisfied `at` the given

  position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-start-unicode"></span>`fn is_word_start_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordStartUnicode`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

- <span id="lookmatcher-is-word-end-unicode"></span>`fn is_word_end_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordEndUnicode`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

- <span id="lookmatcher-is-word-start-half-ascii"></span>`fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordStartHalfAscii`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-end-half-ascii"></span>`fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool`

  Returns true when [`Look::WordEndHalfAscii`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

- <span id="lookmatcher-is-word-start-half-unicode"></span>`fn is_word_start_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordStartHalfUnicode`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

- <span id="lookmatcher-is-word-end-half-unicode"></span>`fn is_word_end_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

  Returns true when [`Look::WordEndHalfUnicode`](../../index.md) is satisfied `at` the

  given position in `haystack`.

  

  # Panics

  

  This may panic when `at > haystack.len()`. Note that `at ==

  haystack.len()` is legal and guaranteed not to panic.

  

  # Errors

  

  This returns an error when Unicode word boundary tables

  are not available. Specifically, this only occurs when the

  `unicode-word-boundary` feature is not enabled.

#### Trait Implementations

##### `impl Any for LookMatcher`

- <span id="lookmatcher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LookMatcher`

- <span id="lookmatcher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LookMatcher`

- <span id="lookmatcher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LookMatcher`

- <span id="lookmatcher-clone"></span>`fn clone(&self) -> LookMatcher` — [`LookMatcher`](#lookmatcher)

##### `impl CloneToUninit for LookMatcher`

- <span id="lookmatcher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for LookMatcher`

- <span id="lookmatcher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LookMatcher`

- <span id="lookmatcher-default"></span>`fn default() -> LookMatcher` — [`LookMatcher`](#lookmatcher)

##### `impl<T> From for LookMatcher`

- <span id="lookmatcher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LookMatcher`

- <span id="lookmatcher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for LookMatcher`

- <span id="lookmatcher-toowned-type-owned"></span>`type Owned = T`

- <span id="lookmatcher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lookmatcher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LookMatcher`

- <span id="lookmatcher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lookmatcher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LookMatcher`

- <span id="lookmatcher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lookmatcher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnicodeWordBoundaryError`

```rust
struct UnicodeWordBoundaryError(());
```

*Defined in [`regex-automata-0.4.13/src/util/look.rs:1280`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/look.rs#L1280)*

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

  Returns an error if and only if Unicode word boundary data is

  unavailable.

#### Trait Implementations

##### `impl Any for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-clone"></span>`fn clone(&self) -> UnicodeWordBoundaryError` — [`UnicodeWordBoundaryError`](#unicodewordboundaryerror)

##### `impl CloneToUninit for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for UnicodeWordBoundaryError`

##### `impl<T> From for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-toowned-type-owned"></span>`type Owned = T`

- <span id="unicodewordboundaryerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unicodewordboundaryerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unicodewordboundaryerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnicodeWordBoundaryError`

- <span id="unicodewordboundaryerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unicodewordboundaryerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/util/look.rs:62-135`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/look.rs#L62-L135)*

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

  Flip the look-around assertion to its equivalent for reverse searches.

  For example, `StartLF` gets translated to `EndLF`.

  

  Some assertions, such as `WordUnicode`, remain the same since they

  match the same positions regardless of the direction of the search.

- <span id="look-as-repr"></span>`const fn as_repr(self) -> u32`

  Return the underlying representation of this look-around enumeration

  as an integer. Giving the return value to the `Look::from_repr`

  constructor is guaranteed to return the same look-around variant that

  one started with within a semver compatible release of this crate.

- <span id="look-from-repr"></span>`const fn from_repr(repr: u32) -> Option<Look>` — [`Look`](#look)

  Given the underlying representation of a `Look` value, return the

  corresponding `Look` value if the representation is valid. Otherwise

  `None` is returned.

- <span id="look-as-char"></span>`const fn as_char(self) -> char`

  Returns a convenient single codepoint representation of this

  look-around assertion. Each assertion is guaranteed to be represented

  by a distinct character.

  

  This is useful for succinctly representing a look-around assertion in

  human friendly but succinct output intended for a programmer working on

  regex internals.

#### Trait Implementations

##### `impl Any for Look`

- <span id="look-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Look`

- <span id="look-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Look`

- <span id="look-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Look`

- <span id="look-clone"></span>`fn clone(&self) -> Look` — [`Look`](#look)

##### `impl CloneToUninit for Look`

- <span id="look-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Look`

##### `impl Debug for Look`

- <span id="look-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Look`

##### `impl<T> From for Look`

- <span id="look-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Look`

- <span id="look-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Look`

- <span id="look-partialeq-eq"></span>`fn eq(&self, other: &Look) -> bool` — [`Look`](#look)

##### `impl StructuralPartialEq for Look`

##### `impl ToOwned for Look`

- <span id="look-toowned-type-owned"></span>`type Owned = T`

- <span id="look-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="look-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Look`

- <span id="look-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="look-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Look`

- <span id="look-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="look-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

