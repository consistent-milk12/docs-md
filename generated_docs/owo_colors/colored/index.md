*[owo_colors](../index.md) / [colored](index.md)*

---

# Module `colored`

Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from
[`colored`](https://docs.rs/colored) to owo-colors.

Just replace:

```rust
mod colored {}
use colored::*;
```

with

```rust
use owo_colors::colored::*;
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Color`](#color) | enum |  |
| [`OwoColorize`](#owocolorize) | trait |  |

## Enums

### `Color`

```rust
enum Color {
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

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765900590/owo-colors-4.2.3/src/colors.rs#L108-L127)*

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

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](../colors/ansi_colors/index.md#ansicolors)

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

- <span id="ansicolors-partialeq-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](../colors/ansi_colors/index.md#ansicolors)

##### `impl StructuralPartialEq for AnsiColors`

##### `impl<U> TryFrom for AnsiColors`

- <span id="ansicolors-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ansicolors-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnsiColors`

- <span id="ansicolors-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ansicolors-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `OwoColorize`

```rust
trait OwoColorize: Sized { ... }
```

*Defined in [`owo-colors-4.2.3/src/lib.rs:263-489`](../../../.source_1765900590/owo-colors-4.2.3/src/lib.rs#L263-L489)*

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

Use [`style`](OwoColorize::style) to apply a [`Style`](../index.md)


<details>
<summary><strong>Methods (56)</strong> - click to expand</summary>

**Provided:**
- [`OwoColorize::fg`](#fn-owocolorizefg)
- [`OwoColorize::bg`](#fn-owocolorizebg)
- [`OwoColorize::black`](#fn-owocolorizeblack)
- [`OwoColorize::on_black`](#fn-owocolorizeon-black)
- [`OwoColorize::red`](#fn-owocolorizered)
- [`OwoColorize::on_red`](#fn-owocolorizeon-red)
- [`OwoColorize::green`](#fn-owocolorizegreen)
- [`OwoColorize::on_green`](#fn-owocolorizeon-green)
- [`OwoColorize::yellow`](#fn-owocolorizeyellow)
- [`OwoColorize::on_yellow`](#fn-owocolorizeon-yellow)
- [`OwoColorize::blue`](#fn-owocolorizeblue)
- [`OwoColorize::on_blue`](#fn-owocolorizeon-blue)
- [`OwoColorize::magenta`](#fn-owocolorizemagenta)
- [`OwoColorize::on_magenta`](#fn-owocolorizeon-magenta)
- [`OwoColorize::purple`](#fn-owocolorizepurple)
- [`OwoColorize::on_purple`](#fn-owocolorizeon-purple)
- [`OwoColorize::cyan`](#fn-owocolorizecyan)
- [`OwoColorize::on_cyan`](#fn-owocolorizeon-cyan)
- [`OwoColorize::white`](#fn-owocolorizewhite)
- [`OwoColorize::on_white`](#fn-owocolorizeon-white)
- [`OwoColorize::default_color`](#fn-owocolorizedefault-color)
- [`OwoColorize::on_default_color`](#fn-owocolorizeon-default-color)
- [`OwoColorize::bright_black`](#fn-owocolorizebright-black)
- [`OwoColorize::on_bright_black`](#fn-owocolorizeon-bright-black)
- [`OwoColorize::bright_red`](#fn-owocolorizebright-red)
- [`OwoColorize::on_bright_red`](#fn-owocolorizeon-bright-red)
- [`OwoColorize::bright_green`](#fn-owocolorizebright-green)
- [`OwoColorize::on_bright_green`](#fn-owocolorizeon-bright-green)
- [`OwoColorize::bright_yellow`](#fn-owocolorizebright-yellow)
- [`OwoColorize::on_bright_yellow`](#fn-owocolorizeon-bright-yellow)
- [`OwoColorize::bright_blue`](#fn-owocolorizebright-blue)
- [`OwoColorize::on_bright_blue`](#fn-owocolorizeon-bright-blue)
- [`OwoColorize::bright_magenta`](#fn-owocolorizebright-magenta)
- [`OwoColorize::on_bright_magenta`](#fn-owocolorizeon-bright-magenta)
- [`OwoColorize::bright_purple`](#fn-owocolorizebright-purple)
- [`OwoColorize::on_bright_purple`](#fn-owocolorizeon-bright-purple)
- [`OwoColorize::bright_cyan`](#fn-owocolorizebright-cyan)
- [`OwoColorize::on_bright_cyan`](#fn-owocolorizeon-bright-cyan)
- [`OwoColorize::bright_white`](#fn-owocolorizebright-white)
- [`OwoColorize::on_bright_white`](#fn-owocolorizeon-bright-white)
- [`OwoColorize::bold`](#fn-owocolorizebold)
- [`OwoColorize::dimmed`](#fn-owocolorizedimmed)
- [`OwoColorize::italic`](#fn-owocolorizeitalic)
- [`OwoColorize::underline`](#fn-owocolorizeunderline)
- [`OwoColorize::blink`](#fn-owocolorizeblink)
- [`OwoColorize::blink_fast`](#fn-owocolorizeblink-fast)
- [`OwoColorize::reversed`](#fn-owocolorizereversed)
- [`OwoColorize::hidden`](#fn-owocolorizehidden)
- [`OwoColorize::strikethrough`](#fn-owocolorizestrikethrough)
- [`OwoColorize::color`](#fn-owocolorizecolor)
- [`OwoColorize::on_color`](#fn-owocolorizeon-color)
- [`OwoColorize::fg_rgb`](#fn-owocolorizefg-rgb)
- [`OwoColorize::bg_rgb`](#fn-owocolorizebg-rgb)
- [`OwoColorize::truecolor`](#fn-owocolorizetruecolor)
- [`OwoColorize::on_truecolor`](#fn-owocolorizeon-truecolor)
- [`OwoColorize::style`](#fn-owocolorizestyle)

</details>

#### Provided Methods

- `fn OwoColorize::fg<C: Color>(&self) -> FgColorDisplay<'_, C, Self>`

  Set the foreground color generically
  
  ```rust
  use owo_colors::{OwoColorize, colors::*};
  
  println!("{}", "red foreground".fg::<Red>());
  ```

- `fn OwoColorize::bg<C: Color>(&self) -> BgColorDisplay<'_, C, Self>`

  Set the background color generically.
  
  ```rust
  use owo_colors::{OwoColorize, colors::*};
  
  println!("{}", "black background".bg::<Black>());
  ```

- `fn OwoColorize::black(&self) -> FgColorDisplay<'_, colors::Black, Self>`

  Change the foreground color to black

- `fn OwoColorize::on_black(&self) -> BgColorDisplay<'_, colors::Black, Self>`

  Change the background color to black

- `fn OwoColorize::red(&self) -> FgColorDisplay<'_, colors::Red, Self>`

  Change the foreground color to red

- `fn OwoColorize::on_red(&self) -> BgColorDisplay<'_, colors::Red, Self>`

  Change the background color to red

- `fn OwoColorize::green(&self) -> FgColorDisplay<'_, colors::Green, Self>`

  Change the foreground color to green

- `fn OwoColorize::on_green(&self) -> BgColorDisplay<'_, colors::Green, Self>`

  Change the background color to green

- `fn OwoColorize::yellow(&self) -> FgColorDisplay<'_, colors::Yellow, Self>`

  Change the foreground color to yellow

- `fn OwoColorize::on_yellow(&self) -> BgColorDisplay<'_, colors::Yellow, Self>`

  Change the background color to yellow

- `fn OwoColorize::blue(&self) -> FgColorDisplay<'_, colors::Blue, Self>`

  Change the foreground color to blue

- `fn OwoColorize::on_blue(&self) -> BgColorDisplay<'_, colors::Blue, Self>`

  Change the background color to blue

- `fn OwoColorize::magenta(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to magenta

- `fn OwoColorize::on_magenta(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to magenta

- `fn OwoColorize::purple(&self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to purple

- `fn OwoColorize::on_purple(&self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to purple

- `fn OwoColorize::cyan(&self) -> FgColorDisplay<'_, colors::Cyan, Self>`

  Change the foreground color to cyan

- `fn OwoColorize::on_cyan(&self) -> BgColorDisplay<'_, colors::Cyan, Self>`

  Change the background color to cyan

- `fn OwoColorize::white(&self) -> FgColorDisplay<'_, colors::White, Self>`

  Change the foreground color to white

- `fn OwoColorize::on_white(&self) -> BgColorDisplay<'_, colors::White, Self>`

  Change the background color to white

- `fn OwoColorize::default_color(&self) -> FgColorDisplay<'_, colors::Default, Self>`

  Change the foreground color to the terminal default

- `fn OwoColorize::on_default_color(&self) -> BgColorDisplay<'_, colors::Default, Self>`

  Change the background color to the terminal default

- `fn OwoColorize::bright_black(&self) -> FgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the foreground color to bright black

- `fn OwoColorize::on_bright_black(&self) -> BgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the background color to bright black

- `fn OwoColorize::bright_red(&self) -> FgColorDisplay<'_, colors::BrightRed, Self>`

  Change the foreground color to bright red

- `fn OwoColorize::on_bright_red(&self) -> BgColorDisplay<'_, colors::BrightRed, Self>`

  Change the background color to bright red

- `fn OwoColorize::bright_green(&self) -> FgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the foreground color to bright green

- `fn OwoColorize::on_bright_green(&self) -> BgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the background color to bright green

- `fn OwoColorize::bright_yellow(&self) -> FgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the foreground color to bright yellow

- `fn OwoColorize::on_bright_yellow(&self) -> BgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the background color to bright yellow

- `fn OwoColorize::bright_blue(&self) -> FgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the foreground color to bright blue

- `fn OwoColorize::on_bright_blue(&self) -> BgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the background color to bright blue

- `fn OwoColorize::bright_magenta(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright magenta

- `fn OwoColorize::on_bright_magenta(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright magenta

- `fn OwoColorize::bright_purple(&self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright purple

- `fn OwoColorize::on_bright_purple(&self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright purple

- `fn OwoColorize::bright_cyan(&self) -> FgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the foreground color to bright cyan

- `fn OwoColorize::on_bright_cyan(&self) -> BgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the background color to bright cyan

- `fn OwoColorize::bright_white(&self) -> FgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the foreground color to bright white

- `fn OwoColorize::on_bright_white(&self) -> BgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the background color to bright white

- `fn OwoColorize::bold(&self) -> styles::BoldDisplay<'_, Self>`

  Make the text bold

- `fn OwoColorize::dimmed(&self) -> styles::DimDisplay<'_, Self>`

  Make the text dim

- `fn OwoColorize::italic(&self) -> styles::ItalicDisplay<'_, Self>`

  Make the text italicized

- `fn OwoColorize::underline(&self) -> styles::UnderlineDisplay<'_, Self>`

  Make the text underlined

- `fn OwoColorize::blink(&self) -> styles::BlinkDisplay<'_, Self>`

  Make the text blink

- `fn OwoColorize::blink_fast(&self) -> styles::BlinkFastDisplay<'_, Self>`

  Make the text blink (but fast!)

- `fn OwoColorize::reversed(&self) -> styles::ReversedDisplay<'_, Self>`

  Swap the foreground and background colors

- `fn OwoColorize::hidden(&self) -> styles::HiddenDisplay<'_, Self>`

  Hide the text

- `fn OwoColorize::strikethrough(&self) -> styles::StrikeThroughDisplay<'_, Self>`

  Cross out the text

- `fn OwoColorize::color<Color: DynColor>(&self, color: Color) -> FgDynColorDisplay<'_, Color, Self>`

  Set the foreground color at runtime. Only use if you do not know which color will be used at
  compile-time. If the color is constant, use either `OwoColorize::fg` or
  a color-specific method, such as `OwoColorize::green`,
  
  ```rust
  use owo_colors::{OwoColorize, AnsiColors};
  
  println!("{}", "green".color(AnsiColors::Green));
  ```

- `fn OwoColorize::on_color<Color: DynColor>(&self, color: Color) -> BgDynColorDisplay<'_, Color, Self>`

  Set the background color at runtime. Only use if you do not know what color to use at
  compile-time. If the color is constant, use either `OwoColorize::bg` or
  a color-specific method, such as `OwoColorize::on_yellow`,
  
  ```rust
  use owo_colors::{OwoColorize, AnsiColors};
  
  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));
  ```

- `fn OwoColorize::fg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> FgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the foreground color to a specific RGB value.

- `fn OwoColorize::bg_rgb<const R: u8, const G: u8, const B: u8>(&self) -> BgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the background color to a specific RGB value.

- `fn OwoColorize::truecolor(&self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self>`

  Sets the foreground color to an RGB value.

- `fn OwoColorize::on_truecolor(&self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self>`

  Sets the background color to an RGB value.

- `fn OwoColorize::style(&self, style: Style) -> Styled<&Self>`

  Apply a runtime-determined style

#### Implementors

- `D`

