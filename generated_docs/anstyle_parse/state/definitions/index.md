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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:8-26`](../../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L8-L26)*

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for State`

- <span id="state-default"></span>`fn default() -> State` — [`State`](#state)

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

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:59-77`](../../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L59-L77)*

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](#action)

##### `impl CloneToUninit for Action`

- <span id="action-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Action`

- <span id="action-default"></span>`fn default() -> Action` — [`Action`](#action)

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

- <span id="action-partialeq-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](#action)

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

## Functions

### `unpack`

```rust
const fn unpack(delta: u8) -> (State, Action)
```

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:115-124`](../../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L115-L124)*

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

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:37-54`](../../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L37-L54)*

### `ACTIONS`
```rust
const ACTIONS: [Action; 16];
```

*Defined in [`anstyle-parse-0.2.7/src/state/definitions.rs:88-105`](../../../../.source_1765633015/anstyle-parse-0.2.7/src/state/definitions.rs#L88-L105)*

