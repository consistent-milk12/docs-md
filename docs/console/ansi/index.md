*[console](../index.md) / [ansi](index.md)*

---

# Module `ansi`

## Structs

### `Matches<'a>`

```rust
struct Matches<'a> {
    s: &'a str,
    it: core::iter::Peekable<core::str::CharIndices<'a>>,
}
```

#### Implementations

- `fn new(s: &'a str) -> Self`

#### Trait Implementations

##### `impl<'a> Debug for Matches<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FusedIterator for Matches<'_>`

##### `impl<I> IntoIterator for Matches<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Matches<'a>`

- `type Item = Match<'a>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Match<'a>`

```rust
struct Match<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}
```

#### Implementations

- `fn as_str(self: &Self) -> &'a str`

#### Trait Implementations

##### `impl<'a> Debug for Match<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `WithoutAnsi<'a>`

```rust
struct WithoutAnsi<'a> {
    str: &'a str,
}
```

A wrapper struct that implements [`core::fmt::Display`](../../miette_derive/index.md), only displaying non-ansi parts.

#### Implementations

- `fn new(str: &'a str) -> Self`

#### Trait Implementations

##### `impl Display for WithoutAnsi<'_>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for WithoutAnsi<'a>`

- `fn to_string(self: &Self) -> String`

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

- `fn new(s: &'a str) -> AnsiCodeIterator<'a>` — [`AnsiCodeIterator`](../index.md)

- `fn current_slice(self: &Self) -> &str`

- `fn rest_slice(self: &Self) -> &str`

#### Trait Implementations

##### `impl FusedIterator for AnsiCodeIterator<'_>`

##### `impl<I> IntoIterator for AnsiCodeIterator<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for AnsiCodeIterator<'a>`

- `type Item = (&'a str, bool)`

- `fn next(self: &mut Self) -> Option<(&'a str, bool)>`

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

- `fn is_final(self: &Self) -> bool`

- `fn is_trapped(self: &Self) -> bool`

- `fn transition(self: &mut Self, c: char)`

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for State`

- `fn default() -> Self`

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

