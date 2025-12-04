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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Stream`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

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

