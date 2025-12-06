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

##### `impl Clone for Action`

- `fn clone(self: &Self) -> Action` — [`Action`](definitions/index.md)

##### `impl Copy for Action`

##### `impl Debug for Action`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Action`

- `fn default() -> Action` — [`Action`](definitions/index.md)

##### `impl Eq for Action`

##### `impl PartialEq for Action`

- `fn eq(self: &Self, other: &Action) -> bool` — [`Action`](definitions/index.md)

##### `impl StructuralPartialEq for Action`

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

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](definitions/index.md)

##### `impl Copy for State`

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for State`

- `fn default() -> State` — [`State`](definitions/index.md)

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](definitions/index.md)

##### `impl StructuralPartialEq for State`

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

