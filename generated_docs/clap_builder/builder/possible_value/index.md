*[clap_builder](../../index.md) / [builder](../index.md) / [possible_value](index.md)*

---

# Module `possible_value`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PossibleValue`](#possiblevalue) | struct | A possible value of an argument. |

## Structs

### `PossibleValue`

```rust
struct PossibleValue {
    name: crate::builder::Str,
    help: Option<crate::builder::StyledStr>,
    aliases: Vec<crate::builder::Str>,
    hide: bool,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/possible_value.rs:40-45`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/possible_value.rs#L40-L45)*

A possible value of an argument.

This is used for specifying [possible values] of [Args].

See also `PossibleValuesParser`

<div class="warning">

**NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required
to [hide] single values from help messages and shell completions or to attach [`help`](../../output/help/index.md) to
possible values.

</div>

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, builder::PossibleValue, ArgAction};
let cfg = Arg::new("config")
    .action(ArgAction::Set)
    .value_name("FILE")
    .value_parser([
        PossibleValue::new("fast"),
        PossibleValue::new("slow").help("slower than fast"),
        PossibleValue::new("secret speed").hide(true)
    ]);
```





#### Implementations

- <span id="possiblevalue-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](../str/index.md#str)

  Create a [`PossibleValue`](#possiblevalue) with its name.

  

  The name will be used to decide whether this value was provided by the user to an argument.

  

  <div class="warning">

  

  **NOTE:** In case it is not [hidden] it will also be shown in help messages for arguments

  that use it as a [possible value] and have not hidden them through `Arg::hide_possible_values(true)`.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("fast")

  ;

  ```

  

  

- <span id="possiblevalue-help"></span>`fn help(self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`StyledStr`](../styled_str/index.md#styledstr)

  Sets the help description of the value.

  

  This is typically displayed in completions (where supported) and should be a short, one-line

  description.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .help("not fast")

  ;

  ```

- <span id="possiblevalue-hide"></span>`fn hide(self, yes: bool) -> Self`

  Hides this value from help and shell completions.

  

  This is an alternative to hiding through `Arg::hide_possible_values(true)`, if you only

  want to hide some values.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("secret")

      .hide(true)

  ;

  ```

- <span id="possiblevalue-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

  Sets a *hidden* alias for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .alias("not-fast")

  ;

  ```

- <span id="possiblevalue-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

  Sets multiple *hidden* aliases for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .aliases(["not-fast", "snake-like"])

  ;

  ```

#### Trait Implementations

##### `impl Any for PossibleValue`

- <span id="possiblevalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PossibleValue`

- <span id="possiblevalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PossibleValue`

- <span id="possiblevalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PossibleValue`

- <span id="possiblevalue-clone"></span>`fn clone(&self) -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl CloneToUninit for PossibleValue`

- <span id="possiblevalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PossibleValue`

- <span id="possiblevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PossibleValue`

- <span id="possiblevalue-default"></span>`fn default() -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Eq for PossibleValue`

##### `impl<T> From for PossibleValue`

- <span id="possiblevalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PossibleValue`

- <span id="possiblevalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PossibleValue`

- <span id="possiblevalue-partialeq-eq"></span>`fn eq(&self, other: &PossibleValue) -> bool` — [`PossibleValue`](#possiblevalue)

##### `impl StructuralPartialEq for PossibleValue`

##### `impl ToOwned for PossibleValue`

- <span id="possiblevalue-toowned-type-owned"></span>`type Owned = T`

- <span id="possiblevalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="possiblevalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PossibleValue`

- <span id="possiblevalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="possiblevalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PossibleValue`

- <span id="possiblevalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="possiblevalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

