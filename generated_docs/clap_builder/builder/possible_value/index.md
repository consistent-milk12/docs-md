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

- <span id="possiblevalue-help"></span>`fn help(self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`StyledStr`](../styled_str/index.md#styledstr)

- <span id="possiblevalue-hide"></span>`fn hide(self, yes: bool) -> Self`

- <span id="possiblevalue-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

- <span id="possiblevalue-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

#### Trait Implementations

##### `impl Clone for PossibleValue`

- <span id="possiblevalue-clone"></span>`fn clone(&self) -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Debug for PossibleValue`

- <span id="possiblevalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PossibleValue`

- <span id="possiblevalue-default"></span>`fn default() -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Eq for PossibleValue`

##### `impl PartialEq for PossibleValue`

- <span id="possiblevalue-eq"></span>`fn eq(&self, other: &PossibleValue) -> bool` — [`PossibleValue`](#possiblevalue)

##### `impl StructuralPartialEq for PossibleValue`

