*[owo_colors](../index.md) / [combo](index.md)*

---

# Module `combo`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ComboColorDisplay`](#combocolordisplay) | struct | A wrapper type which applies both a foreground and background color |
| [`ComboDynColorDisplay`](#combodyncolordisplay) | struct | Wrapper around a type which implements all the formatters the wrapped type does, with the addition of changing the foreground and background color. |
| [`impl_fmt_for_combo!`](#impl-fmt-for-combo) | macro |  |
| [`color_methods!`](#color-methods) | macro | implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay |

## Structs

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>(&'a T, core::marker::PhantomData<(Fg, Bg)>);
```

*Defined in [`owo-colors-4.2.3/src/combo.rs:11`](../../../.source_1765521767/owo-colors-4.2.3/src/combo.rs#L11)*

A wrapper type which applies both a foreground and background color

#### Implementations

- <span id="combocolordisplay-new"></span>`const fn new(thing: &'a T) -> Self`

  Create a new [`ComboColorDisplay`](#combocolordisplay), from a pair of foreground and background types

  which implement [`Color`](../index.md).

  

  This is a const function: in non-const contexts, calling the [`OwoColorize`](../index.md)

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

- <span id="combocolordisplay-into-styled"></span>`const fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

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

- <span id="combocolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](../index.md#color)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="combocolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay), [`Color`](../index.md#color)

  Set the foreground color at runtime. Only use if you do not know which color will be used at

  compile-time. If the color is constant, use either `OwoColorize::fg` or

  a color-specific method, such as `OwoColorize::green`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "green".color(AnsiColors::Green));

  ```

- <span id="combocolordisplay-fg"></span>`const fn fg<C: Color>(self) -> ComboColorDisplay<'a, C, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay)

  Set the foreground color generically

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "red foreground".fg::<Red>());

  ```

- <span id="combocolordisplay-bg"></span>`const fn bg<C: Color>(self) -> ComboColorDisplay<'a, Fg, C, T>` — [`ComboColorDisplay`](#combocolordisplay)

  Set the background color generically.

  

  ```rust

  use owo_colors::{OwoColorize, colors::*};

  

  println!("{}", "black background".bg::<Black>());

  ```

- <span id="combocolordisplay-on-black"></span>`const fn on_black(self) -> ComboColorDisplay<'a, Fg, colors::Black, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](../colors/index.md#black)

  Change the background color to black

- <span id="combocolordisplay-black"></span>`const fn black(self) -> ComboColorDisplay<'a, colors::Black, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Black`](../colors/index.md#black)

  Change the foreground color to black

- <span id="combocolordisplay-on-red"></span>`const fn on_red(self) -> ComboColorDisplay<'a, Fg, colors::Red, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](../colors/index.md#red)

  Change the background color to red

- <span id="combocolordisplay-red"></span>`const fn red(self) -> ComboColorDisplay<'a, colors::Red, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Red`](../colors/index.md#red)

  Change the foreground color to red

- <span id="combocolordisplay-on-green"></span>`const fn on_green(self) -> ComboColorDisplay<'a, Fg, colors::Green, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](../colors/index.md#green)

  Change the background color to green

- <span id="combocolordisplay-green"></span>`const fn green(self) -> ComboColorDisplay<'a, colors::Green, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Green`](../colors/index.md#green)

  Change the foreground color to green

- <span id="combocolordisplay-on-yellow"></span>`const fn on_yellow(self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](../colors/index.md#yellow)

  Change the background color to yellow

- <span id="combocolordisplay-yellow"></span>`const fn yellow(self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Yellow`](../colors/index.md#yellow)

  Change the foreground color to yellow

- <span id="combocolordisplay-on-blue"></span>`const fn on_blue(self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](../colors/index.md#blue)

  Change the background color to blue

- <span id="combocolordisplay-blue"></span>`const fn blue(self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Blue`](../colors/index.md#blue)

  Change the foreground color to blue

- <span id="combocolordisplay-on-magenta"></span>`const fn on_magenta(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md#magenta)

  Change the background color to magenta

- <span id="combocolordisplay-magenta"></span>`const fn magenta(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md#magenta)

  Change the foreground color to magenta

- <span id="combocolordisplay-on-purple"></span>`const fn on_purple(self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md#magenta)

  Change the background color to purple

- <span id="combocolordisplay-purple"></span>`const fn purple(self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Magenta`](../colors/index.md#magenta)

  Change the foreground color to purple

- <span id="combocolordisplay-on-cyan"></span>`const fn on_cyan(self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](../colors/index.md#cyan)

  Change the background color to cyan

- <span id="combocolordisplay-cyan"></span>`const fn cyan(self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`Cyan`](../colors/index.md#cyan)

  Change the foreground color to cyan

- <span id="combocolordisplay-on-white"></span>`const fn on_white(self) -> ComboColorDisplay<'a, Fg, colors::White, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](../colors/index.md#white)

  Change the background color to white

- <span id="combocolordisplay-white"></span>`const fn white(self) -> ComboColorDisplay<'a, colors::White, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`White`](../colors/index.md#white)

  Change the foreground color to white

- <span id="combocolordisplay-on-bright-black"></span>`const fn on_bright_black(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](../colors/index.md#brightblack)

  Change the background color to bright black

- <span id="combocolordisplay-bright-black"></span>`const fn bright_black(self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlack`](../colors/index.md#brightblack)

  Change the foreground color to bright black

- <span id="combocolordisplay-on-bright-red"></span>`const fn on_bright_red(self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](../colors/index.md#brightred)

  Change the background color to bright red

- <span id="combocolordisplay-bright-red"></span>`const fn bright_red(self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightRed`](../colors/index.md#brightred)

  Change the foreground color to bright red

- <span id="combocolordisplay-on-bright-green"></span>`const fn on_bright_green(self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](../colors/index.md#brightgreen)

  Change the background color to bright green

- <span id="combocolordisplay-bright-green"></span>`const fn bright_green(self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightGreen`](../colors/index.md#brightgreen)

  Change the foreground color to bright green

- <span id="combocolordisplay-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](../colors/index.md#brightyellow)

  Change the background color to bright yellow

- <span id="combocolordisplay-bright-yellow"></span>`const fn bright_yellow(self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightYellow`](../colors/index.md#brightyellow)

  Change the foreground color to bright yellow

- <span id="combocolordisplay-on-bright-blue"></span>`const fn on_bright_blue(self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](../colors/index.md#brightblue)

  Change the background color to bright blue

- <span id="combocolordisplay-bright-blue"></span>`const fn bright_blue(self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightBlue`](../colors/index.md#brightblue)

  Change the foreground color to bright blue

- <span id="combocolordisplay-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md#brightmagenta)

  Change the background color to bright magenta

- <span id="combocolordisplay-bright-magenta"></span>`const fn bright_magenta(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md#brightmagenta)

  Change the foreground color to bright magenta

- <span id="combocolordisplay-on-bright-purple"></span>`const fn on_bright_purple(self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md#brightmagenta)

  Change the background color to bright purple

- <span id="combocolordisplay-bright-purple"></span>`const fn bright_purple(self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightMagenta`](../colors/index.md#brightmagenta)

  Change the foreground color to bright purple

- <span id="combocolordisplay-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](../colors/index.md#brightcyan)

  Change the background color to bright cyan

- <span id="combocolordisplay-bright-cyan"></span>`const fn bright_cyan(self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightCyan`](../colors/index.md#brightcyan)

  Change the foreground color to bright cyan

- <span id="combocolordisplay-on-bright-white"></span>`const fn on_bright_white(self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](../colors/index.md#brightwhite)

  Change the background color to bright white

- <span id="combocolordisplay-bright-white"></span>`const fn bright_white(self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>` — [`ComboColorDisplay`](#combocolordisplay), [`BrightWhite`](../colors/index.md#brightwhite)

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

*Defined in [`owo-colors-4.2.3/src/combo.rs:17`](../../../.source_1765521767/owo-colors-4.2.3/src/combo.rs#L17)*

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](#combocolordisplay) instead.

#### Implementations

- <span id="combodyncolordisplay-new"></span>`const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`

  Create a new [`ComboDynColorDisplay`](#combodyncolordisplay), from a pair of types which implement

  [`DynColor`](../index.md).

  

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

- <span id="combodyncolordisplay-into-styled"></span>`fn into_styled(self) -> Styled<&'a T>` — [`Styled`](../index.md#styled)

  Convert self to a generic [`Styled`](../index.md).

  

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

- <span id="combodyncolordisplay-on-color"></span>`const fn on_color<NewBg: DynColor>(self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

  Set the background color at runtime. Only use if you do not know what color to use at

  compile-time. If the color is constant, use either `OwoColorize::bg` or

  a color-specific method, such as `OwoColorize::on_yellow`,

  

  ```rust

  use owo_colors::{OwoColorize, AnsiColors};

  

  println!("{}", "yellow background".on_color(AnsiColors::BrightYellow));

  ```

- <span id="combodyncolordisplay-color"></span>`const fn color<NewFg: DynColor>(self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>` — [`ComboDynColorDisplay`](#combodyncolordisplay)

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

## Macros

### `impl_fmt_for_combo!`

*Defined in [`owo-colors-4.2.3/src/combo.rs:19-51`](../../../.source_1765521767/owo-colors-4.2.3/src/combo.rs#L19-L51)*

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/combo.rs:66-516`](../../../.source_1765521767/owo-colors-4.2.3/src/combo.rs#L66-L516)*

implement specialized color methods for FgColorDisplay BgColorDisplay, ComboColorDisplay

