*[clap_builder](../../index.md) / [builder](../index.md) / [arg](index.md)*

---

# Module `arg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Arg`](#arg) | struct | The abstract representation of a command line argument. |

## Structs

### `Arg`

```rust
struct Arg {
    id: crate::Id,
    help: Option<crate::builder::StyledStr>,
    long_help: Option<crate::builder::StyledStr>,
    action: Option<crate::ArgAction>,
    value_parser: Option<super::ValueParser>,
    blacklist: Vec<crate::Id>,
    settings: arg_settings::ArgFlags,
    overrides: Vec<crate::Id>,
    groups: Vec<crate::Id>,
    requires: Vec<(crate::builder::ArgPredicate, crate::Id)>,
    r_ifs: Vec<(crate::Id, crate::builder::OsStr)>,
    r_ifs_all: Vec<(crate::Id, crate::builder::OsStr)>,
    r_unless: Vec<crate::Id>,
    r_unless_all: Vec<crate::Id>,
    short: Option<char>,
    long: Option<crate::builder::Str>,
    aliases: Vec<(crate::builder::Str, bool)>,
    short_aliases: Vec<(char, bool)>,
    disp_ord: Option<usize>,
    val_names: Vec<crate::builder::Str>,
    num_vals: Option<crate::builder::ValueRange>,
    val_delim: Option<char>,
    default_vals: Vec<crate::builder::OsStr>,
    default_vals_ifs: Vec<(crate::Id, crate::builder::ArgPredicate, Option<Vec<crate::builder::OsStr>>)>,
    default_missing_vals: Vec<crate::builder::OsStr>,
    terminator: Option<crate::builder::Str>,
    index: Option<usize>,
    help_heading: Option<Option<crate::builder::Str>>,
    ext: crate::builder::ext::Extensions,
}
```

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](../../index.md)s, using the builder pattern and setting options
manually, or using a usage string which is far less verbose but has fewer options. You can also
use a combination of the two methods to achieve the best of both worlds.

- [Basic API][crate::Arg#basic-api]
- [Value Handling][crate::Arg#value-handling]
- [Help][crate::Arg#help-1]
- [Advanced Argument Relations][crate::Arg#advanced-argument-relations]
- [Reflection][crate::Arg#reflection]

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, arg, ArgAction};
// Using the traditional builder pattern and setting each option manually
let cfg = Arg::new("config")
      .short('c')
      .long("config")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("Provides a config file to myprog");
// Using a usage string (setting a similar argument to the one above)
let input = arg!(-i --input <FILE> "Provides an input file to the program");
```

#### Implementations

- <span id="arg-group"></span>`fn group(self, group_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../index.md), [`Id`](../../index.md)

- <span id="arg-groups"></span>`fn groups(self, group_ids: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../index.md)

- <span id="arg-default-value-if"></span>`fn default_value_if(self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, default: impl IntoResettable<OsStr>) -> Self` — [`Id`](../../index.md), [`ArgPredicate`](../index.md), [`IntoResettable`](../index.md), [`OsStr`](../index.md)

- <span id="arg-default-values-if"></span>`fn default_values_if(self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, defaults: impl IntoIterator<Item = impl Into<OsStr>>) -> Self` — [`Id`](../../index.md), [`ArgPredicate`](../index.md), [`OsStr`](../index.md)

- <span id="arg-default-value-ifs"></span>`fn default_value_ifs(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>)>) -> Self` — [`Id`](../../index.md), [`ArgPredicate`](../index.md), [`IntoResettable`](../index.md), [`OsStr`](../index.md)

- <span id="arg-default-values-ifs"></span>`fn default_values_ifs(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoIterator<Item = impl Into<OsStr>>)>) -> Self` — [`Id`](../../index.md), [`ArgPredicate`](../index.md), [`OsStr`](../index.md)

- <span id="arg-required-unless-present"></span>`fn required_unless_present(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../index.md), [`Id`](../../index.md)

- <span id="arg-required-unless-present-all"></span>`fn required_unless_present_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../index.md)

- <span id="arg-required-unless-present-any"></span>`fn required_unless_present_any(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../index.md)

- <span id="arg-required-if-eq"></span>`fn required_if_eq(self, arg_id: impl Into<Id>, val: impl Into<OsStr>) -> Self` — [`Id`](../../index.md), [`OsStr`](../index.md)

- <span id="arg-required-if-eq-any"></span>`fn required_if_eq_any(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self` — [`Id`](../../index.md), [`OsStr`](../index.md)

- <span id="arg-required-if-eq-all"></span>`fn required_if_eq_all(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self` — [`Id`](../../index.md), [`OsStr`](../index.md)

- <span id="arg-requires-if"></span>`fn requires_if(self, val: impl Into<ArgPredicate>, arg_id: impl Into<Id>) -> Self` — [`ArgPredicate`](../index.md), [`Id`](../../index.md)

- <span id="arg-requires-ifs"></span>`fn requires_ifs(self, ifs: impl IntoIterator<Item = (impl Into<ArgPredicate>, impl Into<Id>)>) -> Self` — [`ArgPredicate`](../index.md), [`Id`](../../index.md)

- <span id="arg-conflicts-with"></span>`fn conflicts_with(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../index.md), [`Id`](../../index.md)

- <span id="arg-conflicts-with-all"></span>`fn conflicts_with_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../index.md)

- <span id="arg-overrides-with"></span>`fn overrides_with(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../index.md), [`Id`](../../index.md)

- <span id="arg-overrides-with-all"></span>`fn overrides_with_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../index.md)

#### Trait Implementations

##### `impl Clone for Arg`

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](../../index.md)

##### `impl Debug for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](../../index.md)

##### `impl Display for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- <span id="arg-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](../../index.md)

##### `impl PartialEq for Arg`

- <span id="arg-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](../../index.md)

##### `impl PartialOrd for Arg`

- <span id="arg-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> ToString for Arg`

- <span id="arg-to-string"></span>`fn to_string(&self) -> String`

