*[bitflags](../index.md) / [parser](index.md)*

---

# Module `parser`

Parsing flags from text.

Format and parse a flags value as text using the following grammar:

- _Flags:_ (_Whitespace_ _Flag_ _Whitespace_)`|`*
- _Flag:_ _Name_ | _Hex Number_
- _Name:_ The name of any defined flag
- _Hex Number_: `0x`([0-9a-fA-F])*
- _Whitespace_: (\s)*

As an example, this is how `Flags::A | Flags::B | 0x0c` can be represented as text:

```text
A | B | 0x0c
```

Alternatively, it could be represented without whitespace:

```text
A|B|0x0C
```

Note that identifiers are *case-sensitive*, so the following is *not equivalent*:

```text
a|b|0x0C
```

## Contents

- [Structs](#structs)
  - [`ParseError`](#parseerror)
- [Enums](#enums)
  - [`ParseErrorKind`](#parseerrorkind)
- [Traits](#traits)
  - [`WriteHex`](#writehex)
  - [`ParseHex`](#parsehex)
- [Functions](#functions)
  - [`to_writer`](#to_writer)
  - [`from_str`](#from_str)
  - [`to_writer_truncate`](#to_writer_truncate)
  - [`from_str_truncate`](#from_str_truncate)
  - [`to_writer_strict`](#to_writer_strict)
  - [`from_str_strict`](#from_str_strict)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct | An error encountered while parsing flags from text. |
| [`ParseErrorKind`](#parseerrorkind) | enum |  |
| [`WriteHex`](#writehex) | trait | Encode a value as a hex string. |
| [`ParseHex`](#parsehex) | trait | Parse a value from a hex string. |
| [`to_writer`](#to_writer) | fn | Write a flags value as text. |
| [`from_str`](#from_str) | fn | Parse a flags value from text. |
| [`to_writer_truncate`](#to_writer_truncate) | fn | Write a flags value as text, ignoring any unknown bits. |
| [`from_str_truncate`](#from_str_truncate) | fn | Parse a flags value from text. |
| [`to_writer_strict`](#to_writer_strict) | fn | Write only the contained, defined, named flags in a flags value as text. |
| [`from_str_strict`](#from_str_strict) | fn | Parse a flags value from text. |

## Structs

### `ParseError`

```rust
struct ParseError(ParseErrorKind);
```

An error encountered while parsing flags from text.

#### Implementations

- <span id="parseerror-invalid-hex-flag"></span>`fn invalid_hex_flag(flag: impl fmt::Display) -> Self`

- <span id="parseerror-invalid-named-flag"></span>`fn invalid_named_flag(flag: impl fmt::Display) -> Self`

- <span id="parseerror-empty-flag"></span>`const fn empty_flag() -> Self`

#### Trait Implementations

##### `impl Debug for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl<T> ToString for ParseError`

- <span id="parseerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ParseErrorKind`

```rust
enum ParseErrorKind {
    EmptyFlag,
    InvalidNamedFlag {
        got: String,
    },
    InvalidHexFlag {
        got: String,
    },
}
```

#### Trait Implementations

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `WriteHex`

```rust
trait WriteHex { ... }
```

Encode a value as a hex string.

Implementors of this trait should not write the `0x` prefix.

#### Required Methods

- `fn write_hex<W: fmt::Write>(&self, writer: W) -> fmt::Result`

  Write the value as hex.

### `ParseHex`

```rust
trait ParseHex { ... }
```

Parse a value from a hex string.

#### Required Methods

- `fn parse_hex(input: &str) -> Result<Self, ParseError>`

  Parse the value from hex.

## Functions

### `to_writer`

```rust
fn to_writer<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex
```

Write a flags value as text.

Any bits that aren't part of a contained flag will be formatted as a hex number.

### `from_str`

```rust
fn from_str<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex
```

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be retained.

### `to_writer_truncate`

```rust
fn to_writer_truncate<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex
```

Write a flags value as text, ignoring any unknown bits.

### `from_str_truncate`

```rust
fn from_str_truncate<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex
```

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be ignored.

### `to_writer_strict`

```rust
fn to_writer_strict<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
```

Write only the contained, defined, named flags in a flags value as text.

### `from_str_strict`

```rust
fn from_str_strict<B: Flags>(input: &str) -> Result<B, ParseError>
```

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
This function will fail to parse hex values.

