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

- <span id="arg-get-id"></span>`fn get_id(&self) -> &Id` — [`Id`](../../index.md)

- <span id="arg-get-help"></span>`fn get_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="arg-get-long-help"></span>`fn get_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="arg-get-display-order"></span>`fn get_display_order(&self) -> usize`

- <span id="arg-get-help-heading"></span>`fn get_help_heading(&self) -> Option<&str>`

- <span id="arg-get-short"></span>`fn get_short(&self) -> Option<char>`

- <span id="arg-get-visible-short-aliases"></span>`fn get_visible_short_aliases(&self) -> Option<Vec<char>>`

- <span id="arg-get-all-short-aliases"></span>`fn get_all_short_aliases(&self) -> Option<Vec<char>>`

- <span id="arg-get-short-and-visible-aliases"></span>`fn get_short_and_visible_aliases(&self) -> Option<Vec<char>>`

- <span id="arg-get-long"></span>`fn get_long(&self) -> Option<&str>`

- <span id="arg-get-visible-aliases"></span>`fn get_visible_aliases(&self) -> Option<Vec<&str>>`

- <span id="arg-get-all-aliases"></span>`fn get_all_aliases(&self) -> Option<Vec<&str>>`

- <span id="arg-get-long-and-visible-aliases"></span>`fn get_long_and_visible_aliases(&self) -> Option<Vec<&str>>`

- <span id="arg-get-aliases"></span>`fn get_aliases(&self) -> Option<Vec<&str>>`

- <span id="arg-get-possible-values"></span>`fn get_possible_values(&self) -> Vec<PossibleValue>` — [`PossibleValue`](../index.md)

- <span id="arg-get-value-names"></span>`fn get_value_names(&self) -> Option<&[Str]>` — [`Str`](../index.md)

- <span id="arg-get-num-args"></span>`fn get_num_args(&self) -> Option<ValueRange>` — [`ValueRange`](../index.md)

- <span id="arg-get-min-vals"></span>`fn get_min_vals(&self) -> usize`

- <span id="arg-get-value-delimiter"></span>`fn get_value_delimiter(&self) -> Option<char>`

- <span id="arg-get-value-terminator"></span>`fn get_value_terminator(&self) -> Option<&Str>` — [`Str`](../index.md)

- <span id="arg-get-index"></span>`fn get_index(&self) -> Option<usize>`

- <span id="arg-get-value-hint"></span>`fn get_value_hint(&self) -> ValueHint` — [`ValueHint`](../../index.md)

- <span id="arg-get-default-values"></span>`fn get_default_values(&self) -> &[OsStr]` — [`OsStr`](../index.md)

- <span id="arg-is-positional"></span>`fn is_positional(&self) -> bool`

- <span id="arg-is-required-set"></span>`fn is_required_set(&self) -> bool`

- <span id="arg-is-multiple-values-set"></span>`fn is_multiple_values_set(&self) -> bool`

- <span id="arg-is-takes-value-set"></span>`fn is_takes_value_set(&self) -> bool`

- <span id="arg-is-allow-hyphen-values-set"></span>`fn is_allow_hyphen_values_set(&self) -> bool`

- <span id="arg-is-allow-negative-numbers-set"></span>`fn is_allow_negative_numbers_set(&self) -> bool`

- <span id="arg-get-action"></span>`fn get_action(&self) -> &ArgAction` — [`ArgAction`](../../index.md)

- <span id="arg-get-value-parser"></span>`fn get_value_parser(&self) -> &super::ValueParser` — [`ValueParser`](../index.md)

- <span id="arg-is-global-set"></span>`fn is_global_set(&self) -> bool`

- <span id="arg-is-next-line-help-set"></span>`fn is_next_line_help_set(&self) -> bool`

- <span id="arg-is-hide-set"></span>`fn is_hide_set(&self) -> bool`

- <span id="arg-is-hide-default-value-set"></span>`fn is_hide_default_value_set(&self) -> bool`

- <span id="arg-is-hide-possible-values-set"></span>`fn is_hide_possible_values_set(&self) -> bool`

- <span id="arg-is-hide-short-help-set"></span>`fn is_hide_short_help_set(&self) -> bool`

- <span id="arg-is-hide-long-help-set"></span>`fn is_hide_long_help_set(&self) -> bool`

- <span id="arg-is-require-equals-set"></span>`fn is_require_equals_set(&self) -> bool`

- <span id="arg-is-exclusive-set"></span>`fn is_exclusive_set(&self) -> bool`

- <span id="arg-is-trailing-var-arg-set"></span>`fn is_trailing_var_arg_set(&self) -> bool`

- <span id="arg-is-last-set"></span>`fn is_last_set(&self) -> bool`

- <span id="arg-is-ignore-case-set"></span>`fn is_ignore_case_set(&self) -> bool`

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

