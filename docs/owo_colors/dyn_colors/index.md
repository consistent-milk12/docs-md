*[owo_colors](../index.md) / [dyn_colors](index.md)*

---

# Module `dyn_colors`

## Structs

### `ParseColorError`

```rust
struct ParseColorError;
```

An error for when the color can not be parsed from a string at runtime

#### Trait Implementations

##### `impl Debug for ParseColorError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<D> OwoColorize for ParseColorError`

## Enums

### `DynColors`

```rust
enum DynColors {
    Ansi(crate::AnsiColors),
    Css(crate::CssColors),
    Xterm(crate::XtermColors),
    Rgb(u8, u8, u8),
}
```

An enum describing runtime-configurable colors

This can be displayed using [`FgDynColorDisplay`](FgDynColorDisplay) or [`BgDynColorDisplay`](BgDynColorDisplay),
allowing for multiple types of colors to be used at runtime.

#### Trait Implementations

##### `impl Clone for DynColors`

- `fn clone(self: &Self) -> DynColors` — [`DynColors`](../index.md)

##### `impl Copy for DynColors`

##### `impl Debug for DynColors`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DynColor for DynColors`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DynColors`

##### `impl FromStr for DynColors`

- `type Err = ParseColorError`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<D> OwoColorize for DynColors`

##### `impl PartialEq for DynColors`

- `fn eq(self: &Self, other: &DynColors) -> bool` — [`DynColors`](../index.md)

##### `impl StructuralPartialEq for DynColors`

