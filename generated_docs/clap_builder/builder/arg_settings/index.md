*[clap_builder](../../index.md) / [builder](../index.md) / [arg_settings](index.md)*

---

# Module `arg_settings`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgFlags`](#argflags) | struct |  |
| [`ArgSettings`](#argsettings) | enum | Various settings that apply to arguments and may be set, unset, and checked via getter/setter methods [`Arg::setting`], [`Arg::unset_setting`], and [`Arg::is_set`]. |

## Structs

### `ArgFlags`

```rust
struct ArgFlags(u32);
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_settings.rs:5`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_settings.rs#L5)*

#### Implementations

- <span id="argflags-set"></span>`fn set(&mut self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- <span id="argflags-unset"></span>`fn unset(&mut self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- <span id="argflags-is-set"></span>`fn is_set(&self, setting: ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

- <span id="argflags-insert"></span>`fn insert(&mut self, other: Self)`

#### Trait Implementations

##### `impl Any for ArgFlags`

- <span id="argflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl BitOr for ArgFlags`

- <span id="argflags-bitor-type-output"></span>`type Output = ArgFlags`

- <span id="argflags-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl<T> Borrow for ArgFlags`

- <span id="argflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgFlags`

- <span id="argflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgFlags`

- <span id="argflags-clone"></span>`fn clone(&self) -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl CloneToUninit for ArgFlags`

- <span id="argflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ArgFlags`

##### `impl Debug for ArgFlags`

- <span id="argflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgFlags`

- <span id="argflags-default"></span>`fn default() -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl Eq for ArgFlags`

##### `impl<T> From for ArgFlags`

- <span id="argflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgFlags`

- <span id="argflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgFlags`

- <span id="argflags-partialeq-eq"></span>`fn eq(&self, other: &ArgFlags) -> bool` — [`ArgFlags`](#argflags)

##### `impl StructuralPartialEq for ArgFlags`

##### `impl ToOwned for ArgFlags`

- <span id="argflags-toowned-type-owned"></span>`type Owned = T`

- <span id="argflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgFlags`

- <span id="argflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgFlags`

- <span id="argflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ArgSettings`

```rust
enum ArgSettings {
    Required,
    Global,
    Hidden,
    NextLineHelp,
    HidePossibleValues,
    AllowHyphenValues,
    AllowNegativeNumbers,
    RequireEquals,
    Last,
    TrailingVarArg,
    HideDefaultValue,
    IgnoreCase,
    HiddenShortHelp,
    HiddenLongHelp,
    Exclusive,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_settings.rs:44-64`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_settings.rs#L44-L64)*

Various settings that apply to arguments and may be set, unset, and checked via getter/setter
methods `Arg::setting`, `Arg::unset_setting`, and `Arg::is_set`. This is what the
[`Arg`](../arg/index.md) methods which accept a `bool` use internally.





#### Implementations

- <span id="argsettings-bit"></span>`fn bit(self) -> u32`

#### Trait Implementations

##### `impl Any for ArgSettings`

- <span id="argsettings-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgSettings`

- <span id="argsettings-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgSettings`

- <span id="argsettings-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgSettings`

- <span id="argsettings-clone"></span>`fn clone(&self) -> ArgSettings` — [`ArgSettings`](#argsettings)

##### `impl CloneToUninit for ArgSettings`

- <span id="argsettings-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ArgSettings`

##### `impl Debug for ArgSettings`

- <span id="argsettings-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArgSettings`

- <span id="argsettings-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgSettings`

- <span id="argsettings-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgSettings`

- <span id="argsettings-partialeq-eq"></span>`fn eq(&self, other: &ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

##### `impl StructuralPartialEq for ArgSettings`

##### `impl ToOwned for ArgSettings`

- <span id="argsettings-toowned-type-owned"></span>`type Owned = T`

- <span id="argsettings-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argsettings-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgSettings`

- <span id="argsettings-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argsettings-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgSettings`

- <span id="argsettings-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argsettings-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

