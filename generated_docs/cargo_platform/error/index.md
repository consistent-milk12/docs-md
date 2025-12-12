*[cargo_platform](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct |  |
| [`ParseErrorKind`](#parseerrorkind) | enum |  |

## Structs

### `ParseError`

```rust
struct ParseError {
    kind: ParseErrorKind,
    orig: String,
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:4-7`](../../../.source_1765521767/cargo-platform-0.3.1/src/error.rs#L4-L7)*

#### Implementations

- <span id="parseerror-new"></span>`fn new(orig: &str, kind: ParseErrorKind) -> ParseError` — [`ParseErrorKind`](#parseerrorkind), [`ParseError`](#parseerror)

#### Trait Implementations

##### `impl Debug for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl ToString for ParseError`

- <span id="parseerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ParseErrorKind`

```rust
enum ParseErrorKind {
    UnterminatedString,
    UnexpectedChar(char),
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    IncompleteExpr(&'static str),
    UnterminatedExpression(String),
    InvalidTarget(String),
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:11-21`](../../../.source_1765521767/cargo-platform-0.3.1/src/error.rs#L11-L21)*

#### Trait Implementations

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ParseErrorKind`

- <span id="parseerrorkind-to-string"></span>`fn to_string(&self) -> String`

