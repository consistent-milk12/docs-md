*[anstyle](../index.md) / [style](index.md)*

---

# Module `style`

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

- `const fn get_fg_color(self: Self) -> Option<crate::Color>` — [`Color`](../index.md)

- `const fn get_bg_color(self: Self) -> Option<crate::Color>` — [`Color`](../index.md)

- `const fn get_underline_color(self: Self) -> Option<crate::Color>` — [`Color`](../index.md)

- `const fn get_effects(self: Self) -> crate::Effects` — [`Effects`](../index.md)

- `const fn is_plain(self: Self) -> bool`

#### Trait Implementations

##### `impl BitOr for Style`

- `type Output = Style`

- `fn bitor(self: Self, rhs: crate::Effects) -> Self` — [`Effects`](../index.md)

##### `impl BitOrAssign for Style`

- `fn bitor_assign(self: &mut Self, other: crate::Effects)` — [`Effects`](../index.md)

##### `impl Clone for Style`

- `fn clone(self: &Self) -> Style` — [`Style`](../index.md)

##### `impl Copy for Style`

##### `impl Debug for Style`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Style`

- `fn default() -> Style` — [`Style`](../index.md)

##### `impl Display for Style`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for Style`

- `fn cmp(self: &Self, other: &Style) -> $crate::cmp::Ordering` — [`Style`](../index.md)

##### `impl PartialEq for Style`

- `fn eq(self: &Self, other: &crate::Effects) -> bool` — [`Effects`](../index.md)

##### `impl PartialOrd for Style`

- `fn partial_cmp(self: &Self, other: &Style) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Style`](../index.md)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- `type Output = Style`

- `fn sub(self: Self, other: crate::Effects) -> Self` — [`Effects`](../index.md)

##### `impl SubAssign for Style`

- `fn sub_assign(self: &mut Self, other: crate::Effects)` — [`Effects`](../index.md)

##### `impl<T> ToString for Style`

- `fn to_string(self: &Self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

#### Trait Implementations

##### `impl Clone for StyleDisplay`

- `fn clone(self: &Self) -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StyleDisplay`

- `fn default() -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Display for StyleDisplay`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for StyleDisplay`

- `fn to_string(self: &Self) -> String`

