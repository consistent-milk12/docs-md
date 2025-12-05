# Crate `anstyle_parse`

Parser for implementing virtual terminal emulators

[`Parser`](#parser) is implemented according to [Paul Williams' ANSI parser
state machine]. The state machine doesn't assign meaning to the parsed data
and is thus not itself sufficient for writing a terminal emulator. Instead,
it is expected that an implementation of [`Perform`](#perform) is provided which does
something useful with the parsed data. The [`Parser`](#parser) handles the book
keeping, and the [`Perform`](#perform) gets to simply handle actions.

# Examples

For an example of using the [`Parser`](#parser) please see the examples folder. The example included
there simply logs all the actions [`Perform`](#perform) does. One quick thing to see it in action is to
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
    subparams: [u8; 32],
    params: [u16; 32],
    current_subparams: u8,
    len: usize,
}
```

#### Fields

- **`subparams`**: `[u8; 32]`

  Number of subparameters for each parameter.
  
  For each entry in the `params` slice, this stores the length of the param as number of
  subparams at the same index as the param in the `params` slice.
  
  At the subparam positions the length will always be `0`.

- **`params`**: `[u16; 32]`

  All parameters and subparameters.

- **`current_subparams`**: `u8`

  Number of suparameters in the current parameter.

- **`len`**: `usize`

  Total number of parameters and subparameters.

#### Implementations

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> ParamsIter<'_>` — [`ParamsIter`](../params/index.md)

- `fn is_full(self: &Self) -> bool`

- `fn clear(self: &mut Self)`

- `fn push(self: &mut Self, item: u16)`

- `fn extend(self: &mut Self, item: u16)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Params` — [`Params`](../params/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Params` — [`Params`](../params/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Params) -> bool` — [`Params`](../params/index.md)

##### `impl StructuralPartialEq`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
    params: &'a Params,
    index: usize,
}
```

Immutable subparameter iterator.

#### Implementations

- `fn new(params: &'a Params) -> Self` — [`Params`](../params/index.md)

#### Trait Implementations

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = &'a [u16]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Parser<C>`

```rust
struct Parser<C> {
    state: state::State,
    intermediates: [u8; 2],
    intermediate_idx: usize,
    params: Params,
    param: u16,
    osc_raw: alloc::vec::Vec<u8>,
    osc_params: [(usize, usize); 16],
    osc_num_params: usize,
    ignoring: bool,
    utf8_parser: C,
}
```

Parser for raw _VTE_ protocol which delegates actions to a [`Perform`](#perform)

#### Implementations

- `fn new() -> Parser` — [`Parser`](../index.md)

- `fn params(self: &Self) -> &Params` — [`Params`](../params/index.md)

- `fn intermediates(self: &Self) -> &[u8]`

- `fn advance<P: Perform>(self: &mut Self, performer: &mut P, byte: u8)`

- `fn process_utf8<P>(self: &mut Self, performer: &mut P, byte: u8)`

- `fn perform_state_change<P>(self: &mut Self, performer: &mut P, state: State, action: Action, byte: u8)` — [`State`](../state/definitions/index.md), [`Action`](../state/definitions/index.md)

- `fn osc_dispatch<P: Perform>(self: &Self, performer: &mut P, byte: u8)`

- `fn perform_action<P: Perform>(self: &mut Self, performer: &mut P, action: Action, byte: u8)` — [`Action`](../state/definitions/index.md)

#### Trait Implementations

##### `impl Clone<C: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Parser<C>` — [`Parser`](../index.md)

##### `impl Debug<C: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<C: $crate::default::Default>`

- `fn default() -> Parser<C>` — [`Parser`](../index.md)

##### `impl Eq<C: $crate::cmp::Eq>`

##### `impl PartialEq<C: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &Parser<C>) -> bool` — [`Parser`](../index.md)

##### `impl StructuralPartialEq<C>`

### `AsciiParser`

```rust
struct AsciiParser;
```

Only allow parsing 7-bit ASCII

#### Trait Implementations

##### `impl CharAccumulator`

- `fn add(self: &mut Self, _byte: u8) -> Option<char>`

##### `impl Clone`

- `fn clone(self: &Self) -> AsciiParser` — [`AsciiParser`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> AsciiParser` — [`AsciiParser`](../index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AsciiParser) -> bool` — [`AsciiParser`](../index.md)

##### `impl StructuralPartialEq`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8::Parser,
}
```

Allow parsing UTF-8

#### Trait Implementations

##### `impl CharAccumulator`

- `fn add(self: &mut Self, byte: u8) -> Option<char>`

##### `impl Clone`

- `fn clone(self: &Self) -> Utf8Parser` — [`Utf8Parser`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Utf8Parser` — [`Utf8Parser`](../index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Utf8Parser) -> bool` — [`Utf8Parser`](../index.md)

##### `impl StructuralPartialEq`

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

Performs actions requested by the [`Parser`](#parser)

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

Most flexible [`CharAccumulator`](#characcumulator) for [`Parser`](#parser) based on active features

