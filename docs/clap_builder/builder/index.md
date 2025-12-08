*[clap_builder](../index.md) / [builder](index.md)*

---

# Module `builder`

Define [`Command`](command/index.md) line [arguments][`Arg`](arg/index.md)

## Modules

- [`action`](action/index.md) - 
- [`app_settings`](app_settings/index.md) - 
- [`arg`](arg/index.md) - 
- [`arg_group`](arg_group/index.md) - 
- [`arg_predicate`](arg_predicate/index.md) - 
- [`arg_settings`](arg_settings/index.md) - 
- [`command`](command/index.md) - 
- [`ext`](ext/index.md) - 
- [`os_str`](os_str/index.md) - 
- [`possible_value`](possible_value/index.md) - 
- [`range`](range/index.md) - 
- [`resettable`](resettable/index.md) - 
- [`str`](str/index.md) - 
- [`styled_str`](styled_str/index.md) - 
- [`value_hint`](value_hint/index.md) - 
- [`value_parser`](value_parser/index.md) - 
- [`debug_asserts`](debug_asserts/index.md) - 
- [`styling`](styling/index.md) - Terminal [`Styles`] for help and error output

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

- `fn from_static_ref(name: &'static str) -> Self`

- `fn into_inner(self: Self) -> Inner` — [`Inner`](str/inner/index.md)

- `fn as_str(self: &Self) -> &str`

#### Trait Implementations

##### `impl AsRef for Str`

- `fn as_ref(self: &Self) -> &std::path::Path`

##### `impl Clone for Str`

- `fn clone(self: &Self) -> Str` — [`Str`](str/index.md)

##### `impl Debug for Str`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- `fn default() -> Str` — [`Str`](str/index.md)

##### `impl Deref for Str`

- `type Target = str`

- `fn deref(self: &Self) -> &str`

##### `impl Display for Str`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl Hash for Str`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for Str`

- `fn into_resettable(self: Self) -> Resettable<Id>` — [`Resettable`](resettable/index.md), [`Id`](../util/id/index.md)

##### `impl Ord for Str`

- `fn cmp(self: &Self, other: &Str) -> $crate::cmp::Ordering` — [`Str`](str/index.md)

##### `impl PartialEq for Str`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd for Str`

- `fn partial_cmp(self: &Self, other: &Str) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Str`](str/index.md)

##### `impl<P, T> Receiver for Str`

- `type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl<T> ToString for Str`

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

There are two methods for constructing [`Arg`](arg/index.md)s, using the builder pattern and setting options
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

- `fn _build(self: &mut Self)`

- `fn name_no_brackets(self: &Self) -> String`

- `fn stylized(self: &Self, styles: &Styles, required: Option<bool>) -> StyledStr` — [`Styles`](styling/index.md), [`StyledStr`](styled_str/index.md)

- `fn stylize_arg_suffix(self: &Self, styles: &Styles, required: Option<bool>) -> StyledStr` — [`Styles`](styling/index.md), [`StyledStr`](styled_str/index.md)

- `fn render_arg_val(self: &Self, required: bool) -> String`

- `fn is_multiple(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Arg`

- `fn clone(self: &Self) -> Arg` — [`Arg`](arg/index.md)

##### `impl Debug for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- `fn default() -> Arg` — [`Arg`](arg/index.md)

##### `impl Display for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- `fn cmp(self: &Self, other: &Arg) -> Ordering` — [`Arg`](arg/index.md)

##### `impl PartialEq for Arg`

- `fn eq(self: &Self, other: &Arg) -> bool` — [`Arg`](arg/index.md)

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

- `fn get_id(self: &Self) -> &Id` — [`Id`](../util/id/index.md)

- `fn is_required_set(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for ArgGroup`

- `fn clone(self: &Self) -> ArgGroup` — [`ArgGroup`](arg_group/index.md)

##### `impl Debug for ArgGroup`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgGroup`

- `fn default() -> ArgGroup` — [`ArgGroup`](arg_group/index.md)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- `fn eq(self: &Self, other: &ArgGroup) -> bool` — [`ArgGroup`](arg_group/index.md)

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

- `fn new(name: impl Into<Str>) -> Self` — [`Str`](str/index.md)

- `fn arg(self: Self, a: impl Into<Arg>) -> Self` — [`Arg`](arg/index.md)

- `fn arg_internal(self: &mut Self, arg: Arg)` — [`Arg`](arg/index.md)

- `fn args(self: Self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](arg/index.md)

- `fn mut_arg<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`

- `fn mut_args<F>(self: Self, f: F) -> Self`

- `fn mut_group<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`

- `fn mut_subcommand<F>(self: Self, name: impl AsRef<str>, f: F) -> Self`

- `fn mut_subcommands<F>(self: Self, f: F) -> Self`

- `fn group(self: Self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](arg_group/index.md)

- `fn groups(self: Self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](arg_group/index.md)

- `fn subcommand(self: Self, subcmd: impl Into<Command>) -> Self` — [`Command`](command/index.md)

- `fn subcommand_internal(self: Self, subcmd: Self) -> Self`

- `fn subcommands(self: Self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`

- `fn defer(self: Self, deferred: fn(Command) -> Command) -> Self` — [`Command`](command/index.md)

- `fn debug_assert(self: Self)`

- `fn error(self: &mut Self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](../error/kind/index.md), [`Error`](../index.md)

- `fn get_matches(self: Self) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn get_matches_mut(self: &mut Self) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn try_get_matches(self: Self) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md), [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn get_matches_from<I, T>(self: Self, itr: I) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn try_get_matches_from<I, T>(self: Self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md), [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn try_get_matches_from_mut<I, T>(self: &mut Self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md), [`ArgMatches`](../parser/matches/arg_matches/index.md)

- `fn print_help(self: &mut Self) -> io::Result<()>`

- `fn print_long_help(self: &mut Self) -> io::Result<()>`

- `fn render_help(self: &mut Self) -> StyledStr` — [`StyledStr`](styled_str/index.md)

- `fn render_long_help(self: &mut Self) -> StyledStr` — [`StyledStr`](styled_str/index.md)

- `fn render_version(self: &Self) -> String`

- `fn render_long_version(self: &Self) -> String`

- `fn render_usage(self: &mut Self) -> StyledStr` — [`StyledStr`](styled_str/index.md)

- `fn render_usage_(self: &mut Self) -> Option<StyledStr>` — [`StyledStr`](styled_str/index.md)

#### Trait Implementations

##### `impl Clone for Command`

- `fn clone(self: &Self) -> Command` — [`Command`](command/index.md)

##### `impl Debug for Command`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Command`

- `fn default() -> Self`

##### `impl Display for Command`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output` — [`Id`](../util/id/index.md)

##### `impl<T> ToString for Command`

- `fn to_string(self: &Self) -> String`

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

- `fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- `fn as_os_str(self: &Self) -> &std::ffi::OsStr`

- `fn to_os_string(self: &Self) -> std::ffi::OsString`

#### Trait Implementations

##### `impl AsRef for OsStr`

- `fn as_ref(self: &Self) -> &std::ffi::OsStr`

##### `impl Clone for OsStr`

- `fn clone(self: &Self) -> OsStr` — [`OsStr`](os_str/index.md)

##### `impl Debug for OsStr`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- `fn default() -> OsStr` — [`OsStr`](os_str/index.md)

##### `impl Deref for OsStr`

- `type Target = OsStr`

- `fn deref(self: &Self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl Hash for OsStr`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for OsStr`

- `fn into_resettable(self: Self) -> Resettable<OsStr>` — [`Resettable`](resettable/index.md), [`OsStr`](os_str/index.md)

##### `impl Ord for OsStr`

- `fn cmp(self: &Self, other: &OsStr) -> $crate::cmp::Ordering` — [`OsStr`](os_str/index.md)

##### `impl PartialEq for OsStr`

- `fn eq(self: &Self, other: &OsStr) -> bool` — [`OsStr`](os_str/index.md)

##### `impl PartialOrd for OsStr`

- `fn partial_cmp(self: &Self, other: &OsStr) -> $crate::option::Option<$crate::cmp::Ordering>` — [`OsStr`](os_str/index.md)

##### `impl<P, T> Receiver for OsStr`

- `type Target = T`

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

- `fn new(name: impl Into<Str>) -> Self` — [`Str`](str/index.md)

- `fn help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](resettable/index.md), [`StyledStr`](styled_str/index.md)

- `fn hide(self: Self, yes: bool) -> Self`

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](resettable/index.md), [`Str`](str/index.md)

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](str/index.md)

#### Trait Implementations

##### `impl Clone for PossibleValue`

- `fn clone(self: &Self) -> PossibleValue` — [`PossibleValue`](possible_value/index.md)

##### `impl Debug for PossibleValue`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PossibleValue`

- `fn default() -> PossibleValue` — [`PossibleValue`](possible_value/index.md)

##### `impl Eq for PossibleValue`

##### `impl PartialEq for PossibleValue`

- `fn eq(self: &Self, other: &PossibleValue) -> bool` — [`PossibleValue`](possible_value/index.md)

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

- `const EMPTY: Self`

- `const SINGLE: Self`

- `const OPTIONAL: Self`

- `const FULL: Self`

- `fn new(range: impl Into<Self>) -> Self`

- `fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- `fn min_values(self: &Self) -> usize`

- `fn max_values(self: &Self) -> usize`

- `fn takes_values(self: &Self) -> bool`

- `fn is_unbounded(self: &Self) -> bool`

- `fn is_fixed(self: &Self) -> bool`

- `fn is_multiple(self: &Self) -> bool`

- `fn num_values(self: &Self) -> Option<usize>`

- `fn accepts_more(self: &Self, current: usize) -> bool`

#### Trait Implementations

##### `impl Clone for ValueRange`

- `fn clone(self: &Self) -> ValueRange` — [`ValueRange`](range/index.md)

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- `fn default() -> Self`

##### `impl Display for ValueRange`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl Hash for ValueRange`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for ValueRange`

- `fn into_resettable(self: Self) -> Resettable<ValueRange>` — [`Resettable`](resettable/index.md), [`ValueRange`](range/index.md)

##### `impl PartialEq for ValueRange`

- `fn eq(self: &Self, other: &ValueRange) -> bool` — [`ValueRange`](range/index.md)

##### `impl RangeBounds for ValueRange`

- `fn start_bound(self: &Self) -> std::ops::Bound<&usize>`

- `fn end_bound(self: &Self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl<T> ToString for ValueRange`

- `fn to_string(self: &Self) -> String`

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

- `const fn new() -> Self`

- `fn ansi(self: &Self) -> impl std::fmt::Display + '_`

- `fn push_string(self: &mut Self, msg: String)`

- `fn push_str(self: &mut Self, msg: &str)`

- `fn trim_start_lines(self: &mut Self)`

- `fn trim_end(self: &mut Self)`

- `fn replace_newline_var(self: &mut Self)`

- `fn indent(self: &mut Self, initial: &str, trailing: &str)`

- `fn wrap(self: &mut Self, _hard_width: usize)`

- `fn display_width(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn as_styled_str(self: &Self) -> &str`

- `fn iter_text(self: &Self) -> impl Iterator<Item = &str>`

- `fn push_styled(self: &mut Self, other: &Self)`

- `fn write_to(self: &Self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for StyledStr`

- `fn clone(self: &Self) -> StyledStr` — [`StyledStr`](styled_str/index.md)

##### `impl Debug for StyledStr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StyledStr`

- `fn default() -> StyledStr` — [`StyledStr`](styled_str/index.md)

##### `impl Display for StyledStr`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<I> IntoResettable for StyledStr`

- `fn into_resettable(self: Self) -> Resettable<StyledStr>` — [`Resettable`](resettable/index.md), [`StyledStr`](styled_str/index.md)

##### `impl Ord for StyledStr`

- `fn cmp(self: &Self, other: &StyledStr) -> $crate::cmp::Ordering` — [`StyledStr`](styled_str/index.md)

##### `impl PartialEq for StyledStr`

- `fn eq(self: &Self, other: &StyledStr) -> bool` — [`StyledStr`](styled_str/index.md)

##### `impl PartialOrd for StyledStr`

- `fn partial_cmp(self: &Self, other: &StyledStr) -> $crate::option::Option<$crate::cmp::Ordering>` — [`StyledStr`](styled_str/index.md)

##### `impl StructuralPartialEq for StyledStr`

##### `impl<T> ToString for StyledStr`

- `fn to_string(self: &Self) -> String`

##### `impl Write for StyledStr`

- `fn write_str(self: &mut Self, s: &str) -> Result<(), std::fmt::Error>`

- `fn write_char(self: &mut Self, c: char) -> Result<(), std::fmt::Error>`

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

- `const fn plain() -> Self`

- `const fn styled() -> Self`

- `const fn header(self: Self, style: Style) -> Self`

- `const fn error(self: Self, style: Style) -> Self`

- `const fn usage(self: Self, style: Style) -> Self`

- `const fn literal(self: Self, style: Style) -> Self`

- `const fn placeholder(self: Self, style: Style) -> Self`

- `const fn valid(self: Self, style: Style) -> Self`

- `const fn invalid(self: Self, style: Style) -> Self`

- `const fn context(self: Self, style: Style) -> Self`

- `const fn context_value(self: Self, style: Style) -> Self`

#### Trait Implementations

##### `impl AppExt for Styles`

##### `impl Clone for Styles`

- `fn clone(self: &Self) -> Styles` — [`Styles`](styling/index.md)

##### `impl Debug for Styles`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Styles`

- `fn default() -> Self`

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolValueParser`

- `fn clone(self: &Self) -> BoolValueParser` — [`BoolValueParser`](value_parser/index.md)

##### `impl Copy for BoolValueParser`

##### `impl Debug for BoolValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BoolValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for BoolValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for BoolValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

Parse bool-like string values

See also:
- `ValueParser::bool` for different human readable bool representations
- [`FalseyValueParser`](value_parser/index.md) for assuming non-false is true

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

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolishValueParser`

- `fn clone(self: &Self) -> BoolishValueParser` — [`BoolishValueParser`](value_parser/index.md)

##### `impl Copy for BoolishValueParser`

##### `impl Debug for BoolishValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BoolishValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for BoolishValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for BoolishValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(std::marker::PhantomData<E>);
```

Parse an `ValueEnum` value.

See also:
- [`PossibleValuesParser`](value_parser/index.md)

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

- `fn new() -> Self`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static> Clone for EnumValueParser<E>`

- `fn clone(self: &Self) -> EnumValueParser<E>` — [`EnumValueParser`](value_parser/index.md)

##### `impl<E: $crate::fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static> Debug for EnumValueParser<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> Default for EnumValueParser<E>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for EnumValueParser<E>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> TypedValueParser for EnumValueParser<E>`

- `type Value = E`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

Parse false-like string values, everything else is `true`

See also:
- `ValueParser::bool` for assuming non-false is true
- [`BoolishValueParser`](value_parser/index.md) for different human readable bool representations

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

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md)

#### Trait Implementations

##### `impl Clone for FalseyValueParser`

- `fn clone(self: &Self) -> FalseyValueParser` — [`FalseyValueParser`](value_parser/index.md)

##### `impl Copy for FalseyValueParser`

##### `impl Debug for FalseyValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for FalseyValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for FalseyValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for FalseyValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

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

- `fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapValueParser<P, F>`

- `fn clone(self: &Self) -> MapValueParser<P, F>` — [`MapValueParser`](value_parser/index.md)

##### `impl<P: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for MapValueParser<P, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for MapValueParser<P, F>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl<P, F, T> TypedValueParser for MapValueParser<P, F>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

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

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NonEmptyStringValueParser`

- `fn clone(self: &Self) -> NonEmptyStringValueParser` — [`NonEmptyStringValueParser`](value_parser/index.md)

##### `impl Copy for NonEmptyStringValueParser`

##### `impl Debug for NonEmptyStringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NonEmptyStringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for NonEmptyStringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for NonEmptyStringValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for OsStringValueParser`

- `fn clone(self: &Self) -> OsStringValueParser` — [`OsStringValueParser`](value_parser/index.md)

##### `impl Copy for OsStringValueParser`

##### `impl Debug for OsStringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for OsStringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for OsStringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for OsStringValueParser`

- `type Value = OsString`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse(self: &Self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for PathBufValueParser`

- `fn clone(self: &Self) -> PathBufValueParser` — [`PathBufValueParser`](value_parser/index.md)

##### `impl Copy for PathBufValueParser`

##### `impl Debug for PathBufValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PathBufValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for PathBufValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for PathBufValueParser`

- `type Value = PathBuf`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

### `PossibleValuesParser`

```rust
struct PossibleValuesParser(Vec<super::PossibleValue>);
```

Verify the value is from an enumerated set of `PossibleValue`.

See also:
- [`EnumValueParser`](value_parser/index.md) for directly supporting `ValueEnum` types
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

- `fn new(values: impl Into<PossibleValuesParser>) -> Self` — [`PossibleValuesParser`](value_parser/index.md)

#### Trait Implementations

##### `impl Clone for PossibleValuesParser`

- `fn clone(self: &Self) -> PossibleValuesParser` — [`PossibleValuesParser`](value_parser/index.md)

##### `impl Debug for PossibleValuesParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for PossibleValuesParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for PossibleValuesParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

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

- `fn new() -> Self`

- `fn range<B: RangeBounds<i64>>(self: Self, range: B) -> Self`

- `fn format_bounds(self: &Self) -> String`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + TryFrom<i64> + Clone + Send + Sync> Clone for RangedI64ValueParser<T>`

- `fn clone(self: &Self) -> RangedI64ValueParser<T>` — [`RangedI64ValueParser`](value_parser/index.md)

##### `impl<T: $crate::marker::Copy + TryFrom<i64> + Clone + Send + Sync> Copy for RangedI64ValueParser<T>`

##### `impl<T: $crate::fmt::Debug + TryFrom<i64> + Clone + Send + Sync> Debug for RangedI64ValueParser<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync> Default for RangedI64ValueParser<T>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for RangedI64ValueParser<T>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl<T: TryFrom<i64> + Clone + Send + Sync + 'static> TypedValueParser for RangedI64ValueParser<T>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

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

- `fn new() -> Self`

- `fn range<B: RangeBounds<u64>>(self: Self, range: B) -> Self`

- `fn format_bounds(self: &Self) -> String`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + TryFrom<u64>> Clone for RangedU64ValueParser<T>`

- `fn clone(self: &Self) -> RangedU64ValueParser<T>` — [`RangedU64ValueParser`](value_parser/index.md)

##### `impl<T: $crate::marker::Copy + TryFrom<u64>> Copy for RangedU64ValueParser<T>`

##### `impl<T: $crate::fmt::Debug + TryFrom<u64>> Debug for RangedU64ValueParser<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: TryFrom<u64>> Default for RangedU64ValueParser<T>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for RangedU64ValueParser<T>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl<T: TryFrom<u64> + Clone + Send + Sync + 'static> TypedValueParser for RangedU64ValueParser<T>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

### `StringValueParser`

```rust
struct StringValueParser {
}
```

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for StringValueParser`

- `fn clone(self: &Self) -> StringValueParser` — [`StringValueParser`](value_parser/index.md)

##### `impl Copy for StringValueParser`

##### `impl Debug for StringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for StringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for StringValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

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

- `fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: $crate::clone::Clone, F: $crate::clone::Clone> Clone for TryMapValueParser<P, F>`

- `fn clone(self: &Self) -> TryMapValueParser<P, F>` — [`TryMapValueParser`](value_parser/index.md)

##### `impl<P: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for TryMapValueParser<P, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for TryMapValueParser<P, F>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl<P, F, T, E> TypedValueParser for TryMapValueParser<P, F>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

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

- `fn suggest_arg(arg: impl Into<Str>) -> Self` — [`Str`](str/index.md)

- `fn suggest(text: impl Into<StyledStr>) -> Self` — [`StyledStr`](styled_str/index.md)

- `fn and_suggest(self: Self, text: impl Into<StyledStr>) -> Self` — [`StyledStr`](styled_str/index.md)

#### Trait Implementations

##### `impl Clone for UnknownArgumentValueParser`

- `fn clone(self: &Self) -> UnknownArgumentValueParser` — [`UnknownArgumentValueParser`](value_parser/index.md)

##### `impl Debug for UnknownArgumentValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for UnknownArgumentValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

##### `impl TypedValueParser for UnknownArgumentValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

- `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`ValueSource`](../parser/matches/value_source/index.md), [`TypedValueParser`](value_parser/index.md), [`Error`](../index.md)

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
- `ValueParser::new` for additional [`TypedValueParser`](value_parser/index.md) that can be used

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

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, source: ValueSource) -> Result<AnyValue, crate::Error>` — [`Command`](command/index.md), [`Arg`](arg/index.md), [`ValueSource`](../parser/matches/value_source/index.md), [`AnyValue`](../util/any_value/index.md), [`Error`](../index.md)

- `fn type_id(self: &Self) -> AnyValueId` — [`AnyValueId`](../util/any_value/index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md)

- `fn any_value_parser(self: &Self) -> &dyn AnyValueParser` — [`AnyValueParser`](value_parser/index.md)

#### Trait Implementations

##### `impl Clone for ValueParser`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for ValueParser`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl<I> IntoResettable for ValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md), [`ValueParser`](value_parser/index.md)

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

- `fn max_num_args(self: &Self) -> ValueRange` — [`ValueRange`](range/index.md)

- `fn default_num_args(self: &Self) -> ValueRange` — [`ValueRange`](range/index.md)

- `fn default_value(self: &Self) -> Option<&'static std::ffi::OsStr>`

- `fn default_missing_value(self: &Self) -> Option<&'static std::ffi::OsStr>`

- `fn default_value_parser(self: &Self) -> Option<super::ValueParser>` — [`ValueParser`](value_parser/index.md)

- `fn value_type_id(self: &Self) -> Option<AnyValueId>` — [`AnyValueId`](../util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for ArgAction`

- `fn clone(self: &Self) -> ArgAction` — [`ArgAction`](action/index.md)

##### `impl Debug for ArgAction`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoResettable for crate::builder::ArgAction`

- `fn into_resettable(self: Self) -> Resettable<ArgAction>` — [`Resettable`](resettable/index.md), [`ArgAction`](action/index.md)

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

- `fn clone(self: &Self) -> ArgPredicate` — [`ArgPredicate`](arg_predicate/index.md)

##### `impl Debug for ArgPredicate`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl PartialEq for ArgPredicate`

- `fn eq(self: &Self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](arg_predicate/index.md)

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

- `fn into_option(self: Self) -> Option<T>`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Resettable<T>`

- `fn clone(self: &Self) -> Resettable<T>` — [`Resettable`](resettable/index.md)

##### `impl<T: $crate::marker::Copy> Copy for Resettable<T>`

##### `impl<T: $crate::fmt::Debug> Debug for Resettable<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for Resettable<T>`

##### `impl<T: $crate::hash::Hash> Hash for Resettable<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T> IntoResettable for Resettable<T>`

- `fn into_resettable(self: Self) -> Resettable<T>` — [`Resettable`](resettable/index.md)

##### `impl<T: $crate::cmp::Ord> Ord for Resettable<T>`

- `fn cmp(self: &Self, other: &Resettable<T>) -> $crate::cmp::Ordering` — [`Resettable`](resettable/index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for Resettable<T>`

- `fn eq(self: &Self, other: &Resettable<T>) -> bool` — [`Resettable`](resettable/index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for Resettable<T>`

- `fn partial_cmp(self: &Self, other: &Resettable<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Resettable`](resettable/index.md)

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

- `fn clone(self: &Self) -> ValueHint` — [`ValueHint`](value_hint/index.md)

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ValueHint`

- `fn default() -> ValueHint` — [`ValueHint`](value_hint/index.md)

##### `impl Eq for ValueHint`

##### `impl FromStr for ValueHint`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoResettable for crate::builder::ValueHint`

- `fn into_resettable(self: Self) -> Resettable<ValueHint>` — [`Resettable`](resettable/index.md), [`ValueHint`](value_hint/index.md)

##### `impl PartialEq for ValueHint`

- `fn eq(self: &Self, other: &ValueHint) -> bool` — [`ValueHint`](value_hint/index.md)

##### `impl StructuralPartialEq for ValueHint`

## Traits

