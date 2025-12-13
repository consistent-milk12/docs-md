*[clap_builder](../../index.md) / [builder](../index.md) / [app_settings](index.md)*

---

# Module `app_settings`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AppFlags`](#appflags) | struct |  |
| [`AppSettings`](#appsettings) | enum | Application level settings, which affect how [`Command`] operates |

## Structs

### `AppFlags`

```rust
struct AppFlags(u32);
```

*Defined in [`clap_builder-4.5.53/src/builder/app_settings.rs:7`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/app_settings.rs#L7)*

#### Implementations

- <span id="appflags-set"></span>`fn set(&mut self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- <span id="appflags-unset"></span>`fn unset(&mut self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- <span id="appflags-is-set"></span>`fn is_set(&self, setting: AppSettings) -> bool` — [`AppSettings`](#appsettings)

- <span id="appflags-insert"></span>`fn insert(&mut self, other: Self)`

#### Trait Implementations

##### `impl Any for AppFlags`

- <span id="appflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl BitOr for AppFlags`

- <span id="appflags-bitor-type-output"></span>`type Output = AppFlags`

- <span id="appflags-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl<T> Borrow for AppFlags`

- <span id="appflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AppFlags`

- <span id="appflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AppFlags`

- <span id="appflags-clone"></span>`fn clone(&self) -> AppFlags` — [`AppFlags`](#appflags)

##### `impl CloneToUninit for AppFlags`

- <span id="appflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AppFlags`

##### `impl Debug for AppFlags`

- <span id="appflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AppFlags`

- <span id="appflags-default"></span>`fn default() -> AppFlags` — [`AppFlags`](#appflags)

##### `impl Eq for AppFlags`

##### `impl<T> From for AppFlags`

- <span id="appflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AppFlags`

- <span id="appflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AppFlags`

- <span id="appflags-partialeq-eq"></span>`fn eq(&self, other: &AppFlags) -> bool` — [`AppFlags`](#appflags)

##### `impl StructuralPartialEq for AppFlags`

##### `impl ToOwned for AppFlags`

- <span id="appflags-toowned-type-owned"></span>`type Owned = T`

- <span id="appflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="appflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AppFlags`

- <span id="appflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="appflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AppFlags`

- <span id="appflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="appflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AppSettings`

```rust
enum AppSettings {
    IgnoreErrors,
    AllowHyphenValues,
    AllowNegativeNumbers,
    AllArgsOverrideSelf,
    AllowMissingPositional,
    TrailingVarArg,
    DontDelimitTrailingValues,
    InferLongArgs,
    InferSubcommands,
    SubcommandRequired,
    AllowExternalSubcommands,
    Multicall,
    SubcommandsNegateReqs,
    ArgsNegateSubcommands,
    SubcommandPrecedenceOverArg,
    FlattenHelp,
    ArgRequiredElseHelp,
    NextLineHelp,
    DisableColoredHelp,
    DisableHelpFlag,
    DisableHelpSubcommand,
    DisableVersionFlag,
    PropagateVersion,
    Hidden,
    HidePossibleValues,
    HelpExpected,
    NoBinaryName,
    ColorAuto,
    ColorAlways,
    ColorNever,
    Built,
    BinNameBuilt,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/app_settings.rs:48-82`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/app_settings.rs#L48-L82)*

Application level settings, which affect how [`Command`](../command/index.md) operates

<div class="warning">

**NOTE:** When these settings are used, they apply only to current command, and are *not*
propagated down or up through child or parent subcommands

</div>


#### Implementations

- <span id="appsettings-bit"></span>`fn bit(self) -> u32`

#### Trait Implementations

##### `impl Any for AppSettings`

- <span id="appsettings-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AppSettings`

- <span id="appsettings-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AppSettings`

- <span id="appsettings-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AppSettings`

- <span id="appsettings-clone"></span>`fn clone(&self) -> AppSettings` — [`AppSettings`](#appsettings)

##### `impl CloneToUninit for AppSettings`

- <span id="appsettings-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AppSettings`

##### `impl Debug for AppSettings`

- <span id="appsettings-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AppSettings`

- <span id="appsettings-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AppSettings`

- <span id="appsettings-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AppSettings`

- <span id="appsettings-partialeq-eq"></span>`fn eq(&self, other: &AppSettings) -> bool` — [`AppSettings`](#appsettings)

##### `impl StructuralPartialEq for AppSettings`

##### `impl ToOwned for AppSettings`

- <span id="appsettings-toowned-type-owned"></span>`type Owned = T`

- <span id="appsettings-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="appsettings-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AppSettings`

- <span id="appsettings-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="appsettings-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AppSettings`

- <span id="appsettings-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="appsettings-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

