*[owo_colors](../../index.md) / [colors](../index.md) / [custom](index.md)*

---

# Module `custom`

## Structs

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

A custom RGB color, determined at compile time

#### Implementations

- `const ANSI_FG_U8: [u8; 19]`

- `const ANSI_BG_U8: [u8; 19]`

- `const RAW_ANSI_FG_U8: [u8; 16]`

- `const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl<const R: u8, const G: u8, const B: u8> Color for CustomColor<R, G, B>`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for CustomColor<R, G, B>`

## Enums

### `Plane`

```rust
enum Plane {
    Fg,
    Bg,
}
```

#### Trait Implementations

##### `impl<D> OwoColorize for Plane`

## Functions

### `generate_lookup`

```rust
const fn generate_lookup() -> [[u8; 3]; 256]
```

### `rgb_to_ansi`

```rust
const fn rgb_to_ansi(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 19]
```

### `rgb_to_ansi_color`

```rust
const fn rgb_to_ansi_color(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 16]
```

### `bytes_to_str`

```rust
const fn bytes_to_str(bytes: &'static [u8]) -> &'static str
```

This exists since unwrap() isn't const-safe (it invokes formatting infrastructure)

## Constants

### `U8_TO_STR`

```rust
const U8_TO_STR: [[u8; 3]; 256];
```

