*[owo_colors](../index.md) / [dyn_styles](index.md)*

---

# Module `dyn_styles`

## Contents

- [Structs](#structs)
  - [`Styled`](#styled)
  - [`Style`](#style)
  - [`StyleFlags`](#styleflags)
  - [`StylePrefixFormatter`](#styleprefixformatter)
  - [`StyleSuffixFormatter`](#stylesuffixformatter)
- [Enums](#enums)
  - [`Effect`](#effect)
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
  - [`color_methods!`](#color-methods)
  - [`style_methods!`](#style-methods)
  - [`style_flags_methods!`](#style-flags-methods)
  - [`impl_fmt!`](#impl-fmt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Styled`](#styled) | struct | A wrapper type which applies a [`Style`] when displaying the inner type |
| [`Style`](#style) | struct | A pre-computed style that can be applied to a struct using [`OwoColorize::style`]. |
| [`StyleFlags`](#styleflags) | struct |  |
| [`StylePrefixFormatter`](#styleprefixformatter) | struct | Formatter for the prefix of a [`Style`]. |
| [`StyleSuffixFormatter`](#stylesuffixformatter) | struct | Formatter for the suffix of a [`Style`]. |
| [`Effect`](#effect) | enum | A runtime-configurable text effect for use with [`Style`] |
| [`style`](#style) | fn | Helper to create [`Style`]s more ergonomically |
| [`DIMMED_SHIFT`](#dimmed-shift) | const |  |
| [`ITALIC_SHIFT`](#italic-shift) | const |  |
| [`UNDERLINE_SHIFT`](#underline-shift) | const |  |
| [`BLINK_SHIFT`](#blink-shift) | const |  |
| [`BLINK_FAST_SHIFT`](#blink-fast-shift) | const |  |
| [`REVERSED_SHIFT`](#reversed-shift) | const |  |
| [`HIDDEN_SHIFT`](#hidden-shift) | const |  |
| [`STRIKETHROUGH_SHIFT`](#strikethrough-shift) | const |  |
| [`color_methods!`](#color-methods) | macro |  |
| [`style_methods!`](#style-methods) | macro |  |
| [`style_flags_methods!`](#style-flags-methods) | macro |  |
| [`impl_fmt!`](#impl-fmt) | macro |  |

## Structs

### `Styled<T>`

```rust
struct Styled<T> {
    target: T,
    pub style: Style,
}
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:60-65`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L60-L65)*

A wrapper type which applies a [`Style`](../index.md) when displaying the inner type

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

- <span id="cratestyled-isstyled-style"></span>`fn style(&self) -> &Style` — [`Style`](../index.md#style)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:83-88`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L83-L88)*

A pre-computed style that can be applied to a struct using `OwoColorize::style`.

Its interface mimics that of [`OwoColorize`](../index.md), but instead of chaining methods on your
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

- <span id="style-style"></span>`const fn style<T>(&self, target: T) -> Styled<T>` — [`Styled`](../index.md#styled)

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

- <span id="style-set-effect"></span>`const fn set_effect(self, effect: Effect, to: bool) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-set-effects"></span>`const fn set_effects(self, effects: &[Effect], to: bool) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-effect"></span>`const fn effect(self, effect: Effect) -> Self` — [`Effect`](../index.md#effect)

  Apply a given effect from the style

- <span id="style-remove-effect"></span>`const fn remove_effect(self, effect: Effect) -> Self` — [`Effect`](../index.md#effect)

  Remove a given effect from the style

- <span id="style-effects"></span>`const fn effects(self, effects: &[Effect]) -> Self` — [`Effect`](../index.md#effect)

  Apply a given set of effects to the style

- <span id="style-remove-effects"></span>`const fn remove_effects(self, effects: &[Effect]) -> Self` — [`Effect`](../index.md#effect)

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

- <span id="style-prefix-formatter"></span>`const fn prefix_formatter(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

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

- <span id="style-suffix-formatter"></span>`const fn suffix_formatter(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

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

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md#style)

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

- <span id="style-partialeq-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md#style)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:92`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L92)*

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

- <span id="styleflags-clone"></span>`fn clone(&self) -> StyleFlags` — [`StyleFlags`](#styleflags)

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

- <span id="styleflags-partialeq-eq"></span>`fn eq(&self, other: &StyleFlags) -> bool` — [`StyleFlags`](#styleflags)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:597`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L597)*

Formatter for the prefix of a [`Style`](../index.md).

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

- <span id="styleprefixformatter-clone"></span>`fn clone(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

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

- <span id="styleprefixformatter-partialeq-eq"></span>`fn eq(&self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:611`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L611)*

Formatter for the suffix of a [`Style`](../index.md).

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

- <span id="stylesuffixformatter-clone"></span>`fn clone(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

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

- <span id="stylesuffixformatter-partialeq-eq"></span>`fn eq(&self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

##### `impl StructuralPartialEq for StyleSuffixFormatter`

##### `impl<U> TryFrom for StyleSuffixFormatter`

- <span id="stylesuffixformatter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stylesuffixformatter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyleSuffixFormatter`

- <span id="stylesuffixformatter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stylesuffixformatter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:10-20`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L10-L20)*

A runtime-configurable text effect for use with [`Style`](../index.md)

#### Trait Implementations

##### `impl Any for Effect`

- <span id="effect-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Effect`

- <span id="effect-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Effect`

- <span id="effect-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Effect`

- <span id="effect-clone"></span>`fn clone(&self) -> Effect` — [`Effect`](../index.md#effect)

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

## Functions

### `style`

```rust
const fn style() -> Style
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:626-628`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L626-L628)*

Helper to create [`Style`](../index.md)s more ergonomically

## Constants

### `DIMMED_SHIFT`
```rust
const DIMMED_SHIFT: u8 = 0u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:102`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L102)*

### `ITALIC_SHIFT`
```rust
const ITALIC_SHIFT: u8 = 1u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:103`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L103)*

### `UNDERLINE_SHIFT`
```rust
const UNDERLINE_SHIFT: u8 = 2u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:104`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L104)*

### `BLINK_SHIFT`
```rust
const BLINK_SHIFT: u8 = 3u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:105`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L105)*

### `BLINK_FAST_SHIFT`
```rust
const BLINK_FAST_SHIFT: u8 = 4u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:106`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L106)*

### `REVERSED_SHIFT`
```rust
const REVERSED_SHIFT: u8 = 5u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:107`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L107)*

### `HIDDEN_SHIFT`
```rust
const HIDDEN_SHIFT: u8 = 6u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:108`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L108)*

### `STRIKETHROUGH_SHIFT`
```rust
const STRIKETHROUGH_SHIFT: u8 = 7u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:109`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L109)*

## Macros

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:22-42`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L22-L42)*

### `style_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:44-55`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L44-L55)*

### `style_flags_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:111-126`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L111-L126)*

### `impl_fmt!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:653-666`](../../../.source_1765633015/owo-colors-4.2.3/src/dyn_styles.rs#L653-L666)*

