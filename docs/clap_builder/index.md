# Crate `clap_builder`

# `clap_builder`

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
    // [REDACTED: Private Fields]
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
# use clap_builder as clap;
# use clap::{Command, Arg};
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

- `fn short_flag(self: Self, short: impl IntoResettable<char>) -> Self`
  Sets the short version of the subcommand flag without the preceding `-`.

- `fn long_flag(self: Self, long: impl Into<Str>) -> Self`
  Sets the long version of the subcommand flag without the preceding `--`.

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Sets a hidden alias to this subcommand.

- `fn short_flag_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as  "hidden" short flag subcommand

- `fn long_flag_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a "hidden" long flag subcommand.

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Sets multiple hidden aliases to this subcommand.

- `fn short_flag_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as "hidden" short flag subcommands.

- `fn long_flag_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as "hidden" long flag subcommands.

- `fn visible_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Sets a visible alias to this subcommand.

- `fn visible_short_flag_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as  "visible" short flag subcommand

- `fn visible_long_flag_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a "visible" long flag subcommand.

- `fn visible_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Sets multiple visible aliases to this subcommand.

- `fn visible_short_flag_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as *visible* short flag subcommands.

- `fn visible_long_flag_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as *visible* long flag subcommands.

- `fn display_order(self: Self, ord: impl IntoResettable<usize>) -> Self`
  Set the placement of this subcommand within the help.

- `fn hide(self: Self, yes: bool) -> Self`
  Specifies that this [`subcommand`] should be hidden from help messages

- `fn subcommand_required(self: Self, yes: bool) -> Self`
  If no [`subcommand`] is present at runtime, error and exit gracefully.

- `fn allow_external_subcommands(self: Self, yes: bool) -> Self`
  Assume unexpected positional arguments are a [`subcommand`].

- `fn external_subcommand_value_parser(self: Self, parser: impl IntoResettable<super::ValueParser>) -> Self`
  Specifies how to parse external subcommand arguments.

- `fn args_conflicts_with_subcommands(self: Self, yes: bool) -> Self`
  Specifies that use of an argument prevents the use of [`subcommands`].

- `fn subcommand_precedence_over_arg(self: Self, yes: bool) -> Self`
  Prevent subcommands from being consumed as an arguments value.

- `fn subcommand_negates_reqs(self: Self, yes: bool) -> Self`
  Allows [`subcommands`] to override all requirements of the parent command.

- `fn multicall(self: Self, yes: bool) -> Self`
  Multiple-personality program dispatched on the binary name (`argv[0]`)

- `fn subcommand_value_name(self: Self, value_name: impl IntoResettable<Str>) -> Self`
  Sets the value name used for subcommands when printing usage and help.

- `fn subcommand_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Sets the help heading used for subcommands when printing usage and help.

- `fn new(name: impl Into<Str>) -> Self`
  Creates a new instance of an `Command`.

- `fn arg(self: Self, a: impl Into<Arg>) -> Self`
  Adds an [argument] to the list of valid possibilities.

- `fn args(self: Self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self`
  Adds multiple [arguments] to the list of valid possibilities.

- `fn mut_arg<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate an [`Arg`] after it's been added to a [`Command`].

- `fn mut_args<F>(self: Self, f: F) -> Self`
  Allows one to mutate all [`Arg`]s after they've been added to a [`Command`].

- `fn mut_group<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate an [`ArgGroup`] after it's been added to a [`Command`].

- `fn mut_subcommand<F>(self: Self, name: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate a [`Command`] after it's been added as a subcommand.

- `fn mut_subcommands<F>(self: Self, f: F) -> Self`
  Allows one to mutate all [`Command`]s after they've been added as subcommands.

- `fn group(self: Self, group: impl Into<ArgGroup>) -> Self`
  Adds an [`ArgGroup`] to the application.

- `fn groups(self: Self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self`
  Adds multiple [`ArgGroup`]s to the [`Command`] at once.

- `fn subcommand(self: Self, subcmd: impl Into<Command>) -> Self`
  Adds a subcommand to the list of valid possibilities.

- `fn subcommands(self: Self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`
  Adds multiple subcommands to the list of valid possibilities.

- `fn defer(self: Self, deferred: fn(Command) -> Command) -> Self`
  Delay initialization for parts of the `Command`

- `fn debug_assert(self: Self)`
  Catch problems earlier in the development cycle.

- `fn error(self: &mut Self, kind: ErrorKind, message: impl fmt::Display) -> Error`
  Custom error message for post-parsing validation

- `fn get_matches(self: Self) -> ArgMatches`
  Parse [`env::args_os`], [exiting][Error::exit] on failure.

- `fn get_matches_mut(self: &mut Self) -> ArgMatches`
  Parse [`env::args_os`], [exiting][Error::exit] on failure.

- `fn try_get_matches(self: Self) -> ClapResult<ArgMatches>`
  Parse [`env::args_os`], returning a [`clap::Result`] on failure.

- `fn get_matches_from<I, T>(self: Self, itr: I) -> ArgMatches`
  Parse the specified arguments, [exiting][Error::exit] on failure.

- `fn try_get_matches_from<I, T>(self: Self, itr: I) -> ClapResult<ArgMatches>`
  Parse the specified arguments, returning a [`clap::Result`] on failure.

- `fn try_get_matches_from_mut<I, T>(self: &mut Self, itr: I) -> ClapResult<ArgMatches>`
  Parse the specified arguments, returning a [`clap::Result`] on failure.

- `fn print_help(self: &mut Self) -> io::Result<()>`
  Prints the short help message (`-h`) to [`io::stdout()`].

- `fn print_long_help(self: &mut Self) -> io::Result<()>`
  Prints the long help message (`--help`) to [`io::stdout()`].

- `fn render_help(self: &mut Self) -> StyledStr`
  Render the short help message (`-h`) to a [`StyledStr`]

- `fn render_long_help(self: &mut Self) -> StyledStr`
  Render the long help message (`--help`) to a [`StyledStr`].

- `fn render_version(self: &Self) -> String`
  Version message rendered as if the user ran `-V`.

- `fn render_long_version(self: &Self) -> String`
  Version message rendered as if the user ran `--version`.

- `fn render_usage(self: &mut Self) -> StyledStr`
  Usage statement

- `fn no_binary_name(self: Self, yes: bool) -> Self`
  Specifies that the parser should not assume the first argument passed is the binary name.

- `fn ignore_errors(self: Self, yes: bool) -> Self`
  Try not to fail on parse errors, like missing option values.

- `fn args_override_self(self: Self, yes: bool) -> Self`
  Replace prior occurrences of arguments rather than error

- `fn dont_delimit_trailing_values(self: Self, yes: bool) -> Self`
  Disables the automatic [delimiting of values][Arg::value_delimiter] after `--` or when [`Arg::trailing_var_arg`]

- `fn color(self: Self, color: ColorChoice) -> Self`
  Sets when to color output.

- `fn styles(self: Self, styles: Styles) -> Self`
  Sets the [`Styles`] for terminal output

- `fn term_width(self: Self, width: usize) -> Self`
  Sets the terminal width at which to wrap help messages.

- `fn max_term_width(self: Self, width: usize) -> Self`
  Limit the line length for wrapping help when using the current terminal's width.

- `fn disable_version_flag(self: Self, yes: bool) -> Self`
  Disables `-V` and `--version` flag.

- `fn propagate_version(self: Self, yes: bool) -> Self`
  Specifies to use the version of the current command for all [`subcommands`].

- `fn next_line_help(self: Self, yes: bool) -> Self`
  Places the help string for all arguments and subcommands on the line after them.

- `fn disable_help_flag(self: Self, yes: bool) -> Self`
  Disables `-h` and `--help` flag.

- `fn disable_help_subcommand(self: Self, yes: bool) -> Self`
  Disables the `help` [`subcommand`].

- `fn disable_colored_help(self: Self, yes: bool) -> Self`
  Disables colorized help messages.

- `fn help_expected(self: Self, yes: bool) -> Self`
   Panic if help descriptions are omitted.

- `fn hide_possible_values(self: Self, yes: bool) -> Self`
  Tells `clap` *not* to print possible values when displaying help information.

- `fn infer_long_args(self: Self, yes: bool) -> Self`
  Allow partial matches of long arguments or their [aliases].

- `fn infer_subcommands(self: Self, yes: bool) -> Self`
  Allow partial matches of [subcommand] names and their [aliases].

- `fn name(self: Self, name: impl Into<Str>) -> Self`
  (Re)Sets the program's name.

- `fn bin_name(self: Self, name: impl IntoResettable<String>) -> Self`
  Overrides the runtime-determined name of the binary for help and error messages.

- `fn display_name(self: Self, name: impl IntoResettable<String>) -> Self`
  Overrides the runtime-determined display name of the program for help and error messages.

- `fn author(self: Self, author: impl IntoResettable<Str>) -> Self`
  Sets the author(s) for the help message.

- `fn about(self: Self, about: impl IntoResettable<StyledStr>) -> Self`
  Sets the program's description for the short help (`-h`).

- `fn long_about(self: Self, long_about: impl IntoResettable<StyledStr>) -> Self`
  Sets the program's description for the long help (`--help`).

- `fn after_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for after auto-generated short help (`-h`).

- `fn after_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for after auto-generated long help (`--help`).

- `fn before_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for before auto-generated short help (`-h`).

- `fn before_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for before auto-generated long help (`--help`).

- `fn version(self: Self, ver: impl IntoResettable<Str>) -> Self`
  Sets the version for the short version (`-V`) and help messages.

- `fn long_version(self: Self, ver: impl IntoResettable<Str>) -> Self`
  Sets the version for the long version (`--version`) and help messages.

- `fn override_usage(self: Self, usage: impl IntoResettable<StyledStr>) -> Self`
  Overrides the `clap` generated usage string for help and error messages.

- `fn override_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Overrides the `clap` generated help message (both `-h` and `--help`).

- `fn help_template(self: Self, s: impl IntoResettable<StyledStr>) -> Self`
  Sets the help template to be used, overriding the default format.

- `fn flatten_help(self: Self, yes: bool) -> Self`
  Flatten subcommand help into the current command's help

- `fn next_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Set the default section heading for future args.

- `fn next_display_order(self: Self, disp_ord: impl IntoResettable<usize>) -> Self`
  Change the starting value for assigning future display orders for args.

- `fn arg_required_else_help(self: Self, yes: bool) -> Self`
  Exit gracefully if no arguments are present (e.g. `$ myprog`).

- `fn allow_missing_positional(self: Self, yes: bool) -> Self`
  Allows one to implement two styles of CLIs where positionals can be used out of order.

- `fn build(self: &mut Self)`
  Prepare for introspecting on all included [`Command`]s

- `fn get_display_name(self: &Self) -> Option<&str>`
  Get the name of the binary.

- `fn get_bin_name(self: &Self) -> Option<&str>`
  Get the name of the binary.

- `fn set_bin_name(self: &mut Self, name: impl Into<String>)`
  Set binary name. Uses `&mut self` instead of `self`.

- `fn get_name(self: &Self) -> &str`
  Get the name of the cmd.

- `fn get_name_and_visible_aliases(self: &Self) -> Vec<&str>`
  Get all known names of the cmd (i.e. primary name and visible aliases).

- `fn get_version(self: &Self) -> Option<&str>`
  Get the version of the cmd.

- `fn get_long_version(self: &Self) -> Option<&str>`
  Get the long version of the cmd.

- `fn get_display_order(self: &Self) -> usize`
  Get the placement within help

- `fn get_author(self: &Self) -> Option<&str>`
  Get the authors of the cmd.

- `fn get_short_flag(self: &Self) -> Option<char>`
  Get the short flag of the subcommand.

- `fn get_long_flag(self: &Self) -> Option<&str>`
  Get the long flag of the subcommand.

- `fn get_about(self: &Self) -> Option<&StyledStr>`
  Get the help message specified via [`Command::about`].

- `fn get_long_about(self: &Self) -> Option<&StyledStr>`
  Get the help message specified via [`Command::long_about`].

- `fn is_flatten_help_set(self: &Self) -> bool`
  Get the custom section heading specified via [`Command::flatten_help`].

- `fn get_next_help_heading(self: &Self) -> Option<&str>`
  Get the custom section heading specified via [`Command::next_help_heading`].

- `fn get_visible_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *visible* aliases for this subcommand.

- `fn get_visible_short_flag_aliases(self: &Self) -> impl Iterator<Item = char> + '_`
  Iterate through the *visible* short aliases for this subcommand.

- `fn get_visible_long_flag_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *visible* long aliases for this subcommand.

- `fn get_all_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the set of *all* the aliases for this subcommand, both visible and hidden.

- `fn get_all_short_flag_aliases(self: &Self) -> impl Iterator<Item = char> + '_`
  Iterate through the set of *all* the short aliases for this subcommand, both visible and hidden.

- `fn get_all_long_flag_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the set of *all* the long aliases for this subcommand, both visible and hidden.

- `fn get_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *hidden* aliases for this subcommand.

- `fn get_color(self: &Self) -> ColorChoice`
  Should we color the output?

- `fn get_styles(self: &Self) -> &Styles`
  Return the current `Styles` for the `Command`

- `fn get_subcommands(self: &Self) -> impl Iterator<Item = &Command>`
  Iterate through the set of subcommands, getting a reference to each.

- `fn get_subcommands_mut(self: &mut Self) -> impl Iterator<Item = &mut Command>`
  Iterate through the set of subcommands, getting a mutable reference to each.

- `fn has_subcommands(self: &Self) -> bool`
  Returns `true` if this `Command` has subcommands.

- `fn get_subcommand_help_heading(self: &Self) -> Option<&str>`
  Returns the help heading for listing subcommands.

- `fn get_subcommand_value_name(self: &Self) -> Option<&str>`
  Returns the subcommand value name.

- `fn get_before_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_before_long_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_after_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_after_long_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn find_subcommand(self: &Self, name: impl AsRef<std::ffi::OsStr>) -> Option<&Command>`
  Find subcommand such that its name or one of aliases equals `name`.

- `fn find_subcommand_mut(self: &mut Self, name: impl AsRef<std::ffi::OsStr>) -> Option<&mut Command>`
  Find subcommand such that its name or one of aliases equals `name`, returning

- `fn get_groups(self: &Self) -> impl Iterator<Item = &ArgGroup>`
  Iterate through the set of groups.

- `fn get_arguments(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the set of arguments.

- `fn get_positionals(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the *positionals* arguments.

- `fn get_opts(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the *options*.

- `fn get_arg_conflicts_with(self: &Self, arg: &Arg) -> Vec<&Arg>`
  Get a list of all arguments the given argument conflicts with.

- `fn is_no_binary_name_set(self: &Self) -> bool`
  Report whether [`Command::no_binary_name`] is set

- `fn is_dont_delimit_trailing_values_set(self: &Self) -> bool`
  Report whether [`Command::dont_delimit_trailing_values`] is set

- `fn is_disable_version_flag_set(self: &Self) -> bool`
  Report whether [`Command::disable_version_flag`] is set

- `fn is_propagate_version_set(self: &Self) -> bool`
  Report whether [`Command::propagate_version`] is set

- `fn is_next_line_help_set(self: &Self) -> bool`
  Report whether [`Command::next_line_help`] is set

- `fn is_disable_help_flag_set(self: &Self) -> bool`
  Report whether [`Command::disable_help_flag`] is set

- `fn is_disable_help_subcommand_set(self: &Self) -> bool`
  Report whether [`Command::disable_help_subcommand`] is set

- `fn is_disable_colored_help_set(self: &Self) -> bool`
  Report whether [`Command::disable_colored_help`] is set

- `fn is_arg_required_else_help_set(self: &Self) -> bool`
  Report whether [`Command::arg_required_else_help`] is set

- `fn is_allow_missing_positional_set(self: &Self) -> bool`
  Report whether [`Command::allow_missing_positional`] is set

- `fn is_hide_set(self: &Self) -> bool`
  Report whether [`Command::hide`] is set

- `fn is_subcommand_required_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_required`] is set

- `fn is_allow_external_subcommands_set(self: &Self) -> bool`
  Report whether [`Command::allow_external_subcommands`] is set

- `fn get_external_subcommand_value_parser(self: &Self) -> Option<&super::ValueParser>`
  Configured parser for values passed to an external subcommand

- `fn is_args_conflicts_with_subcommands_set(self: &Self) -> bool`
  Report whether [`Command::args_conflicts_with_subcommands`] is set

- `fn is_subcommand_precedence_over_arg_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_precedence_over_arg`] is set

- `fn is_subcommand_negates_reqs_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_negates_reqs`] is set

- `fn is_multicall_set(self: &Self) -> bool`
  Report whether [`Command::multicall`] is set

#### Trait Implementations

##### `impl From`

- `fn from(cmd: &Command) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Command`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `Arg`

```rust
struct Arg {
    // [REDACTED: Private Fields]
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
# use clap_builder as clap;
# use clap::{Arg, arg, ArgAction};
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

- `fn group(self: Self, group_id: impl IntoResettable<Id>) -> Self`
  The name of the [`ArgGroup`] the argument belongs to.

- `fn groups(self: Self, group_ids: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  The names of [`ArgGroup`]'s the argument belongs to.

- `fn default_value_if(self: Self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, default: impl IntoResettable<OsStr>) -> Self`
  Specifies the value of the argument if `arg` has been used at runtime.

- `fn default_values_if(self: Self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, defaults: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Specifies the values of the argument if `arg` has been used at runtime.

- `fn default_value_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>)>) -> Self`
  Specifies multiple values and conditions in the same manner as [`Arg::default_value_if`].

- `fn default_values_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoIterator<Item = impl Into<OsStr>>)>) -> Self`
  Specifies multiple values and conditions in the same manner as [`Arg::default_values_if`].

- `fn required_unless_present(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Set this arg as [required] as long as the specified argument is not present at runtime.

- `fn required_unless_present_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets this arg as [required] unless *all* of the specified arguments are present at runtime.

- `fn required_unless_present_any(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets this arg as [required] unless *any* of the specified arguments are present at runtime.

- `fn required_if_eq(self: Self, arg_id: impl Into<Id>, val: impl Into<OsStr>) -> Self`
  This argument is [required] only if the specified `arg` is present at runtime and its value

- `fn required_if_eq_any(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self`
  Specify this argument is [required] based on multiple conditions.

- `fn required_if_eq_all(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self`
  Specify this argument is [required] based on multiple conditions.

- `fn requires_if(self: Self, val: impl Into<ArgPredicate>, arg_id: impl Into<Id>) -> Self`
  Require another argument if this arg matches the [`ArgPredicate`]

- `fn requires_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<ArgPredicate>, impl Into<Id>)>) -> Self`
  Allows multiple conditional requirements.

- `fn conflicts_with(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  This argument is mutually exclusive with the specified argument.

- `fn conflicts_with_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  This argument is mutually exclusive with the specified arguments.

- `fn overrides_with(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Sets an overridable argument.

- `fn overrides_with_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets multiple mutually overridable arguments by name.

- `fn new(id: impl Into<Id>) -> Self`
  Create a new [`Arg`] with a unique name.

- `fn id(self: Self, id: impl Into<Id>) -> Self`
  Set the identifier used for referencing this argument in the clap API.

- `fn short(self: Self, s: impl IntoResettable<char>) -> Self`
  Sets the short version of the argument without the preceding `-`.

- `fn long(self: Self, l: impl IntoResettable<Str>) -> Self`
  Sets the long version of the argument without the preceding `--`.

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a hidden long flag.

- `fn short_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as a hidden short flag.

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as hidden long flags.

- `fn short_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which functions as a hidden short flag.

- `fn visible_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a visible long flag.

- `fn visible_short_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as a visible short flag.

- `fn visible_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as visible long flags.

- `fn visible_short_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as visible short flags.

- `fn index(self: Self, idx: impl IntoResettable<usize>) -> Self`
  Specifies the index of a positional argument **starting at** 1.

- `fn trailing_var_arg(self: Self, yes: bool) -> Self`
  This is a "var arg" and everything that follows should be captured by it, as if the user had

- `fn last(self: Self, yes: bool) -> Self`
  This arg is the last, or final, positional argument (i.e. has the highest

- `fn required(self: Self, yes: bool) -> Self`
  Specifies that the argument must be present.

- `fn requires(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Sets an argument that is required when this one is present

- `fn exclusive(self: Self, yes: bool) -> Self`
  This argument must be passed alone; it conflicts with all other arguments.

- `fn global(self: Self, yes: bool) -> Self`
  Specifies that an argument can be matched to all child [`Subcommand`]s.

- `fn help(self: Self, h: impl IntoResettable<StyledStr>) -> Self`
  Sets the description of the argument for short help (`-h`).

- `fn long_help(self: Self, h: impl IntoResettable<StyledStr>) -> Self`
  Sets the description of the argument for long help (`--help`).

- `fn display_order(self: Self, ord: impl IntoResettable<usize>) -> Self`
  Allows custom ordering of args within the help message.

- `fn help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Override the `--help` section this appears in.

- `fn next_line_help(self: Self, yes: bool) -> Self`
  Render the [help][Arg::help] on the line after the argument.

- `fn hide(self: Self, yes: bool) -> Self`
  Do not display the argument in help message.

- `fn hide_possible_values(self: Self, yes: bool) -> Self`
  Do not display the [possible values][crate::builder::ValueParser::possible_values] in the help message.

- `fn hide_default_value(self: Self, yes: bool) -> Self`
  Do not display the default value of the argument in the help message.

- `fn hide_short_help(self: Self, yes: bool) -> Self`
  Hides an argument from short help (`-h`).

- `fn hide_long_help(self: Self, yes: bool) -> Self`
  Hides an argument from long help (`--help`).

- `fn get_id(self: &Self) -> &Id`
  Get the name of the argument

- `fn get_help(self: &Self) -> Option<&StyledStr>`
  Get the help specified for this argument, if any

- `fn get_long_help(self: &Self) -> Option<&StyledStr>`
  Get the long help specified for this argument, if any

- `fn get_display_order(self: &Self) -> usize`
  Get the placement within help

- `fn get_help_heading(self: &Self) -> Option<&str>`
  Get the help heading specified for this argument, if any

- `fn get_short(self: &Self) -> Option<char>`
  Get the short option name for this argument, if any

- `fn get_visible_short_aliases(self: &Self) -> Option<Vec<char>>`
  Get visible short aliases for this argument, if any

- `fn get_all_short_aliases(self: &Self) -> Option<Vec<char>>`
  Get *all* short aliases for this argument, if any, both visible and hidden.

- `fn get_short_and_visible_aliases(self: &Self) -> Option<Vec<char>>`
  Get the short option name and its visible aliases, if any

- `fn get_long(self: &Self) -> Option<&str>`
  Get the long option name for this argument, if any

- `fn get_visible_aliases(self: &Self) -> Option<Vec<&str>>`
  Get visible aliases for this argument, if any

- `fn get_all_aliases(self: &Self) -> Option<Vec<&str>>`
  Get *all* aliases for this argument, if any, both visible and hidden.

- `fn get_long_and_visible_aliases(self: &Self) -> Option<Vec<&str>>`
  Get the long option name and its visible aliases, if any

- `fn get_aliases(self: &Self) -> Option<Vec<&str>>`
  Get hidden aliases for this argument, if any

- `fn get_possible_values(self: &Self) -> Vec<PossibleValue>`
  Get the names of possible values for this argument. Only useful for user

- `fn get_value_names(self: &Self) -> Option<&[Str]>`
  Get the names of values for this argument.

- `fn get_num_args(self: &Self) -> Option<ValueRange>`
  Get the number of values for this argument.

- `fn get_value_delimiter(self: &Self) -> Option<char>`
  Get the delimiter between multiple values

- `fn get_value_terminator(self: &Self) -> Option<&Str>`
  Get the value terminator for this argument. The `value_terminator` is a value

- `fn get_index(self: &Self) -> Option<usize>`
  Get the index of this argument, if any

- `fn get_value_hint(self: &Self) -> ValueHint`
  Get the value hint of this argument

- `fn get_default_values(self: &Self) -> &[OsStr]`
  Get the default values specified for this argument, if any

- `fn is_positional(self: &Self) -> bool`
  Checks whether this argument is a positional or not.

- `fn is_required_set(self: &Self) -> bool`
  Reports whether [`Arg::required`] is set

- `fn is_allow_hyphen_values_set(self: &Self) -> bool`
  Report whether [`Arg::allow_hyphen_values`] is set

- `fn is_allow_negative_numbers_set(self: &Self) -> bool`
  Report whether [`Arg::allow_negative_numbers`] is set

- `fn get_action(self: &Self) -> &ArgAction`
  Behavior when parsing the argument

- `fn get_value_parser(self: &Self) -> &super::ValueParser`
  Configured parser for argument values

- `fn is_global_set(self: &Self) -> bool`
  Report whether [`Arg::global`] is set

- `fn is_next_line_help_set(self: &Self) -> bool`
  Report whether [`Arg::next_line_help`] is set

- `fn is_hide_set(self: &Self) -> bool`
  Report whether [`Arg::hide`] is set

- `fn is_hide_default_value_set(self: &Self) -> bool`
  Report whether [`Arg::hide_default_value`] is set

- `fn is_hide_possible_values_set(self: &Self) -> bool`
  Report whether [`Arg::hide_possible_values`] is set

- `fn is_hide_short_help_set(self: &Self) -> bool`
  Report whether [`Arg::hide_short_help`] is set

- `fn is_hide_long_help_set(self: &Self) -> bool`
  Report whether [`Arg::hide_long_help`] is set

- `fn is_require_equals_set(self: &Self) -> bool`
  Report whether [`Arg::require_equals`] is set

- `fn is_exclusive_set(self: &Self) -> bool`
  Reports whether [`Arg::exclusive`] is set

- `fn is_trailing_var_arg_set(self: &Self) -> bool`
  Report whether [`Arg::trailing_var_arg`] is set

- `fn is_last_set(self: &Self) -> bool`
  Reports whether [`Arg::last`] is set

- `fn is_ignore_case_set(self: &Self) -> bool`
  Reports whether [`Arg::ignore_case`] is set

- `fn action(self: Self, action: impl IntoResettable<ArgAction>) -> Self`
  Specify how to react to an argument when parsing it.

- `fn value_parser(self: Self, parser: impl IntoResettable<super::ValueParser>) -> Self`
  Specify the typed behavior of the argument.

- `fn num_args(self: Self, qty: impl IntoResettable<ValueRange>) -> Self`
  Specifies the number of arguments parsed per occurrence

- `fn value_name(self: Self, name: impl IntoResettable<Str>) -> Self`
  Placeholder for the argument's value in the help message / usage.

- `fn value_names(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Placeholders for the argument's values in the help message / usage.

- `fn value_hint(self: Self, value_hint: impl IntoResettable<ValueHint>) -> Self`
  Provide the shell a hint about how to complete this argument.

- `fn ignore_case(self: Self, yes: bool) -> Self`
  Match values against [`PossibleValuesParser`][crate::builder::PossibleValuesParser] without matching case.

- `fn allow_hyphen_values(self: Self, yes: bool) -> Self`
  Allows values which start with a leading hyphen (`-`)

- `fn allow_negative_numbers(self: Self, yes: bool) -> Self`
  Allows negative numbers to pass as values.

- `fn require_equals(self: Self, yes: bool) -> Self`
  Requires that options use the `--option=val` syntax

- `fn value_delimiter(self: Self, d: impl IntoResettable<char>) -> Self`
  Allow grouping of multiple values via a delimiter.

- `fn value_terminator(self: Self, term: impl IntoResettable<Str>) -> Self`
  Sentinel to **stop** parsing multiple values of a given argument.

- `fn raw(self: Self, yes: bool) -> Self`
  Consume all following arguments.

- `fn default_value(self: Self, val: impl IntoResettable<OsStr>) -> Self`
  Value for the argument when not present.

- `fn default_values(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when not present.

- `fn default_missing_value(self: Self, val: impl IntoResettable<OsStr>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_value_os(self: Self, val: impl Into<OsStr>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_values(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_values_os(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when the flag is present but no value is specified.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(a: &Arg) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Arg) -> Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Arg) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default`

- `fn default() -> Arg`

### `ArgGroup`

```rust
struct ArgGroup {
    // [REDACTED: Private Fields]
}
```

Specifies a logical group of [arguments](#arguments)


You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts](#conflicts)
 or [requirements](#requirements)

- check which argument in a group was specified on the command-line

For visually grouping arguments in help, see instead
`Arg::help_heading`.

# Examples

The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
the arguments from the specified group is present at runtime.

```rust
# use clap_builder as clap;
# use clap::{Command, arg, ArgGroup, error::ErrorKind};
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
# use clap_builder as clap;
# use clap::{Command, arg, ArgGroup, Id};
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
[arguments](#arguments)
: crate::Arg
[conflicts](#conflicts)
: crate::Arg::conflicts_with()
[requirements](#requirements)
: crate::Arg::requires()

#### Implementations

- `fn new(id: impl Into<Id>) -> Self`
  Create a `ArgGroup` using a unique name.

- `fn id(self: Self, id: impl Into<Id>) -> Self`
  Sets the group name.

- `fn arg(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Adds an [argument] to this group by name

- `fn args(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Adds multiple [arguments] to this group by name

- `fn get_args(self: &Self) -> impl Iterator<Item = &Id>`
  Getters for all args. It will return a vector of `Id`

- `fn multiple(self: Self, yes: bool) -> Self`
  Allows more than one of the [`Arg`]s in this group to be used. (Default: `false`)

- `fn is_multiple(self: &mut Self) -> bool`
  Return true if the group allows more than one of the arguments

- `fn required(self: Self, yes: bool) -> Self`
  Require an argument from the group to be present when parsing.

- `fn requires(self: Self, id: impl IntoResettable<Id>) -> Self`
  Specify an argument or group that must be present when this group is.

- `fn requires_all(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Specify arguments or groups that must be present when this group is.

- `fn conflicts_with(self: Self, id: impl IntoResettable<Id>) -> Self`
  Specify an argument or group that must **not** be present when this group is.

- `fn conflicts_with_all(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Specify arguments or groups that must **not** be present when this group is.

- `fn get_id(self: &Self) -> &Id`
  Get the name of the group

- `fn is_required_set(self: &Self) -> bool`
  Reports whether [`ArgGroup::required`] is set

#### Trait Implementations

##### `impl From`

- `fn from(g: &ArgGroup) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ArgGroup`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgGroup) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ArgGroup`

### `ArgMatches`

```rust
struct ArgMatches {
    // [REDACTED: Private Fields]
}
```

Container for parse results.

Used to get information about the arguments that were supplied to the program at runtime by
the user. New instances of this struct are obtained by using the `Command::get_matches` family of
methods.

# Examples

```no_run
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
# use clap::parser::ValueSource;
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

- `fn get_one<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<&T>`
  Gets the value of a specific option or positional argument.

- `fn get_count(self: &Self, id: &str) -> u8`
  Gets the value of a specific [`ArgAction::Count`][crate::ArgAction::Count] flag

- `fn get_flag(self: &Self, id: &str) -> bool`
  Gets the value of a specific [`ArgAction::SetTrue`][crate::ArgAction::SetTrue] or [`ArgAction::SetFalse`][crate::ArgAction::SetFalse] flag

- `fn get_many<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<ValuesRef<'_, T>>`
  Iterate over values of a specific option or positional argument.

- `fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<OccurrencesRef<'_, T>>`
  Iterate over the values passed to each occurrence of an option.

- `fn get_raw(self: &Self, id: &str) -> Option<RawValues<'_>>`
  Iterate over the original argument values.

- `fn get_raw_occurrences(self: &Self, id: &str) -> Option<RawOccurrences<'_>>`
  Iterate over the original values for each occurrence of an option.

- `fn remove_one<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<T>`
  Returns the value of a specific option or positional argument.

- `fn remove_many<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<Values<T>>`
  Return values of a specific option or positional argument.

- `fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<Occurrences<T>>`
  Return values for each occurrence of an option.

- `fn contains_id(self: &Self, id: &str) -> bool`
  Check if values are present for the argument or group id

- `fn ids(self: &Self) -> IdsRef<'_>`
  Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`].

- `fn args_present(self: &Self) -> bool`
  Check if any [`Arg`][crate::Arg]s were present on the command line

- `fn value_source(self: &Self, id: &str) -> Option<ValueSource>`
  Report where argument value came from

- `fn index_of(self: &Self, id: &str) -> Option<usize>`
  The first index of that an argument showed up.

- `fn indices_of(self: &Self, id: &str) -> Option<Indices<'_>>`
  All indices an argument appeared at when parsing.

- `fn try_get_one<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<&T>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_one`]

- `fn try_get_many<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<ValuesRef<'_, T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_many`]

- `fn try_get_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<OccurrencesRef<'_, T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_occurrences`]

- `fn try_get_raw(self: &Self, id: &str) -> Result<Option<RawValues<'_>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_raw`]

- `fn try_get_raw_occurrences(self: &Self, id: &str) -> Result<Option<RawOccurrences<'_>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_raw_occurrences`]

- `fn try_remove_one<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<T>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_one`]

- `fn try_remove_many<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Values<T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_many`]

- `fn try_remove_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Occurrences<T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_occurrences`]

- `fn try_contains_id(self: &Self, id: &str) -> Result<bool, MatchesError>`
  Non-panicking version of [`ArgMatches::contains_id`]

- `fn try_clear_id(self: &mut Self, id: &str) -> Result<bool, MatchesError>`
  Clears the values for the given `id`

- `fn subcommand(self: &Self) -> Option<(&str, &ArgMatches)>`
  The name and `ArgMatches` of the current [subcommand].

- `fn remove_subcommand(self: &mut Self) -> Option<(String, ArgMatches)>`
  Return the name and `ArgMatches` of the current [subcommand].

- `fn subcommand_matches(self: &Self, name: &str) -> Option<&ArgMatches>`
  The `ArgMatches` for the current [subcommand].

- `fn subcommand_name(self: &Self) -> Option<&str>`
  The name of the current [subcommand].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ArgMatches`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgMatches) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ArgMatches`

### `Id`

```rust
struct Id();
```

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- `fn as_str(self: &Self) -> &str`
  Get the raw string of the `Id`

#### Trait Implementations

##### `impl From`

- `fn from(name: &'static str) -> Self`

##### `impl From`

- `fn from(name: &&'static str) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(name: &Str) -> Self`

##### `impl From`

- `fn from(id: &Id) -> Self`

##### `impl From`

- `fn from(name: Str) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<Str>`

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<String>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &str`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Id`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Id) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Str) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default`

- `fn default() -> Id`

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
# #[cfg(feature = "help")] {
# use clap_builder as clap;
# use clap::Command;
# use clap::Arg;
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
# }
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
  # use clap::builder::TypedValueParser as _;
  # use clap::builder::BoolishValueParser;
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # #[cfg(feature = "help")] {
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # }
  ```

- **`HelpShort`**

  When encountered, display `Command::print_help`
  
  # Examples
  
  ```rust
  # #[cfg(feature = "help")] {
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # }
  ```

- **`HelpLong`**

  When encountered, display `Command::print_long_help`
  
  # Examples
  
  ```rust
  # #[cfg(feature = "help")] {
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  # }
  ```

- **`Version`**

  When encountered, display `Command::version`
  
  Depending on the flag, `Command::long_version` may be shown
  
  # Examples
  
  ```rust
  # use clap_builder as clap;
  # use clap::Command;
  # use clap::Arg;
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
  Returns whether this action accepts values on the command-line

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable`

- `fn into_resettable(self: Self) -> Resettable<ArgAction>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ArgAction`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable`

- `fn into_resettable(self: Self) -> Resettable<ValueHint>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ValueHint`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ValueHint) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ValueHint`

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
  # #[cfg(feature = "color")] {
  # use clap_builder as clap;
  # use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Auto)
      .get_matches();
  # }
  ```

- **`Always`**

  Enables colored output regardless of whether or not the output is going to a terminal/TTY.
  
  # Examples
  
  ```rust
  # #[cfg(feature = "color")] {
  # use clap_builder as clap;
  # use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Always)
      .get_matches();
  # }
  ```

- **`Never`**

  Disables colored output no matter if the output is going to a terminal/TTY, or not.
  
  # Examples
  
  ```rust
  # #[cfg(feature = "color")] {
  # use clap_builder as clap;
  # use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Never)
      .get_matches();
  # }
  ```

#### Implementations

- `fn possible_values() -> impl Iterator<Item = PossibleValue>`
  Report all `possible_values`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ColorChoice`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ColorChoice) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value(self: &Self) -> Option<PossibleValue>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ColorChoice`

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

Create an [`Arg`](#arg) from a usage string.

Allows creation of basic settings for the [`Arg`](#arg).

<div class="warning">

**NOTE**: Not all settings may be set using the usage string method. Some properties are
only available via the builder pattern.

</div>

# Syntax

Usage strings typically following the form:

```notrust
[explicit name] [short](#short)
 [long](#long)
 [value names] [...] [help string]
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
# use clap_builder as clap;
# use clap::{Command, Arg, arg};
let cmd = Command::new("prog")
    .args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input](#input)
 "an optional input file to use")
    ]);

let m = cmd.try_get_matches_from(["prog", "--config", "file.toml"]).unwrap();
assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
assert_eq!(m.get_one::<String>("input"), None);
```


### `value_parser!`

Select a [`ValueParser`](#valueparser) implementation from the intended type

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
# use clap_builder as clap;
# use std::path::PathBuf;
# use std::path::Path;
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
# use clap_builder as clap;
# use clap::ColorChoice;
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

