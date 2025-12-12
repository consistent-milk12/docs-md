*[anstream](../index.md) / [adapter](index.md)*

---

# Module `adapter`

Gracefully degrade styled output

## Contents

- [Modules](#modules)
  - [`strip`](#strip)
  - [`wincon`](#wincon)
- [Structs](#structs)
  - [`StripBytes`](#stripbytes)
  - [`StripBytesIter`](#stripbytesiter)
  - [`StripStr`](#stripstr)
  - [`StripStrIter`](#stripstriter)
  - [`StrippedBytes`](#strippedbytes)
  - [`StrippedStr`](#strippedstr)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
- [Functions](#functions)
  - [`strip_bytes`](#strip-bytes)
  - [`strip_str`](#strip-str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`strip`](#strip) | mod |  |
| [`wincon`](#wincon) | mod |  |
| [`StripBytes`](#stripbytes) | struct |  |
| [`StripBytesIter`](#stripbytesiter) | struct |  |
| [`StripStr`](#stripstr) | struct |  |
| [`StripStrIter`](#stripstriter) | struct |  |
| [`StrippedBytes`](#strippedbytes) | struct |  |
| [`StrippedStr`](#strippedstr) | struct |  |
| [`WinconBytes`](#winconbytes) | struct |  |
| [`WinconBytesIter`](#winconbytesiter) | struct |  |
| [`strip_bytes`](#strip-bytes) | fn |  |
| [`strip_str`](#strip-str) | fn |  |

## Modules

- [`strip`](strip/index.md)
- [`wincon`](wincon/index.md)

## Structs

### `StripBytes`

```rust
struct StripBytes {
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:245-248`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L245-L248)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripbytes-new"></span>`fn new() -> Self`

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](strip/index.md#stripbytesiter)

#### Trait Implementations

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl Debug for StripBytes`

- <span id="stripbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl Eq for StripBytes`

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:268-272`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L268-L272)*

See [`StripBytes`](strip/index.md)

#### Trait Implementations

##### `impl Debug for StripBytesIter<'s>`

- <span id="stripbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripBytesIter<'s>`

##### `impl IntoIterator for StripBytesIter<'s>`

- <span id="stripbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripBytesIter<'s>`

- <span id="stripbytesiter-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="stripbytesiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripBytesIter<'s>`

- <span id="stripbytesiter-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](strip/index.md#stripbytesiter)

##### `impl StructuralPartialEq for StripBytesIter<'s>`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:79-81`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L79-L81)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripstr-new"></span>`fn new() -> Self`

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](strip/index.md#stripstriter)

#### Trait Implementations

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl Debug for StripStr`

- <span id="stripstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl Eq for StripStr`

##### `impl PartialEq for StripStr`

- <span id="stripstr-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](strip/index.md#stripstr)

##### `impl StructuralPartialEq for StripStr`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:100-103`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L100-L103)*

See [`StripStr`](strip/index.md)

#### Trait Implementations

##### `impl Debug for StripStrIter<'s>`

- <span id="stripstriter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripStrIter<'s>`

##### `impl IntoIterator for StripStrIter<'s>`

- <span id="stripstriter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripstriter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripstriter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripStrIter<'s>`

- <span id="stripstriter-iterator-type-item"></span>`type Item = &'s str`

- <span id="stripstriter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripStrIter<'s>`

- <span id="stripstriter-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](strip/index.md#stripstriter)

##### `impl StructuralPartialEq for StripStrIter<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:184-188`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L184-L188)*

See [`strip_bytes`](strip/index.md)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

#### Trait Implementations

##### `impl Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl Debug for StrippedBytes<'s>`

- <span id="strippedbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl Eq for StrippedBytes<'s>`

##### `impl IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl StructuralPartialEq for StrippedBytes<'s>`

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:28-31`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L28-L31)*

See [`strip_str`](strip/index.md)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

#### Trait Implementations

##### `impl Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl Debug for StrippedStr<'s>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StrippedStr<'s>`

##### `impl IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedStr<'s>`

- <span id="strippedstr-iterator-type-item"></span>`type Item = &'s str`

- <span id="strippedstr-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedStr<'s>`

- <span id="strippedstr-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl StructuralPartialEq for StrippedStr<'s>`

##### `impl ToString for StrippedStr<'s>`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:3-6`](../../../.source_1765210505/anstream-0.6.21/src/adapter/wincon.rs#L3-L6)*

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

#### Trait Implementations

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl Debug for WinconBytes`

- <span id="winconbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:28-32`](../../../.source_1765210505/anstream-0.6.21/src/adapter/wincon.rs#L28-L32)*

See [`WinconBytes`](wincon/index.md)

#### Trait Implementations

##### `impl Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WinconBytesIter<'s>`

##### `impl IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-iterator-type-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

##### `impl StructuralPartialEq for WinconBytesIter<'s>`

## Functions

### `strip_bytes`

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:178-180`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L178-L180)*

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

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:22-24`](../../../.source_1765210505/anstream-0.6.21/src/adapter/strip.rs#L22-L24)*

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](strip/index.md).

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

