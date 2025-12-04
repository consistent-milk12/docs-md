# Crate `anstyle_parse`

Parser for implementing virtual terminal emulators

[`Parser`](anstyle_parse/index.md) is implemented according to [Paul Williams' ANSI parser
state machine]. The state machine doesn't assign meaning to the parsed data
and is thus not itself sufficient for writing a terminal emulator. Instead,
it is expected that an implementation of [`Perform`](anstyle_parse/index.md) is provided which does
something useful with the parsed data. The [`Parser`](anstyle_parse/index.md) handles the book
keeping, and the [`Perform`](anstyle_parse/index.md) gets to simply handle actions.

# Examples

For an example of using the [`Parser`](anstyle_parse/index.md) please see the examples folder. The example included
there simply logs all the actions [`Perform`](anstyle_parse/index.md) does. One quick thing to see it in action is to
pipe `vim` into it

```sh
cargo build --release --example parselog
vim | target/release/examples/parselog
```

Just type `:q` to exit.

# Differences from original state machine description

* UTF-8 Support for Input
* OSC Strings can be terminated by 0x07
* Only supports 7-bit codes. Some 8-bit codes are still supported, but they no longer work in
  all states.

[Paul Williams' ANSI parser state machine]: https://vt100.net/emu/dec_ansi_parser

## Modules

- [`state`](state/index.md) - ANSI escape code parsing state machine

## Structs

### `Params`

```rust
struct Params {
}
```

#### Implementations

- `fn len(self: &Self) -> usize`
  Returns the number of parameters.

- `fn is_empty(self: &Self) -> bool`
  Returns `true` if there are no parameters present.

- `fn iter(self: &Self) -> ParamsIter<'_>`
  Returns an iterator over all parameters and subparameters.

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

- `fn clone(self: &Self) -> Params`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Params) -> bool`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Params`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
}
```

Immutable subparameter iterator.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a>`

- `type Item = &'a [u16]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Parser<C>`

```rust
struct Parser<C> {
}
```

Parser for raw _VTE_ protocol which delegates actions to a [`Perform`](anstyle_parse/index.md)

#### Implementations

- `fn new() -> Parser`
  Create a new Parser

- `fn advance<P: Perform>(self: &mut Self, performer: &mut P, byte: u8)`
  Advance the parser state

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

##### `impl Clone<C: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Parser<C>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<C: $crate::cmp::Eq>`

##### `impl PartialEq<C: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &Parser<C>) -> bool`

##### `impl StructuralPartialEq<C>`

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

##### `impl Debug<C: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<C: $crate::default::Default>`

- `fn default() -> Parser<C>`

### `AsciiParser`

```rust
struct AsciiParser;
```

Only allow parsing 7-bit ASCII

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

##### `impl CharAccumulator`

- `fn add(self: &mut Self, _byte: u8) -> Option<char>`

##### `impl Clone`

- `fn clone(self: &Self) -> AsciiParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AsciiParser) -> bool`

##### `impl StructuralPartialEq`

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

##### `impl Default`

- `fn default() -> AsciiParser`

### `Utf8Parser`

```rust
struct Utf8Parser {
}
```

Allow parsing UTF-8

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

##### `impl CharAccumulator`

- `fn add(self: &mut Self, byte: u8) -> Option<char>`

##### `impl Clone`

- `fn clone(self: &Self) -> Utf8Parser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Utf8Parser) -> bool`

##### `impl StructuralPartialEq`

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

##### `impl Default`

- `fn default() -> Utf8Parser`

## Traits

### `CharAccumulator`

```rust
trait CharAccumulator: Default { ... }
```

Build a `char` out of bytes

#### Required Methods

- `fn add(self: &mut Self, byte: u8) -> Option<char>`

  Build a `char` out of bytes

### `Perform`

```rust
trait Perform { ... }
```

Performs actions requested by the [`Parser`](anstyle_parse/index.md)

Actions in this case mean, for example, handling a CSI escape sequence describing cursor
movement, or simply printing characters to the screen.

The methods on this type correspond to actions described in
<http://vt100.net/emu/dec_ansi_parser>. I've done my best to describe them in
a useful way in my own words for completeness, but the site should be
referenced if something isn't clear. If the site disappears at some point in
the future, consider checking archive.org.

#### Required Methods

- `fn print(self: &mut Self, _c: char)`

  Draw a character to the screen and update states.

- `fn execute(self: &mut Self, _byte: u8)`

  Execute a C0 or C1 control function.

- `fn hook(self: &mut Self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8)`

  Invoked when a final character arrives in first part of device control string.

- `fn put(self: &mut Self, _byte: u8)`

  Pass bytes as part of a device control string to the handle chosen in `hook`. C0 controls

- `fn unhook(self: &mut Self)`

  Called when a device control string is terminated.

- `fn osc_dispatch(self: &mut Self, _params: &[&[u8]], _bell_terminated: bool)`

  Dispatch an operating system command.

- `fn csi_dispatch(self: &mut Self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8)`

  A final character has arrived for a CSI sequence

- `fn esc_dispatch(self: &mut Self, _intermediates: &[u8], _ignore: bool, _byte: u8)`

  The final character of an escape sequence has arrived.

## Type Aliases

### `DefaultCharAccumulator`

```rust
type DefaultCharAccumulator = Utf8Parser;
```

Most flexible [`CharAccumulator`](anstyle_parse/index.md) for [`Parser`](anstyle_parse/index.md) based on active features

