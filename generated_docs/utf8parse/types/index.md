*[utf8parse](../index.md) / [types](index.md)*

---

# Module `types`

Types supporting the UTF-8 parser

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Action`](#action) | enum | Action to take when receiving a byte |
| [`State`](#state) | enum | States the parser can be in. |

## Enums

### `Action`

```rust
enum Action {
    InvalidSequence,
    EmitByte,
    SetByte1,
    SetByte2,
    SetByte2Top,
    SetByte3,
    SetByte3Top,
    SetByte4,
}
```

*Defined in [`utf8parse-0.2.2/src/types.rs:5-22`](../../../.source_1765521767/utf8parse-0.2.2/src/types.rs#L5-L22)*

Action to take when receiving a byte

#### Variants

- **`InvalidSequence`**

  Unexpected byte; sequence is invalid

- **`EmitByte`**

  Received valid 7-bit ASCII byte which can be directly emitted.

- **`SetByte1`**

  Set the bottom continuation byte

- **`SetByte2`**

  Set the 2nd-from-last continuation byte

- **`SetByte2Top`**

  Set the 2nd-from-last byte which is part of a two byte sequence

- **`SetByte3`**

  Set the 3rd-from-last continuation byte

- **`SetByte3Top`**

  Set the 3rd-from-last byte which is part of a three byte sequence

- **`SetByte4`**

  Set the top byte of a four byte sequence.

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

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `State`

```rust
enum State {
    Ground,
    Tail3,
    Tail2,
    Tail1,
    U3_2_e0,
    U3_2_ed,
    Utf8_4_3_f0,
    Utf8_4_3_f4,
}
```

*Defined in [`utf8parse-0.2.2/src/types.rs:30-48`](../../../.source_1765521767/utf8parse-0.2.2/src/types.rs#L30-L48)*

States the parser can be in.

There is a state for each initial input of the 3 and 4 byte sequences since
the following bytes are subject to different conditions than a tail byte.

#### Variants

- **`Ground`**

  Ground state; expect anything

- **`Tail3`**

  3 tail bytes

- **`Tail2`**

  2 tail bytes

- **`Tail1`**

  1 tail byte

- **`U3_2_e0`**

  UTF8-3 starting with E0

- **`U3_2_ed`**

  UTF8-3 starting with ED

- **`Utf8_4_3_f0`**

  UTF8-4 starting with F0

- **`Utf8_4_3_f4`**

  UTF8-4 starting with F4

#### Implementations

- <span id="state-advance"></span>`fn advance(self, byte: u8) -> (State, Action)` — [`State`](#state), [`Action`](#action)

  Advance the parser state.

  

  This takes the current state and input byte into consideration, to determine the next state

  and any action that should be taken.

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

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

