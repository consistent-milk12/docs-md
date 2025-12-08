*[clap_builder](../../index.md) / [builder](../index.md) / [app_settings](index.md)*

---

# Module `app_settings`

## Structs

### `AppFlags`

```rust
struct AppFlags(u32);
```

#### Implementations

- `fn set(self: &mut Self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- `fn unset(self: &mut Self, setting: AppSettings)` — [`AppSettings`](#appsettings)

- `fn is_set(self: &Self, setting: AppSettings) -> bool` — [`AppSettings`](#appsettings)

- `fn insert(self: &mut Self, other: Self)`

#### Trait Implementations

##### `impl BitOr for AppFlags`

- `type Output = AppFlags`

- `fn bitor(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for AppFlags`

- `fn clone(self: &Self) -> AppFlags` — [`AppFlags`](#appflags)

##### `impl Copy for AppFlags`

##### `impl Debug for AppFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AppFlags`

- `fn default() -> AppFlags` — [`AppFlags`](#appflags)

##### `impl Eq for AppFlags`

##### `impl PartialEq for AppFlags`

- `fn eq(self: &Self, other: &AppFlags) -> bool` — [`AppFlags`](#appflags)

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

Application level settings, which affect how [`Command`](../command/index.md) operates

<div class="warning">

**NOTE:** When these settings are used, they apply only to current command, and are *not*
propagated down or up through child or parent subcommands

</div>


#### Implementations

- `fn bit(self: Self) -> u32`

#### Trait Implementations

##### `impl Clone for AppSettings`

- `fn clone(self: &Self) -> AppSettings` — [`AppSettings`](#appsettings)

##### `impl Copy for AppSettings`

##### `impl Debug for AppSettings`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for AppSettings`

- `fn eq(self: &Self, other: &AppSettings) -> bool` — [`AppSettings`](#appsettings)

##### `impl StructuralPartialEq for AppSettings`

