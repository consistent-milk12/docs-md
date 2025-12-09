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

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`color`](#color)
  - [`effect`](#effect)
  - [`reset`](#reset)
  - [`style`](#style)
- [Structs](#structs)
  - [`Ansi256Color`](#ansi256color)
  - [`RgbColor`](#rgbcolor)
  - [`DisplayBuffer`](#displaybuffer)
  - [`NullFormatter`](#nullformatter)
  - [`Effects`](#effects)
  - [`Metadata`](#metadata)
  - [`EffectsDisplay`](#effectsdisplay)
  - [`EffectIter`](#effectiter)
  - [`EffectIndexIter`](#effectindexiter)
  - [`Reset`](#reset)
  - [`Style`](#style)
  - [`StyleDisplay`](#styledisplay)
- [Enums](#enums)
  - [`Color`](#color)
  - [`AnsiColor`](#ansicolor)
- [Constants](#constants)
  - [`DISPLAY_BUFFER_CAPACITY`](#display_buffer_capacity)
  - [`METADATA`](#metadata)
  - [`RESET`](#reset)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`color`](#color) | mod |  |
| [`effect`](#effect) | mod |  |
| [`reset`](#reset) | mod |  |
| [`style`](#style) | mod |  |
| [`Ansi256Color`](#ansi256color) | struct | 256 (8-bit) color support |
| [`RgbColor`](#rgbcolor) | struct | 24-bit ANSI RGB color codes |
| [`DisplayBuffer`](#displaybuffer) | struct |  |
| [`NullFormatter`](#nullformatter) | struct |  |
| [`Effects`](#effects) | struct | A set of text effects |
| [`Metadata`](#metadata) | struct |  |
| [`EffectsDisplay`](#effectsdisplay) | struct |  |
| [`EffectIter`](#effectiter) | struct | Enumerate each enabled value in [`Effects`] |
| [`EffectIndexIter`](#effectindexiter) | struct |  |
| [`Reset`](#reset) | struct | Reset terminal formatting |
| [`Style`](#style) | struct | ANSI Text styling |
| [`StyleDisplay`](#styledisplay) | struct |  |
| [`Color`](#color) | enum | Any ANSI color code scheme |
| [`AnsiColor`](#ansicolor) | enum | Available 4-bit ANSI color palette codes |
| [`DISPLAY_BUFFER_CAPACITY`](#display_buffer_capacity) | const |  |
| [`METADATA`](#metadata) | const |  |
| [`RESET`](#reset) | const |  |

## Modules

- [`macros`](macros/index.md)
- [`color`](color/index.md)
- [`effect`](effect/index.md)
- [`reset`](reset/index.md)
- [`style`](style/index.md)

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

- <span id="ansi256color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- <span id="ansi256color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

- <span id="ansi256color-index"></span>`const fn index(self) -> u8`

- <span id="ansi256color-into-ansi"></span>`const fn into_ansi(self) -> Option<AnsiColor>` — [`AnsiColor`](#ansicolor)

- <span id="ansi256color-from-ansi"></span>`const fn from_ansi(color: AnsiColor) -> Self` — [`AnsiColor`](#ansicolor)

- <span id="ansi256color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="ansi256color-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="ansi256color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="ansi256color-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="ansi256color-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

#### Trait Implementations

##### `impl Clone for Ansi256Color`

- <span id="ansi256color-clone"></span>`fn clone(&self) -> Ansi256Color` — [`Ansi256Color`](#ansi256color)

##### `impl Copy for Ansi256Color`

##### `impl Debug for Ansi256Color`

- <span id="ansi256color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ansi256Color`

##### `impl Hash for Ansi256Color`

- <span id="ansi256color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Ansi256Color`

- <span id="ansi256color-cmp"></span>`fn cmp(&self, other: &Ansi256Color) -> cmp::Ordering` — [`Ansi256Color`](#ansi256color)

##### `impl PartialEq for Ansi256Color`

- <span id="ansi256color-eq"></span>`fn eq(&self, other: &Ansi256Color) -> bool` — [`Ansi256Color`](#ansi256color)

##### `impl PartialOrd for Ansi256Color`

- <span id="ansi256color-partial-cmp"></span>`fn partial_cmp(&self, other: &Ansi256Color) -> option::Option<cmp::Ordering>` — [`Ansi256Color`](#ansi256color)

##### `impl StructuralPartialEq for Ansi256Color`

### `RgbColor`

```rust
struct RgbColor(u8, u8, u8);
```

24-bit ANSI RGB color codes

#### Implementations

- <span id="rgbcolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- <span id="rgbcolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

- <span id="rgbcolor-r"></span>`const fn r(self) -> u8`

- <span id="rgbcolor-g"></span>`const fn g(self) -> u8`

- <span id="rgbcolor-b"></span>`const fn b(self) -> u8`

- <span id="rgbcolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="rgbcolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="rgbcolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="rgbcolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="rgbcolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

#### Trait Implementations

##### `impl Clone for RgbColor`

- <span id="rgbcolor-clone"></span>`fn clone(&self) -> RgbColor` — [`RgbColor`](#rgbcolor)

##### `impl Copy for RgbColor`

##### `impl Debug for RgbColor`

- <span id="rgbcolor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RgbColor`

##### `impl Hash for RgbColor`

- <span id="rgbcolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for RgbColor`

- <span id="rgbcolor-cmp"></span>`fn cmp(&self, other: &RgbColor) -> cmp::Ordering` — [`RgbColor`](#rgbcolor)

##### `impl PartialEq for RgbColor`

- <span id="rgbcolor-eq"></span>`fn eq(&self, other: &RgbColor) -> bool` — [`RgbColor`](#rgbcolor)

##### `impl PartialOrd for RgbColor`

- <span id="rgbcolor-partial-cmp"></span>`fn partial_cmp(&self, other: &RgbColor) -> option::Option<cmp::Ordering>` — [`RgbColor`](#rgbcolor)

##### `impl StructuralPartialEq for RgbColor`

### `DisplayBuffer`

```rust
struct DisplayBuffer {
    buffer: [u8; 19],
    len: usize,
}
```

#### Implementations

- <span id="displaybuffer-write-str"></span>`fn write_str(self, part: &'static str) -> Self`

- <span id="displaybuffer-write-code"></span>`fn write_code(self, code: u8) -> Self`

- <span id="displaybuffer-as-str"></span>`fn as_str(&self) -> &str`

- <span id="displaybuffer-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for DisplayBuffer`

- <span id="displaybuffer-clone"></span>`fn clone(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

##### `impl Copy for DisplayBuffer`

##### `impl Debug for DisplayBuffer`

- <span id="displaybuffer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DisplayBuffer`

- <span id="displaybuffer-default"></span>`fn default() -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

##### `impl Display for DisplayBuffer`

- <span id="displaybuffer-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for DisplayBuffer`

- <span id="displaybuffer-to-string"></span>`fn to_string(&self) -> String`

### `NullFormatter`

```rust
struct NullFormatter(&'static str);
```

#### Trait Implementations

##### `impl Clone for NullFormatter`

- <span id="nullformatter-clone"></span>`fn clone(&self) -> NullFormatter` — [`NullFormatter`](color/index.md)

##### `impl Copy for NullFormatter`

##### `impl Debug for NullFormatter`

- <span id="nullformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NullFormatter`

- <span id="nullformatter-default"></span>`fn default() -> NullFormatter` — [`NullFormatter`](color/index.md)

##### `impl Display for NullFormatter`

- <span id="nullformatter-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for NullFormatter`

- <span id="nullformatter-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="effects-plain"></span>`const PLAIN: Self`

- <span id="effects-bold"></span>`const BOLD: Self`

- <span id="effects-dimmed"></span>`const DIMMED: Self`

- <span id="effects-italic"></span>`const ITALIC: Self`

- <span id="effects-underline"></span>`const UNDERLINE: Self`

- <span id="effects-double-underline"></span>`const DOUBLE_UNDERLINE: Self`

- <span id="effects-curly-underline"></span>`const CURLY_UNDERLINE: Self`

- <span id="effects-dotted-underline"></span>`const DOTTED_UNDERLINE: Self`

- <span id="effects-dashed-underline"></span>`const DASHED_UNDERLINE: Self`

- <span id="effects-blink"></span>`const BLINK: Self`

- <span id="effects-invert"></span>`const INVERT: Self`

- <span id="effects-hidden"></span>`const HIDDEN: Self`

- <span id="effects-strikethrough"></span>`const STRIKETHROUGH: Self`

- <span id="effects-new"></span>`const fn new() -> Self`

- <span id="effects-is-plain"></span>`const fn is_plain(self) -> bool`

- <span id="effects-contains"></span>`const fn contains(self, other: Effects) -> bool` — [`Effects`](#effects)

- <span id="effects-insert"></span>`const fn insert(self, other: Effects) -> Self` — [`Effects`](#effects)

- <span id="effects-remove"></span>`const fn remove(self, other: Effects) -> Self` — [`Effects`](#effects)

- <span id="effects-clear"></span>`const fn clear(self) -> Self`

- <span id="effects-set"></span>`const fn set(self, other: Self, enable: bool) -> Self`

- <span id="effects-iter"></span>`fn iter(self) -> EffectIter` — [`EffectIter`](#effectiter)

- <span id="effects-index-iter"></span>`fn index_iter(self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md)

- <span id="effects-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

- <span id="effects-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Effects`

- <span id="effects-output"></span>`type Output = Effects`

- <span id="effects-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- <span id="effects-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl Clone for Effects`

- <span id="effects-clone"></span>`fn clone(&self) -> Effects` — [`Effects`](#effects)

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- <span id="effects-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- <span id="effects-default"></span>`fn default() -> Effects` — [`Effects`](#effects)

##### `impl Eq for Effects`

##### `impl Hash for Effects`

- <span id="effects-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Effects`

- <span id="effects-cmp"></span>`fn cmp(&self, other: &Effects) -> cmp::Ordering` — [`Effects`](#effects)

##### `impl PartialEq for Effects`

- <span id="effects-eq"></span>`fn eq(&self, other: &Effects) -> bool` — [`Effects`](#effects)

##### `impl PartialOrd for Effects`

- <span id="effects-partial-cmp"></span>`fn partial_cmp(&self, other: &Effects) -> option::Option<cmp::Ordering>` — [`Effects`](#effects)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- <span id="effects-output"></span>`type Output = Effects`

- <span id="effects-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- <span id="effects-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

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

- <span id="effectsdisplay-clone"></span>`fn clone(&self) -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md)

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- <span id="effectsdisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EffectsDisplay`

- <span id="effectsdisplay-default"></span>`fn default() -> EffectsDisplay` — [`EffectsDisplay`](effect/index.md)

##### `impl Display for EffectsDisplay`

- <span id="effectsdisplay-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for EffectsDisplay`

- <span id="effectsdisplay-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="effectiter-clone"></span>`fn clone(&self) -> EffectIter` — [`EffectIter`](#effectiter)

##### `impl Debug for EffectIter`

- <span id="effectiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIter`

##### `impl<I> IntoIterator for EffectIter`

- <span id="effectiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectiter-intoiter"></span>`type IntoIter = I`

- <span id="effectiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIter`

- <span id="effectiter-item"></span>`type Item = Effects`

- <span id="effectiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- <span id="effectiter-eq"></span>`fn eq(&self, other: &EffectIter) -> bool` — [`EffectIter`](#effectiter)

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

- <span id="effectindexiter-clone"></span>`fn clone(&self) -> EffectIndexIter` — [`EffectIndexIter`](effect/index.md)

##### `impl Debug for EffectIndexIter`

- <span id="effectindexiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl<I> IntoIterator for EffectIndexIter`

- <span id="effectindexiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectindexiter-intoiter"></span>`type IntoIter = I`

- <span id="effectindexiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIndexIter`

- <span id="effectindexiter-item"></span>`type Item = usize`

- <span id="effectindexiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- <span id="effectindexiter-eq"></span>`fn eq(&self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](effect/index.md)

##### `impl StructuralPartialEq for EffectIndexIter`

### `Reset`

```rust
struct Reset;
```

Reset terminal formatting

#### Implementations

- <span id="reset-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

#### Trait Implementations

##### `impl Clone for Reset`

- <span id="reset-clone"></span>`fn clone(&self) -> Reset` — [`Reset`](#reset)

##### `impl Copy for Reset`

##### `impl Debug for Reset`

- <span id="reset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Reset`

- <span id="reset-default"></span>`fn default() -> Reset` — [`Reset`](#reset)

##### `impl Display for Reset`

- <span id="reset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Reset`

##### `impl Hash for Reset`

- <span id="reset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Reset`

- <span id="reset-cmp"></span>`fn cmp(&self, other: &Reset) -> cmp::Ordering` — [`Reset`](#reset)

##### `impl PartialEq for Reset`

- <span id="reset-eq"></span>`fn eq(&self, other: &Reset) -> bool` — [`Reset`](#reset)

##### `impl PartialOrd for Reset`

- <span id="reset-partial-cmp"></span>`fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>` — [`Reset`](#reset)

##### `impl StructuralPartialEq for Reset`

##### `impl<T> ToString for Reset`

- <span id="reset-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="style-bold"></span>`const fn bold(self) -> Self`

- <span id="style-dimmed"></span>`const fn dimmed(self) -> Self`

- <span id="style-italic"></span>`const fn italic(self) -> Self`

- <span id="style-underline"></span>`const fn underline(self) -> Self`

- <span id="style-blink"></span>`const fn blink(self) -> Self`

- <span id="style-invert"></span>`const fn invert(self) -> Self`

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

#### Trait Implementations

##### `impl BitOr for Style`

- <span id="style-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl BitOrAssign for Style`

- <span id="style-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](#style)

##### `impl Display for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Style`

- <span id="style-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](#style)

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](#style)

##### `impl PartialOrd for Style`

- <span id="style-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](#effects)

##### `impl SubAssign for Style`

- <span id="style-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](#effects)

##### `impl<T> ToString for Style`

- <span id="style-to-string"></span>`fn to_string(&self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

#### Trait Implementations

##### `impl Clone for StyleDisplay`

- <span id="styledisplay-clone"></span>`fn clone(&self) -> StyleDisplay` — [`StyleDisplay`](style/index.md)

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- <span id="styledisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleDisplay`

- <span id="styledisplay-default"></span>`fn default() -> StyleDisplay` — [`StyleDisplay`](style/index.md)

##### `impl Display for StyleDisplay`

- <span id="styledisplay-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for StyleDisplay`

- <span id="styledisplay-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="color-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- <span id="color-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

- <span id="color-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-fg-to"></span>`fn write_fg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-bg-to"></span>`fn write_bg_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="color-render-underline"></span>`fn render_underline(self) -> impl core::fmt::Display + Copy`

- <span id="color-write-underline-to"></span>`fn write_underline_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for Color`

- <span id="color-clone"></span>`fn clone(&self) -> Color` — [`Color`](#color)

##### `impl Copy for Color`

##### `impl Debug for Color`

- <span id="color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Color`

##### `impl Hash for Color`

- <span id="color-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Color`

- <span id="color-cmp"></span>`fn cmp(&self, other: &Color) -> cmp::Ordering` — [`Color`](#color)

##### `impl PartialEq for Color`

- <span id="color-eq"></span>`fn eq(&self, other: &Color) -> bool` — [`Color`](#color)

##### `impl PartialOrd for Color`

- <span id="color-partial-cmp"></span>`fn partial_cmp(&self, other: &Color) -> option::Option<cmp::Ordering>` — [`Color`](#color)

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

- <span id="ansicolor-on"></span>`fn on(self, background: impl Into<Color>) -> crate::Style` — [`Color`](#color), [`Style`](#style)

- <span id="ansicolor-on-default"></span>`const fn on_default(self) -> crate::Style` — [`Style`](#style)

- <span id="ansicolor-render-fg"></span>`fn render_fg(self) -> impl core::fmt::Display + Copy`

- <span id="ansicolor-as-fg-str"></span>`fn as_fg_str(&self) -> &'static str`

- <span id="ansicolor-as-fg-buffer"></span>`fn as_fg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="ansicolor-render-bg"></span>`fn render_bg(self) -> impl core::fmt::Display + Copy`

- <span id="ansicolor-as-bg-str"></span>`fn as_bg_str(&self) -> &'static str`

- <span id="ansicolor-as-bg-buffer"></span>`fn as_bg_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="ansicolor-as-underline-buffer"></span>`fn as_underline_buffer(&self) -> DisplayBuffer` — [`DisplayBuffer`](color/index.md)

- <span id="ansicolor-bright"></span>`fn bright(self, yes: bool) -> Self`

- <span id="ansicolor-is-bright"></span>`fn is_bright(self) -> bool`

#### Trait Implementations

##### `impl Clone for AnsiColor`

- <span id="ansicolor-clone"></span>`fn clone(&self) -> AnsiColor` — [`AnsiColor`](#ansicolor)

##### `impl Copy for AnsiColor`

##### `impl Debug for AnsiColor`

- <span id="ansicolor-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColor`

##### `impl Hash for AnsiColor`

- <span id="ansicolor-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for AnsiColor`

- <span id="ansicolor-cmp"></span>`fn cmp(&self, other: &AnsiColor) -> cmp::Ordering` — [`AnsiColor`](#ansicolor)

##### `impl PartialEq for AnsiColor`

- <span id="ansicolor-eq"></span>`fn eq(&self, other: &AnsiColor) -> bool` — [`AnsiColor`](#ansicolor)

##### `impl PartialOrd for AnsiColor`

- <span id="ansicolor-partial-cmp"></span>`fn partial_cmp(&self, other: &AnsiColor) -> option::Option<cmp::Ordering>` — [`AnsiColor`](#ansicolor)

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

