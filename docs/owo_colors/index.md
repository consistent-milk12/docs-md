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
# #[cfg(feature = "supports-color")] {
use owo_colors::{OwoColorize, Stream::Stdout};

println!(
    "{}",
    "colored blue if a supported terminal"
        .if_supports_color(Stdout, |text| text.bright_blue())
);
# }
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

## Modules

- [`colors`](colors/index.md) - Color types for used for being generic over the color
- [`styles`](styles/index.md) - Different display styles (strikethrough, bold, etc.)
- [`colored`](colored/index.md) - Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from

## Structs

### `FgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct FgColorDisplay<'a, C: Color, T: ?Sized>();
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- `const fn new(thing: &'a T) -> Self`
  Create a new [`FgColorDisplay`], from a reference to a type which implements

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn fg<C: Color>(self: Self) -> FgColorDisplay<'a, C, T>`
  Set the foreground color generically

- `const fn bg<C: Color>(self: Self) -> ComboColorDisplay<'a, Fg, C, T>`
  Set the background color generically.

- `const fn black(self: Self) -> FgColorDisplay<'a, colors::Black, T>`
  Change the foreground color to black

- `const fn on_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::Black, T>`
  Change the background color to black

- `const fn red(self: Self) -> FgColorDisplay<'a, colors::Red, T>`
  Change the foreground color to red

- `const fn on_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::Red, T>`
  Change the background color to red

- `const fn green(self: Self) -> FgColorDisplay<'a, colors::Green, T>`
  Change the foreground color to green

- `const fn on_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::Green, T>`
  Change the background color to green

- `const fn yellow(self: Self) -> FgColorDisplay<'a, colors::Yellow, T>`
  Change the foreground color to yellow

- `const fn on_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>`
  Change the background color to yellow

- `const fn blue(self: Self) -> FgColorDisplay<'a, colors::Blue, T>`
  Change the foreground color to blue

- `const fn on_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>`
  Change the background color to blue

- `const fn magenta(self: Self) -> FgColorDisplay<'a, colors::Magenta, T>`
  Change the foreground color to magenta

- `const fn on_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>`
  Change the background color to magenta

- `const fn purple(self: Self) -> FgColorDisplay<'a, colors::Magenta, T>`
  Change the foreground color to purple

- `const fn on_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>`
  Change the background color to purple

- `const fn cyan(self: Self) -> FgColorDisplay<'a, colors::Cyan, T>`
  Change the foreground color to cyan

- `const fn on_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>`
  Change the background color to cyan

- `const fn white(self: Self) -> FgColorDisplay<'a, colors::White, T>`
  Change the foreground color to white

- `const fn on_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::White, T>`
  Change the background color to white

- `const fn bright_black(self: Self) -> FgColorDisplay<'a, colors::BrightBlack, T>`
  Change the foreground color to bright black

- `const fn on_bright_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>`
  Change the background color to bright black

- `const fn bright_red(self: Self) -> FgColorDisplay<'a, colors::BrightRed, T>`
  Change the foreground color to bright red

- `const fn on_bright_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>`
  Change the background color to bright red

- `const fn bright_green(self: Self) -> FgColorDisplay<'a, colors::BrightGreen, T>`
  Change the foreground color to bright green

- `const fn on_bright_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>`
  Change the background color to bright green

- `const fn bright_yellow(self: Self) -> FgColorDisplay<'a, colors::BrightYellow, T>`
  Change the foreground color to bright yellow

- `const fn on_bright_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>`
  Change the background color to bright yellow

- `const fn bright_blue(self: Self) -> FgColorDisplay<'a, colors::BrightBlue, T>`
  Change the foreground color to bright blue

- `const fn on_bright_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>`
  Change the background color to bright blue

- `const fn bright_magenta(self: Self) -> FgColorDisplay<'a, colors::BrightMagenta, T>`
  Change the foreground color to bright magenta

- `const fn on_bright_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>`
  Change the background color to bright magenta

- `const fn bright_purple(self: Self) -> FgColorDisplay<'a, colors::BrightMagenta, T>`
  Change the foreground color to bright purple

- `const fn on_bright_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>`
  Change the background color to bright purple

- `const fn bright_cyan(self: Self) -> FgColorDisplay<'a, colors::BrightCyan, T>`
  Change the foreground color to bright cyan

- `const fn on_bright_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>`
  Change the background color to bright cyan

- `const fn bright_white(self: Self) -> FgColorDisplay<'a, colors::BrightWhite, T>`
  Change the foreground color to bright white

- `const fn on_bright_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>`
  Change the background color to bright white

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Color: crate::Color, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Color: crate::Color, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Color: crate::Color, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Color: crate::Color, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Color: crate::Color, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Color: crate::Color, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Color: crate::Color, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Color: crate::Color, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Color: crate::Color, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgColorDisplay<'a, C: Color, T: ?Sized>`

```rust
struct BgColorDisplay<'a, C: Color, T: ?Sized>();
```

Transparent wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Recommended to be constructed using
[`OwoColorize`](#owocolorize).

#### Implementations

- `const fn new(thing: &'a T) -> Self`
  Create a new [`BgColorDisplay`], from a reference to a type which implements

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn fg<C: Color>(self: Self) -> ComboColorDisplay<'a, C, Bg, T>`
  Set the foreground color generically

- `const fn bg<C: Color>(self: Self) -> BgColorDisplay<'a, C, T>`
  Set the background color generically.

- `const fn on_black(self: Self) -> BgColorDisplay<'a, colors::Black, T>`
  Change the background color to black

- `const fn black(self: Self) -> ComboColorDisplay<'a, colors::Black, Bg, T>`
  Change the foreground color to black

- `const fn on_red(self: Self) -> BgColorDisplay<'a, colors::Red, T>`
  Change the background color to red

- `const fn red(self: Self) -> ComboColorDisplay<'a, colors::Red, Bg, T>`
  Change the foreground color to red

- `const fn on_green(self: Self) -> BgColorDisplay<'a, colors::Green, T>`
  Change the background color to green

- `const fn green(self: Self) -> ComboColorDisplay<'a, colors::Green, Bg, T>`
  Change the foreground color to green

- `const fn on_yellow(self: Self) -> BgColorDisplay<'a, colors::Yellow, T>`
  Change the background color to yellow

- `const fn yellow(self: Self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>`
  Change the foreground color to yellow

- `const fn on_blue(self: Self) -> BgColorDisplay<'a, colors::Blue, T>`
  Change the background color to blue

- `const fn blue(self: Self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>`
  Change the foreground color to blue

- `const fn on_magenta(self: Self) -> BgColorDisplay<'a, colors::Magenta, T>`
  Change the background color to magenta

- `const fn magenta(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>`
  Change the foreground color to magenta

- `const fn on_purple(self: Self) -> BgColorDisplay<'a, colors::Magenta, T>`
  Change the background color to purple

- `const fn purple(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>`
  Change the foreground color to purple

- `const fn on_cyan(self: Self) -> BgColorDisplay<'a, colors::Cyan, T>`
  Change the background color to cyan

- `const fn cyan(self: Self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>`
  Change the foreground color to cyan

- `const fn on_white(self: Self) -> BgColorDisplay<'a, colors::White, T>`
  Change the background color to white

- `const fn white(self: Self) -> ComboColorDisplay<'a, colors::White, Bg, T>`
  Change the foreground color to white

- `const fn on_bright_black(self: Self) -> BgColorDisplay<'a, colors::BrightBlack, T>`
  Change the background color to bright black

- `const fn bright_black(self: Self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>`
  Change the foreground color to bright black

- `const fn on_bright_red(self: Self) -> BgColorDisplay<'a, colors::BrightRed, T>`
  Change the background color to bright red

- `const fn bright_red(self: Self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>`
  Change the foreground color to bright red

- `const fn on_bright_green(self: Self) -> BgColorDisplay<'a, colors::BrightGreen, T>`
  Change the background color to bright green

- `const fn bright_green(self: Self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>`
  Change the foreground color to bright green

- `const fn on_bright_yellow(self: Self) -> BgColorDisplay<'a, colors::BrightYellow, T>`
  Change the background color to bright yellow

- `const fn bright_yellow(self: Self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>`
  Change the foreground color to bright yellow

- `const fn on_bright_blue(self: Self) -> BgColorDisplay<'a, colors::BrightBlue, T>`
  Change the background color to bright blue

- `const fn bright_blue(self: Self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>`
  Change the foreground color to bright blue

- `const fn on_bright_magenta(self: Self) -> BgColorDisplay<'a, colors::BrightMagenta, T>`
  Change the background color to bright magenta

- `const fn bright_magenta(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>`
  Change the foreground color to bright magenta

- `const fn on_bright_purple(self: Self) -> BgColorDisplay<'a, colors::BrightMagenta, T>`
  Change the background color to bright purple

- `const fn bright_purple(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>`
  Change the foreground color to bright purple

- `const fn on_bright_cyan(self: Self) -> BgColorDisplay<'a, colors::BrightCyan, T>`
  Change the background color to bright cyan

- `const fn bright_cyan(self: Self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>`
  Change the foreground color to bright cyan

- `const fn on_bright_white(self: Self) -> BgColorDisplay<'a, colors::BrightWhite, T>`
  Change the background color to bright white

- `const fn bright_white(self: Self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>`
  Change the foreground color to bright white

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Color: crate::Color, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Color: crate::Color, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Color: crate::Color, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Color: crate::Color, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Color: crate::Color, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Color: crate::Color, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Color: crate::Color, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Color: crate::Color, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Color: crate::Color, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct FgDynColorDisplay<'a, Color: DynColor, T: ?Sized>();
```

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the foreground color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- `const fn new(thing: &'a T, color: Fg) -> Self`
  Create a new [`FgDynColorDisplay`], from a reference to a type which implements

- `fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn on_color<Bg: DynColor>(self: Self, bg: Bg) -> ComboDynColorDisplay<'a, Fg, Bg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> FgDynColorDisplay<'a, NewFg, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Color: crate::DynColor, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Color: crate::DynColor, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Color: crate::DynColor, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Color: crate::DynColor, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Color: crate::DynColor, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>`

```rust
struct BgDynColorDisplay<'a, Color: DynColor, T: ?Sized>();
```

Wrapper around a type which implements all the formatters the wrapped type does,
with the addition of changing the background color. Is not recommended unless compile-time
coloring is not an option.

#### Implementations

- `const fn new(thing: &'a T, color: Bg) -> Self`
  Create a new [`BgDynColorDisplay`], from a reference to a type which implements

- `fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> BgDynColorDisplay<'a, NewBg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn color<Fg: DynColor>(self: Self, fg: Fg) -> ComboDynColorDisplay<'a, Fg, Bg, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Color: crate::DynColor, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Color: crate::DynColor, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Color: crate::DynColor, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Color: crate::DynColor, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Color: crate::DynColor, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Color: crate::DynColor, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Color: crate::DynColor, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Rgb`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl DynColor`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl OwoColorize<D>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Rgb) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>`

```rust
struct ComboColorDisplay<'a, Fg: Color, Bg: Color, T: ?Sized>();
```

A wrapper type which applies both a foreground and background color

#### Implementations

- `const fn new(thing: &'a T) -> Self`
  Create a new [`ComboColorDisplay`], from a pair of foreground and background types

- `const fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> ComboDynColorDisplay<'a, <Fg as >::DynEquivalent, NewBg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, <Bg as >::DynEquivalent, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `const fn fg<C: Color>(self: Self) -> ComboColorDisplay<'a, C, Bg, T>`
  Set the foreground color generically

- `const fn bg<C: Color>(self: Self) -> ComboColorDisplay<'a, Fg, C, T>`
  Set the background color generically.

- `const fn on_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::Black, T>`
  Change the background color to black

- `const fn black(self: Self) -> ComboColorDisplay<'a, colors::Black, Bg, T>`
  Change the foreground color to black

- `const fn on_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::Red, T>`
  Change the background color to red

- `const fn red(self: Self) -> ComboColorDisplay<'a, colors::Red, Bg, T>`
  Change the foreground color to red

- `const fn on_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::Green, T>`
  Change the background color to green

- `const fn green(self: Self) -> ComboColorDisplay<'a, colors::Green, Bg, T>`
  Change the foreground color to green

- `const fn on_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::Yellow, T>`
  Change the background color to yellow

- `const fn yellow(self: Self) -> ComboColorDisplay<'a, colors::Yellow, Bg, T>`
  Change the foreground color to yellow

- `const fn on_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::Blue, T>`
  Change the background color to blue

- `const fn blue(self: Self) -> ComboColorDisplay<'a, colors::Blue, Bg, T>`
  Change the foreground color to blue

- `const fn on_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>`
  Change the background color to magenta

- `const fn magenta(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>`
  Change the foreground color to magenta

- `const fn on_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::Magenta, T>`
  Change the background color to purple

- `const fn purple(self: Self) -> ComboColorDisplay<'a, colors::Magenta, Bg, T>`
  Change the foreground color to purple

- `const fn on_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::Cyan, T>`
  Change the background color to cyan

- `const fn cyan(self: Self) -> ComboColorDisplay<'a, colors::Cyan, Bg, T>`
  Change the foreground color to cyan

- `const fn on_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::White, T>`
  Change the background color to white

- `const fn white(self: Self) -> ComboColorDisplay<'a, colors::White, Bg, T>`
  Change the foreground color to white

- `const fn on_bright_black(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlack, T>`
  Change the background color to bright black

- `const fn bright_black(self: Self) -> ComboColorDisplay<'a, colors::BrightBlack, Bg, T>`
  Change the foreground color to bright black

- `const fn on_bright_red(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightRed, T>`
  Change the background color to bright red

- `const fn bright_red(self: Self) -> ComboColorDisplay<'a, colors::BrightRed, Bg, T>`
  Change the foreground color to bright red

- `const fn on_bright_green(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightGreen, T>`
  Change the background color to bright green

- `const fn bright_green(self: Self) -> ComboColorDisplay<'a, colors::BrightGreen, Bg, T>`
  Change the foreground color to bright green

- `const fn on_bright_yellow(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightYellow, T>`
  Change the background color to bright yellow

- `const fn bright_yellow(self: Self) -> ComboColorDisplay<'a, colors::BrightYellow, Bg, T>`
  Change the foreground color to bright yellow

- `const fn on_bright_blue(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightBlue, T>`
  Change the background color to bright blue

- `const fn bright_blue(self: Self) -> ComboColorDisplay<'a, colors::BrightBlue, Bg, T>`
  Change the foreground color to bright blue

- `const fn on_bright_magenta(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>`
  Change the background color to bright magenta

- `const fn bright_magenta(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>`
  Change the foreground color to bright magenta

- `const fn on_bright_purple(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightMagenta, T>`
  Change the background color to bright purple

- `const fn bright_purple(self: Self) -> ComboColorDisplay<'a, colors::BrightMagenta, Bg, T>`
  Change the foreground color to bright purple

- `const fn on_bright_cyan(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightCyan, T>`
  Change the background color to bright cyan

- `const fn bright_cyan(self: Self) -> ComboColorDisplay<'a, colors::BrightCyan, Bg, T>`
  Change the foreground color to bright cyan

- `const fn on_bright_white(self: Self) -> ComboColorDisplay<'a, Fg, colors::BrightWhite, T>`
  Change the background color to bright white

- `const fn bright_white(self: Self) -> ComboColorDisplay<'a, colors::BrightWhite, Bg, T>`
  Change the foreground color to bright white

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Fg: Color, Bg: Color, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>`

```rust
struct ComboDynColorDisplay<'a, Fg: DynColor, Bg: DynColor, T: ?Sized>();
```

Wrapper around a type which implements all the formatters the wrapped type does, with the
addition of changing the foreground and background color.

If compile-time coloring is an option, consider using [`ComboColorDisplay`](#combocolordisplay) instead.

#### Implementations

- `const fn new(thing: &'a T, fg: Fg, bg: Bg) -> Self`
  Create a new [`ComboDynColorDisplay`], from a pair of types which implement

- `fn into_styled(self: Self) -> Styled<&'a T>`
  Convert self to a generic [`Styled`].

- `const fn on_color<NewBg: DynColor>(self: Self, bg: NewBg) -> ComboDynColorDisplay<'a, Fg, NewBg, T>`
  Set the background color at runtime. Only use if you do not know what color to use at

- `const fn color<NewFg: DynColor>(self: Self, fg: NewFg) -> ComboDynColorDisplay<'a, NewFg, Bg, T>`
  Set the foreground color at runtime. Only use if you do not know which color will be used at

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Binary<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Binary>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerExp<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::LowerHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Octal>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl Pointer<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Pointer>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperExp>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::UpperHex>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Debug<'a, Fg: DynColor, Bg: DynColor, T: ?Sized + fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StyledList<T, U>`

```rust
struct StyledList<T, U>(T)
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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<T, U>`

- `fn from(list: T) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display<T, U>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(color: &'a str) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> AnsiColors`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl DynColor`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl OwoColorize<D>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AnsiColors) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> CssColors`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl DynColor`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl OwoColorize<D>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &CssColors) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From`

- `fn from(x: u8) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> XtermColors`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl DynColor`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl OwoColorize<D>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &XtermColors) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a ANSI code to a formatter to set the foreground to this color

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a ANSI code to a formatter to set the background to this color

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  A function to output a raw ANSI code to a formatter to set the foreground to this color,

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn fg<C: Color>(self: &Self) -> FgColorDisplay<'_, C, Self>`

  Set the foreground color generically

- `fn bg<C: Color>(self: &Self) -> BgColorDisplay<'_, C, Self>`

  Set the background color generically.

- `fn black(self: &Self) -> FgColorDisplay<'_, colors::Black, Self>`

  Change the foreground color to black

- `fn on_black(self: &Self) -> BgColorDisplay<'_, colors::Black, Self>`

  Change the background color to black

- `fn red(self: &Self) -> FgColorDisplay<'_, colors::Red, Self>`

  Change the foreground color to red

- `fn on_red(self: &Self) -> BgColorDisplay<'_, colors::Red, Self>`

  Change the background color to red

- `fn green(self: &Self) -> FgColorDisplay<'_, colors::Green, Self>`

  Change the foreground color to green

- `fn on_green(self: &Self) -> BgColorDisplay<'_, colors::Green, Self>`

  Change the background color to green

- `fn yellow(self: &Self) -> FgColorDisplay<'_, colors::Yellow, Self>`

  Change the foreground color to yellow

- `fn on_yellow(self: &Self) -> BgColorDisplay<'_, colors::Yellow, Self>`

  Change the background color to yellow

- `fn blue(self: &Self) -> FgColorDisplay<'_, colors::Blue, Self>`

  Change the foreground color to blue

- `fn on_blue(self: &Self) -> BgColorDisplay<'_, colors::Blue, Self>`

  Change the background color to blue

- `fn magenta(self: &Self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to magenta

- `fn on_magenta(self: &Self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to magenta

- `fn purple(self: &Self) -> FgColorDisplay<'_, colors::Magenta, Self>`

  Change the foreground color to purple

- `fn on_purple(self: &Self) -> BgColorDisplay<'_, colors::Magenta, Self>`

  Change the background color to purple

- `fn cyan(self: &Self) -> FgColorDisplay<'_, colors::Cyan, Self>`

  Change the foreground color to cyan

- `fn on_cyan(self: &Self) -> BgColorDisplay<'_, colors::Cyan, Self>`

  Change the background color to cyan

- `fn white(self: &Self) -> FgColorDisplay<'_, colors::White, Self>`

  Change the foreground color to white

- `fn on_white(self: &Self) -> BgColorDisplay<'_, colors::White, Self>`

  Change the background color to white

- `fn default_color(self: &Self) -> FgColorDisplay<'_, colors::Default, Self>`

  Change the foreground color to the terminal default

- `fn on_default_color(self: &Self) -> BgColorDisplay<'_, colors::Default, Self>`

  Change the background color to the terminal default

- `fn bright_black(self: &Self) -> FgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the foreground color to bright black

- `fn on_bright_black(self: &Self) -> BgColorDisplay<'_, colors::BrightBlack, Self>`

  Change the background color to bright black

- `fn bright_red(self: &Self) -> FgColorDisplay<'_, colors::BrightRed, Self>`

  Change the foreground color to bright red

- `fn on_bright_red(self: &Self) -> BgColorDisplay<'_, colors::BrightRed, Self>`

  Change the background color to bright red

- `fn bright_green(self: &Self) -> FgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the foreground color to bright green

- `fn on_bright_green(self: &Self) -> BgColorDisplay<'_, colors::BrightGreen, Self>`

  Change the background color to bright green

- `fn bright_yellow(self: &Self) -> FgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the foreground color to bright yellow

- `fn on_bright_yellow(self: &Self) -> BgColorDisplay<'_, colors::BrightYellow, Self>`

  Change the background color to bright yellow

- `fn bright_blue(self: &Self) -> FgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the foreground color to bright blue

- `fn on_bright_blue(self: &Self) -> BgColorDisplay<'_, colors::BrightBlue, Self>`

  Change the background color to bright blue

- `fn bright_magenta(self: &Self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright magenta

- `fn on_bright_magenta(self: &Self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright magenta

- `fn bright_purple(self: &Self) -> FgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the foreground color to bright purple

- `fn on_bright_purple(self: &Self) -> BgColorDisplay<'_, colors::BrightMagenta, Self>`

  Change the background color to bright purple

- `fn bright_cyan(self: &Self) -> FgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the foreground color to bright cyan

- `fn on_bright_cyan(self: &Self) -> BgColorDisplay<'_, colors::BrightCyan, Self>`

  Change the background color to bright cyan

- `fn bright_white(self: &Self) -> FgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the foreground color to bright white

- `fn on_bright_white(self: &Self) -> BgColorDisplay<'_, colors::BrightWhite, Self>`

  Change the background color to bright white

- `fn bold(self: &Self) -> styles::BoldDisplay<'_, Self>`

  Make the text bold

- `fn dimmed(self: &Self) -> styles::DimDisplay<'_, Self>`

  Make the text dim

- `fn italic(self: &Self) -> styles::ItalicDisplay<'_, Self>`

  Make the text italicized

- `fn underline(self: &Self) -> styles::UnderlineDisplay<'_, Self>`

  Make the text underlined

- `fn blink(self: &Self) -> styles::BlinkDisplay<'_, Self>`

  Make the text blink

- `fn blink_fast(self: &Self) -> styles::BlinkFastDisplay<'_, Self>`

  Make the text blink (but fast!)

- `fn reversed(self: &Self) -> styles::ReversedDisplay<'_, Self>`

  Swap the foreground and background colors

- `fn hidden(self: &Self) -> styles::HiddenDisplay<'_, Self>`

  Hide the text

- `fn strikethrough(self: &Self) -> styles::StrikeThroughDisplay<'_, Self>`

  Cross out the text

- `fn color<Color: DynColor>(self: &Self, color: Color) -> FgDynColorDisplay<'_, Color, Self>`

  Set the foreground color at runtime. Only use if you do not know which color will be used at

- `fn on_color<Color: DynColor>(self: &Self, color: Color) -> BgDynColorDisplay<'_, Color, Self>`

  Set the background color at runtime. Only use if you do not know what color to use at

- `fn fg_rgb<const R: u8, const G: u8, const B: u8>(self: &Self) -> FgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the foreground color to a specific RGB value.

- `fn bg_rgb<const R: u8, const G: u8, const B: u8>(self: &Self) -> BgColorDisplay<'_, colors::CustomColor<R, G, B>, Self>`

  Set the background color to a specific RGB value.

- `fn truecolor(self: &Self, r: u8, g: u8, b: u8) -> FgDynColorDisplay<'_, Rgb, Self>`

  Sets the foreground color to an RGB value.

- `fn on_truecolor(self: &Self, r: u8, g: u8, b: u8) -> BgDynColorDisplay<'_, Rgb, Self>`

  Sets the background color to an RGB value.

- `fn style(self: &Self, style: Style) -> Styled<&Self>`

  Apply a runtime-determined style

