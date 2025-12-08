_[utf8parse](../index.md) / [types](index.md)_

---

# Module `types`

Types supporting the UTF-8 parser

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

- `fn clone(self: &Self) -> Action` — [`Action`](#action)

##### `impl Copy for Action`

##### `impl Debug for Action`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn advance(self: Self, byte: u8) -> (State, Action)` — [`State`](#state), [`Action`](#action)

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Copy for State`

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for State`

- `fn default() -> State` — [`State`](#state)

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`
