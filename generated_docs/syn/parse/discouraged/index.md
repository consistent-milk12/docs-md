*[syn](../../index.md) / [parse](../index.md) / [discouraged](index.md)*

---

# Module `discouraged`

Extensions to the parsing API with niche applicability.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Speculative`](#speculative) | trait | Extensions to the `ParseStream` API to support speculative parsing. |
| [`AnyDelimiter`](#anydelimiter) | trait | Extensions to the `ParseStream` API to support manipulating invisible delimiters the same as if they were visible. |

## Traits

### `Speculative`

```rust
trait Speculative { ... }
```

*Defined in [`syn-2.0.111/src/discouraged.rs:13-165`](../../../../.source_1765210505/syn-2.0.111/src/discouraged.rs#L13-L165)*

Extensions to the `ParseStream` API to support speculative parsing.

#### Required Methods

- `fn advance_to(&self, fork: &Self)`

  Advance this parse stream to the position of a forked parse stream.

#### Implementors

- [`ParseBuffer`](../index.md#parsebuffer)

### `AnyDelimiter`

```rust
trait AnyDelimiter { ... }
```

*Defined in [`syn-2.0.111/src/discouraged.rs:205-209`](../../../../.source_1765210505/syn-2.0.111/src/discouraged.rs#L205-L209)*

Extensions to the `ParseStream` API to support manipulating invisible
delimiters the same as if they were visible.

#### Required Methods

- `fn parse_any_delimiter(&self) -> Result<(Delimiter, DelimSpan, ParseBuffer<'_>)>`

  Returns the delimiter, the span of the delimiter token, and the nested

#### Implementors

- [`ParseBuffer`](../index.md#parsebuffer)

