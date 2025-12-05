# Crate `supports_hyperlinks`

Detects whether the current terminal supports [hyperlinks in terminal
emulators](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

It tries to detect and support all known terminals and terminal families that
support this. If a declaration is wrong, missing, or could be improved, please
send a PR!

## Example

The API is super simple!

```rust
use supports_hyperlinks::Stream;

if supports_hyperlinks::on(Stream::Stdout) {
    println!("This terminal supports hyperlinks on stdout");
} else {
    println!("No hyperlinks, please");
}
```

And that's it!

## Forcing hyperlinks in tools that use `supports-hyperlinks`

You may set the `FORCE_HYPERLINK` environment variable to force
`supports-hyperlinks` to return true for its checks. If the value is `0`, this
will force it to be _false_, instead.

## MSRV

The minimum supported Rust version is 1.70.0.

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

### `supports_hyperlinks`

```rust
fn supports_hyperlinks() -> bool
```

Returns true if the current terminal, detected through various environment
variables, is known to support hyperlink rendering.

### `on`

```rust
fn on(stream: Stream) -> bool
```

Returns true if `stream` is a TTY, and the current terminal
[supports_hyperlinks](#supports-hyperlinks).

