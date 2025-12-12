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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Stream`](#stream) | enum | possible stream sources |
| [`supports_hyperlinks`](#supports-hyperlinks) | fn | Returns true if the current terminal, detected through various environment variables, is known to support hyperlink rendering. |
| [`is_a_tty`](#is-a-tty) | fn |  |
| [`on`](#on) | fn | Returns true if `stream` is a TTY, and the current terminal [supports_hyperlinks]. |

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

*Defined in [`supports-hyperlinks-3.1.0/src/lib.rs:5-8`](../../.source_1765521767/supports-hyperlinks-3.1.0/src/lib.rs#L5-L8)*

possible stream sources

#### Trait Implementations

##### `impl Clone for Stream`

- <span id="stream-clone"></span>`fn clone(&self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- <span id="stream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `supports_hyperlinks`

```rust
fn supports_hyperlinks() -> bool
```

*Defined in [`supports-hyperlinks-3.1.0/src/lib.rs:12-53`](../../.source_1765521767/supports-hyperlinks-3.1.0/src/lib.rs#L12-L53)*

Returns true if the current terminal, detected through various environment
variables, is known to support hyperlink rendering.

### `is_a_tty`

```rust
fn is_a_tty(stream: Stream) -> bool
```

*Defined in [`supports-hyperlinks-3.1.0/src/lib.rs:55-61`](../../.source_1765521767/supports-hyperlinks-3.1.0/src/lib.rs#L55-L61)*

### `on`

```rust
fn on(stream: Stream) -> bool
```

*Defined in [`supports-hyperlinks-3.1.0/src/lib.rs:65-67`](../../.source_1765521767/supports-hyperlinks-3.1.0/src/lib.rs#L65-L67)*

Returns true if `stream` is a TTY, and the current terminal
[supports_hyperlinks](#supports-hyperlinks).

