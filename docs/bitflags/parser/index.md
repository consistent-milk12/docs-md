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

## Structs

### `ParseError`

```rust
struct ParseError(ParseErrorKind);
```

An error encountered while parsing flags from text.

#### Implementations

- `fn invalid_hex_flag(flag: impl fmt::Display) -> Self`

- `fn invalid_named_flag(flag: impl fmt::Display) -> Self`

- `const fn empty_flag() -> Self`

#### Trait Implementations

##### `impl Debug for ParseError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ParseError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl<T> ToString for ParseError`

- `fn to_string(self: &Self) -> String`

## Traits

### `WriteHex`

```rust
trait WriteHex { ... }
```

Encode a value as a hex string.

Implementors of this trait should not write the `0x` prefix.

#### Required Methods

- `fn write_hex<W: fmt::Write>(self: &Self, writer: W) -> fmt::Result`

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

