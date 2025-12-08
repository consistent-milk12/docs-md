*[anstream](../../index.md) / [adapter](../index.md) / [wincon](index.md)*

---

# Module `wincon`

## Structs

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

### `WinconCapture`

```rust
struct WinconCapture {
    style: anstyle::Style,
    printable: String,
    ready: Option<anstyle::Style>,
}
```

#### Implementations

- `fn reset(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for WinconCapture`

- `fn clone(self: &Self) -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Debug for WinconCapture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for WinconCapture`

- `fn default() -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Eq for WinconCapture`

##### `impl PartialEq for WinconCapture`

- `fn eq(self: &Self, other: &WinconCapture) -> bool` — [`WinconCapture`](#winconcapture)

##### `impl Perform for WinconCapture`

- `fn print(self: &mut Self, c: char)`

- `fn execute(self: &mut Self, byte: u8)`

- `fn csi_dispatch(self: &mut Self, params: &anstyle_parse::Params, _intermediates: &[u8], ignore: bool, action: u8)`

##### `impl StructuralPartialEq for WinconCapture`

## Enums

### `CsiState`

```rust
enum CsiState {
    Normal,
    PrepareCustomColor,
    Ansi256,
    Rgb,
    Underline,
}
```

#### Trait Implementations

##### `impl Clone for CsiState`

- `fn clone(self: &Self) -> CsiState` — [`CsiState`](#csistate)

##### `impl Copy for CsiState`

##### `impl Debug for CsiState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CsiState`

##### `impl PartialEq for CsiState`

- `fn eq(self: &Self, other: &CsiState) -> bool` — [`CsiState`](#csistate)

##### `impl StructuralPartialEq for CsiState`

### `ColorTarget`

```rust
enum ColorTarget {
    Fg,
    Bg,
    Underline,
}
```

#### Trait Implementations

##### `impl Clone for ColorTarget`

- `fn clone(self: &Self) -> ColorTarget` — [`ColorTarget`](#colortarget)

##### `impl Copy for ColorTarget`

##### `impl Debug for ColorTarget`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ColorTarget`

##### `impl PartialEq for ColorTarget`

- `fn eq(self: &Self, other: &ColorTarget) -> bool` — [`ColorTarget`](#colortarget)

##### `impl StructuralPartialEq for ColorTarget`

## Functions

### `next_bytes`

```rust
fn next_bytes(bytes: &mut &[u8], parser: &mut anstyle_parse::Parser, capture: &mut WinconCapture) -> Option<(anstyle::Style, String)>
```

### `to_ansi_color`

```rust
fn to_ansi_color(digit: u16) -> Option<anstyle::AnsiColor>
```

