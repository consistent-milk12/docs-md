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

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](#action)

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

