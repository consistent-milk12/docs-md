# Crate `supports_unicode`

Detects whether a terminal supports unicode.

This crate is a Rust port mashing together
[@sindresorhus](https://github.com/sindresorhus)'
[`is-unicode-supported`](https://npm.im/is-unicode-supported) and
[@iarna](https://github.com/iarna)'s
[`has-unicode`](https://npm.im/has-unicode) NPM packages.

## Example

```rust
use supports_unicode::Stream;

if supports_unicode::on(Stream::Stdout) {
    println!("stdout supports unicode output");
} else {
    println!("no unicode, please");
}
```

## MSRV

This crate requires rustc 1.70.0 or later.

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

### `on`

```rust
fn on(stream: Stream) -> bool
```

Returns true if `stream` is a TTY or the current terminal
[supports_unicode](#supports-unicode).

### `supports_unicode`

```rust
fn supports_unicode() -> bool
```

Returns true if the current terminal, detected through various environment
variables, is known to support unicode rendering.

