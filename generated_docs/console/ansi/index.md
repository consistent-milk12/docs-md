*[console](../index.md) / [ansi](index.md)*

---

# Module `ansi`

## Contents

- [Structs](#structs)
  - [`Matches`](#matches)
  - [`Match`](#match)
  - [`WithoutAnsi`](#withoutansi)
  - [`AnsiCodeIterator`](#ansicodeiterator)
- [Enums](#enums)
  - [`State`](#state)
- [Functions](#functions)
  - [`find_ansi_code_exclusive`](#find-ansi-code-exclusive)
  - [`strip_ansi_codes`](#strip-ansi-codes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Matches`](#matches) | struct |  |
| [`Match`](#match) | struct |  |
| [`WithoutAnsi`](#withoutansi) | struct | A wrapper struct that implements [`core::fmt::Display`], only displaying non-ansi parts. |
| [`AnsiCodeIterator`](#ansicodeiterator) | struct | An iterator over ansi codes in a string. |
| [`State`](#state) | enum |  |
| [`find_ansi_code_exclusive`](#find-ansi-code-exclusive) | fn |  |
| [`strip_ansi_codes`](#strip-ansi-codes) | fn | Helper function to strip ansi codes. |

## Structs

### `Matches<'a>`

```rust
struct Matches<'a> {
    s: &'a str,
    it: core::iter::Peekable<core::str::CharIndices<'a>>,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:109-112`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L109-L112)*

#### Implementations

- <span id="matches-new"></span>`fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl Any for Matches<'a>`

- <span id="matches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Matches<'a>`

- <span id="matches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Matches<'a>`

- <span id="matches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Matches<'a>`

- <span id="matches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Matches<'a>`

- <span id="matches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Matches<'_>`

##### `impl<U> Into for Matches<'a>`

- <span id="matches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Matches<'a>`

- <span id="matches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="matches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Matches<'a>`

- <span id="matches-iterator-type-item"></span>`type Item = Match<'a>`

- <span id="matches-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Matches<'a>`

- <span id="matches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Matches<'a>`

- <span id="matches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match<'a>`

```rust
struct Match<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:122-126`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L122-L126)*

#### Implementations

- <span id="match-as-str"></span>`fn as_str(&self) -> &'a str`

#### Trait Implementations

##### `impl Any for Match<'a>`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match<'a>`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match<'a>`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Match<'a>`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Match<'a>`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Match<'a>`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Match<'a>`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match<'a>`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:206-208`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L206-L208)*

A wrapper struct that implements [`core::fmt::Display`](../../miette_derive/index.md), only displaying non-ansi parts.

#### Implementations

- <span id="withoutansi-new"></span>`fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Any for WithoutAnsi<'a>`

- <span id="withoutansi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithoutAnsi<'a>`

- <span id="withoutansi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithoutAnsi<'a>`

- <span id="withoutansi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for WithoutAnsi<'_>`

- <span id="withoutansi-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for WithoutAnsi<'a>`

- <span id="withoutansi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithoutAnsi<'a>`

- <span id="withoutansi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for WithoutAnsi<'a>`

- <span id="withoutansi-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for WithoutAnsi<'a>`

- <span id="withoutansi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withoutansi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithoutAnsi<'a>`

- <span id="withoutansi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withoutansi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AnsiCodeIterator<'a>`

```rust
struct AnsiCodeIterator<'a> {
    s: &'a str,
    pending_item: Option<(&'a str, bool)>,
    last_idx: usize,
    cur_idx: usize,
    iter: Matches<'a>,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:233-239`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L233-L239)*

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- <span id="ansicodeiterator-new"></span>`fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](#ansicodeiterator)

  Creates a new ansi code iterator.

- <span id="ansicodeiterator-current-slice"></span>`fn current_slice(&self) -> &str`

  Returns the string slice up to the current match.

- <span id="ansicodeiterator-rest-slice"></span>`fn rest_slice(&self) -> &str`

  Returns the string slice from the current match to the end.

#### Trait Implementations

##### `impl Any for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl<U> Into for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ansicodeiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ansicodeiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-iterator-type-item"></span>`type Item = (&'a str, bool)`

- <span id="ansicodeiterator-iterator-next"></span>`fn next(&mut self) -> Option<(&'a str, bool)>`

##### `impl<U> TryFrom for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansicodeiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansicodeiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `State`

```rust
enum State {
    Start,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    Trap,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:10-24`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L10-L24)*

#### Implementations

- <span id="state-is-final"></span>`fn is_final(&self) -> bool`

- <span id="state-is-trapped"></span>`fn is_trapped(&self) -> bool`

- <span id="state-transition"></span>`fn transition(&mut self, c: char)`

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> Self`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `find_ansi_code_exclusive`

```rust
fn find_ansi_code_exclusive(it: &mut core::iter::Peekable<core::str::CharIndices<'_>>) -> Option<(usize, usize)>
```

*Defined in [`console-0.16.1/src/ansi.rs:149-188`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L149-L188)*

### `strip_ansi_codes`

```rust
fn strip_ansi_codes(s: &str) -> alloc::borrow::Cow<'_, str>
```

*Defined in [`console-0.16.1/src/ansi.rs:192-203`](../../../.source_1765633015/console-0.16.1/src/ansi.rs#L192-L203)*

Helper function to strip ansi codes.

