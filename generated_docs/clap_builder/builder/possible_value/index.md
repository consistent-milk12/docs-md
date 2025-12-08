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

- `fn get_name(self: &Self) -> &str`

- `fn get_help(self: &Self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- `fn is_hide_set(self: &Self) -> bool`

- `fn should_show_help(self: &Self) -> bool`

- `fn get_visible_quoted_name(self: &Self) -> Option<std::borrow::Cow<'_, str>>`

- `fn get_name_and_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`

- `fn matches(self: &Self, value: &str, ignore_case: bool) -> bool`

#### Trait Implementations

##### `impl Clone for PossibleValue`

- `fn clone(self: &Self) -> PossibleValue` — [`PossibleValue`](../index.md)

##### `impl Debug for PossibleValue`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PossibleValue`

- `fn default() -> PossibleValue` — [`PossibleValue`](../index.md)

##### `impl Eq for PossibleValue`

##### `impl PartialEq for PossibleValue`

- `fn eq(self: &Self, other: &PossibleValue) -> bool` — [`PossibleValue`](../index.md)

##### `impl StructuralPartialEq for PossibleValue`

