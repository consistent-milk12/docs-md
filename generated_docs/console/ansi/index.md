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
  - [`find_ansi_code_exclusive`](#find_ansi_code_exclusive)
  - [`strip_ansi_codes`](#strip_ansi_codes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Matches`](#matches) | struct |  |
| [`Match`](#match) | struct |  |
| [`WithoutAnsi`](#withoutansi) | struct | A wrapper struct that implements [`core::fmt::Display`], only displaying non-ansi parts. |
| [`AnsiCodeIterator`](#ansicodeiterator) | struct | An iterator over ansi codes in a string. |
| [`State`](#state) | enum |  |
| [`find_ansi_code_exclusive`](#find_ansi_code_exclusive) | fn |  |
| [`strip_ansi_codes`](#strip_ansi_codes) | fn | Helper function to strip ansi codes. |

## Structs

### `Matches<'a>`

```rust
struct Matches<'a> {
    s: &'a str,
    it: core::iter::Peekable<core::str::CharIndices<'a>>,
}
```

#### Implementations

- <span id="matches-new"></span>`fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl<'a> Debug for Matches<'a>`

- <span id="matches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Matches<'_>`

##### `impl<I> IntoIterator for Matches<'a>`

- <span id="matches-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-intoiter"></span>`type IntoIter = I`

- <span id="matches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for Matches<'a>`

- <span id="matches-item"></span>`type Item = Match<'a>`

- <span id="matches-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Match<'a>`

```rust
struct Match<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}
```

#### Implementations

- <span id="match-as-str"></span>`fn as_str(&self) -> &'a str`

#### Trait Implementations

##### `impl<'a> Debug for Match<'a>`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

A wrapper struct that implements [`core::fmt::Display`](../../miette_derive/fmt/index.md), only displaying non-ansi parts.

#### Implementations

- <span id="withoutansi-new"></span>`fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Display for WithoutAnsi<'_>`

- <span id="withoutansi-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for WithoutAnsi<'a>`

- <span id="withoutansi-to-string"></span>`fn to_string(&self) -> String`

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

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- <span id="ansicodeiterator-new"></span>`fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](../index.md)

- <span id="ansicodeiterator-current-slice"></span>`fn current_slice(&self) -> &str`

- <span id="ansicodeiterator-rest-slice"></span>`fn rest_slice(&self) -> &str`

#### Trait Implementations

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl<I> IntoIterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ansicodeiterator-intoiter"></span>`type IntoIter = I`

- <span id="ansicodeiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-item"></span>`type Item = (&'a str, bool)`

- <span id="ansicodeiterator-next"></span>`fn next(&mut self) -> Option<(&'a str, bool)>`

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

#### Implementations

- <span id="state-is-final"></span>`fn is_final(&self) -> bool`

- <span id="state-is-trapped"></span>`fn is_trapped(&self) -> bool`

- <span id="state-transition"></span>`fn transition(&mut self, c: char)`

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> Self`

## Functions

### `find_ansi_code_exclusive`

```rust
fn find_ansi_code_exclusive(it: &mut core::iter::Peekable<core::str::CharIndices<'_>>) -> Option<(usize, usize)>
```

### `strip_ansi_codes`

```rust
fn strip_ansi_codes(s: &str) -> alloc::borrow::Cow<'_, str>
```

Helper function to strip ansi codes.

