*[clap_builder](../../index.md) / [parser](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parser`](#parser) | struct |  |
| [`PendingArg`](#pendingarg) | struct |  |
| [`ParseState`](#parsestate) | enum |  |
| [`ParseResult`](#parseresult) | enum | Recoverable Parsing results. |
| [`Identifier`](#identifier) | enum |  |

## Structs

### `Parser<'cmd>`

```rust
struct Parser<'cmd> {
    cmd: &'cmd mut crate::builder::Command,
    cur_idx: std::cell::Cell<usize>,
    flag_subcmd_at: Option<usize>,
    flag_subcmd_skip: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:23-31`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L23-L31)*

#### Fields

- **`flag_subcmd_at`**: `Option<usize>`

  Index of the previous flag subcommand in a group of flags.

- **`flag_subcmd_skip`**: `usize`

  Counter indicating the number of items to skip
  when revisiting the group of flags which includes the flag subcommand.

#### Implementations

- <span id="parser-new"></span>`fn new(cmd: &'cmd mut Command) -> Self` — [`Command`](../../builder/command/index.md#command)

#### Trait Implementations

##### `impl Any for Parser<'cmd>`

- <span id="parser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parser<'cmd>`

- <span id="parser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parser<'cmd>`

- <span id="parser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Parser<'cmd>`

- <span id="parser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parser<'cmd>`

- <span id="parser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Parser<'cmd>`

- <span id="parser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parser<'cmd>`

- <span id="parser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PendingArg`

```rust
struct PendingArg {
    id: crate::util::Id,
    ident: Option<Identifier>,
    raw_vals: Vec<std::ffi::OsString>,
    trailing_idx: Option<usize>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1666-1671`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1666-L1671)*

#### Trait Implementations

##### `impl Any for PendingArg`

- <span id="pendingarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PendingArg`

- <span id="pendingarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PendingArg`

- <span id="pendingarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PendingArg`

- <span id="pendingarg-clone"></span>`fn clone(&self) -> PendingArg` — [`PendingArg`](#pendingarg)

##### `impl CloneToUninit for PendingArg`

- <span id="pendingarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PendingArg`

- <span id="pendingarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PendingArg`

##### `impl<T> From for PendingArg`

- <span id="pendingarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PendingArg`

- <span id="pendingarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PendingArg`

- <span id="pendingarg-partialeq-eq"></span>`fn eq(&self, other: &PendingArg) -> bool` — [`PendingArg`](#pendingarg)

##### `impl StructuralPartialEq for PendingArg`

##### `impl ToOwned for PendingArg`

- <span id="pendingarg-toowned-type-owned"></span>`type Owned = T`

- <span id="pendingarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pendingarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PendingArg`

- <span id="pendingarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pendingarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PendingArg`

- <span id="pendingarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pendingarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ParseState`

```rust
enum ParseState {
    ValuesDone,
    Opt(crate::util::Id),
    Pos(crate::util::Id),
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1629-1633`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1629-L1633)*

#### Trait Implementations

##### `impl Any for ParseState`

- <span id="parsestate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseState`

- <span id="parsestate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseState`

- <span id="parsestate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseState`

- <span id="parsestate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseState`

##### `impl<T> From for ParseState`

- <span id="parsestate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseState`

- <span id="parsestate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ParseState`

- <span id="parsestate-partialeq-eq"></span>`fn eq(&self, other: &ParseState) -> bool` — [`ParseState`](#parsestate)

##### `impl StructuralPartialEq for ParseState`

##### `impl<U> TryFrom for ParseState`

- <span id="parsestate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsestate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseState`

- <span id="parsestate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsestate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParseResult`

```rust
enum ParseResult {
    FlagSubCommand(String),
    Opt(crate::util::Id),
    ValuesDone,
    AttachedValueNotConsumed,
    UnneededAttachedValue {
        rest: String,
        used: Vec<crate::util::Id>,
        arg: String,
    },
    MaybeHyphenValue,
    EqualsNotProvided {
        arg: String,
    },
    NoMatchingArg {
        arg: String,
    },
    NoArg,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1638-1663`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1638-L1663)*

Recoverable Parsing results.

#### Variants

- **`AttachedValueNotConsumed`**

  Value attached to the short flag is not consumed(e.g. 'u' for `-cu` is
  not consumed).

- **`UnneededAttachedValue`**

  This long flag doesn't need a value but is provided one.

- **`MaybeHyphenValue`**

  This flag might be an hyphen Value.

- **`EqualsNotProvided`**

  Equals required but not provided.

- **`NoMatchingArg`**

  Failed to match a Arg.

- **`NoArg`**

  No argument found e.g. parser is given `-` when parsing a flag.

#### Trait Implementations

##### `impl Any for ParseResult`

- <span id="parseresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseResult`

- <span id="parseresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseResult`

- <span id="parseresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ParseResult`

- <span id="parseresult-clone"></span>`fn clone(&self) -> ParseResult` — [`ParseResult`](#parseresult)

##### `impl CloneToUninit for ParseResult`

- <span id="parseresult-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ParseResult`

- <span id="parseresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseResult`

- <span id="parseresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseResult`

- <span id="parseresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ParseResult`

- <span id="parseresult-partialeq-eq"></span>`fn eq(&self, other: &ParseResult) -> bool` — [`ParseResult`](#parseresult)

##### `impl StructuralPartialEq for ParseResult`

##### `impl ToOwned for ParseResult`

- <span id="parseresult-toowned-type-owned"></span>`type Owned = T`

- <span id="parseresult-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parseresult-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ParseResult`

- <span id="parseresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseResult`

- <span id="parseresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Identifier`

```rust
enum Identifier {
    Short,
    Long,
    Index,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1674-1678`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1674-L1678)*

#### Trait Implementations

##### `impl Any for Identifier`

- <span id="identifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Identifier`

- <span id="identifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Identifier`

- <span id="identifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Identifier`

- <span id="identifier-clone"></span>`fn clone(&self) -> Identifier` — [`Identifier`](#identifier)

##### `impl CloneToUninit for Identifier`

- <span id="identifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Identifier`

##### `impl Debug for Identifier`

- <span id="identifier-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Identifier`

##### `impl<T> From for Identifier`

- <span id="identifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Identifier`

- <span id="identifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Identifier`

- <span id="identifier-partialeq-eq"></span>`fn eq(&self, other: &Identifier) -> bool` — [`Identifier`](#identifier)

##### `impl StructuralPartialEq for Identifier`

##### `impl ToOwned for Identifier`

- <span id="identifier-toowned-type-owned"></span>`type Owned = T`

- <span id="identifier-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="identifier-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Identifier`

- <span id="identifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="identifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Identifier`

- <span id="identifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="identifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

