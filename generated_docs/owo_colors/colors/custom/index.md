*[owo_colors](../../index.md) / [colors](../index.md) / [custom](index.md)*

---

# Module `custom`

## Contents

- [Structs](#structs)
  - [`CustomColor`](#customcolor)
- [Enums](#enums)
  - [`Plane`](#plane)
- [Functions](#functions)
  - [`generate_lookup`](#generate_lookup)
  - [`rgb_to_ansi`](#rgb_to_ansi)
  - [`rgb_to_ansi_color`](#rgb_to_ansi_color)
  - [`bytes_to_str`](#bytes_to_str)
- [Constants](#constants)
  - [`U8_TO_STR`](#u8_to_str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CustomColor`](#customcolor) | struct | A custom RGB color, determined at compile time |
| [`Plane`](#plane) | enum |  |
| [`generate_lookup`](#generate_lookup) | fn |  |
| [`rgb_to_ansi`](#rgb_to_ansi) | fn |  |
| [`rgb_to_ansi_color`](#rgb_to_ansi_color) | fn |  |
| [`bytes_to_str`](#bytes_to_str) | fn | This exists since unwrap() isn't const-safe (it invokes formatting infrastructure) |
| [`U8_TO_STR`](#u8_to_str) | const |  |

## Structs

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:83`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L83)*

A custom RGB color, determined at compile time

#### Implementations

- <span id="customcolor-const-ansi-fg-u8"></span>`const ANSI_FG_U8: [u8; 19]`

- <span id="customcolor-const-ansi-bg-u8"></span>`const ANSI_BG_U8: [u8; 19]`

- <span id="customcolor-const-raw-ansi-fg-u8"></span>`const RAW_ANSI_FG_U8: [u8; 16]`

- <span id="customcolor-const-raw-ansi-bg-u8"></span>`const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl Color for CustomColor<R, G, B>`

- <span id="customcolor-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="customcolor-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="customcolor-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="customcolor-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for CustomColor<R, G, B>`

## Enums

### `Plane`

```rust
enum Plane {
    Fg,
    Bg,
}
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:21-24`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L21-L24)*

#### Trait Implementations

##### `impl OwoColorize for Plane`

## Functions

### `generate_lookup`

```rust
const fn generate_lookup() -> [[u8; 3]; 256]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:5-19`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L5-L19)*

### `rgb_to_ansi`

```rust
const fn rgb_to_ansi(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 19]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:26-52`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L26-L52)*

### `rgb_to_ansi_color`

```rust
const fn rgb_to_ansi_color(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 16]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:54-80`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L54-L80)*

### `bytes_to_str`

```rust
const fn bytes_to_str(bytes: &'static [u8]) -> &'static str
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:86-91`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L86-L91)*

This exists since unwrap() isn't const-safe (it invokes formatting infrastructure)

## Constants

### `U8_TO_STR`
```rust
const U8_TO_STR: [[u8; 3]; 256];
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:3`](../../../../.source_1765210505/owo-colors-4.2.3/src/colors/custom.rs#L3)*

