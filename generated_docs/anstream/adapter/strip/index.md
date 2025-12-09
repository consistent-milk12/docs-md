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

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:28-31`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L28-L31)*

See [`strip_str`](#strip-str)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

#### Trait Implementations

##### `impl Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl Debug for StrippedStr<'s>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StrippedStr<'s>`

##### `impl IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedStr<'s>`

- <span id="strippedstr-type-item"></span>`type Item = &'s str`

- <span id="strippedstr-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedStr<'s>`

- <span id="strippedstr-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](#strippedstr)

##### `impl StructuralPartialEq for StrippedStr<'s>`

##### `impl ToString for StrippedStr<'s>`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:79-81`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L79-L81)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripstr-new"></span>`fn new() -> Self`

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](#stripstriter)

#### Trait Implementations

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](#stripstr)

##### `impl Debug for StripStr`

- <span id="stripstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](#stripstr)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- <span id="stripstr-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](#stripstr)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:100-103`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L100-L103)*

See [`StripStr`](#stripstr)

#### Trait Implementations

##### `impl Debug for StripStrIter<'s>`

- <span id="stripstriter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripStrIter<'s>`

##### `impl IntoIterator for StripStrIter<'s>`

- <span id="stripstriter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripstriter-type-intoiter"></span>`type IntoIter = I`

- <span id="stripstriter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripStrIter<'s>`

- <span id="stripstriter-type-item"></span>`type Item = &'s str`

- <span id="stripstriter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripStrIter<'s>`

- <span id="stripstriter-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](#stripstriter)

##### `impl StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:184-188`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L184-L188)*

See [`strip_bytes`](#strip-bytes)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

#### Trait Implementations

##### `impl Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl Debug for StrippedBytes<'s>`

- <span id="strippedbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl Eq for StrippedBytes<'s>`

##### `impl IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-type-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](#strippedbytes)

##### `impl StructuralPartialEq for StrippedBytes<'s>`

### `StripBytes`

```rust
struct StripBytes {
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:245-248`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L245-L248)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripbytes-new"></span>`fn new() -> Self`

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](#stripbytesiter)

#### Trait Implementations

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Debug for StripBytes`

- <span id="stripbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:268-272`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L268-L272)*

See [`StripBytes`](#stripbytes)

#### Trait Implementations

##### `impl Debug for StripBytesIter<'s>`

- <span id="stripbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripBytesIter<'s>`

##### `impl IntoIterator for StripBytesIter<'s>`

- <span id="stripbytesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripbytesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="stripbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripBytesIter<'s>`

- <span id="stripbytesiter-type-item"></span>`type Item = &'s [u8]`

- <span id="stripbytesiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripBytesIter<'s>`

- <span id="stripbytesiter-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](#stripbytesiter)

##### `impl StructuralPartialEq for StripBytesIter<'s>`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8parse::Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:332-334`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L332-L334)*

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

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:345`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L345)*

#### Trait Implementations

##### `impl Receiver for VtUtf8Receiver<'_>`

- <span id="vtutf8receiver-codepoint"></span>`fn codepoint(&mut self, _: char)`

- <span id="vtutf8receiver-invalid-sequence"></span>`fn invalid_sequence(&mut self)`

## Functions

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:22-24`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L22-L24)*

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](#stripstr).

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

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:115-144`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L115-L144)*

### `from_utf8_unchecked`

```rust
unsafe fn from_utf8_unchecked<'b>(bytes: &'b [u8], safety_justification: &'static str) -> &'b str
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:147-156`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L147-L156)*

### `is_utf8_continuation`

```rust
fn is_utf8_continuation(b: u8) -> bool
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:159-161`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L159-L161)*

### `strip_bytes`

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:178-180`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L178-L180)*

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

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:284-329`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L284-L329)*

### `is_printable_bytes`

```rust
fn is_printable_bytes(action: anstyle_parse::state::Action, byte: u8) -> bool
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:358-367`](../../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L358-L367)*

