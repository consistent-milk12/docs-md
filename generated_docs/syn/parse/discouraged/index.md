*[syn](../../index.md) / [parse](../index.md) / [discouraged](index.md)*

---

# Module `discouraged`

Extensions to the parsing API with niche applicability.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Speculative`](#speculative) | trait | Extensions to the `ParseStream` API to support speculative parsing. |
| [`AnyDelimiter`](#anydelimiter) | trait | Extensions to the `ParseStream` API to support manipulating invisible |

## Traits

### `Speculative`

```rust
trait Speculative { ... }
```

Extensions to the `ParseStream` API to support speculative parsing.

#### Required Methods

- `fn advance_to(&self, fork: &Self)`

  Advance this parse stream to the position of a forked parse stream.

#### Implementors

- [`ParseBuffer`](../index.md)

### `AnyDelimiter`

```rust
trait AnyDelimiter { ... }
```

Extensions to the `ParseStream` API to support manipulating invisible
delimiters the same as if they were visible.

#### Required Methods

- `fn parse_any_delimiter(&self) -> Result<(Delimiter, DelimSpan, ParseBuffer<'_>)>`

  Returns the delimiter, the span of the delimiter token, and the nested

#### Implementors

- [`ParseBuffer`](../index.md)

