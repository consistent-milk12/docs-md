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

- <span id="style-get-fg-color"></span>`const fn get_fg_color(self) -> Option<crate::Color>` — [`Color`](../index.md)

- <span id="style-get-bg-color"></span>`const fn get_bg_color(self) -> Option<crate::Color>` — [`Color`](../index.md)

- <span id="style-get-underline-color"></span>`const fn get_underline_color(self) -> Option<crate::Color>` — [`Color`](../index.md)

- <span id="style-get-effects"></span>`const fn get_effects(self) -> crate::Effects` — [`Effects`](../index.md)

- <span id="style-is-plain"></span>`const fn is_plain(self) -> bool`

#### Trait Implementations

##### `impl BitOr for Style`

- <span id="style-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](../index.md)

##### `impl BitOrAssign for Style`

- <span id="style-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md)

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](../index.md)

##### `impl Display for Style`

- <span id="style-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Style`

- <span id="style-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](../index.md)

##### `impl PartialEq for Style`

- <span id="style-eq"></span>`fn eq(&self, other: &crate::Effects) -> bool` — [`Effects`](../index.md)

##### `impl PartialOrd for Style`

- <span id="style-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](../index.md)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](../index.md)

##### `impl SubAssign for Style`

- <span id="style-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md)

##### `impl<T> ToString for Style`

- <span id="style-to-string"></span>`fn to_string(&self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

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

##### `impl<T> ToString for StyleDisplay`

- <span id="styledisplay-to-string"></span>`fn to_string(&self) -> String`

