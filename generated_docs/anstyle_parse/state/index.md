*[anstyle_parse](../index.md) / [state](index.md)*

---

# Module `state`

ANSI escape code parsing state machine

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`definitions`](#definitions) | mod |  |
| [`table`](#table) | mod |  |
| [`Action`](#action) | enum |  |
| [`State`](#state) | enum |  |
| [`state_change`](#state_change) | fn | Transition to next [`State`] |
| [`state_change_`](#state_change_) | fn |  |

## Modules

- [`definitions`](definitions/index.md)
- [`table`](table/index.md)

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:59-77`](../../../.source_1765210505/anstyle-parse-0.2.7/src/state/definitions.rs#L59-L77)*

#### Trait Implementations

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](definitions/index.md)

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Action`

- <span id="action-default"></span>`fn default() -> Action` — [`Action`](definitions/index.md)

##### `impl Eq for Action`

##### `impl PartialEq for Action`

- <span id="action-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](definitions/index.md)

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:8-26`](../../../.source_1765210505/anstyle-parse-0.2.7/src/state/definitions.rs#L8-L26)*

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](definitions/index.md)

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](definitions/index.md)

##### `impl Eq for State`

##### `impl PartialEq for State`

- <span id="state-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](definitions/index.md)

##### `impl StructuralPartialEq for State`

## Functions

### `state_change`

```rust
const fn state_change(state: State, byte: u8) -> (State, Action)
```

*Defined in [`anstyle-parse-0.2.7/src/state/mod.rs:25-35`](../../../.source_1765210505/anstyle-parse-0.2.7/src/state/mod.rs#L25-L35)*

Transition to next [`State`](definitions/index.md)

Note: This does not directly support UTF-8.
- If the data is validated as UTF-8 (e.g. `str`) or single-byte C1 control codes are
  unsupported, then treat [`Action::BeginUtf8`](../index.md) and [`Action::Execute`](../index.md) for UTF-8 continuations
  as [`Action::Print`](../index.md).
- If the data is not validated, then a UTF-8 state machine will need to be implemented on top,
  starting with [`Action::BeginUtf8`](../index.md).

Note: When [`State::Anywhere`](../index.md) is returned, revert back to the prior state.

### `state_change_`

```rust
const fn state_change_(state: State, byte: u8) -> u8
```

*Defined in [`anstyle-parse-0.2.7/src/state/mod.rs:38-43`](../../../.source_1765210505/anstyle-parse-0.2.7/src/state/mod.rs#L38-L43)*

