*[owo_colors](../../index.md) / [colors](../index.md) / [dynamic](index.md)*

---

# Module `dynamic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Rgb`](#rgb) | struct | Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color) |

## Structs

### `Rgb`

```rust
struct Rgb(u8, u8, u8);
```

Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
or [`OwoColorize::on_color`](OwoColorize::on_color)

#### Trait Implementations

##### `impl Clone for Rgb`

- <span id="rgb-clone"></span>`fn clone(&self) -> Rgb` — [`Rgb`](../../index.md)

##### `impl Copy for Rgb`

##### `impl Debug for Rgb`

- <span id="rgb-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DynColor for Rgb`

- <span id="rgb-fmt-ansi-fg"></span>`fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-ansi-bg"></span>`fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-raw-ansi-fg"></span>`fn fmt_raw_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="rgb-fmt-raw-ansi-bg"></span>`fn fmt_raw_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Rgb`

##### `impl<D> OwoColorize for Rgb`

##### `impl PartialEq for Rgb`

- <span id="rgb-eq"></span>`fn eq(&self, other: &Rgb) -> bool` — [`Rgb`](../../index.md)

##### `impl StructuralPartialEq for Rgb`

