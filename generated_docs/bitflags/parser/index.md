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
  - [`to_writer`](#to-writer)
  - [`from_str`](#from-str)
  - [`to_writer_truncate`](#to-writer-truncate)
  - [`from_str_truncate`](#from-str-truncate)
  - [`to_writer_strict`](#to-writer-strict)
  - [`from_str_strict`](#from-str-strict)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct | An error encountered while parsing flags from text. |
| [`ParseErrorKind`](#parseerrorkind) | enum |  |
| [`WriteHex`](#writehex) | trait | Encode a value as a hex string. |
| [`ParseHex`](#parsehex) | trait | Parse a value from a hex string. |
| [`to_writer`](#to-writer) | fn | Write a flags value as text. |
| [`from_str`](#from-str) | fn | Parse a flags value from text. |
| [`to_writer_truncate`](#to-writer-truncate) | fn | Write a flags value as text, ignoring any unknown bits. |
| [`from_str_truncate`](#from-str-truncate) | fn | Parse a flags value from text. |
| [`to_writer_strict`](#to-writer-strict) | fn | Write only the contained, defined, named flags in a flags value as text. |
| [`from_str_strict`](#from-str-strict) | fn | Parse a flags value from text. |

## Structs

### `ParseError`

```rust
struct ParseError(ParseErrorKind);
```

*Defined in [`bitflags-2.10.0/src/parser.rs:244`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L244)*

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

##### `impl ToString for ParseError`

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

*Defined in [`bitflags-2.10.0/src/parser.rs:248-262`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L248-L262)*

#### Trait Implementations

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `WriteHex`

```rust
trait WriteHex { ... }
```

*Defined in [`bitflags-2.10.0/src/parser.rs:227-230`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L227-L230)*

Encode a value as a hex string.

Implementors of this trait should not write the `0x` prefix.

#### Required Methods

- `fn write_hex<W: fmt::Write>(&self, writer: W) -> fmt::Result`

  Write the value as hex.

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `ParseHex`

```rust
trait ParseHex { ... }
```

*Defined in [`bitflags-2.10.0/src/parser.rs:235-240`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L235-L240)*

Parse a value from a hex string.

#### Required Methods

- `fn parse_hex(input: &str) -> Result<Self, ParseError>`

  Parse the value from hex.

#### Implementors

- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `to_writer`

```rust
fn to_writer<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex
```

*Defined in [`bitflags-2.10.0/src/parser.rs:42-78`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L42-L78)*

Write a flags value as text.

Any bits that aren't part of a contained flag will be formatted as a hex number.

### `from_str`

```rust
fn from_str<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex
```

*Defined in [`bitflags-2.10.0/src/parser.rs:99-137`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L99-L137)*

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be retained.

### `to_writer_truncate`

```rust
fn to_writer_truncate<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
where
    <B as >::Bits: WriteHex
```

*Defined in [`bitflags-2.10.0/src/parser.rs:142-147`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L142-L147)*

Write a flags value as text, ignoring any unknown bits.

### `from_str_truncate`

```rust
fn from_str_truncate<B: Flags>(input: &str) -> Result<B, ParseError>
where
    <B as >::Bits: ParseHex
```

*Defined in [`bitflags-2.10.0/src/parser.rs:155-160`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L155-L160)*

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
Unknown bits will be ignored.

### `to_writer_strict`

```rust
fn to_writer_strict<B: Flags>(flags: &B, writer: impl Write) -> Result<(), fmt::Error>
```

*Defined in [`bitflags-2.10.0/src/parser.rs:165-181`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L165-L181)*

Write only the contained, defined, named flags in a flags value as text.

### `from_str_strict`

```rust
fn from_str_strict<B: Flags>(input: &str) -> Result<B, ParseError>
```

*Defined in [`bitflags-2.10.0/src/parser.rs:189-220`](../../../.source_1765521767/bitflags-2.10.0/src/parser.rs#L189-L220)*

Parse a flags value from text.

This function will fail on any names that don't correspond to defined flags.
This function will fail to parse hex values.

