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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:60-65`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L60-L65)*

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

- <span id="cratestyled-type-inner"></span>`type Inner = T`

- <span id="cratestyled-style"></span>`fn style(&self) -> &Style` — [`Style`](../index.md#style)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:83-88`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L83-L88)*

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

- <span id="style-style"></span>`const fn style<T>(&self, target: T) -> Styled<T>` — [`Styled`](../index.md#styled)

- <span id="style-fg"></span>`const fn fg<C: Color>(self) -> Self`

- <span id="style-bg"></span>`const fn bg<C: Color>(self) -> Self`

- <span id="style-remove-fg"></span>`const fn remove_fg(self) -> Self`

- <span id="style-remove-bg"></span>`const fn remove_bg(self) -> Self`

- <span id="style-black"></span>`const fn black(self) -> Self`

- <span id="style-on-black"></span>`const fn on_black(self) -> Self`

- <span id="style-red"></span>`const fn red(self) -> Self`

- <span id="style-on-red"></span>`const fn on_red(self) -> Self`

- <span id="style-green"></span>`const fn green(self) -> Self`

- <span id="style-on-green"></span>`const fn on_green(self) -> Self`

- <span id="style-yellow"></span>`const fn yellow(self) -> Self`

- <span id="style-on-yellow"></span>`const fn on_yellow(self) -> Self`

- <span id="style-blue"></span>`const fn blue(self) -> Self`

- <span id="style-on-blue"></span>`const fn on_blue(self) -> Self`

- <span id="style-magenta"></span>`const fn magenta(self) -> Self`

- <span id="style-on-magenta"></span>`const fn on_magenta(self) -> Self`

- <span id="style-purple"></span>`const fn purple(self) -> Self`

- <span id="style-on-purple"></span>`const fn on_purple(self) -> Self`

- <span id="style-cyan"></span>`const fn cyan(self) -> Self`

- <span id="style-on-cyan"></span>`const fn on_cyan(self) -> Self`

- <span id="style-white"></span>`const fn white(self) -> Self`

- <span id="style-on-white"></span>`const fn on_white(self) -> Self`

- <span id="style-default-color"></span>`const fn default_color(self) -> Self`

- <span id="style-on-default-color"></span>`const fn on_default_color(self) -> Self`

- <span id="style-bright-black"></span>`const fn bright_black(self) -> Self`

- <span id="style-on-bright-black"></span>`const fn on_bright_black(self) -> Self`

- <span id="style-bright-red"></span>`const fn bright_red(self) -> Self`

- <span id="style-on-bright-red"></span>`const fn on_bright_red(self) -> Self`

- <span id="style-bright-green"></span>`const fn bright_green(self) -> Self`

- <span id="style-on-bright-green"></span>`const fn on_bright_green(self) -> Self`

- <span id="style-bright-yellow"></span>`const fn bright_yellow(self) -> Self`

- <span id="style-on-bright-yellow"></span>`const fn on_bright_yellow(self) -> Self`

- <span id="style-bright-blue"></span>`const fn bright_blue(self) -> Self`

- <span id="style-on-bright-blue"></span>`const fn on_bright_blue(self) -> Self`

- <span id="style-bright-magenta"></span>`const fn bright_magenta(self) -> Self`

- <span id="style-on-bright-magenta"></span>`const fn on_bright_magenta(self) -> Self`

- <span id="style-bright-purple"></span>`const fn bright_purple(self) -> Self`

- <span id="style-on-bright-purple"></span>`const fn on_bright_purple(self) -> Self`

- <span id="style-bright-cyan"></span>`const fn bright_cyan(self) -> Self`

- <span id="style-on-bright-cyan"></span>`const fn on_bright_cyan(self) -> Self`

- <span id="style-bright-white"></span>`const fn bright_white(self) -> Self`

- <span id="style-on-bright-white"></span>`const fn on_bright_white(self) -> Self`

- <span id="style-bold"></span>`const fn bold(self) -> Self`

- <span id="style-dimmed"></span>`const fn dimmed(self) -> Self`

- <span id="style-italic"></span>`const fn italic(self) -> Self`

- <span id="style-underline"></span>`const fn underline(self) -> Self`

- <span id="style-blink"></span>`const fn blink(self) -> Self`

- <span id="style-blink-fast"></span>`const fn blink_fast(self) -> Self`

- <span id="style-reversed"></span>`const fn reversed(self) -> Self`

- <span id="style-hidden"></span>`const fn hidden(self) -> Self`

- <span id="style-strikethrough"></span>`const fn strikethrough(self) -> Self`

- <span id="style-set-effect"></span>`const fn set_effect(self, effect: Effect, to: bool) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-set-effects"></span>`const fn set_effects(self, effects: &[Effect], to: bool) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-effect"></span>`const fn effect(self, effect: Effect) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-remove-effect"></span>`const fn remove_effect(self, effect: Effect) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-effects"></span>`const fn effects(self, effects: &[Effect]) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-remove-effects"></span>`const fn remove_effects(self, effects: &[Effect]) -> Self` — [`Effect`](../index.md#effect)

- <span id="style-remove-all-effects"></span>`const fn remove_all_effects(self) -> Self`

- <span id="style-color"></span>`fn color<Color: DynColor>(self, color: Color) -> Self`

- <span id="style-on-color"></span>`fn on_color<Color: DynColor>(self, color: Color) -> Self`

- <span id="style-fg-rgb"></span>`const fn fg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

- <span id="style-bg-rgb"></span>`const fn bg_rgb<const R: u8, const G: u8, const B: u8>(self) -> Self`

- <span id="style-truecolor"></span>`const fn truecolor(self, r: u8, g: u8, b: u8) -> Self`

- <span id="style-on-truecolor"></span>`const fn on_truecolor(self, r: u8, g: u8, b: u8) -> Self`

- <span id="style-is-plain"></span>`const fn is_plain(&self) -> bool`

- <span id="style-prefix-formatter"></span>`const fn prefix_formatter(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

- <span id="style-suffix-formatter"></span>`const fn suffix_formatter(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

- <span id="style-fmt-prefix"></span>`fn fmt_prefix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="style-fmt-suffix"></span>`fn fmt_suffix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Self`

##### `impl OwoColorize for Style`

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md#style)

##### `impl StructuralPartialEq for Style`

### `StyleFlags`

```rust
struct StyleFlags(u8);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:92`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L92)*

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

##### `impl OwoColorize for StyleFlags`

##### `impl PartialEq for StyleFlags`

- <span id="styleflags-eq"></span>`fn eq(&self, other: &StyleFlags) -> bool` — [`StyleFlags`](#styleflags)

##### `impl StructuralPartialEq for StyleFlags`

### `StylePrefixFormatter`

```rust
struct StylePrefixFormatter(Style);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:597`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L597)*

Formatter for the prefix of a [`Style`](../index.md).

This is used to get the ANSI escape codes for the style without
the suffix, which is useful for formatting the prefix separately.

#### Trait Implementations

##### `impl Clone for StylePrefixFormatter`

- <span id="styleprefixformatter-clone"></span>`fn clone(&self) -> StylePrefixFormatter` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

##### `impl Copy for StylePrefixFormatter`

##### `impl Debug for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StylePrefixFormatter`

- <span id="styleprefixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for StylePrefixFormatter`

##### `impl PartialEq for StylePrefixFormatter`

- <span id="styleprefixformatter-eq"></span>`fn eq(&self, other: &StylePrefixFormatter) -> bool` — [`StylePrefixFormatter`](../index.md#styleprefixformatter)

##### `impl StructuralPartialEq for StylePrefixFormatter`

### `StyleSuffixFormatter`

```rust
struct StyleSuffixFormatter(Style);
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:611`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L611)*

Formatter for the suffix of a [`Style`](../index.md).

This is used to get the ANSI escape codes for the style without
the prefix, which is useful for formatting the suffix separately.

#### Trait Implementations

##### `impl Clone for StyleSuffixFormatter`

- <span id="stylesuffixformatter-clone"></span>`fn clone(&self) -> StyleSuffixFormatter` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

##### `impl Copy for StyleSuffixFormatter`

##### `impl Debug for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StyleSuffixFormatter`

- <span id="stylesuffixformatter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for StyleSuffixFormatter`

##### `impl PartialEq for StyleSuffixFormatter`

- <span id="stylesuffixformatter-eq"></span>`fn eq(&self, other: &StyleSuffixFormatter) -> bool` — [`StyleSuffixFormatter`](../index.md#stylesuffixformatter)

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

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:10-20`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L10-L20)*

A runtime-configurable text effect for use with [`Style`](../index.md)

#### Trait Implementations

##### `impl Clone for Effect`

- <span id="effect-clone"></span>`fn clone(&self) -> Effect` — [`Effect`](../index.md#effect)

##### `impl Copy for Effect`

##### `impl Debug for Effect`

- <span id="effect-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Effect`

## Functions

### `style`

```rust
const fn style() -> Style
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:626-628`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L626-L628)*

Helper to create [`Style`](../index.md)s more ergonomically

## Constants

### `DIMMED_SHIFT`
```rust
const DIMMED_SHIFT: u8 = 0u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:102`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L102)*

### `ITALIC_SHIFT`
```rust
const ITALIC_SHIFT: u8 = 1u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:103`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L103)*

### `UNDERLINE_SHIFT`
```rust
const UNDERLINE_SHIFT: u8 = 2u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:104`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L104)*

### `BLINK_SHIFT`
```rust
const BLINK_SHIFT: u8 = 3u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:105`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L105)*

### `BLINK_FAST_SHIFT`
```rust
const BLINK_FAST_SHIFT: u8 = 4u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:106`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L106)*

### `REVERSED_SHIFT`
```rust
const REVERSED_SHIFT: u8 = 5u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:107`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L107)*

### `HIDDEN_SHIFT`
```rust
const HIDDEN_SHIFT: u8 = 6u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:108`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L108)*

### `STRIKETHROUGH_SHIFT`
```rust
const STRIKETHROUGH_SHIFT: u8 = 7u8;
```

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:109`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L109)*

## Macros

### `color_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:22-42`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L22-L42)*

### `style_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:44-55`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L44-L55)*

### `style_flags_methods!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:111-126`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L111-L126)*

### `impl_fmt!`

*Defined in [`owo-colors-4.2.3/src/dyn_styles.rs:653-666`](../../../.source_1765210505/owo-colors-4.2.3/src/dyn_styles.rs#L653-L666)*

