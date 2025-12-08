# Crate `supports_color`

Detects whether a terminal supports color, and gives details about that
support. It takes into account the `NO_COLOR` environment variable.

This crate is a Rust port of [@sindresorhus](https://github.com/sindresorhus)'
[NPM package by the same name](https://npm.im/supports-color).

## Example

```rust
use supports_color::Stream;

if let Some(support) = supports_color::on(Stream::Stdout) {
    if support.has_16m {
        println!("16 million (RGB) colors are supported");
    } else if support.has_256 {
        println!("256-bit colors are supported.");
    } else if support.has_basic {
        println!("Only basic ANSI colors are supported.");
    }
} else {
    println!("No color support.");
}
```

## Contents

- [Structs](#structs)
  - [`ColorLevel`](#colorlevel)
- [Enums](#enums)
  - [`Stream`](#stream)
- [Functions](#functions)
  - [`env_force_color`](#env_force_color)
  - [`env_no_color`](#env_no_color)
  - [`as_str`](#as_str)
  - [`translate_level`](#translate_level)
  - [`is_a_tty`](#is_a_tty)
  - [`supports_color`](#supports_color)
  - [`check_ansi_color`](#check_ansi_color)
  - [`check_colorterm_16m`](#check_colorterm_16m)
  - [`check_term_16m`](#check_term_16m)
  - [`check_256_color`](#check_256_color)
  - [`on`](#on)
  - [`on_cached`](#on_cached)
- [Macros](#macros)
  - [`assert_stream_in_bounds!`](#assert_stream_in_bounds)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ColorLevel`](#colorlevel) | struct | Color level support details. |
| [`Stream`](#stream) | enum | possible stream sources |
| [`env_force_color`](#env_force_color) | fn |  |
| [`env_no_color`](#env_no_color) | fn |  |
| [`as_str`](#as_str) | fn |  |
| [`translate_level`](#translate_level) | fn |  |
| [`is_a_tty`](#is_a_tty) | fn |  |
| [`supports_color`](#supports_color) | fn |  |
| [`check_ansi_color`](#check_ansi_color) | fn |  |
| [`check_colorterm_16m`](#check_colorterm_16m) | fn |  |
| [`check_term_16m`](#check_term_16m) | fn |  |
| [`check_256_color`](#check_256_color) | fn |  |
| [`on`](#on) | fn | Returns a [ColorLevel] if a [Stream] supports terminal colors. |
| [`on_cached`](#on_cached) | fn | Returns a [ColorLevel] if a [Stream] supports terminal colors, caching the result to |
| [`assert_stream_in_bounds!`](#assert_stream_in_bounds) | macro |  |

## Structs

### `ColorLevel`

```rust
struct ColorLevel {
    level: usize,
    pub has_basic: bool,
    pub has_256: bool,
    pub has_16m: bool,
}
```

Color level support details.

This type is returned from [on](#on). See documentation for its fields for more details.

#### Fields

- **`has_basic`**: `bool`

  Basic ANSI colors are supported.

- **`has_256`**: `bool`

  256-bit colors are supported.

- **`has_16m`**: `bool`

  16 million (RGB) colors are supported.

#### Trait Implementations

##### `impl Clone for ColorLevel`

- <span id="colorlevel-clone"></span>`fn clone(&self) -> ColorLevel` — [`ColorLevel`](#colorlevel)

##### `impl Copy for ColorLevel`

##### `impl Debug for ColorLevel`

- <span id="colorlevel-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColorLevel`

##### `impl Hash for ColorLevel`

- <span id="colorlevel-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ColorLevel`

- <span id="colorlevel-eq"></span>`fn eq(&self, other: &ColorLevel) -> bool` — [`ColorLevel`](#colorlevel)

##### `impl StructuralPartialEq for ColorLevel`

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

possible stream sources

#### Trait Implementations

##### `impl Clone for Stream`

- <span id="stream-clone"></span>`fn clone(&self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- <span id="stream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `env_force_color`

```rust
fn env_force_color() -> usize
```

### `env_no_color`

```rust
fn env_no_color() -> bool
```

### `as_str`

```rust
fn as_str<E>(option: &Result<String, E>) -> Result<&str, &E>
```

### `translate_level`

```rust
fn translate_level(level: usize) -> Option<ColorLevel>
```

### `is_a_tty`

```rust
fn is_a_tty(stream: Stream) -> bool
```

### `supports_color`

```rust
fn supports_color(stream: Stream) -> usize
```

### `check_ansi_color`

```rust
fn check_ansi_color(term: Option<&str>) -> bool
```

### `check_colorterm_16m`

```rust
fn check_colorterm_16m(colorterm: &str) -> bool
```

### `check_term_16m`

```rust
fn check_term_16m(term: &str) -> bool
```

### `check_256_color`

```rust
fn check_256_color(term: &str) -> bool
```

### `on`

```rust
fn on(stream: Stream) -> Option<ColorLevel>
```

Returns a [ColorLevel] if a [Stream] supports terminal colors.

### `on_cached`

```rust
fn on_cached(stream: Stream) -> Option<ColorLevel>
```

Returns a [ColorLevel] if a [Stream] supports terminal colors, caching the result to
be returned from then on.

If you expect your environment to change between calls, use [`on`](#on)

## Macros

### `assert_stream_in_bounds!`

