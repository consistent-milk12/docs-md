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

- `fn clone(self: &Self) -> ColorLevel` — [`ColorLevel`](#colorlevel)

##### `impl Copy for ColorLevel`

##### `impl Debug for ColorLevel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ColorLevel`

##### `impl Hash for ColorLevel`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ColorLevel`

- `fn eq(self: &Self, other: &ColorLevel) -> bool` — [`ColorLevel`](#colorlevel)

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

- `fn clone(self: &Self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

