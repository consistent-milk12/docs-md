# Crate `anstyle`

ANSI Text Styling

*A portmanteau of "ansi style"*

`anstyle` provides core types describing [ANSI styling escape
codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for interoperability
between crates.

Example use cases:
- An argument parser allowing callers to define the colors used in the help-output without
  putting the text formatting crate in the public API
- A style description parser that can work with any text formatting crate

Priorities:
1. API stability
2. Low compile-time and binary-size overhead
3. `const` friendly API for callers to statically define their stylesheet

For integration with text styling crate, see:
- [anstyle-ansi-term](https://docs.rs/anstyle-ansi-term)
- [anstyle-crossterm](https://docs.rs/anstyle-crossterm)
- [anstyle-owo-colors](https://docs.rs/anstyle-owo-colors)
- [anstyle-termcolor](https://docs.rs/anstyle-termcolor)
- [anstyle-yansi](https://docs.rs/anstyle-yansi)

User-styling parsers:
- [anstyle-git](https://docs.rs/anstyle-git): Parse Git style descriptions
- [anstyle-ls](https://docs.rs/anstyle-ls): Parse `LS_COLORS` style descriptions

Convert to other formats
- [anstream](https://docs.rs/anstream): A simple cross platform library for writing colored text to a terminal
- [anstyle-roff](https://docs.rs/anstyle-roff): For converting to ROFF
- [anstyle-syntect](https://docs.rs/anstyle-syntect): For working with syntax highlighting

Utilities
- [anstyle-lossy](https://docs.rs/anstyle-lossy): Convert between `anstyle::Color` types
- [anstyle-parse](https://docs.rs/anstyle-parse): Parsing ANSI Style Escapes
- [anstyle-wincon](https://docs.rs/anstyle-wincon): Styling legacy Microsoft terminals

# Examples

The core type is [`Style`](#style):
```rust
let style = anstyle::Style::new().bold();
```

## Modules

- [`macros`](macros/index.md) - 
- [`color`](color/index.md) - 
- [`effect`](effect/index.md) - 
- [`reset`](reset/index.md) - 
- [`style`](style/index.md) - 

## Structs

### `Ansi256Color`

```rust
struct Ansi256Color(u8);
```

256 (8-bit) color support

- `0..16` are [`AnsiColor`](#ansicolor) palette codes
- `0..232` map to [`RgbColor`](#rgbcolor) color values
- `232..` map to [`RgbColor`](#rgbcolor) gray-scale values

#### Implementations

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](#style)

- `const fn index(self: Self) -> u8`

- `const fn into_ansi(self: Self) -> Option<AnsiColor>` — [`AnsiColor`](#ansicolor)

- `const fn from_ansi(color: AnsiColor) -> Self` — [`AnsiColor`](#ansicolor)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

#### Trait Implementations

##### `impl Clone for Ansi256Color`

- `fn clone(self: &Self) -> Ansi256Color` — [`Ansi256Color`](#ansi256color)

##### `impl Copy for Ansi256Color`

##### `impl Debug for Ansi256Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Ansi256Color`

##### `impl Hash for Ansi256Color`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Ansi256Color`

- `fn cmp(self: &Self, other: &Ansi256Color) -> $crate::cmp::Ordering` — [`Ansi256Color`](#ansi256color)

##### `impl PartialEq for Ansi256Color`

- `fn eq(self: &Self, other: &Ansi256Color) -> bool` — [`Ansi256Color`](#ansi256color)

##### `impl PartialOrd for Ansi256Color`

- `fn partial_cmp(self: &Self, other: &Ansi256Color) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Ansi256Color`](#ansi256color)

##### `impl StructuralPartialEq for Ansi256Color`

### `RgbColor`

```rust
struct RgbColor(u8, u8, u8);
```

24-bit ANSI RGB color codes

#### Implementations

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](#style)

- `const fn r(self: Self) -> u8`

- `const fn g(self: Self) -> u8`

- `const fn b(self: Self) -> u8`

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

#### Trait Implementations

##### `impl Clone for RgbColor`

- `fn clone(self: &Self) -> RgbColor` — [`RgbColor`](#rgbcolor)

##### `impl Copy for RgbColor`

##### `impl Debug for RgbColor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for RgbColor`

##### `impl Hash for RgbColor`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for RgbColor`

- `fn cmp(self: &Self, other: &RgbColor) -> $crate::cmp::Ordering` — [`RgbColor`](#rgbcolor)

##### `impl PartialEq for RgbColor`

- `fn eq(self: &Self, other: &RgbColor) -> bool` — [`RgbColor`](#rgbcolor)

##### `impl PartialOrd for RgbColor`

- `fn partial_cmp(self: &Self, other: &RgbColor) -> $crate::option::Option<$crate::cmp::Ordering>` — [`RgbColor`](#rgbcolor)

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

- `fn clone(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

##### `impl Copy for DisplayBuffer`

##### `impl Debug for DisplayBuffer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DisplayBuffer`

- `fn default() -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

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

- `fn clone(self: &Self) -> NullFormatter` — [`NullFormatter`](color/index.md)

##### `impl Copy for NullFormatter`

##### `impl Debug for NullFormatter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NullFormatter`

- `fn default() -> NullFormatter` — [`NullFormatter`](color/index.md)

##### `impl Display for NullFormatter`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for NullFormatter`

- `fn to_string(self: &Self) -> String`

### `Effects`

```rust
struct Effects(u16);
```

A set of text effects

# Examples

```rust
let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
```

#### Implementations

- `const PLAIN: Self`

- `const BOLD: Self`

- `const DIMMED: Self`

- `const ITALIC: Self`

- `const UNDERLINE: Self`

- `const DOUBLE_UNDERLINE: Self`

- `const CURLY_UNDERLINE: Self`

- `const DOTTED_UNDERLINE: Self`

- `const DASHED_UNDERLINE: Self`

- `const BLINK: Self`

- `const INVERT: Self`

- `const HIDDEN: Self`

- `const STRIKETHROUGH: Self`

- `const fn new() -> Self`

- `const fn is_plain(self: Self) -> bool`

- `const fn contains(self: Self, other: Effects) -> bool` — [`Effects`](#effects)

- `const fn insert(self: Self, other: Effects) -> Self` — [`Effects`](#effects)

- `const fn remove(self: Self, other: Effects) -> Self` — [`Effects`](#effects)

- `const fn clear(self: Self) -> Self`

- `const fn set(self: Self, other: Self, enable: bool) -> Self`

- `fn iter(self: Self) -> EffectIter` — [`EffectIter`](#effectiter)

- `fn index_iter(self: Self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md)

- `fn render(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Effects`

- `type Output = Effects`

- `fn bitor(self: Self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl Clone for Effects`

- `fn clone(self: &Self) -> Effects` — [`Effects`](#effects)

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- `fn default() -> Effects` — [`Effects`](#effects)

##### `impl Eq for Effects`

##### `impl Hash for Effects`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Effects`

- `fn cmp(self: &Self, other: &Effects) -> $crate::cmp::Ordering` — [`Effects`](#effects)

##### `impl PartialEq for Effects`

- `fn eq(self: &Self, other: &Effects) -> bool` — [`Effects`](#effects)

##### `impl PartialOrd for Effects`

- `fn partial_cmp(self: &Self, other: &Effects) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Effects`](#effects)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- `type Output = Effects`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- `fn sub_assign(self: &mut Self, other: Self)`

### `Metadata`

```rust
struct Metadata {
    name: &'static str,
    escape: &'static str,
}
```

### `EffectsDisplay`

```rust
struct EffectsDisplay(Effects);
```

#### Trait Implementations

##### `impl Clone for EffectsDisplay`

- `fn clone(self: &Self) -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md)

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for EffectsDisplay`

- `fn default() -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md)

##### `impl Display for EffectsDisplay`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for EffectsDisplay`

- `fn to_string(self: &Self) -> String`

### `EffectIter`

```rust
struct EffectIter {
    index: usize,
    effects: Effects,
}
```

Enumerate each enabled value in [`Effects`](#effects)

#### Trait Implementations

##### `impl Clone for EffectIter`

- `fn clone(self: &Self) -> EffectIter` — [`EffectIter`](#effectiter)

##### `impl Debug for EffectIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for EffectIter`

##### `impl<I> IntoIterator for EffectIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for EffectIter`

- `type Item = Effects`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- `fn eq(self: &Self, other: &EffectIter) -> bool` — [`EffectIter`](#effectiter)

##### `impl StructuralPartialEq for EffectIter`

### `EffectIndexIter`

```rust
struct EffectIndexIter {
    index: usize,
    effects: Effects,
}
```

#### Trait Implementations

##### `impl Clone for EffectIndexIter`

- `fn clone(self: &Self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md)

##### `impl Debug for EffectIndexIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl<I> IntoIterator for EffectIndexIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for EffectIndexIter`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- `fn eq(self: &Self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](effect/index.md)

##### `impl StructuralPartialEq for EffectIndexIter`

### `Reset`

```rust
struct Reset;
```

Reset terminal formatting

#### Implementations

- `fn render(self: Self) -> impl core::fmt::Display + Copy`

#### Trait Implementations

##### `impl Clone for Reset`

- `fn clone(self: &Self) -> Reset` — [`Reset`](#reset)

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Reset`

- `fn default() -> Reset` — [`Reset`](#reset)

##### `impl Display for Reset`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl Hash for Reset`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Reset`

- `fn cmp(self: &Self, other: &Reset) -> $crate::cmp::Ordering` — [`Reset`](#reset)

##### `impl PartialEq for Reset`

- `fn eq(self: &Self, other: &Reset) -> bool` — [`Reset`](#reset)

##### `impl PartialOrd for Reset`

- `fn partial_cmp(self: &Self, other: &Reset) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Reset`](#reset)

##### `impl StructuralPartialEq for Reset`

##### `impl<T> ToString for Reset`

- `fn to_string(self: &Self) -> String`

### `Style`

```rust
struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}
```

ANSI Text styling

You can print a `Style` to render the corresponding ANSI code.
Using the alternate flag `#` will render the ANSI reset code, if needed.
Together, this makes it convenient to render styles using inline format arguments.

# Examples

```rust
let style = anstyle::Style::new().bold();

let value = 42;
println!("{style}{value}{style:#}");
```

#### Implementations

- `const fn get_fg_color(self: Self) -> Option<crate::Color>` — [`Color`](#color)

- `const fn get_bg_color(self: Self) -> Option<crate::Color>` — [`Color`](#color)

- `const fn get_underline_color(self: Self) -> Option<crate::Color>` — [`Color`](#color)

- `const fn get_effects(self: Self) -> crate::Effects` — [`Effects`](#effects)

- `const fn is_plain(self: Self) -> bool`

#### Trait Implementations

##### `impl BitOr for Style`

- `type Output = Style`

- `fn bitor(self: Self, rhs: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl BitOrAssign for Style`

- `fn bitor_assign(self: &mut Self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Style`

- `fn default() -> Style` — [`Style`](#style)

##### `impl Display for Style`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Style`

- `fn cmp(self: &Self, other: &Style) -> $crate::cmp::Ordering` — [`Style`](#style)

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &crate::Effects) -> bool` — [`Effects`](#effects)

##### `impl PartialOrd for Style`

- `fn partial_cmp(self: &Self, other: &Style) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- `type Output = Style`

- `fn sub(self: Self, other: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl SubAssign for Style`

- `fn sub_assign(self: &mut Self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl<T> ToString for Style`

- `fn to_string(self: &Self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

#### Trait Implementations

##### `impl Clone for StyleDisplay`

- `fn clone(self: &Self) -> StyleDisplay` — [`StyleDisplay`](style/index.md)

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StyleDisplay`

- `fn default() -> StyleDisplay` — [`StyleDisplay`](style/index.md)

##### `impl Display for StyleDisplay`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for StyleDisplay`

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
  
  - `0..16` are [`AnsiColor`](#ansicolor) palette codes
  - `0..232` map to [`RgbColor`](#rgbcolor) color values
  - `232..` map to [`RgbColor`](#rgbcolor) gray-scale values

- **`Rgb`**

  24-bit ANSI RGB color codes

#### Implementations

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](#style)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_fg_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_bg_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- `fn render_underline(self: Self) -> impl core::fmt::Display + Copy`

- `fn write_underline_to(self: Self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for Color`

- `fn clone(self: &Self) -> Color` — [`Color`](#color)

##### `impl Copy for Color`

##### `impl Debug for Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Color`

##### `impl Hash for Color`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Color`

- `fn cmp(self: &Self, other: &Color) -> $crate::cmp::Ordering` — [`Color`](#color)

##### `impl PartialEq for Color`

- `fn eq(self: &Self, other: &Color) -> bool` — [`Color`](#color)

##### `impl PartialOrd for Color`

- `fn partial_cmp(self: &Self, other: &Color) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Color`](#color)

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

- `fn on(self: Self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- `const fn on_default(self: Self) -> crate::Style` — [`Style`](#style)

- `fn render_fg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_fg_str(self: &Self) -> &'static str`

- `fn as_fg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn render_bg(self: Self) -> impl core::fmt::Display + Copy`

- `fn as_bg_str(self: &Self) -> &'static str`

- `fn as_bg_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn as_underline_buffer(self: &Self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- `fn bright(self: Self, yes: bool) -> Self`

- `fn is_bright(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for AnsiColor`

- `fn clone(self: &Self) -> AnsiColor` — [`AnsiColor`](#ansicolor)

##### `impl Copy for AnsiColor`

##### `impl Debug for AnsiColor`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AnsiColor`

##### `impl Hash for AnsiColor`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for AnsiColor`

- `fn cmp(self: &Self, other: &AnsiColor) -> $crate::cmp::Ordering` — [`AnsiColor`](#ansicolor)

##### `impl PartialEq for AnsiColor`

- `fn eq(self: &Self, other: &AnsiColor) -> bool` — [`AnsiColor`](#ansicolor)

##### `impl PartialOrd for AnsiColor`

- `fn partial_cmp(self: &Self, other: &AnsiColor) -> $crate::option::Option<$crate::cmp::Ordering>` — [`AnsiColor`](#ansicolor)

##### `impl StructuralPartialEq for AnsiColor`

## Constants

### `DISPLAY_BUFFER_CAPACITY`

```rust
const DISPLAY_BUFFER_CAPACITY: usize = 19usize;
```

### `METADATA`

```rust
const METADATA: [Metadata; 12];
```

### `RESET`

```rust
const RESET: &str;
```

