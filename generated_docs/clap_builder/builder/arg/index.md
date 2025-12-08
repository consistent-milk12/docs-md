*[clap_builder](../../index.md) / [builder](../index.md) / [arg](index.md)*

---

# Module `arg`

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

- `fn get_id(self: &Self) -> &Id` — [`Id`](../../index.md)

- `fn get_help(self: &Self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- `fn get_long_help(self: &Self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- `fn get_display_order(self: &Self) -> usize`

- `fn get_help_heading(self: &Self) -> Option<&str>`

- `fn get_short(self: &Self) -> Option<char>`

- `fn get_visible_short_aliases(self: &Self) -> Option<Vec<char>>`

- `fn get_all_short_aliases(self: &Self) -> Option<Vec<char>>`

- `fn get_short_and_visible_aliases(self: &Self) -> Option<Vec<char>>`

- `fn get_long(self: &Self) -> Option<&str>`

- `fn get_visible_aliases(self: &Self) -> Option<Vec<&str>>`

- `fn get_all_aliases(self: &Self) -> Option<Vec<&str>>`

- `fn get_long_and_visible_aliases(self: &Self) -> Option<Vec<&str>>`

- `fn get_aliases(self: &Self) -> Option<Vec<&str>>`

- `fn get_possible_values(self: &Self) -> Vec<PossibleValue>` — [`PossibleValue`](../index.md)

- `fn get_value_names(self: &Self) -> Option<&[Str]>` — [`Str`](../index.md)

- `fn get_num_args(self: &Self) -> Option<ValueRange>` — [`ValueRange`](../index.md)

- `fn get_min_vals(self: &Self) -> usize`

- `fn get_value_delimiter(self: &Self) -> Option<char>`

- `fn get_value_terminator(self: &Self) -> Option<&Str>` — [`Str`](../index.md)

- `fn get_index(self: &Self) -> Option<usize>`

- `fn get_value_hint(self: &Self) -> ValueHint` — [`ValueHint`](../../index.md)

- `fn get_default_values(self: &Self) -> &[OsStr]` — [`OsStr`](../index.md)

- `fn is_positional(self: &Self) -> bool`

- `fn is_required_set(self: &Self) -> bool`

- `fn is_multiple_values_set(self: &Self) -> bool`

- `fn is_takes_value_set(self: &Self) -> bool`

- `fn is_allow_hyphen_values_set(self: &Self) -> bool`

- `fn is_allow_negative_numbers_set(self: &Self) -> bool`

- `fn get_action(self: &Self) -> &ArgAction` — [`ArgAction`](../../index.md)

- `fn get_value_parser(self: &Self) -> &super::ValueParser` — [`ValueParser`](../index.md)

- `fn is_global_set(self: &Self) -> bool`

- `fn is_next_line_help_set(self: &Self) -> bool`

- `fn is_hide_set(self: &Self) -> bool`

- `fn is_hide_default_value_set(self: &Self) -> bool`

- `fn is_hide_possible_values_set(self: &Self) -> bool`

- `fn is_hide_short_help_set(self: &Self) -> bool`

- `fn is_hide_long_help_set(self: &Self) -> bool`

- `fn is_require_equals_set(self: &Self) -> bool`

- `fn is_exclusive_set(self: &Self) -> bool`

- `fn is_trailing_var_arg_set(self: &Self) -> bool`

- `fn is_last_set(self: &Self) -> bool`

- `fn is_ignore_case_set(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Arg`

- `fn clone(self: &Self) -> Arg` — [`Arg`](../../index.md)

##### `impl Debug for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- `fn default() -> Arg` — [`Arg`](../../index.md)

##### `impl Display for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- `fn cmp(self: &Self, other: &Arg) -> Ordering` — [`Arg`](../../index.md)

##### `impl PartialEq for Arg`

- `fn eq(self: &Self, other: &Arg) -> bool` — [`Arg`](../../index.md)

##### `impl PartialOrd for Arg`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl<T> ToString for Arg`

- `fn to_string(self: &Self) -> String`

