*[regex_automata](../../index.md) / [util](../index.md) / [utf8](index.md)*

---

# Module `utf8`

Utilities for dealing with UTF-8.

This module provides some UTF-8 related helper routines, including an
incremental decoder.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`is_word_byte`](#is-word-byte) | fn | Returns true if and only if the given byte is considered a word character. |
| [`decode`](#decode) | fn | Decodes the next UTF-8 encoded codepoint from the given byte slice. |
| [`decode_last`](#decode-last) | fn | Decodes the last UTF-8 encoded codepoint from the given byte slice. |
| [`len`](#len) | fn | Given a UTF-8 leading byte, this returns the total number of code units in the following encoded codepoint. |
| [`is_boundary`](#is-boundary) | fn | Returns true if and only if the given offset in the given bytes falls on a valid UTF-8 encoded codepoint boundary. |
| [`is_leading_or_invalid_byte`](#is-leading-or-invalid-byte) | fn | Returns true if and only if the given byte is either a valid leading UTF-8 byte, or is otherwise an invalid byte that can never appear anywhere in a valid UTF-8 sequence. |

## Functions

### `is_word_byte`

```rust
fn is_word_byte(b: u8) -> bool
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:16-41`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L16-L41)*

Returns true if and only if the given byte is considered a word character.
This only applies to ASCII.

This was copied from regex-syntax so that we can use it to determine the
starting DFA state while searching without depending on regex-syntax. The
definition is never going to change, so there's no maintenance/bit-rot
hazard here.

### `decode`

```rust
fn decode(bytes: &[u8]) -> Option<Result<char, u8>>
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:56-70`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L56-L70)*

Decodes the next UTF-8 encoded codepoint from the given byte slice.

If no valid encoding of a codepoint exists at the beginning of the given
byte slice, then the first byte is returned instead.

This returns `None` if and only if `bytes` is empty.

This never panics.

*WARNING*: This is not designed for performance. If you're looking for a
fast UTF-8 decoder, this is not it. If you feel like you need one in this
crate, then please file an issue and discuss your use case.

### `decode_last`

```rust
fn decode_last(bytes: &[u8]) -> Option<Result<char, u8>>
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:79-93`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L79-L93)*

Decodes the last UTF-8 encoded codepoint from the given byte slice.

If no valid encoding of a codepoint exists at the end of the given byte
slice, then the last byte is returned instead.

This returns `None` if and only if `bytes` is empty.

### `len`

```rust
fn len(byte: u8) -> Option<usize>
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:101-110`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L101-L110)*

Given a UTF-8 leading byte, this returns the total number of code units
in the following encoded codepoint.

If the given byte is not a valid UTF-8 leading byte, then this returns
`None`.

### `is_boundary`

```rust
fn is_boundary(bytes: &[u8], i: usize) -> bool
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:118-132`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L118-L132)*

Returns true if and only if the given offset in the given bytes falls on a
valid UTF-8 encoded codepoint boundary.

If `bytes` is not valid UTF-8, then the behavior of this routine is
unspecified.

### `is_leading_or_invalid_byte`

```rust
fn is_leading_or_invalid_byte(b: u8) -> bool
```

*Defined in [`regex-automata-0.4.13/src/util/utf8.rs:138-159`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/utf8.rs#L138-L159)*

Returns true if and only if the given byte is either a valid leading UTF-8
byte, or is otherwise an invalid byte that can never appear anywhere in a
valid UTF-8 sequence.

