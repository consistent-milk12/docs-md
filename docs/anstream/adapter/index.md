*[anstream](../index.md) / [adapter](index.md)*

---

# Module `adapter`

Gracefully degrade styled output

## Modules

- [`strip`](strip/index.md) - 
- [`wincon`](wincon/index.md) - 

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

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- `fn new() -> Self`

- `fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](#winconbytesiter)

#### Trait Implementations

##### `impl Clone for WinconBytes`

- `fn clone(self: &Self) -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Debug for WinconBytes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for WinconBytes`

- `fn default() -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- `fn eq(self: &Self, other: &WinconBytes) -> bool` — [`WinconBytes`](#winconbytes)

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'s> Eq for WinconBytesIter<'s>`

##### `impl<I> IntoIterator for WinconBytesIter<'s>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- `type Item = (Style, String)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'s> PartialEq for WinconBytesIter<'s>`

- `fn eq(self: &Self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](#winconbytesiter)

##### `impl<'s> StructuralPartialEq for WinconBytesIter<'s>`

## Functions

