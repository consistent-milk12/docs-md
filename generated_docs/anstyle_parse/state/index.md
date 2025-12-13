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
| [`state_change`](#state-change) | fn | Transition to next [`State`] |
| [`state_change_`](#state-change) | fn |  |

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:59-77`](../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L59-L77)*

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](definitions/index.md#action)

##### `impl CloneToUninit for Action`

- <span id="action-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Action`

- <span id="action-default"></span>`fn default() -> Action` — [`Action`](definitions/index.md#action)

##### `impl Eq for Action`

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Action`

- <span id="action-partialeq-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](definitions/index.md#action)

##### `impl StructuralPartialEq for Action`

##### `impl ToOwned for Action`

- <span id="action-toowned-type-owned"></span>`type Owned = T`

- <span id="action-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="action-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:8-26`](../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L8-L26)*

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](definitions/index.md#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](definitions/index.md#state)

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](definitions/index.md#state)

##### `impl StructuralPartialEq for State`

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `state_change`

```rust
const fn state_change(state: State, byte: u8) -> (State, Action)
```

*Defined in [`anstyle-parse-0.2.7/src/state/mod.rs:25-35`](../../../.source_1765633015/anstyle-parse-0.2.7/src/state/mod.rs#L25-L35)*

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

*Defined in [`anstyle-parse-0.2.7/src/state/mod.rs:38-43`](../../../.source_1765633015/anstyle-parse-0.2.7/src/state/mod.rs#L38-L43)*

