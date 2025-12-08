*[owo_colors](../index.md) / [dyn_styles](index.md)*

---

# Module `dyn_styles`

## Structs

### `Styled<T>`

```rust
struct Styled<T> {
    target: T,
    pub style: Style,
}
```

A wrapper type which applies a [`Style`](../index.md) when displaying the inner type

#### Fields

- **`target`**: `T`

  The target value to be styled

- **`style`**: `Style`

  The style to apply to target

#### Implementations

- `const fn inner(self: &Self) -> &T`

- `const fn inner_mut(self: &mut Self) -> &mut T`

#### Trait Implementations

##### `impl<T: fmt::Binary> Binary for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Debug> Debug for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Display for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Display> IsStyled for crate::Styled<T>`

- `type Inner = T`

- `fn style(self: &Self) -> &Style` — [`Style`](../index.md)

- `fn inner(self: &Self) -> &T`

##### `impl<T: fmt::LowerExp> LowerExp for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::LowerHex> LowerHex for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Octal> Octal for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Styled<T>`

##### `impl<T: fmt::Pointer> Pointer for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::UpperExp> UpperExp for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::UpperHex> UpperHex for Styled<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `const fn new() -> Self`

- `const fn style<T>(self: &Self, target: T) -> Styled<T>` — [`Styled`](../index.md)

- `const fn fg<C: Color>(self: Self) -> Self`

- `const fn bg<C: Color>(self: Self) -> Self`

- `const fn remove_fg(self: Self) -> Self`

- `const fn remove_bg(self: Self) -> Self`

- `const fn black(self: Self) -> Self`

- `const fn on_black(self: Self) -> Self`

- `const fn red(self: Self) -> Self`

- `const fn on_red(self: Self) -> Self`

- `const fn green(self: Self) -> Self`

- `const fn on_green(self: Self) -> Self`

- `const fn yellow(self: Self) -> Self`

- `const fn on_yellow(self: Self) -> Self`

- `const fn blue(self: Self) -> Self`

- `const fn on_blue(self: Self) -> Self`

- `const fn magenta(self: Self) -> Self`

- `const fn on_magenta(self: Self) -> Self`

- `const fn purple(self: Self) -> Self`

- `const fn on_purple(self: Self) -> Self`

- `const fn cyan(self: Self) -> Self`

- `const fn on_cyan(self: Self) -> Self`

- `const fn white(self: Self) -> Self`

- `const fn on_white(self: Self) -> Self`

- `const fn default_color(self: Self) -> Self`

- `const fn on_default_color(self: Self) -> Self`

- `const fn bright_black(self: Self) -> Self`

- `const fn on_bright_black(self: Self) -> Self`

- `const fn bright_red(self: Self) -> Self`

- `const fn on_bright_red(self: Self) -> Self`

- `const fn bright_green(self: Self) -> Self`

- `const fn on_bright_green(self: Self) -> Self`

- `const fn bright_yellow(self: Self) -> Self`

- `const fn on_bright_yellow(self: Self) -> Self`

- `const fn bright_blue(self: Self) -> Self`

- `const fn on_bright_blue(self: Self) -> Self`

- `const fn bright_magenta(self: Self) -> Self`

- `const fn on_bright_magenta(self: Self) -> Self`

- `const fn bright_purple(self: Self) -> Self`

- `const fn on_bright_purple(self: Self) -> Self`

- `const fn bright_cyan(self: Self) -> Self`

- `const fn on_bright_cyan(self: Self) -> Self`

- `const fn bright_white(self: Self) -> Self`

- `const fn on_bright_white(self: Self) -> Self`

- `const fn bold(self: Self) -> Self`

- `const fn dimmed(self: Self) -> Self`

- `const fn italic(self: Self) -> Self`

- `const fn underline(self: Self) -> Self`

- `const fn blink(self: Self) -> Self`

- `const fn blink_fast(self: Self) -> Self`

- `const fn reversed(self: Self) -> Self`

- `const fn hidden(self: Self) -> Self`

- `const fn strikethrough(self: Self) -> Self`

- `const fn set_effect(self: Self, effect: Effect, to: bool) -> Self` — [`Effect`](../index.md)

- `const fn set_effects(self: Self, effects: &[Effect], to: bool) -> Self` — [`Effect`](../index.md)

- `const fn effect(self: Self, effect: Effect) -> Self` — [`Effect`](../index.md)

- `const fn remove_effect(self: Self, effect: Effect) -> Self` — [`Effect`](../index.md)

- `const fn effects(self: Self, effects: &[Effect]) -> Self` — [`Effect`](../index.md)

- `const fn remove_effects(self: Self, effects: &[Effect]) -> Self` — [`Effect`](../index.md)

- `const fn remove_all_effects(self: Self) -> Self`

- `fn color<Color: DynColor>(self: Self, color: Color) -> Self`

- `fn on_color<Color: DynColor>(self: Self, color: Color) -> Self`

- `const fn fg_rgb<const R: u8, const G: u8, const B: u8>(self: Self) -> Self`

- `const fn bg_rgb<const R: u8, const G: u8, const B: u8>(self: Self) -> Self`

- `const fn truecolor(self: Self, r: u8, g: u8, b: u8) -> Self`

- `const fn on_truecolor(self: Self, r: u8, g: u8, b: u8) -> Self`

- `const fn is_plain(self: &Self) -> bool`

- `const fn prefix_formatter(self: &Self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md)

- `const fn suffix_formatter(self: &Self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md)

- `fn fmt_prefix(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_suffix(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](../index.md)

##### `impl Copy for Style`

##### `impl Debug for Style`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Style`

- `fn default() -> Self`

##### `impl<D> OwoColorize for Style`

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &Style) -> bool` — [`Style`](../index.md)

##### `impl StructuralPartialEq for Style`

### `StyleFlags`

```rust
struct StyleFlags(u8);
```

#### Implementations

- `const fn is_plain(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for StyleFlags`

- `fn clone(self: &Self) -> StyleFlags` — [`StyleFlags`](#styleflags)

##### `impl Copy for StyleFlags`

##### `impl Debug for StyleFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StyleFlags`

- `fn default() -> Self`

##### `impl<D> OwoColorize for StyleFlags`

##### `impl PartialEq for StyleFlags`

- `fn eq(self: &Self, other: &StyleFlags) -> bool` — [`StyleFlags`](#styleflags)

##### `impl StructuralPartialEq for StyleFlags`

### `StylePrefixFormatter`

```rust
struct StylePrefixFormatter(Style);
```

Formatter for the prefix of a [`Style`](../index.md).

This is used to get the ANSI escape codes for the style without
the suffix, which is useful for formatting the prefix separately.

#### Trait Implementations

##### `impl Clone for StylePrefixFormatter`

- `fn clone(self: &Self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md)

##### `impl Copy for StylePrefixFormatter`

##### `impl Debug for StylePrefixFormatter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for StylePrefixFormatter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StylePrefixFormatter`

##### `impl PartialEq for StylePrefixFormatter`

- `fn eq(self: &Self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](../index.md)

##### `impl StructuralPartialEq for StylePrefixFormatter`

### `StyleSuffixFormatter`

```rust
struct StyleSuffixFormatter(Style);
```

Formatter for the suffix of a [`Style`](../index.md).

This is used to get the ANSI escape codes for the style without
the prefix, which is useful for formatting the suffix separately.

#### Trait Implementations

##### `impl Clone for StyleSuffixFormatter`

- `fn clone(self: &Self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md)

##### `impl Copy for StyleSuffixFormatter`

##### `impl Debug for StyleSuffixFormatter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for StyleSuffixFormatter`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StyleSuffixFormatter`

##### `impl PartialEq for StyleSuffixFormatter`

- `fn eq(self: &Self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](../index.md)

##### `impl StructuralPartialEq for StyleSuffixFormatter`

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

A runtime-configurable text effect for use with [`Style`](../index.md)

#### Trait Implementations

##### `impl Clone for Effect`

- `fn clone(self: &Self) -> Effect` — [`Effect`](../index.md)

##### `impl Copy for Effect`

##### `impl Debug for Effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for Effect`

## Functions

### `style`

```rust
const fn style() -> Style
```

Helper to create [`Style`](../index.md)s more ergonomically

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

### `color_methods!`

### `style_methods!`

### `style_flags_methods!`

### `impl_fmt!`

