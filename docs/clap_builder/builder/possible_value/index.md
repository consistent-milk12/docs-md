*[clap_builder](../../index.md) / [builder](../index.md) / [possible_value](index.md)*

---

# Module `possible_value`

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

- `fn new(name: impl Into<Str>) -> Self` — [`Str`](../str/index.md)

- `fn help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../resettable/index.md), [`StyledStr`](../styled_str/index.md)

- `fn hide(self: Self, yes: bool) -> Self`

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md), [`Str`](../str/index.md)

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md)

#### Trait Implementations

##### `impl Clone for PossibleValue`

- `fn clone(self: &Self) -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Debug for PossibleValue`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PossibleValue`

- `fn default() -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Eq for PossibleValue`

##### `impl PartialEq for PossibleValue`

- `fn eq(self: &Self, other: &PossibleValue) -> bool` — [`PossibleValue`](#possiblevalue)

##### `impl StructuralPartialEq for PossibleValue`

