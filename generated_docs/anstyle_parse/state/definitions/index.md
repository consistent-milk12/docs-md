*[anstyle_parse](../../index.md) / [state](../index.md) / [definitions](index.md)*

---

# Module `definitions`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`State`](#state) | enum |  |
| [`Action`](#action) | enum |  |
| [`unpack`](#unpack) | fn | Unpack a u8 into a State and Action |
| [`STATES`](#states) | const |  |
| [`ACTIONS`](#actions) | const |  |

## Enums

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:8-26`](../../../../.source_1765521767/anstyle-parse-0.2.7/src/state/definitions.rs#L8-L26)*

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](#state)

##### `impl Eq for State`

##### `impl PartialEq for State`

- <span id="state-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:59-77`](../../../../.source_1765521767/anstyle-parse-0.2.7/src/state/definitions.rs#L59-L77)*

#### Trait Implementations

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](#action)

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Action`

- <span id="action-default"></span>`fn default() -> Action` — [`Action`](#action)

##### `impl Eq for Action`

##### `impl PartialEq for Action`

- <span id="action-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](#action)

##### `impl StructuralPartialEq for Action`

## Functions

### `unpack`

```rust
const fn unpack(delta: u8) -> (State, Action)
```

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:115-124`](../../../../.source_1765521767/anstyle-parse-0.2.7/src/state/definitions.rs#L115-L124)*

Unpack a u8 into a State and Action

The implementation of this assumes that there are *precisely* 16 variants for both Action and
State. Furthermore, it assumes that the enums are tag-only; that is, there is no data in any
variant.

Bad things will happen if those invariants are violated.

## Constants

### `STATES`
```rust
const STATES: [State; 16];
```

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:37-54`](../../../../.source_1765521767/anstyle-parse-0.2.7/src/state/definitions.rs#L37-L54)*

### `ACTIONS`
```rust
const ACTIONS: [Action; 16];
```

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:88-105`](../../../../.source_1765521767/anstyle-parse-0.2.7/src/state/definitions.rs#L88-L105)*

