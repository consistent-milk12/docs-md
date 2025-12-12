*[owo_colors](../../index.md) / [colors](../index.md) / [ansi_colors](index.md)*

---

# Module `ansi_colors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AnsiColors`](#ansicolors) | enum | Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color) or [`OwoColorize::on_color`](OwoColorize::on_color) |

## Enums

### `AnsiColors`

```rust
enum AnsiColors {
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

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

Available standard ANSI colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for AnsiColors`

- <span id="ansicolors-clone"></span>`fn clone(&self) -> AnsiColors` — [`AnsiColors`](#ansicolors)

##### `impl Copy for AnsiColors`

##### `impl Debug for AnsiColors`

- <span id="ansicolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for AnsiColors`

- <span id="ansicolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ansicolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AnsiColors`

##### `impl OwoColorize for AnsiColors`

##### `impl PartialEq for AnsiColors`

- <span id="ansicolors-eq"></span>`fn eq(&self, other: &AnsiColors) -> bool` — [`AnsiColors`](#ansicolors)

##### `impl StructuralPartialEq for AnsiColors`

