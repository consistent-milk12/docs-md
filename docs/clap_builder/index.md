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

## Modules

- [`builder`](builder/index.md) - Define [`Command`] line [arguments][`Arg`]
- [`error`](error/index.md) - Error reporting
- [`parser`](parser/index.md) - [`Command`][crate::Command] line argument parser

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

- `fn name(self: Self, name: impl Into<Str>) -> Self` — [`Str`](builder/str/index.md)

- `fn bin_name(self: Self, name: impl IntoResettable<String>) -> Self` — [`IntoResettable`](builder/resettable/index.md)

- `fn display_name(self: Self, name: impl IntoResettable<String>) -> Self` — [`IntoResettable`](builder/resettable/index.md)

- `fn author(self: Self, author: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`Str`](builder/str/index.md)

- `fn about(self: Self, about: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn long_about(self: Self, long_about: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn after_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn after_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn before_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn before_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn version(self: Self, ver: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`Str`](builder/str/index.md)

- `fn long_version(self: Self, ver: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`Str`](builder/str/index.md)

- `fn override_usage(self: Self, usage: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn override_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn help_template(self: Self, s: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`StyledStr`](builder/styled_str/index.md)

- `fn setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](builder/app_settings/index.md)

- `fn unset_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](builder/app_settings/index.md)

- `fn global_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](builder/app_settings/index.md)

- `fn unset_global_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](builder/app_settings/index.md)

- `fn flatten_help(self: Self, yes: bool) -> Self`

- `fn next_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md), [`Str`](builder/str/index.md)

- `fn next_display_order(self: Self, disp_ord: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](builder/resettable/index.md)

- `fn arg_required_else_help(self: Self, yes: bool) -> Self`

- `fn allow_missing_positional(self: Self, yes: bool) -> Self`

#### Trait Implementations

##### `impl Clone for Command`

- `fn clone(self: &Self) -> Command` — [`Command`](builder/command/index.md)

##### `impl Debug for Command`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Command`

- `fn default() -> Self`

##### `impl Display for Command`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output` — [`Id`](util/id/index.md)

##### `impl<T> ToString for Command`

- `fn to_string(self: &Self) -> String`

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

There are two methods for constructing [`Arg`](builder/arg/index.md)s, using the builder pattern and setting options
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

- `fn get_id(self: &Self) -> &Id` — [`Id`](util/id/index.md)

- `fn get_help(self: &Self) -> Option<&StyledStr>` — [`StyledStr`](builder/styled_str/index.md)

- `fn get_long_help(self: &Self) -> Option<&StyledStr>` — [`StyledStr`](builder/styled_str/index.md)

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

- `fn get_possible_values(self: &Self) -> Vec<PossibleValue>` — [`PossibleValue`](builder/possible_value/index.md)

- `fn get_value_names(self: &Self) -> Option<&[Str]>` — [`Str`](builder/str/index.md)

- `fn get_num_args(self: &Self) -> Option<ValueRange>` — [`ValueRange`](builder/range/index.md)

- `fn get_min_vals(self: &Self) -> usize`

- `fn get_value_delimiter(self: &Self) -> Option<char>`

- `fn get_value_terminator(self: &Self) -> Option<&Str>` — [`Str`](builder/str/index.md)

- `fn get_index(self: &Self) -> Option<usize>`

- `fn get_value_hint(self: &Self) -> ValueHint` — [`ValueHint`](builder/value_hint/index.md)

- `fn get_default_values(self: &Self) -> &[OsStr]` — [`OsStr`](builder/os_str/index.md)

- `fn is_positional(self: &Self) -> bool`

- `fn is_required_set(self: &Self) -> bool`

- `fn is_multiple_values_set(self: &Self) -> bool`

- `fn is_takes_value_set(self: &Self) -> bool`

- `fn is_allow_hyphen_values_set(self: &Self) -> bool`

- `fn is_allow_negative_numbers_set(self: &Self) -> bool`

- `fn get_action(self: &Self) -> &ArgAction` — [`ArgAction`](builder/action/index.md)

- `fn get_value_parser(self: &Self) -> &super::ValueParser` — [`ValueParser`](builder/value_parser/index.md)

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

- `fn clone(self: &Self) -> Arg` — [`Arg`](builder/arg/index.md)

##### `impl Debug for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- `fn default() -> Arg` — [`Arg`](builder/arg/index.md)

##### `impl Display for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- `fn cmp(self: &Self, other: &Arg) -> Ordering` — [`Arg`](builder/arg/index.md)

##### `impl PartialEq for Arg`

- `fn eq(self: &Self, other: &Arg) -> bool` — [`Arg`](builder/arg/index.md)

##### `impl PartialOrd for Arg`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl<T> ToString for Arg`

- `fn to_string(self: &Self) -> String`

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

Specifies a logical group of [arguments](#arguments)

You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts](#conflicts) or [requirements](#requirements)
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

- `fn get_id(self: &Self) -> &Id` — [`Id`](util/id/index.md)

- `fn is_required_set(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for ArgGroup`

- `fn clone(self: &Self) -> ArgGroup` — [`ArgGroup`](builder/arg_group/index.md)

##### `impl Debug for ArgGroup`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgGroup`

- `fn default() -> ArgGroup` — [`ArgGroup`](builder/arg_group/index.md)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- `fn eq(self: &Self, other: &ArgGroup) -> bool` — [`ArgGroup`](builder/arg_group/index.md)

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

- `fn try_get_arg(self: &Self, arg: &str) -> Result<Option<&MatchedArg>, MatchesError>` — [`MatchedArg`](parser/matches/matched_arg/index.md), [`MatchesError`](parser/error/index.md)

- `fn try_get_arg_t<T: Any + Send + Sync + 'static>(self: &Self, arg: &str) -> Result<Option<&MatchedArg>, MatchesError>` — [`MatchedArg`](parser/matches/matched_arg/index.md), [`MatchesError`](parser/error/index.md)

- `fn try_remove_arg_t<T: Any + Send + Sync + 'static>(self: &mut Self, arg: &str) -> Result<Option<MatchedArg>, MatchesError>` — [`MatchedArg`](parser/matches/matched_arg/index.md), [`MatchesError`](parser/error/index.md)

- `fn verify_arg_t<T: Any + Send + Sync + 'static>(self: &Self, arg: &MatchedArg) -> Result<(), MatchesError>` — [`MatchedArg`](parser/matches/matched_arg/index.md), [`MatchesError`](parser/error/index.md)

- `fn verify_arg(self: &Self, _arg: &str) -> Result<(), MatchesError>` — [`MatchesError`](parser/error/index.md)

- `fn get_arg<'s>(self: &'s Self, arg: &str) -> Option<&'s MatchedArg>` — [`MatchedArg`](parser/matches/matched_arg/index.md)

- `fn get_subcommand(self: &Self, name: &str) -> Option<&SubCommand>` — [`SubCommand`](parser/matches/arg_matches/index.md)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- `fn clone(self: &Self) -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md)

##### `impl Debug for ArgMatches`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgMatches`

- `fn default() -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- `fn eq(self: &Self, other: &ArgMatches) -> bool` — [`ArgMatches`](parser/matches/arg_matches/index.md)

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

- `const HELP: &'static str`

- `const VERSION: &'static str`

- `const EXTERNAL: &'static str`

- `fn from_static_ref(name: &'static str) -> Self`

- `fn as_str(self: &Self) -> &str`

- `fn as_internal_str(self: &Self) -> &Str` — [`Str`](builder/str/index.md)

#### Trait Implementations

##### `impl AsRef for Id`

- `fn as_ref(self: &Self) -> &str`

##### `impl Clone for Id`

- `fn clone(self: &Self) -> Id` — [`Id`](util/id/index.md)

##### `impl Debug for Id`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- `fn default() -> Id` — [`Id`](util/id/index.md)

##### `impl Display for Id`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl Hash for Id`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for Id`

- `fn into_resettable(self: Self) -> Resettable<String>` — [`Resettable`](builder/resettable/index.md)

##### `impl Ord for Id`

- `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering` — [`Id`](util/id/index.md)

##### `impl PartialEq for Id`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd for Id`

- `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Id`](util/id/index.md)

##### `impl StructuralPartialEq for Id`

##### `impl<T> ToString for Id`

- `fn to_string(self: &Self) -> String`

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

- `fn takes_values(self: &Self) -> bool`

- `fn max_num_args(self: &Self) -> ValueRange` — [`ValueRange`](builder/range/index.md)

- `fn default_num_args(self: &Self) -> ValueRange` — [`ValueRange`](builder/range/index.md)

- `fn default_value(self: &Self) -> Option<&'static std::ffi::OsStr>`

- `fn default_missing_value(self: &Self) -> Option<&'static std::ffi::OsStr>`

- `fn default_value_parser(self: &Self) -> Option<super::ValueParser>` — [`ValueParser`](builder/value_parser/index.md)

- `fn value_type_id(self: &Self) -> Option<AnyValueId>` — [`AnyValueId`](util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for ArgAction`

- `fn clone(self: &Self) -> ArgAction` — [`ArgAction`](builder/action/index.md)

##### `impl Debug for ArgAction`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoResettable for crate::builder::ArgAction`

- `fn into_resettable(self: Self) -> Resettable<ArgAction>` — [`Resettable`](builder/resettable/index.md), [`ArgAction`](builder/action/index.md)

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
  [`.num_args(1..)`](#num-args1) and Command must use `Command::trailing_var_arg(true)`. The result is that the
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

- `fn clone(self: &Self) -> ValueHint` — [`ValueHint`](builder/value_hint/index.md)

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ValueHint`

- `fn default() -> ValueHint` — [`ValueHint`](builder/value_hint/index.md)

##### `impl Eq for ValueHint`

##### `impl FromStr for ValueHint`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoResettable for crate::builder::ValueHint`

- `fn into_resettable(self: Self) -> Resettable<ValueHint>` — [`Resettable`](builder/resettable/index.md), [`ValueHint`](builder/value_hint/index.md)

##### `impl PartialEq for ValueHint`

- `fn eq(self: &Self, other: &ValueHint) -> bool` — [`ValueHint`](builder/value_hint/index.md)

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

- `fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](builder/possible_value/index.md)

#### Trait Implementations

##### `impl Clone for ColorChoice`

- `fn clone(self: &Self) -> ColorChoice` — [`ColorChoice`](util/color/index.md)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ColorChoice`

- `fn default() -> ColorChoice` — [`ColorChoice`](util/color/index.md)

##### `impl Display for ColorChoice`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl FromStr for ColorChoice`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq for ColorChoice`

- `fn eq(self: &Self, other: &ColorChoice) -> bool` — [`ColorChoice`](util/color/index.md)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl<T> ToString for ColorChoice`

- `fn to_string(self: &Self) -> String`

##### `impl ValueEnum for ColorChoice`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value(self: &Self) -> Option<PossibleValue>` — [`PossibleValue`](builder/possible_value/index.md)

## Traits

## Type Aliases

### `Error`

```rust
type Error = error::Error<error::DefaultFormatter>;
```

Command Line Argument Parser Error

See `Command::error` to create an error.


## Macros

### `command!`

Requires `cargo` feature flag to be enabled.

### `arg!`

Create an [`Arg`](builder/arg/index.md) from a usage string.

Allows creation of basic settings for the [`Arg`](builder/arg/index.md).

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

Select a [`ValueParser`](builder/value_parser/index.md) implementation from the intended type

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

