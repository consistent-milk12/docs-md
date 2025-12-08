*[anstream](../../index.md) / [adapter](../index.md) / [wincon](index.md)*

---

# Module `wincon`

## Contents

- [Structs](#structs)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
  - [`WinconCapture`](#winconcapture)
- [Enums](#enums)
  - [`CsiState`](#csistate)
  - [`ColorTarget`](#colortarget)
- [Functions](#functions)
  - [`next_bytes`](#next_bytes)
  - [`to_ansi_color`](#to_ansi_color)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WinconBytes`](#winconbytes) | struct | Incrementally convert to wincon calls for non-contiguous data |
| [`WinconBytesIter`](#winconbytesiter) | struct | See [`WinconBytes`] |
| [`WinconCapture`](#winconcapture) | struct |  |
| [`CsiState`](#csistate) | enum |  |
| [`ColorTarget`](#colortarget) | enum |  |
| [`next_bytes`](#next_bytes) | fn |  |
| [`to_ansi_color`](#to_ansi_color) | fn |  |

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

- <span id="winconbytes-new"></span>`fn new() -> Self`

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](../index.md)

#### Trait Implementations

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](../index.md)

##### `impl Debug for WinconBytes`

- <span id="winconbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](../index.md)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](../index.md)

##### `impl StructuralPartialEq for WinconBytes`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

See [`WinconBytes`](../index.md)

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

- <span id="winconbytesiter-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](../index.md)

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

- <span id="winconcapture-reset"></span>`fn reset(&mut self)`

#### Trait Implementations

##### `impl Clone for WinconCapture`

- <span id="winconcapture-clone"></span>`fn clone(&self) -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Debug for WinconCapture`

- <span id="winconcapture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconCapture`

- <span id="winconcapture-default"></span>`fn default() -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Eq for WinconCapture`

##### `impl PartialEq for WinconCapture`

- <span id="winconcapture-eq"></span>`fn eq(&self, other: &WinconCapture) -> bool` — [`WinconCapture`](#winconcapture)

##### `impl Perform for WinconCapture`

- <span id="winconcapture-print"></span>`fn print(&mut self, c: char)`

- <span id="winconcapture-execute"></span>`fn execute(&mut self, byte: u8)`

- <span id="winconcapture-csi-dispatch"></span>`fn csi_dispatch(&mut self, params: &anstyle_parse::Params, _intermediates: &[u8], ignore: bool, action: u8)`

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

- <span id="csistate-clone"></span>`fn clone(&self) -> CsiState` — [`CsiState`](#csistate)

##### `impl Copy for CsiState`

##### `impl Debug for CsiState`

- <span id="csistate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CsiState`

##### `impl PartialEq for CsiState`

- <span id="csistate-eq"></span>`fn eq(&self, other: &CsiState) -> bool` — [`CsiState`](#csistate)

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

- <span id="colortarget-clone"></span>`fn clone(&self) -> ColorTarget` — [`ColorTarget`](#colortarget)

##### `impl Copy for ColorTarget`

##### `impl Debug for ColorTarget`

- <span id="colortarget-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColorTarget`

##### `impl PartialEq for ColorTarget`

- <span id="colortarget-eq"></span>`fn eq(&self, other: &ColorTarget) -> bool` — [`ColorTarget`](#colortarget)

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

