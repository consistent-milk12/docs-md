*[owo_colors](../index.md) / [dyn_colors](index.md)*

---

# Module `dyn_colors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseColorError`](#parsecolorerror) | struct | An error for when the color can not be parsed from a string at runtime |
| [`DynColors`](#dyncolors) | enum | An enum describing runtime-configurable colors |

## Structs

### `ParseColorError`

```rust
struct ParseColorError;
```

An error for when the color can not be parsed from a string at runtime

#### Trait Implementations

##### `impl Debug for ParseColorError`

- <span id="parsecolorerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="dyncolors-clone"></span>`fn clone(&self) -> DynColors` — [`DynColors`](../index.md)

##### `impl Copy for DynColors`

##### `impl Debug for DynColors`

- <span id="dyncolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for DynColors`

- <span id="dyncolors-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="dyncolors-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DynColors`

##### `impl FromStr for DynColors`

- <span id="dyncolors-err"></span>`type Err = ParseColorError`

- <span id="dyncolors-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<D> OwoColorize for DynColors`

##### `impl PartialEq for DynColors`

- <span id="dyncolors-eq"></span>`fn eq(&self, other: &DynColors) -> bool` — [`DynColors`](../index.md)

##### `impl StructuralPartialEq for DynColors`

