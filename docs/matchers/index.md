# Crate `matchers`

Regex matchers on character and byte streams.

## Overview

The [`regex`](#regex) crate implements regular expression matching on strings and byte
arrays. However, in order to match the output of implementations of `fmt::Debug`
and `fmt::Display`, or by any code which writes to an instance of `fmt::Write`
or `io::Write`, it is necessary to first allocate a buffer, write to that
buffer, and then match the buffer against a regex.

In cases where it is not necessary to extract substrings, but only to test whether
or not output matches a regex, it is not strictly necessary to allocate and
write this output to a buffer. This crate provides a simple interface on top of
the lower-level `regex-automata` library that implements `fmt::Write` and
`io::Write` for regex patterns. This may be used to test whether streaming
output matches a pattern without buffering that output.

Users who need to extract substrings based on a pattern or who already have
buffered data should probably use the [`regex`](#regex) crate instead.

## Syntax

This crate uses the same [regex syntax][syntax] of the `regex-automata` crate.




## Modules

- [`BuildError`](BuildError/index.md) - 

## Structs

### `Pattern<A>`

```rust
struct Pattern<A> {
    automaton: A,
    anchored: regex_automata::Anchored,
}
```

A compiled match pattern that can match multipe inputs, or return a
[`Matcher`](#matcher) that matches a single input.


#### Implementations

- `fn matcher(self: &Self) -> Matcher<&A>` — [`Matcher`](#matcher)

- `fn matches(self: &Self, s: &impl AsRef<str>) -> bool`

- `fn debug_matches(self: &Self, d: &impl fmt::Debug) -> bool`

- `fn display_matches(self: &Self, d: &impl fmt::Display) -> bool`

- `fn read_matches(self: &Self, io: impl io::Read) -> io::Result<bool>`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for Pattern<A>`

- `fn clone(self: &Self) -> Pattern<A>` — [`Pattern`](#pattern)

##### `impl<A: $crate::fmt::Debug> Debug for Pattern<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FromStr for Pattern`

- `type Err = BuildError`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

### `Matcher<A>`

```rust
struct Matcher<A> {
    automaton: A,
    state: regex_automata::util::primitives::StateID,
}
```

A reference to a [`Pattern`](#pattern) that matches a single input.


#### Implementations

- `fn advance(self: &mut Self, input: u8)`

- `fn is_matched(self: &Self) -> bool`

- `fn matches(self: Self, s: &impl AsRef<str>) -> bool`

- `fn debug_matches(self: Self, d: &impl fmt::Debug) -> bool`

- `fn display_matches(self: Self, d: &impl fmt::Display) -> bool`

- `fn read_matches(self: Self, io: impl io::Read + Sized) -> io::Result<bool>`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for Matcher<A>`

- `fn clone(self: &Self) -> Matcher<A>` — [`Matcher`](#matcher)

##### `impl<A: $crate::fmt::Debug> Debug for Matcher<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A: Automaton> Write for Matcher<A>`

- `fn write(self: &mut Self, bytes: &[u8]) -> Result<usize, io::Error>`

- `fn flush(self: &mut Self) -> Result<(), io::Error>`

