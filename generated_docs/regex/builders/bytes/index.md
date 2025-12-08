*[regex](../../index.md) / [builders](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Structs

### `RegexBuilder`

```rust
struct RegexBuilder {
    builder: super::Builder,
}
```

A configurable builder for a [`Regex`](../../regex/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- `fn new(pattern: &str) -> RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn build(self: &Self) -> Result<Regex, Error>` — [`Regex`](../../regex/bytes/index.md), [`Error`](../../index.md)

- `fn unicode(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn multi_line(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn crlf(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn octal(self: &mut Self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn size_limit(self: &mut Self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn dfa_size_limit(self: &mut Self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

#### Trait Implementations

##### `impl Clone for RegexBuilder`

- `fn clone(self: &Self) -> RegexBuilder` — [`RegexBuilder`](../../bytes/index.md)

##### `impl Debug for RegexBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RegexSetBuilder`

```rust
struct RegexSetBuilder {
    builder: super::Builder,
}
```

A configurable builder for a [`RegexSet`](../../regexset/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- `fn new<I, S>(patterns: I) -> RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn build(self: &Self) -> Result<RegexSet, Error>` — [`RegexSet`](../../regexset/bytes/index.md), [`Error`](../../index.md)

- `fn unicode(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn multi_line(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn crlf(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn octal(self: &mut Self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn size_limit(self: &mut Self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn dfa_size_limit(self: &mut Self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

#### Trait Implementations

##### `impl Clone for RegexSetBuilder`

- `fn clone(self: &Self) -> RegexSetBuilder` — [`RegexSetBuilder`](../../bytes/index.md)

##### `impl Debug for RegexSetBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

