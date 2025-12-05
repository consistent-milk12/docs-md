*[owo_colors](../index.md) / [colored](index.md)*

---

# Module `colored`

Module for drop-in [`colored`](https://docs.rs/colored) support to aid in porting code from
[`colored`](https://docs.rs/colored) to owo-colors.

Just replace:

```rust
mod colored {}
use colored::*;
```

with

```rust
use owo_colors::colored::*;
```

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
    Default,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
```

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> AnsiColors` — [`AnsiColors`](../../colors/ansi_colors/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DynColor`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl OwoColorize<D>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AnsiColors) -> bool` — [`AnsiColors`](../../colors/ansi_colors/index.md)

##### `impl StructuralPartialEq`

## Traits

