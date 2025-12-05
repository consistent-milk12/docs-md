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

##### `impl Clone`

- `fn clone(self: &Self) -> ColorLevel` — [`ColorLevel`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ColorLevel) -> bool` — [`ColorLevel`](../index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> Stream` — [`Stream`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

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

