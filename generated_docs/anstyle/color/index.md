*[anstyle](../index.md) / [color](index.md)*

---

# Module `color`

## Contents

- [Structs](#structs)
  - [`Ansi256Color`](#ansi256color)
  - [`RgbColor`](#rgbcolor)
  - [`DisplayBuffer`](#displaybuffer)
  - [`NullFormatter`](#nullformatter)
- [Enums](#enums)
  - [`Color`](#color)
  - [`AnsiColor`](#ansicolor)
- [Constants](#constants)
  - [`DISPLAY_BUFFER_CAPACITY`](#display-buffer-capacity)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ansi256Color`](#ansi256color) | struct | 256 (8-bit) color support |
| [`RgbColor`](#rgbcolor) | struct | 24-bit ANSI RGB color codes |
| [`DisplayBuffer`](#displaybuffer) | struct |  |
| [`NullFormatter`](#nullformatter) | struct |  |
| [`Color`](#color) | enum | Any ANSI color code scheme |
| [`AnsiColor`](#ansicolor) | enum | Available 4-bit ANSI color palette codes |
| [`DISPLAY_BUFFER_CAPACITY`](#display-buffer-capacity) | const |  |

## Structs

### `Ansi256Color`

```rust
struct Ansi256Color(u8);
```

*Defined in [`anstyle-1.0.13/src/color.rs:352`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L352)*

256 (8-bit) color support

- `0..16` are [`AnsiColor`](../index.md) palette codes
- `0..232` map to [`RgbColor`](../index.md) color values
- `232..` map to [`RgbColor`](../index.md) gray-scale values

#### Implementations

- <span id="ansi256color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md#color), [`Style`](../index.md#style)

- <span id="ansi256color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](../index.md#style)

- <span id="ansi256color-index"></span>`const fn index(self) -> u8`

- <span id="ansi256color-into-ansi"></span>`const fn into_ansi(self) -> Option<AnsiColor>` — [`AnsiColor`](../index.md#ansicolor)

- <span id="ansi256color-from-ansi"></span>`const fn from_ansi(color: AnsiColor) -> Self` — [`AnsiColor`](../index.md#ansicolor)

- <span id="ansi256color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="ansi256color-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="ansi256color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="ansi256color-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="ansi256color-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

#### Trait Implementations

##### `impl Clone for Ansi256Color`

- <span id="ansi256color-clone"></span>`fn clone(&self) -> Ansi256Color` — [`Ansi256Color`](../index.md#ansi256color)

##### `impl Copy for Ansi256Color`

##### `impl Debug for Ansi256Color`

- <span id="ansi256color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ansi256Color`

##### `impl Hash for Ansi256Color`

- <span id="ansi256color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Ansi256Color`

- <span id="ansi256color-cmp"></span>`fn cmp(&self, other: &Ansi256Color) -> cmp::Ordering` — [`Ansi256Color`](../index.md#ansi256color)

##### `impl PartialEq for Ansi256Color`

- <span id="ansi256color-eq"></span>`fn eq(&self, other: &Ansi256Color) -> bool` — [`Ansi256Color`](../index.md#ansi256color)

##### `impl PartialOrd for Ansi256Color`

- <span id="ansi256color-partial-cmp"></span>`fn partial_cmp(&self, other: &Ansi256Color) -> option::Option<cmp::Ordering>` — [`Ansi256Color`](../index.md#ansi256color)

##### `impl StructuralPartialEq for Ansi256Color`

### `RgbColor`

```rust
struct RgbColor(u8, u8, u8);
```

*Defined in [`anstyle-1.0.13/src/color.rs:476`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L476)*

24-bit ANSI RGB color codes

#### Implementations

- <span id="rgbcolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md#color), [`Style`](../index.md#style)

- <span id="rgbcolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](../index.md#style)

- <span id="rgbcolor-r"></span>`const fn r(self) -> u8`

- <span id="rgbcolor-g"></span>`const fn g(self) -> u8`

- <span id="rgbcolor-b"></span>`const fn b(self) -> u8`

- <span id="rgbcolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="rgbcolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="rgbcolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="rgbcolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="rgbcolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

#### Trait Implementations

##### `impl Clone for RgbColor`

- <span id="rgbcolor-clone"></span>`fn clone(&self) -> RgbColor` — [`RgbColor`](../index.md#rgbcolor)

##### `impl Copy for RgbColor`

##### `impl Debug for RgbColor`

- <span id="rgbcolor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RgbColor`

##### `impl Hash for RgbColor`

- <span id="rgbcolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for RgbColor`

- <span id="rgbcolor-cmp"></span>`fn cmp(&self, other: &RgbColor) -> cmp::Ordering` — [`RgbColor`](../index.md#rgbcolor)

##### `impl PartialEq for RgbColor`

- <span id="rgbcolor-eq"></span>`fn eq(&self, other: &RgbColor) -> bool` — [`RgbColor`](../index.md#rgbcolor)

##### `impl PartialOrd for RgbColor`

- <span id="rgbcolor-partial-cmp"></span>`fn partial_cmp(&self, other: &RgbColor) -> option::Option<cmp::Ordering>` — [`RgbColor`](../index.md#rgbcolor)

##### `impl StructuralPartialEq for RgbColor`

### `DisplayBuffer`

```rust
struct DisplayBuffer {
    buffer: [u8; 19],
    len: usize,
}
```

*Defined in [`anstyle-1.0.13/src/color.rs:571-574`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L571-L574)*

#### Implementations

- <span id="displaybuffer-write-str"></span>`fn write_str(self, part: &'static str) -> Self`

- <span id="displaybuffer-write-code"></span>`fn write_code(self, code: u8) -> Self`

- <span id="displaybuffer-as-str"></span>`fn as_str(&self) -> &str`

- <span id="displaybuffer-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for DisplayBuffer`

- <span id="displaybuffer-clone"></span>`fn clone(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

##### `impl Copy for DisplayBuffer`

##### `impl Debug for DisplayBuffer`

- <span id="displaybuffer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DisplayBuffer`

- <span id="displaybuffer-default"></span>`fn default() -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

##### `impl Display for DisplayBuffer`

- <span id="displaybuffer-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for DisplayBuffer`

- <span id="displaybuffer-to-string"></span>`fn to_string(&self) -> String`

### `NullFormatter`

```rust
struct NullFormatter(&'static str);
```

*Defined in [`anstyle-1.0.13/src/color.rs:635`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L635)*

#### Trait Implementations

##### `impl Clone for NullFormatter`

- <span id="nullformatter-clone"></span>`fn clone(&self) -> NullFormatter` — [`NullFormatter`](#nullformatter)

##### `impl Copy for NullFormatter`

##### `impl Debug for NullFormatter`

- <span id="nullformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NullFormatter`

- <span id="nullformatter-default"></span>`fn default() -> NullFormatter` — [`NullFormatter`](#nullformatter)

##### `impl Display for NullFormatter`

- <span id="nullformatter-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for NullFormatter`

- <span id="nullformatter-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Color`

```rust
enum Color {
    Ansi(AnsiColor),
    Ansi256(Ansi256Color),
    Rgb(RgbColor),
}
```

*Defined in [`anstyle-1.0.13/src/color.rs:4-17`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L4-L17)*

Any ANSI color code scheme

#### Variants

- **`Ansi`**

  Available 4-bit ANSI color palette codes
  
  The user's terminal defines the meaning of the each palette code.

- **`Ansi256`**

  256 (8-bit) color support
  
  - `0..16` are [`AnsiColor`](../index.md) palette codes
  - `0..232` map to [`RgbColor`](../index.md) color values
  - `232..` map to [`RgbColor`](../index.md) gray-scale values

- **`Rgb`**

  24-bit ANSI RGB color codes

#### Implementations

- <span id="color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md#color), [`Style`](../index.md#style)

- <span id="color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](../index.md#style)

- <span id="color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-fg-to"></span>`fn write_fg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-bg-to"></span>`fn write_bg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-underline"></span>`fn render_underline(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-underline-to"></span>`fn write_underline_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for Color`

- <span id="color-clone"></span>`fn clone(&self) -> Color` — [`Color`](../index.md#color)

##### `impl Copy for Color`

##### `impl Debug for Color`

- <span id="color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Color`

##### `impl Hash for Color`

- <span id="color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Color`

- <span id="color-cmp"></span>`fn cmp(&self, other: &Color) -> cmp::Ordering` — [`Color`](../index.md#color)

##### `impl PartialEq for Color`

- <span id="color-eq"></span>`fn eq(&self, other: &Color) -> bool` — [`Color`](../index.md#color)

##### `impl PartialOrd for Color`

- <span id="color-partial-cmp"></span>`fn partial_cmp(&self, other: &Color) -> option::Option<cmp::Ordering>` — [`Color`](../index.md#color)

##### `impl StructuralPartialEq for Color`

### `AnsiColor`

```rust
enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
```

*Defined in [`anstyle-1.0.13/src/color.rs:138-186`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L138-L186)*

Available 4-bit ANSI color palette codes

The user's terminal defines the meaning of the each palette code.

#### Variants

- **`Black`**

  Black: #0 (foreground code `30`, background code `40`).

- **`Red`**

  Red: #1 (foreground code `31`, background code `41`).

- **`Green`**

  Green: #2 (foreground code `32`, background code `42`).

- **`Yellow`**

  Yellow: #3 (foreground code `33`, background code `43`).

- **`Blue`**

  Blue: #4 (foreground code `34`, background code `44`).

- **`Magenta`**

  Magenta: #5 (foreground code `35`, background code `45`).

- **`Cyan`**

  Cyan: #6 (foreground code `36`, background code `46`).

- **`White`**

  White: #7 (foreground code `37`, background code `47`).

- **`BrightBlack`**

  Bright black: #0 (foreground code `90`, background code `100`).

- **`BrightRed`**

  Bright red: #1 (foreground code `91`, background code `101`).

- **`BrightGreen`**

  Bright green: #2 (foreground code `92`, background code `102`).

- **`BrightYellow`**

  Bright yellow: #3 (foreground code `93`, background code `103`).

- **`BrightBlue`**

  Bright blue: #4 (foreground code `94`, background code `104`).

- **`BrightMagenta`**

  Bright magenta: #5 (foreground code `95`, background code `105`).

- **`BrightCyan`**

  Bright cyan: #6 (foreground code `96`, background code `106`).

- **`BrightWhite`**

  Bright white: #7 (foreground code `97`, background code `107`).

#### Implementations

- <span id="ansicolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md#color), [`Style`](../index.md#style)

- <span id="ansicolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](../index.md#style)

- <span id="ansicolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="ansicolor-as-fg-str"></span>`fn as_fg_str(&self) -> &'static str`

- <span id="ansicolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="ansicolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="ansicolor-as-bg-str"></span>`fn as_bg_str(&self) -> &'static str`

- <span id="ansicolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="ansicolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- <span id="ansicolor-bright"></span>`fn bright(self, yes: bool) -> Self`

- <span id="ansicolor-is-bright"></span>`fn is_bright(self) -> bool`

#### Trait Implementations

##### `impl Clone for AnsiColor`

- <span id="ansicolor-clone"></span>`fn clone(&self) -> AnsiColor` — [`AnsiColor`](../index.md#ansicolor)

##### `impl Copy for AnsiColor`

##### `impl Debug for AnsiColor`

- <span id="ansicolor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColor`

##### `impl Hash for AnsiColor`

- <span id="ansicolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for AnsiColor`

- <span id="ansicolor-cmp"></span>`fn cmp(&self, other: &AnsiColor) -> cmp::Ordering` — [`AnsiColor`](../index.md#ansicolor)

##### `impl PartialEq for AnsiColor`

- <span id="ansicolor-eq"></span>`fn eq(&self, other: &AnsiColor) -> bool` — [`AnsiColor`](../index.md#ansicolor)

##### `impl PartialOrd for AnsiColor`

- <span id="ansicolor-partial-cmp"></span>`fn partial_cmp(&self, other: &AnsiColor) -> option::Option<cmp::Ordering>` — [`AnsiColor`](../index.md#ansicolor)

##### `impl StructuralPartialEq for AnsiColor`

## Constants

### `DISPLAY_BUFFER_CAPACITY`
```rust
const DISPLAY_BUFFER_CAPACITY: usize = 19usize;
```

*Defined in [`anstyle-1.0.13/src/color.rs:568`](../../../.source_1765210505/anstyle-1.0.13/src/color.rs#L568)*

