*[anstream](../index.md) / [adapter](index.md)*

---

# Module `adapter`

Gracefully degrade styled output

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

- `fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](../../adapter/strip/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StripBytes` — [`StripBytes`](../../adapter/strip/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> StripBytes` — [`StripBytes`](../../adapter/strip/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StripBytes) -> bool` — [`StripBytes`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

See [`StripBytes`](strip/index.md)

#### Trait Implementations

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'s>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'s>`

- `type Item = &'s [u8]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq<'s>`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

Incrementally strip non-contiguous data

#### Implementations

- `fn new() -> Self`

- `fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](../../adapter/strip/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StripStr` — [`StripStr`](../../adapter/strip/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> StripStr` — [`StripStr`](../../adapter/strip/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StripStr) -> bool` — [`StripStr`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

See [`StripStr`](strip/index.md)

#### Trait Implementations

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'s>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'s>`

- `type Item = &'s str`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq<'s>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

See [`strip_bytes`](strip/index.md)

#### Implementations

- `fn new(bytes: &'s [u8]) -> Self`

- `fn extend(self: &mut Self, bytes: &'s [u8])`

- `fn is_empty(self: &Self) -> bool`

- `fn into_vec(self: Self) -> Vec<u8>`

#### Trait Implementations

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> StrippedBytes<'s>` — [`StrippedBytes`](../../adapter/strip/index.md)

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'s>`

- `fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](../../adapter/strip/index.md)

##### `impl Eq<'s>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'s>`

- `type Item = &'s [u8]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq<'s>`

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

See [`strip_str`](strip/index.md)

#### Implementations

- `fn new(data: &'s str) -> Self`

- `fn to_string(self: &Self) -> String`

#### Trait Implementations

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> StrippedStr<'s>` — [`StrippedStr`](../../adapter/strip/index.md)

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'s>`

- `fn default() -> StrippedStr<'s>` — [`StrippedStr`](../../adapter/strip/index.md)

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq<'s>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'s>`

- `type Item = &'s str`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](../../adapter/strip/index.md)

##### `impl StructuralPartialEq<'s>`

##### `impl ToString<T>`

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

- `fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](../../adapter/wincon/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> WinconBytes` — [`WinconBytes`](../../adapter/wincon/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> WinconBytes` — [`WinconBytes`](../../adapter/wincon/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &WinconBytes) -> bool` — [`WinconBytes`](../../adapter/wincon/index.md)

##### `impl StructuralPartialEq`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

See [`WinconBytes`](wincon/index.md)

#### Trait Implementations

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'s>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

- `type Item = (Style, String)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](../../adapter/wincon/index.md)

##### `impl StructuralPartialEq<'s>`

## Functions

