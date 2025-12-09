*[clap_builder](../index.md) / [builder](index.md)*

---

# Module `builder`

Define [`Command`](../index.md) line [arguments][`Arg`](../index.md)

## Contents

- [Modules](#modules)
  - [`action`](#action)
  - [`app_settings`](#app_settings)
  - [`arg`](#arg)
  - [`arg_group`](#arg_group)
  - [`arg_predicate`](#arg_predicate)
  - [`arg_settings`](#arg_settings)
  - [`command`](#command)
  - [`ext`](#ext)
  - [`os_str`](#os_str)
  - [`possible_value`](#possible_value)
  - [`range`](#range)
  - [`resettable`](#resettable)
  - [`str`](#str)
  - [`styled_str`](#styled_str)
  - [`value_hint`](#value_hint)
  - [`value_parser`](#value_parser)
  - [`debug_asserts`](#debug_asserts)
  - [`styling`](#styling)
- [Structs](#structs)
  - [`Str`](#str)
  - [`Arg`](#arg)
  - [`ArgGroup`](#arggroup)
  - [`Command`](#command)
  - [`OsStr`](#osstr)
  - [`PossibleValue`](#possiblevalue)
  - [`ValueRange`](#valuerange)
  - [`StyledStr`](#styledstr)
  - [`Styles`](#styles)
  - [`BoolValueParser`](#boolvalueparser)
  - [`BoolishValueParser`](#boolishvalueparser)
  - [`EnumValueParser`](#enumvalueparser)
  - [`FalseyValueParser`](#falseyvalueparser)
  - [`MapValueParser`](#mapvalueparser)
  - [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
  - [`OsStringValueParser`](#osstringvalueparser)
  - [`PathBufValueParser`](#pathbufvalueparser)
  - [`PossibleValuesParser`](#possiblevaluesparser)
  - [`RangedI64ValueParser`](#rangedi64valueparser)
  - [`RangedU64ValueParser`](#rangedu64valueparser)
  - [`StringValueParser`](#stringvalueparser)
  - [`TryMapValueParser`](#trymapvalueparser)
  - [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
  - [`ValueParser`](#valueparser)
- [Enums](#enums)
  - [`ArgAction`](#argaction)
  - [`ArgPredicate`](#argpredicate)
  - [`Resettable`](#resettable)
  - [`ValueHint`](#valuehint)
- [Traits](#traits)
  - [`IntoResettable`](#intoresettable)
  - [`TypedValueParser`](#typedvalueparser)
  - [`ValueParserFactory`](#valueparserfactory)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`action`](#action) | mod |  |
| [`app_settings`](#app_settings) | mod |  |
| [`arg`](#arg) | mod |  |
| [`arg_group`](#arg_group) | mod |  |
| [`arg_predicate`](#arg_predicate) | mod |  |
| [`arg_settings`](#arg_settings) | mod |  |
| [`command`](#command) | mod |  |
| [`ext`](#ext) | mod |  |
| [`os_str`](#os_str) | mod |  |
| [`possible_value`](#possible_value) | mod |  |
| [`range`](#range) | mod |  |
| [`resettable`](#resettable) | mod |  |
| [`str`](#str) | mod |  |
| [`styled_str`](#styled_str) | mod |  |
| [`value_hint`](#value_hint) | mod |  |
| [`value_parser`](#value_parser) | mod |  |
| [`debug_asserts`](#debug_asserts) | mod |  |
| [`styling`](#styling) | mod | Terminal [`Styles`] for help and error output |
| [`Str`](#str) | struct |  |
| [`Arg`](#arg) | struct |  |
| [`ArgGroup`](#arggroup) | struct |  |
| [`Command`](#command) | struct |  |
| [`OsStr`](#osstr) | struct |  |
| [`PossibleValue`](#possiblevalue) | struct |  |
| [`ValueRange`](#valuerange) | struct |  |
| [`StyledStr`](#styledstr) | struct |  |
| [`Styles`](#styles) | struct |  |
| [`BoolValueParser`](#boolvalueparser) | struct |  |
| [`BoolishValueParser`](#boolishvalueparser) | struct |  |
| [`EnumValueParser`](#enumvalueparser) | struct |  |
| [`FalseyValueParser`](#falseyvalueparser) | struct |  |
| [`MapValueParser`](#mapvalueparser) | struct |  |
| [`NonEmptyStringValueParser`](#nonemptystringvalueparser) | struct |  |
| [`OsStringValueParser`](#osstringvalueparser) | struct |  |
| [`PathBufValueParser`](#pathbufvalueparser) | struct |  |
| [`PossibleValuesParser`](#possiblevaluesparser) | struct |  |
| [`RangedI64ValueParser`](#rangedi64valueparser) | struct |  |
| [`RangedU64ValueParser`](#rangedu64valueparser) | struct |  |
| [`StringValueParser`](#stringvalueparser) | struct |  |
| [`TryMapValueParser`](#trymapvalueparser) | struct |  |
| [`UnknownArgumentValueParser`](#unknownargumentvalueparser) | struct |  |
| [`ValueParser`](#valueparser) | struct |  |
| [`ArgAction`](#argaction) | enum |  |
| [`ArgPredicate`](#argpredicate) | enum |  |
| [`Resettable`](#resettable) | enum |  |
| [`ValueHint`](#valuehint) | enum |  |
| [`IntoResettable`](#intoresettable) | trait |  |
| [`TypedValueParser`](#typedvalueparser) | trait |  |
| [`ValueParserFactory`](#valueparserfactory) | trait |  |

## Modules

- [`action`](action/index.md)
- [`app_settings`](app_settings/index.md)
- [`arg`](arg/index.md)
- [`arg_group`](arg_group/index.md)
- [`arg_predicate`](arg_predicate/index.md)
- [`arg_settings`](arg_settings/index.md)
- [`command`](command/index.md)
- [`ext`](ext/index.md)
- [`os_str`](os_str/index.md)
- [`possible_value`](possible_value/index.md)
- [`range`](range/index.md)
- [`resettable`](resettable/index.md)
- [`str`](str/index.md)
- [`styled_str`](styled_str/index.md)
- [`value_hint`](value_hint/index.md)
- [`value_parser`](value_parser/index.md)
- [`debug_asserts`](debug_asserts/index.md)
- [`styling`](styling/index.md) — Terminal [`Styles`] for help and error output

## Structs

### `Str`

```rust
struct Str {
    name: inner::Inner,
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

#### Implementations

- <span id="str-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="str-into-inner"></span>`fn into_inner(self) -> Inner` — [`Inner`](str/inner/index.md)

- <span id="str-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl AsRef for Str`

- <span id="str-as-ref"></span>`fn as_ref(&self) -> &std::path::Path`

##### `impl Clone for Str`

- <span id="str-clone"></span>`fn clone(&self) -> Str` — [`Str`](#str)

##### `impl Debug for Str`

- <span id="str-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- <span id="str-default"></span>`fn default() -> Str` — [`Str`](#str)

##### `impl Deref for Str`

- <span id="str-target"></span>`type Target = str`

- <span id="str-deref"></span>`fn deref(&self) -> &str`

##### `impl Display for Str`

- <span id="str-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl Hash for Str`

- <span id="str-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for Str`

- <span id="str-into-resettable"></span>`fn into_resettable(self) -> Resettable<OsStr>` — [`Resettable`](#resettable), [`OsStr`](#osstr)

##### `impl Ord for Str`

- <span id="str-cmp"></span>`fn cmp(&self, other: &Str) -> cmp::Ordering` — [`Str`](#str)

##### `impl PartialEq for Str`

- <span id="str-eq"></span>`fn eq(&self, other: &str) -> bool`

##### `impl PartialOrd for Str`

- <span id="str-partial-cmp"></span>`fn partial_cmp(&self, other: &Str) -> option::Option<cmp::Ordering>` — [`Str`](#str)

##### `impl<P, T> Receiver for Str`

- <span id="str-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl<T> ToString for Str`

- <span id="str-to-string"></span>`fn to_string(&self) -> String`

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

There are two methods for constructing [`Arg`](../index.md)s, using the builder pattern and setting options
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

- <span id="arg-get-id"></span>`fn get_id(&self) -> &Id` — [`Id`](../index.md)

- <span id="arg-get-help"></span>`fn get_help(&self) -> Option<&StyledStr>` — [`StyledStr`](#styledstr)

- <span id="arg-get-long-help"></span>`fn get_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](#styledstr)

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

- <span id="arg-get-possible-values"></span>`fn get_possible_values(&self) -> Vec<PossibleValue>` — [`PossibleValue`](#possiblevalue)

- <span id="arg-get-value-names"></span>`fn get_value_names(&self) -> Option<&[Str]>` — [`Str`](#str)

- <span id="arg-get-num-args"></span>`fn get_num_args(&self) -> Option<ValueRange>` — [`ValueRange`](#valuerange)

- <span id="arg-get-min-vals"></span>`fn get_min_vals(&self) -> usize`

- <span id="arg-get-value-delimiter"></span>`fn get_value_delimiter(&self) -> Option<char>`

- <span id="arg-get-value-terminator"></span>`fn get_value_terminator(&self) -> Option<&Str>` — [`Str`](#str)

- <span id="arg-get-index"></span>`fn get_index(&self) -> Option<usize>`

- <span id="arg-get-value-hint"></span>`fn get_value_hint(&self) -> ValueHint` — [`ValueHint`](../index.md)

- <span id="arg-get-default-values"></span>`fn get_default_values(&self) -> &[OsStr]` — [`OsStr`](#osstr)

- <span id="arg-is-positional"></span>`fn is_positional(&self) -> bool`

- <span id="arg-is-required-set"></span>`fn is_required_set(&self) -> bool`

- <span id="arg-is-multiple-values-set"></span>`fn is_multiple_values_set(&self) -> bool`

- <span id="arg-is-takes-value-set"></span>`fn is_takes_value_set(&self) -> bool`

- <span id="arg-is-allow-hyphen-values-set"></span>`fn is_allow_hyphen_values_set(&self) -> bool`

- <span id="arg-is-allow-negative-numbers-set"></span>`fn is_allow_negative_numbers_set(&self) -> bool`

- <span id="arg-get-action"></span>`fn get_action(&self) -> &ArgAction` — [`ArgAction`](../index.md)

- <span id="arg-get-value-parser"></span>`fn get_value_parser(&self) -> &super::ValueParser` — [`ValueParser`](#valueparser)

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

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](../index.md)

##### `impl Debug for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](../index.md)

##### `impl Display for Arg`

- <span id="arg-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- <span id="arg-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](../index.md)

##### `impl PartialEq for Arg`

- <span id="arg-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](../index.md)

##### `impl PartialOrd for Arg`

- <span id="arg-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> ToString for Arg`

- <span id="arg-to-string"></span>`fn to_string(&self) -> String`

### `ArgGroup`

```rust
struct ArgGroup {
    id: crate::util::Id,
    args: Vec<crate::util::Id>,
    required: bool,
    requires: Vec<crate::util::Id>,
    conflicts: Vec<crate::util::Id>,
    multiple: bool,
}
```

Specifies a logical group of [arguments]

You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts] or [requirements]
- check which argument in a group was specified on the command-line

For visually grouping arguments in help, see instead
`Arg::help_heading`.

# Examples

The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
the arguments from the specified group is present at runtime.

```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, error::ErrorKind};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor", "patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major", "--patch"]);
// Because we used two args in the group it's an error
assert!(result.is_err());
let err = result.unwrap_err();
assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
```

This next example shows a passing parse of the same scenario
```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, Id};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor","patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major"]);
assert!(result.is_ok());
let matches = result.unwrap();
// We may not know which of the args was used, so we can test for the group...
assert!(matches.contains_id("vers"));
// We can also ask the group which arg was used
assert_eq!(matches
    .get_one::<Id>("vers")
    .expect("`vers` is required")
    .as_str(),
    "major"
);
// we could also alternatively check each arg individually (not shown here)
```




#### Implementations

- <span id="arggroup-get-id"></span>`fn get_id(&self) -> &Id` — [`Id`](../index.md)

- <span id="arggroup-is-required-set"></span>`fn is_required_set(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](../index.md)

##### `impl Debug for ArgGroup`

- <span id="arggroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](../index.md)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](../index.md)

##### `impl StructuralPartialEq for ArgGroup`

### `Command`

```rust
struct Command {
    name: crate::builder::Str,
    long_flag: Option<crate::builder::Str>,
    short_flag: Option<char>,
    display_name: Option<String>,
    bin_name: Option<String>,
    author: Option<crate::builder::Str>,
    version: Option<crate::builder::Str>,
    long_version: Option<crate::builder::Str>,
    about: Option<crate::builder::StyledStr>,
    long_about: Option<crate::builder::StyledStr>,
    before_help: Option<crate::builder::StyledStr>,
    before_long_help: Option<crate::builder::StyledStr>,
    after_help: Option<crate::builder::StyledStr>,
    after_long_help: Option<crate::builder::StyledStr>,
    aliases: Vec<(crate::builder::Str, bool)>,
    short_flag_aliases: Vec<(char, bool)>,
    long_flag_aliases: Vec<(crate::builder::Str, bool)>,
    usage_str: Option<crate::builder::StyledStr>,
    usage_name: Option<String>,
    help_str: Option<crate::builder::StyledStr>,
    disp_ord: Option<usize>,
    template: Option<crate::builder::StyledStr>,
    settings: crate::builder::app_settings::AppFlags,
    g_settings: crate::builder::app_settings::AppFlags,
    args: crate::mkeymap::MKeyMap,
    subcommands: Vec<Command>,
    groups: Vec<crate::builder::ArgGroup>,
    current_help_heading: Option<crate::builder::Str>,
    current_disp_ord: Option<usize>,
    subcommand_value_name: Option<crate::builder::Str>,
    subcommand_heading: Option<crate::builder::Str>,
    external_value_parser: Option<super::ValueParser>,
    long_help_exists: bool,
    deferred: Option<fn(Command) -> Command>,
    app_ext: crate::builder::ext::Extensions,
}
```

Build a command-line interface.

This includes defining arguments, subcommands, parser behavior, and help output.
Once all configuration is complete,
the `Command::get_matches` family of methods starts the runtime-parsing
process. These methods then return information about the user supplied
arguments (or lack thereof).

When deriving a `Parser`, you can use
`CommandFactory::command` to access the
`Command`.

- [Basic API][crate::Command#basic-api]
- [Application-wide Settings][crate::Command#application-wide-settings]
- [Command-specific Settings][crate::Command#command-specific-settings]
- [Subcommand-specific Settings][crate::Command#subcommand-specific-settings]
- [Reflection][crate::Command#reflection]

# Examples

```no_run
use clap_builder as clap;
use clap::{Command, Arg};
let m = Command::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(
        Arg::new("in_file")
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();

// Your program logic starts here...
```


#### Implementations

- <span id="command-no-binary-name"></span>`fn no_binary_name(self, yes: bool) -> Self`

- <span id="command-ignore-errors"></span>`fn ignore_errors(self, yes: bool) -> Self`

- <span id="command-args-override-self"></span>`fn args_override_self(self, yes: bool) -> Self`

- <span id="command-dont-delimit-trailing-values"></span>`fn dont_delimit_trailing_values(self, yes: bool) -> Self`

- <span id="command-color"></span>`fn color(self, color: ColorChoice) -> Self` — [`ColorChoice`](../index.md)

- <span id="command-styles"></span>`fn styles(self, styles: Styles) -> Self` — [`Styles`](#styles)

- <span id="command-term-width"></span>`fn term_width(self, width: usize) -> Self`

- <span id="command-max-term-width"></span>`fn max_term_width(self, width: usize) -> Self`

- <span id="command-disable-version-flag"></span>`fn disable_version_flag(self, yes: bool) -> Self`

- <span id="command-propagate-version"></span>`fn propagate_version(self, yes: bool) -> Self`

- <span id="command-next-line-help"></span>`fn next_line_help(self, yes: bool) -> Self`

- <span id="command-disable-help-flag"></span>`fn disable_help_flag(self, yes: bool) -> Self`

- <span id="command-disable-help-subcommand"></span>`fn disable_help_subcommand(self, yes: bool) -> Self`

- <span id="command-disable-colored-help"></span>`fn disable_colored_help(self, yes: bool) -> Self`

- <span id="command-help-expected"></span>`fn help_expected(self, yes: bool) -> Self`

- <span id="command-hide-possible-values"></span>`fn hide_possible_values(self, yes: bool) -> Self`

- <span id="command-infer-long-args"></span>`fn infer_long_args(self, yes: bool) -> Self`

- <span id="command-infer-subcommands"></span>`fn infer_subcommands(self, yes: bool) -> Self`

#### Trait Implementations

##### `impl Clone for Command`

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](../index.md)

##### `impl Debug for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Command`

- <span id="command-default"></span>`fn default() -> Self`

##### `impl Display for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- <span id="command-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](../index.md)

##### `impl<T> ToString for Command`

- <span id="command-to-string"></span>`fn to_string(&self) -> String`

### `OsStr`

```rust
struct OsStr {
    name: inner::Inner,
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- <span id="osstr-from-static-ref"></span>`fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- <span id="osstr-as-os-str"></span>`fn as_os_str(&self) -> &std::ffi::OsStr`

- <span id="osstr-to-os-string"></span>`fn to_os_string(&self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl AsRef for OsStr`

- <span id="osstr-as-ref"></span>`fn as_ref(&self) -> &std::ffi::OsStr`

##### `impl Clone for OsStr`

- <span id="osstr-clone"></span>`fn clone(&self) -> OsStr` — [`OsStr`](#osstr)

##### `impl Debug for OsStr`

- <span id="osstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- <span id="osstr-default"></span>`fn default() -> OsStr` — [`OsStr`](#osstr)

##### `impl Deref for OsStr`

- <span id="osstr-target"></span>`type Target = OsStr`

- <span id="osstr-deref"></span>`fn deref(&self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl Hash for OsStr`

- <span id="osstr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for OsStr`

- <span id="osstr-into-resettable"></span>`fn into_resettable(self) -> Resettable<OsStr>` — [`Resettable`](#resettable), [`OsStr`](#osstr)

##### `impl Ord for OsStr`

- <span id="osstr-cmp"></span>`fn cmp(&self, other: &OsStr) -> cmp::Ordering` — [`OsStr`](#osstr)

##### `impl PartialEq for OsStr`

- <span id="osstr-eq"></span>`fn eq(&self, other: &&std::ffi::OsStr) -> bool`

##### `impl PartialOrd for OsStr`

- <span id="osstr-partial-cmp"></span>`fn partial_cmp(&self, other: &OsStr) -> option::Option<cmp::Ordering>` — [`OsStr`](#osstr)

##### `impl<P, T> Receiver for OsStr`

- <span id="osstr-target"></span>`type Target = T`

##### `impl StructuralPartialEq for OsStr`

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
to [hide] single values from help messages and shell completions or to attach [`help`](../output/help/index.md) to
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

- <span id="possiblevalue-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](#str)

- <span id="possiblevalue-help"></span>`fn help(self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](#intoresettable), [`StyledStr`](#styledstr)

- <span id="possiblevalue-hide"></span>`fn hide(self, yes: bool) -> Self`

- <span id="possiblevalue-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](#intoresettable), [`Str`](#str)

- <span id="possiblevalue-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](#str)

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

### `ValueRange`

```rust
struct ValueRange {
    start_inclusive: usize,
    end_inclusive: usize,
}
```

Values per occurrence for an argument

#### Implementations

- <span id="valuerange-empty"></span>`const EMPTY: Self`

- <span id="valuerange-single"></span>`const SINGLE: Self`

- <span id="valuerange-optional"></span>`const OPTIONAL: Self`

- <span id="valuerange-full"></span>`const FULL: Self`

- <span id="valuerange-new"></span>`fn new(range: impl Into<Self>) -> Self`

- <span id="valuerange-raw"></span>`fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- <span id="valuerange-min-values"></span>`fn min_values(&self) -> usize`

- <span id="valuerange-max-values"></span>`fn max_values(&self) -> usize`

- <span id="valuerange-takes-values"></span>`fn takes_values(&self) -> bool`

- <span id="valuerange-is-unbounded"></span>`fn is_unbounded(&self) -> bool`

- <span id="valuerange-is-fixed"></span>`fn is_fixed(&self) -> bool`

- <span id="valuerange-is-multiple"></span>`fn is_multiple(&self) -> bool`

- <span id="valuerange-num-values"></span>`fn num_values(&self) -> Option<usize>`

- <span id="valuerange-accepts-more"></span>`fn accepts_more(&self, current: usize) -> bool`

#### Trait Implementations

##### `impl Clone for ValueRange`

- <span id="valuerange-clone"></span>`fn clone(&self) -> ValueRange` — [`ValueRange`](#valuerange)

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- <span id="valuerange-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- <span id="valuerange-default"></span>`fn default() -> Self`

##### `impl Display for ValueRange`

- <span id="valuerange-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl Hash for ValueRange`

- <span id="valuerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for ValueRange`

- <span id="valuerange-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueRange>` — [`Resettable`](#resettable), [`ValueRange`](#valuerange)

##### `impl PartialEq for ValueRange`

- <span id="valuerange-eq"></span>`fn eq(&self, other: &ValueRange) -> bool` — [`ValueRange`](#valuerange)

##### `impl RangeBounds for ValueRange`

- <span id="valuerange-start-bound"></span>`fn start_bound(&self) -> std::ops::Bound<&usize>`

- <span id="valuerange-end-bound"></span>`fn end_bound(&self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl<T> ToString for ValueRange`

- <span id="valuerange-to-string"></span>`fn to_string(&self) -> String`

### `StyledStr`

```rust
struct StyledStr(String);
```

Terminal-styling container

Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)

# Examples

```rust
use clap_builder as clap;
// `cstr!` converts tags to ANSI codes
let after_help: &'static str = color_print::cstr!(
r#"<bold><underline>Examples</underline></bold>

  <dim>$</dim> <bold>mybin --input file.toml</bold>
"#);

let cmd = clap::Command::new("mybin")
    .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
    // ...
  ;
```

#### Implementations

- <span id="styledstr-new"></span>`const fn new() -> Self`

- <span id="styledstr-ansi"></span>`fn ansi(&self) -> impl std::fmt::Display + '_`

- <span id="styledstr-push-string"></span>`fn push_string(&mut self, msg: String)`

- <span id="styledstr-push-str"></span>`fn push_str(&mut self, msg: &str)`

- <span id="styledstr-trim-start-lines"></span>`fn trim_start_lines(&mut self)`

- <span id="styledstr-trim-end"></span>`fn trim_end(&mut self)`

- <span id="styledstr-replace-newline-var"></span>`fn replace_newline_var(&mut self)`

- <span id="styledstr-indent"></span>`fn indent(&mut self, initial: &str, trailing: &str)`

- <span id="styledstr-wrap"></span>`fn wrap(&mut self, _hard_width: usize)`

- <span id="styledstr-display-width"></span>`fn display_width(&self) -> usize`

- <span id="styledstr-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="styledstr-as-styled-str"></span>`fn as_styled_str(&self) -> &str`

- <span id="styledstr-iter-text"></span>`fn iter_text(&self) -> impl Iterator<Item = &str>`

- <span id="styledstr-push-styled"></span>`fn push_styled(&mut self, other: &Self)`

- <span id="styledstr-write-to"></span>`fn write_to(&self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for StyledStr`

- <span id="styledstr-clone"></span>`fn clone(&self) -> StyledStr` — [`StyledStr`](#styledstr)

##### `impl Debug for StyledStr`

- <span id="styledstr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyledStr`

- <span id="styledstr-default"></span>`fn default() -> StyledStr` — [`StyledStr`](#styledstr)

##### `impl Display for StyledStr`

- <span id="styledstr-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<I> IntoResettable for StyledStr`

- <span id="styledstr-into-resettable"></span>`fn into_resettable(self) -> Resettable<StyledStr>` — [`Resettable`](#resettable), [`StyledStr`](#styledstr)

##### `impl Ord for StyledStr`

- <span id="styledstr-cmp"></span>`fn cmp(&self, other: &StyledStr) -> cmp::Ordering` — [`StyledStr`](#styledstr)

##### `impl PartialEq for StyledStr`

- <span id="styledstr-eq"></span>`fn eq(&self, other: &StyledStr) -> bool` — [`StyledStr`](#styledstr)

##### `impl PartialOrd for StyledStr`

- <span id="styledstr-partial-cmp"></span>`fn partial_cmp(&self, other: &StyledStr) -> option::Option<cmp::Ordering>` — [`StyledStr`](#styledstr)

##### `impl StructuralPartialEq for StyledStr`

##### `impl<T> ToString for StyledStr`

- <span id="styledstr-to-string"></span>`fn to_string(&self) -> String`

##### `impl Write for StyledStr`

- <span id="styledstr-write-str"></span>`fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error>`

- <span id="styledstr-write-char"></span>`fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error>`

### `Styles`

```rust
struct Styles {
    header: Style,
    error: Style,
    usage: Style,
    literal: Style,
    placeholder: Style,
    valid: Style,
    invalid: Style,
    context: Style,
    context_value: Option<Style>,
}
```

Terminal styling definitions

See also `Command::styles`.

# Example

clap v3 styling
```rust
use clap_builder as clap;
use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

#### Implementations

- <span id="styles-plain"></span>`const fn plain() -> Self`

- <span id="styles-styled"></span>`const fn styled() -> Self`

- <span id="styles-header"></span>`const fn header(self, style: Style) -> Self`

- <span id="styles-error"></span>`const fn error(self, style: Style) -> Self`

- <span id="styles-usage"></span>`const fn usage(self, style: Style) -> Self`

- <span id="styles-literal"></span>`const fn literal(self, style: Style) -> Self`

- <span id="styles-placeholder"></span>`const fn placeholder(self, style: Style) -> Self`

- <span id="styles-valid"></span>`const fn valid(self, style: Style) -> Self`

- <span id="styles-invalid"></span>`const fn invalid(self, style: Style) -> Self`

- <span id="styles-context"></span>`const fn context(self, style: Style) -> Self`

- <span id="styles-context-value"></span>`const fn context_value(self, style: Style) -> Self`

#### Trait Implementations

##### `impl AppExt for Styles`

##### `impl Clone for Styles`

- <span id="styles-clone"></span>`fn clone(&self) -> Styles` — [`Styles`](#styles)

##### `impl Debug for Styles`

- <span id="styles-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Styles`

- <span id="styles-default"></span>`fn default() -> Self`

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="boolvalueparser-new"></span>`fn new() -> Self`

- <span id="boolvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](#possiblevalue)

#### Trait Implementations

##### `impl Clone for BoolValueParser`

- <span id="boolvalueparser-clone"></span>`fn clone(&self) -> BoolValueParser` — [`BoolValueParser`](#boolvalueparser)

##### `impl Copy for BoolValueParser`

##### `impl Debug for BoolValueParser`

- <span id="boolvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolValueParser`

- <span id="boolvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for BoolValueParser`

- <span id="boolvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolValueParser`

- <span id="boolvalueparser-value"></span>`type Value = bool`

- <span id="boolvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="boolvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

Parse bool-like string values

See also:
- `ValueParser::bool` for different human readable bool representations
- [`FalseyValueParser`](#falseyvalueparser) for assuming non-false is true

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::BoolishValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::BoolishValueParser::new();
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("true")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("Yes")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oN")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("1")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- <span id="boolishvalueparser-new"></span>`fn new() -> Self`

- <span id="boolishvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](#possiblevalue)

#### Trait Implementations

##### `impl Clone for BoolishValueParser`

- <span id="boolishvalueparser-clone"></span>`fn clone(&self) -> BoolishValueParser` — [`BoolishValueParser`](#boolishvalueparser)

##### `impl Copy for BoolishValueParser`

##### `impl Debug for BoolishValueParser`

- <span id="boolishvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolishValueParser`

- <span id="boolishvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for BoolishValueParser`

- <span id="boolishvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolishValueParser`

- <span id="boolishvalueparser-value"></span>`type Value = bool`

- <span id="boolishvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="boolishvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(std::marker::PhantomData<E>);
```

Parse an `ValueEnum` value.

See also:
- [`PossibleValuesParser`](#possiblevaluesparser)

# Example

```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::ColorChoice;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;

// Usage
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::EnumValueParser::<ColorChoice>::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: ColorChoice = *m.get_one("color")
    .expect("required");
assert_eq!(port, ColorChoice::Always);

// Semantics
let value_parser = clap::builder::EnumValueParser::<ColorChoice>::new();
// or
let value_parser = clap::value_parser!(ColorChoice);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), ColorChoice::Always);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), ColorChoice::Auto);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), ColorChoice::Never);
```

#### Implementations

- <span id="enumvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<E: clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static> Clone for EnumValueParser<E>`

- <span id="enumvalueparser-clone"></span>`fn clone(&self) -> EnumValueParser<E>` — [`EnumValueParser`](#enumvalueparser)

##### `impl<E: fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static> Debug for EnumValueParser<E>`

- <span id="enumvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> Default for EnumValueParser<E>`

- <span id="enumvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for EnumValueParser<E>`

- <span id="enumvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> TypedValueParser for EnumValueParser<E>`

- <span id="enumvalueparser-value"></span>`type Value = E`

- <span id="enumvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="enumvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

Parse false-like string values, everything else is `true`

See also:
- `ValueParser::bool` for assuming non-false is true
- [`BoolishValueParser`](#boolishvalueparser) for different human readable bool representations

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::FalseyValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::FalseyValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- <span id="falseyvalueparser-new"></span>`fn new() -> Self`

- <span id="falseyvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](#possiblevalue)

#### Trait Implementations

##### `impl Clone for FalseyValueParser`

- <span id="falseyvalueparser-clone"></span>`fn clone(&self) -> FalseyValueParser` — [`FalseyValueParser`](#falseyvalueparser)

##### `impl Copy for FalseyValueParser`

##### `impl Debug for FalseyValueParser`

- <span id="falseyvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FalseyValueParser`

- <span id="falseyvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for FalseyValueParser`

- <span id="falseyvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for FalseyValueParser`

- <span id="falseyvalueparser-value"></span>`type Value = bool`

- <span id="falseyvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="falseyvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `MapValueParser<P, F>`

```rust
struct MapValueParser<P, F> {
    parser: P,
    func: F,
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::map`

#### Implementations

- <span id="mapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: clone::Clone, F: clone::Clone> Clone for MapValueParser<P, F>`

- <span id="mapvalueparser-clone"></span>`fn clone(&self) -> MapValueParser<P, F>` — [`MapValueParser`](#mapvalueparser)

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for MapValueParser<P, F>`

- <span id="mapvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for MapValueParser<P, F>`

- <span id="mapvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl<P, F, T> TypedValueParser for MapValueParser<P, F>`

- <span id="mapvalueparser-value"></span>`type Value = T`

- <span id="mapvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="mapvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="mapvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `NonEmptyStringValueParser`

```rust
struct NonEmptyStringValueParser {
}
```

Parse non-empty string values

See also:
- `ValueParser::string`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: &String = m.get_one("append")
    .expect("required");
assert_eq!(port, "true");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::NonEmptyStringValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), "random");
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
```

#### Implementations

- <span id="nonemptystringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-clone"></span>`fn clone(&self) -> NonEmptyStringValueParser` — [`NonEmptyStringValueParser`](#nonemptystringvalueparser)

##### `impl Copy for NonEmptyStringValueParser`

##### `impl Debug for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-value"></span>`type Value = String`

- <span id="nonemptystringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="osstringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for OsStringValueParser`

- <span id="osstringvalueparser-clone"></span>`fn clone(&self) -> OsStringValueParser` — [`OsStringValueParser`](#osstringvalueparser)

##### `impl Copy for OsStringValueParser`

##### `impl Debug for OsStringValueParser`

- <span id="osstringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OsStringValueParser`

- <span id="osstringvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for OsStringValueParser`

- <span id="osstringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for OsStringValueParser`

- <span id="osstringvalueparser-value"></span>`type Value = OsString`

- <span id="osstringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="osstringvalueparser-parse"></span>`fn parse(&self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="pathbufvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for PathBufValueParser`

- <span id="pathbufvalueparser-clone"></span>`fn clone(&self) -> PathBufValueParser` — [`PathBufValueParser`](#pathbufvalueparser)

##### `impl Copy for PathBufValueParser`

##### `impl Debug for PathBufValueParser`

- <span id="pathbufvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathBufValueParser`

- <span id="pathbufvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for PathBufValueParser`

- <span id="pathbufvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PathBufValueParser`

- <span id="pathbufvalueparser-value"></span>`type Value = PathBuf`

- <span id="pathbufvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="pathbufvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `PossibleValuesParser`

```rust
struct PossibleValuesParser(Vec<super::PossibleValue>);
```

Verify the value is from an enumerated set of `PossibleValue`.

See also:
- [`EnumValueParser`](#enumvalueparser) for directly supporting `ValueEnum` types
- `TypedValueParser::map` for adapting values to a more specialized type, like an external
  enums that can't implement `ValueEnum`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::PossibleValuesParser::new(["always", "auto", "never"]))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: &String = m.get_one("color")
    .expect("required");
assert_eq!(port, "always");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::PossibleValuesParser::new(["always", "auto", "never"]);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), "always");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), "auto");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), "never");
```

#### Implementations

- <span id="possiblevaluesparser-new"></span>`fn new(values: impl Into<PossibleValuesParser>) -> Self` — [`PossibleValuesParser`](#possiblevaluesparser)

#### Trait Implementations

##### `impl Clone for PossibleValuesParser`

- <span id="possiblevaluesparser-clone"></span>`fn clone(&self) -> PossibleValuesParser` — [`PossibleValuesParser`](#possiblevaluesparser)

##### `impl Debug for PossibleValuesParser`

- <span id="possiblevaluesparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for PossibleValuesParser`

- <span id="possiblevaluesparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PossibleValuesParser`

- <span id="possiblevaluesparser-value"></span>`type Value = String`

- <span id="possiblevaluesparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="possiblevaluesparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`Error`](../index.md)

- <span id="possiblevaluesparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync>`

```rust
struct RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync> {
    bounds: (std::ops::Bound<i64>, std::ops::Bound<i64>),
    target: std::marker::PhantomData<T>,
}
```

Parse number that fall within a range of values

<div class="warning">

**NOTE:** To capture negative values, you will also need to set
`Arg::allow_negative_numbers` or
`Arg::allow_hyphen_values`.

</div>

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedI64ValueParser::<i32>::new().range(-1..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).unwrap(), -1);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- <span id="rangedi64valueparser-new"></span>`fn new() -> Self`

- <span id="rangedi64valueparser-range"></span>`fn range<B: RangeBounds<i64>>(self, range: B) -> Self`

- <span id="rangedi64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T: clone::Clone + TryFrom<i64> + Clone + Send + Sync> Clone for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-clone"></span>`fn clone(&self) -> RangedI64ValueParser<T>` — [`RangedI64ValueParser`](#rangedi64valueparser)

##### `impl<T: marker::Copy + TryFrom<i64> + Clone + Send + Sync> Copy for RangedI64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<i64> + Clone + Send + Sync> Debug for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync> Default for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<i64> + Clone + Send + Sync + 'static> TypedValueParser for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-value"></span>`type Value = T`

- <span id="rangedi64valueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `RangedU64ValueParser<T: TryFrom<u64>>`

```rust
struct RangedU64ValueParser<T: TryFrom<u64>> {
    bounds: (std::ops::Bound<u64>, std::ops::Bound<u64>),
    target: std::marker::PhantomData<T>,
}
```

Parse number that fall within a range of values

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u64).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u64 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedU64ValueParser::<u32>::new().range(0..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- <span id="rangedu64valueparser-new"></span>`fn new() -> Self`

- <span id="rangedu64valueparser-range"></span>`fn range<B: RangeBounds<u64>>(self, range: B) -> Self`

- <span id="rangedu64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T: clone::Clone + TryFrom<u64>> Clone for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-clone"></span>`fn clone(&self) -> RangedU64ValueParser<T>` — [`RangedU64ValueParser`](#rangedu64valueparser)

##### `impl<T: marker::Copy + TryFrom<u64>> Copy for RangedU64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<u64>> Debug for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<u64>> Default for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<u64> + Clone + Send + Sync + 'static> TypedValueParser for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-value"></span>`type Value = T`

- <span id="rangedu64valueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `StringValueParser`

```rust
struct StringValueParser {
}
```

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="stringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for StringValueParser`

- <span id="stringvalueparser-clone"></span>`fn clone(&self) -> StringValueParser` — [`StringValueParser`](#stringvalueparser)

##### `impl Copy for StringValueParser`

##### `impl Debug for StringValueParser`

- <span id="stringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StringValueParser`

- <span id="stringvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for StringValueParser`

- <span id="stringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for StringValueParser`

- <span id="stringvalueparser-value"></span>`type Value = String`

- <span id="stringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="stringvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `TryMapValueParser<P, F>`

```rust
struct TryMapValueParser<P, F> {
    parser: P,
    func: F,
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::try_map`

#### Implementations

- <span id="trymapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: clone::Clone, F: clone::Clone> Clone for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-clone"></span>`fn clone(&self) -> TryMapValueParser<P, F>` — [`TryMapValueParser`](#trymapvalueparser)

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl<P, F, T, E> TypedValueParser for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-value"></span>`type Value = T`

- <span id="trymapvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="trymapvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](#possiblevalue)

### `UnknownArgumentValueParser`

```rust
struct UnknownArgumentValueParser {
    arg: Option<crate::builder::Str>,
    suggestions: Vec<crate::builder::StyledStr>,
}
```

When encountered, report `ErrorKind::UnknownArgument`

Useful to help users migrate, either from old versions or similar tools.

# Examples

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
let cmd = Command::new("mycmd")
    .args([
        Arg::new("current-dir")
            .short('C'),
        Arg::new("current-dir-unknown")
            .long("cwd")
            .aliases(["current-dir", "directory", "working-directory", "root"])
            .value_parser(clap::builder::UnknownArgumentValueParser::suggest_arg("-C"))
            .hide(true),
    ]);

// Use a supported version of the argument
let matches = cmd.clone().try_get_matches_from(["mycmd", "-C", ".."]).unwrap();
assert!(matches.contains_id("current-dir"));
assert_eq!(
    matches.get_many::<String>("current-dir").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    vec![".."]
);

// Use one of the invalid versions
let err = cmd.try_get_matches_from(["mycmd", "--cwd", ".."]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
```

#### Implementations

- <span id="unknownargumentvalueparser-suggest-arg"></span>`fn suggest_arg(arg: impl Into<Str>) -> Self` — [`Str`](#str)

- <span id="unknownargumentvalueparser-suggest"></span>`fn suggest(text: impl Into<StyledStr>) -> Self` — [`StyledStr`](#styledstr)

- <span id="unknownargumentvalueparser-and-suggest"></span>`fn and_suggest(self, text: impl Into<StyledStr>) -> Self` — [`StyledStr`](#styledstr)

#### Trait Implementations

##### `impl Clone for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-clone"></span>`fn clone(&self) -> UnknownArgumentValueParser` — [`UnknownArgumentValueParser`](#unknownargumentvalueparser)

##### `impl Debug for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-value"></span>`type Value = String`

- <span id="unknownargumentvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

- <span id="unknownargumentvalueparser-parse-ref"></span>`fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../index.md), [`Arg`](../index.md), [`ValueSource`](../parser/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../index.md)

### `ValueParser`

```rust
struct ValueParser(ValueParserInner);
```

Parse/validate argument values

Specified with `Arg::value_parser`.

`ValueParser` defines how to convert a raw argument value into a validated and typed value for
use within an application.

See
- `value_parser!` for automatically selecting an implementation for a given type
- `ValueParser::new` for additional [`TypedValueParser`](#typedvalueparser) that can be used

# Example

```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .long("color")
            .value_parser(["always", "auto", "never"])
            .default_value("auto")
    )
    .arg(
        clap::Arg::new("hostname")
            .long("hostname")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .action(clap::ArgAction::Set)
            .required(true)
    )
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(
    ["cmd", "--hostname", "rust-lang.org", "--port", "3001"]
).unwrap();

let color: &String = m.get_one("color")
    .expect("default");
assert_eq!(color, "auto");

let hostname: &String = m.get_one("hostname")
    .expect("required");
assert_eq!(hostname, "rust-lang.org");

let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

#### Implementations

- <span id="valueparser-new"></span>`fn new<P>(other: P) -> Self`

- <span id="valueparser-bool"></span>`const fn bool() -> Self`

- <span id="valueparser-string"></span>`const fn string() -> Self`

- <span id="valueparser-os-string"></span>`const fn os_string() -> Self`

- <span id="valueparser-path-buf"></span>`const fn path_buf() -> Self`

#### Trait Implementations

##### `impl Clone for ValueParser`

- <span id="valueparser-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for ValueParser`

- <span id="valueparser-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl<I> IntoResettable for ValueParser`

- <span id="valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](#resettable), [`ValueParser`](#valueparser)

## Enums

### `ArgAction`

```rust
enum ArgAction {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
    Help,
    HelpShort,
    HelpLong,
    Version,
}
```

Behavior of arguments when they are encountered while parsing

# Examples

```rust
#[cfg(feature = "help")] {
use clap_builder as clap;
use clap::Command;
use clap::Arg;
let cmd = Command::new("mycmd")
    .arg(
        Arg::new("special-help")
            .short('?')
            .action(clap::ArgAction::Help)
    );

// Existing help still exists
let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);

// New help available
let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}
```

#### Variants

- **`Set`**

  When encountered, store the associated value(s) in `ArgMatches`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::Set)
      );
  
  let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
      vec!["value"]
  );
  ```

- **`Append`**

  When encountered, store the associated value(s) in `ArgMatches`
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::Append)
      );
  
  let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value1", "--flag", "value2"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
      vec!["value1", "value2"]
  );
  ```

- **`SetTrue`**

  When encountered, act as if `"true"` was encountered on the command-line
  
  If no `default_value` is set, it will be `false`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::SetTrue)
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      true
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      false
  );
  ```
  
  You can use `TypedValueParser::map` to have the
  flag control an application-specific type:
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  use clap::builder::TypedValueParser as _;
  use clap::builder::BoolishValueParser;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::SetTrue)
              .value_parser(
                  BoolishValueParser::new()
                  .map(|b| -> usize {
                      if b { 10 } else { 5 }
                  })
              )
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_one::<usize>("flag").copied(),
      Some(10)
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_one::<usize>("flag").copied(),
      Some(5)
  );
  ```

- **`SetFalse`**

  When encountered, act as if `"false"` was encountered on the command-line
  
  If no `default_value` is set, it will be `true`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::SetFalse)
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      false
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      true
  );
  ```

- **`Count`**

  When encountered, increment a `u8` counter starting from `0`.
  
  If no `default_value` is set, it will be `0`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::Count)
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_count("flag"),
      2
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_count("flag"),
      0
  );
  ```

- **`Help`**

  When encountered, display `Command::print_help`
  
  Depending on the flag, `Command::print_long_help` may be shown
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::Help)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`HelpShort`**

  When encountered, display `Command::print_help`
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::HelpShort)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`HelpLong`**

  When encountered, display `Command::print_long_help`
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::HelpLong)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`Version`**

  When encountered, display `Command::version`
  
  Depending on the flag, `Command::long_version` may be shown
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .version("1.0.0")
      .arg(
          Arg::new("special-version")
              .long("special-version")
              .action(clap::ArgAction::Version)
      );
  
  // Existing version still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "--version"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
  
  // New version available
  let err = cmd.try_get_matches_from(["mycmd", "--special-version"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
  ```

#### Implementations

- <span id="argaction-takes-values"></span>`fn takes_values(&self) -> bool`

- <span id="argaction-max-num-args"></span>`fn max_num_args(&self) -> ValueRange` — [`ValueRange`](#valuerange)

- <span id="argaction-default-num-args"></span>`fn default_num_args(&self) -> ValueRange` — [`ValueRange`](#valuerange)

- <span id="argaction-default-value"></span>`fn default_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-missing-value"></span>`fn default_missing_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-value-parser"></span>`fn default_value_parser(&self) -> Option<super::ValueParser>` — [`ValueParser`](#valueparser)

- <span id="argaction-value-type-id"></span>`fn value_type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](../util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for ArgAction`

- <span id="argaction-clone"></span>`fn clone(&self) -> ArgAction` — [`ArgAction`](../index.md)

##### `impl Debug for ArgAction`

- <span id="argaction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoResettable for crate::builder::ArgAction`

- <span id="cratebuilderargaction-into-resettable"></span>`fn into_resettable(self) -> Resettable<ArgAction>` — [`Resettable`](#resettable), [`ArgAction`](../index.md)

### `ArgPredicate`

```rust
enum ArgPredicate {
    IsPresent,
    Equals(crate::builder::OsStr),
}
```

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl Clone for ArgPredicate`

- <span id="argpredicate-clone"></span>`fn clone(&self) -> ArgPredicate` — [`ArgPredicate`](#argpredicate)

##### `impl Debug for ArgPredicate`

- <span id="argpredicate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl PartialEq for ArgPredicate`

- <span id="argpredicate-eq"></span>`fn eq(&self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](#argpredicate)

##### `impl StructuralPartialEq for ArgPredicate`

### `Resettable<T>`

```rust
enum Resettable<T> {
    Value(T),
    Reset,
}
```

Clearable builder value

This allows a builder function to both accept any value that can `Into::into` `T` (like
`&str` into `OsStr`) as well as `None` to reset it to the default.  This is needed to
workaround a limitation where you can't have a function argument that is `impl Into<Option<T>>`
where `T` is `impl Into<S>` accept `None` as its type is ambiguous.

# Example

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
fn common() -> Command {
    Command::new("cli")
        .arg(Arg::new("input").short('i').long("input"))
}
let mut command = common();
command.mut_arg("input", |arg| arg.short(None));
```

#### Variants

- **`Value`**

  Overwrite builder value

- **`Reset`**

  Reset builder value

#### Implementations

- <span id="resettable-into-option"></span>`fn into_option(self) -> Option<T>`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Resettable<T>`

- <span id="resettable-clone"></span>`fn clone(&self) -> Resettable<T>` — [`Resettable`](#resettable)

##### `impl<T: marker::Copy> Copy for Resettable<T>`

##### `impl<T: fmt::Debug> Debug for Resettable<T>`

- <span id="resettable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Resettable<T>`

##### `impl<T: hash::Hash> Hash for Resettable<T>`

- <span id="resettable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> IntoResettable for Resettable<T>`

- <span id="resettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<T>` — [`Resettable`](#resettable)

##### `impl<T: cmp::Ord> Ord for Resettable<T>`

- <span id="resettable-cmp"></span>`fn cmp(&self, other: &Resettable<T>) -> cmp::Ordering` — [`Resettable`](#resettable)

##### `impl<T: cmp::PartialEq> PartialEq for Resettable<T>`

- <span id="resettable-eq"></span>`fn eq(&self, other: &Resettable<T>) -> bool` — [`Resettable`](#resettable)

##### `impl<T: cmp::PartialOrd> PartialOrd for Resettable<T>`

- <span id="resettable-partial-cmp"></span>`fn partial_cmp(&self, other: &Resettable<T>) -> option::Option<cmp::Ordering>` — [`Resettable`](#resettable)

##### `impl<T> StructuralPartialEq for Resettable<T>`

### `ValueHint`

```rust
enum ValueHint {
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
}
```

Provide shell with hint on how to complete an argument.

See `Arg::value_hint` to set this on an argument.

See the `clap_complete` crate for completion script generation.

Overview of which hints are supported by which shell:

| Hint                   | zsh | fish[^1] | dynamic |
| ---------------------- | --- | ---------|---------|
| `AnyPath`              | Yes | Yes      | Yes     |
| `FilePath`             | Yes | Yes      | Yes     |
| `DirPath`              | Yes | Yes      | Yes     |
| `ExecutablePath`       | Yes | Partial  | Yes     |
| `CommandName`          | Yes | Yes      | No      |
| `CommandString`        | Yes | Partial  | No      |
| `CommandWithArguments` | Yes |          | No      |
| `Username`             | Yes | Yes      | No      |
| `Hostname`             | Yes | Yes      | No      |
| `Url`                  | Yes |          | No      |
| `EmailAddress`         | Yes |          | No      |

[^1]: fish completions currently only support named arguments (e.g. -o or --opt), not
      positional arguments.

#### Variants

- **`Unknown`**

  Default value if hint is not specified. Follows shell default behavior, which is usually
  auto-completing filenames.

- **`Other`**

  None of the hints below apply. Disables shell completion for this argument.

- **`AnyPath`**

  Any existing path.

- **`FilePath`**

  Path to a file.

- **`DirPath`**

  Path to a directory.

- **`ExecutablePath`**

  Path to an executable file.

- **`CommandName`**

  Name of a command, without arguments. May be relative to PATH, or full path to executable.

- **`CommandString`**

  A single string containing a command and its arguments.

- **`CommandWithArguments`**

  Capture the remaining arguments as a command name and arguments for that command. This is
  common when writing shell wrappers that execute anther command, for example `sudo` or `env`.
  
  This hint is special, the argument must be a positional argument and have
  `.num_args(1..)` and Command must use `Command::trailing_var_arg(true)`. The result is that the
  command line `my_app ls -la /` will be parsed as `["ls", "-la", "/"]` and clap won't try to
  parse the `-la` argument itself.
  
  

- **`Username`**

  Name of a local operating system user.

- **`Hostname`**

  Host name of a computer.
  Shells usually parse `/etc/hosts` and `.ssh/known_hosts` to complete hostnames.

- **`Url`**

  Complete web address.

- **`EmailAddress`**

  Email address.

#### Trait Implementations

##### `impl Clone for ValueHint`

- <span id="valuehint-clone"></span>`fn clone(&self) -> ValueHint` — [`ValueHint`](../index.md)

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- <span id="valuehint-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ValueHint`

- <span id="valuehint-default"></span>`fn default() -> ValueHint` — [`ValueHint`](../index.md)

##### `impl Eq for ValueHint`

##### `impl FromStr for ValueHint`

- <span id="valuehint-err"></span>`type Err = String`

- <span id="valuehint-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- <span id="valuehint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoResettable for crate::builder::ValueHint`

- <span id="cratebuildervaluehint-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueHint>` — [`Resettable`](#resettable), [`ValueHint`](../index.md)

##### `impl PartialEq for ValueHint`

- <span id="valuehint-eq"></span>`fn eq(&self, other: &ValueHint) -> bool` — [`ValueHint`](../index.md)

##### `impl StructuralPartialEq for ValueHint`

## Traits

### `IntoResettable<T>`

```rust
trait IntoResettable<T> { ... }
```

Convert to the intended resettable type

#### Required Methods

- `fn into_resettable(self) -> Resettable<T>`

  Convert to the intended resettable type

#### Implementors

- [`ArgAction`](../index.md)
- [`BoolValueParser`](#boolvalueparser)
- [`BoolishValueParser`](#boolishvalueparser)
- [`EnumValueParser`](#enumvalueparser)
- [`FalseyValueParser`](#falseyvalueparser)
- [`Id`](../index.md)
- [`MapValueParser`](#mapvalueparser)
- [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
- [`OsStr`](#osstr)
- [`OsStringValueParser`](#osstringvalueparser)
- [`PathBufValueParser`](#pathbufvalueparser)
- [`PossibleValuesParser`](#possiblevaluesparser)
- [`RangedI64ValueParser`](#rangedi64valueparser)
- [`RangedU64ValueParser`](#rangedu64valueparser)
- [`Resettable`](#resettable)
- [`Str`](#str)
- [`StringValueParser`](#stringvalueparser)
- [`StyledStr`](#styledstr)
- [`TryMapValueParser`](#trymapvalueparser)
- [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
- [`ValueHint`](../index.md)
- [`ValueParser`](#valueparser)
- [`ValueRange`](#valuerange)
- `I`
- `Option<&'static str>`
- `Option<char>`
- `Option<crate::builder::ArgAction>`
- `Option<crate::builder::ValueHint>`
- `Option<crate::builder::ValueParser>`
- `Option<usize>`
- `char`
- `usize`

### `TypedValueParser`

```rust
trait TypedValueParser: Clone + Send + Sync + 'static { ... }
```

Parse/validate argument values

As alternatives to implementing `TypedValueParser`,
- Use `Fn(&str) -> Result<T, E>` which implements `TypedValueParser`
- `TypedValueParser::map` or `TypedValueParser::try_map` to adapt an existing `TypedValueParser`

See `ValueParserFactory` to register `TypedValueParser::Value` with
`value_parser!`.

# Example

```rust
#[cfg(feature = "error-context")] {
use clap_builder as clap;
use clap::error::ErrorKind;
use clap::error::ContextKind;
use clap::error::ContextValue;
#[derive(Clone)]
struct Custom(u32);

#[derive(Clone)]
struct CustomValueParser;

impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;

        const INVALID_VALUE: u32 = 10;
        if val == INVALID_VALUE {
            let mut err = clap::Error::new(ErrorKind::ValueValidation)
                .with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(ContextKind::InvalidArg, ContextValue::String(arg.to_string()));
            }
            err.insert(ContextKind::InvalidValue, ContextValue::String(INVALID_VALUE.to_string()));
            return Err(err);
        }

        Ok(Custom(val))
    }
}
}
```

#### Associated Types

- `type Value: 3`

#### Required Methods

- `fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

#### Provided Methods

- `fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

  Reflect on enumerated value properties

- `fn map<T, F>(self, func: F) -> MapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

- `fn try_map<T, E, F>(self, func: F) -> TryMapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

#### Implementors

- [`BoolValueParser`](#boolvalueparser)
- [`BoolishValueParser`](#boolishvalueparser)
- [`EnumValueParser`](#enumvalueparser)
- [`FalseyValueParser`](#falseyvalueparser)
- [`MapValueParser`](#mapvalueparser)
- [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
- [`OsStringValueParser`](#osstringvalueparser)
- [`PathBufValueParser`](#pathbufvalueparser)
- [`PossibleValuesParser`](#possiblevaluesparser)
- [`RangedI64ValueParser`](#rangedi64valueparser)
- [`RangedU64ValueParser`](#rangedu64valueparser)
- [`StringValueParser`](#stringvalueparser)
- [`TryMapValueParser`](#trymapvalueparser)
- [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
- `F`

### `ValueParserFactory`

```rust
trait ValueParserFactory { ... }
```

Register a type with [`value_parser!`][crate::value_parser!]

# Example

```rust
use clap_builder as clap;
#[derive(Copy, Clone, Debug)]
pub struct Custom(u32);

impl clap::builder::ValueParserFactory for Custom {
    type Parser = CustomValueParser;
    fn value_parser() -> Self::Parser {
        CustomValueParser
    }
}

#[derive(Clone, Debug)]
pub struct CustomValueParser;
impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Custom(val))
    }
}

let parser: CustomValueParser = clap::value_parser!(Custom);
```

#### Associated Types

- `type Parser`

#### Required Methods

- `fn value_parser() -> <Self as >::Parser`

  Create the specified `Self::Parser`

#### Implementors

- `Box<T>`
- `Box<std::ffi::OsStr>`
- `Box<std::path::Path>`
- `Box<str>`
- `String`
- `bool`
- `i16`
- `i32`
- `i64`
- `i8`
- `std::ffi::OsString`
- `std::num::Saturating<T>`
- `std::num::Wrapping<T>`
- `std::path::PathBuf`
- `std::sync::Arc<T>`
- `u16`
- `u32`
- `u64`
- `u8`

