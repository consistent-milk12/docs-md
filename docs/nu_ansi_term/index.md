# Crate `nu_ansi_term`

This is a library for controlling colors and formatting, such as
red bold text or blue underlined text, on ANSI terminals.


## Basic usage

There are three main types in this crate that you need to be
concerned with: [`AnsiString`](#ansistring), [`Style`](style/index.md), and [`Color`](style/index.md).

A `Style` holds stylistic information: foreground and background colors,
whether the text should be bold, or blinking, or other properties. The
[`Color`](style/index.md) enum represents the available colors. And an [`AnsiString`](#ansistring) is a
string paired with a [`Style`](style/index.md).

[`Color`](style/index.md) is also available as an alias to `Color`.

To format a string, call the `paint` method on a `Style` or a `Color`,
passing in the string you want to format as the argument. For example,
here’s how to get some red text:

```rust
use nu_ansi_term::Color::Red;

println!("This is in red: {}", Red.paint("a red string"));
```

It’s important to note that the `paint` method does *not* actually return a
string with the ANSI control characters surrounding it. Instead, it returns
an [`AnsiString`](#ansistring) value that has a `Display` implementation that, when
formatted, returns the characters. This allows strings to be printed with a
minimum of [`String`](#string) allocations being performed behind the scenes.

If you *do* want to get at the escape codes, then you can convert the
[`AnsiString`](#ansistring) to a string as you would any other `Display` value:

```rust
use nu_ansi_term::Color::Red;

let red_string = Red.paint("a red string").to_string();
```


## Bold, underline, background, and other styles

For anything more complex than plain foreground color changes, you need to
construct `Style` values themselves, rather than beginning with a `Color`.
You can do this by chaining methods based on a new `Style`, created with
`Style::new()`. Each method creates a new style that has that specific
property set. For example:

```rust
use nu_ansi_term::Style;

println!("How about some {} and {}?",
         Style::new().bold().paint("bold"),
         Style::new().underline().paint("underline"));
```

For brevity, these methods have also been implemented for `Color` values,
so you can give your styles a foreground color without having to begin with
an empty `Style` value:

```rust
use nu_ansi_term::Color::{Blue, Yellow};

println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
```

The complete list of styles you can use are: `bold`, `dimmed`, `italic`,
`underline`, `blink`, `reverse`, `hidden`, `strikethrough`, and `on` for
background colors.

In some cases, you may find it easier to change the foreground on an
existing `Style` rather than starting from the appropriate `Color`.
You can do this using the `fg` method:

```rust
use nu_ansi_term::Style;
use nu_ansi_term::Color::{Blue, Cyan, Yellow};

println!("Yellow on blue: {}", Style::new().on(Blue).fg(Yellow).paint("yow!"));
println!("Also yellow on blue: {}", Cyan.on(Blue).fg(Yellow).paint("zow!"));
```

You can turn a `Color` into a `Style` with the `normal` method.
This will produce the exact same `AnsiString` as if you just used the
`paint` method on the `Color` directly, but it’s useful in certain cases:
for example, you may have a method that returns `Styles`, and need to
represent both the “red bold” and “red, but not bold” styles with values of
the same type. The `Style` struct also has a [`Default`](#default) implementation if you
want to have a style with *nothing* set.

```rust
use nu_ansi_term::Style;
use nu_ansi_term::Color::Red;

Red.normal().paint("yet another red string");
Style::default().paint("a completely regular string");
```


## Extended colors

You can access the extended range of 256 colors by using the `Color::Fixed`
variant, which takes an argument of the color number to use. This can be
included wherever you would use a `Color`:

```rust
use nu_ansi_term::Color::Fixed;

Fixed(134).paint("A sort of light purple");
Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup");
```

The first sixteen of these values are the same as the normal and bold
standard color variants. There’s nothing stopping you from using these as
`Fixed` colors instead, but there’s nothing to be gained by doing so
either.

You can also access full 24-bit color by using the `Color::Rgb` variant,
which takes separate `u8` arguments for red, green, and blue:

```rust
use nu_ansi_term::Color::Rgb;

Rgb(70, 130, 180).paint("Steel blue");
```

## Combining successive colored strings

The benefit of writing ANSI escape codes to the terminal is that they
*stack*: you do not need to end every colored string with a reset code if
the text that follows it is of a similar style. For example, if you want to
have some blue text followed by some blue bold text, it’s possible to send
the ANSI code for blue, followed by the ANSI code for bold, and finishing
with a reset code without having to have an extra one between the two
strings.

This crate can optimise the ANSI codes that get printed in situations like
this, making life easier for your terminal renderer. The [`AnsiStrings`](#ansistrings)
type takes a slice of several [`AnsiString`](#ansistring) values, and will iterate over
each of them, printing only the codes for the styles that need to be updated
as part of its formatting routine.

The following code snippet uses this to enclose a binary number displayed in
red bold text inside some red, but not bold, brackets:

```rust
use nu_ansi_term::Color::Red;
use nu_ansi_term::{AnsiString, AnsiStrings};

let some_value = format!("{:b}", 42);
let strings: &[AnsiString<'static>] = &[
    Red.paint("["),
    Red.bold().paint(some_value),
    Red.paint("]"),
];

println!("Value: {}", AnsiStrings(strings));
```

There are several things to note here. Firstly, the `paint` method can take
*either* an owned [`String`](#string) or a borrowed `&str`. Internally, an [`AnsiString`](#ansistring)
holds a copy-on-write (`Cow`) string value to deal with both owned and
borrowed strings at the same time. This is used here to display a `String`,
the result of the `format!` call, using the same mechanism as some
statically-available `&str` slices. Secondly, that the [`AnsiStrings`](#ansistrings) value
works in the same way as its singular counterpart, with a `Display`
implementation that only performs the formatting when required.

## Byte strings

This library also supports formatting `\[u8]` byte strings; this supports
applications working with text in an unknown encoding.  [`Style`](style/index.md) and
[`Color`](style/index.md) support painting `\[u8]` values, resulting in an [`AnsiByteString`](#ansibytestring).
This type does not implement `Display`, as it may not contain UTF-8, but
it does provide a method `write_to` to write the result to any value that
implements `Write`:

```rust
use nu_ansi_term::Color::Green;

Green.paint("user data".as_bytes()).write_to(&mut std::io::stdout()).unwrap();
```

Similarly, the type [`AnsiByteStrings`](#ansibytestrings) supports writing a list of
[`AnsiByteString`](#ansibytestring) values with minimal escape sequences:

```rust
use nu_ansi_term::Color::Green;
use nu_ansi_term::AnsiByteStrings;

AnsiByteStrings(&[
    Green.paint("user data 1\n".as_bytes()),
    Green.bold().paint("user data 2\n".as_bytes()),
]).write_to(&mut std::io::stdout()).unwrap();
```



























## Modules

- [`ansi`](ansi/index.md) - 
- [`gradient`](gradient/index.md) - 

## Structs

### `Infix`

```rust
struct Infix(crate::style::Style, crate::style::Style);
```

Like `AnsiString`, but only displays the difference between two
styles.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::infix`.

#### Trait Implementations

##### `impl Clone for Infix`

- `fn clone(self: &Self) -> Infix` — [`Infix`](ansi/index.md)

##### `impl Copy for Infix`

##### `impl Debug for Infix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Infix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Infix`

- `fn to_string(self: &Self) -> String`

### `Prefix`

```rust
struct Prefix(crate::style::Style);
```

Like `AnsiString`, but only displays the style prefix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::prefix`.

#### Trait Implementations

##### `impl Clone for Prefix`

- `fn clone(self: &Self) -> Prefix` — [`Prefix`](ansi/index.md)

##### `impl Copy for Prefix`

##### `impl Debug for Prefix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Prefix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Prefix`

- `fn to_string(self: &Self) -> String`

### `Suffix`

```rust
struct Suffix(crate::style::Style);
```

Like `AnsiString`, but only displays the style suffix.

This type implements the `Display` trait, meaning it can be written to a
`std::fmt` formatting without doing any extra allocation, and written to a
string with the `.to_string()` method. For examples, see
`Style::suffix`.

#### Trait Implementations

##### `impl Clone for Suffix`

- `fn clone(self: &Self) -> Suffix` — [`Suffix`](ansi/index.md)

##### `impl Copy for Suffix`

##### `impl Debug for Suffix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Suffix`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Suffix`

- `fn to_string(self: &Self) -> String`

### `Style`

```rust
struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub is_bold: bool,
    pub is_dimmed: bool,
    pub is_italic: bool,
    pub is_underline: bool,
    pub is_blink: bool,
    pub is_reverse: bool,
    pub is_hidden: bool,
    pub is_strikethrough: bool,
    pub prefix_with_reset: bool,
}
```

A style is a collection of properties that can format a string
using ANSI escape codes.

# Examples

```rust
use nu_ansi_term::{Style, Color};

let style = Style::new().bold().on(Color::Black);
println!("{}", style.paint("Bold on black"));
```

#### Fields

- **`foreground`**: `Option<Color>`

  The style's foreground color, if it has one.

- **`background`**: `Option<Color>`

  The style's background color, if it has one.

- **`is_bold`**: `bool`

  Whether this style is bold.

- **`is_dimmed`**: `bool`

  Whether this style is dimmed.

- **`is_italic`**: `bool`

  Whether this style is italic.

- **`is_underline`**: `bool`

  Whether this style is underlined.

- **`is_blink`**: `bool`

  Whether this style is blinking.

- **`is_reverse`**: `bool`

  Whether this style has reverse colors.

- **`is_hidden`**: `bool`

  Whether this style is hidden.

- **`is_strikethrough`**: `bool`

  Whether this style is struckthrough.

- **`prefix_with_reset`**: `bool`

  Wether this style is always displayed starting with a reset code to clear any remaining style artifacts

#### Implementations

- `fn write_prefix<W: AnyWrite + ?Sized>(self: &Self, f: &mut W) -> Result<(), <W as >::Error>` — [`AnyWrite`](write/index.md)

- `fn write_suffix<W: AnyWrite + ?Sized>(self: &Self, f: &mut W) -> Result<(), <W as >::Error>` — [`AnyWrite`](write/index.md)

#### Trait Implementations

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](style/index.md)

##### `impl Copy for Style`

##### `impl Debug for crate::style::Style`

- `fn fmt(self: &Self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- `fn default() -> Style` — [`Style`](style/index.md)

##### `impl Eq for Style`

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &Style) -> bool` — [`Style`](style/index.md)

##### `impl StructuralPartialEq for Style`

### `AnsiGenericString<'a, S: 'a + ToOwned + ?Sized>`

```rust
struct AnsiGenericString<'a, S: 'a + ToOwned + ?Sized>
where
    <S as ToOwned>::Owned: fmt::Debug {
    style: crate::style::Style,
    string: alloc::borrow::Cow<'a, S>,
    oscontrol: Option<OSControl<'a, S>>,
}
```

An `AnsiGenericString` includes a generic string type and a `Style` to
display that string.  `AnsiString` and `AnsiByteString` are aliases for
this type on `str` and `\[u8]`, respectively.

#### Implementations

- `const fn style_ref(self: &Self) -> &Style` — [`Style`](style/index.md)

- `fn style_ref_mut(self: &mut Self) -> &mut Style` — [`Style`](style/index.md)

- `fn as_str(self: &Self) -> &S`

- `fn title<I>(s: I) -> Self`

- `fn hyperlink<I>(self: Self, url: I) -> Self`

- `fn url_string(self: &Self) -> Option<&S>`

#### Trait Implementations

##### `impl<'a, S: 'a + ToOwned + ?Sized> Clone for AnsiGenericString<'a, S>`

- `fn clone(self: &Self) -> AnsiGenericString<'a, S>` — [`AnsiGenericString`](#ansigenericstring)

##### `impl<'a, S: $crate::fmt::Debug + 'a + ToOwned + ?Sized> Debug for AnsiGenericString<'a, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, S: $crate::cmp::Eq + 'a + ToOwned + ?Sized> Eq for AnsiGenericString<'a, S>`

##### `impl<'a, S: $crate::cmp::PartialEq + 'a + ToOwned + ?Sized> PartialEq for AnsiGenericString<'a, S>`

- `fn eq(self: &Self, other: &AnsiGenericString<'a, S>) -> bool` — [`AnsiGenericString`](#ansigenericstring)

##### `impl<'a, S: 'a + ToOwned + ?Sized> StructuralPartialEq for AnsiGenericString<'a, S>`

##### `impl<T> ToString for AnsiGenericString<'a, S>`

- `fn to_string(self: &Self) -> String`

### `AnsiGenericStrings<'a, S>`

```rust
struct AnsiGenericStrings<'a, S>(&'a [AnsiGenericString<'a, S>])
where
    <S as ToOwned>::Owned: fmt::Debug,
    S: PartialEq + 'a + ToOwned + ?Sized;
```

A set of `AnsiGenericStrings`s collected together, in order to be
written with a minimum of control characters.

#### Implementations

- `fn write_to_any<W: AnyWrite<Wstr = S> + ?Sized>(self: &Self, w: &mut W) -> Result<(), <W as >::Error>` — [`AnyWrite`](write/index.md)

#### Trait Implementations

##### `impl<'a, S> Debug for AnsiGenericStrings<'a, S>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, S> Eq for AnsiGenericStrings<'a, S>`

##### `impl<'a, S> PartialEq for AnsiGenericStrings<'a, S>`

- `fn eq(self: &Self, other: &AnsiGenericStrings<'a, S>) -> bool` — [`AnsiGenericStrings`](#ansigenericstrings)

##### `impl<'a, S> StructuralPartialEq for AnsiGenericStrings<'a, S>`

##### `impl<T> ToString for AnsiGenericStrings<'a, S>`

- `fn to_string(self: &Self) -> String`

### `Gradient`

```rust
struct Gradient {
    pub start: crate::rgb::Rgb,
    pub end: crate::rgb::Rgb,
}
```

Linear color gradient between two color stops

#### Fields

- **`start`**: `crate::rgb::Rgb`

  Start Color of Gradient

- **`end`**: `crate::rgb::Rgb`

  End Color of Gradient

#### Implementations

- `const fn new(start: Rgb, end: Rgb) -> Self` — [`Rgb`](#rgb)

- `const fn from_color_rgb(start: Color, end: Color) -> Self` — [`Color`](style/index.md)

- `fn at(self: &Self, t: f32) -> Rgb` — [`Rgb`](#rgb)

- `const fn reverse(self: &Self) -> Self`

- `fn build(self: &Self, text: &str, target: TargetGround) -> String` — [`TargetGround`](#targetground)

#### Trait Implementations

##### `impl Clone for Gradient`

- `fn clone(self: &Self) -> Gradient` — [`Gradient`](#gradient)

##### `impl Copy for Gradient`

##### `impl Debug for Gradient`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Gradient`

##### `impl PartialEq for Gradient`

- `fn eq(self: &Self, other: &Gradient) -> bool` — [`Gradient`](#gradient)

##### `impl StructuralPartialEq for Gradient`

### `Rgb`

```rust
struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

#### Fields

- **`r`**: `u8`

  Red

- **`g`**: `u8`

  Green

- **`b`**: `u8`

  Blue

#### Implementations

- `const fn new(r: u8, g: u8, b: u8) -> Self`

- `const fn from_hex(hex: u32) -> Self`

- `fn from_hex_string(hex: String) -> Self`

- `fn from_f32(r: f32, g: f32, b: f32) -> Self`

- `const fn gray(x: u8) -> Self`

- `fn gray_f32(x: f32) -> Self`

- `fn lerp(self: &Self, other: Self, t: f32) -> Self`

#### Trait Implementations

##### `impl ANSIColorCode for Rgb`

- `fn ansi_color_code(self: &Self, target: TargetGround) -> String` — [`TargetGround`](#targetground)

##### `impl Add for Rgb`

- `type Output = Rgb`

- `fn add(self: Self, rhs: Rgb) -> <Self as >::Output` — [`Rgb`](#rgb)

##### `impl Clone for Rgb`

- `fn clone(self: &Self) -> Rgb` — [`Rgb`](#rgb)

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Rgb`

##### `impl Mul for Rgb`

- `type Output = Rgb`

- `fn mul(self: Self, rhs: &f32) -> <Self as >::Output`

##### `impl Neg for Rgb`

- `type Output = Rgb`

- `fn neg(self: Self) -> <Self as >::Output`

##### `impl PartialEq for Rgb`

- `fn eq(self: &Self, other: &Rgb) -> bool` — [`Rgb`](#rgb)

##### `impl StructuralPartialEq for Rgb`

##### `impl Sub for Rgb`

- `type Output = Rgb`

- `fn sub(self: Self, rhs: Rgb) -> <Self as >::Output` — [`Rgb`](#rgb)

## Enums

### `Color`

```rust
enum Color {
    Black,
    DarkGray,
    Red,
    LightRed,
    Green,
    LightGreen,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Purple,
    LightPurple,
    Magenta,
    LightMagenta,
    Cyan,
    LightCyan,
    White,
    LightGray,
    Fixed(u8),
    Rgb(u8, u8, u8),
    Default,
}
```

A color is one specific type of ANSI escape code, and can refer
to either the foreground or background color.

These use the standard numeric sequences.
See <http://invisible-island.net/xterm/ctlseqs/ctlseqs.html>

#### Variants

- **`Black`**

  Color #0 (foreground code `30`, background code `40`).
  
  This is not necessarily the background color, and using it as one may
  render the text hard to read on terminals with dark backgrounds.

- **`DarkGray`**

  Color #0 (foreground code `90`, background code `100`).

- **`Red`**

  Color #1 (foreground code `31`, background code `41`).

- **`LightRed`**

  Color #1 (foreground code `91`, background code `101`).

- **`Green`**

  Color #2 (foreground code `32`, background code `42`).

- **`LightGreen`**

  Color #2 (foreground code `92`, background code `102`).

- **`Yellow`**

  Color #3 (foreground code `33`, background code `43`).

- **`LightYellow`**

  Color #3 (foreground code `93`, background code `103`).

- **`Blue`**

  Color #4 (foreground code `34`, background code `44`).

- **`LightBlue`**

  Color #4 (foreground code `94`, background code `104`).

- **`Purple`**

  Color #5 (foreground code `35`, background code `45`).

- **`LightPurple`**

  Color #5 (foreground code `95`, background code `105`).

- **`Magenta`**

  Color #5 (foreground code `35`, background code `45`).

- **`LightMagenta`**

  Color #5 (foreground code `95`, background code `105`).

- **`Cyan`**

  Color #6 (foreground code `36`, background code `46`).

- **`LightCyan`**

  Color #6 (foreground code `96`, background code `106`).

- **`White`**

  Color #7 (foreground code `37`, background code `47`).
  
  As above, this is not necessarily the foreground color, and may be
  hard to read on terminals with light backgrounds.

- **`LightGray`**

  Color #7 (foreground code `97`, background code `107`).

- **`Fixed`**

  A color number from 0 to 255, for use in 256-color terminal
  environments.
  
  - colors 0 to 7 are the `Black` to `White` variants respectively.
    These colors can usually be changed in the terminal emulator.
  - colors 8 to 15 are brighter versions of the eight colors above.
    These can also usually be changed in the terminal emulator, or it
    could be configured to use the original colors and show the text in
    bold instead. It varies depending on the program.
  - colors 16 to 231 contain several palettes of bright colors,
    arranged in six squares measuring six by six each.
  - colors 232 to 255 are shades of grey from black to white.
  
  It might make more sense to look at a [color chart][cc].
  

- **`Rgb`**

  A 24-bit Rgb color, as specified by ISO-8613-3.

- **`Default`**

  The default color (foreground code `39`, background codr `49`).

#### Implementations

- `fn paint<'a, I, S: 'a + ToOwned + ?Sized>(self: Self, input: I) -> AnsiGenericString<'a, S>` — [`AnsiGenericString`](#ansigenericstring)

#### Trait Implementations

##### `impl Clone for Color`

- `fn clone(self: &Self) -> Color` — [`Color`](style/index.md)

##### `impl Copy for Color`

##### `impl Debug for Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Color`

- `fn default() -> Color` — [`Color`](style/index.md)

##### `impl Eq for Color`

##### `impl PartialEq for Color`

- `fn eq(self: &Self, other: &Color) -> bool` — [`Color`](style/index.md)

##### `impl StructuralPartialEq for Color`

### `TargetGround`

```rust
enum TargetGround {
    Foreground,
    Background,
}
```

#### Implementations

- `const fn code(self: &Self) -> u8`

#### Trait Implementations

##### `impl Clone for TargetGround`

- `fn clone(self: &Self) -> TargetGround` — [`TargetGround`](#targetground)

##### `impl Copy for TargetGround`

##### `impl Debug for TargetGround`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TargetGround`

##### `impl PartialEq for TargetGround`

- `fn eq(self: &Self, other: &TargetGround) -> bool` — [`TargetGround`](#targetground)

##### `impl StructuralPartialEq for TargetGround`

## Traits

### `ANSIColorCode`

```rust
trait ANSIColorCode { ... }
```

#### Required Methods

- `fn ansi_color_code(self: &Self, target: TargetGround) -> String`

## Functions

### `AnsiStrings`

```rust
const fn AnsiStrings<'a>(arg: &'a [AnsiString<'a>]) -> AnsiStrings<'a>
```

A function to construct an `AnsiStrings` instance.

### `AnsiByteStrings`

```rust
const fn AnsiByteStrings<'a>(arg: &'a [AnsiByteString<'a>]) -> AnsiByteStrings<'a>
```

A function to construct an `AnsiByteStrings` instance.

### `sub_string`

```rust
fn sub_string(start: usize, len: usize, strs: &crate::display::AnsiStrings<'_>) -> alloc::vec::Vec<crate::display::AnsiString<'static>>
```

Return a substring of the given AnsiStrings sequence, while keeping the formatting.

### `unstyle`

```rust
fn unstyle(strs: &crate::display::AnsiStrings<'_>) -> alloc::string::String
```

Return a concatenated copy of `strs` without the formatting, as an allocated `String`.

### `unstyled_len`

```rust
fn unstyled_len(strs: &crate::display::AnsiStrings<'_>) -> usize
```

Return the unstyled length of AnsiStrings. This is equaivalent to `unstyle(strs).len()`.

### `build_all_gradient_text`

```rust
fn build_all_gradient_text(text: &str, foreground: Gradient, background: Gradient) -> alloc::string::String
```

## Type Aliases

### `AnsiString<'a>`

```rust
type AnsiString<'a> = AnsiGenericString<'a, str>;
```

An ANSI String is a string coupled with the `Style` to display it
in a terminal.

Although not technically a string itself, it can be turned into
one with the `to_string` method.

# Examples

```rust
use nu_ansi_term::AnsiString;
use nu_ansi_term::Color::Red;

let red_string = Red.paint("a red string");
println!("{}", red_string);
```

```rust
use nu_ansi_term::AnsiString;

let plain_string = AnsiString::from("a plain string");
```

### `AnsiByteString<'a>`

```rust
type AnsiByteString<'a> = AnsiGenericString<'a, [u8]>;
```

An `AnsiByteString` represents a formatted series of bytes.  Use
`AnsiByteString` when styling text with an unknown encoding.

### `AnsiStrings<'a>`

```rust
type AnsiStrings<'a> = AnsiGenericStrings<'a, str>;
```

A set of `AnsiString`s collected together, in order to be written with a
minimum of control characters.

### `AnsiByteStrings<'a>`

```rust
type AnsiByteStrings<'a> = AnsiGenericStrings<'a, [u8]>;
```

A set of `AnsiByteString`s collected together, in order to be
written with a minimum of control characters.

