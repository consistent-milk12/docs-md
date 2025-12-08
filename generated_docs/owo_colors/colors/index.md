*[owo_colors](../index.md) / [colors](index.md)*

---

# Module `colors`

Color types for used for being generic over the color

## Contents

- [Modules](#modules)
  - [`ansi_colors`](#ansi_colors)
  - [`css`](#css)
  - [`xterm`](#xterm)
  - [`custom`](#custom)
  - [`dynamic`](#dynamic)
- [Structs](#structs)
  - [`Black`](#black)
  - [`Red`](#red)
  - [`Green`](#green)
  - [`Yellow`](#yellow)
  - [`Blue`](#blue)
  - [`Magenta`](#magenta)
  - [`Cyan`](#cyan)
  - [`White`](#white)
  - [`Default`](#default)
  - [`BrightBlack`](#brightblack)
  - [`BrightRed`](#brightred)
  - [`BrightGreen`](#brightgreen)
  - [`BrightYellow`](#brightyellow)
  - [`BrightBlue`](#brightblue)
  - [`BrightMagenta`](#brightmagenta)
  - [`BrightCyan`](#brightcyan)
  - [`BrightWhite`](#brightwhite)
  - [`unnamed`](#unnamed)
- [Macros](#macros)
  - [`colors!`](#colors)
  - [`impl_fmt_for!`](#impl_fmt_for)
  - [`impl_fmt_for_dyn!`](#impl_fmt_for_dyn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ansi_colors`](#ansi_colors) | mod |  |
| [`css`](#css) | mod | CSS named colors. |
| [`xterm`](#xterm) | mod | XTerm 256-bit colors. |
| [`custom`](#custom) | mod |  |
| [`dynamic`](#dynamic) | mod |  |
| [`Black`](#black) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Red`](#red) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Green`](#green) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Yellow`](#yellow) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Blue`](#blue) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Magenta`](#magenta) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Cyan`](#cyan) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`White`](#white) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`Default`](#default) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightBlack`](#brightblack) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightRed`](#brightred) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightGreen`](#brightgreen) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightYellow`](#brightyellow) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightBlue`](#brightblue) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightMagenta`](#brightmagenta) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightCyan`](#brightcyan) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`BrightWhite`](#brightwhite) | struct | A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods. |
| [`unnamed`](#unnamed) | struct |  |
| [`colors!`](#colors) | macro |  |
| [`impl_fmt_for!`](#impl_fmt_for) | macro |  |
| [`impl_fmt_for_dyn!`](#impl_fmt_for_dyn) | macro |  |

## Modules

- [`ansi_colors`](ansi_colors/index.md) - 
- [`css`](css/index.md) - CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
- [`xterm`](xterm/index.md) - XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.
- [`custom`](custom/index.md) - 
- [`dynamic`](dynamic/index.md) - 

## Structs

### `Black`

```rust
struct Black;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Black`

- <span id="black-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="black-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Black`

### `Red`

```rust
struct Red;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Red`

- <span id="red-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="red-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Red`

### `Green`

```rust
struct Green;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Green`

- <span id="green-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="green-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Green`

### `Yellow`

```rust
struct Yellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Yellow`

- <span id="yellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="yellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Yellow`

### `Blue`

```rust
struct Blue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Blue`

- <span id="blue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="blue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Blue`

### `Magenta`

```rust
struct Magenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Magenta`

- <span id="magenta-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="magenta-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="magenta-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="magenta-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Magenta`

### `Cyan`

```rust
struct Cyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Cyan`

- <span id="cyan-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="cyan-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Cyan`

### `White`

```rust
struct White;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for White`

- <span id="white-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="white-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for White`

### `Default`

```rust
struct Default;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Default`

- <span id="default-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="default-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="default-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="default-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Default`

### `BrightBlack`

```rust
struct BrightBlack;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlack`

- <span id="brightblack-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblack-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblack-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblack-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightBlack`

### `BrightRed`

```rust
struct BrightRed;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightRed`

- <span id="brightred-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightred-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightRed`

### `BrightGreen`

```rust
struct BrightGreen;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightGreen`

- <span id="brightgreen-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightgreen-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightGreen`

### `BrightYellow`

```rust
struct BrightYellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightYellow`

- <span id="brightyellow-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightyellow-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightyellow-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightyellow-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightYellow`

### `BrightBlue`

```rust
struct BrightBlue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlue`

- <span id="brightblue-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblue-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblue-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblue-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightBlue`

### `BrightMagenta`

```rust
struct BrightMagenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightMagenta`

- <span id="brightmagenta-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightmagenta-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightmagenta-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightmagenta-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightMagenta`

### `BrightCyan`

```rust
struct BrightCyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightCyan`

- <span id="brightcyan-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightcyan-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightcyan-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightcyan-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightCyan`

### `BrightWhite`

```rust
struct BrightWhite;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightWhite`

- <span id="brightwhite-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightwhite-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightwhite-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightwhite-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightWhite`

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

A custom RGB color, determined at compile time

#### Implementations

- <span id="customcolor-ansi-fg-u8"></span>`const ANSI_FG_U8: [u8; 19]`

- <span id="customcolor-ansi-bg-u8"></span>`const ANSI_BG_U8: [u8; 19]`

- <span id="customcolor-raw-ansi-fg-u8"></span>`const RAW_ANSI_FG_U8: [u8; 16]`

- <span id="customcolor-raw-ansi-bg-u8"></span>`const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl<const R: u8, const G: u8, const B: u8> Color for CustomColor<R, G, B>`

- <span id="customcolor-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="customcolor-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="customcolor-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="customcolor-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for CustomColor<R, G, B>`

## Macros

### `colors!`

### `impl_fmt_for!`

### `impl_fmt_for_dyn!`

