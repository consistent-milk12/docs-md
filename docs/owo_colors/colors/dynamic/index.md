*[owo_colors](../../index.md) / [colors](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Structs

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for Rgb`

- `fn clone(self: &Self) -> Rgb` — [`Rgb`](#rgb)

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DynColor for Rgb`

- `fn fmt_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_fg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn fmt_raw_ansi_bg(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Rgb`

##### `impl<D> OwoColorize for Rgb`

##### `impl PartialEq for Rgb`

- `fn eq(self: &Self, other: &Rgb) -> bool` — [`Rgb`](#rgb)

##### `impl StructuralPartialEq for Rgb`

