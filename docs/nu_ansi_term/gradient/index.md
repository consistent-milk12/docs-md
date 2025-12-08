*[nu_ansi_term](../index.md) / [gradient](index.md)*

---

# Module `gradient`

## Structs

### `Gradient`

```rust
struct Gradient {
    pub start: crate::rgb::Rgb,
    pub end: crate::rgb::Rgb,
}
```

Linear color gradient between two color stops

#### Fields

- **`start`**: `crate::rgb::Rgb`

  Start Color of Gradient

- **`end`**: `crate::rgb::Rgb`

  End Color of Gradient

#### Implementations

- `const fn new(start: Rgb, end: Rgb) -> Self` — [`Rgb`](../index.md)

- `const fn from_color_rgb(start: Color, end: Color) -> Self` — [`Color`](../style/index.md)

- `fn at(self: &Self, t: f32) -> Rgb` — [`Rgb`](../index.md)

- `const fn reverse(self: &Self) -> Self`

- `fn build(self: &Self, text: &str, target: TargetGround) -> String` — [`TargetGround`](../index.md)

#### Trait Implementations

##### `impl Clone for Gradient`

- `fn clone(self: &Self) -> Gradient` — [`Gradient`](../index.md)

##### `impl Copy for Gradient`

##### `impl Debug for Gradient`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Gradient`

##### `impl PartialEq for Gradient`

- `fn eq(self: &Self, other: &Gradient) -> bool` — [`Gradient`](../index.md)

##### `impl StructuralPartialEq for Gradient`

## Enums

### `TargetGround`

```rust
enum TargetGround {
    Foreground,
    Background,
}
```

#### Implementations

- `const fn code(self: &Self) -> u8`

#### Trait Implementations

##### `impl Clone for TargetGround`

- `fn clone(self: &Self) -> TargetGround` — [`TargetGround`](../index.md)

##### `impl Copy for TargetGround`

##### `impl Debug for TargetGround`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for TargetGround`

##### `impl PartialEq for TargetGround`

- `fn eq(self: &Self, other: &TargetGround) -> bool` — [`TargetGround`](../index.md)

##### `impl StructuralPartialEq for TargetGround`

## Traits

### `ANSIColorCode`

```rust
trait ANSIColorCode { ... }
```

#### Required Methods

- `fn ansi_color_code(self: &Self, target: TargetGround) -> String`

## Functions

### `build_all_gradient_text`

```rust
fn build_all_gradient_text(text: &str, foreground: Gradient, background: Gradient) -> alloc::string::String
```

