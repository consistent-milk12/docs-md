*[anstyle](../index.md) / [color](index.md)*

---

# Module `color`

## Structs

### `Ansi256Color`

```rust
struct Ansi256Color(u8);
```

256 (8-bit) color support

- `0..16` are [`AnsiColor`](../index.md) palette codes
- `0..232` map to [`RgbColor`](../index.md) color values
- `232..` map to [`RgbColor`](../index.md) gray-scale values

#### Implementations

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md), [`Style`](../index.md)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](../index.md)

- `const fn index(self: Self) -> u8`

- `const fn into_ansi(self: Self) -> Option<AnsiColor>` — [`AnsiColor`](../index.md)

- `const fn from_ansi(color: AnsiColor) -> Self` — [`AnsiColor`](../index.md)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

#### Trait Implementations

##### `impl Clone for Ansi256Color`

- `fn clone(self: &Self) -> Ansi256Color` — [`Ansi256Color`](../index.md)

##### `impl Copy for Ansi256Color`

##### `impl Debug for Ansi256Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Ansi256Color`

##### `impl Hash for Ansi256Color`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Ansi256Color`

- `fn cmp(self: &Self, other: &Ansi256Color) -> $crate::cmp::Ordering` — [`Ansi256Color`](../index.md)

##### `impl PartialEq for Ansi256Color`

- `fn eq(self: &Self, other: &Ansi256Color) -> bool` — [`Ansi256Color`](../index.md)

##### `impl PartialOrd for Ansi256Color`

- `fn partial_cmp(self: &Self, other: &Ansi256Color) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Ansi256Color`](../index.md)

##### `impl StructuralPartialEq for Ansi256Color`

### `RgbColor`

```rust
struct RgbColor(u8, u8, u8);
```

24-bit ANSI RGB color codes

#### Implementations

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md), [`Style`](../index.md)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](../index.md)

- `const fn r(self: Self) -> u8`

- `const fn g(self: Self) -> u8`

- `const fn b(self: Self) -> u8`

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

#### Trait Implementations

##### `impl Clone for RgbColor`

- `fn clone(self: &Self) -> RgbColor` — [`RgbColor`](../index.md)

##### `impl Copy for RgbColor`

##### `impl Debug for RgbColor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RgbColor`

##### `impl Hash for RgbColor`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for RgbColor`

- `fn cmp(self: &Self, other: &RgbColor) -> $crate::cmp::Ordering` — [`RgbColor`](../index.md)

##### `impl PartialEq for RgbColor`

- `fn eq(self: &Self, other: &RgbColor) -> bool` — [`RgbColor`](../index.md)

##### `impl PartialOrd for RgbColor`

- `fn partial_cmp(self: &Self, other: &RgbColor) -> $crate::option::Option<$crate::cmp::Ordering>` — [`RgbColor`](../index.md)

##### `impl StructuralPartialEq for RgbColor`

### `DisplayBuffer`

```rust
struct DisplayBuffer {
    buffer: [u8; 19],
    len: usize,
}
```

#### Implementations

- `fn write_str(self: Self, part: &'static str) -> Self`

- `fn write_code(self: Self, code: u8) -> Self`

- `fn as_str(self: &Self) -> &str`

- `fn write_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for DisplayBuffer`

- `fn clone(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

##### `impl Copy for DisplayBuffer`

##### `impl Debug for DisplayBuffer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DisplayBuffer`

- `fn default() -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

##### `impl Display for DisplayBuffer`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for DisplayBuffer`

- `fn to_string(self: &Self) -> String`

### `NullFormatter`

```rust
struct NullFormatter(&'static str);
```

#### Trait Implementations

##### `impl Clone for NullFormatter`

- `fn clone(self: &Self) -> NullFormatter` — [`NullFormatter`](#nullformatter)

##### `impl Copy for NullFormatter`

##### `impl Debug for NullFormatter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NullFormatter`

- `fn default() -> NullFormatter` — [`NullFormatter`](#nullformatter)

##### `impl Display for NullFormatter`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for NullFormatter`

- `fn to_string(self: &Self) -> String`

## Enums

### `Color`

```rust
enum Color {
    Ansi(AnsiColor),
    Ansi256(Ansi256Color),
    Rgb(RgbColor),
}
```

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

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md), [`Style`](../index.md)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](../index.md)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_fg_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_bg_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- `fn render_underline(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_underline_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for Color`

- `fn clone(self: &Self) -> Color` — [`Color`](../index.md)

##### `impl Copy for Color`

##### `impl Debug for Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Color`

##### `impl Hash for Color`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Color`

- `fn cmp(self: &Self, other: &Color) -> $crate::cmp::Ordering` — [`Color`](../index.md)

##### `impl PartialEq for Color`

- `fn eq(self: &Self, other: &Color) -> bool` — [`Color`](../index.md)

##### `impl PartialOrd for Color`

- `fn partial_cmp(self: &Self, other: &Color) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Color`](../index.md)

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

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](../index.md), [`Style`](../index.md)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](../index.md)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_str(self: &Self) -> &'static str`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_str(self: &Self) -> &'static str`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](#displaybuffer)

- `fn bright(self: Self, yes: bool) -> Self`

- `fn is_bright(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for AnsiColor`

- `fn clone(self: &Self) -> AnsiColor` — [`AnsiColor`](../index.md)

##### `impl Copy for AnsiColor`

##### `impl Debug for AnsiColor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AnsiColor`

##### `impl Hash for AnsiColor`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for AnsiColor`

- `fn cmp(self: &Self, other: &AnsiColor) -> $crate::cmp::Ordering` — [`AnsiColor`](../index.md)

##### `impl PartialEq for AnsiColor`

- `fn eq(self: &Self, other: &AnsiColor) -> bool` — [`AnsiColor`](../index.md)

##### `impl PartialOrd for AnsiColor`

- `fn partial_cmp(self: &Self, other: &AnsiColor) -> $crate::option::Option<$crate::cmp::Ordering>` — [`AnsiColor`](../index.md)

##### `impl StructuralPartialEq for AnsiColor`

## Constants

### `DISPLAY_BUFFER_CAPACITY`

```rust
const DISPLAY_BUFFER_CAPACITY: usize = 19usize;
```

