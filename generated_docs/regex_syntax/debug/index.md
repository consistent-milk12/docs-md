*[regex_syntax](../index.md) / [debug](index.md)*

---

# Module `debug`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Byte`](#byte) | struct | A type that wraps a single byte with a convenient fmt::Debug impl that |
| [`Bytes`](#bytes) | struct | A type that provides a human readable debug impl for arbitrary bytes. |
| [`utf8_decode`](#utf8_decode) | fn | Decodes the next UTF-8 encoded codepoint from the given byte slice. |

## Structs

### `Byte`

```rust
struct Byte(u8);
```

A type that wraps a single byte with a convenient fmt::Debug impl that
escapes the byte.

#### Trait Implementations

##### `impl Debug for Byte`

- <span id="byte-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Bytes<'a>`

```rust
struct Bytes<'a>(&'a [u8]);
```

A type that provides a human readable debug impl for arbitrary bytes.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything.

N.B. This is copied nearly verbatim from regex-automata. Sigh.

#### Trait Implementations

##### `impl<'a> Debug for Bytes<'a>`

- <span id="bytes-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Functions

### `utf8_decode`

```rust
fn utf8_decode(bytes: &[u8]) -> Option<Result<char, u8>>
```

Decodes the next UTF-8 encoded codepoint from the given byte slice.

If no valid encoding of a codepoint exists at the beginning of the given
byte slice, then the first byte is returned instead.

This returns `None` if and only if `bytes` is empty.

