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

*Defined in [`console-0.16.1/src/ansi.rs:109-112`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L109-L112)*

#### Implementations

- <span id="matches-new"></span>`fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl Debug for Matches<'a>`

- <span id="matches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Matches<'_>`

##### `impl IntoIterator for Matches<'a>`

- <span id="matches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="matches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Matches<'a>`

- <span id="matches-iterator-type-item"></span>`type Item = Match<'a>`

- <span id="matches-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Match<'a>`

```rust
struct Match<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:122-126`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L122-L126)*

#### Implementations

- <span id="match-as-str"></span>`fn as_str(&self) -> &'a str`

#### Trait Implementations

##### `impl Debug for Match<'a>`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

*Defined in [`console-0.16.1/src/ansi.rs:206-208`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L206-L208)*

A wrapper struct that implements [`core::fmt::Display`](../../miette_derive/fmt/index.md), only displaying non-ansi parts.

#### Implementations

- <span id="withoutansi-new"></span>`fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Display for WithoutAnsi<'_>`

- <span id="withoutansi-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for WithoutAnsi<'a>`

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

*Defined in [`console-0.16.1/src/ansi.rs:233-239`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L233-L239)*

An iterator over ansi codes in a string.

This type can be used to scan over ansi codes in a string.
It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
the original string and `is_ansi` indicates if the slice contains
ansi codes or string values.

#### Implementations

- <span id="ansicodeiterator-new"></span>`fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](#ansicodeiterator)

- <span id="ansicodeiterator-current-slice"></span>`fn current_slice(&self) -> &str`

- <span id="ansicodeiterator-rest-slice"></span>`fn rest_slice(&self) -> &str`

#### Trait Implementations

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl IntoIterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ansicodeiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ansicodeiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for AnsiCodeIterator<'a>`

- <span id="ansicodeiterator-iterator-type-item"></span>`type Item = (&'a str, bool)`

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

*Defined in [`console-0.16.1/src/ansi.rs:10-24`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L10-L24)*

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

*Defined in [`console-0.16.1/src/ansi.rs:149-188`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L149-L188)*

### `strip_ansi_codes`

```rust
fn strip_ansi_codes(s: &str) -> alloc::borrow::Cow<'_, str>
```

*Defined in [`console-0.16.1/src/ansi.rs:192-203`](../../../.source_1765521767/console-0.16.1/src/ansi.rs#L192-L203)*

Helper function to strip ansi codes.

