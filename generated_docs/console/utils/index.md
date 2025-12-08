*[console](../index.md) / [utils](index.md)*

---

# Module `utils`

## Structs

### `Attributes`

```rust
struct Attributes(u16);
```

#### Implementations

- `const fn new() -> Self`

- `const fn insert(self: Self, attr: Attribute) -> Self` — [`Attribute`](../index.md)

- `const fn bits(self: Self) -> BitsIter` — [`BitsIter`](#bitsiter)

- `fn attrs(self: Self) -> impl Iterator<Item = Attribute>` — [`Attribute`](../index.md)

- `fn is_empty(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for Attributes`

- `fn clone(self: &Self) -> Attributes` — [`Attributes`](#attributes)

##### `impl Copy for Attributes`

##### `impl Debug for Attributes`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Display for Attributes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Attributes`

##### `impl PartialEq for Attributes`

- `fn eq(self: &Self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl StructuralPartialEq for Attributes`

##### `impl<T> ToString for Attributes`

- `fn to_string(self: &Self) -> String`

### `BitsIter`

```rust
struct BitsIter(u16);
```

#### Trait Implementations

##### `impl<I> IntoIterator for BitsIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for BitsIter`

- `type Item = u16`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `Style`

```rust
struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    fg_bright: bool,
    bg_bright: bool,
    attrs: Attributes,
    force: Option<bool>,
    for_stderr: bool,
}
```

A stored style that can be applied.

#### Implementations

- `const fn new() -> Self`

- `fn from_dotted_str(s: &str) -> Self`

- `fn apply_to<D>(self: &Self, val: D) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn force_styling(self: Self, value: bool) -> Self`

- `const fn for_stderr(self: Self) -> Self`

- `const fn for_stdout(self: Self) -> Self`

- `const fn fg(self: Self, color: Color) -> Self` — [`Color`](../index.md)

- `const fn bg(self: Self, color: Color) -> Self` — [`Color`](../index.md)

- `const fn attr(self: Self, attr: Attribute) -> Self` — [`Attribute`](../index.md)

- `const fn black(self: Self) -> Self`

- `const fn red(self: Self) -> Self`

- `const fn green(self: Self) -> Self`

- `const fn yellow(self: Self) -> Self`

- `const fn blue(self: Self) -> Self`

- `const fn magenta(self: Self) -> Self`

- `const fn cyan(self: Self) -> Self`

- `const fn white(self: Self) -> Self`

- `const fn color256(self: Self, color: u8) -> Self`

- `const fn bright(self: Self) -> Self`

- `const fn on_black(self: Self) -> Self`

- `const fn on_red(self: Self) -> Self`

- `const fn on_green(self: Self) -> Self`

- `const fn on_yellow(self: Self) -> Self`

- `const fn on_blue(self: Self) -> Self`

- `const fn on_magenta(self: Self) -> Self`

- `const fn on_cyan(self: Self) -> Self`

- `const fn on_white(self: Self) -> Self`

- `const fn on_color256(self: Self, color: u8) -> Self`

- `const fn on_bright(self: Self) -> Self`

- `const fn bold(self: Self) -> Self`

- `const fn dim(self: Self) -> Self`

- `const fn italic(self: Self) -> Self`

- `const fn underlined(self: Self) -> Self`

- `const fn blink(self: Self) -> Self`

- `const fn blink_fast(self: Self) -> Self`

- `const fn reverse(self: Self) -> Self`

- `const fn hidden(self: Self) -> Self`

- `const fn strikethrough(self: Self) -> Self`

#### Trait Implementations

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](../index.md)

##### `impl Debug for Style`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Style`

- `fn default() -> Self`

##### `impl Eq for Style`

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &Style) -> bool` — [`Style`](../index.md)

##### `impl StructuralPartialEq for Style`

### `StyledObject<D>`

```rust
struct StyledObject<D> {
    style: Style,
    val: D,
}
```

A formatting wrapper that can be styled for a terminal.

#### Implementations

- `fn force_styling(self: Self, value: bool) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `fn for_stderr(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn for_stdout(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn fg(self: Self, color: Color) -> StyledObject<D>` — [`Color`](../index.md), [`StyledObject`](../index.md)

- `const fn bg(self: Self, color: Color) -> StyledObject<D>` — [`Color`](../index.md), [`StyledObject`](../index.md)

- `const fn attr(self: Self, attr: Attribute) -> StyledObject<D>` — [`Attribute`](../index.md), [`StyledObject`](../index.md)

- `const fn black(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn red(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn green(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn yellow(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn blue(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn magenta(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn cyan(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn white(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn color256(self: Self, color: u8) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn bright(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_black(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_red(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_green(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_yellow(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_blue(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_magenta(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_cyan(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_white(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_color256(self: Self, color: u8) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn on_bright(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn bold(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn dim(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn italic(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn underlined(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn blink(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn blink_fast(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn reverse(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn hidden(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

- `const fn strikethrough(self: Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

#### Trait Implementations

##### `impl<D: fmt::Binary> Binary for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: $crate::clone::Clone> Clone for StyledObject<D>`

- `fn clone(self: &Self) -> StyledObject<D>` — [`StyledObject`](../index.md)

##### `impl<D: fmt::Debug> Debug for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Display> Display for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerExp> LowerExp for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerHex> LowerHex for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Octal> Octal for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Pointer> Pointer for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for StyledObject<D>`

- `fn to_string(self: &Self) -> String`

##### `impl<D: fmt::UpperExp> UpperExp for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::UpperHex> UpperHex for StyledObject<D>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Emoji<'a, 'b>`

```rust
struct Emoji<'a, 'b>(&'a str, &'b str);
```

"Intelligent" emoji formatter.

This struct intelligently wraps an emoji so that it is rendered
only on systems that want emojis and renders a fallback on others.

Example:

```rust
use console::Emoji;
println!("[3/4] {}Downloading ...", Emoji("🚚 ", ""));
println!("[4/4] {} Done!", Emoji("✨", ":-)"));
```

#### Implementations

- `fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b>` — [`Emoji`](../index.md)

#### Trait Implementations

##### `impl<'a, 'b> Clone for Emoji<'a, 'b>`

- `fn clone(self: &Self) -> Emoji<'a, 'b>` — [`Emoji`](../index.md)

##### `impl<'a, 'b> Copy for Emoji<'a, 'b>`

##### `impl Display for Emoji<'_, '_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Emoji<'a, 'b>`

- `fn to_string(self: &Self) -> String`

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
    Color256(u8),
}
```

A terminal color.

#### Implementations

- `fn ansi_num(self: Self) -> usize`

- `fn is_color256(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for Color`

- `fn clone(self: &Self) -> Color` — [`Color`](../index.md)

##### `impl Copy for Color`

##### `impl Debug for Color`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Color`

##### `impl PartialEq for Color`

- `fn eq(self: &Self, other: &Color) -> bool` — [`Color`](../index.md)

##### `impl StructuralPartialEq for Color`

### `Attribute`

```rust
enum Attribute {
    Bold,
    Dim,
    Italic,
    Underlined,
    Blink,
    BlinkFast,
    Reverse,
    Hidden,
    StrikeThrough,
}
```

A terminal style attribute.

#### Implementations

- `const MAP: [Attribute; 9]`

#### Trait Implementations

##### `impl Clone for Attribute`

- `fn clone(self: &Self) -> Attribute` — [`Attribute`](../index.md)

##### `impl Copy for Attribute`

##### `impl Debug for Attribute`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Attribute`

##### `impl Ord for Attribute`

- `fn cmp(self: &Self, other: &Attribute) -> $crate::cmp::Ordering` — [`Attribute`](../index.md)

##### `impl PartialEq for Attribute`

- `fn eq(self: &Self, other: &Attribute) -> bool` — [`Attribute`](../index.md)

##### `impl PartialOrd for Attribute`

- `fn partial_cmp(self: &Self, other: &Attribute) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Attribute`](../index.md)

##### `impl StructuralPartialEq for Attribute`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

Defines the alignment for padding operations.

#### Trait Implementations

##### `impl Clone for Alignment`

- `fn clone(self: &Self) -> Alignment` — [`Alignment`](../index.md)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- `fn eq(self: &Self, other: &Alignment) -> bool` — [`Alignment`](../index.md)

##### `impl StructuralPartialEq for Alignment`

## Functions

### `default_colors_enabled`

```rust
fn default_colors_enabled(out: &crate::term::Term) -> bool
```

### `colors_enabled`

```rust
fn colors_enabled() -> bool
```

Returns `true` if colors should be enabled for stdout.

This honors the [clicolors spec](http://bixense.com/clicolors/).

* `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
* `CLICOLOR == 0`: Don't output ANSI color escape codes.
* `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.

### `set_colors_enabled`

```rust
fn set_colors_enabled(val: bool)
```

Forces colorization on or off for stdout.

This overrides the default for the current process and changes the return value of the
`colors_enabled` function.

### `colors_enabled_stderr`

```rust
fn colors_enabled_stderr() -> bool
```

Returns `true` if colors should be enabled for stderr.

This honors the [clicolors spec](http://bixense.com/clicolors/).

* `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
* `CLICOLOR == 0`: Don't output ANSI color escape codes.
* `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.

### `set_colors_enabled_stderr`

```rust
fn set_colors_enabled_stderr(val: bool)
```

Forces colorization on or off for stderr.

This overrides the default for the current process and changes the return value of the
`colors_enabled` function.

### `measure_text_width`

```rust
fn measure_text_width(s: &str) -> usize
```

Measure the width of a string in terminal characters.

### `style`

```rust
fn style<D>(val: D) -> StyledObject<D>
```

Wraps an object for formatting for styling.

Example:

```rust,no_run
use console::style;
format!("Hello {}", style("World").cyan());
```

This is a shortcut for making a new style and applying it
to a value:

```rust,no_run
use console::Style;
format!("Hello {}", Style::new().cyan().apply_to("World"));
```

### `str_width`

```rust
fn str_width(s: &str) -> usize
```

### `char_width`

```rust
fn char_width(c: char) -> usize
```

### `truncate_str`

```rust
fn truncate_str<'a>(s: &'a str, width: usize, tail: &str) -> alloc::borrow::Cow<'a, str>
```

Truncates a string to a certain number of characters.

This ensures that escape codes are not screwed up in the process.
If the maximum length is hit the string will be truncated but
escapes code will still be honored.  If truncation takes place
the tail string will be appended.

### `pad_str`

```rust
fn pad_str<'a>(s: &'a str, width: usize, align: Alignment, truncate: Option<&str>) -> alloc::borrow::Cow<'a, str>
```

Pads a string to fill a certain number of characters.

This will honor ansi codes correctly and allows you to align a string
on the left, right or centered.  Additionally truncation can be enabled
by setting `truncate` to a string that should be used as a truncation
marker.

### `pad_str_with`

```rust
fn pad_str_with<'a>(s: &'a str, width: usize, align: Alignment, truncate: Option<&str>, pad: char) -> alloc::borrow::Cow<'a, str>
```

Pads a string with specific padding to fill a certain number of characters.

This will honor ansi codes correctly and allows you to align a string
on the left, right or centered.  Additionally truncation can be enabled
by setting `truncate` to a string that should be used as a truncation
marker.

## Macros

### `impl_fmt!`

