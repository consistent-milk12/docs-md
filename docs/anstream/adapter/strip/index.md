*[anstream](../../index.md) / [adapter](../index.md) / [strip](index.md)*

---

# Module `strip`

## Structs

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](#strip-str)

#### Implementations

- `fn new(data: &'s str) -> Self`

- `fn to_string(self: &Self) -> String`

#### Trait Implementations

##### `impl<'s> Clone for StrippedStr<'s>`

- `fn clone(self: &Self) -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl<'s> Debug for StrippedStr<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'s> Default for StrippedStr<'s>`

- `fn default() -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl Display for StrippedStr<'_>`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<'s> Eq for StrippedStr<'s>`

##### `impl<I> IntoIterator for StrippedStr<'s>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s> Iterator for StrippedStr<'s>`

- `type Item = &'s str`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StrippedStr<'s>`

- `fn eq(self: &Self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](#strippedstr)

##### `impl<'s> StructuralPartialEq for StrippedStr<'s>`

##### `impl<T> ToString for StrippedStr<'s>`

- `fn to_string(self: &Self) -> String`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

Incrementally strip non-contiguous data

#### Implementations

- `fn new() -> Self`

- `fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](#stripstriter)

#### Trait Implementations

##### `impl Clone for StripStr`

- `fn clone(self: &Self) -> StripStr` — [`StripStr`](#stripstr)

##### `impl Debug for StripStr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StripStr`

- `fn default() -> StripStr` — [`StripStr`](#stripstr)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- `fn eq(self: &Self, other: &StripStr) -> bool` — [`StripStr`](#stripstr)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

See [`StripStr`](#stripstr)

#### Trait Implementations

##### `impl<'s> Debug for StripStrIter<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'s> Eq for StripStrIter<'s>`

##### `impl<I> IntoIterator for StripStrIter<'s>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s> Iterator for StripStrIter<'s>`

- `type Item = &'s str`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StripStrIter<'s>`

- `fn eq(self: &Self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](#stripstriter)

##### `impl<'s> StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

See [`strip_bytes`](#strip-bytes)

#### Implementations

- `fn new(bytes: &'s [u8]) -> Self`

- `fn extend(self: &mut Self, bytes: &'s [u8])`

- `fn is_empty(self: &Self) -> bool`

- `fn into_vec(self: Self) -> Vec<u8>`

#### Trait Implementations

##### `impl<'s> Clone for StrippedBytes<'s>`

- `fn clone(self: &Self) -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl<'s> Debug for StrippedBytes<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'s> Default for StrippedBytes<'s>`

- `fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl<'s> Eq for StrippedBytes<'s>`

##### `impl<I> IntoIterator for StrippedBytes<'s>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s> Iterator for StrippedBytes<'s>`

- `type Item = &'s [u8]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StrippedBytes<'s>`

- `fn eq(self: &Self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](#strippedbytes)

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

- `fn new() -> Self`

- `fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](#stripbytesiter)

#### Trait Implementations

##### `impl Clone for StripBytes`

- `fn clone(self: &Self) -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Debug for StripBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StripBytes`

- `fn default() -> StripBytes` — [`StripBytes`](#stripbytes)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- `fn eq(self: &Self, other: &StripBytes) -> bool` — [`StripBytes`](#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

See [`StripBytes`](#stripbytes)

#### Trait Implementations

##### `impl<'s> Debug for StripBytesIter<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'s> Eq for StripBytesIter<'s>`

##### `impl<I> IntoIterator for StripBytesIter<'s>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s> Iterator for StripBytesIter<'s>`

- `type Item = &'s [u8]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StripBytesIter<'s>`

- `fn eq(self: &Self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](#stripbytesiter)

##### `impl<'s> StructuralPartialEq for StripBytesIter<'s>`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8parse::Parser,
}
```

#### Implementations

- `fn add(self: &mut Self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Utf8Parser`

- `fn clone(self: &Self) -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Debug for Utf8Parser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Utf8Parser`

- `fn default() -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Eq for Utf8Parser`

##### `impl PartialEq for Utf8Parser`

- `fn eq(self: &Self, other: &Utf8Parser) -> bool` — [`Utf8Parser`](#utf8parser)

##### `impl StructuralPartialEq for Utf8Parser`

### `VtUtf8Receiver<'a>`

```rust
struct VtUtf8Receiver<'a>(&'a mut bool);
```

#### Trait Implementations

##### `impl Receiver for VtUtf8Receiver<'_>`

- `fn codepoint(self: &mut Self, _: char)`

- `fn invalid_sequence(self: &mut Self)`

## Functions

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

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

