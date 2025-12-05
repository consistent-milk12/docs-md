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

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Red`

```rust
struct Red;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Green`

```rust
struct Green;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Yellow`

```rust
struct Yellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Blue`

```rust
struct Blue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Magenta`

```rust
struct Magenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Cyan`

```rust
struct Cyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `White`

```rust
struct White;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `Default`

```rust
struct Default;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightBlack`

```rust
struct BrightBlack;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightRed`

```rust
struct BrightRed;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightGreen`

```rust
struct BrightGreen;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightYellow`

```rust
struct BrightYellow;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightBlue`

```rust
struct BrightBlue;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightMagenta`

```rust
struct BrightMagenta;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightCyan`

```rust
struct BrightCyan;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

### `BrightWhite`

```rust
struct BrightWhite;
```

A color for use with [`OwoColorize`](crate::OwoColorize)'s `fg` and `bg` methods.

#### Trait Implementations

##### `impl Color`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

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

##### `impl Color<const R: u8, const G: u8, const B: u8>`

- `const ANSI_FG: &'static str`

- `const ANSI_BG: &'static str`

- `const RAW_ANSI_FG: &'static str`

- `const RAW_ANSI_BG: &'static str`

##### `impl OwoColorize<D>`

