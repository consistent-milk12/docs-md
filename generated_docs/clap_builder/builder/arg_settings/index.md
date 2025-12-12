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

*Defined in [`clap_builder-4.5.53/src/builder/arg_settings.rs:5`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/arg_settings.rs#L5)*

#### Implementations

- <span id="argflags-set"></span>`fn set(&mut self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- <span id="argflags-unset"></span>`fn unset(&mut self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- <span id="argflags-is-set"></span>`fn is_set(&self, setting: ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

- <span id="argflags-insert"></span>`fn insert(&mut self, other: Self)`

#### Trait Implementations

##### `impl BitOr for ArgFlags`

- <span id="argflags-bitor-type-output"></span>`type Output = ArgFlags`

- <span id="argflags-bitor"></span>`fn bitor(self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for ArgFlags`

- <span id="argflags-clone"></span>`fn clone(&self) -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl Copy for ArgFlags`

##### `impl Debug for ArgFlags`

- <span id="argflags-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgFlags`

- <span id="argflags-default"></span>`fn default() -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl Eq for ArgFlags`

##### `impl PartialEq for ArgFlags`

- <span id="argflags-eq"></span>`fn eq(&self, other: &ArgFlags) -> bool` — [`ArgFlags`](#argflags)

##### `impl StructuralPartialEq for ArgFlags`

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

*Defined in [`clap_builder-4.5.53/src/builder/arg_settings.rs:44-64`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/arg_settings.rs#L44-L64)*

Various settings that apply to arguments and may be set, unset, and checked via getter/setter
methods `Arg::setting`, `Arg::unset_setting`, and `Arg::is_set`. This is what the
[`Arg`](../../index.md) methods which accept a `bool` use internally.





#### Implementations

- <span id="argsettings-bit"></span>`fn bit(self) -> u32`

#### Trait Implementations

##### `impl Clone for ArgSettings`

- <span id="argsettings-clone"></span>`fn clone(&self) -> ArgSettings` — [`ArgSettings`](#argsettings)

##### `impl Copy for ArgSettings`

##### `impl Debug for ArgSettings`

- <span id="argsettings-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for ArgSettings`

- <span id="argsettings-eq"></span>`fn eq(&self, other: &ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

##### `impl StructuralPartialEq for ArgSettings`

