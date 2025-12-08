*[clap_builder](../../index.md) / [builder](../index.md) / [arg_settings](index.md)*

---

# Module `arg_settings`

## Structs

### `ArgFlags`

```rust
struct ArgFlags(u32);
```

#### Implementations

- `fn set(self: &mut Self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- `fn unset(self: &mut Self, setting: ArgSettings)` — [`ArgSettings`](#argsettings)

- `fn is_set(self: &Self, setting: ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

- `fn insert(self: &mut Self, other: Self)`

#### Trait Implementations

##### `impl BitOr for ArgFlags`

- `type Output = ArgFlags`

- `fn bitor(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl Clone for ArgFlags`

- `fn clone(self: &Self) -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl Copy for ArgFlags`

##### `impl Debug for ArgFlags`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgFlags`

- `fn default() -> ArgFlags` — [`ArgFlags`](#argflags)

##### `impl Eq for ArgFlags`

##### `impl PartialEq for ArgFlags`

- `fn eq(self: &Self, other: &ArgFlags) -> bool` — [`ArgFlags`](#argflags)

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

Various settings that apply to arguments and may be set, unset, and checked via getter/setter
methods `Arg::setting`, `Arg::unset_setting`, and `Arg::is_set`. This is what the
[`Arg`](../../index.md) methods which accept a `bool` use internally.





#### Implementations

- `fn bit(self: Self) -> u32`

#### Trait Implementations

##### `impl Clone for ArgSettings`

- `fn clone(self: &Self) -> ArgSettings` — [`ArgSettings`](#argsettings)

##### `impl Copy for ArgSettings`

##### `impl Debug for ArgSettings`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for ArgSettings`

- `fn eq(self: &Self, other: &ArgSettings) -> bool` — [`ArgSettings`](#argsettings)

##### `impl StructuralPartialEq for ArgSettings`

