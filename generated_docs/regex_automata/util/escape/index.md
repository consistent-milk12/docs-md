*[regex_automata](../../index.md) / [util](../index.md) / [escape](index.md)*

---

# Module `escape`

Provides convenience routines for escaping raw bytes.

Since this crate tends to deal with `&[u8]` everywhere and the default
`Debug` implementation just shows decimal integers, it makes debugging those
representations quite difficult. This module provides types that show `&[u8]`
as if it were a string, with invalid UTF-8 escaped into its byte-by-byte hex
representation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugByte`](#debugbyte) | struct | Provides a convenient `Debug` implementation for a `u8`. |
| [`DebugHaystack`](#debughaystack) | struct | Provides a convenient `Debug` implementation for `&[u8]`. |

## Structs

### `DebugByte`

```rust
struct DebugByte(u8);
```

*Defined in [`regex-automata-0.4.13/src/util/escape.rs:19`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/escape.rs#L19)*

Provides a convenient `Debug` implementation for a `u8`.

The `Debug` impl treats the byte as an ASCII, and emits a human readable
representation of it. If the byte isn't ASCII, then it's emitted as a hex
escape sequence.

#### Trait Implementations

##### `impl Clone for DebugByte`

- <span id="debugbyte-clone"></span>`fn clone(&self) -> DebugByte` — [`DebugByte`](#debugbyte)

##### `impl Copy for DebugByte`

##### `impl Debug for DebugByte`

- <span id="debugbyte-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `DebugHaystack<'a>`

```rust
struct DebugHaystack<'a>(&'a [u8]);
```

*Defined in [`regex-automata-0.4.13/src/util/escape.rs:49`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/escape.rs#L49)*

Provides a convenient `Debug` implementation for `&[u8]`.

This generally works best when the bytes are presumed to be mostly UTF-8,
but will work for anything. For any bytes that aren't UTF-8, they are
emitted as hex escape sequences.

#### Trait Implementations

##### `impl Debug for DebugHaystack<'a>`

- <span id="debughaystack-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

