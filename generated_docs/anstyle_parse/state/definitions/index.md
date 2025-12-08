*[anstyle_parse](../../index.md) / [state](../index.md) / [definitions](index.md)*

---

# Module `definitions`

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

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](../index.md)

##### `impl Copy for State`

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for State`

- `fn default() -> State` — [`State`](../index.md)

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](../index.md)

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

#### Trait Implementations

##### `impl Clone for Action`

- `fn clone(self: &Self) -> Action` — [`Action`](../index.md)

##### `impl Copy for Action`

##### `impl Debug for Action`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Action`

- `fn default() -> Action` — [`Action`](../index.md)

##### `impl Eq for Action`

##### `impl PartialEq for Action`

- `fn eq(self: &Self, other: &Action) -> bool` — [`Action`](../index.md)

##### `impl StructuralPartialEq for Action`

## Functions

### `unpack`

```rust
const fn unpack(delta: u8) -> (State, Action)
```

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

### `ACTIONS`

```rust
const ACTIONS: [Action; 16];
```

