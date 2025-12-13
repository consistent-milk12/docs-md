*[owo_colors](../../index.md) / [colors](../index.md) / [custom](index.md)*

---

# Module `custom`

## Contents

- [Structs](#structs)
  - [`CustomColor`](#customcolor)
- [Enums](#enums)
  - [`Plane`](#plane)
- [Functions](#functions)
  - [`generate_lookup`](#generate-lookup)
  - [`rgb_to_ansi`](#rgb-to-ansi)
  - [`rgb_to_ansi_color`](#rgb-to-ansi-color)
  - [`bytes_to_str`](#bytes-to-str)
- [Constants](#constants)
  - [`U8_TO_STR`](#u8-to-str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CustomColor`](#customcolor) | struct | A custom RGB color, determined at compile time |
| [`Plane`](#plane) | enum |  |
| [`generate_lookup`](#generate-lookup) | fn |  |
| [`rgb_to_ansi`](#rgb-to-ansi) | fn |  |
| [`rgb_to_ansi_color`](#rgb-to-ansi-color) | fn |  |
| [`bytes_to_str`](#bytes-to-str) | fn | This exists since unwrap() isn't const-safe (it invokes formatting infrastructure) |
| [`U8_TO_STR`](#u8-to-str) | const |  |

## Structs

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:83`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L83)*

A custom RGB color, determined at compile time

#### Implementations

- <span id="customcolor-const-ansi-fg-u8"></span>`const ANSI_FG_U8: [u8; 19]`

- <span id="customcolor-const-ansi-bg-u8"></span>`const ANSI_BG_U8: [u8; 19]`

- <span id="customcolor-const-raw-ansi-fg-u8"></span>`const RAW_ANSI_FG_U8: [u8; 16]`

- <span id="customcolor-const-raw-ansi-bg-u8"></span>`const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl Any for CustomColor<R, G, B>`

- <span id="customcolor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CustomColor<R, G, B>`

- <span id="customcolor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CustomColor<R, G, B>`

- <span id="customcolor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Color for CustomColor<R, G, B>`

- <span id="customcolor-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="customcolor-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="customcolor-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="customcolor-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<T> From for CustomColor<R, G, B>`

- <span id="customcolor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CustomColor<R, G, B>`

- <span id="customcolor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for CustomColor<R, G, B>`

##### `impl<U> TryFrom for CustomColor<R, G, B>`

- <span id="customcolor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="customcolor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CustomColor<R, G, B>`

- <span id="customcolor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="customcolor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Plane`

```rust
enum Plane {
    Fg,
    Bg,
}
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:21-24`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L21-L24)*

#### Trait Implementations

##### `impl Any for Plane`

- <span id="plane-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Plane`

- <span id="plane-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Plane`

- <span id="plane-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Plane`

- <span id="plane-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Plane`

- <span id="plane-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Plane`

##### `impl<U> TryFrom for Plane`

- <span id="plane-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="plane-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Plane`

- <span id="plane-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="plane-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `generate_lookup`

```rust
const fn generate_lookup() -> [[u8; 3]; 256]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:5-19`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L5-L19)*

### `rgb_to_ansi`

```rust
const fn rgb_to_ansi(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 19]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:26-52`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L26-L52)*

### `rgb_to_ansi_color`

```rust
const fn rgb_to_ansi_color(r: u8, g: u8, b: u8, plane: Plane) -> [u8; 16]
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:54-80`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L54-L80)*

### `bytes_to_str`

```rust
const fn bytes_to_str(bytes: &'static [u8]) -> &'static str
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:86-91`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L86-L91)*

This exists since unwrap() isn't const-safe (it invokes formatting infrastructure)

## Constants

### `U8_TO_STR`
```rust
const U8_TO_STR: [[u8; 3]; 256];
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:3`](../../../../.source_1765633015/owo-colors-4.2.3/src/colors/custom.rs#L3)*

