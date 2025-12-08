*[regex](../index.md) / [builders](index.md)*

---

# Module `builders`

## Modules

- [`string`](string/index.md) - 
- [`bytes`](bytes/index.md) - 

## Structs

### `Builder`

```rust
struct Builder {
    pats: alloc::vec::Vec<alloc::string::String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
}
```

A builder for constructing a `Regex`, `bytes::Regex`, `RegexSet` or a
`bytes::RegexSet`.

This is essentially the implementation of the four different builder types
in the public API: `RegexBuilder`, `bytes::RegexBuilder`, `RegexSetBuilder`
and `bytes::RegexSetBuilder`.

#### Implementations

- `fn new<I, S>(patterns: I) -> Builder` — [`Builder`](#builder)

- `fn build_one_string(self: &Self) -> Result<crate::Regex, Error>` — [`Regex`](../index.md), [`Error`](../index.md)

- `fn build_one_bytes(self: &Self) -> Result<crate::bytes::Regex, Error>` — [`Regex`](../regex/bytes/index.md), [`Error`](../index.md)

- `fn build_many_string(self: &Self) -> Result<crate::RegexSet, Error>` — [`RegexSet`](../index.md), [`Error`](../index.md)

- `fn build_many_bytes(self: &Self) -> Result<crate::bytes::RegexSet, Error>` — [`RegexSet`](../regexset/bytes/index.md), [`Error`](../index.md)

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn multi_line(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn crlf(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut Builder` — [`Builder`](#builder)

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn unicode(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn octal(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn size_limit(self: &mut Self, limit: usize) -> &mut Builder` — [`Builder`](#builder)

- `fn dfa_size_limit(self: &mut Self, limit: usize) -> &mut Builder` — [`Builder`](#builder)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

