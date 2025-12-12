*[anstyle](../index.md) / [style](index.md)*

---

# Module `style`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Style`](#style) | struct | ANSI Text styling |
| [`StyleDisplay`](#styledisplay) | struct |  |

## Structs

### `Style`

```rust
struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}
```

*Defined in [`anstyle-1.0.13/src/style.rs:18-23`](../../../.source_1765521767/anstyle-1.0.13/src/style.rs#L18-L23)*

ANSI Text styling

You can print a `Style` to render the corresponding ANSI code.
Using the alternate flag `#` will render the ANSI reset code, if needed.
Together, this makes it convenient to render styles using inline format arguments.

# Examples

```rust
let style = anstyle::Style::new().bold();

let value = 42;
println!("{style}{value}{style:#}");
```

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

- <span id="style-fg-color"></span>`const fn fg_color(self, fg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

- <span id="style-bg-color"></span>`const fn bg_color(self, bg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

- <span id="style-underline-color"></span>`const fn underline_color(self, underline: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

- <span id="style-effects"></span>`const fn effects(self, effects: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

- <span id="style-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

- <span id="style-fmt-to"></span>`fn fmt_to(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

- <span id="style-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

- <span id="style-render-reset"></span>`fn render_reset(self) -> impl core::fmt::Display + Copy`

- <span id="style-write-reset-to"></span>`fn write_reset_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Style`

- <span id="style-bitor-type-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl BitOrAssign for Style`

- <span id="style-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](../index.md#style)

##### `impl Display for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Style`

- <span id="style-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](../index.md#style)

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md#style)

##### `impl PartialOrd for Style`

- <span id="style-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](../index.md#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-sub-type-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl SubAssign for Style`

- <span id="style-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl ToString for Style`

- <span id="style-to-string"></span>`fn to_string(&self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

*Defined in [`anstyle-1.0.13/src/style.rs:423`](../../../.source_1765521767/anstyle-1.0.13/src/style.rs#L423)*

#### Trait Implementations

##### `impl Clone for StyleDisplay`

- <span id="styledisplay-clone"></span>`fn clone(&self) -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- <span id="styledisplay-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleDisplay`

- <span id="styledisplay-default"></span>`fn default() -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Display for StyleDisplay`

- <span id="styledisplay-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for StyleDisplay`

- <span id="styledisplay-to-string"></span>`fn to_string(&self) -> String`

