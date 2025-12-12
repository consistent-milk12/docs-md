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

*Defined in [`clap_builder-4.5.53/src/builder/app_settings.rs:7`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/app_settings.rs#L7)*

#### Implementations

- <span id="appflags-set"></span>`fn set(&mut self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- <span id="appflags-unset"></span>`fn unset(&mut self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- <span id="appflags-is-set"></span>`fn is_set(&self, setting: AppSettings) -> bool` — [`AppSettings`](#appsettings)

- <span id="appflags-insert"></span>`fn insert(&mut self, other: Self)`

#### Trait Implementations

##### `impl BitOr for AppFlags`

- <span id="appflags-bitor-type-output"></span>`type Output = AppFlags`

- <span id="appflags-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for AppFlags`

- <span id="appflags-clone"></span>`fn clone(&self) -> AppFlags` — [`AppFlags`](#appflags)

##### `impl Copy for AppFlags`

##### `impl Debug for AppFlags`

- <span id="appflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AppFlags`

- <span id="appflags-default"></span>`fn default() -> AppFlags` — [`AppFlags`](#appflags)

##### `impl Eq for AppFlags`

##### `impl PartialEq for AppFlags`

- <span id="appflags-eq"></span>`fn eq(&self, other: &AppFlags) -> bool` — [`AppFlags`](#appflags)

##### `impl StructuralPartialEq for AppFlags`

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

*Defined in [`clap_builder-4.5.53/src/builder/app_settings.rs:48-82`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/app_settings.rs#L48-L82)*

Application level settings, which affect how [`Command`](../../index.md) operates

<div class="warning">

**NOTE:** When these settings are used, they apply only to current command, and are *not*
propagated down or up through child or parent subcommands

</div>


#### Implementations

- <span id="appsettings-bit"></span>`fn bit(self) -> u32`

#### Trait Implementations

##### `impl Clone for AppSettings`

- <span id="appsettings-clone"></span>`fn clone(&self) -> AppSettings` — [`AppSettings`](#appsettings)

##### `impl Copy for AppSettings`

##### `impl Debug for AppSettings`

- <span id="appsettings-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for AppSettings`

- <span id="appsettings-eq"></span>`fn eq(&self, other: &AppSettings) -> bool` — [`AppSettings`](#appsettings)

##### `impl StructuralPartialEq for AppSettings`

