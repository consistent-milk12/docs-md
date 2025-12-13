# Crate `owo_colors`

|**Quick Links**|[`OwoColorize`](#owocolorize)|[`Style`](#style)|[`StyledList`](styled_list/index.md)|[`github`](https://github.com/owo-colors/owo-colors)|
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
  - [`dyn_colors`](#dyn-colors)
  - [`dyn_styles`](#dyn-styles)
  - [`styled_list`](#styled-list)
  - [`styles`](#styles)
  - [`private`](#private)
  - [`colored`](#colored)
- [Structs](#structs)
  - [`FgColorDisplay`](#fgcolordisplay)
  - [`BgColorDisplay`](#bgcolordisplay)
  - [`FgDynColorDisplay`](#fgdyncolordisplay)
  - [`BgDynColorDisplay`](#bgdyncolordisplay)
  - [`Rgb`](#rgb)
  - [`ComboColorDisplay`](#combocolordisplay)
  - [`ComboDynColorDisplay`](#combodyncolordisplay)
  - [`StyledList`](#styledlist)
  - [`ParseColorError`](#parsecolorerror)
  - [`Styled`](#styled)
  - [`Style`](#style)
  - [`StyleFlags`](#styleflags)
  - [`StylePrefixFormatter`](#styleprefixformatter)
  - [`StyleSuffixFormatter`](#stylesuffixformatter)
- [Enums](#enums)
  - [`AnsiColors`](#ansicolors)
  - [`CssColors`](#csscolors)
  - [`XtermColors`](#xtermcolors)
  - [`DynColors`](#dyncolors)
  - [`Effect`](#effect)
- [Traits](#traits)
  - [`Color`](#color)
  - [`DynColor`](#dyncolor)
  - [`OwoColorize`](#owocolorize)
- [Functions](#functions)
  - [`style`](#style)
- [Constants](#constants)
  - [`DIMMED_SHIFT`](#dimmed-shift)
  - [`ITALIC_SHIFT`](#italic-shift)
  - [`UNDERLINE_SHIFT`](#underline-shift)
  - [`BLINK_SHIFT`](#blink-shift)
  - [`BLINK_FAST_SHIFT`](#blink-fast-shift)
  - [`REVERSED_SHIFT`](#reversed-shift)
  - [`HIDDEN_SHIFT`](#hidden-shift)
  - [`STRIKETHROUGH_SHIFT`](#strikethrough-shift)
- [Macros](#macros)
  - [`style_methods!`](#style-methods)
  - [`color_methods!`](#color-methods)
  - [`color_methods!`](#color-methods)
  - [`style_methods!`](#style-methods)
  - [`style_flags_methods!`](#style-flags-methods)
  - [`impl_fmt!`](#impl-fmt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`colors`](#colors) | mod | Color types for used for being generic over the color |
| [`combo`](#combo) | mod |  |
| [`dyn_colors`](#dyn-colors) | mod |  |
| [`dyn_styles`](#dyn-styles) | mod |  |
| [`styled_list`](#styled-list) | mod |  |
| [`styles`](#styles) | mod | Different display styles (strikethrough, bold, etc.) |
| [`private`](#private) | mod |  |
| [`colored`](#colored) | mod | Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from [`colored`](https://docs.rs/colored) to owo-colors. |
| [`FgColorDisplay`](#fgcolordisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground color. |
| [`BgColorDisplay`](#bgcolordisplay) | struct | Transparent wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the background color. |
| [`FgDynColorDisplay`](#fgdyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground color. |
| [`BgDynColorDisplay`](#bgdyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the background color. |
| [`Rgb`](#rgb) | struct |  |
| [`ComboColorDisplay`](#combocolordisplay) | struct |  |
| [`ComboDynColorDisplay`](#combodyncolordisplay) | struct |  |
| [`StyledList`](#styledlist) | struct |  |
| [`ParseColorError`](#parsecolorerror) | struct | An error for when the color can not be parsed from a string at runtime |
| [`Styled`](#styled) | struct | A wrapper type which applies a [`Style`] when displaying the inner type |
| [`Style`](#style) | struct | A pre-computed style that can be applied to a struct using [`OwoColorize::style`]. |
| [`StyleFlags`](#styleflags) | struct |  |
| [`StylePrefixFormatter`](#styleprefixformatter) | struct | Formatter for the prefix of a [`Style`]. |
| [`StyleSuffixFormatter`](#stylesuffixformatter) | struct | Formatter for the suffix of a [`Style`]. |
| [`AnsiColors`](#ansicolors) | enum |  |
| [`CssColors`](#csscolors) | enum |  |
| [`XtermColors`](#xtermcolors) | enum |  |
| [`DynColors`](#dyncolors) | enum | An enum describing runtime-configurable colors |
| [`Effect`](#effect) | enum | A runtime-configurable text effect for use with [`Style`] |
| [`Color`](#color) | trait | A trait for describing a type which can be used with [`FgColorDisplay`] or [`BgColorDisplay`] |
| [`DynColor`](#dyncolor) | trait | A trait describing a runtime-configurable color which can displayed using [`FgDynColorDisplay`] or [`BgDynColorDisplay`]. |
| [`OwoColorize`](#owocolorize) | trait | Extension trait for colorizing a type which implements any std formatter ([`Display`](core::fmt::Display), [`Debug`](core::fmt::Debug), [`UpperHex`](core::fmt::UpperHex), etc.) |
| [`style`](#style) | fn | Helper to create [`Style`]s more ergonomically |
| [`DIMMED_SHIFT`](#dimmed-shift) | const |  |
| [`ITALIC_SHIFT`](#italic-shift) | const |  |
| [`UNDERLINE_SHIFT`](#underline-shift) | const |  |
| [`BLINK_SHIFT`](#blink-shift) | const |  |
| [`BLINK_FAST_SHIFT`](#blink-fast-shift) | const |  |
| [`REVERSED_SHIFT`](#reversed-shift) | const |  |
| [`HIDDEN_SHIFT`](#hidden-shift) | const |  |
| [`STRIKETHROUGH_SHIFT`](#strikethrough-shift) | const |  |
| [`style_methods!`](#style-methods) | macro |  |
| [`color_methods!`](#color-methods) | macro |  |
| [`color_methods!`](#color-methods) | macro |  |
| [`style_methods!`](#style-methods) | macro |  |
| [`style_flags_methods!`](#style-flags-methods) | macro |  |
| [`impl_fmt!`](#impl-fmt) | macro |  |

## Modules

- [`colors`](colors/index.md) — Color types for used for being generic over the color
- [`combo`](combo/index.md)
- [`dyn_colors`](dyn_colors/index.md)
- [`dyn_styles`](dyn_styles/index.md)
- [`styled_list`](styled_list/index.md)
- [`styles`](styles/index.md) — Different display styles (strikethrough, bold, etc.)
- [`private`](private/index.md)
- [`colored`](colored/index.md) — Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from

## Structs

### `FgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct FgColorDisplay<'a, C: Color, T: ?Sized>(&'a T, core::marker::PhantomData<C>);
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:154`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L154)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- <span id="fgcolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

  Create a new [`FgColorDisplay`](#fgcolordisplay), from a reference to a type which implements

  [`Color`](#color).

  

  This is a const function: in non-const contexts, `OwoColorize::fg` or one of the

  other methods on it may be more convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::Green, FgColorDisplay};

  

  const GREEN_TEXT: FgColorDisplay<Green, str> = FgColorDisplay::new("green");

  

  println!("{}", GREEN_TEXT);

  assert_eq!(format!("{}", GREEN_TEXT), "\x1b[32mgreen\x1b[39m");

  ```

- <span id="fgcolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  Typical use:

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_blue() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_blue() {

      "hello".blue().into_styled()

  } else {

      "hello".green().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[34mhello\x1b[0m");

  ```

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::{Blue, Green}, FgColorDisplay, Styled};

  

  const fn is_blue() -> bool {

      // ...

      true

  }

  

  const STYLED_STR: Styled<&str> = if is_blue() {

      FgColorDisplay::<Blue, _>::new("Hello").into_styled()

  } else {

      FgColorDisplay::<Green, _>::new("Hello").into_styled()

  };

  

  println!("{}", STYLED_STR);

  assert_eq!(format!("{}", STYLED_STR), "\x1b[34mHello\x1b[0m");

  ```

- <span id="fgcolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>` — [`FgDynColorDisplay`](#fgdyncolordisplay)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

- <span id="fgcolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay), [`Color`](#color)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="fgcolordisplay-fg"></span>`const fn fg<C: Color>(self) -> FgColorDisplay<'a, C, T>` — [`FgColorDisplay`](#fgcolordisplay)

  Set the foreground color generically

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "red foreground".fg::<Red>());

  ```

- <span id="fgcolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay)

  Set the background color generically.

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "black background".bg::<Black>());

  ```

- <span id="fgcolordisplay-black"></span>`const fn black(self) -> FgColorDisplay<'a, colors::Black, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Black`](colors/index.md#black)

  Change the foreground color to black

- <span id="fgcolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Black`](colors/index.md#black)

  Change the background color to black

- <span id="fgcolordisplay-red"></span>`const fn red(self) -> FgColorDisplay<'a, colors::Red, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Red`](colors/index.md#red)

  Change the foreground color to red

- <span id="fgcolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Red`](colors/index.md#red)

  Change the background color to red

- <span id="fgcolordisplay-green"></span>`const fn green(self) -> FgColorDisplay<'a, colors::Green, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Green`](colors/index.md#green)

  Change the foreground color to green

- <span id="fgcolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Green`](colors/index.md#green)

  Change the background color to green

- <span id="fgcolordisplay-yellow"></span>`const fn yellow(self) -> FgColorDisplay<'a, colors::Yellow, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the foreground color to yellow

- <span id="fgcolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the background color to yellow

- <span id="fgcolordisplay-blue"></span>`const fn blue(self) -> FgColorDisplay<'a, colors::Blue, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Blue`](colors/index.md#blue)

  Change the foreground color to blue

- <span id="fgcolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Blue`](colors/index.md#blue)

  Change the background color to blue

- <span id="fgcolordisplay-magenta"></span>`const fn magenta(self) -> FgColorDisplay<'a, colors::Magenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to magenta

- <span id="fgcolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to magenta

- <span id="fgcolordisplay-purple"></span>`const fn purple(self) -> FgColorDisplay<'a, colors::Magenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to purple

- <span id="fgcolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to purple

- <span id="fgcolordisplay-cyan"></span>`const fn cyan(self) -> FgColorDisplay<'a, colors::Cyan, T>` — [`FgColorDisplay`](#fgcolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the foreground color to cyan

- <span id="fgcolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the background color to cyan

- <span id="fgcolordisplay-white"></span>`const fn white(self) -> FgColorDisplay<'a, colors::White, T>` — [`FgColorDisplay`](#fgcolordisplay), [`White`](colors/index.md#white)

  Change the foreground color to white

- <span id="fgcolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`White`](colors/index.md#white)

  Change the background color to white

- <span id="fgcolordisplay-bright-black"></span>`const fn bright_black(self) -> FgColorDisplay<'a, colors::BrightBlack, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the foreground color to bright black

- <span id="fgcolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the background color to bright black

- <span id="fgcolordisplay-bright-red"></span>`const fn bright_red(self) -> FgColorDisplay<'a, colors::BrightRed, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the foreground color to bright red

- <span id="fgcolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the background color to bright red

- <span id="fgcolordisplay-bright-green"></span>`const fn bright_green(self) -> FgColorDisplay<'a, colors::BrightGreen, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the foreground color to bright green

- <span id="fgcolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the background color to bright green

- <span id="fgcolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> FgColorDisplay<'a, colors::BrightYellow, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the foreground color to bright yellow

- <span id="fgcolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the background color to bright yellow

- <span id="fgcolordisplay-bright-blue"></span>`const fn bright_blue(self) -> FgColorDisplay<'a, colors::BrightBlue, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the foreground color to bright blue

- <span id="fgcolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the background color to bright blue

- <span id="fgcolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> FgColorDisplay<'a, colors::BrightMagenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright magenta

- <span id="fgcolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright magenta

- <span id="fgcolordisplay-bright-purple"></span>`const fn bright_purple(self) -> FgColorDisplay<'a, colors::BrightMagenta, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright purple

- <span id="fgcolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright purple

- <span id="fgcolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> FgColorDisplay<'a, colors::BrightCyan, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the foreground color to bright cyan

- <span id="fgcolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the background color to bright cyan

- <span id="fgcolordisplay-bright-white"></span>`const fn bright_white(self) -> FgColorDisplay<'a, colors::BrightWhite, T>` — [`FgColorDisplay`](#fgcolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the foreground color to bright white

- <span id="fgcolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the background color to bright white

#### Trait Implementations

##### `impl<T> Any for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Binary> Binary for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Debug> Debug for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Display> Display for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Color: crate::Color, T: ?Sized + fmt::LowerExp> LowerExp for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::LowerHex> LowerHex for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Octal> Octal for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for FgColorDisplay<'a, C, T>`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Pointer> Pointer for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fgcolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FgColorDisplay<'a, C, T>`

- <span id="fgcolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fgcolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Color: crate::Color, T: ?Sized + fmt::UpperExp> UpperExp for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::UpperHex> UpperHex for FgColorDisplay<'a, Color, T>`

- <span id="fgcolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct BgColorDisplay<'a, C: Color, T: ?Sized>(&'a T, core::marker::PhantomData<C>);
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:160`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L160)*

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- <span id="bgcolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

  Create a new [`BgColorDisplay`](#bgcolordisplay), from a reference to a type which implements

  [`Color`](#color).

  

  This is a const function: in non-const contexts, `OwoColorize::bg` may be more

  convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::Red, BgColorDisplay};

  

  const RED_BG_TEXT: BgColorDisplay<Red, str> = BgColorDisplay::new("red background");

  

  println!("{}", RED_BG_TEXT);

  assert_eq!(format!("{}", RED_BG_TEXT), "\x1b[41mred background\x1b[49m");

  ```

- <span id="bgcolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  Typical use:

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_red() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_red() {

      "hello".on_red().into_styled()

  } else {

      "hello".on_yellow().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[41mhello\x1b[0m");

  ```

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::{Red, Yellow}, BgColorDisplay, Styled};

  

  const fn is_red() -> bool {

      // ...

      true

  }

  

  const STYLED_STR: Styled<&str> = if is_red() {

      BgColorDisplay::<Red, _>::new("Hello").into_styled()

  } else {

      BgColorDisplay::<Yellow, _>::new("Hello").into_styled()

  };

  

  println!("{}", STYLED_STR);

  assert_eq!(format!("{}", STYLED_STR), "\x1b[41mHello\x1b[0m");

  ```

- <span id="bgcolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay), [`Color`](#color)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

- <span id="bgcolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>` — [`BgDynColorDisplay`](#bgdyncolordisplay)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="bgcolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay)

  Set the foreground color generically

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "red foreground".fg::<Red>());

  ```

- <span id="bgcolordisplay-bg"></span>`const fn bg<C: Color>(self) -> BgColorDisplay<'a, C, T>` — [`BgColorDisplay`](#bgcolordisplay)

  Set the background color generically.

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "black background".bg::<Black>());

  ```

- <span id="bgcolordisplay-on-black"></span>`const fn on_black(self) -> BgColorDisplay<'a, colors::Black, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Black`](colors/index.md#black)

  Change the background color to black

- <span id="bgcolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Black`](colors/index.md#black)

  Change the foreground color to black

- <span id="bgcolordisplay-on-red"></span>`const fn on_red(self) -> BgColorDisplay<'a, colors::Red, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Red`](colors/index.md#red)

  Change the background color to red

- <span id="bgcolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Red`](colors/index.md#red)

  Change the foreground color to red

- <span id="bgcolordisplay-on-green"></span>`const fn on_green(self) -> BgColorDisplay<'a, colors::Green, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Green`](colors/index.md#green)

  Change the background color to green

- <span id="bgcolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Green`](colors/index.md#green)

  Change the foreground color to green

- <span id="bgcolordisplay-on-yellow"></span>`const fn on_yellow(self) -> BgColorDisplay<'a, colors::Yellow, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the background color to yellow

- <span id="bgcolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the foreground color to yellow

- <span id="bgcolordisplay-on-blue"></span>`const fn on_blue(self) -> BgColorDisplay<'a, colors::Blue, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Blue`](colors/index.md#blue)

  Change the background color to blue

- <span id="bgcolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Blue`](colors/index.md#blue)

  Change the foreground color to blue

- <span id="bgcolordisplay-on-magenta"></span>`const fn on_magenta(self) -> BgColorDisplay<'a, colors::Magenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to magenta

- <span id="bgcolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to magenta

- <span id="bgcolordisplay-on-purple"></span>`const fn on_purple(self) -> BgColorDisplay<'a, colors::Magenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to purple

- <span id="bgcolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to purple

- <span id="bgcolordisplay-on-cyan"></span>`const fn on_cyan(self) -> BgColorDisplay<'a, colors::Cyan, T>` — [`BgColorDisplay`](#bgcolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the background color to cyan

- <span id="bgcolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the foreground color to cyan

- <span id="bgcolordisplay-on-white"></span>`const fn on_white(self) -> BgColorDisplay<'a, colors::White, T>` — [`BgColorDisplay`](#bgcolordisplay), [`White`](colors/index.md#white)

  Change the background color to white

- <span id="bgcolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`White`](colors/index.md#white)

  Change the foreground color to white

- <span id="bgcolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> BgColorDisplay<'a, colors::BrightBlack, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the background color to bright black

- <span id="bgcolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the foreground color to bright black

- <span id="bgcolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> BgColorDisplay<'a, colors::BrightRed, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the background color to bright red

- <span id="bgcolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the foreground color to bright red

- <span id="bgcolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> BgColorDisplay<'a, colors::BrightGreen, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the background color to bright green

- <span id="bgcolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the foreground color to bright green

- <span id="bgcolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> BgColorDisplay<'a, colors::BrightYellow, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the background color to bright yellow

- <span id="bgcolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the foreground color to bright yellow

- <span id="bgcolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> BgColorDisplay<'a, colors::BrightBlue, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the background color to bright blue

- <span id="bgcolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the foreground color to bright blue

- <span id="bgcolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> BgColorDisplay<'a, colors::BrightMagenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright magenta

- <span id="bgcolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright magenta

- <span id="bgcolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> BgColorDisplay<'a, colors::BrightMagenta, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright purple

- <span id="bgcolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright purple

- <span id="bgcolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> BgColorDisplay<'a, colors::BrightCyan, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the background color to bright cyan

- <span id="bgcolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the foreground color to bright cyan

- <span id="bgcolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> BgColorDisplay<'a, colors::BrightWhite, T>` — [`BgColorDisplay`](#bgcolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the background color to bright white

- <span id="bgcolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the foreground color to bright white

#### Trait Implementations

##### `impl<T> Any for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Binary> Binary for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Debug> Debug for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Display> Display for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Color: crate::Color, T: ?Sized + fmt::LowerExp> LowerExp for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::LowerHex> LowerHex for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Octal> Octal for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BgColorDisplay<'a, C, T>`

##### `impl<Color: crate::Color, T: ?Sized + fmt::Pointer> Pointer for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bgcolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for BgColorDisplay<'a, C, T>`

- <span id="bgcolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bgcolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Color: crate::Color, T: ?Sized + fmt::UpperExp> UpperExp for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::Color, T: ?Sized + fmt::UpperHex> UpperHex for BgColorDisplay<'a, Color, T>`

- <span id="bgcolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>(&'a T, Color);
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:165`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L165)*

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- <span id="cratefgdyncolordisplay-new"></span>`const fn new(thing: &'a T, color: Fg) -> Self`

  Create a new [`FgDynColorDisplay`](#fgdyncolordisplay), from a reference to a type which implements

  [`DynColor`](#dyncolor).

  

  This is a const function: in non-const contexts, `OwoColorize::color` may be more

  convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{AnsiColors, FgDynColorDisplay};

  

  const DYN_RED_TEXT: FgDynColorDisplay<AnsiColors, str> =

     FgDynColorDisplay::new("red text (dynamic)", AnsiColors::Red);

  

  println!("{}", DYN_RED_TEXT);

  assert_eq!(format!("{}", DYN_RED_TEXT), "\x1b[31mred text (dynamic)\x1b[39m");

  ```

- <span id="cratefgdyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::{AnsiColors, CssColors, OwoColorize};

  

  fn is_blue() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_blue() {

      "hello".color(AnsiColors::Blue).into_styled()

  } else {

      "hello".color(CssColors::DarkSeaGreen).into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[34mhello\x1b[0m");

  ```

- <span id="cratefgdyncolordisplay-on-color"></span>`const fn on_color<Bg: DynColor>(self, bg: Bg) -> ComboDynColorDisplay<'a, Fg, Bg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="cratefgdyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>` — [`FgDynColorDisplay`](#fgdyncolordisplay)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

#### Trait Implementations

##### `impl<T> Any for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Binary> Binary for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Debug> Debug for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Display> Display for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::LowerExp> LowerExp for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::LowerHex> LowerHex for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Octal> Octal for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for FgDynColorDisplay<'a, Color, T>`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Pointer> Pointer for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fgdyncolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fgdyncolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::UpperExp> UpperExp for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::UpperHex> UpperHex for FgDynColorDisplay<'a, Color, T>`

- <span id="fgdyncolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>(&'a T, Color);
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:170`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L170)*

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- <span id="cratebgdyncolordisplay-new"></span>`const fn new(thing: &'a T, color: Bg) -> Self`

  Create a new [`BgDynColorDisplay`](#bgdyncolordisplay), from a reference to a type which implements

  [`DynColor`](#dyncolor).

  

  This is a const function: in non-const contexts, `OwoColorize::on_color` may be more

  convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{AnsiColors, BgDynColorDisplay};

  

  const DYN_GREEN_BG_TEXT: BgDynColorDisplay<AnsiColors, str> =

     BgDynColorDisplay::new("green background (dynamic)", AnsiColors::Green);

  

  println!("{}", DYN_GREEN_BG_TEXT);

  assert_eq!(format!("{}", DYN_GREEN_BG_TEXT), "\x1b[42mgreen background (dynamic)\x1b[49m");

  ```

- <span id="cratebgdyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  ```rust

  use owo_colors::{AnsiColors, CssColors, OwoColorize};

  

  fn is_red() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_red() {

      "hello".on_color(AnsiColors::Red).into_styled()

  } else {

      "hello".on_color(CssColors::LightGoldenRodYellow).into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[41mhello\x1b[0m");

  ```

- <span id="cratebgdyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>` — [`BgDynColorDisplay`](#bgdyncolordisplay)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="cratebgdyncolordisplay-color"></span>`const fn color<Fg: DynColor>(self, fg: Fg) -> ComboDynColorDisplay<'a, Fg, Bg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

#### Trait Implementations

##### `impl<T> Any for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Binary> Binary for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Debug> Debug for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Display> Display for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::LowerExp> LowerExp for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::LowerHex> LowerHex for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Octal> Octal for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for BgDynColorDisplay<'a, Color, T>`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::Pointer> Pointer for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bgdyncolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bgdyncolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::UpperExp> UpperExp for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Color: crate::DynColor, T: ?Sized + fmt::UpperHex> UpperHex for BgDynColorDisplay<'a, Color, T>`

- <span id="bgdyncolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

*Defined in [`owo-colors-4.2.3/src/colors/dynamic.rs:10`](../../.source_1765633015/owo-colors-4.2.3/src/colors/dynamic.rs#L10)*

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for Rgb`

- <span id="rgb-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rgb`

- <span id="rgb-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rgb`

- <span id="rgb-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Rgb`

- <span id="rgb-clone"></span>`fn clone(&self) -> Rgb` — [`Rgb`](colors/dynamic/index.md#rgb)

##### `impl CloneToUninit for Rgb`

- <span id="rgb-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- <span id="rgb-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for Rgb`

- <span id="rgb-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Rgb`

##### `impl<T> From for Rgb`

- <span id="rgb-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Rgb`

- <span id="rgb-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Rgb`

##### `impl PartialEq for Rgb`

- <span id="rgb-partialeq-eq"></span>`fn eq(&self, other: &Rgb) -> bool` — [`Rgb`](colors/dynamic/index.md#rgb)

##### `impl StructuralPartialEq for Rgb`

##### `impl<U> TryFrom for Rgb`

- <span id="rgb-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rgb-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rgb`

- <span id="rgb-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rgb-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>(&'a T, core::marker::PhantomData<(Fg, Bg)>);
```

*Defined in [`owo-colors-4.2.3/src/combo.rs:11`](../../.source_1765633015/owo-colors-4.2.3/src/combo.rs#L11)*

A wrapper type which applies both a foreground and background color

#### Implementations

- <span id="combocolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

  Create a new [`ComboColorDisplay`](combo/index.md), from a pair of foreground and background types

  which implement [`Color`](#color).

  

  This is a const function: in non-const contexts, calling the [`OwoColorize`](#owocolorize)

  functions may be more convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::{Blue, White}, ComboColorDisplay};

  

  const COMBO_TEXT: ComboColorDisplay<Blue, White, str> =

     ComboColorDisplay::new("blue text on white background");

  

  println!("{}", COMBO_TEXT);

  assert_eq!(format!("{}", COMBO_TEXT), "\x1b[34;47mblue text on white background\x1b[0m");

  ```

- <span id="combocolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  Typical use:

  

  ```rust

  use owo_colors::OwoColorize;

  

  fn is_black_on_white() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_black_on_white() {

      "hello".black().on_white().into_styled()

  } else {

      "hello".white().on_black().into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[30;47mhello\x1b[0m");

  ```

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{colors::{Black, White}, ComboColorDisplay, Styled};

  

  const fn is_black_on_white() -> bool {

      // ...

      true

  }

  

  const STYLED_STR: Styled<&str> = if is_black_on_white() {

      ComboColorDisplay::<Black, White, _>::new("Hello").into_styled()

  } else {

      ComboColorDisplay::<White, Black, _>::new("Hello").into_styled()

  };

  

  println!("{}", STYLED_STR);

  assert_eq!(format!("{}", STYLED_STR), "\x1b[30;47mHello\x1b[0m");

  ```

- <span id="combocolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay), [`Color`](#color)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="combocolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay), [`Color`](#color)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

- <span id="combocolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay)

  Set the foreground color generically

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "red foreground".fg::<Red>());

  ```

- <span id="combocolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay)

  Set the background color generically.

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "black background".bg::<Black>());

  ```

- <span id="combocolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Black`](colors/index.md#black)

  Change the background color to black

- <span id="combocolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Black`](colors/index.md#black)

  Change the foreground color to black

- <span id="combocolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Red`](colors/index.md#red)

  Change the background color to red

- <span id="combocolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Red`](colors/index.md#red)

  Change the foreground color to red

- <span id="combocolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Green`](colors/index.md#green)

  Change the background color to green

- <span id="combocolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Green`](colors/index.md#green)

  Change the foreground color to green

- <span id="combocolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the background color to yellow

- <span id="combocolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Yellow`](colors/index.md#yellow)

  Change the foreground color to yellow

- <span id="combocolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Blue`](colors/index.md#blue)

  Change the background color to blue

- <span id="combocolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Blue`](colors/index.md#blue)

  Change the foreground color to blue

- <span id="combocolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to magenta

- <span id="combocolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to magenta

- <span id="combocolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the background color to purple

- <span id="combocolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Magenta`](colors/index.md#magenta)

  Change the foreground color to purple

- <span id="combocolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the background color to cyan

- <span id="combocolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`Cyan`](colors/index.md#cyan)

  Change the foreground color to cyan

- <span id="combocolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`White`](colors/index.md#white)

  Change the background color to white

- <span id="combocolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`White`](colors/index.md#white)

  Change the foreground color to white

- <span id="combocolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the background color to bright black

- <span id="combocolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlack`](colors/index.md#brightblack)

  Change the foreground color to bright black

- <span id="combocolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the background color to bright red

- <span id="combocolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightRed`](colors/index.md#brightred)

  Change the foreground color to bright red

- <span id="combocolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the background color to bright green

- <span id="combocolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightGreen`](colors/index.md#brightgreen)

  Change the foreground color to bright green

- <span id="combocolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the background color to bright yellow

- <span id="combocolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightYellow`](colors/index.md#brightyellow)

  Change the foreground color to bright yellow

- <span id="combocolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the background color to bright blue

- <span id="combocolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightBlue`](colors/index.md#brightblue)

  Change the foreground color to bright blue

- <span id="combocolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright magenta

- <span id="combocolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright magenta

- <span id="combocolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the background color to bright purple

- <span id="combocolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightMagenta`](colors/index.md#brightmagenta)

  Change the foreground color to bright purple

- <span id="combocolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the background color to bright cyan

- <span id="combocolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightCyan`](colors/index.md#brightcyan)

  Change the foreground color to bright cyan

- <span id="combocolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the background color to bright white

- <span id="combocolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](combo/index.md#combocolordisplay), [`BrightWhite`](colors/index.md#brightwhite)

  Change the foreground color to bright white

#### Trait Implementations

##### `impl<T> Any for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::Binary> Binary for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::Debug> Debug for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::Display> Display for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::LowerExp> LowerExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::LowerHex> LowerHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::Octal> Octal for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ComboColorDisplay<'a, Fg, Bg, T>`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::Pointer> Pointer for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="combocolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="combocolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::UpperExp> UpperExp for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: Color, Bg: Color, T: ?Sized + fmt::UpperHex> UpperHex for ComboColorDisplay<'a, Fg, Bg, T>`

- <span id="combocolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>`

```rust
struct ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>(&'a T, Fg, Bg);
```

*Defined in [`owo-colors-4.2.3/src/combo.rs:17`](../../.source_1765633015/owo-colors-4.2.3/src/combo.rs#L17)*

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](combo/index.md) instead.

#### Implementations

- <span id="combodyncolordisplay-new"></span>`const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`

  Create a new [`ComboDynColorDisplay`](combo/index.md), from a pair of types which implement

  [`DynColor`](#dyncolor).

  

  This is a const function: in non-const contexts, other functions may be more convenient.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{ComboDynColorDisplay, XtermColors};

  

  const COMBO_DYN_TEXT: ComboDynColorDisplay<XtermColors, XtermColors, str> =

      ComboDynColorDisplay::new(

          "blue text on lilac background (dynamic)",

          XtermColors::BlueRibbon,

          XtermColors::WistfulLilac,

      );

  

  println!("{}", COMBO_DYN_TEXT);

  assert_eq!(format!("{}", COMBO_DYN_TEXT), "\x1b[38;5;27;48;5;146mblue text on lilac background (dynamic)\x1b[0m");

  ```

- <span id="combodyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](#styled)

  Convert self to a generic [`Styled`](#styled).

  

  This method erases color-related type parameters, and can be

  used to unify types across branches.

  

  # Example

  

  Typical use:

  

  ```rust

  use owo_colors::{AnsiColors, CssColors, OwoColorize};

  

  fn is_black_on_white() -> bool {

      // ...

      true

  }

  

  let styled_str = if is_black_on_white() {

      "hello".color(AnsiColors::Black).on_color(AnsiColors::White).into_styled()

  } else {

      "hello".color(CssColors::White).on_color(CssColors::Black).into_styled()

  };

  

  println!("{}", styled_str);

  assert_eq!(format!("{}", styled_str), "\x1b[30;47mhello\x1b[0m");

  ```

- <span id="combodyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="combodyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>` — [`ComboDynColorDisplay`](combo/index.md#combodyncolordisplay)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

#### Trait Implementations

##### `impl<T> Any for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Binary> Binary for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Debug> Debug for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Display> Display for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerExp> LowerExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerHex> LowerHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Octal> Octal for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for ComboDynColorDisplay<'a, Fg, Bg, T>`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Pointer> Pointer for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="combodyncolordisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="combodyncolordisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperExp> UpperExp for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperHex> UpperHex for ComboDynColorDisplay<'a, Fg, Bg, T>`

- <span id="combodyncolordisplay-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StyledList<T, U>`

```rust
struct StyledList<T, U>(T, core::marker::PhantomData<fn(U)>)
where
    T: AsRef<[U]>,
    U: IsStyled;
```

*Defined in [`owo-colors-4.2.3/src/styled_list.rs:64-67`](../../.source_1765633015/owo-colors-4.2.3/src/styled_list.rs#L64-L67)*

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

##### `impl<T> Any for StyledList<T, U>`

- <span id="styledlist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyledList<T, U>`

- <span id="styledlist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyledList<T, U>`

- <span id="styledlist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, U> Display for StyledList<T, U>`

- <span id="styledlist-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StyledList<T, U>`

- <span id="styledlist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for StyledList<T, U>`

- <span id="styledlist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StyledList<T, U>`

##### `impl<T, U> TryFrom for StyledList<T, U>`

- <span id="styledlist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledlist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StyledList<T, U>`

- <span id="styledlist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledlist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseColorError`

```rust
struct ParseColorError;
```

*Defined in [`owo-colors-4.2.3/src/dyn_colors.rs:72`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_colors.rs#L72)*

An error for when the color can not be parsed from a string at runtime

#### Trait Implementations

##### `impl Any for ParseColorError`

- <span id="parsecolorerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseColorError`

- <span id="parsecolorerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseColorError`

- <span id="parsecolorerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseColorError`

- <span id="parsecolorerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseColorError`

- <span id="parsecolorerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseColorError`

- <span id="parsecolorerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ParseColorError`

##### `impl<U> TryFrom for ParseColorError`

- <span id="parsecolorerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsecolorerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseColorError`

- <span id="parsecolorerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsecolorerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Styled<T>`

```rust
struct Styled<T> {
    target: T,
    pub style: Style,
}
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:60-65`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L60-L65)*

A wrapper type which applies a [`Style`](#style) when displaying the inner type

#### Fields

- **`target`**: `T`

  The target value to be styled

- **`style`**: `Style`

  The style to apply to target

#### Implementations

- <span id="styled-inner"></span>`const fn inner(&self) -> &T`

  Returns a reference to the inner value to be styled

- <span id="styled-inner-mut"></span>`const fn inner_mut(&mut self) -> &mut T`

  Returns a mutable reference to the inner value to be styled.

  

  *This method is const on Rust 1.83+.*

#### Trait Implementations

##### `impl<T> Any for Styled<T>`

- <span id="styled-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T: fmt::Binary> Binary for Styled<T>`

- <span id="styled-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for Styled<T>`

- <span id="styled-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Styled<T>`

- <span id="styled-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for Styled<T>`

- <span id="styled-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Display for Styled<T>`

- <span id="styled-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Styled<T>`

- <span id="styled-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Styled<T>`

- <span id="styled-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Display> IsStyled for crate::Styled<T>`

- <span id="cratestyled-isstyled-type-inner"></span>`type Inner = T`

- <span id="cratestyled-isstyled-style"></span>`fn style(&self) -> &Style` — [`Style`](#style)

- <span id="cratestyled-isstyled-inner"></span>`fn inner(&self) -> &T`

##### `impl<T: fmt::LowerExp> LowerExp for Styled<T>`

- <span id="styled-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::LowerHex> LowerHex for Styled<T>`

- <span id="styled-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Octal> Octal for Styled<T>`

- <span id="styled-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Styled<T>`

##### `impl<T: fmt::Pointer> Pointer for Styled<T>`

- <span id="styled-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> TryFrom for Styled<T>`

- <span id="styled-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styled-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Styled<T>`

- <span id="styled-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styled-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: fmt::UpperExp> UpperExp for Styled<T>`

- <span id="styled-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::UpperHex> UpperHex for Styled<T>`

- <span id="styled-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Style`

```rust
struct Style {
    fg: Option<crate::DynColors>,
    bg: Option<crate::DynColors>,
    bold: bool,
    style_flags: StyleFlags,
}
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:83-88`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L83-L88)*

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

  Create a new style to be applied later

- <span id="style-style"></span>`const fn style<T>(&self, target: T) -> Styled<T>` — [`Styled`](#styled)

  Apply the style to a given struct to output.

  

  # Example

  

  Usage in const contexts:

  

  ```rust

  use owo_colors::{OwoColorize, Style, Styled};

  

  const STYLED_TEXT: Styled<&'static str> = Style::new().bold().italic().style("bold and italic text");

  

  println!("{}", STYLED_TEXT);

  assert_eq!(format!("{}", STYLED_TEXT), "\u{1b}[1;3mbold and italic text\u{1b}[0m");

  ```

- <span id="style-fg"></span>`const fn fg<C: Color>(self) -> Self`

  Set the foreground color generically

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "red foreground".fg::<Red>());

  ```

- <span id="style-bg"></span>`const fn bg<C: Color>(self) -> Self`

  Set the background color generically.

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "black background".bg::<Black>());

  ```

- <span id="style-remove-fg"></span>`const fn remove_fg(self) -> Self`

  Removes the foreground color from the style. Note that this does not apply

  the default color, but rather represents not changing the current terminal color.

  

  If you wish to actively change the terminal color back to the default, see

  `Style::default_color`.

- <span id="style-remove-bg"></span>`const fn remove_bg(self) -> Self`

  Removes the background color from the style. Note that this does not apply

  the default color, but rather represents not changing the current terminal color.

  

  If you wish to actively change the terminal color back to the default, see

  `Style::on_default_color`.

- <span id="style-black"></span>`const fn black(self) -> Self`

  Change the foreground color to black

- <span id="style-on-black"></span>`const fn on_black(self) -> Self`

  Change the foreground color to black

- <span id="style-red"></span>`const fn red(self) -> Self`

  Change the foreground color to red

- <span id="style-on-red"></span>`const fn on_red(self) -> Self`

  Change the foreground color to red

- <span id="style-green"></span>`const fn green(self) -> Self`

  Change the foreground color to green

- <span id="style-on-green"></span>`const fn on_green(self) -> Self`

  Change the foreground color to green

- <span id="style-yellow"></span>`const fn yellow(self) -> Self`

  Change the foreground color to yellow

- <span id="style-on-yellow"></span>`const fn on_yellow(self) -> Self`

  Change the foreground color to yellow

- <span id="style-blue"></span>`const fn blue(self) -> Self`

  Change the foreground color to blue

- <span id="style-on-blue"></span>`const fn on_blue(self) -> Self`

  Change the foreground color to blue

- <span id="style-magenta"></span>`const fn magenta(self) -> Self`

  Change the foreground color to magenta

- <span id="style-on-magenta"></span>`const fn on_magenta(self) -> Self`

  Change the foreground color to magenta

- <span id="style-purple"></span>`const fn purple(self) -> Self`

  Change the foreground color to purple

- <span id="style-on-purple"></span>`const fn on_purple(self) -> Self`

  Change the foreground color to purple

- <span id="style-cyan"></span>`const fn cyan(self) -> Self`

  Change the foreground color to cyan

- <span id="style-on-cyan"></span>`const fn on_cyan(self) -> Self`

  Change the foreground color to cyan

- <span id="style-white"></span>`const fn white(self) -> Self`

  Change the foreground color to white

- <span id="style-on-white"></span>`const fn on_white(self) -> Self`

  Change the foreground color to white

- <span id="style-default-color"></span>`const fn default_color(self) -> Self`

  Change the foreground color to the terminal default

- <span id="style-on-default-color"></span>`const fn on_default_color(self) -> Self`

  Change the foreground color to the terminal default

- <span id="style-bright-black"></span>`const fn bright_black(self) -> Self`

  Change the foreground color to bright black

- <span id="style-on-bright-black"></span>`const fn on_bright_black(self) -> Self`

  Change the foreground color to bright black

- <span id="style-bright-red"></span>`const fn bright_red(self) -> Self`

  Change the foreground color to bright red

- <span id="style-on-bright-red"></span>`const fn on_bright_red(self) -> Self`

  Change the foreground color to bright red

- <span id="style-bright-green"></span>`const fn bright_green(self) -> Self`

  Change the foreground color to bright green

- <span id="style-on-bright-green"></span>`const fn on_bright_green(self) -> Self`

  Change the foreground color to bright green

- <span id="style-bright-yellow"></span>`const fn bright_yellow(self) -> Self`

  Change the foreground color to bright yellow

- <span id="style-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> Self`

  Change the foreground color to bright yellow

- <span id="style-bright-blue"></span>`const fn bright_blue(self) -> Self`

  Change the foreground color to bright blue

- <span id="style-on-bright-blue"></span>`const fn on_bright_blue(self) -> Self`

  Change the foreground color to bright blue

- <span id="style-bright-magenta"></span>`const fn bright_magenta(self) -> Self`

  Change the foreground color to bright magenta

- <span id="style-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> Self`

  Change the foreground color to bright magenta

- <span id="style-bright-purple"></span>`const fn bright_purple(self) -> Self`

  Change the foreground color to bright purple

- <span id="style-on-bright-purple"></span>`const fn on_bright_purple(self) -> Self`

  Change the foreground color to bright purple

- <span id="style-bright-cyan"></span>`const fn bright_cyan(self) -> Self`

  Change the foreground color to bright cyan

- <span id="style-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> Self`

  Change the foreground color to bright cyan

- <span id="style-bright-white"></span>`const fn bright_white(self) -> Self`

  Change the foreground color to bright white

- <span id="style-on-bright-white"></span>`const fn on_bright_white(self) -> Self`

  Change the foreground color to bright white

- <span id="style-bold"></span>`const fn bold(self) -> Self`

  Make the text bold

- <span id="style-dimmed"></span>`const fn dimmed(self) -> Self`

  Make the text dim

- <span id="style-italic"></span>`const fn italic(self) -> Self`

  Make the text italicized

- <span id="style-underline"></span>`const fn underline(self) -> Self`

  Make the text underlined

- <span id="style-blink"></span>`const fn blink(self) -> Self`

  Make the text blink

- <span id="style-blink-fast"></span>`const fn blink_fast(self) -> Self`

  Make the text blink (but fast!)

- <span id="style-reversed"></span>`const fn reversed(self) -> Self`

  Swap the foreground and background colors

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

  Hide the text

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

  Cross out the text

- <span id="style-set-effect"></span>`const fn set_effect(self, effect: Effect, to: bool) -> Self` — [`Effect`](#effect)

- <span id="style-set-effects"></span>`const fn set_effects(self, effects: &[Effect], to: bool) -> Self` — [`Effect`](#effect)

- <span id="style-effect"></span>`const fn effect(self, effect: Effect) -> Self` — [`Effect`](#effect)

  Apply a given effect from the style

- <span id="style-remove-effect"></span>`const fn remove_effect(self, effect: Effect) -> Self` — [`Effect`](#effect)

  Remove a given effect from the style

- <span id="style-effects"></span>`const fn effects(self, effects: &[Effect]) -> Self` — [`Effect`](#effect)

  Apply a given set of effects to the style

- <span id="style-remove-effects"></span>`const fn remove_effects(self, effects: &[Effect]) -> Self` — [`Effect`](#effect)

  Remove a given set of effects from the style

- <span id="style-remove-all-effects"></span>`const fn remove_all_effects(self) -> Self`

  Disables all the given effects from the style

- <span id="style-color"></span>`fn color<Color: DynColor>(self, color: Color) -> Self`

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either [`OwoColorize::fg`](crate::OwoColorize::fg) or

  a color-specific method, such as [`OwoColorize::green`](crate::OwoColorize::green),

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

- <span id="style-on-color"></span>`fn on_color<Color: DynColor>(self, color: Color) -> Self`

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either [`OwoColorize::bg`](crate::OwoColorize::bg) or

  a color-specific method, such as [`OwoColorize::on_yellow`](crate::OwoColorize::on_yellow),

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="style-fg-rgb"></span>`const fn fg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

  Set the foreground color to a specific RGB value.

- <span id="style-bg-rgb"></span>`const fn bg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

  Set the background color to a specific RGB value.

- <span id="style-truecolor"></span>`const fn truecolor(self, r: u8, g: u8, b: u8) -> Self`

  Sets the foreground color to an RGB value.

- <span id="style-on-truecolor"></span>`const fn on_truecolor(self, r: u8, g: u8, b: u8) -> Self`

  Sets the background color to an RGB value.

- <span id="style-is-plain"></span>`const fn is_plain(&self) -> bool`

  Returns true if the style does not apply any formatting.

- <span id="style-prefix-formatter"></span>`const fn prefix_formatter(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](#styleprefixformatter)

  Returns a formatter for the style's ANSI prefix.

  

  This can be used to separate out the prefix and suffix of a style.

  

  # Example

  

  ```rust

  use owo_colors::Style;

  use std::fmt::Write;

  

  let style = Style::new().red().on_blue();

  let prefix = style.prefix_formatter();

  let suffix = style.suffix_formatter();

  

  // Write the prefix and suffix separately.

  let mut output = String::new();

  write!(output, "{}", prefix);

  output.push_str("Hello");

  write!(output, "{}", suffix);

  

  assert_eq!(output, "\x1b[31;44mHello\x1b[0m");

  ```

- <span id="style-suffix-formatter"></span>`const fn suffix_formatter(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](#stylesuffixformatter)

  Returns a formatter for the style's ANSI suffix.

  

  This can be used to separate out the prefix and suffix of a style.

  

  # Example

  

  See `Style::prefix_formatter`.

- <span id="style-fmt-prefix"></span>`fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Applies the ANSI-prefix for this style to the given formatter

- <span id="style-fmt-suffix"></span>`fn fmt_suffix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Applies the ANSI-suffix for this style to the given formatter

#### Trait Implementations

##### `impl Any for Style`

- <span id="style-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Style`

- <span id="style-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Style`

- <span id="style-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl CloneToUninit for Style`

- <span id="style-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl<T> From for Style`

- <span id="style-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Style`

- <span id="style-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Style`

##### `impl PartialEq for Style`

- <span id="style-partialeq-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

##### `impl<U> TryFrom for Style`

- <span id="style-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="style-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Style`

- <span id="style-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="style-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StyleFlags`

```rust
struct StyleFlags(u8);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:92`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L92)*

#### Implementations

- <span id="styleflags-is-plain"></span>`const fn is_plain(&self) -> bool`

#### Trait Implementations

##### `impl Any for StyleFlags`

- <span id="styleflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyleFlags`

- <span id="styleflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyleFlags`

- <span id="styleflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyleFlags`

- <span id="styleflags-clone"></span>`fn clone(&self) -> StyleFlags` — [`StyleFlags`](dyn_styles/index.md#styleflags)

##### `impl CloneToUninit for StyleFlags`

- <span id="styleflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StyleFlags`

##### `impl Debug for StyleFlags`

- <span id="styleflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleFlags`

- <span id="styleflags-default"></span>`fn default() -> Self`

##### `impl<T> From for StyleFlags`

- <span id="styleflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyleFlags`

- <span id="styleflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StyleFlags`

##### `impl PartialEq for StyleFlags`

- <span id="styleflags-partialeq-eq"></span>`fn eq(&self, other: &StyleFlags) -> bool` — [`StyleFlags`](dyn_styles/index.md#styleflags)

##### `impl StructuralPartialEq for StyleFlags`

##### `impl<U> TryFrom for StyleFlags`

- <span id="styleflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styleflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyleFlags`

- <span id="styleflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styleflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StylePrefixFormatter`

```rust
struct StylePrefixFormatter(Style);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:597`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L597)*

Formatter for the prefix of a [`Style`](#style).

This is used to get the ANSI escape codes for the style without
the suffix, which is useful for formatting the prefix separately.

#### Trait Implementations

##### `impl Any for StylePrefixFormatter`

- <span id="styleprefixformatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StylePrefixFormatter`

- <span id="styleprefixformatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StylePrefixFormatter`

- <span id="styleprefixformatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StylePrefixFormatter`

- <span id="styleprefixformatter-clone"></span>`fn clone(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](#styleprefixformatter)

##### `impl CloneToUninit for StylePrefixFormatter`

- <span id="styleprefixformatter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StylePrefixFormatter`

##### `impl Debug for StylePrefixFormatter`

- <span id="styleprefixformatter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StylePrefixFormatter`

- <span id="styleprefixformatter-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StylePrefixFormatter`

- <span id="styleprefixformatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StylePrefixFormatter`

- <span id="styleprefixformatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StylePrefixFormatter`

##### `impl PartialEq for StylePrefixFormatter`

- <span id="styleprefixformatter-partialeq-eq"></span>`fn eq(&self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](#styleprefixformatter)

##### `impl StructuralPartialEq for StylePrefixFormatter`

##### `impl<U> TryFrom for StylePrefixFormatter`

- <span id="styleprefixformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styleprefixformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StylePrefixFormatter`

- <span id="styleprefixformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styleprefixformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StyleSuffixFormatter`

```rust
struct StyleSuffixFormatter(Style);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:611`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L611)*

Formatter for the suffix of a [`Style`](#style).

This is used to get the ANSI escape codes for the style without
the prefix, which is useful for formatting the suffix separately.

#### Trait Implementations

##### `impl Any for StyleSuffixFormatter`

- <span id="stylesuffixformatter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyleSuffixFormatter`

- <span id="stylesuffixformatter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyleSuffixFormatter`

- <span id="stylesuffixformatter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyleSuffixFormatter`

- <span id="stylesuffixformatter-clone"></span>`fn clone(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](#stylesuffixformatter)

##### `impl CloneToUninit for StyleSuffixFormatter`

- <span id="stylesuffixformatter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StyleSuffixFormatter`

##### `impl Debug for StyleSuffixFormatter`

- <span id="stylesuffixformatter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StyleSuffixFormatter`

- <span id="stylesuffixformatter-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StyleSuffixFormatter`

- <span id="stylesuffixformatter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyleSuffixFormatter`

- <span id="stylesuffixformatter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StyleSuffixFormatter`

##### `impl PartialEq for StyleSuffixFormatter`

- <span id="stylesuffixformatter-partialeq-eq"></span>`fn eq(&self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](#stylesuffixformatter)

##### `impl StructuralPartialEq for StyleSuffixFormatter`

##### `impl<U> TryFrom for StyleSuffixFormatter`

- <span id="stylesuffixformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stylesuffixformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyleSuffixFormatter`

- <span id="stylesuffixformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stylesuffixformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../.source_1765633015/owo-colors-4.2.3/src/colors.rs#L108-L127)*

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for AnsiColors`

- <span id="ansicolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnsiColors`

- <span id="ansicolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnsiColors`

- <span id="ansicolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AnsiColors`

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](colors/ansi_colors/index.md#ansicolors)

##### `impl CloneToUninit for AnsiColors`

- <span id="ansicolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AnsiColors`

##### `impl Debug for AnsiColors`

- <span id="ansicolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for AnsiColors`

- <span id="ansicolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColors`

##### `impl<T> From for AnsiColors`

- <span id="ansicolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AnsiColors`

- <span id="ansicolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for AnsiColors`

##### `impl PartialEq for AnsiColors`

- <span id="ansicolors-partialeq-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](colors/ansi_colors/index.md#ansicolors)

##### `impl StructuralPartialEq for AnsiColors`

##### `impl<U> TryFrom for AnsiColors`

- <span id="ansicolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansicolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnsiColors`

- <span id="ansicolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansicolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`owo-colors-4.2.3/src/colors/css.rs:84-232`](../../.source_1765633015/owo-colors-4.2.3/src/colors/css.rs#L84-L232)*

Available CSS colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for CssColors`

- <span id="csscolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CssColors`

- <span id="csscolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CssColors`

- <span id="csscolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CssColors`

- <span id="csscolors-clone"></span>`fn clone(&self) -> CssColors` — [`CssColors`](colors/css/dynamic/index.md#csscolors)

##### `impl CloneToUninit for CssColors`

- <span id="csscolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CssColors`

##### `impl Debug for CssColors`

- <span id="csscolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for CssColors`

- <span id="csscolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="csscolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CssColors`

##### `impl<T> From for CssColors`

- <span id="csscolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CssColors`

- <span id="csscolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CssColors`

##### `impl PartialEq for CssColors`

- <span id="csscolors-partialeq-eq"></span>`fn eq(&self, other: &CssColors) -> bool` — [`CssColors`](colors/css/dynamic/index.md#csscolors)

##### `impl StructuralPartialEq for CssColors`

##### `impl<U> TryFrom for CssColors`

- <span id="csscolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="csscolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CssColors`

- <span id="csscolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="csscolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`owo-colors-4.2.3/src/colors/xterm.rs:123-380`](../../.source_1765633015/owo-colors-4.2.3/src/colors/xterm.rs#L123-L380)*

Available Xterm colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Any for XtermColors`

- <span id="xtermcolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for XtermColors`

- <span id="xtermcolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for XtermColors`

- <span id="xtermcolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for XtermColors`

- <span id="xtermcolors-clone"></span>`fn clone(&self) -> XtermColors` — [`XtermColors`](colors/xterm/dynamic/index.md#xtermcolors)

##### `impl CloneToUninit for XtermColors`

- <span id="xtermcolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for XtermColors`

##### `impl Debug for XtermColors`

- <span id="xtermcolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for XtermColors`

- <span id="xtermcolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="xtermcolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for XtermColors`

##### `impl<T> From for XtermColors`

- <span id="xtermcolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for XtermColors`

- <span id="xtermcolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for XtermColors`

##### `impl PartialEq for XtermColors`

- <span id="xtermcolors-partialeq-eq"></span>`fn eq(&self, other: &XtermColors) -> bool` — [`XtermColors`](colors/xterm/dynamic/index.md#xtermcolors)

##### `impl StructuralPartialEq for XtermColors`

##### `impl<U> TryFrom for XtermColors`

- <span id="xtermcolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="xtermcolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for XtermColors`

- <span id="xtermcolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="xtermcolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DynColors`

```rust
enum DynColors {
    Ansi(crate::AnsiColors),
    Css(crate::CssColors),
    Xterm(crate::XtermColors),
    Rgb(u8, u8, u8),
}
```

*Defined in [`owo-colors-4.2.3/src/dyn_colors.rs:13-18`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_colors.rs#L13-L18)*

An enum describing runtime-configurable colors

This can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay) or [`BgDynColorDisplay`](BgDynColorDisplay),
allowing for multiple types of colors to be used at runtime.

#### Trait Implementations

##### `impl Any for DynColors`

- <span id="dyncolors-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DynColors`

- <span id="dyncolors-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DynColors`

- <span id="dyncolors-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DynColors`

- <span id="dyncolors-clone"></span>`fn clone(&self) -> DynColors` — [`DynColors`](#dyncolors)

##### `impl CloneToUninit for DynColors`

- <span id="dyncolors-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DynColors`

##### `impl Debug for DynColors`

- <span id="dyncolors-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for DynColors`

- <span id="dyncolors-dyncolor-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-dyncolor-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DynColors`

##### `impl<T> From for DynColors`

- <span id="dyncolors-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for DynColors`

- <span id="dyncolors-fromstr-type-err"></span>`type Err = ParseColorError`

- <span id="dyncolors-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<U> Into for DynColors`

- <span id="dyncolors-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DynColors`

##### `impl PartialEq for DynColors`

- <span id="dyncolors-partialeq-eq"></span>`fn eq(&self, other: &DynColors) -> bool` — [`DynColors`](#dyncolors)

##### `impl StructuralPartialEq for DynColors`

##### `impl<U> TryFrom for DynColors`

- <span id="dyncolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyncolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DynColors`

- <span id="dyncolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyncolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:10-20`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L10-L20)*

A runtime-configurable text effect for use with [`Style`](#style)

#### Trait Implementations

##### `impl Any for Effect`

- <span id="effect-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Effect`

- <span id="effect-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Effect`

- <span id="effect-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Effect`

- <span id="effect-clone"></span>`fn clone(&self) -> Effect` — [`Effect`](#effect)

##### `impl CloneToUninit for Effect`

- <span id="effect-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Effect`

##### `impl Debug for Effect`

- <span id="effect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Effect`

- <span id="effect-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Effect`

- <span id="effect-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Effect`

##### `impl<U> TryFrom for Effect`

- <span id="effect-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="effect-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Effect`

- <span id="effect-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="effect-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Color`

```rust
trait Color: private::Sealed { ... }
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:102-125`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L102-L125)*

A trait for describing a type which can be used with [`FgColorDisplay`](#fgcolordisplay) or
[`BgColorDisplay`](#bgcolordisplay)

#### Associated Constants

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

#### Implementors

- [`AeroBlue`](colors/xterm/index.md#aeroblue)
- [`AltoBeige`](colors/xterm/index.md#altobeige)
- [`Alto`](colors/xterm/index.md#alto)
- [`AnakiwaBlue`](colors/xterm/index.md#anakiwablue)
- [`Aqua`](colors/xterm/index.md#aqua)
- [`Aquamarine`](colors/xterm/index.md#aquamarine)
- [`AzureRadiance`](colors/xterm/index.md#azureradiance)
- [`BayLeaf`](colors/xterm/index.md#bayleaf)
- [`Bermuda`](colors/xterm/index.md#bermuda)
- [`BittersweetOrange`](colors/xterm/index.md#bittersweetorange)
- [`Black`](colors/index.md#black)
- [`Black`](colors/xterm/index.md#black)
- [`BlazeOrange`](colors/xterm/index.md#blazeorange)
- [`BlueRibbon`](colors/xterm/index.md#blueribbon)
- [`BlueStone`](colors/xterm/index.md#bluestone)
- [`Blue`](colors/index.md#blue)
- [`Blue`](colors/xterm/index.md#blue)
- [`BlushPink`](colors/xterm/index.md#blushpink)
- [`BondiBlue`](colors/xterm/index.md#bondiblue)
- [`Boulder`](colors/xterm/index.md#boulder)
- [`Bouquet`](colors/xterm/index.md#bouquet)
- [`BrightBlack`](colors/index.md#brightblack)
- [`BrightBlue`](colors/index.md#brightblue)
- [`BrightCyan`](colors/index.md#brightcyan)
- [`BrightElectricViolet`](colors/xterm/index.md#brightelectricviolet)
- [`BrightGreen`](colors/index.md#brightgreen)
- [`BrightGreen`](colors/xterm/index.md#brightgreen)
- [`BrightHeliotrope`](colors/xterm/index.md#brightheliotrope)
- [`BrightMagenta`](colors/index.md#brightmagenta)
- [`BrightRed`](colors/index.md#brightred)
- [`BrightRed`](colors/xterm/index.md#brightred)
- [`BrightTurquoise`](colors/xterm/index.md#brightturquoise)
- [`BrightWhite`](colors/index.md#brightwhite)
- [`BrightYellow`](colors/index.md#brightyellow)
- [`BrighterElectricViolet`](colors/xterm/index.md#brighterelectricviolet)
- [`Brown`](colors/xterm/index.md#brown)
- [`BuddhaGold`](colors/xterm/index.md#buddhagold)
- [`CamaroneGreen`](colors/xterm/index.md#camaronegreen)
- [`CanCanPink`](colors/xterm/index.md#cancanpink)
- [`Canary`](colors/xterm/index.md#canary)
- [`Caramel`](colors/xterm/index.md#caramel)
- [`CaribbeanGreen`](colors/xterm/index.md#caribbeangreen)
- [`Celadon`](colors/xterm/index.md#celadon)
- [`Cerulean`](colors/xterm/index.md#cerulean)
- [`ChartreuseGreen`](colors/xterm/index.md#chartreusegreen)
- [`ChartreuseYellow`](colors/xterm/index.md#chartreuseyellow)
- [`ChelseaCucumber`](colors/xterm/index.md#chelseacucumber)
- [`ChetwodeBlue`](colors/xterm/index.md#chetwodeblue)
- [`ClamShell`](colors/xterm/index.md#clamshell)
- [`ClayCreekOlive`](colors/xterm/index.md#claycreekolive)
- [`CodGray`](colors/xterm/index.md#codgray)
- [`ConiferGreen`](colors/xterm/index.md#conifergreen)
- [`CopperRose`](colors/xterm/index.md#copperrose)
- [`Copperfield`](colors/xterm/index.md#copperfield)
- [`Corn`](colors/xterm/index.md#corn)
- [`CornflowerBlue`](colors/xterm/index.md#cornflowerblue)
- [`CosmosSalmon`](colors/xterm/index.md#cosmossalmon)
- [`CottonCandy`](colors/xterm/index.md#cottoncandy)
- [`CranberryPink`](colors/xterm/index.md#cranberrypink)
- [`Cumulus`](colors/xterm/index.md#cumulus)
- [`CustomColor`](colors/custom/index.md#customcolor)
- [`Cyan`](colors/index.md#cyan)
- [`Cyan`](colors/xterm/index.md#cyan)
- [`Dandelion`](colors/xterm/index.md#dandelion)
- [`DarkAlto`](colors/xterm/index.md#darkalto)
- [`DarkAnakiwaBlue`](colors/xterm/index.md#darkanakiwablue)
- [`DarkAquamarine`](colors/xterm/index.md#darkaquamarine)
- [`DarkBlue`](colors/xterm/index.md#darkblue)
- [`DarkBrightGreen`](colors/xterm/index.md#darkbrightgreen)
- [`DarkCodGray`](colors/xterm/index.md#darkcodgray)
- [`DarkCorn`](colors/xterm/index.md#darkcorn)
- [`DarkCornflowerBlue`](colors/xterm/index.md#darkcornflowerblue)
- [`DarkDoveGray`](colors/xterm/index.md#darkdovegray)
- [`DarkFeijoaGreen`](colors/xterm/index.md#darkfeijoagreen)
- [`DarkFlirt`](colors/xterm/index.md#darkflirt)
- [`DarkFreshEggplant`](colors/xterm/index.md#darkfresheggplant)
- [`DarkGray`](colors/xterm/index.md#darkgray)
- [`DarkGreen`](colors/xterm/index.md#darkgreen)
- [`DarkHeliotropePurple`](colors/xterm/index.md#darkheliotropepurple)
- [`DarkHotPink`](colors/xterm/index.md#darkhotpink)
- [`DarkLavenderRose`](colors/xterm/index.md#darklavenderrose)
- [`DarkLimeade`](colors/xterm/index.md#darklimeade)
- [`DarkMalibuBlue`](colors/xterm/index.md#darkmalibublue)
- [`DarkMediumPurple`](colors/xterm/index.md#darkmediumpurple)
- [`DarkMineShaft`](colors/xterm/index.md#darkmineshaft)
- [`DarkMintGreen`](colors/xterm/index.md#darkmintgreen)
- [`DarkPastelGreen`](colors/xterm/index.md#darkpastelgreen)
- [`DarkPurplePizzazz`](colors/xterm/index.md#darkpurplepizzazz)
- [`DarkPurple`](colors/xterm/index.md#darkpurple)
- [`DarkRose`](colors/xterm/index.md#darkrose)
- [`DarkScreaminGreen`](colors/xterm/index.md#darkscreamingreen)
- [`DarkSilverChalice`](colors/xterm/index.md#darksilverchalice)
- [`DarkSilver`](colors/xterm/index.md#darksilver)
- [`DarkSpringGreen`](colors/xterm/index.md#darkspringgreen)
- [`DarkTachaOrange`](colors/xterm/index.md#darktachaorange)
- [`DarkTundora`](colors/xterm/index.md#darktundora)
- [`DarkViolet`](colors/xterm/index.md#darkviolet)
- [`DecoOrange`](colors/xterm/index.md#decoorange)
- [`DeepCerulean`](colors/xterm/index.md#deepcerulean)
- [`DeepSeaGreen`](colors/xterm/index.md#deepseagreen)
- [`Default`](colors/index.md#default)
- [`DelugePurple`](colors/xterm/index.md#delugepurple)
- [`DollyYellow`](colors/xterm/index.md#dollyyellow)
- [`DoveGray`](colors/xterm/index.md#dovegray)
- [`DownyTeal`](colors/xterm/index.md#downyteal)
- [`DustyGray`](colors/xterm/index.md#dustygray)
- [`ElectricIndigo`](colors/xterm/index.md#electricindigo)
- [`ElectricPurple`](colors/xterm/index.md#electricpurple)
- [`ElectricViolet`](colors/xterm/index.md#electricviolet)
- [`EndeavourBlue`](colors/xterm/index.md#endeavourblue)
- [`Feijoa`](colors/xterm/index.md#feijoa)
- [`FernGreen`](colors/xterm/index.md#ferngreen)
- [`Flirt`](colors/xterm/index.md#flirt)
- [`FlushOrange`](colors/xterm/index.md#flushorange)
- [`FogPink`](colors/xterm/index.md#fogpink)
- [`FrenchPassLightBlue`](colors/xterm/index.md#frenchpasslightblue)
- [`FuchsiaPink`](colors/xterm/index.md#fuchsiapink)
- [`Fuchsia`](colors/xterm/index.md#fuchsia)
- [`GalleryGray`](colors/xterm/index.md#gallerygray)
- [`GladeGreen`](colors/xterm/index.md#gladegreen)
- [`Gold`](colors/xterm/index.md#gold)
- [`GrandisCaramel`](colors/xterm/index.md#grandiscaramel)
- [`Gray`](colors/xterm/index.md#gray)
- [`GreenYellow`](colors/xterm/index.md#greenyellow)
- [`Green`](colors/index.md#green)
- [`Green`](colors/xterm/index.md#green)
- [`GuardsmanRed`](colors/xterm/index.md#guardsmanred)
- [`GulfStream`](colors/xterm/index.md#gulfstream)
- [`HavelockBlue`](colors/xterm/index.md#havelockblue)
- [`Heliotrope`](colors/xterm/index.md#heliotrope)
- [`HillaryOlive`](colors/xterm/index.md#hillaryolive)
- [`HippieBlue`](colors/xterm/index.md#hippieblue)
- [`HollywoodCerise`](colors/xterm/index.md#hollywoodcerise)
- [`Honeysuckle`](colors/xterm/index.md#honeysuckle)
- [`HopbushPink`](colors/xterm/index.md#hopbushpink)
- [`HotPink`](colors/xterm/index.md#hotpink)
- [`Indigo`](colors/xterm/index.md#indigo)
- [`Jade`](colors/xterm/index.md#jade)
- [`JapaneseLaurel`](colors/xterm/index.md#japaneselaurel)
- [`JungleMist`](colors/xterm/index.md#junglemist)
- [`JuniperGreen`](colors/xterm/index.md#junipergreen)
- [`LaserLemon`](colors/xterm/index.md#laserlemon)
- [`LavenderRose`](colors/xterm/index.md#lavenderrose)
- [`Lavender`](colors/xterm/index.md#lavender)
- [`LightAnakiwaBlue`](colors/xterm/index.md#lightanakiwablue)
- [`LightAquamarine`](colors/xterm/index.md#lightaquamarine)
- [`LightAzureRadiance`](colors/xterm/index.md#lightazureradiance)
- [`LightCaribbeanGreen`](colors/xterm/index.md#lightcaribbeangreen)
- [`LightCodGray`](colors/xterm/index.md#lightcodgray)
- [`LightElectricViolet`](colors/xterm/index.md#lightelectricviolet)
- [`LightFlirt`](colors/xterm/index.md#lightflirt)
- [`LightFreshEggplant`](colors/xterm/index.md#lightfresheggplant)
- [`LightGray`](colors/xterm/index.md#lightgray)
- [`LightHeliotrope`](colors/xterm/index.md#lightheliotrope)
- [`LightHollywoodCerise`](colors/xterm/index.md#lighthollywoodcerise)
- [`LightJapaneseLaurel`](colors/xterm/index.md#lightjapaneselaurel)
- [`LightLimeade`](colors/xterm/index.md#lightlimeade)
- [`LightMalibuBlue`](colors/xterm/index.md#lightmalibublue)
- [`LightMineShaft`](colors/xterm/index.md#lightmineshaft)
- [`LightMintGreen`](colors/xterm/index.md#lightmintgreen)
- [`LightOrchid`](colors/xterm/index.md#lightorchid)
- [`LightPastelGreen`](colors/xterm/index.md#lightpastelgreen)
- [`LightScreaminGreen`](colors/xterm/index.md#lightscreamingreen)
- [`LightSilverChalice`](colors/xterm/index.md#lightsilverchalice)
- [`LightSpringGreen`](colors/xterm/index.md#lightspringgreen)
- [`LighterAquamarine`](colors/xterm/index.md#lighteraquamarine)
- [`LighterHeliotrope`](colors/xterm/index.md#lighterheliotrope)
- [`Lilac`](colors/xterm/index.md#lilac)
- [`Lime`](colors/xterm/index.md#lime)
- [`Limeade`](colors/xterm/index.md#limeade)
- [`LochmaraBlue`](colors/xterm/index.md#lochmarablue)
- [`Magenta`](colors/index.md#magenta)
- [`Malachite`](colors/xterm/index.md#malachite)
- [`MalibuBlue`](colors/xterm/index.md#malibublue)
- [`MangoTango`](colors/xterm/index.md#mangotango)
- [`Maroon`](colors/xterm/index.md#maroon)
- [`MatrixPink`](colors/xterm/index.md#matrixpink)
- [`Mauve`](colors/xterm/index.md#mauve)
- [`MediumPurple`](colors/xterm/index.md#mediumpurple)
- [`MediumVioletRed`](colors/xterm/index.md#mediumvioletred)
- [`MelroseLilac`](colors/xterm/index.md#melroselilac)
- [`Mercury`](colors/xterm/index.md#mercury)
- [`MidnightBlue`](colors/xterm/index.md#midnightblue)
- [`MineShaft`](colors/xterm/index.md#mineshaft)
- [`MintGreen`](colors/xterm/index.md#mintgreen)
- [`MuesliOrange`](colors/xterm/index.md#muesliorange)
- [`NavyBlue`](colors/xterm/index.md#navyblue)
- [`NobelGray`](colors/xterm/index.md#nobelgray)
- [`OliveGreen`](colors/xterm/index.md#olivegreen)
- [`Olive`](colors/xterm/index.md#olive)
- [`Orchid`](colors/xterm/index.md#orchid)
- [`OrientBlue`](colors/xterm/index.md#orientblue)
- [`OysterBay`](colors/xterm/index.md#oysterbay)
- [`PaleGoldenrod`](colors/xterm/index.md#palegoldenrod)
- [`PastelGreen`](colors/xterm/index.md#pastelgreen)
- [`PersianGreen`](colors/xterm/index.md#persiangreen)
- [`PharlapPink`](colors/xterm/index.md#pharlappink)
- [`PigmentIndigo`](colors/xterm/index.md#pigmentindigo)
- [`PinkFlamingo`](colors/xterm/index.md#pinkflamingo)
- [`PinkLace`](colors/xterm/index.md#pinklace)
- [`PinkSalmon`](colors/xterm/index.md#pinksalmon)
- [`PirateGold`](colors/xterm/index.md#pirategold)
- [`Pistachio`](colors/xterm/index.md#pistachio)
- [`PixieGreen`](colors/xterm/index.md#pixiegreen)
- [`PoloBlue`](colors/xterm/index.md#poloblue)
- [`PompadourMagenta`](colors/xterm/index.md#pompadourmagenta)
- [`PortafinoYellow`](colors/xterm/index.md#portafinoyellow)
- [`PurplePizzazz`](colors/xterm/index.md#purplepizzazz)
- [`Purple`](colors/xterm/index.md#purple)
- [`RazzmatazzCerise`](colors/xterm/index.md#razzmatazzcerise)
- [`Red`](colors/index.md#red)
- [`Red`](colors/xterm/index.md#red)
- [`ReefPaleYellow`](colors/xterm/index.md#reefpaleyellow)
- [`RioGrandeGreen`](colors/xterm/index.md#riograndegreen)
- [`RobinEggBlue`](colors/xterm/index.md#robineggblue)
- [`RomanOrange`](colors/xterm/index.md#romanorange)
- [`Rose`](colors/xterm/index.md#rose)
- [`RoseofSharonOrange`](colors/xterm/index.md#roseofsharonorange)
- [`Rosewood`](colors/xterm/index.md#rosewood)
- [`Salmon`](colors/xterm/index.md#salmon)
- [`ScampiIndigo`](colors/xterm/index.md#scampiindigo)
- [`ScienceBlue`](colors/xterm/index.md#scienceblue)
- [`ScorpionGray`](colors/xterm/index.md#scorpiongray)
- [`ScorpionOlive`](colors/xterm/index.md#scorpionolive)
- [`ScreaminGreen`](colors/xterm/index.md#screamingreen)
- [`SeaPink`](colors/xterm/index.md#seapink)
- [`ShakespeareBlue`](colors/xterm/index.md#shakespeareblue)
- [`SilverChalice`](colors/xterm/index.md#silverchalice)
- [`SilverTree`](colors/xterm/index.md#silvertree)
- [`Silver`](colors/xterm/index.md#silver)
- [`SlateBlue`](colors/xterm/index.md#slateblue)
- [`SnowyMint`](colors/xterm/index.md#snowymint)
- [`SpringGreen`](colors/xterm/index.md#springgreen)
- [`StratosBlue`](colors/xterm/index.md#stratosblue)
- [`StrikemasterPurple`](colors/xterm/index.md#strikemasterpurple)
- [`Sundown`](colors/xterm/index.md#sundown)
- [`Tacao`](colors/xterm/index.md#tacao)
- [`TachaOrange`](colors/xterm/index.md#tachaorange)
- [`TanBeige`](colors/xterm/index.md#tanbeige)
- [`TapestryPink`](colors/xterm/index.md#tapestrypink)
- [`Teal`](colors/xterm/index.md#teal)
- [`TennOrange`](colors/xterm/index.md#tennorange)
- [`TexasRose`](colors/xterm/index.md#texasrose)
- [`ThistlePink`](colors/xterm/index.md#thistlepink)
- [`Tradewind`](colors/xterm/index.md#tradewind)
- [`Tundora`](colors/xterm/index.md#tundora)
- [`UserBlack`](colors/xterm/index.md#userblack)
- [`UserBlue`](colors/xterm/index.md#userblue)
- [`UserBrightBlack`](colors/xterm/index.md#userbrightblack)
- [`UserBrightBlue`](colors/xterm/index.md#userbrightblue)
- [`UserBrightCyan`](colors/xterm/index.md#userbrightcyan)
- [`UserBrightGreen`](colors/xterm/index.md#userbrightgreen)
- [`UserBrightMagenta`](colors/xterm/index.md#userbrightmagenta)
- [`UserBrightRed`](colors/xterm/index.md#userbrightred)
- [`UserBrightWhite`](colors/xterm/index.md#userbrightwhite)
- [`UserBrightYellow`](colors/xterm/index.md#userbrightyellow)
- [`UserCyan`](colors/xterm/index.md#usercyan)
- [`UserGreen`](colors/xterm/index.md#usergreen)
- [`UserMagenta`](colors/xterm/index.md#usermagenta)
- [`UserRed`](colors/xterm/index.md#userred)
- [`UserWhite`](colors/xterm/index.md#userwhite)
- [`UserYellow`](colors/xterm/index.md#useryellow)
- [`VerdunGreen`](colors/xterm/index.md#verdungreen)
- [`Viking`](colors/xterm/index.md#viking)
- [`VistaBlue`](colors/xterm/index.md#vistablue)
- [`VividTangerine`](colors/xterm/index.md#vividtangerine)
- [`White`](colors/index.md#white)
- [`White`](colors/xterm/index.md#white)
- [`WildBlueYonder`](colors/xterm/index.md#wildblueyonder)
- [`WildWatermelon`](colors/xterm/index.md#wildwatermelon)
- [`WistfulLilac`](colors/xterm/index.md#wistfullilac)
- [`YellowSea`](colors/xterm/index.md#yellowsea)
- [`Yellow`](colors/index.md#yellow)
- [`Yellow`](colors/xterm/index.md#yellow)

### `DynColor`

```rust
trait DynColor: private::Sealed { ... }
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:130-148`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L130-L148)*

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

#### Implementors

- [`AnsiColors`](colors/ansi_colors/index.md#ansicolors)
- [`CssColors`](colors/css/dynamic/index.md#csscolors)
- [`DynColors`](#dyncolors)
- [`Rgb`](colors/dynamic/index.md#rgb)
- [`XtermColors`](colors/xterm/dynamic/index.md#xtermcolors)
- `str`

### `OwoColorize`

```rust
trait OwoColorize: Sized { ... }
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:263-489`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L263-L489)*

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


#### Provided Methods

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

#### Implementors

- `D`

## Functions

### `style`

```rust
const fn style() -> Style
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:626-628`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L626-L628)*

Helper to create [`Style`](#style)s more ergonomically

## Constants

### `DIMMED_SHIFT`
```rust
const DIMMED_SHIFT: u8 = 0u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:102`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L102)*

### `ITALIC_SHIFT`
```rust
const ITALIC_SHIFT: u8 = 1u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:103`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L103)*

### `UNDERLINE_SHIFT`
```rust
const UNDERLINE_SHIFT: u8 = 2u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:104`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L104)*

### `BLINK_SHIFT`
```rust
const BLINK_SHIFT: u8 = 3u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:105`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L105)*

### `BLINK_FAST_SHIFT`
```rust
const BLINK_FAST_SHIFT: u8 = 4u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:106`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L106)*

### `REVERSED_SHIFT`
```rust
const REVERSED_SHIFT: u8 = 5u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:107`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L107)*

### `HIDDEN_SHIFT`
```rust
const HIDDEN_SHIFT: u8 = 6u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:108`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L108)*

### `STRIKETHROUGH_SHIFT`
```rust
const STRIKETHROUGH_SHIFT: u8 = 7u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:109`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L109)*

## Macros

### `style_methods!`

*Defined in [`owo-colors-4.2.3/src/lib.rs:172-183`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L172-L183)*

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/lib.rs:187-207`](../../.source_1765633015/owo-colors-4.2.3/src/lib.rs#L187-L207)*

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:22-42`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L22-L42)*

### `style_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:44-55`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L44-L55)*

### `style_flags_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:111-126`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L111-L126)*

### `impl_fmt!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:653-666`](../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L653-L666)*

