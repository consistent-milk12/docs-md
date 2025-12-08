*[anstream](../../index.md) / [adapter](../index.md) / [strip](index.md)*

---

# Module `strip`

## Contents

- [Structs](#structs)
  - [`StrippedStr`](#strippedstr)
  - [`StripStr`](#stripstr)
  - [`StripStrIter`](#stripstriter)
  - [`StrippedBytes`](#strippedbytes)
  - [`StripBytes`](#stripbytes)
  - [`StripBytesIter`](#stripbytesiter)
  - [`Utf8Parser`](#utf8parser)
  - [`VtUtf8Receiver`](#vtutf8receiver)
- [Functions](#functions)
  - [`strip_str`](#strip_str)
  - [`next_str`](#next_str)
  - [`from_utf8_unchecked`](#from_utf8_unchecked)
  - [`is_utf8_continuation`](#is_utf8_continuation)
  - [`strip_bytes`](#strip_bytes)
  - [`next_bytes`](#next_bytes)
  - [`is_printable_bytes`](#is_printable_bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StrippedStr`](#strippedstr) | struct | See [`strip_str`] |
| [`StripStr`](#stripstr) | struct | Incrementally strip non-contiguous data |
| [`StripStrIter`](#stripstriter) | struct | See [`StripStr`] |
| [`StrippedBytes`](#strippedbytes) | struct | See [`strip_bytes`] |
| [`StripBytes`](#stripbytes) | struct | Incrementally strip non-contiguous data |
| [`StripBytesIter`](#stripbytesiter) | struct | See [`StripBytes`] |
| [`Utf8Parser`](#utf8parser) | struct |  |
| [`VtUtf8Receiver`](#vtutf8receiver) | struct |  |
| [`strip_str`](#strip_str) | fn | Strip ANSI escapes from a `&str`, returning the printable content |
| [`next_str`](#next_str) | fn |  |
| [`from_utf8_unchecked`](#from_utf8_unchecked) | fn |  |
| [`is_utf8_continuation`](#is_utf8_continuation) | fn |  |
| [`strip_bytes`](#strip_bytes) | fn | Strip ANSI escapes from bytes, returning the printable content |
| [`next_bytes`](#next_bytes) | fn |  |
| [`is_printable_bytes`](#is_printable_bytes) | fn |  |

## Structs

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](../index.md)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

#### Trait Implementations

##### `impl<'s> Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](../index.md)

##### `impl<'s> Debug for StrippedStr<'s>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](../index.md)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<'s> Eq for StrippedStr<'s>`

##### `impl<I> IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'s> Iterator for StrippedStr<'s>`

- <span id="strippedstr-item"></span>`type Item = &'s str`

- <span id="strippedstr-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StrippedStr<'s>`

- <span id="strippedstr-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](../index.md)

##### `impl<'s> StructuralPartialEq for StrippedStr<'s>`

##### `impl<T> ToString for StrippedStr<'s>`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripstr-new"></span>`fn new() -> Self`

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](../index.md)

#### Trait Implementations

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](../index.md)

##### `impl Debug for StripStr`

- <span id="stripstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](../index.md)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- <span id="stripstr-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](../index.md)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

See [`StripStr`](../index.md)

#### Trait Implementations

##### `impl<'s> Debug for StripStrIter<'s>`

- <span id="stripstriter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Eq for StripStrIter<'s>`

##### `impl<I> IntoIterator for StripStrIter<'s>`

- <span id="stripstriter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripstriter-intoiter"></span>`type IntoIter = I`

- <span id="stripstriter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'s> Iterator for StripStrIter<'s>`

- <span id="stripstriter-item"></span>`type Item = &'s str`

- <span id="stripstriter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StripStrIter<'s>`

- <span id="stripstriter-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](../index.md)

##### `impl<'s> StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

See [`strip_bytes`](../index.md)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

#### Trait Implementations

##### `impl<'s> Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](../index.md)

##### `impl<'s> Debug for StrippedBytes<'s>`

- <span id="strippedbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](../index.md)

##### `impl<'s> Eq for StrippedBytes<'s>`

##### `impl<I> IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'s> Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](../index.md)

##### `impl<'s> StructuralPartialEq for StrippedBytes<'s>`

### `StripBytes`

```rust
struct StripBytes {
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripbytes-new"></span>`fn new() -> Self`

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](../index.md)

#### Trait Implementations

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](../index.md)

##### `impl Debug for StripBytes`

- <span id="stripbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](../index.md)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](../index.md)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

See [`StripBytes`](../index.md)

#### Trait Implementations

##### `impl<'s> Debug for StripBytesIter<'s>`

- <span id="stripbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Eq for StripBytesIter<'s>`

##### `impl<I> IntoIterator for StripBytesIter<'s>`

- <span id="stripbytesiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripbytesiter-intoiter"></span>`type IntoIter = I`

- <span id="stripbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'s> Iterator for StripBytesIter<'s>`

- <span id="stripbytesiter-item"></span>`type Item = &'s [u8]`

- <span id="stripbytesiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StripBytesIter<'s>`

- <span id="stripbytesiter-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](../index.md)

##### `impl<'s> StructuralPartialEq for StripBytesIter<'s>`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8parse::Parser,
}
```

#### Implementations

- <span id="utf8parser-add"></span>`fn add(&mut self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Parser`

- <span id="utf8parser-clone"></span>`fn clone(&self) -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Debug for Utf8Parser`

- <span id="utf8parser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8Parser`

- <span id="utf8parser-default"></span>`fn default() -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Eq for Utf8Parser`

##### `impl PartialEq for Utf8Parser`

- <span id="utf8parser-eq"></span>`fn eq(&self, other: &Utf8Parser) -> bool` — [`Utf8Parser`](#utf8parser)

##### `impl StructuralPartialEq for Utf8Parser`

### `VtUtf8Receiver<'a>`

```rust
struct VtUtf8Receiver<'a>(&'a mut bool);
```

#### Trait Implementations

##### `impl Receiver for VtUtf8Receiver<'_>`

- <span id="vtutf8receiver-codepoint"></span>`fn codepoint(&mut self, _: char)`

- <span id="vtutf8receiver-invalid-sequence"></span>`fn invalid_sequence(&mut self)`

## Functions

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](../index.md).

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

### `next_str`

```rust
fn next_str<'s>(bytes: &mut &'s [u8], state: &mut anstyle_parse::state::State) -> Option<&'s str>
```

### `from_utf8_unchecked`

```rust
unsafe fn from_utf8_unchecked<'b>(bytes: &'b [u8], safety_justification: &'static str) -> &'b str
```

### `is_utf8_continuation`

```rust
fn is_utf8_continuation(b: u8) -> bool
```

### `strip_bytes`

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes<'_>
```

Strip ANSI escapes from bytes, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_bytes(styled_text.as_bytes()).into_vec();
assert_eq!(plain_str.as_slice(), &b"foo bar"[..]);
```

### `next_bytes`

```rust
fn next_bytes<'s>(bytes: &mut &'s [u8], state: &mut anstyle_parse::state::State, utf8parser: &mut Utf8Parser) -> Option<&'s [u8]>
```

### `is_printable_bytes`

```rust
fn is_printable_bytes(action: anstyle_parse::state::Action, byte: u8) -> bool
```

