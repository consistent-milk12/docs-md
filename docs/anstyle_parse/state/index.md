*[anstyle_parse](../index.md) / [state](index.md)*

---

# Module `state`

ANSI escape code parsing state machine

## Enums

### `Action`

```rust
enum Action {
    Nop,
    Clear,
    Collect,
    CsiDispatch,
    EscDispatch,
    Execute,
    Hook,
    Ignore,
    OscEnd,
    OscPut,
    OscStart,
    Param,
    Print,
    Put,
    Unhook,
    BeginUtf8,
}
```

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Action` — [`Action`](../../state/definitions/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Action` — [`Action`](../../state/definitions/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Action) -> bool` — [`Action`](../../state/definitions/index.md)

##### `impl StructuralPartialEq`

### `State`

```rust
enum State {
    Anywhere,
    CsiEntry,
    CsiIgnore,
    CsiIntermediate,
    CsiParam,
    DcsEntry,
    DcsIgnore,
    DcsIntermediate,
    DcsParam,
    DcsPassthrough,
    Escape,
    EscapeIntermediate,
    Ground,
    OscString,
    SosPmApcString,
    Utf8,
}
```

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> State` — [`State`](../../state/definitions/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> State` — [`State`](../../state/definitions/index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](../../state/definitions/index.md)

##### `impl StructuralPartialEq`

## Functions

### `state_change`

```rust
const fn state_change(state: State, byte: u8) -> (State, Action)
```

Transition to next [`State`](definitions/index.md)

Note: This does not directly support UTF-8.
- If the data is validated as UTF-8 (e.g. `str`) or single-byte C1 control codes are
  unsupported, then treat `Action::BeginUtf8` and `Action::Execute` for UTF-8 continuations
  as `Action::Print`.
- If the data is not validated, then a UTF-8 state machine will need to be implemented on top,
  starting with `Action::BeginUtf8`.

Note: When `State::Anywhere` is returned, revert back to the prior state.

