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


## Contents

- [Modules](#modules)
  - [`params`](#params)
  - [`state`](#state)
- [Structs](#structs)
  - [`Params`](#params)
  - [`ParamsIter`](#paramsiter)
  - [`Parser`](#parser)
  - [`AsciiParser`](#asciiparser)
  - [`Utf8Parser`](#utf8parser)
  - [`VtUtf8Receiver`](#vtutf8receiver)
- [Traits](#traits)
  - [`CharAccumulator`](#characcumulator)
  - [`Perform`](#perform)
- [Type Aliases](#type-aliases)
  - [`DefaultCharAccumulator`](#defaultcharaccumulator)
- [Constants](#constants)
  - [`MAX_INTERMEDIATES`](#max_intermediates)
  - [`MAX_OSC_PARAMS`](#max_osc_params)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`params`](#params) | mod | Fixed size parameters list with optional subparameters. |
| [`state`](#state) | mod | ANSI escape code parsing state machine |
| [`Params`](#params) | struct |  |
| [`ParamsIter`](#paramsiter) | struct |  |
| [`Parser`](#parser) | struct | Parser for raw _VTE_ protocol which delegates actions to a [`Perform`] |
| [`AsciiParser`](#asciiparser) | struct | Only allow parsing 7-bit ASCII |
| [`Utf8Parser`](#utf8parser) | struct | Allow parsing UTF-8 |
| [`VtUtf8Receiver`](#vtutf8receiver) | struct |  |
| [`CharAccumulator`](#characcumulator) | trait | Build a `char` out of bytes |
| [`Perform`](#perform) | trait | Performs actions requested by the [`Parser`] |
| [`DefaultCharAccumulator`](#defaultcharaccumulator) | type | Most flexible [`CharAccumulator`] for [`Parser`] based on active features |
| [`MAX_INTERMEDIATES`](#max_intermediates) | const |  |
| [`MAX_OSC_PARAMS`](#max_osc_params) | const |  |

## Modules

- [`params`](params/index.md) — Fixed size parameters list with optional subparameters.
- [`state`](state/index.md) — ANSI escape code parsing state machine

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

*Defined in [`anstyle-parse-0.2.7/src/params.rs:8-25`](../../.source_1765210505/anstyle-parse-0.2.7/src/params.rs#L8-L25)*

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

- <span id="params-len"></span>`fn len(&self) -> usize`

- <span id="params-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="params-iter"></span>`fn iter(&self) -> ParamsIter<'_>` — [`ParamsIter`](params/index.md#paramsiter)

- <span id="params-is-full"></span>`fn is_full(&self) -> bool`

- <span id="params-clear"></span>`fn clear(&mut self)`

- <span id="params-push"></span>`fn push(&mut self, item: u16)`

- <span id="params-extend"></span>`fn extend(&mut self, item: u16)`

#### Trait Implementations

##### `impl Clone for Params`

- <span id="params-clone"></span>`fn clone(&self) -> Params` — [`Params`](params/index.md#params)

##### `impl Debug for Params`

- <span id="params-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default for Params`

- <span id="params-default"></span>`fn default() -> Params` — [`Params`](params/index.md#params)

##### `impl Eq for Params`

##### `impl IntoIterator for &'a Params`

- <span id="a-params-type-intoiter"></span>`type IntoIter = ParamsIter<'a>`

- <span id="a-params-type-item"></span>`type Item = &'a [u16]`

- <span id="a-params-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Params`

- <span id="params-eq"></span>`fn eq(&self, other: &Params) -> bool` — [`Params`](params/index.md#params)

##### `impl StructuralPartialEq for Params`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
    params: &'a Params,
    index: usize,
}
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:88-91`](../../.source_1765210505/anstyle-parse-0.2.7/src/params.rs#L88-L91)*

Immutable subparameter iterator.

#### Implementations

- <span id="paramsiter-new"></span>`fn new(params: &'a Params) -> Self` — [`Params`](params/index.md#params)

#### Trait Implementations

##### `impl IntoIterator for ParamsIter<'a>`

- <span id="paramsiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="paramsiter-type-intoiter"></span>`type IntoIter = I`

- <span id="paramsiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ParamsIter<'a>`

- <span id="paramsiter-type-item"></span>`type Item = &'a [u16]`

- <span id="paramsiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="paramsiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:62-76`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L62-L76)*

Parser for raw _VTE_ protocol which delegates actions to a [`Perform`](#perform)

#### Implementations

- <span id="parser-new"></span>`fn new() -> Parser` — [`Parser`](#parser)

- <span id="parser-params"></span>`fn params(&self) -> &Params` — [`Params`](params/index.md#params)

- <span id="parser-intermediates"></span>`fn intermediates(&self) -> &[u8]`

- <span id="parser-advance"></span>`fn advance<P: Perform>(&mut self, performer: &mut P, byte: u8)`

- <span id="parser-process-utf8"></span>`fn process_utf8<P>(&mut self, performer: &mut P, byte: u8)`

- <span id="parser-perform-state-change"></span>`fn perform_state_change<P>(&mut self, performer: &mut P, state: State, action: Action, byte: u8)` — [`State`](state/definitions/index.md#state), [`Action`](state/definitions/index.md#action)

- <span id="parser-osc-dispatch"></span>`fn osc_dispatch<P: Perform>(&self, performer: &mut P, byte: u8)`

- <span id="parser-perform-action"></span>`fn perform_action<P: Perform>(&mut self, performer: &mut P, action: Action, byte: u8)` — [`Action`](state/definitions/index.md#action)

#### Trait Implementations

##### `impl<C: clone::Clone> Clone for Parser<C>`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser<C>` — [`Parser`](#parser)

##### `impl<C: fmt::Debug> Debug for Parser<C>`

- <span id="parser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<C: default::Default> Default for Parser<C>`

- <span id="parser-default"></span>`fn default() -> Parser<C>` — [`Parser`](#parser)

##### `impl<C: cmp::Eq> Eq for Parser<C>`

##### `impl<C: cmp::PartialEq> PartialEq for Parser<C>`

- <span id="parser-eq"></span>`fn eq(&self, other: &Parser<C>) -> bool` — [`Parser`](#parser)

##### `impl<C> StructuralPartialEq for Parser<C>`

### `AsciiParser`

```rust
struct AsciiParser;
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:339`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L339)*

Only allow parsing 7-bit ASCII

#### Trait Implementations

##### `impl CharAccumulator for AsciiParser`

- <span id="asciiparser-add"></span>`fn add(&mut self, _byte: u8) -> Option<char>`

##### `impl Clone for AsciiParser`

- <span id="asciiparser-clone"></span>`fn clone(&self) -> AsciiParser` — [`AsciiParser`](#asciiparser)

##### `impl Debug for AsciiParser`

- <span id="asciiparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AsciiParser`

- <span id="asciiparser-default"></span>`fn default() -> AsciiParser` — [`AsciiParser`](#asciiparser)

##### `impl Eq for AsciiParser`

##### `impl PartialEq for AsciiParser`

- <span id="asciiparser-eq"></span>`fn eq(&self, other: &AsciiParser) -> bool` — [`AsciiParser`](#asciiparser)

##### `impl StructuralPartialEq for AsciiParser`

### `Utf8Parser`

```rust
struct Utf8Parser {
    utf8_parser: utf8::Parser,
}
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:350-352`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L350-L352)*

Allow parsing UTF-8

#### Trait Implementations

##### `impl CharAccumulator for Utf8Parser`

- <span id="utf8parser-add"></span>`fn add(&mut self, byte: u8) -> Option<char>`

##### `impl Clone for Utf8Parser`

- <span id="utf8parser-clone"></span>`fn clone(&self) -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Debug for Utf8Parser`

- <span id="utf8parser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8Parser`

- <span id="utf8parser-default"></span>`fn default() -> Utf8Parser` — [`Utf8Parser`](#utf8parser)

##### `impl Eq for Utf8Parser`

##### `impl PartialEq for Utf8Parser`

- <span id="utf8parser-eq"></span>`fn eq(&self, other: &Utf8Parser) -> bool` — [`Utf8Parser`](#utf8parser)

##### `impl StructuralPartialEq for Utf8Parser`

### `VtUtf8Receiver<'a>`

```rust
struct VtUtf8Receiver<'a>(&'a mut Option<char>);
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:365`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L365)*

#### Trait Implementations

##### `impl Receiver for VtUtf8Receiver<'_>`

- <span id="vtutf8receiver-codepoint"></span>`fn codepoint(&mut self, c: char)`

- <span id="vtutf8receiver-invalid-sequence"></span>`fn invalid_sequence(&mut self)`

## Traits

### `CharAccumulator`

```rust
trait CharAccumulator: Default { ... }
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:323-328`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L323-L328)*

Build a `char` out of bytes

#### Required Methods

- `fn add(&mut self, byte: u8) -> Option<char>`

  Build a `char` out of bytes

#### Implementors

- [`AsciiParser`](#asciiparser)
- [`Utf8Parser`](#utf8parser)

### `Perform`

```rust
trait Perform { ... }
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:388-438`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L388-L438)*

Performs actions requested by the [`Parser`](#parser)

Actions in this case mean, for example, handling a CSI escape sequence describing cursor
movement, or simply printing characters to the screen.

The methods on this type correspond to actions described in
<http://vt100.net/emu/dec_ansi_parser>. I've done my best to describe them in
a useful way in my own words for completeness, but the site should be
referenced if something isn't clear. If the site disappears at some point in
the future, consider checking archive.org.

#### Provided Methods

- `fn print(&mut self, _c: char)`

  Draw a character to the screen and update states.

- `fn execute(&mut self, _byte: u8)`

  Execute a C0 or C1 control function.

- `fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8)`

  Invoked when a final character arrives in first part of device control string.

- `fn put(&mut self, _byte: u8)`

  Pass bytes as part of a device control string to the handle chosen in `hook`. C0 controls

- `fn unhook(&mut self)`

  Called when a device control string is terminated.

- `fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool)`

  Dispatch an operating system command.

- `fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _action: u8)`

  A final character has arrived for a CSI sequence

- `fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8)`

  The final character of an escape sequence has arrived.

## Type Aliases

### `DefaultCharAccumulator`

```rust
type DefaultCharAccumulator = Utf8Parser;
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:332`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L332)*

Most flexible [`CharAccumulator`](#characcumulator) for [`Parser`](#parser) based on active features

## Constants

### `MAX_INTERMEDIATES`
```rust
const MAX_INTERMEDIATES: usize = 2usize;
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:54`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L54)*

### `MAX_OSC_PARAMS`
```rust
const MAX_OSC_PARAMS: usize = 16usize;
```

*Defined in [`anstyle-parse-0.2.7/src/lib.rs:55`](../../.source_1765210505/anstyle-parse-0.2.7/src/lib.rs#L55)*

