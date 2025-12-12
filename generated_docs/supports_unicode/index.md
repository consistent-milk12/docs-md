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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Stream`](#stream) | enum | possible stream sources |
| [`is_a_tty`](#is-a-tty) | fn |  |
| [`on`](#on) | fn | Returns true if `stream` is a TTY or the current terminal [supports_unicode]. |
| [`supports_unicode`](#supports-unicode) | fn | Returns true if the current terminal, detected through various environment variables, is known to support unicode rendering. |

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

*Defined in [`supports-unicode-3.0.0/src/lib.rs:5-8`](../../.source_1765210505/supports-unicode-3.0.0/src/lib.rs#L5-L8)*

possible stream sources

#### Trait Implementations

##### `impl Clone for Stream`

- <span id="stream-clone"></span>`fn clone(&self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- <span id="stream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `is_a_tty`

```rust
fn is_a_tty(stream: Stream) -> bool
```

*Defined in [`supports-unicode-3.0.0/src/lib.rs:10-16`](../../.source_1765210505/supports-unicode-3.0.0/src/lib.rs#L10-L16)*

### `on`

```rust
fn on(stream: Stream) -> bool
```

*Defined in [`supports-unicode-3.0.0/src/lib.rs:20-27`](../../.source_1765210505/supports-unicode-3.0.0/src/lib.rs#L20-L27)*

Returns true if `stream` is a TTY or the current terminal
[supports_unicode](#supports-unicode).

### `supports_unicode`

```rust
fn supports_unicode() -> bool
```

*Defined in [`supports-unicode-3.0.0/src/lib.rs:31-52`](../../.source_1765210505/supports-unicode-3.0.0/src/lib.rs#L31-L52)*

Returns true if the current terminal, detected through various environment
variables, is known to support unicode rendering.

