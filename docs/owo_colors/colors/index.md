*[owo_colors](../index.md) / [colors](index.md)*

---

# Module `colors`

Color types for used for being generic over the color

## Modules

- [`css`](css/index.md) - CSS named colors. Not as widely supported as standard ANSI as it relies on 48bit color support.
- [`xterm`](xterm/index.md) - XTerm 256-bit colors. Not as widely supported as standard ANSI but contains 240 more colors.

## Structs

### `Black`

```rust
struct Black;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Black`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Black`

### `Red`

```rust
struct Red;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Red`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Red`

### `Green`

```rust
struct Green;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Green`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Green`

### `Yellow`

```rust
struct Yellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Yellow`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Yellow`

### `Blue`

```rust
struct Blue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Blue`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Blue`

### `Magenta`

```rust
struct Magenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Magenta`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Magenta`

### `Cyan`

```rust
struct Cyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Cyan`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Cyan`

### `White`

```rust
struct White;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for White`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for White`

### `Default`

```rust
struct Default;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for Default`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for Default`

### `BrightBlack`

```rust
struct BrightBlack;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlack`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightBlack`

### `BrightRed`

```rust
struct BrightRed;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightRed`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightRed`

### `BrightGreen`

```rust
struct BrightGreen;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightGreen`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightGreen`

### `BrightYellow`

```rust
struct BrightYellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightYellow`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightYellow`

### `BrightBlue`

```rust
struct BrightBlue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightBlue`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightBlue`

### `BrightMagenta`

```rust
struct BrightMagenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightMagenta`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightMagenta`

### `BrightCyan`

```rust
struct BrightCyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightCyan`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightCyan`

### `BrightWhite`

```rust
struct BrightWhite;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color for BrightWhite`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl<D> OwoColorize for BrightWhite`

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

