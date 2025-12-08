# Crate `owo_colors`

|**Quick Links**|[`OwoColorize`](#owocolorize)|[`Style`](#style)|[`StyledList`](#styledlist)|[`github`](https://github.com/owo-colors/owo-colors)|
|-|-|-|-|-|

---

This crate provides [`OwoColorize`](#owocolorize), an extension trait for colorizing a
given type.

## Example

```rust
use owo_colors::OwoColorize;

// Foreground colors
println!("My number is {:#x}!", 10.green());
// Background colors
println!("My number is not {}!", 4.on_red());
```

## Generically color

```rust
use owo_colors::OwoColorize;
use owo_colors::colors::*;

// Generically color
println!("My number might be {}!", 4.fg::<Black>().bg::<Yellow>());
```

## Stylize

```rust
use owo_colors::OwoColorize;

println!("{}", "strikethrough".strikethrough());
```

## Only Style on Supported Terminals

```rust
#[cfg(feature = "supports-color")] {
use owo_colors::{OwoColorize, Stream::Stdout};

println!(
    "{}",
    "colored blue if a supported terminal"
        .if_supports_color(Stdout, |text| text.bright_blue())
);
}
```

Supports `NO_COLOR`/`FORCE_COLOR` environment variables, checks if it's a tty, checks
if it's running in CI (and thus likely supports color), and checks which terminal is being
used. (Note: requires `supports-colors` feature)

## Style Objects

owo-colors also features the ability to create a [`Style`](#style) object and use it to
apply the same set of colors/effects to any number of things to display.

```rust
use owo_colors::{OwoColorize, Style};

let my_style = Style::new()
    .red()
    .on_white()
    .strikethrough();

let text = "red text, white background, struck through";
println!("{}", text.style(my_style));
```

## Contents

- [Modules](#modules)
  - [`colors`](#colors)
  - [`combo`](#combo)
  - [`dyn_colors`](#dyn_colors)
  - [`dyn_styles`](#dyn_styles)
  - [`styled_list`](#styled_list)
  - [`styles`](#styles)
  - [`private`](#private)
  - [`colored`](#colored)
- [Structs](#structs)
  - [`FgColorDisplay`](#fgcolordisplay)
  - [`BgColorDisplay`](#bgcolordisplay)
  - [`FgDynColorDisplay`](#fgdyncolordisplay)
  - [`BgDynColorDisplay`](#bgdyncolordisplay)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`ParseColorError`](#parsecolorerror)
  - [`Styled`](#styled)
  - [`Style`](#style)
  - [`StyleFlags`](#styleflags)
  - [`StylePrefixFormatter`](#styleprefixformatter)
  - [`StyleSuffixFormatter`](#stylesuffixformatter)
- [Enums](#enums)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`DynColors`](#dyncolors)
  - [`Effect`](#effect)
- [Traits](#traits)
  - [`Color`](#color)
  - [`DynColor`](#dyncolor)
  - [`OwoColorize`](#owocolorize)
- [Functions](#functions)
  - [`style`](#style)
- [Constants](#constants)
  - [`DIMMED_SHIFT`](#dimmed_shift)
  - [`ITALIC_SHIFT`](#italic_shift)
  - [`UNDERLINE_SHIFT`](#underline_shift)
  - [`BLINK_SHIFT`](#blink_shift)
  - [`BLINK_FAST_SHIFT`](#blink_fast_shift)
  - [`REVERSED_SHIFT`](#reversed_shift)
  - [`HIDDEN_SHIFT`](#hidden_shift)
  - [`STRIKETHROUGH_SHIFT`](#strikethrough_shift)
- [Macros](#macros)
  - [`style_methods!`](#style_methods)
  - [`color_methods!`](#color_methods)
  - [`color_methods!`](#color_methods)
  - [`style_methods!`](#style_methods)
  - [`style_flags_methods!`](#style_flags_methods)
  - [`impl_fmt!`](#impl_fmt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`colors`](#colors) | mod | Color types for used for being generic over the color |
| [`combo`](#combo) | mod |  |
| [`dyn_colors`](#dyn_colors) | mod |  |
| [`dyn_styles`](#dyn_styles) | mod |  |
| [`styled_list`](#styled_list) | mod |  |
| [`styles`](#styles) | mod | Different display styles (strikethrough, bold, etc.) |
| [`private`](#private) | mod |  |
| [`colored`](#colored) | mod | Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from |
| [`FgColorDisplay`](#fgcolordisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does |
| [`BgColorDisplay`](#bgcolordisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does |
| [`FgDynColorDisplay`](#fgdyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does |
| [`BgDynColorDisplay`](#bgdyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`ParseColorError`](#parsecolorerror) | struct | An error for when the color can not be parsed from a string at runtime |
| [`Styled`](#styled) | struct | A wrapper type which applies a [`Style`] when displaying the inner type |
| [`Style`](#style) | struct | A pre-computed style that can be applied to a struct using [`OwoColorize::style`]. |
| [`StyleFlags`](#styleflags) | struct |  |
| [`StylePrefixFormatter`](#styleprefixformatter) | struct | Formatter for the prefix of a [`Style`]. |
| [`StyleSuffixFormatter`](#stylesuffixformatter) | struct | Formatter for the suffix of a [`Style`]. |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`DynColors`](#dyncolors) | enum | An enum describing runtime-configurable colors |
| [`Effect`](#effect) | enum | A runtime-configurable text effect for use with [`Style`] |
| [`Color`](#color) | trait | A trait for describing a type which can be used with [`FgColorDisplay`] or |
| [`DynColor`](#dyncolor) | trait | A trait describing a runtime-configurable color which can displayed using [`FgDynColorDisplay`] |
| [`OwoColorize`](#owocolorize) | trait | Extension trait for colorizing a type which implements any std formatter |
| [`style`](#style) | fn | Helper to create [`Style`]s more ergonomically |
| [`DIMMED_SHIFT`](#dimmed_shift) | const |  |
| [`ITALIC_SHIFT`](#italic_shift) | const |  |
| [`UNDERLINE_SHIFT`](#underline_shift) | const |  |
| [`BLINK_SHIFT`](#blink_shift) | const |  |
| [`BLINK_FAST_SHIFT`](#blink_fast_shift) | const |  |
| [`REVERSED_SHIFT`](#reversed_shift) | const |  |
| [`HIDDEN_SHIFT`](#hidden_shift) | const |  |
| [`STRIKETHROUGH_SHIFT`](#strikethrough_shift) | const |  |
| [`style_methods!`](#style_methods) | macro |  |
| [`color_methods!`](#color_methods) | macro |  |
| [`color_methods!`](#color_methods) | macro |  |
| [`style_methods!`](#style_methods) | macro |  |
| [`style_flags_methods!`](#style_flags_methods) | macro |  |
| [`impl_fmt!`](#impl_fmt) | macro |  |

## Modules

- [`colors`](colors/index.md) - Color types for used for being generic over the color
- [`combo`](combo/index.md) - 
- [`dyn_colors`](dyn_colors/index.md) - 
- [`dyn_styles`](dyn_styles/index.md) - 
- [`styled_list`](styled_list/index.md) - 
- [`styles`](styles/index.md) - Different display styles (strikethrough, bold, etc.)
- [`private`](private/index.md) - 
- [`colored`](colored/index.md) - Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from

## Structs

### `FgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct FgColorDisplay<'a, C: Color, T: ?Sized>(&'a T, core::marker::PhantomData<C>);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- <span id="fgcolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

- <span id="fgcolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="fgcolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>` — [`FgDynColorDisplay`](#fgdyncolordisplay)

- <span id="fgcolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](#color)

- <span id="fgcolordisplay-fg"></span>`const fn fg<C: Color>(self) -> FgColorDisplay<'a, C, T>` — [`FgColorDisplay`](#fgcolordisplay)

- <span id="fgcolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="fgcolordisplay-black"></span>`const fn black(self) -> FgColorDisplay<'a, colors::Black, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Black`](colors/index.md)

- <span id="fgcolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](colors/index.md)

- <span id="fgcolordisplay-red"></span>`const fn red(self) -> FgColorDisplay<'a, colors::Red, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Red`](colors/index.md)

- <span id="fgcolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](colors/index.md)

- <span id="fgcolordisplay-green"></span>`const fn green(self) -> FgColorDisplay<'a, colors::Green, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Green`](colors/index.md)

- <span id="fgcolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](colors/index.md)

- <span id="fgcolordisplay-yellow"></span>`const fn yellow(self) -> FgColorDisplay<'a, colors::Yellow, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Yellow`](colors/index.md)

- <span id="fgcolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](colors/index.md)

- <span id="fgcolordisplay-blue"></span>`const fn blue(self) -> FgColorDisplay<'a, colors::Blue, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Blue`](colors/index.md)

- <span id="fgcolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](colors/index.md)

- <span id="fgcolordisplay-magenta"></span>`const fn magenta(self) -> FgColorDisplay<'a, colors::Magenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Magenta`](colors/index.md)

- <span id="fgcolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="fgcolordisplay-purple"></span>`const fn purple(self) -> FgColorDisplay<'a, colors::Magenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Magenta`](colors/index.md)

- <span id="fgcolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="fgcolordisplay-cyan"></span>`const fn cyan(self) -> FgColorDisplay<'a, colors::Cyan, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Cyan`](colors/index.md)

- <span id="fgcolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](colors/index.md)

- <span id="fgcolordisplay-white"></span>`const fn white(self) -> FgColorDisplay<'a, colors::White, T>` — [`FgColorDisplay`](#fgcolordisplay), [`White`](colors/index.md)

- <span id="fgcolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](colors/index.md)

- <span id="fgcolordisplay-bright-black"></span>`const fn bright_black(self) -> FgColorDisplay<'a, colors::BrightBlack, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightBlack`](colors/index.md)

- <span id="fgcolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](colors/index.md)

- <span id="fgcolordisplay-bright-red"></span>`const fn bright_red(self) -> FgColorDisplay<'a, colors::BrightRed, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightRed`](colors/index.md)

- <span id="fgcolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](colors/index.md)

- <span id="fgcolordisplay-bright-green"></span>`const fn bright_green(self) -> FgColorDisplay<'a, colors::BrightGreen, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightGreen`](colors/index.md)

- <span id="fgcolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](colors/index.md)

- <span id="fgcolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> FgColorDisplay<'a, colors::BrightYellow, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightYellow`](colors/index.md)

- <span id="fgcolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](colors/index.md)

- <span id="fgcolordisplay-bright-blue"></span>`const fn bright_blue(self) -> FgColorDisplay<'a, colors::BrightBlue, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightBlue`](colors/index.md)

- <span id="fgcolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](colors/index.md)

- <span id="fgcolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> FgColorDisplay<'a, colors::BrightMagenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="fgcolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="fgcolordisplay-bright-purple"></span>`const fn bright_purple(self) -> FgColorDisplay<'a, colors::BrightMagenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="fgcolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="fgcolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> FgColorDisplay<'a, colors::BrightCyan, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightCyan`](colors/index.md)

- <span id="fgcolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](colors/index.md)

- <span id="fgcolordisplay-bright-white"></span>`const fn bright_white(self) -> FgColorDisplay<'a, colors::BrightWhite, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightWhite`](colors/index.md)

- <span id="fgcolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](colors/index.md)

#### Trait Implementations

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Binary> Binary for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Debug> Debug for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Display> Display for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::LowerExp> LowerExp for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::LowerHex> LowerHex for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Octal> Octal for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for FgColorDisplay<'a, C, T>`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Pointer> Pointer for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::UpperExp> UpperExp for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::UpperHex> UpperHex for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct BgColorDisplay<'a, C: Color, T: ?Sized>(&'a T, core::marker::PhantomData<C>);
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- <span id="bgcolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

- <span id="bgcolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="bgcolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](#color)

- <span id="bgcolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>` — [`BgDynColorDisplay`](#bgdyncolordisplay)

- <span id="bgcolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="bgcolordisplay-bg"></span>`const fn bg<C: Color>(self) -> BgColorDisplay<'a, C, T>` — [`BgColorDisplay`](#bgcolordisplay)

- <span id="bgcolordisplay-on-black"></span>`const fn on_black(self) -> BgColorDisplay<'a, colors::Black, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Black`](colors/index.md)

- <span id="bgcolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](colors/index.md)

- <span id="bgcolordisplay-on-red"></span>`const fn on_red(self) -> BgColorDisplay<'a, colors::Red, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Red`](colors/index.md)

- <span id="bgcolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](colors/index.md)

- <span id="bgcolordisplay-on-green"></span>`const fn on_green(self) -> BgColorDisplay<'a, colors::Green, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Green`](colors/index.md)

- <span id="bgcolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](colors/index.md)

- <span id="bgcolordisplay-on-yellow"></span>`const fn on_yellow(self) -> BgColorDisplay<'a, colors::Yellow, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Yellow`](colors/index.md)

- <span id="bgcolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](colors/index.md)

- <span id="bgcolordisplay-on-blue"></span>`const fn on_blue(self) -> BgColorDisplay<'a, colors::Blue, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Blue`](colors/index.md)

- <span id="bgcolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](colors/index.md)

- <span id="bgcolordisplay-on-magenta"></span>`const fn on_magenta(self) -> BgColorDisplay<'a, colors::Magenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Magenta`](colors/index.md)

- <span id="bgcolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="bgcolordisplay-on-purple"></span>`const fn on_purple(self) -> BgColorDisplay<'a, colors::Magenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Magenta`](colors/index.md)

- <span id="bgcolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="bgcolordisplay-on-cyan"></span>`const fn on_cyan(self) -> BgColorDisplay<'a, colors::Cyan, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Cyan`](colors/index.md)

- <span id="bgcolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](colors/index.md)

- <span id="bgcolordisplay-on-white"></span>`const fn on_white(self) -> BgColorDisplay<'a, colors::White, T>` — [`BgColorDisplay`](#bgcolordisplay), [`White`](colors/index.md)

- <span id="bgcolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](colors/index.md)

- <span id="bgcolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> BgColorDisplay<'a, colors::BrightBlack, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightBlack`](colors/index.md)

- <span id="bgcolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](colors/index.md)

- <span id="bgcolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> BgColorDisplay<'a, colors::BrightRed, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightRed`](colors/index.md)

- <span id="bgcolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](colors/index.md)

- <span id="bgcolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> BgColorDisplay<'a, colors::BrightGreen, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightGreen`](colors/index.md)

- <span id="bgcolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](colors/index.md)

- <span id="bgcolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> BgColorDisplay<'a, colors::BrightYellow, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightYellow`](colors/index.md)

- <span id="bgcolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](colors/index.md)

- <span id="bgcolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> BgColorDisplay<'a, colors::BrightBlue, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightBlue`](colors/index.md)

- <span id="bgcolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](colors/index.md)

- <span id="bgcolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> BgColorDisplay<'a, colors::BrightMagenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="bgcolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="bgcolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> BgColorDisplay<'a, colors::BrightMagenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="bgcolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="bgcolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> BgColorDisplay<'a, colors::BrightCyan, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightCyan`](colors/index.md)

- <span id="bgcolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](colors/index.md)

- <span id="bgcolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> BgColorDisplay<'a, colors::BrightWhite, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightWhite`](colors/index.md)

- <span id="bgcolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](colors/index.md)

#### Trait Implementations

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Binary> Binary for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Debug> Debug for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Display> Display for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::LowerExp> LowerExp for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::LowerHex> LowerHex for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Octal> Octal for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for BgColorDisplay<'a, C, T>`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::Pointer> Pointer for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::UpperExp> UpperExp for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::Color, T: ?Sized + fmt::UpperHex> UpperHex for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>(&'a T, Color);
```

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- <span id="cratefgdyncolordisplay-new"></span>`const fn new(thing: &'a T, color: Fg) -> Self`

- <span id="cratefgdyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="cratefgdyncolordisplay-on-color"></span>`const fn on_color<Bg: DynColor>(self, bg: Bg) -> ComboDynColorDisplay<'a, Fg, Bg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

- <span id="cratefgdyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>` — [`FgDynColorDisplay`](#fgdyncolordisplay)

#### Trait Implementations

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Binary> Binary for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Debug> Debug for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Display> Display for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerExp> LowerExp for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerHex> LowerHex for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Octal> Octal for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for FgDynColorDisplay<'a, Color, T>`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Pointer> Pointer for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperExp> UpperExp for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperHex> UpperHex for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>(&'a T, Color);
```

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- <span id="cratebgdyncolordisplay-new"></span>`const fn new(thing: &'a T, color: Bg) -> Self`

- <span id="cratebgdyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="cratebgdyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>` — [`BgDynColorDisplay`](#bgdyncolordisplay)

- <span id="cratebgdyncolordisplay-color"></span>`const fn color<Fg: DynColor>(self, fg: Fg) -> ComboDynColorDisplay<'a, Fg, Bg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

#### Trait Implementations

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Binary> Binary for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Debug> Debug for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Display> Display for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerExp> LowerExp for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerHex> LowerHex for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Octal> Octal for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for BgDynColorDisplay<'a, Color, T>`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::Pointer> Pointer for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperExp> UpperExp for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperHex> UpperHex for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for Rgb`

- <span id="rgb-clone"></span>`fn clone(&self) -> Rgb` — [`Rgb`](#rgb)

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- <span id="rgb-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for Rgb`

- <span id="rgb-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Rgb`

##### `impl<D> OwoColorize for Rgb`

##### `impl PartialEq for Rgb`

- <span id="rgb-eq"></span>`fn eq(&self, other: &Rgb) -> bool` — [`Rgb`](#rgb)

##### `impl StructuralPartialEq for Rgb`

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>(&'a T, core::marker::PhantomData<(Fg, Bg)>);
```

A wrapper type which applies both a foreground and background color

#### Implementations

- <span id="combocolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

- <span id="combocolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="combocolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](#color)

- <span id="combocolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](#color)

- <span id="combocolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="combocolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](#combocolordisplay)

- <span id="combocolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](colors/index.md)

- <span id="combocolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](colors/index.md)

- <span id="combocolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](colors/index.md)

- <span id="combocolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](colors/index.md)

- <span id="combocolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](colors/index.md)

- <span id="combocolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](colors/index.md)

- <span id="combocolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](colors/index.md)

- <span id="combocolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](colors/index.md)

- <span id="combocolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](colors/index.md)

- <span id="combocolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](colors/index.md)

- <span id="combocolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="combocolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="combocolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="combocolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](colors/index.md)

- <span id="combocolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](colors/index.md)

- <span id="combocolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](colors/index.md)

- <span id="combocolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](colors/index.md)

- <span id="combocolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](colors/index.md)

- <span id="combocolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](colors/index.md)

- <span id="combocolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](colors/index.md)

- <span id="combocolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](colors/index.md)

- <span id="combocolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](colors/index.md)

- <span id="combocolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](colors/index.md)

- <span id="combocolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](colors/index.md)

- <span id="combocolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](colors/index.md)

- <span id="combocolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](colors/index.md)

- <span id="combocolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](colors/index.md)

- <span id="combocolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](colors/index.md)

- <span id="combocolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="combocolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="combocolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="combocolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](colors/index.md)

- <span id="combocolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](colors/index.md)

- <span id="combocolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](colors/index.md)

- <span id="combocolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](colors/index.md)

- <span id="combocolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](colors/index.md)

#### Trait Implementations

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Binary> Binary for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Debug> Debug for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Display> Display for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerExp> LowerExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerHex> LowerHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Octal> Octal for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Pointer> Pointer for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperExp> UpperExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperHex> UpperHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>`

```rust
struct ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>(&'a T, Fg, Bg);
```

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](#combocolordisplay) instead.

#### Implementations

- <span id="combodyncolordisplay-new"></span>`const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`

- <span id="combodyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

- <span id="combodyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

- <span id="combodyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

#### Trait Implementations

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Binary> Binary for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Debug> Debug for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Display> Display for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerExp> LowerExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerHex> LowerHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Octal> Octal for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ComboDynColorDisplay<'a, Fg, Bg, T>`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Pointer> Pointer for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperExp> UpperExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperHex> UpperHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StyledList<T, U>`

```rust
struct StyledList<T, U>(T, core::marker::PhantomData<fn(U)>)
where
    T: AsRef<[U]>,
    U: IsStyled;
```

A collection of [`Styled`](#styled) items that are displayed in such a way as to minimize the amount of characters
that are written when displayed.

```rust
use owo_colors::{Style, Styled, StyledList};

let styled_items = [
    Style::new().red().style("Hello "),
    Style::new().green().style("World"),
 ];

// 29 characters
let normal_length = styled_items.iter().map(|item| format!("{}", item).len()).sum::<usize>();
// 25 characters
let styled_length = format!("{}", StyledList::from(styled_items)).len();

assert!(styled_length < normal_length);
```

#### Trait Implementations

##### `impl<T, U> Display for StyledList<T, U>`

- <span id="styledlist-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StyledList<T, U>`

### `ParseColorError`

```rust
struct ParseColorError;
```

An error for when the color can not be parsed from a string at runtime

#### Trait Implementations

##### `impl Debug for ParseColorError`

- <span id="parsecolorerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ParseColorError`

### `Styled<T>`

```rust
struct Styled<T> {
    target: T,
    pub style: Style,
}
```

A wrapper type which applies a [`Style`](#style) when displaying the inner type

#### Fields

- **`target`**: `T`

  The target value to be styled

- **`style`**: `Style`

  The style to apply to target

#### Implementations

- <span id="styled-inner"></span>`const fn inner(&self) -> &T`

- <span id="styled-inner-mut"></span>`const fn inner_mut(&mut self) -> &mut T`

#### Trait Implementations

##### `impl<T: fmt::Binary> Binary for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Debug> Debug for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Display for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Display> IsStyled for crate::Styled<T>`

- <span id="cratestyled-inner"></span>`type Inner = T`

- <span id="cratestyled-style"></span>`fn style(&self) -> &Style` — [`Style`](#style)

- <span id="cratestyled-inner"></span>`fn inner(&self) -> &T`

##### `impl<T: fmt::LowerExp> LowerExp for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::LowerHex> LowerHex for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Octal> Octal for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Styled<T>`

##### `impl<T: fmt::Pointer> Pointer for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::UpperExp> UpperExp for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::UpperHex> UpperHex for Styled<T>`

- <span id="styled-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Style`

```rust
struct Style {
    fg: Option<crate::DynColors>,
    bg: Option<crate::DynColors>,
    bold: bool,
    style_flags: StyleFlags,
}
```

A pre-computed style that can be applied to a struct using `OwoColorize::style`.

Its interface mimics that of [`OwoColorize`](#owocolorize), but instead of chaining methods on your
object, you instead chain them on the `Style` object before applying it.

```rust
use owo_colors::{OwoColorize, Style};

let my_style = Style::new()
    .red()
    .on_white()
    .strikethrough();

println!("{}", "red text, white background, struck through".style(my_style));
```

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

- <span id="style-style"></span>`const fn style<T>(&self, target: T) -> Styled<T>` — [`Styled`](#styled)

- <span id="style-fg"></span>`const fn fg<C: Color>(self) -> Self`

- <span id="style-bg"></span>`const fn bg<C: Color>(self) -> Self`

- <span id="style-remove-fg"></span>`const fn remove_fg(self) -> Self`

- <span id="style-remove-bg"></span>`const fn remove_bg(self) -> Self`

- <span id="style-black"></span>`const fn black(self) -> Self`

- <span id="style-on-black"></span>`const fn on_black(self) -> Self`

- <span id="style-red"></span>`const fn red(self) -> Self`

- <span id="style-on-red"></span>`const fn on_red(self) -> Self`

- <span id="style-green"></span>`const fn green(self) -> Self`

- <span id="style-on-green"></span>`const fn on_green(self) -> Self`

- <span id="style-yellow"></span>`const fn yellow(self) -> Self`

- <span id="style-on-yellow"></span>`const fn on_yellow(self) -> Self`

- <span id="style-blue"></span>`const fn blue(self) -> Self`

- <span id="style-on-blue"></span>`const fn on_blue(self) -> Self`

- <span id="style-magenta"></span>`const fn magenta(self) -> Self`

- <span id="style-on-magenta"></span>`const fn on_magenta(self) -> Self`

- <span id="style-purple"></span>`const fn purple(self) -> Self`

- <span id="style-on-purple"></span>`const fn on_purple(self) -> Self`

- <span id="style-cyan"></span>`const fn cyan(self) -> Self`

- <span id="style-on-cyan"></span>`const fn on_cyan(self) -> Self`

- <span id="style-white"></span>`const fn white(self) -> Self`

- <span id="style-on-white"></span>`const fn on_white(self) -> Self`

- <span id="style-default-color"></span>`const fn default_color(self) -> Self`

- <span id="style-on-default-color"></span>`const fn on_default_color(self) -> Self`

- <span id="style-bright-black"></span>`const fn bright_black(self) -> Self`

- <span id="style-on-bright-black"></span>`const fn on_bright_black(self) -> Self`

- <span id="style-bright-red"></span>`const fn bright_red(self) -> Self`

- <span id="style-on-bright-red"></span>`const fn on_bright_red(self) -> Self`

- <span id="style-bright-green"></span>`const fn bright_green(self) -> Self`

- <span id="style-on-bright-green"></span>`const fn on_bright_green(self) -> Self`

- <span id="style-bright-yellow"></span>`const fn bright_yellow(self) -> Self`

- <span id="style-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> Self`

- <span id="style-bright-blue"></span>`const fn bright_blue(self) -> Self`

- <span id="style-on-bright-blue"></span>`const fn on_bright_blue(self) -> Self`

- <span id="style-bright-magenta"></span>`const fn bright_magenta(self) -> Self`

- <span id="style-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> Self`

- <span id="style-bright-purple"></span>`const fn bright_purple(self) -> Self`

- <span id="style-on-bright-purple"></span>`const fn on_bright_purple(self) -> Self`

- <span id="style-bright-cyan"></span>`const fn bright_cyan(self) -> Self`

- <span id="style-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> Self`

- <span id="style-bright-white"></span>`const fn bright_white(self) -> Self`

- <span id="style-on-bright-white"></span>`const fn on_bright_white(self) -> Self`

- <span id="style-bold"></span>`const fn bold(self) -> Self`

- <span id="style-dimmed"></span>`const fn dimmed(self) -> Self`

- <span id="style-italic"></span>`const fn italic(self) -> Self`

- <span id="style-underline"></span>`const fn underline(self) -> Self`

- <span id="style-blink"></span>`const fn blink(self) -> Self`

- <span id="style-blink-fast"></span>`const fn blink_fast(self) -> Self`

- <span id="style-reversed"></span>`const fn reversed(self) -> Self`

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

- <span id="style-set-effect"></span>`const fn set_effect(self, effect: Effect, to: bool) -> Self` — [`Effect`](#effect)

- <span id="style-set-effects"></span>`const fn set_effects(self, effects: &[Effect], to: bool) -> Self` — [`Effect`](#effect)

- <span id="style-effect"></span>`const fn effect(self, effect: Effect) -> Self` — [`Effect`](#effect)

- <span id="style-remove-effect"></span>`const fn remove_effect(self, effect: Effect) -> Self` — [`Effect`](#effect)

- <span id="style-effects"></span>`const fn effects(self, effects: &[Effect]) -> Self` — [`Effect`](#effect)

- <span id="style-remove-effects"></span>`const fn remove_effects(self, effects: &[Effect]) -> Self` — [`Effect`](#effect)

- <span id="style-remove-all-effects"></span>`const fn remove_all_effects(self) -> Self`

- <span id="style-color"></span>`fn color<Color: DynColor>(self, color: Color) -> Self`

- <span id="style-on-color"></span>`fn on_color<Color: DynColor>(self, color: Color) -> Self`

- <span id="style-fg-rgb"></span>`const fn fg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

- <span id="style-bg-rgb"></span>`const fn bg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

- <span id="style-truecolor"></span>`const fn truecolor(self, r: u8, g: u8, b: u8) -> Self`

- <span id="style-on-truecolor"></span>`const fn on_truecolor(self, r: u8, g: u8, b: u8) -> Self`

- <span id="style-is-plain"></span>`const fn is_plain(&self) -> bool`

- <span id="style-prefix-formatter"></span>`const fn prefix_formatter(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](#styleprefixformatter)

- <span id="style-suffix-formatter"></span>`const fn suffix_formatter(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](#stylesuffixformatter)

- <span id="style-fmt-prefix"></span>`fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="style-fmt-suffix"></span>`fn fmt_suffix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl<D> OwoColorize for Style`

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

### `StyleFlags`

```rust
struct StyleFlags(u8);
```

#### Implementations

- <span id="styleflags-is-plain"></span>`const fn is_plain(&self) -> bool`

#### Trait Implementations

##### `impl Clone for StyleFlags`

- <span id="styleflags-clone"></span>`fn clone(&self) -> StyleFlags` — [`StyleFlags`](dyn_styles/index.md)

##### `impl Copy for StyleFlags`

##### `impl Debug for StyleFlags`

- <span id="styleflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleFlags`

- <span id="styleflags-default"></span>`fn default() -> Self`

##### `impl<D> OwoColorize for StyleFlags`

##### `impl PartialEq for StyleFlags`

- <span id="styleflags-eq"></span>`fn eq(&self, other: &StyleFlags) -> bool` — [`StyleFlags`](dyn_styles/index.md)

##### `impl StructuralPartialEq for StyleFlags`

### `StylePrefixFormatter`

```rust
struct StylePrefixFormatter(Style);
```

Formatter for the prefix of a [`Style`](#style).

This is used to get the ANSI escape codes for the style without
the suffix, which is useful for formatting the prefix separately.

#### Trait Implementations

##### `impl Clone for StylePrefixFormatter`

- <span id="styleprefixformatter-clone"></span>`fn clone(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](#styleprefixformatter)

##### `impl Copy for StylePrefixFormatter`

##### `impl Debug for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StylePrefixFormatter`

##### `impl PartialEq for StylePrefixFormatter`

- <span id="styleprefixformatter-eq"></span>`fn eq(&self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](#styleprefixformatter)

##### `impl StructuralPartialEq for StylePrefixFormatter`

### `StyleSuffixFormatter`

```rust
struct StyleSuffixFormatter(Style);
```

Formatter for the suffix of a [`Style`](#style).

This is used to get the ANSI escape codes for the style without
the prefix, which is useful for formatting the suffix separately.

#### Trait Implementations

##### `impl Clone for StyleSuffixFormatter`

- <span id="stylesuffixformatter-clone"></span>`fn clone(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](#stylesuffixformatter)

##### `impl Copy for StyleSuffixFormatter`

##### `impl Debug for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StyleSuffixFormatter`

##### `impl PartialEq for StyleSuffixFormatter`

- <span id="stylesuffixformatter-eq"></span>`fn eq(&self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](#stylesuffixformatter)

##### `impl StructuralPartialEq for StyleSuffixFormatter`

## Enums

### `AnsiColors`

```rust
enum AnsiColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
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

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for AnsiColors`

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](#ansicolors)

##### `impl Copy for AnsiColors`

##### `impl Debug for AnsiColors`

- <span id="ansicolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for AnsiColors`

- <span id="ansicolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColors`

##### `impl<D> OwoColorize for AnsiColors`

##### `impl PartialEq for AnsiColors`

- <span id="ansicolors-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](#ansicolors)

##### `impl StructuralPartialEq for AnsiColors`

### `CssColors`

```rust
enum CssColors {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGrey,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    FireBrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Grey,
    Green,
    GreenYellow,
    HoneyDew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGrey,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    RebeccaPurple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    SlateGrey,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}
```

Available CSS colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for CssColors`

- <span id="csscolors-clone"></span>`fn clone(&self) -> CssColors` — [`CssColors`](#csscolors)

##### `impl Copy for CssColors`

##### `impl Debug for CssColors`

- <span id="csscolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for CssColors`

- <span id="csscolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CssColors`

##### `impl<D> OwoColorize for CssColors`

##### `impl PartialEq for CssColors`

- <span id="csscolors-eq"></span>`fn eq(&self, other: &CssColors) -> bool` — [`CssColors`](#csscolors)

##### `impl StructuralPartialEq for CssColors`

### `XtermColors`

```rust
enum XtermColors {
    UserBlack,
    UserRed,
    UserGreen,
    UserYellow,
    UserBlue,
    UserMagenta,
    UserCyan,
    UserWhite,
    UserBrightBlack,
    UserBrightRed,
    UserBrightGreen,
    UserBrightYellow,
    UserBrightBlue,
    UserBrightMagenta,
    UserBrightCyan,
    UserBrightWhite,
    Black,
    StratosBlue,
    NavyBlue,
    MidnightBlue,
    DarkBlue,
    Blue,
    CamaroneGreen,
    BlueStone,
    OrientBlue,
    EndeavourBlue,
    ScienceBlue,
    BlueRibbon,
    JapaneseLaurel,
    DeepSeaGreen,
    Teal,
    DeepCerulean,
    LochmaraBlue,
    AzureRadiance,
    LightJapaneseLaurel,
    Jade,
    PersianGreen,
    BondiBlue,
    Cerulean,
    LightAzureRadiance,
    DarkGreen,
    Malachite,
    CaribbeanGreen,
    LightCaribbeanGreen,
    RobinEggBlue,
    Aqua,
    Green,
    DarkSpringGreen,
    SpringGreen,
    LightSpringGreen,
    BrightTurquoise,
    Cyan,
    Rosewood,
    PompadourMagenta,
    PigmentIndigo,
    DarkPurple,
    ElectricIndigo,
    ElectricPurple,
    VerdunGreen,
    ScorpionOlive,
    Lilac,
    ScampiIndigo,
    Indigo,
    DarkCornflowerBlue,
    DarkLimeade,
    GladeGreen,
    JuniperGreen,
    HippieBlue,
    HavelockBlue,
    CornflowerBlue,
    Limeade,
    FernGreen,
    SilverTree,
    Tradewind,
    ShakespeareBlue,
    DarkMalibuBlue,
    DarkBrightGreen,
    DarkPastelGreen,
    PastelGreen,
    DownyTeal,
    Viking,
    MalibuBlue,
    BrightGreen,
    DarkScreaminGreen,
    ScreaminGreen,
    DarkAquamarine,
    Aquamarine,
    LightAquamarine,
    Maroon,
    DarkFreshEggplant,
    LightFreshEggplant,
    Purple,
    ElectricViolet,
    LightElectricViolet,
    Brown,
    CopperRose,
    StrikemasterPurple,
    DelugePurple,
    DarkMediumPurple,
    DarkHeliotropePurple,
    Olive,
    ClayCreekOlive,
    DarkGray,
    WildBlueYonder,
    ChetwodeBlue,
    SlateBlue,
    LightLimeade,
    ChelseaCucumber,
    BayLeaf,
    GulfStream,
    PoloBlue,
    LightMalibuBlue,
    Pistachio,
    LightPastelGreen,
    DarkFeijoaGreen,
    VistaBlue,
    Bermuda,
    DarkAnakiwaBlue,
    ChartreuseGreen,
    LightScreaminGreen,
    DarkMintGreen,
    MintGreen,
    LighterAquamarine,
    AnakiwaBlue,
    BrightRed,
    DarkFlirt,
    Flirt,
    LightFlirt,
    DarkViolet,
    BrightElectricViolet,
    RoseofSharonOrange,
    MatrixPink,
    TapestryPink,
    FuchsiaPink,
    MediumPurple,
    Heliotrope,
    PirateGold,
    MuesliOrange,
    PharlapPink,
    Bouquet,
    Lavender,
    LightHeliotrope,
    BuddhaGold,
    OliveGreen,
    HillaryOlive,
    SilverChalice,
    WistfulLilac,
    MelroseLilac,
    RioGrandeGreen,
    ConiferGreen,
    Feijoa,
    PixieGreen,
    JungleMist,
    LightAnakiwaBlue,
    Lime,
    GreenYellow,
    LightMintGreen,
    Celadon,
    AeroBlue,
    FrenchPassLightBlue,
    GuardsmanRed,
    RazzmatazzCerise,
    MediumVioletRed,
    HollywoodCerise,
    DarkPurplePizzazz,
    BrighterElectricViolet,
    TennOrange,
    RomanOrange,
    CranberryPink,
    HopbushPink,
    Orchid,
    LighterHeliotrope,
    MangoTango,
    Copperfield,
    SeaPink,
    CanCanPink,
    LightOrchid,
    BrightHeliotrope,
    DarkCorn,
    DarkTachaOrange,
    TanBeige,
    ClamShell,
    ThistlePink,
    Mauve,
    Corn,
    TachaOrange,
    DecoOrange,
    PaleGoldenrod,
    AltoBeige,
    FogPink,
    ChartreuseYellow,
    Canary,
    Honeysuckle,
    ReefPaleYellow,
    SnowyMint,
    OysterBay,
    Red,
    DarkRose,
    Rose,
    LightHollywoodCerise,
    PurplePizzazz,
    Fuchsia,
    BlazeOrange,
    BittersweetOrange,
    WildWatermelon,
    DarkHotPink,
    HotPink,
    PinkFlamingo,
    FlushOrange,
    Salmon,
    VividTangerine,
    PinkSalmon,
    DarkLavenderRose,
    BlushPink,
    YellowSea,
    TexasRose,
    Tacao,
    Sundown,
    CottonCandy,
    LavenderRose,
    Gold,
    Dandelion,
    GrandisCaramel,
    Caramel,
    CosmosSalmon,
    PinkLace,
    Yellow,
    LaserLemon,
    DollyYellow,
    PortafinoYellow,
    Cumulus,
    White,
    DarkCodGray,
    CodGray,
    LightCodGray,
    DarkMineShaft,
    MineShaft,
    LightMineShaft,
    DarkTundora,
    Tundora,
    ScorpionGray,
    DarkDoveGray,
    DoveGray,
    Boulder,
    Gray,
    LightGray,
    DustyGray,
    NobelGray,
    DarkSilverChalice,
    LightSilverChalice,
    DarkSilver,
    Silver,
    DarkAlto,
    Alto,
    Mercury,
    GalleryGray,
}
```

Available Xterm colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for XtermColors`

- <span id="xtermcolors-clone"></span>`fn clone(&self) -> XtermColors` — [`XtermColors`](#xtermcolors)

##### `impl Copy for XtermColors`

##### `impl Debug for XtermColors`

- <span id="xtermcolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for XtermColors`

- <span id="xtermcolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for XtermColors`

##### `impl<D> OwoColorize for XtermColors`

##### `impl PartialEq for XtermColors`

- <span id="xtermcolors-eq"></span>`fn eq(&self, other: &XtermColors) -> bool` — [`XtermColors`](#xtermcolors)

##### `impl StructuralPartialEq for XtermColors`

### `DynColors`

```rust
enum DynColors {
    Ansi(crate::AnsiColors),
    Css(crate::CssColors),
    Xterm(crate::XtermColors),
    Rgb(u8, u8, u8),
}
```

An enum describing runtime-configurable colors

This can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay) or [`BgDynColorDisplay`](BgDynColorDisplay),
allowing for multiple types of colors to be used at runtime.

#### Trait Implementations

##### `impl Clone for DynColors`

- <span id="dyncolors-clone"></span>`fn clone(&self) -> DynColors` — [`DynColors`](#dyncolors)

##### `impl Copy for DynColors`

##### `impl Debug for DynColors`

- <span id="dyncolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for DynColors`

- <span id="dyncolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DynColors`

##### `impl FromStr for DynColors`

- <span id="dyncolors-err"></span>`type Err = ParseColorError`

- <span id="dyncolors-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<D> OwoColorize for DynColors`

##### `impl PartialEq for DynColors`

- <span id="dyncolors-eq"></span>`fn eq(&self, other: &DynColors) -> bool` — [`DynColors`](#dyncolors)

##### `impl StructuralPartialEq for DynColors`

### `Effect`

```rust
enum Effect {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    BlinkFast,
    Reversed,
    Hidden,
    Strikethrough,
}
```

A runtime-configurable text effect for use with [`Style`](#style)

#### Trait Implementations

##### `impl Clone for Effect`

- <span id="effect-clone"></span>`fn clone(&self) -> Effect` — [`Effect`](#effect)

##### `impl Copy for Effect`

##### `impl Debug for Effect`

- <span id="effect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Effect`

## Traits

### `Color`

```rust
trait Color: private::Sealed { ... }
```

A trait for describing a type which can be used with [`FgColorDisplay`](#fgcolordisplay) or
[`BgColorDisplay`](#bgcolordisplay)

#### Required Methods

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

### `DynColor`

```rust
trait DynColor: private::Sealed { ... }
```

A trait describing a runtime-configurable color which can displayed using [`FgDynColorDisplay`](#fgdyncolordisplay)
or [`BgDynColorDisplay`](#bgdyncolordisplay). If your color will be known at compile time it
is recommended you avoid this.

#### Required Methods

- `fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a ANSI code to a formatter to set the foreground to this color

- `fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a ANSI code to a formatter to set the background to this color

- `fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a raw ANSI code to a formatter to set the foreground to this color,

- `fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a raw ANSI code to a formatter to set the background to this color,

### `OwoColorize`

```rust
trait OwoColorize: Sized { ... }
```

Extension trait for colorizing a type which implements any std formatter
([`Display`](core::fmt::Display), [`Debug`](core::fmt::Debug), [`UpperHex`](core::fmt::UpperHex),
etc.)

## Example

```rust
use owo_colors::OwoColorize;

println!("My number is {:#x}!", 10.green());
println!("My number is not {}!", 4.on_red());
```

## How to decide which method to use

**Do you have a specific color you want to use?**

Use the specific color's method, such as [`blue`](OwoColorize::blue) or
[`on_green`](OwoColorize::on_green).


**Do you want your colors configurable via generics?**

Use [`fg`](OwoColorize::fg) and [`bg`](OwoColorize::bg) to make it compile-time configurable.


**Do you need to pick a color at runtime?**

Use the [`color`](OwoColorize::color), [`on_color`](OwoColorize::on_color),
[`truecolor`](OwoColorize::truecolor) or [`on_truecolor`](OwoColorize::on_truecolor).

**Do you need some other text modifier?**

* [`bold`](OwoColorize::bold)
* [`dimmed`](OwoColorize::dimmed)
* [`italic`](OwoColorize::italic)
* [`underline`](OwoColorize::underline)
* [`blink`](OwoColorize::blink)
* [`blink_fast`](OwoColorize::blink_fast)
* [`reversed`](OwoColorize::reversed)
* [`hidden`](OwoColorize::hidden)
* [`strikethrough`](OwoColorize::strikethrough)

**Do you want it to only display colors if it's a terminal?**

1. Enable the `supports-colors` feature
2. Colorize inside [`if_supports_color`](OwoColorize::if_supports_color)

**Do you need to store a set of colors/effects to apply to multiple things?**

Use [`style`](OwoColorize::style) to apply a [`Style`](#style)


#### Required Methods

- `fn fg<C: Color>(&self) -> FgColorDisplay<'_, C, Self>`

  Set the foreground color generically

- `fn bg<C: Color>(&self) -> BgColorDisplay<'_, C, Self>`

  Set the background color generically.

- `fn black(&self) -> FgColorDisplay<'_, colors::Black, Self>`

  Change the foreground color to black

- `fn on_black(&self) -> BgColorDisplay<'_, colors::Black, Self>`

  Change the background color to black

- `fn red(&self) -> FgColorDisplay<'_, colors::Red, Self>`

  Change the foreground color to red

- `fn on_red(&self) -> BgColorDisplay<'_, colors::Red, Self>`

  Change the background color to red

- `fn green(&self) -> FgColorDisplay<'_, colors::Green, Self>`

  Change the foreground color to green

- `fn on_green(&self) -> BgColorDisplay<'_, colors::Green, Self>`

  Change the background color to green

- `fn yellow(&self) -> FgColorDisplay<'_, colors::Yellow, Self>`

  Change the foreground color to yellow

- `fn on_yellow(&self) -> BgColorDisplay<'_, colors::Yellow, Self>`

  Change the background color to yellow

- `fn blue(&self) -> FgColorDisplay<'_, colors::Blue, Self>`

  Change the foreground color to blue

- `fn on_blue(&self) -> BgColorDisplay<'_, colors::Blue, Self>`

  Change the background color to blue

- `fn magenta(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to magenta

- `fn on_magenta(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to magenta

- `fn purple(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to purple

- `fn on_purple(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to purple

- `fn cyan(&self) -> FgColorDisplay<'_, colors::Cyan, Self>`

  Change the foreground color to cyan

- `fn on_cyan(&self) -> BgColorDisplay<'_, colors::Cyan, Self>`

  Change the background color to cyan

- `fn white(&self) -> FgColorDisplay<'_, colors::White, Self>`

  Change the foreground color to white

- `fn on_white(&self) -> BgColorDisplay<'_, colors::White, Self>`

  Change the background color to white

- `fn default_color(&self) -> FgColorDisplay<'_, colors::Default, Self>`

  Change the foreground color to the terminal default

- `fn on_default_color(&self) -> BgColorDisplay<'_, colors::Default, Self>`

  Change the background color to the terminal default

- `fn bright_black(&self) -> FgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the foreground color to bright black

- `fn on_bright_black(&self) -> BgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the background color to bright black

- `fn bright_red(&self) -> FgColorDisplay<'_, colors::BrightRed, Self>`

  Change the foreground color to bright red

- `fn on_bright_red(&self) -> BgColorDisplay<'_, colors::BrightRed, Self>`

  Change the background color to bright red

- `fn bright_green(&self) -> FgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the foreground color to bright green

- `fn on_bright_green(&self) -> BgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the background color to bright green

- `fn bright_yellow(&self) -> FgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the foreground color to bright yellow

- `fn on_bright_yellow(&self) -> BgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the background color to bright yellow

- `fn bright_blue(&self) -> FgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the foreground color to bright blue

- `fn on_bright_blue(&self) -> BgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the background color to bright blue

- `fn bright_magenta(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright magenta

- `fn on_bright_magenta(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright magenta

- `fn bright_purple(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright purple

- `fn on_bright_purple(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright purple

- `fn bright_cyan(&self) -> FgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the foreground color to bright cyan

- `fn on_bright_cyan(&self) -> BgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the background color to bright cyan

- `fn bright_white(&self) -> FgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the foreground color to bright white

- `fn on_bright_white(&self) -> BgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the background color to bright white

- `fn bold(&self) -> styles::BoldDisplay<'_, Self>`

  Make the text bold

- `fn dimmed(&self) -> styles::DimDisplay<'_, Self>`

  Make the text dim

- `fn italic(&self) -> styles::ItalicDisplay<'_, Self>`

  Make the text italicized

- `fn underline(&self) -> styles::UnderlineDisplay<'_, Self>`

  Make the text underlined

- `fn blink(&self) -> styles::BlinkDisplay<'_, Self>`

  Make the text blink

- `fn blink_fast(&self) -> styles::BlinkFastDisplay<'_, Self>`

  Make the text blink (but fast!)

- `fn reversed(&self) -> styles::ReversedDisplay<'_, Self>`

  Swap the foreground and background colors

- `fn hidden(&self) -> styles::HiddenDisplay<'_, Self>`

  Hide the text

- `fn strikethrough(&self) -> styles::StrikeThroughDisplay<'_, Self>`

  Cross out the text

- `fn color<Color: DynColor>(&self, color: Color) -> FgDynColorDisplay<'_, Color, Self>`

  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `fn on_color<Color: DynColor>(&self, color: Color) -> BgDynColorDisplay<'_, Color, Self>`

  Set the background color at runtime. Only use if you do not know what color to use at

- `fn fg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> FgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the foreground color to a specific RGB value.

- `fn bg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> BgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the background color to a specific RGB value.

- `fn truecolor(&self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self>`

  Sets the foreground color to an RGB value.

- `fn on_truecolor(&self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self>`

  Sets the background color to an RGB value.

- `fn style(&self, style: Style) -> Styled<&Self>`

  Apply a runtime-determined style

## Functions

### `style`

```rust
const fn style() -> Style
```

Helper to create [`Style`](#style)s more ergonomically

## Constants

### `DIMMED_SHIFT`

```rust
const DIMMED_SHIFT: u8 = 0u8;
```

### `ITALIC_SHIFT`

```rust
const ITALIC_SHIFT: u8 = 1u8;
```

### `UNDERLINE_SHIFT`

```rust
const UNDERLINE_SHIFT: u8 = 2u8;
```

### `BLINK_SHIFT`

```rust
const BLINK_SHIFT: u8 = 3u8;
```

### `BLINK_FAST_SHIFT`

```rust
const BLINK_FAST_SHIFT: u8 = 4u8;
```

### `REVERSED_SHIFT`

```rust
const REVERSED_SHIFT: u8 = 5u8;
```

### `HIDDEN_SHIFT`

```rust
const HIDDEN_SHIFT: u8 = 6u8;
```

### `STRIKETHROUGH_SHIFT`

```rust
const STRIKETHROUGH_SHIFT: u8 = 7u8;
```

## Macros

### `style_methods!`

### `color_methods!`

### `color_methods!`

### `style_methods!`

### `style_flags_methods!`

### `impl_fmt!`

