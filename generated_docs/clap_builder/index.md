# Crate `clap_builder`

Builder implementation for clap.

[docs.rs](https://docs.rs/clap)
- [Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [Derive Reference](https://docs.rs/clap/latest/clap/_derive/index.html)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/license/mit>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`derive`](#derive)
  - [`builder`](#builder)
  - [`error`](#error)
  - [`parser`](#parser)
  - [`mkeymap`](#mkeymap)
  - [`output`](#output)
  - [`util`](#util)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Enums](#enums)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Type Aliases](#type-aliases)
  - [`Error`](#error)
- [Constants](#constants)
  - [`INTERNAL_ERROR_MSG`](#internal_error_msg)
- [Macros](#macros)
  - [`command!`](#command)
  - [`arg!`](#arg)
  - [`value_parser!`](#value_parser)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`derive`](#derive) | mod | This module contains traits that are usable with the `#[derive(...)]` |
| [`builder`](#builder) | mod | Define [`Command`] line [arguments][`Arg`] |
| [`error`](#error) | mod | Error reporting |
| [`parser`](#parser) | mod | [`Command`][crate::Command] line argument parser |
| [`mkeymap`](#mkeymap) | mod |  |
| [`output`](#output) | mod |  |
| [`util`](#util) | mod |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | enum |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`Error`](#error) | type | Command Line Argument Parser Error |
| [`INTERNAL_ERROR_MSG`](#internal_error_msg) | const |  |
| [`command!`](#command) | macro | Requires `cargo` feature flag to be enabled. |
| [`arg!`](#arg) | macro | Create an [`Arg`] from a usage string. |
| [`value_parser!`](#value_parser) | macro | Select a [`ValueParser`] implementation from the intended type |

## Modules

- [`macros`](macros/index.md) - 
- [`derive`](derive/index.md) - This module contains traits that are usable with the `#[derive(...)]`
- [`builder`](builder/index.md) - Define [`Command`] line [arguments][`Arg`]
- [`error`](error/index.md) - Error reporting
- [`parser`](parser/index.md) - [`Command`][crate::Command] line argument parser
- [`mkeymap`](mkeymap/index.md) - 
- [`output`](output/index.md) - 
- [`util`](util/index.md) - 

## Structs

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

- <span id="command-get-usage-name"></span>`fn get_usage_name(&self) -> Option<&str>`

- <span id="command-get-usage-name-fallback"></span>`fn get_usage_name_fallback(&self) -> &str`

- <span id="command-get-display-name"></span>`fn get_display_name(&self) -> Option<&str>`

- <span id="command-get-bin-name"></span>`fn get_bin_name(&self) -> Option<&str>`

- <span id="command-get-bin-name-fallback"></span>`fn get_bin_name_fallback(&self) -> &str`

- <span id="command-set-bin-name"></span>`fn set_bin_name(&mut self, name: impl Into<String>)`

- <span id="command-get-name"></span>`fn get_name(&self) -> &str`

- <span id="command-get-name-str"></span>`fn get_name_str(&self) -> &Str` — [`Str`](builder/index.md)

- <span id="command-get-name-and-visible-aliases"></span>`fn get_name_and_visible_aliases(&self) -> Vec<&str>`

- <span id="command-get-version"></span>`fn get_version(&self) -> Option<&str>`

- <span id="command-get-long-version"></span>`fn get_long_version(&self) -> Option<&str>`

- <span id="command-get-display-order"></span>`fn get_display_order(&self) -> usize`

- <span id="command-get-author"></span>`fn get_author(&self) -> Option<&str>`

- <span id="command-get-short-flag"></span>`fn get_short_flag(&self) -> Option<char>`

- <span id="command-get-long-flag"></span>`fn get_long_flag(&self) -> Option<&str>`

- <span id="command-get-about"></span>`fn get_about(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-get-long-about"></span>`fn get_long_about(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-is-flatten-help-set"></span>`fn is_flatten_help_set(&self) -> bool`

- <span id="command-get-next-help-heading"></span>`fn get_next_help_heading(&self) -> Option<&str>`

- <span id="command-get-visible-aliases"></span>`fn get_visible_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-visible-short-flag-aliases"></span>`fn get_visible_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_`

- <span id="command-get-visible-long-flag-aliases"></span>`fn get_visible_long_flag_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-all-aliases"></span>`fn get_all_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-all-short-flag-aliases"></span>`fn get_all_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_`

- <span id="command-get-all-long-flag-aliases"></span>`fn get_all_long_flag_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-aliases"></span>`fn get_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-is-set"></span>`fn is_set(&self, s: AppSettings) -> bool` — [`AppSettings`](builder/app_settings/index.md)

- <span id="command-get-color"></span>`fn get_color(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

- <span id="command-get-styles"></span>`fn get_styles(&self) -> &Styles` — [`Styles`](builder/index.md)

- <span id="command-get-subcommands"></span>`fn get_subcommands(&self) -> impl Iterator<Item = &Command>` — [`Command`](#command)

- <span id="command-get-subcommands-mut"></span>`fn get_subcommands_mut(&mut self) -> impl Iterator<Item = &mut Command>` — [`Command`](#command)

- <span id="command-has-subcommands"></span>`fn has_subcommands(&self) -> bool`

- <span id="command-get-subcommand-help-heading"></span>`fn get_subcommand_help_heading(&self) -> Option<&str>`

- <span id="command-get-subcommand-value-name"></span>`fn get_subcommand_value_name(&self) -> Option<&str>`

- <span id="command-get-before-help"></span>`fn get_before_help(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-get-before-long-help"></span>`fn get_before_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-get-after-help"></span>`fn get_after_help(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-get-after-long-help"></span>`fn get_after_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](builder/index.md)

- <span id="command-find-subcommand"></span>`fn find_subcommand(&self, name: impl AsRef<std::ffi::OsStr>) -> Option<&Command>` — [`Command`](#command)

- <span id="command-find-subcommand-mut"></span>`fn find_subcommand_mut(&mut self, name: impl AsRef<std::ffi::OsStr>) -> Option<&mut Command>` — [`Command`](#command)

- <span id="command-get-groups"></span>`fn get_groups(&self) -> impl Iterator<Item = &ArgGroup>` — [`ArgGroup`](#arggroup)

- <span id="command-get-arguments"></span>`fn get_arguments(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](#arg)

- <span id="command-get-positionals"></span>`fn get_positionals(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](#arg)

- <span id="command-get-opts"></span>`fn get_opts(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](#arg)

- <span id="command-get-arg-conflicts-with"></span>`fn get_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg>` — [`Arg`](#arg)

- <span id="command-get-global-arg-conflicts-with"></span>`fn get_global_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg>` — [`Arg`](#arg)

- <span id="command-get-subcommands-containing"></span>`fn get_subcommands_containing(&self, arg: &Arg) -> Vec<&Self>` — [`Arg`](#arg)

- <span id="command-is-no-binary-name-set"></span>`fn is_no_binary_name_set(&self) -> bool`

- <span id="command-is-ignore-errors-set"></span>`fn is_ignore_errors_set(&self) -> bool`

- <span id="command-is-dont-delimit-trailing-values-set"></span>`fn is_dont_delimit_trailing_values_set(&self) -> bool`

- <span id="command-is-disable-version-flag-set"></span>`fn is_disable_version_flag_set(&self) -> bool`

- <span id="command-is-propagate-version-set"></span>`fn is_propagate_version_set(&self) -> bool`

- <span id="command-is-next-line-help-set"></span>`fn is_next_line_help_set(&self) -> bool`

- <span id="command-is-disable-help-flag-set"></span>`fn is_disable_help_flag_set(&self) -> bool`

- <span id="command-is-disable-help-subcommand-set"></span>`fn is_disable_help_subcommand_set(&self) -> bool`

- <span id="command-is-disable-colored-help-set"></span>`fn is_disable_colored_help_set(&self) -> bool`

- <span id="command-is-help-expected-set"></span>`fn is_help_expected_set(&self) -> bool`

- <span id="command-is-infer-long-args-set"></span>`fn is_infer_long_args_set(&self) -> bool`

- <span id="command-is-infer-subcommands-set"></span>`fn is_infer_subcommands_set(&self) -> bool`

- <span id="command-is-arg-required-else-help-set"></span>`fn is_arg_required_else_help_set(&self) -> bool`

- <span id="command-is-allow-missing-positional-set"></span>`fn is_allow_missing_positional_set(&self) -> bool`

- <span id="command-is-hide-set"></span>`fn is_hide_set(&self) -> bool`

- <span id="command-is-subcommand-required-set"></span>`fn is_subcommand_required_set(&self) -> bool`

- <span id="command-is-allow-external-subcommands-set"></span>`fn is_allow_external_subcommands_set(&self) -> bool`

- <span id="command-get-external-subcommand-value-parser"></span>`fn get_external_subcommand_value_parser(&self) -> Option<&super::ValueParser>` — [`ValueParser`](builder/index.md)

- <span id="command-is-args-conflicts-with-subcommands-set"></span>`fn is_args_conflicts_with_subcommands_set(&self) -> bool`

- <span id="command-is-subcommand-precedence-over-arg-set"></span>`fn is_subcommand_precedence_over_arg_set(&self) -> bool`

- <span id="command-is-subcommand-negates-reqs-set"></span>`fn is_subcommand_negates_reqs_set(&self) -> bool`

- <span id="command-is-multicall-set"></span>`fn is_multicall_set(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Command`

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](#command)

##### `impl Debug for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Command`

- <span id="command-default"></span>`fn default() -> Self`

##### `impl Display for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- <span id="command-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](#id)

##### `impl<T> ToString for Command`

- <span id="command-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="arg-group"></span>`fn group(self, group_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arg-groups"></span>`fn groups(self, group_ids: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arg-default-value-if"></span>`fn default_value_if(self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, default: impl IntoResettable<OsStr>) -> Self` — [`Id`](#id), [`ArgPredicate`](builder/index.md), [`IntoResettable`](builder/index.md), [`OsStr`](builder/index.md)

- <span id="arg-default-values-if"></span>`fn default_values_if(self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, defaults: impl IntoIterator<Item = impl Into<OsStr>>) -> Self` — [`Id`](#id), [`ArgPredicate`](builder/index.md), [`OsStr`](builder/index.md)

- <span id="arg-default-value-ifs"></span>`fn default_value_ifs(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>)>) -> Self` — [`Id`](#id), [`ArgPredicate`](builder/index.md), [`IntoResettable`](builder/index.md), [`OsStr`](builder/index.md)

- <span id="arg-default-values-ifs"></span>`fn default_values_ifs(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoIterator<Item = impl Into<OsStr>>)>) -> Self` — [`Id`](#id), [`ArgPredicate`](builder/index.md), [`OsStr`](builder/index.md)

- <span id="arg-required-unless-present"></span>`fn required_unless_present(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arg-required-unless-present-all"></span>`fn required_unless_present_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arg-required-unless-present-any"></span>`fn required_unless_present_any(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arg-required-if-eq"></span>`fn required_if_eq(self, arg_id: impl Into<Id>, val: impl Into<OsStr>) -> Self` — [`Id`](#id), [`OsStr`](builder/index.md)

- <span id="arg-required-if-eq-any"></span>`fn required_if_eq_any(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self` — [`Id`](#id), [`OsStr`](builder/index.md)

- <span id="arg-required-if-eq-all"></span>`fn required_if_eq_all(self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self` — [`Id`](#id), [`OsStr`](builder/index.md)

- <span id="arg-requires-if"></span>`fn requires_if(self, val: impl Into<ArgPredicate>, arg_id: impl Into<Id>) -> Self` — [`ArgPredicate`](builder/index.md), [`Id`](#id)

- <span id="arg-requires-ifs"></span>`fn requires_ifs(self, ifs: impl IntoIterator<Item = (impl Into<ArgPredicate>, impl Into<Id>)>) -> Self` — [`ArgPredicate`](builder/index.md), [`Id`](#id)

- <span id="arg-conflicts-with"></span>`fn conflicts_with(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arg-conflicts-with-all"></span>`fn conflicts_with_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arg-overrides-with"></span>`fn overrides_with(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arg-overrides-with-all"></span>`fn overrides_with_all(self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

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

- <span id="arggroup-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](#id)

- <span id="arggroup-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](#id)

- <span id="arggroup-arg"></span>`fn arg(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arggroup-args"></span>`fn args(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arggroup-get-args"></span>`fn get_args(&self) -> impl Iterator<Item = &Id>` — [`Id`](#id)

- <span id="arggroup-multiple"></span>`fn multiple(self, yes: bool) -> Self`

- <span id="arggroup-is-multiple"></span>`fn is_multiple(&mut self) -> bool`

- <span id="arggroup-required"></span>`fn required(self, yes: bool) -> Self`

- <span id="arggroup-requires"></span>`fn requires(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arggroup-requires-all"></span>`fn requires_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

- <span id="arggroup-conflicts-with"></span>`fn conflicts_with(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/index.md), [`Id`](#id)

- <span id="arggroup-conflicts-with-all"></span>`fn conflicts_with_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](#id)

#### Trait Implementations

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl Debug for ArgGroup`

- <span id="arggroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](#arggroup)

##### `impl StructuralPartialEq for ArgGroup`

### `ArgMatches`

```rust
struct ArgMatches {
    valid_args: Vec<crate::util::Id>,
    valid_subcommands: Vec<crate::builder::Str>,
    args: self::flat_map::FlatMap<crate::util::Id, matched_arg::MatchedArg>,
    subcommand: Option<Box<SubCommand>>,
}
```

Container for parse results.

Used to get information about the arguments that were supplied to the program at runtime by
the user. New instances of this struct are obtained by using the `Command::get_matches` family of
methods.

# Examples

```no_run
use clap_builder as clap;
use clap::{Command, Arg, ArgAction};
use clap::parser::ValueSource;
let matches = Command::new("MyApp")
    .arg(Arg::new("out")
        .long("output")
        .required(true)
        .action(ArgAction::Set)
        .default_value("-"))
    .arg(Arg::new("cfg")
        .short('c')
        .action(ArgAction::Set))
    .get_matches(); // builds the instance of ArgMatches

// to get information about the "cfg" argument we created, such as the value supplied we use
// various ArgMatches methods, such as [ArgMatches::get_one]
if let Some(c) = matches.get_one::<String>("cfg") {
    println!("Value for -c: {c}");
}

// The ArgMatches::get_one method returns an Option because the user may not have supplied
// that argument at runtime. But if we specified that the argument was "required" as we did
// with the "out" argument, we can safely unwrap because `clap` verifies that was actually
// used at runtime.
println!("Value for --output: {}", matches.get_one::<String>("out").unwrap());

// You can check the presence of an argument's values
if matches.contains_id("out") {
    // However, if you want to know where the value came from
    if matches.value_source("out").expect("checked contains_id") == ValueSource::CommandLine {
        println!("`out` set by user");
    } else {
        println!("`out` is defaulted");
    }
}
```


#### Implementations

- <span id="argmatches-try-get-one"></span>`fn try_get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Result<Option<&T>, MatchesError>` — [`MatchesError`](parser/index.md)

- <span id="argmatches-try-get-many"></span>`fn try_get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Result<Option<ValuesRef<'_, T>>, MatchesError>` — [`ValuesRef`](parser/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-get-occurrences"></span>`fn try_get_occurrences<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Result<Option<OccurrencesRef<'_, T>>, MatchesError>` — [`OccurrencesRef`](parser/matches/arg_matches/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-get-raw"></span>`fn try_get_raw(&self, id: &str) -> Result<Option<RawValues<'_>>, MatchesError>` — [`RawValues`](parser/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-get-raw-occurrences"></span>`fn try_get_raw_occurrences(&self, id: &str) -> Result<Option<RawOccurrences<'_>>, MatchesError>` — [`RawOccurrences`](parser/matches/arg_matches/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-remove-one"></span>`fn try_remove_one<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Result<Option<T>, MatchesError>` — [`MatchesError`](parser/index.md)

- <span id="argmatches-try-remove-many"></span>`fn try_remove_many<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Result<Option<Values<T>>, MatchesError>` — [`Values`](parser/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-remove-occurrences"></span>`fn try_remove_occurrences<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Result<Option<Occurrences<T>>, MatchesError>` — [`Occurrences`](parser/matches/arg_matches/index.md), [`MatchesError`](parser/index.md)

- <span id="argmatches-try-contains-id"></span>`fn try_contains_id(&self, id: &str) -> Result<bool, MatchesError>` — [`MatchesError`](parser/index.md)

- <span id="argmatches-try-clear-id"></span>`fn try_clear_id(&mut self, id: &str) -> Result<bool, MatchesError>` — [`MatchesError`](parser/index.md)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](#argmatches)

##### `impl Debug for ArgMatches`

- <span id="argmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](#argmatches)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](#argmatches)

##### `impl StructuralPartialEq for ArgMatches`

### `Id`

```rust
struct Id(crate::builder::Str);
```

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- <span id="id-help"></span>`const HELP: &'static str`

- <span id="id-version"></span>`const VERSION: &'static str`

- <span id="id-external"></span>`const EXTERNAL: &'static str`

- <span id="id-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="id-as-str"></span>`fn as_str(&self) -> &str`

- <span id="id-as-internal-str"></span>`fn as_internal_str(&self) -> &Str` — [`Str`](builder/index.md)

#### Trait Implementations

##### `impl AsRef for Id`

- <span id="id-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](#id)

##### `impl Debug for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- <span id="id-default"></span>`fn default() -> Id` — [`Id`](#id)

##### `impl Display for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<I> IntoResettable for Id`

- <span id="id-into-resettable"></span>`fn into_resettable(self) -> Resettable<Str>` — [`Resettable`](builder/index.md), [`Str`](builder/index.md)

##### `impl Ord for Id`

- <span id="id-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](#id)

##### `impl PartialEq for Id`

- <span id="id-eq"></span>`fn eq(&self, other: &&str) -> bool`

##### `impl PartialOrd for Id`

- <span id="id-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](#id)

##### `impl StructuralPartialEq for Id`

##### `impl<T> ToString for Id`

- <span id="id-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="argaction-max-num-args"></span>`fn max_num_args(&self) -> ValueRange` — [`ValueRange`](builder/index.md)

- <span id="argaction-default-num-args"></span>`fn default_num_args(&self) -> ValueRange` — [`ValueRange`](builder/index.md)

- <span id="argaction-default-value"></span>`fn default_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-missing-value"></span>`fn default_missing_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-value-parser"></span>`fn default_value_parser(&self) -> Option<super::ValueParser>` — [`ValueParser`](builder/index.md)

- <span id="argaction-value-type-id"></span>`fn value_type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for ArgAction`

- <span id="argaction-clone"></span>`fn clone(&self) -> ArgAction` — [`ArgAction`](#argaction)

##### `impl Debug for ArgAction`

- <span id="argaction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoResettable for crate::builder::ArgAction`

- <span id="cratebuilderargaction-into-resettable"></span>`fn into_resettable(self) -> Resettable<ArgAction>` — [`Resettable`](builder/index.md), [`ArgAction`](#argaction)

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

- <span id="valuehint-clone"></span>`fn clone(&self) -> ValueHint` — [`ValueHint`](#valuehint)

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- <span id="valuehint-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ValueHint`

- <span id="valuehint-default"></span>`fn default() -> ValueHint` — [`ValueHint`](#valuehint)

##### `impl Eq for ValueHint`

##### `impl FromStr for ValueHint`

- <span id="valuehint-err"></span>`type Err = String`

- <span id="valuehint-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- <span id="valuehint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoResettable for crate::builder::ValueHint`

- <span id="cratebuildervaluehint-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueHint>` — [`Resettable`](builder/index.md), [`ValueHint`](#valuehint)

##### `impl PartialEq for ValueHint`

- <span id="valuehint-eq"></span>`fn eq(&self, other: &ValueHint) -> bool` — [`ValueHint`](#valuehint)

##### `impl StructuralPartialEq for ValueHint`

### `ColorChoice`

```rust
enum ColorChoice {
    Auto,
    Always,
    Never,
}
```

Represents the color preferences for program output

#### Variants

- **`Auto`**

  Enables colored output only when the output is going to a terminal or TTY.
  
  <div class="warning">
  
  **NOTE:** This is the default behavior of `clap`.
  
  </div>
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Auto)
      .get_matches();
  }
  ```

- **`Always`**

  Enables colored output regardless of whether or not the output is going to a terminal/TTY.
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Always)
      .get_matches();
  }
  ```

- **`Never`**

  Disables colored output no matter if the output is going to a terminal/TTY, or not.
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Never)
      .get_matches();
  }
  ```

#### Implementations

- <span id="colorchoice-possible-values"></span>`fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](builder/index.md)

#### Trait Implementations

##### `impl Clone for ColorChoice`

- <span id="colorchoice-clone"></span>`fn clone(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- <span id="colorchoice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ColorChoice`

- <span id="colorchoice-default"></span>`fn default() -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Display for ColorChoice`

- <span id="colorchoice-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl FromStr for ColorChoice`

- <span id="colorchoice-err"></span>`type Err = String`

- <span id="colorchoice-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl<T> ToString for ColorChoice`

- <span id="colorchoice-to-string"></span>`fn to_string(&self) -> String`

##### `impl ValueEnum for ColorChoice`

- <span id="colorchoice-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="colorchoice-to-possible-value"></span>`fn to_possible_value(&self) -> Option<PossibleValue>` — [`PossibleValue`](builder/index.md)

## Traits

## Type Aliases

### `Error`

```rust
type Error = error::Error<error::DefaultFormatter>;
```

Command Line Argument Parser Error

See `Command::error` to create an error.


## Constants

### `INTERNAL_ERROR_MSG`

```rust
const INTERNAL_ERROR_MSG: &str;
```

## Macros

### `command!`

Requires `cargo` feature flag to be enabled.

### `arg!`

Create an [`Arg`](#arg) from a usage string.

Allows creation of basic settings for the [`Arg`](#arg).

<div class="warning">

**NOTE**: Not all settings may be set using the usage string method. Some properties are
only available via the builder pattern.

</div>

# Syntax

Usage strings typically following the form:

```notrust
[explicit name] [short] [long] [value names] [...] [help string]
```

### Explicit Name

The name may be either a bare-word or a string, followed by a `:`, like `name:` or
`"name":`.

*Note:* This is an optional field, if it's omitted the argument will use one of the additional
fields as the name using the following priority order:

 1. Explicit Name
 2. Long
 3. Value Name

See `Arg::id`.

### Short

A short flag is a `-` followed by either a bare-character or quoted character, like `-f` or
`-'f'`.

See `Arg::short`.

### Long

A long flag is a `--` followed by either a bare-word or a string, like `--foo` or
`--"foo"`.

<div class="warning">

**NOTE:** Dashes in the long name (e.g. `--foo-bar`) is not supported and quoting is required
(e.g. `--"foo-bar"`).

</div>

See `Arg::long`.

### Values (Value Notation)

This is set by placing bare-word between:
- `[]` like `[FOO]`
  - Positional argument: optional
  - Named argument: optional value
- `<>` like `<FOO>`: required

See `Arg::value_name`.

### `...`

`...` (three consecutive dots/periods) specifies that this argument may occur multiple
times (not to be confused with multiple values per occurrence).

See `ArgAction::Count` and `ArgAction::Append`.

### Help String

The help string is denoted between a pair of double quotes `""` and may contain any
characters.

# Examples

```rust
use clap_builder as clap;
use clap::{Command, Arg, arg};
let cmd = Command::new("prog")
    .args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input] "an optional input file to use")
    ]);

let m = cmd.try_get_matches_from(["prog", "--config", "file.toml"]).unwrap();
assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
assert_eq!(m.get_one::<String>("input"), None);
```


### `value_parser!`

Select a [`ValueParser`](builder/index.md) implementation from the intended type

Supported types
- [`ValueParserFactory` types][ValueParserFactory], including
  - [Native types][ValueParser]: `bool`, `String`, `OsString`, `PathBuf`
  - [Ranged numeric types][RangedI64ValueParser]: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`
- ``ValueEnum` types`
- ``From<OsString>` types` and ``From<&OsStr>` types`
- ``From<String>` types` and ``From<&str>` types`
- ``FromStr` types`, including usize, isize

# Example

Usage:
```rust
use clap_builder as clap;
use std::path::PathBuf;
use std::path::Path;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("output")
            .value_parser(clap::value_parser!(PathBuf))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "file.txt"]).unwrap();
let port: &PathBuf = m.get_one("output")
    .expect("required");
assert_eq!(port, Path::new("file.txt"));
```

Example mappings:
```rust
use clap_builder as clap;
use clap::ColorChoice;
// Built-in types
let parser = clap::value_parser!(String);
assert_eq!(format!("{parser:?}"), "ValueParser::string");
let parser = clap::value_parser!(std::ffi::OsString);
assert_eq!(format!("{parser:?}"), "ValueParser::os_string");
let parser = clap::value_parser!(std::path::PathBuf);
assert_eq!(format!("{parser:?}"), "ValueParser::path_buf");
clap::value_parser!(u16).range(3000..);
clap::value_parser!(u64).range(3000..);

// FromStr types
let parser = clap::value_parser!(usize);
assert_eq!(format!("{parser:?}"), "_AnonymousValueParser(ValueParser::other(usize))");

// ValueEnum types
clap::value_parser!(ColorChoice);
```

