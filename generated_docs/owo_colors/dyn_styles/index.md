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
  - [`DIMMED_SHIFT`](#dimmed_shift)
  - [`ITALIC_SHIFT`](#italic_shift)
  - [`UNDERLINE_SHIFT`](#underline_shift)
  - [`BLINK_SHIFT`](#blink_shift)
  - [`BLINK_FAST_SHIFT`](#blink_fast_shift)
  - [`REVERSED_SHIFT`](#reversed_shift)
  - [`HIDDEN_SHIFT`](#hidden_shift)
  - [`STRIKETHROUGH_SHIFT`](#strikethrough_shift)
- [Macros](#macros)
  - [`color_methods!`](#color_methods)
  - [`style_methods!`](#style_methods)
  - [`style_flags_methods!`](#style_flags_methods)
  - [`impl_fmt!`](#impl_fmt)

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
| [`DIMMED_SHIFT`](#dimmed_shift) | const |  |
| [`ITALIC_SHIFT`](#italic_shift) | const |  |
| [`UNDERLINE_SHIFT`](#underline_shift) | const |  |
| [`BLINK_SHIFT`](#blink_shift) | const |  |
| [`BLINK_FAST_SHIFT`](#blink_fast_shift) | const |  |
| [`REVERSED_SHIFT`](#reversed_shift) | const |  |
| [`HIDDEN_SHIFT`](#hidden_shift) | const |  |
| [`STRIKETHROUGH_SHIFT`](#strikethrough_shift) | const |  |
| [`color_methods!`](#color_methods) | macro |  |
| [`style_methods!`](#style_methods) | macro |  |
| [`style_flags_methods!`](#style_flags_methods) | macro |  |
| [`impl_fmt!`](#impl_fmt) | macro |  |

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

- <span id="cratestyled-style"></span>`fn style(&self) -> &Style` — [`Style`](../index.md)

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

- <span id="cratestyle-transition-from"></span>`fn transition_from(self: &'a Self, from: &Style) -> Transition<'a>` — [`Style`](../index.md), [`Transition`](../styled_list/index.md)

#### Trait Implementations

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl<D> OwoColorize for Style`

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md)

##### `impl StructuralPartialEq for Style`

### `StyleFlags`

```rust
struct StyleFlags(u8);
```

#### Implementations

- <span id="styleflags-is-plain"></span>`const fn is_plain(&self) -> bool`

#### Trait Implementations

##### `impl Clone for StyleFlags`

- <span id="styleflags-clone"></span>`fn clone(&self) -> StyleFlags` — [`StyleFlags`](#styleflags)

##### `impl Copy for StyleFlags`

##### `impl Debug for StyleFlags`

- <span id="styleflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleFlags`

- <span id="styleflags-default"></span>`fn default() -> Self`

##### `impl<D> OwoColorize for StyleFlags`

##### `impl PartialEq for StyleFlags`

- <span id="styleflags-eq"></span>`fn eq(&self, other: &StyleFlags) -> bool` — [`StyleFlags`](#styleflags)

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

- <span id="styleprefixformatter-clone"></span>`fn clone(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md)

##### `impl Copy for StylePrefixFormatter`

##### `impl Debug for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StylePrefixFormatter`

##### `impl PartialEq for StylePrefixFormatter`

- <span id="styleprefixformatter-eq"></span>`fn eq(&self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](../index.md)

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

- <span id="stylesuffixformatter-clone"></span>`fn clone(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md)

##### `impl Copy for StyleSuffixFormatter`

##### `impl Debug for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StyleSuffixFormatter`

##### `impl PartialEq for StyleSuffixFormatter`

- <span id="stylesuffixformatter-eq"></span>`fn eq(&self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](../index.md)

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

- <span id="effect-clone"></span>`fn clone(&self) -> Effect` — [`Effect`](../index.md)

##### `impl Copy for Effect`

##### `impl Debug for Effect`

- <span id="effect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

