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
  - [`strip_bytes`](#strip_bytes)
  - [`strip_str`](#strip_str)

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
| [`strip_bytes`](#strip_bytes) | fn |  |
| [`strip_str`](#strip_str) | fn |  |

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

See [`StripBytes`](#stripbytes)

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

- <span id="stripbytesiter-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](#stripbytesiter)

##### `impl<'s> StructuralPartialEq for StripBytesIter<'s>`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

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

See [`StripStr`](#stripstr)

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

- <span id="stripstriter-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](#stripstriter)

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

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

#### Trait Implementations

##### `impl<'s> Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl<'s> Debug for StrippedBytes<'s>`

- <span id="strippedbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](#strippedbytes)

##### `impl<'s> Eq for StrippedBytes<'s>`

##### `impl<I> IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'s> Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](#strippedbytes)

##### `impl<'s> StructuralPartialEq for StrippedBytes<'s>`

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](#strip-str)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

#### Trait Implementations

##### `impl<'s> Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

##### `impl<'s> Debug for StrippedStr<'s>`

- <span id="strippedstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](#strippedstr)

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

- <span id="strippedstr-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](#strippedstr)

##### `impl<'s> StructuralPartialEq for StrippedStr<'s>`

##### `impl<T> ToString for StrippedStr<'s>`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](#winconbytesiter)

#### Trait Implementations

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Debug for WinconBytes`

- <span id="winconbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

See [`WinconBytes`](#winconbytes)

#### Trait Implementations

##### `impl<'s> Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'s> Eq for WinconBytesIter<'s>`

##### `impl<I> IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](#winconbytesiter)

##### `impl<'s> StructuralPartialEq for WinconBytesIter<'s>`

## Functions

