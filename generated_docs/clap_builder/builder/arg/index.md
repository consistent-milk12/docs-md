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

*Defined in [`clap_builder-4.5.53/src/builder/arg.rs:60-92`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg.rs#L60-L92)*

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](#arg)s, using the builder pattern and setting options
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

- <span id="arg-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

- <span id="arg-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

- <span id="arg-short"></span>`fn short(self, s: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

- <span id="arg-long"></span>`fn long(self, l: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

- <span id="arg-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

- <span id="arg-short-alias"></span>`fn short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

- <span id="arg-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

- <span id="arg-short-aliases"></span>`fn short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

- <span id="arg-visible-alias"></span>`fn visible_alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

- <span id="arg-visible-short-alias"></span>`fn visible_short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

- <span id="arg-visible-aliases"></span>`fn visible_aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

- <span id="arg-visible-short-aliases"></span>`fn visible_short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

- <span id="arg-index"></span>`fn index(self, idx: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

- <span id="arg-trailing-var-arg"></span>`fn trailing_var_arg(self, yes: bool) -> Self`

- <span id="arg-last"></span>`fn last(self, yes: bool) -> Self`

- <span id="arg-required"></span>`fn required(self, yes: bool) -> Self`

- <span id="arg-requires"></span>`fn requires(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Id`](../../util/id/index.md#id)

- <span id="arg-exclusive"></span>`fn exclusive(self, yes: bool) -> Self`

- <span id="arg-global"></span>`fn global(self, yes: bool) -> Self`

- <span id="arg-is-set"></span>`fn is_set(&self, s: ArgSettings) -> bool` — [`ArgSettings`](../arg_settings/index.md#argsettings)

- <span id="arg-setting"></span>`fn setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](../arg_settings/index.md#argsettings)

- <span id="arg-unset-setting"></span>`fn unset_setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](../arg_settings/index.md#argsettings)

#### Trait Implementations

##### `impl Clone for Arg`

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](#arg)

##### `impl Debug for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](#arg)

##### `impl Display for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- <span id="arg-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](#arg)

##### `impl PartialEq for Arg`

- <span id="arg-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](#arg)

##### `impl PartialOrd for Arg`

- <span id="arg-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl ToString for Arg`

- <span id="arg-to-string"></span>`fn to_string(&self) -> String`

