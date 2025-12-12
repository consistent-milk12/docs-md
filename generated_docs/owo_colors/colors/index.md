*[owo_colors](../index.md) / [colors](index.md)*

---

# Module `colors`

Color types for used for being generic over the color

## Contents

- [Modules](#modules)
  - [`ansi_colors`](#ansi-colors)
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
  - [`CustomColor`](#customcolor)
- [Macros](#macros)
  - [`colors!`](#colors)
  - [`impl_fmt_for!`](#impl-fmt-for)
  - [`impl_fmt_for_dyn!`](#impl-fmt-for-dyn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ansi_colors`](#ansi-colors) | mod |  |
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
| [`CustomColor`](#customcolor) | struct |  |
| [`colors!`](#colors) | macro |  |
| [`impl_fmt_for!`](#impl-fmt-for) | macro |  |
| [`impl_fmt_for_dyn!`](#impl-fmt-for-dyn) | macro |  |

## Modules

- [`ansi_colors`](ansi_colors/index.md)
- [`css`](css/index.md) — CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
- [`xterm`](xterm/index.md) — XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.
- [`custom`](custom/index.md)
- [`dynamic`](dynamic/index.md)

## Structs

### `Black`

```rust
struct Black;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Black`

- <span id="black-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="black-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="black-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="black-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Black`

### `Red`

```rust
struct Red;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Red`

- <span id="red-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="red-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="red-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="red-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Red`

### `Green`

```rust
struct Green;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Green`

- <span id="green-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="green-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="green-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="green-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Green`

### `Yellow`

```rust
struct Yellow;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Yellow`

- <span id="yellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="yellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="yellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="yellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Yellow`

### `Blue`

```rust
struct Blue;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Blue`

- <span id="blue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="blue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="blue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="blue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Blue`

### `Magenta`

```rust
struct Magenta;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Magenta`

- <span id="magenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="magenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="magenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="magenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Magenta`

### `Cyan`

```rust
struct Cyan;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Cyan`

- <span id="cyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="cyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="cyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="cyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Cyan`

### `White`

```rust
struct White;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for White`

- <span id="white-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="white-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="white-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="white-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for White`

### `Default`

```rust
struct Default;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Default`

- <span id="default-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="default-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="default-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="default-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for Default`

### `BrightBlack`

```rust
struct BrightBlack;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlack`

- <span id="brightblack-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblack-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblack-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblack-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightBlack`

### `BrightRed`

```rust
struct BrightRed;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightRed`

- <span id="brightred-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightred-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightred-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightred-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightRed`

### `BrightGreen`

```rust
struct BrightGreen;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightGreen`

- <span id="brightgreen-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightgreen-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightgreen-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightGreen`

### `BrightYellow`

```rust
struct BrightYellow;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightYellow`

- <span id="brightyellow-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightyellow-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightyellow-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightyellow-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightYellow`

### `BrightBlue`

```rust
struct BrightBlue;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlue`

- <span id="brightblue-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightblue-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightblue-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightblue-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightBlue`

### `BrightMagenta`

```rust
struct BrightMagenta;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightMagenta`

- <span id="brightmagenta-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightmagenta-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightmagenta-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightmagenta-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightMagenta`

### `BrightCyan`

```rust
struct BrightCyan;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightCyan`

- <span id="brightcyan-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightcyan-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightcyan-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightcyan-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightCyan`

### `BrightWhite`

```rust
struct BrightWhite;
```

*Defined in [`owo-colors-4.2.3/src/colors.rs:108-127`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L108-L127)*

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightWhite`

- <span id="brightwhite-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="brightwhite-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="brightwhite-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="brightwhite-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for BrightWhite`

### `CustomColor<const R: u8, const G: u8, const B: u8>`

```rust
struct CustomColor<const R: u8, const G: u8, const B: u8>;
```

*Defined in [`owo-colors-4.2.3/src/colors/custom.rs:83`](../../../.source_1765521767/owo-colors-4.2.3/src/colors/custom.rs#L83)*

A custom RGB color, determined at compile time

#### Implementations

- <span id="customcolor-const-ansi-fg-u8"></span>`const ANSI_FG_U8: [u8; 19]`

- <span id="customcolor-const-ansi-bg-u8"></span>`const ANSI_BG_U8: [u8; 19]`

- <span id="customcolor-const-raw-ansi-fg-u8"></span>`const RAW_ANSI_FG_U8: [u8; 16]`

- <span id="customcolor-const-raw-ansi-bg-u8"></span>`const RAW_ANSI_BG_U8: [u8; 16]`

#### Trait Implementations

##### `impl Color for CustomColor<R, G, B>`

- <span id="customcolor-color-const-ansi-fg"></span>`const ANSI_FG: &'static str`

- <span id="customcolor-color-const-ansi-bg"></span>`const ANSI_BG: &'static str`

- <span id="customcolor-color-const-raw-ansi-fg"></span>`const RAW_ANSI_FG: &'static str`

- <span id="customcolor-color-const-raw-ansi-bg"></span>`const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize for CustomColor<R, G, B>`

## Macros

### `colors!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:5-106`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L5-L106)*

### `impl_fmt_for!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:129-151`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L129-L151)*

### `impl_fmt_for_dyn!`

*Defined in [`owo-colors-4.2.3/src/colors.rs:165-187`](../../../.source_1765521767/owo-colors-4.2.3/src/colors.rs#L165-L187)*

