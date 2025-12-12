*[console](../index.md) / [utils](index.md)*

---

# Module `utils`

## Contents

- [Structs](#structs)
  - [`Attributes`](#attributes)
  - [`BitsIter`](#bitsiter)
  - [`Style`](#style)
  - [`StyledObject`](#styledobject)
  - [`Emoji`](#emoji)
- [Enums](#enums)
  - [`Color`](#color)
  - [`Attribute`](#attribute)
  - [`Alignment`](#alignment)
- [Functions](#functions)
  - [`default_colors_enabled`](#default-colors-enabled)
  - [`colors_enabled`](#colors-enabled)
  - [`set_colors_enabled`](#set-colors-enabled)
  - [`colors_enabled_stderr`](#colors-enabled-stderr)
  - [`set_colors_enabled_stderr`](#set-colors-enabled-stderr)
  - [`measure_text_width`](#measure-text-width)
  - [`style`](#style)
  - [`str_width`](#str-width)
  - [`char_width`](#char-width)
  - [`truncate_str`](#truncate-str)
  - [`pad_str`](#pad-str)
  - [`pad_str_with`](#pad-str-with)
- [Macros](#macros)
  - [`impl_fmt!`](#impl-fmt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attributes`](#attributes) | struct |  |
| [`BitsIter`](#bitsiter) | struct |  |
| [`Style`](#style) | struct | A stored style that can be applied. |
| [`StyledObject`](#styledobject) | struct | A formatting wrapper that can be styled for a terminal. |
| [`Emoji`](#emoji) | struct | "Intelligent" emoji formatter. |
| [`Color`](#color) | enum | A terminal color. |
| [`Attribute`](#attribute) | enum | A terminal style attribute. |
| [`Alignment`](#alignment) | enum | Defines the alignment for padding operations. |
| [`default_colors_enabled`](#default-colors-enabled) | fn |  |
| [`colors_enabled`](#colors-enabled) | fn | Returns `true` if colors should be enabled for stdout. |
| [`set_colors_enabled`](#set-colors-enabled) | fn | Forces colorization on or off for stdout. |
| [`colors_enabled_stderr`](#colors-enabled-stderr) | fn | Returns `true` if colors should be enabled for stderr. |
| [`set_colors_enabled_stderr`](#set-colors-enabled-stderr) | fn | Forces colorization on or off for stderr. |
| [`measure_text_width`](#measure-text-width) | fn | Measure the width of a string in terminal characters. |
| [`style`](#style) | fn | Wraps an object for formatting for styling. |
| [`str_width`](#str-width) | fn |  |
| [`char_width`](#char-width) | fn |  |
| [`truncate_str`](#truncate-str) | fn | Truncates a string to a certain number of characters. |
| [`pad_str`](#pad-str) | fn | Pads a string to fill a certain number of characters. |
| [`pad_str_with`](#pad-str-with) | fn | Pads a string with specific padding to fill a certain number of characters. |
| [`impl_fmt!`](#impl-fmt) | macro |  |

## Structs

### `Attributes`

```rust
struct Attributes(u16);
```

*Defined in [`console-0.16.1/src/utils.rs:157`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L157)*

#### Implementations

- <span id="attributes-new"></span>`const fn new() -> Self`

- <span id="attributes-insert"></span>`const fn insert(self, attr: Attribute) -> Self` — [`Attribute`](#attribute)

- <span id="attributes-bits"></span>`const fn bits(self) -> BitsIter` — [`BitsIter`](#bitsiter)

- <span id="attributes-attrs"></span>`fn attrs(self) -> impl Iterator<Item = Attribute>` — [`Attribute`](#attribute)

- <span id="attributes-is-empty"></span>`fn is_empty(self) -> bool`

#### Trait Implementations

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](#attributes)

##### `impl Copy for Attributes`

##### `impl Debug for Attributes`

- <span id="attributes-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Display for Attributes`

- <span id="attributes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Attributes`

##### `impl PartialEq for Attributes`

- <span id="attributes-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl StructuralPartialEq for Attributes`

##### `impl ToString for Attributes`

- <span id="attributes-to-string"></span>`fn to_string(&self) -> String`

### `BitsIter`

```rust
struct BitsIter(u16);
```

*Defined in [`console-0.16.1/src/utils.rs:198`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L198)*

#### Trait Implementations

##### `impl IntoIterator for BitsIter`

- <span id="bitsiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bitsiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bitsiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BitsIter`

- <span id="bitsiter-iterator-type-item"></span>`type Item = u16`

- <span id="bitsiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

*Defined in [`console-0.16.1/src/utils.rs:229-237`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L229-L237)*

A stored style that can be applied.

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

- <span id="style-from-dotted-str"></span>`fn from_dotted_str(s: &str) -> Self`

- <span id="style-apply-to"></span>`fn apply_to<D>(&self, val: D) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="style-force-styling"></span>`const fn force_styling(self, value: bool) -> Self`

- <span id="style-for-stderr"></span>`const fn for_stderr(self) -> Self`

- <span id="style-for-stdout"></span>`const fn for_stdout(self) -> Self`

- <span id="style-fg"></span>`const fn fg(self, color: Color) -> Self` — [`Color`](#color)

- <span id="style-bg"></span>`const fn bg(self, color: Color) -> Self` — [`Color`](#color)

- <span id="style-attr"></span>`const fn attr(self, attr: Attribute) -> Self` — [`Attribute`](#attribute)

- <span id="style-black"></span>`const fn black(self) -> Self`

- <span id="style-red"></span>`const fn red(self) -> Self`

- <span id="style-green"></span>`const fn green(self) -> Self`

- <span id="style-yellow"></span>`const fn yellow(self) -> Self`

- <span id="style-blue"></span>`const fn blue(self) -> Self`

- <span id="style-magenta"></span>`const fn magenta(self) -> Self`

- <span id="style-cyan"></span>`const fn cyan(self) -> Self`

- <span id="style-white"></span>`const fn white(self) -> Self`

- <span id="style-color256"></span>`const fn color256(self, color: u8) -> Self`

- <span id="style-bright"></span>`const fn bright(self) -> Self`

- <span id="style-on-black"></span>`const fn on_black(self) -> Self`

- <span id="style-on-red"></span>`const fn on_red(self) -> Self`

- <span id="style-on-green"></span>`const fn on_green(self) -> Self`

- <span id="style-on-yellow"></span>`const fn on_yellow(self) -> Self`

- <span id="style-on-blue"></span>`const fn on_blue(self) -> Self`

- <span id="style-on-magenta"></span>`const fn on_magenta(self) -> Self`

- <span id="style-on-cyan"></span>`const fn on_cyan(self) -> Self`

- <span id="style-on-white"></span>`const fn on_white(self) -> Self`

- <span id="style-on-color256"></span>`const fn on_color256(self, color: u8) -> Self`

- <span id="style-on-bright"></span>`const fn on_bright(self) -> Self`

- <span id="style-bold"></span>`const fn bold(self) -> Self`

- <span id="style-dim"></span>`const fn dim(self) -> Self`

- <span id="style-italic"></span>`const fn italic(self) -> Self`

- <span id="style-underlined"></span>`const fn underlined(self) -> Self`

- <span id="style-blink"></span>`const fn blink(self) -> Self`

- <span id="style-blink-fast"></span>`const fn blink_fast(self) -> Self`

- <span id="style-reverse"></span>`const fn reverse(self) -> Self`

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

#### Trait Implementations

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](#style)

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl Eq for Style`

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](#style)

##### `impl StructuralPartialEq for Style`

### `StyledObject<D>`

```rust
struct StyledObject<D> {
    style: Style,
    val: D,
}
```

*Defined in [`console-0.16.1/src/utils.rs:515-518`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L515-L518)*

A formatting wrapper that can be styled for a terminal.

#### Implementations

- <span id="styledobject-force-styling"></span>`fn force_styling(self, value: bool) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-for-stderr"></span>`fn for_stderr(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-for-stdout"></span>`const fn for_stdout(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-fg"></span>`const fn fg(self, color: Color) -> StyledObject<D>` — [`Color`](#color), [`StyledObject`](#styledobject)

- <span id="styledobject-bg"></span>`const fn bg(self, color: Color) -> StyledObject<D>` — [`Color`](#color), [`StyledObject`](#styledobject)

- <span id="styledobject-attr"></span>`const fn attr(self, attr: Attribute) -> StyledObject<D>` — [`Attribute`](#attribute), [`StyledObject`](#styledobject)

- <span id="styledobject-black"></span>`const fn black(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-red"></span>`const fn red(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-green"></span>`const fn green(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-yellow"></span>`const fn yellow(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-blue"></span>`const fn blue(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-magenta"></span>`const fn magenta(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-cyan"></span>`const fn cyan(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-white"></span>`const fn white(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-color256"></span>`const fn color256(self, color: u8) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-bright"></span>`const fn bright(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-black"></span>`const fn on_black(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-red"></span>`const fn on_red(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-green"></span>`const fn on_green(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-yellow"></span>`const fn on_yellow(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-blue"></span>`const fn on_blue(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-magenta"></span>`const fn on_magenta(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-cyan"></span>`const fn on_cyan(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-white"></span>`const fn on_white(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-color256"></span>`const fn on_color256(self, color: u8) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-on-bright"></span>`const fn on_bright(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-bold"></span>`const fn bold(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-dim"></span>`const fn dim(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-italic"></span>`const fn italic(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-underlined"></span>`const fn underlined(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-blink"></span>`const fn blink(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-blink-fast"></span>`const fn blink_fast(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-reverse"></span>`const fn reverse(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-hidden"></span>`const fn hidden(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

- <span id="styledobject-strikethrough"></span>`const fn strikethrough(self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

#### Trait Implementations

##### `impl<D: fmt::Binary> Binary for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: clone::Clone> Clone for StyledObject<D>`

- <span id="styledobject-clone"></span>`fn clone(&self) -> StyledObject<D>` — [`StyledObject`](#styledobject)

##### `impl<D: fmt::Debug> Debug for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Display> Display for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerExp> LowerExp for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::LowerHex> LowerHex for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Octal> Octal for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::Pointer> Pointer for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for StyledObject<D>`

- <span id="styledobject-to-string"></span>`fn to_string(&self) -> String`

##### `impl<D: fmt::UpperExp> UpperExp for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D: fmt::UpperHex> UpperHex for StyledObject<D>`

- <span id="styledobject-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Emoji<'a, 'b>`

```rust
struct Emoji<'a, 'b>(&'a str, &'b str);
```

*Defined in [`console-0.16.1/src/utils.rs:762`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L762)*

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

- <span id="emoji-new"></span>`fn new(emoji: &'a str, fallback: &'b str) -> Emoji<'a, 'b>` — [`Emoji`](#emoji)

#### Trait Implementations

##### `impl Clone for Emoji<'a, 'b>`

- <span id="emoji-clone"></span>`fn clone(&self) -> Emoji<'a, 'b>` — [`Emoji`](#emoji)

##### `impl Copy for Emoji<'a, 'b>`

##### `impl Display for Emoji<'_, '_>`

- <span id="emoji-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for Emoji<'a, 'b>`

- <span id="emoji-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`console-0.16.1/src/utils.rs:87-97`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L87-L97)*

A terminal color.

#### Implementations

- <span id="color-ansi-num"></span>`fn ansi_num(self) -> usize`

- <span id="color-is-color256"></span>`fn is_color256(self) -> bool`

#### Trait Implementations

##### `impl Clone for Color`

- <span id="color-clone"></span>`fn clone(&self) -> Color` — [`Color`](#color)

##### `impl Copy for Color`

##### `impl Debug for Color`

- <span id="color-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Color`

##### `impl PartialEq for Color`

- <span id="color-eq"></span>`fn eq(&self, other: &Color) -> bool` — [`Color`](#color)

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

*Defined in [`console-0.16.1/src/utils.rs:128-140`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L128-L140)*

A terminal style attribute.

#### Implementations

- <span id="attribute-const-map"></span>`const MAP: [Attribute; 9]`

#### Trait Implementations

##### `impl Clone for Attribute`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute` — [`Attribute`](#attribute)

##### `impl Copy for Attribute`

##### `impl Debug for Attribute`

- <span id="attribute-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Attribute`

##### `impl Ord for Attribute`

- <span id="attribute-cmp"></span>`fn cmp(&self, other: &Attribute) -> cmp::Ordering` — [`Attribute`](#attribute)

##### `impl PartialEq for Attribute`

- <span id="attribute-eq"></span>`fn eq(&self, other: &Attribute) -> bool` — [`Attribute`](#attribute)

##### `impl PartialOrd for Attribute`

- <span id="attribute-partial-cmp"></span>`fn partial_cmp(&self, other: &Attribute) -> option::Option<cmp::Ordering>` — [`Attribute`](#attribute)

##### `impl StructuralPartialEq for Attribute`

### `Alignment`

```rust
enum Alignment {
    Left,
    Center,
    Right,
}
```

*Defined in [`console-0.16.1/src/utils.rs:221-225`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L221-L225)*

Defines the alignment for padding operations.

#### Trait Implementations

##### `impl Clone for Alignment`

- <span id="alignment-clone"></span>`fn clone(&self) -> Alignment` — [`Alignment`](#alignment)

##### `impl Copy for Alignment`

##### `impl Debug for Alignment`

- <span id="alignment-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Alignment`

##### `impl PartialEq for Alignment`

- <span id="alignment-eq"></span>`fn eq(&self, other: &Alignment) -> bool` — [`Alignment`](#alignment)

##### `impl StructuralPartialEq for Alignment`

## Functions

### `default_colors_enabled`

```rust
fn default_colors_enabled(out: &crate::term::Term) -> bool
```

*Defined in [`console-0.16.1/src/utils.rs:15-19`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L15-L19)*

### `colors_enabled`

```rust
fn colors_enabled() -> bool
```

*Defined in [`console-0.16.1/src/utils.rs:34-36`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L34-L36)*

Returns `true` if colors should be enabled for stdout.

This honors the [clicolors spec](http://bixense.com/clicolors/).

* `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
* `CLICOLOR == 0`: Don't output ANSI color escape codes.
* `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.

### `set_colors_enabled`

```rust
fn set_colors_enabled(val: bool)
```

*Defined in [`console-0.16.1/src/utils.rs:43-45`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L43-L45)*

Forces colorization on or off for stdout.

This overrides the default for the current process and changes the return value of the
`colors_enabled` function.

### `colors_enabled_stderr`

```rust
fn colors_enabled_stderr() -> bool
```

*Defined in [`console-0.16.1/src/utils.rs:55-57`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L55-L57)*

Returns `true` if colors should be enabled for stderr.

This honors the [clicolors spec](http://bixense.com/clicolors/).

* `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped.
* `CLICOLOR == 0`: Don't output ANSI color escape codes.
* `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what.

### `set_colors_enabled_stderr`

```rust
fn set_colors_enabled_stderr(val: bool)
```

*Defined in [`console-0.16.1/src/utils.rs:64-66`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L64-L66)*

Forces colorization on or off for stderr.

This overrides the default for the current process and changes the return value of the
`colors_enabled` function.

### `measure_text_width`

```rust
fn measure_text_width(s: &str) -> usize
```

*Defined in [`console-0.16.1/src/utils.rs:69-83`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L69-L83)*

Measure the width of a string in terminal characters.

### `style`

```rust
fn style<D>(val: D) -> StyledObject<D>
```

*Defined in [`console-0.16.1/src/utils.rs:509-511`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L509-L511)*

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

*Defined in [`console-0.16.1/src/utils.rs:780-790`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L780-L790)*

### `char_width`

```rust
fn char_width(c: char) -> usize
```

*Defined in [`console-0.16.1/src/utils.rs:793-804`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L793-L804)*

### `truncate_str`

```rust
fn truncate_str<'a>(s: &'a str, width: usize, tail: &str) -> alloc::borrow::Cow<'a, str>
```

*Defined in [`console-0.16.1/src/utils.rs:817-884`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L817-L884)*

Truncates a string to a certain number of characters.

This ensures that escape codes are not screwed up in the process.
If the maximum length is hit the string will be truncated but
escapes code will still be honored.  If truncation takes place
the tail string will be appended.

### `pad_str`

```rust
fn pad_str<'a>(s: &'a str, width: usize, align: Alignment, truncate: Option<&str>) -> alloc::borrow::Cow<'a, str>
```

*Defined in [`console-0.16.1/src/utils.rs:892-899`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L892-L899)*

Pads a string to fill a certain number of characters.

This will honor ansi codes correctly and allows you to align a string
on the left, right or centered.  Additionally truncation can be enabled
by setting `truncate` to a string that should be used as a truncation
marker.

### `pad_str_with`

```rust
fn pad_str_with<'a>(s: &'a str, width: usize, align: Alignment, truncate: Option<&str>, pad: char) -> alloc::borrow::Cow<'a, str>
```

*Defined in [`console-0.16.1/src/utils.rs:906-939`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L906-L939)*

Pads a string with specific padding to fill a certain number of characters.

This will honor ansi codes correctly and allows you to align a string
on the left, right or centered.  Additionally truncation can be enabled
by setting `truncate` to a string that should be used as a truncation
marker.

## Macros

### `impl_fmt!`

*Defined in [`console-0.16.1/src/utils.rs:691-737`](../../../.source_1765521767/console-0.16.1/src/utils.rs#L691-L737)*

