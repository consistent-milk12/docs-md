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
  - [`Command`](#command)
  - [`Arg`](#arg)
  - [`ArgGroup`](#arggroup)
  - [`ArgMatches`](#argmatches)
  - [`Id`](#id)
- [Enums](#enums)
  - [`ArgAction`](#argaction)
  - [`ValueHint`](#valuehint)
  - [`ColorChoice`](#colorchoice)
- [Traits](#traits)
  - [`Args`](#args)
  - [`CommandFactory`](#commandfactory)
  - [`FromArgMatches`](#fromargmatches)
  - [`Parser`](#parser)
  - [`Subcommand`](#subcommand)
  - [`ValueEnum`](#valueenum)
- [Type Aliases](#type-aliases)
  - [`Error`](#error)
- [Constants](#constants)
  - [`INTERNAL_ERROR_MSG`](#internal-error-msg)
- [Macros](#macros)
  - [`command!`](#command)
  - [`arg!`](#arg)
  - [`value_parser!`](#value-parser)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`derive`](#derive) | mod | This module contains traits that are usable with the `#[derive(...)]` macros in `clap_derive`. |
| [`builder`](#builder) | mod | Define [`Command`] line [arguments][`Arg`] |
| [`error`](#error) | mod | Error reporting |
| [`parser`](#parser) | mod | [`Command`][crate::Command] line argument parser |
| [`mkeymap`](#mkeymap) | mod |  |
| [`output`](#output) | mod |  |
| [`util`](#util) | mod |  |
| [`Command`](#command) | struct |  |
| [`Arg`](#arg) | struct |  |
| [`ArgGroup`](#arggroup) | struct |  |
| [`ArgMatches`](#argmatches) | struct |  |
| [`Id`](#id) | struct |  |
| [`ArgAction`](#argaction) | enum |  |
| [`ValueHint`](#valuehint) | enum |  |
| [`ColorChoice`](#colorchoice) | enum |  |
| [`Args`](#args) | trait |  |
| [`CommandFactory`](#commandfactory) | trait |  |
| [`FromArgMatches`](#fromargmatches) | trait |  |
| [`Parser`](#parser) | trait |  |
| [`Subcommand`](#subcommand) | trait |  |
| [`ValueEnum`](#valueenum) | trait |  |
| [`Error`](#error) | type | Command Line Argument Parser Error |
| [`INTERNAL_ERROR_MSG`](#internal-error-msg) | const |  |
| [`command!`](#command) | macro | Requires `cargo` feature flag to be enabled. |
| [`arg!`](#arg) | macro | Create an [`Arg`] from a usage string. |
| [`value_parser!`](#value-parser) | macro | Select a [`ValueParser`] implementation from the intended type |

## Modules

- [`macros`](macros/index.md)
- [`derive`](derive/index.md) — This module contains traits that are usable with the `#[derive(...)]`
- [`builder`](builder/index.md) — Define [`Command`] line [arguments][`Arg`]
- [`error`](error/index.md) — Error reporting
- [`parser`](parser/index.md) — [`Command`][crate::Command] line argument parser
- [`mkeymap`](mkeymap/index.md)
- [`output`](output/index.md)
- [`util`](util/index.md)

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

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:74-113`](../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L74-L113)*

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

- <span id="command-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](builder/str/index.md#str)

  Creates a new instance of an `Command`.

  

  It is common, but not required, to use binary name as the `name`. This

  name will only be displayed to the user when they request to print

  version or help and usage information.

  

  See also [`command!`](crate::command!) and [`crate_name!`](crate::crate_name!).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  Command::new("My Program")

  ;

  ```

- <span id="command-arg"></span>`fn arg(self, a: impl Into<Arg>) -> Self` — [`Arg`](builder/arg/index.md#arg)

  Adds an [argument] to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, Arg};

  Command::new("myprog")

      // Adding a single "flag" argument with a short and help text, using Arg::new()

      .arg(

          Arg::new("debug")

             .short('d')

             .help("turns on debugging mode")

      )

      // Adding a single "option" argument with a short, a long, and help text using the less

      // verbose Arg::from()

      .arg(

          arg!(-c --config <CONFIG> "Optionally sets a config file to use")

      )

  ;

  ```

- <span id="command-arg-internal"></span>`fn arg_internal(&mut self, arg: Arg)` — [`Arg`](builder/arg/index.md#arg)

- <span id="command-args"></span>`fn args(self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](builder/arg/index.md#arg)

  Adds multiple [arguments] to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, Arg};

  Command::new("myprog")

      .args([

          arg!(-d --debug "turns on debugging info"),

          Arg::new("input").help("the input file to use")

      ])

  ;

  ```

- <span id="command-mut-arg"></span>`fn mut_arg<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate an [`Arg`](builder/arg/index.md) after it's been added to a [`Command`](builder/command/index.md).

  

  # Panics

  

  If the argument is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .arg(Arg::new("bar")

          .short('b')

          .action(ArgAction::SetTrue))

      .mut_arg("bar", |a| a.short('B'));

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "-b"]);

  

  // Since we changed `bar`'s short to "B" this should err as there

  // is no `-b` anymore, only `-B`

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "-B"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-args"></span>`fn mut_args<F>(self, f: F) -> Self`

  Allows one to mutate all [`Arg`](builder/arg/index.md)s after they've been added to a [`Command`](builder/command/index.md).

  

  This does not affect the built-in `--help` or `--version` arguments.

  

  # Examples

  

  ```ignore

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .arg(Arg::new("bar")

          .long("bar")

          .action(ArgAction::SetTrue))

      .arg(Arg::new("baz")

          .long("baz")

          .action(ArgAction::SetTrue))

      .mut_args(|a| {

          if let Some(l) = a.get_long().map(|l| format!("prefix-{l}")) {

              a.long(l)

          } else {

              a

          }

      });

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "--bar"]);

  

  // Since we changed `bar`'s long to "prefix-bar" this should err as there

  // is no `--bar` anymore, only `--prefix-bar`.

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "--prefix-bar"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-group"></span>`fn mut_group<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate an [`ArgGroup`](builder/arg_group/index.md) after it's been added to a [`Command`](builder/command/index.md).

  

  # Panics

  

  If the argument is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  

  Command::new("foo")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major "auto increase major"))

      .arg(arg!(--minor "auto increase minor"))

      .arg(arg!(--patch "auto increase patch"))

      .group(ArgGroup::new("vers")

           .args(["set-ver", "major", "minor","patch"])

           .required(true))

      .mut_group("vers", |a| a.required(false));

  ```

- <span id="command-mut-subcommand"></span>`fn mut_subcommand<F>(self, name: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate a [`Command`](builder/command/index.md) after it's been added as a subcommand.

  

  This can be useful for modifying auto-generated arguments of nested subcommands with

  `Command::mut_arg`.

  

  # Panics

  

  If the subcommand is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  

  let mut cmd = Command::new("foo")

          .subcommand(Command::new("bar"))

          .mut_subcommand("bar", |subcmd| subcmd.disable_help_flag(true));

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "bar", "--help"]);

  

  // Since we disabled the help flag on the "bar" subcommand, this should err.

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "bar"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-subcommands"></span>`fn mut_subcommands<F>(self, f: F) -> Self`

  Allows one to mutate all [`Command`](builder/command/index.md)s after they've been added as subcommands.

  

  This does not affect the built-in `--help` or `--version` arguments.

  

  # Examples

  

  ```ignore

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .subcommands([

          Command::new("fetch"),

          Command::new("push"),

      ])

      // Allow title-case subcommands

      .mut_subcommands(|sub| {

          let name = sub.get_name();

          let alias = name.chars().enumerate().map(|(i, c)| {

              if i == 0 {

                  c.to_ascii_uppercase()

              } else {

                  c

              }

          }).collect::<String>();

          sub.alias(alias)

      });

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "fetch"]);

  assert!(res.is_ok());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "Fetch"]);

  assert!(res.is_ok());

  ```

- <span id="command-group"></span>`fn group(self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](builder/arg_group/index.md#arggroup)

  Adds an [`ArgGroup`](builder/arg_group/index.md) to the application.

  

  [`ArgGroup`](builder/arg_group/index.md)s are a family of related arguments.

  By placing them in a logical group, you can build easier requirement and exclusion rules.

  

  Example use cases:

  - Make an entire [`ArgGroup`](builder/arg_group/index.md) required, meaning that one (and *only*

    one) argument from that group must be present at runtime.

  - Name an [`ArgGroup`](builder/arg_group/index.md) as a conflict to another argument.

    Meaning any of the arguments that belong to that group will cause a failure if present with

    the conflicting argument.

  - Ensure exclusion between arguments.

  - Extract a value from a group instead of determining exactly which argument was used.

  

  # Examples

  

  The following example demonstrates using an [`ArgGroup`](builder/arg_group/index.md) to ensure that one, and only one,

  of the arguments from the specified group is present at runtime.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  Command::new("cmd")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major "auto increase major"))

      .arg(arg!(--minor "auto increase minor"))

      .arg(arg!(--patch "auto increase patch"))

      .group(ArgGroup::new("vers")

           .args(["set-ver", "major", "minor","patch"])

           .required(true))

  ;

  ```

- <span id="command-groups"></span>`fn groups(self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](builder/arg_group/index.md#arggroup)

  Adds multiple [`ArgGroup`](builder/arg_group/index.md)s to the [`Command`](builder/command/index.md) at once.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  Command::new("cmd")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major         "auto increase major"))

      .arg(arg!(--minor         "auto increase minor"))

      .arg(arg!(--patch         "auto increase patch"))

      .arg(arg!(-c <FILE>       "a config file").required(false))

      .arg(arg!(-i <IFACE>      "an interface").required(false))

      .groups([

          ArgGroup::new("vers")

              .args(["set-ver", "major", "minor","patch"])

              .required(true),

          ArgGroup::new("input")

              .args(["c", "i"])

      ])

  ;

  ```

- <span id="command-subcommand"></span>`fn subcommand(self, subcmd: impl Into<Command>) -> Self` — [`Command`](builder/command/index.md#command)

  Adds a subcommand to the list of valid possibilities.

  

  Subcommands are effectively sub-[`Command`](builder/command/index.md)s, because they can contain their own arguments,

  subcommands, version, usage, etc. They also function just like [`Command`](builder/command/index.md)s, in that they get

  their own auto generated help, version, and usage.

  

  A subcommand's `Command::name` will be used for:

  - The argument the user passes in

  - Programmatically looking up the subcommand

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  Command::new("myprog")

      .subcommand(Command::new("config")

          .about("Controls configuration features")

          .arg(arg!(<config> "Required configuration file to use")))

  ;

  ```

- <span id="command-subcommand-internal"></span>`fn subcommand_internal(self, subcmd: Self) -> Self`

- <span id="command-subcommands"></span>`fn subcommands(self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`

  Adds multiple subcommands to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, };

  Command::new("myprog")

  .subcommands( [

         Command::new("config").about("Controls configuration functionality")

                                  .arg(Arg::new("config_file")),

         Command::new("debug").about("Controls debug functionality")])

  ;

  ```

- <span id="command-defer"></span>`fn defer(self, deferred: fn(Command) -> Command) -> Self` — [`Command`](builder/command/index.md#command)

  Delay initialization for parts of the `Command`

  

  This is useful for large applications to delay definitions of subcommands until they are

  being invoked.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  Command::new("myprog")

      .subcommand(Command::new("config")

          .about("Controls configuration features")

          .defer(|cmd| {

              cmd.arg(arg!(<config> "Required configuration file to use"))

          })

      )

  ;

  ```

- <span id="command-debug-assert"></span>`fn debug_assert(self)`

  Catch problems earlier in the development cycle.

  

  Most error states are handled as asserts under the assumption they are programming mistake

  and not something to handle at runtime.  Rather than relying on tests (manual or automated)

  that exhaustively test your CLI to ensure the asserts are evaluated, this will run those

  asserts in a way convenient for running as a test.

  

  **Note:** This will not help with asserts in [`ArgMatches`](parser/matches/arg_matches/index.md), those will need exhaustive

  testing of your CLI.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  fn cmd() -> Command {

      Command::new("foo")

          .arg(

              Arg::new("bar").short('b').action(ArgAction::SetTrue)

          )

  }

  

  #[test]

  fn verify_app() {

      cmd().debug_assert();

  }

  

  fn main() {

      let m = cmd().get_matches_from(vec!["foo", "-b"]);

      println!("{}", m.get_flag("bar"));

  }

  ```

- <span id="command-error"></span>`fn error(&mut self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](error/kind/index.md#errorkind), [`Error`](#error)

  Custom error message for post-parsing validation

  

  **Note:** this will ensure the `Command` has been sufficiently `built` for any

  relevant context, including usage.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, error::ErrorKind};

  let mut cmd = Command::new("myprog");

  let err = cmd.error(ErrorKind::InvalidValue, "Some failure case");

  ```

- <span id="command-get-matches"></span>`fn get_matches(self) -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, `exiting` on failure.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let matches = Command::new("myprog")

      // Args and options go here...

      .get_matches();

  ```

  

- <span id="command-get-matches-mut"></span>`fn get_matches_mut(&mut self) -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, `exiting` on failure.

  

  Like `Command::get_matches` but doesn't consume the `Command`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let mut cmd = Command::new("myprog")

      // Args and options go here...

      ;

  let matches = cmd.get_matches_mut();

  ```

  

- <span id="command-try-get-matches"></span>`fn try_get_matches(self) -> ClapResult<ArgMatches>` — [`Result`](error/index.md#result), [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](error/kind/index.md) is a

  `ErrorKind::DisplayHelp` or `ErrorKind::DisplayVersion` respectively. You must call

  `Error::exit` or perform a `std::process::exit`.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let matches = Command::new("myprog")

      // Args and options go here...

      .try_get_matches()

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

  

  

  

  

- <span id="command-get-matches-from"></span>`fn get_matches_from<I, T>(self, itr: I) -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, `exiting` on failure.

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let matches = Command::new("myprog")

      // Args and options go here...

      .get_matches_from(arg_vec);

  ```

  

  

- <span id="command-try-get-matches-from"></span>`fn try_get_matches_from<I, T>(self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](error/index.md#result), [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](error/kind/index.md) is a `ErrorKind::DisplayHelp`

  or `ErrorKind::DisplayVersion` respectively. You must call `Error::exit` or

  perform a `std::process::exit` yourself.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let matches = Command::new("myprog")

      // Args and options go here...

      .try_get_matches_from(arg_vec)

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

  

  

  

  

  

  

- <span id="command-try-get-matches-from-mut"></span>`fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](error/index.md#result), [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  Like `Command::try_get_matches_from` but doesn't consume the `Command`.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](error/kind/index.md) is a [`ErrorKind::DisplayHelp`](#errorkinddisplayhelp)

  or [`ErrorKind::DisplayVersion`](#errorkinddisplayversion) respectively. You must call `Error::exit` or

  perform a [`std::process::exit`](../libc/index.md) yourself.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let mut cmd = Command::new("myprog");

      // Args and options go here...

  let matches = cmd.try_get_matches_from_mut(arg_vec)

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

- <span id="command-print-help"></span>`fn print_help(&mut self) -> io::Result<()>`

  Prints the short help message (`-h`) to `io::stdout()`.

  

  See also `Command::print_long_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  let mut cmd = Command::new("myprog");

  cmd.print_help();

  ```

- <span id="command-print-long-help"></span>`fn print_long_help(&mut self) -> io::Result<()>`

  Prints the long help message (`--help`) to `io::stdout()`.

  

  See also `Command::print_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  let mut cmd = Command::new("myprog");

  cmd.print_long_help();

  ```

  

  

  

- <span id="command-render-help"></span>`fn render_help(&mut self) -> StyledStr` — [`StyledStr`](builder/styled_str/index.md#styledstr)

  Render the short help message (`-h`) to a [`StyledStr`](builder/styled_str/index.md)

  

  See also `Command::render_long_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  let mut out = io::stdout();

  let help = cmd.render_help();

  println!("{help}");

  ```

  

  

- <span id="command-render-long-help"></span>`fn render_long_help(&mut self) -> StyledStr` — [`StyledStr`](builder/styled_str/index.md#styledstr)

  Render the long help message (`--help`) to a [`StyledStr`](builder/styled_str/index.md).

  

  See also `Command::render_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  let mut out = io::stdout();

  let help = cmd.render_long_help();

  println!("{help}");

  ```

  

  

- <span id="command-render-version"></span>`fn render_version(&self) -> String`

  Version message rendered as if the user ran `-V`.

  

  See also `Command::render_long_version`.

  

  ### Coloring

  

  This function does not try to color the message nor it inserts any [ANSI escape codes].

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let cmd = Command::new("myprog");

  println!("{}", cmd.render_version());

  ```

  

  

  

- <span id="command-render-long-version"></span>`fn render_long_version(&self) -> String`

  Version message rendered as if the user ran `--version`.

  

  See also `Command::render_version`.

  

  ### Coloring

  

  This function does not try to color the message nor it inserts any [ANSI escape codes].

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let cmd = Command::new("myprog");

  println!("{}", cmd.render_long_version());

  ```

  

  

  

- <span id="command-render-usage"></span>`fn render_usage(&mut self) -> StyledStr` — [`StyledStr`](builder/styled_str/index.md#styledstr)

  Usage statement

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  println!("{}", cmd.render_usage());

  ```

- <span id="command-render-usage"></span>`fn render_usage_(&mut self) -> Option<StyledStr>` — [`StyledStr`](builder/styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl Any for Command`

- <span id="command-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Command`

- <span id="command-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Command`

- <span id="command-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Command`

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](builder/command/index.md#command)

##### `impl CloneToUninit for Command`

- <span id="command-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Command`

- <span id="command-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Command`

- <span id="command-default"></span>`fn default() -> Self`

##### `impl Display for Command`

- <span id="command-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Command`

- <span id="command-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Index for Command`

- <span id="command-index-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](util/id/index.md#id)

##### `impl<U> Into for Command`

- <span id="command-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Command`

- <span id="command-toowned-type-owned"></span>`type Owned = T`

- <span id="command-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="command-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Command`

- <span id="command-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Command`

- <span id="command-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="command-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Command`

- <span id="command-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="command-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`clap_builder-4.5.53/src/builder/arg.rs:60-92`](../../.source_1765521767/clap_builder-4.5.53/src/builder/arg.rs#L60-L92)*

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

- <span id="arg-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](util/id/index.md#id)

  Create a new [`Arg`](builder/arg/index.md) with a unique name.

  

  The name is used to check whether or not the argument was used at

  runtime, get values, set relationships with other args, etc..

  

  By default, an `Arg` is

  - Positional, see `Arg::short` or `Arg::long` turn it into an option

  - Accept a single value, see `Arg::action` to override this

  

  <div class="warning">

  

  **NOTE:** In the case of arguments that take values (i.e. `Arg::action(ArgAction::Set)`)

  and positional arguments (i.e. those without a preceding `-` or `--`) the name will also

  be displayed when the user prints the usage/help information of the program.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  Arg::new("config")

  ;

  ```

- <span id="arg-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](util/id/index.md#id)

  Set the identifier used for referencing this argument in the clap API.

  

  See `Arg::new` for more details.

- <span id="arg-short"></span>`fn short(self, s: impl IntoResettable<char>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable)

  Sets the short version of the argument without the preceding `-`.

  

  By default `V` and `h` are used by the auto-generated `version` and `help` arguments,

  respectively. You will need to disable the auto-generated flags

  (`disable_help_flag`,

  `disable_version_flag`) and define your own.

  

  # Examples

  

  When calling `short`, use a single valid UTF-8 character which will allow using the

  argument via a single hyphen (`-`) such as `-c`:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg,  ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("config")

          .short('c')

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "prog", "-c", "file.toml"

      ]);

  

  assert_eq!(m.get_one::<String>("config").map(String::as_str), Some("file.toml"));

  ```

  

  To use `-h` for your own flag and still have help:

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg,  ArgAction};

  let m = Command::new("prog")

      .disable_help_flag(true)

      .arg(Arg::new("host")

          .short('h')

          .long("host"))

      .arg(Arg::new("help")

          .long("help")

          .global(true)

          .action(ArgAction::Help))

      .get_matches_from(vec![

          "prog", "-h", "wikipedia.org"

      ]);

  

  assert_eq!(m.get_one::<String>("host").map(String::as_str), Some("wikipedia.org"));

  ```

- <span id="arg-long"></span>`fn long(self, l: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Str`](builder/str/index.md#str)

  Sets the long version of the argument without the preceding `--`.

  

  By default `version` and `help` are used by the auto-generated `version` and `help`

  arguments, respectively. You may use the word `version` or `help` for the long form of your

  own arguments, in which case `clap` simply will not assign those to the auto-generated

  `version` or `help` arguments.

  

  <div class="warning">

  

  **NOTE:** Any leading `-` characters will be stripped

  

  </div>

  

  # Examples

  

  To set `long` use a word containing valid UTF-8. If you supply a double leading

  `--` such as `--config` they will be stripped. Hyphens in the middle of the word, however,

  will *not* be stripped (i.e. `config-file` is allowed).

  

  Setting `long` allows using the argument via a double hyphen (`--`) such as `--config`

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("cfg")

          .long("config")

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "prog", "--config", "file.toml"

      ]);

  

  assert_eq!(m.get_one::<String>("cfg").map(String::as_str), Some("file.toml"));

  ```

- <span id="arg-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Str`](builder/str/index.md#str)

  Add an alias, which functions as a hidden long flag.

  

  This is more efficient, and easier than creating multiple hidden arguments as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

              .long("test")

              .alias("alias")

              .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "--alias", "cool"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "cool");

  ```

- <span id="arg-short-alias"></span>`fn short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable)

  Add an alias, which functions as a hidden short flag.

  

  This is more efficient, and easier than creating multiple hidden arguments as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

              .short('t')

              .short_alias('e')

              .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "-e", "cool"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "cool");

  ```

- <span id="arg-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](builder/str/index.md#str)

  Add aliases, which function as hidden long flags.

  

  This is more efficient, and easier than creating multiple hidden subcommands as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                      .long("test")

                      .aliases(["do-stuff", "do-tests", "tests"])

                      .action(ArgAction::SetTrue)

                      .help("the file to add")

                      .required(false))

              .get_matches_from(vec![

                  "prog", "--do-tests"

              ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-short-aliases"></span>`fn short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

  Add aliases, which functions as a hidden short flag.

  

  This is more efficient, and easier than creating multiple hidden subcommands as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                      .short('t')

                      .short_aliases(['e', 's'])

                      .action(ArgAction::SetTrue)

                      .help("the file to add")

                      .required(false))

              .get_matches_from(vec![

                  "prog", "-s"

              ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-visible-alias"></span>`fn visible_alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Str`](builder/str/index.md#str)

  Add an alias, which functions as a visible long flag.

  

  Like `Arg::alias`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .visible_alias("something-awesome")

                  .long("test")

                  .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "--something-awesome", "coffee"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "coffee");

  ```

- <span id="arg-visible-short-alias"></span>`fn visible_short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable)

  Add an alias, which functions as a visible short flag.

  

  Like `Arg::short_alias`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .visible_short_alias('t')

                  .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "-t", "coffee"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "coffee");

  ```

- <span id="arg-visible-aliases"></span>`fn visible_aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](builder/str/index.md#str)

  Add aliases, which function as visible long flags.

  

  Like `Arg::aliases`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .action(ArgAction::SetTrue)

                  .visible_aliases(["something", "awesome", "cool"]))

         .get_matches_from(vec![

              "prog", "--awesome"

          ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-visible-short-aliases"></span>`fn visible_short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

  Add aliases, which function as visible short flags.

  

  Like `Arg::short_aliases`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .action(ArgAction::SetTrue)

                  .visible_short_aliases(['t', 'e']))

         .get_matches_from(vec![

              "prog", "-t"

          ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-index"></span>`fn index(self, idx: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable)

  Specifies the index of a positional argument **starting at** 1.

  

  <div class="warning">

  

  **NOTE:** The index refers to position according to **other positional argument**. It does

  not define position in the argument list as a whole.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** You can optionally leave off the `index` method, and the index will be

  assigned in order of evaluation. Utilizing the `index` method allows for setting

  indexes out of order

  

  </div>

  

  <div class="warning">

  

  **NOTE:** This is only meant to be used for positional arguments and shouldn't to be used

  with `Arg::short` or `Arg::long`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** When utilized with `Arg::num_args(1..)`, only the **last** positional argument

  may be defined as having a variable number of arguments (i.e. with the highest index)

  

  </div>

  

  # Panics

  

  [`Command`](builder/command/index.md) will [`panic!`](../miette/panic/index.md) if indexes are skipped (such as defining `index(1)` and `index(3)`

  but not `index(2)`, or a positional argument is defined as multiple and is not the highest

  index (debug builds)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  Arg::new("config")

      .index(1)

  ;

  ```

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("mode")

          .index(1))

      .arg(Arg::new("debug")

          .long("debug")

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "prog", "--debug", "fast"

      ]);

  

  assert!(m.contains_id("mode"));

  assert_eq!(m.get_one::<String>("mode").unwrap(), "fast"); // notice index(1) means "first positional"

                                                            // *not* first argument

  ```

  

  

  

- <span id="arg-trailing-var-arg"></span>`fn trailing_var_arg(self, yes: bool) -> Self`

  This is a "var arg" and everything that follows should be captured by it, as if the user had

  used a `--`.

  

  <div class="warning">

  

  **NOTE:** To start the trailing "var arg" on unknown flags (and not just a positional

  value), set `allow_hyphen_values`.  Either way, users still

  have the option to explicitly escape ambiguous arguments with `--`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** `Arg::value_delimiter` still applies if set.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** Setting this requires `Arg::num_args(..)`.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  let m = Command::new("myprog")

      .arg(arg!(<cmd> ... "commands to run").trailing_var_arg(true))

      .get_matches_from(vec!["myprog", "arg1", "-r", "val1"]);

  

  let trail: Vec<_> = m.get_many::<String>("cmd").unwrap().collect();

  assert_eq!(trail, ["arg1", "-r", "val1"]);

  ```

- <span id="arg-last"></span>`fn last(self, yes: bool) -> Self`

  This arg is the last, or final, positional argument (i.e. has the highest

  index) and is *only* able to be accessed via the `--` syntax (i.e. `$ prog args --

  last_arg`).

  

  Even, if no other arguments are left to parse, if the user omits the `--` syntax

  they will receive an [`UnknownArgument`](#unknownargument) error. Setting an argument to `.last(true)` also

  allows one to access this arg early using the `--` syntax. Accessing an arg early, even with

  the `--` syntax is otherwise not possible.

  

  <div class="warning">

  

  **NOTE:** This will change the usage string to look like `$ prog [OPTIONS] [-- <ARG>]` if

  `ARG` is marked as `.last(true)`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** This setting will imply [`crate::Command::dont_collapse_args_in_usage`](builder/command/index.md#dont-collapse-args-in-usage) because failing

  to set this can make the usage string very confusing.

  

  </div>

  

  <div class="warning">

  

  **NOTE**: This setting only applies to positional arguments, and has no effect on OPTIONS

  

  </div>

  

  <div class="warning">

  

  **NOTE:** Setting this requires `taking values`

  

  </div>

  

  <div class="warning">

  

  **WARNING:** Using this setting *and* having child subcommands is not

  recommended with the exception of *also* using

  [`crate::Command::args_conflicts_with_subcommands`](builder/command/index.md#args-conflicts-with-subcommands)

  (or [`crate::Command::subcommand_negates_reqs`](builder/command/index.md#subcommand-negates-reqs) if the argument marked `Last` is also

  marked `Arg::required`)

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Arg, ArgAction};

  Arg::new("args")

      .action(ArgAction::Set)

      .last(true)

  ;

  ```

  

  Setting `last` ensures the arg has the highest [`index`](../gimli/read/index/index.md) of all positional args

  and requires that the `--` syntax be used to access it early.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("first"))

      .arg(Arg::new("second"))

      .arg(Arg::new("third")

          .action(ArgAction::Set)

          .last(true))

      .try_get_matches_from(vec![

          "prog", "one", "--", "three"

      ]);

  

  assert!(res.is_ok());

  let m = res.unwrap();

  assert_eq!(m.get_one::<String>("third").unwrap(), "three");

  assert_eq!(m.get_one::<String>("second"), None);

  ```

  

  Even if the positional argument marked `Last` is the only argument left to parse,

  failing to use the `--` syntax results in an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("first"))

      .arg(Arg::new("second"))

      .arg(Arg::new("third")

          .action(ArgAction::Set)

          .last(true))

      .try_get_matches_from(vec![

          "prog", "one", "two", "three"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);

  ```

  

- <span id="arg-required"></span>`fn required(self, yes: bool) -> Self`

  Specifies that the argument must be present.

  

  Required by default means it is required, when no other conflicting rules or overrides have

  been evaluated. Conflicting rules take precedence over being required.

  

  **Pro tip:** Flags (i.e. not positional, or arguments that take values) shouldn't be

  required by default. This is because if a flag were to be required, it should simply be

  implied. No additional information is required from user. Flags by their very nature are

  simply boolean on/off switches. The only time a user *should* be required to use a flag

  is if the operation is destructive in nature, and the user is essentially proving to you,

  "Yes, I know what I'm doing."

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .required(true)

  ;

  ```

  

  Setting required requires that the argument be used at runtime.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .required(true)

          .action(ArgAction::Set)

          .long("config"))

      .try_get_matches_from(vec![

          "prog", "--config", "file.conf",

      ]);

  

  assert!(res.is_ok());

  ```

  

  Setting required and then *not* supplying that argument at runtime is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .required(true)

          .action(ArgAction::Set)

          .long("config"))

      .try_get_matches_from(vec![

          "prog"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

  ```

- <span id="arg-requires"></span>`fn requires(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Id`](util/id/index.md#id)

  Sets an argument that is required when this one is present

  

  i.e. when using this argument, the following argument *must* be present.

  

  <div class="warning">

  

  **NOTE:** [Conflicting] rules and [override] rules take precedence over being required

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .requires("input")

  ;

  ```

  

  Setting `Arg::requires(name)` requires that the argument be used at runtime if the

  defining argument is used. If the defining argument isn't used, the other argument isn't

  required

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .action(ArgAction::Set)

          .requires("input")

          .long("config"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog"

      ]);

  

  assert!(res.is_ok()); // We didn't use cfg, so input wasn't required

  ```

  

  Setting `Arg::requires(name)` and *not* supplying that argument is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .action(ArgAction::Set)

          .requires("input")

          .long("config"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog", "--config", "file.conf"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

  ```

  

  

- <span id="arg-exclusive"></span>`fn exclusive(self, yes: bool) -> Self`

  This argument must be passed alone; it conflicts with all other arguments.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .exclusive(true)

  ;

  ```

  

  Setting an exclusive argument and having any other arguments present at runtime

  is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("exclusive")

          .action(ArgAction::Set)

          .exclusive(true)

          .long("exclusive"))

      .arg(Arg::new("debug")

          .long("debug"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog", "--exclusive", "file.conf", "file.txt"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arg-global"></span>`fn global(self, yes: bool) -> Self`

  Specifies that an argument can be matched to all child [`Subcommand`](derive/index.md)s.

  

  <div class="warning">

  

  **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands), however

  their values once a user uses them will be propagated back up to parents. In effect, this

  means one should *define* all global arguments at the top level, however it doesn't matter

  where the user *uses* the global argument.

  

  </div>

  

  # Examples

  

  Assume an application with two subcommands, and you'd like to define a

  `--verbose` flag that can be called on any of the subcommands and parent, but you don't

  want to clutter the source with three duplicate [`Arg`](builder/arg/index.md) definitions.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("verb")

          .long("verbose")

          .short('v')

          .action(ArgAction::SetTrue)

          .global(true))

      .subcommand(Command::new("test"))

      .subcommand(Command::new("do-stuff"))

      .get_matches_from(vec![

          "prog", "do-stuff", "--verbose"

      ]);

  

  assert_eq!(m.subcommand_name(), Some("do-stuff"));

  let sub_m = m.subcommand_matches("do-stuff").unwrap();

  assert_eq!(sub_m.get_flag("verb"), true);

  ```

- <span id="arg-is-set"></span>`fn is_set(&self, s: ArgSettings) -> bool` — [`ArgSettings`](builder/arg_settings/index.md#argsettings)

- <span id="arg-setting"></span>`fn setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](builder/arg_settings/index.md#argsettings)

- <span id="arg-unset-setting"></span>`fn unset_setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](builder/arg_settings/index.md#argsettings)

#### Trait Implementations

##### `impl Any for Arg`

- <span id="arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Arg`

- <span id="arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Arg`

- <span id="arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Arg`

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](builder/arg/index.md#arg)

##### `impl CloneToUninit for Arg`

- <span id="arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Arg`

- <span id="arg-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](builder/arg/index.md#arg)

##### `impl Display for Arg`

- <span id="arg-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl<T> From for Arg`

- <span id="arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Arg`

- <span id="arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Arg`

- <span id="arg-ord-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](builder/arg/index.md#arg)

##### `impl PartialEq for Arg`

- <span id="arg-partialeq-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](builder/arg/index.md#arg)

##### `impl PartialOrd for Arg`

- <span id="arg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl ToOwned for Arg`

- <span id="arg-toowned-type-owned"></span>`type Owned = T`

- <span id="arg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Arg`

- <span id="arg-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Arg`

- <span id="arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Arg`

- <span id="arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`clap_builder-4.5.53/src/builder/arg_group.rs:68-75`](../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_group.rs#L68-L75)*

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

- <span id="arggroup-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](util/id/index.md#id)

  Create a `ArgGroup` using a unique name.

  

  The name will be used to get values from the group or refer to the group inside of conflict

  and requirement rules.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::new("config")

  ;

  ```

- <span id="arggroup-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](util/id/index.md#id)

  Sets the group name.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::default().id("config")

  ;

  ```

- <span id="arggroup-arg"></span>`fn arg(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Id`](util/id/index.md#id)

  Adds an [argument] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .arg("flag")

          .arg("color"))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-args"></span>`fn args(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](util/id/index.md#id)

  Adds multiple [arguments] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-get-args"></span>`fn get_args(&self) -> impl Iterator<Item = &Id>` — [`Id`](util/id/index.md#id)

  Getters for all args. It will return a vector of `Id`

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let args: Vec<&str> = vec!["a1".into(), "a4".into()];

  let grp = ArgGroup::new("program").args(&args);

  

  for (pos, arg) in grp.get_args().enumerate() {

      assert_eq!(*arg, args[pos]);

  }

  ```

- <span id="arggroup-multiple"></span>`fn multiple(self, yes: bool) -> Self`

  Allows more than one of the [`Arg`](#arg)s in this group to be used. (Default: `false`)

  

  # Examples

  

  Notice in this example we use *both* the `-f` and `-c` flags which are both part of the

  group

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .multiple(true))

      .get_matches_from(vec!["myprog", "-f", "-c"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  ```

  In this next example, we show the default behavior (i.e. `multiple(false)`) which will throw

  an error if more than one of the args in the group was used.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .try_get_matches_from(vec!["myprog", "-f", "-c"]);

  // Because we used both args in the group it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-is-multiple"></span>`fn is_multiple(&mut self) -> bool`

  Return true if the group allows more than one of the arguments

  in this group to be used. (Default: `false`)

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let mut group = ArgGroup::new("myprog")

      .args(["f", "c"])

      .multiple(true);

  

  assert!(group.is_multiple());

  ```

- <span id="arggroup-required"></span>`fn required(self, yes: bool) -> Self`

  Require an argument from the group to be present when parsing.

  

  This is unless conflicting with another argument.  A required group will be displayed in

  the usage string of the application in the format `<arg|arg2|arg3>`.

  

  <div class="warning">

  

  **NOTE:** This setting only applies to the current [`Command`](builder/command/index.md) / [`Subcommand`](derive/index.md)s, and not

  globally.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** By default, `ArgGroup::multiple` is set to `false` which when combined with

  `ArgGroup::required(true)` states, "One and *only one* arg must be used from this group.

  Use of more than one arg is an error." Vice setting `ArgGroup::multiple(true)` which

  states, '*At least* one arg from this group must be used. Using multiple is OK."

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .required(true))

      .try_get_matches_from(vec!["myprog"]);

  // Because we didn't use any of the args in the group, it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

  

- <span id="arggroup-requires"></span>`fn requires(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Id`](util/id/index.md#id)

  Specify an argument or group that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when any one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires("debug"))

      .try_get_matches_from(vec!["myprog", "-c"]);

  // because we used an arg from the group, and the group requires "-d" to be used, it's an

  // error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-requires-all"></span>`fn requires_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](util/id/index.md#id)

  Specify arguments or groups that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group requires "-d" and "-v" to be used,

  // yet we only used "-d" it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-conflicts-with"></span>`fn conflicts_with(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](builder/resettable/index.md#intoresettable), [`Id`](util/id/index.md#id)

  Specify an argument or group that must **not** be present when this group is.

  

  Exclusion (aka conflict) rules function just like [argument exclusion rules], you can name

  other arguments or groups that must *not* be present when one of the arguments from this

  group are used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with("debug"))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group conflicts with "-d", it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-conflicts-with-all"></span>`fn conflicts_with_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](util/id/index.md#id)

  Specify arguments or groups that must **not** be present when this group is.

  

  Exclusion rules function just like [argument exclusion rules], you can name other arguments

  or groups that must *not* be present when one of the arguments from this group are used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-v"]);

  // because we used an arg from the group, and the group conflicts with either "-v" or "-d"

  // it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

#### Trait Implementations

##### `impl Any for ArgGroup`

- <span id="arggroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgGroup`

- <span id="arggroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgGroup`

- <span id="arggroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](builder/arg_group/index.md#arggroup)

##### `impl CloneToUninit for ArgGroup`

- <span id="arggroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgGroup`

- <span id="arggroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](builder/arg_group/index.md#arggroup)

##### `impl Eq for ArgGroup`

##### `impl<T> From for ArgGroup`

- <span id="arggroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgGroup`

- <span id="arggroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-partialeq-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](builder/arg_group/index.md#arggroup)

##### `impl StructuralPartialEq for ArgGroup`

##### `impl ToOwned for ArgGroup`

- <span id="arggroup-toowned-type-owned"></span>`type Owned = T`

- <span id="arggroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arggroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgGroup`

- <span id="arggroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arggroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgGroup`

- <span id="arggroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arggroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArgMatches`

```rust
struct ArgMatches {
    valid_args: Vec<crate::util::Id>,
    valid_subcommands: Vec<crate::builder::Str>,
    args: self::flat_map::FlatMap<crate::util::Id, matched_arg::MatchedArg>,
    subcommand: Option<Box<SubCommand>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:67-74`](../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L67-L74)*

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

- <span id="argmatches-get-one"></span>`fn get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<&T>`

  Gets the value of a specific option or positional argument.

  

  i.e. an argument that `takes an additional value` at runtime.

  

  Returns an error if the wrong type was used.

  

  Returns `None` if the option wasn't present.

  

  <div class="warning">

  

  *NOTE:* This will always return `Some(value)` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_one`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("port")

          .value_parser(value_parser!(usize))

          .action(ArgAction::Set)

          .required(true))

      .get_matches_from(vec!["myapp", "2020"]);

  

  let port: usize = *m

      .get_one("port")

      .expect("`port`is required");

  assert_eq!(port, 2020);

  ```

  

- <span id="argmatches-get-count"></span>`fn get_count(&self, id: &str) -> u8`

  Gets the value of a specific `ArgAction::Count` flag

  

  # Panic

  

  If the argument's action is not `ArgAction::Count`

  

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

  assert_eq!(

      matches.get_count("flag"),

      2

  );

  ```

- <span id="argmatches-get-flag"></span>`fn get_flag(&self, id: &str) -> bool`

  Gets the value of a specific `ArgAction::SetTrue` or `ArgAction::SetFalse` flag

  

  # Panic

  

  If the argument's action is not `ArgAction::SetTrue` or `ArgAction::SetFalse`

  

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

  ```

- <span id="argmatches-get-many"></span>`fn get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<ValuesRef<'_, T>>` — [`ValuesRef`](parser/matches/arg_matches/index.md#valuesref)

  Iterate over values of a specific option or positional argument.

  

  i.e. an argument that takes multiple values at runtime.

  

  Returns an error if the wrong type was used.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_many`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("ports")

          .action(ArgAction::Append)

          .value_parser(value_parser!(usize))

          .short('p')

          .required(true))

      .get_matches_from(vec![

          "myprog", "-p", "22", "-p", "80", "-p", "2020"

      ]);

  let vals: Vec<usize> = m.get_many("ports")

      .expect("`port`is required")

      .copied()

      .collect();

  assert_eq!(vals, [22, 80, 2020]);

  ```

- <span id="argmatches-get-occurrences"></span>`fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<OccurrencesRef<'_, T>>` — [`OccurrencesRef`](parser/matches/arg_matches/index.md#occurrencesref)

  Iterate over the values passed to each occurrence of an option.

  

  Each item is itself an iterator containing the arguments passed to a single occurrence

  of the option.

  

  If the option doesn't support multiple occurrences, or there was only a single occurrence,

  the iterator will only contain a single item.

  

  Returns `None` if the option wasn't present.

  

  # Panics

  

  If the argument definition and access mismatch (debug builds). To handle this case programmatically, see

  `ArgMatches::try_get_occurrences`.

  

  # Examples

  ```rust

  use clap_builder as clap;

  use clap::{Command,Arg, ArgAction, value_parser};

  let m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(String)))

      .get_matches_from(vec![

          "myprog", "-x", "a", "b", "-x", "c", "d"]);

  let vals: Vec<Vec<&String>> = m.get_occurrences("x").unwrap().map(Iterator::collect).collect();

  assert_eq!(vals, [["a", "b"], ["c", "d"]]);

  ```

- <span id="argmatches-get-raw"></span>`fn get_raw(&self, id: &str) -> Option<RawValues<'_>>` — [`RawValues`](parser/matches/arg_matches/index.md#rawvalues)

  Iterate over the original argument values.

  

  An `OsStr` on Unix-like systems is any series of bytes, regardless of whether or not they

  contain valid UTF-8. Since [`String`](#string)s in Rust are guaranteed to be valid UTF-8, a valid

  filename on a Unix system as an argument value may contain invalid UTF-8.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_raw`.

  

  # Examples

  

  ```rust

  #[cfg(unix)] {

  use clap_builder as clap;

  use clap::{Command, arg, value_parser};

  use std::ffi::{OsStr,OsString};

  use std::os::unix::ffi::{OsStrExt,OsStringExt};

  use std::path::PathBuf;

  

  let m = Command::new("utf8")

      .arg(arg!(<arg> ... "some arg").value_parser(value_parser!(PathBuf)))

      .get_matches_from(vec![OsString::from("myprog"),

                                  // "Hi"

                                  OsString::from_vec(vec![b'H', b'i']),

                                  // "{0xe9}!"

                                  OsString::from_vec(vec![0xe9, b'!'])]);

  

  let mut itr = m.get_raw("arg")

      .expect("`port`is required")

      .into_iter();

  assert_eq!(itr.next(), Some(OsStr::new("Hi")));

  assert_eq!(itr.next(), Some(OsStr::from_bytes(&[0xe9, b'!'])));

  assert_eq!(itr.next(), None);

  }

  ```

  

  

- <span id="argmatches-get-raw-occurrences"></span>`fn get_raw_occurrences(&self, id: &str) -> Option<RawOccurrences<'_>>` — [`RawOccurrences`](parser/matches/arg_matches/index.md#rawoccurrences)

  Iterate over the original values for each occurrence of an option.

  

  Similar to `ArgMatches::get_occurrences` but returns raw values.

  

  An `OsStr` on Unix-like systems is any series of bytes, regardless of whether or not they

  contain valid UTF-8. Since [`String`](#string)s in Rust are guaranteed to be valid UTF-8, a valid

  filename on a Unix system as an argument value may contain invalid UTF-8.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_raw_occurrences`.

  

  # Examples

  

  ```rust

  #[cfg(unix)] {

  use clap_builder as clap;

  use clap::{Command, arg, value_parser, ArgAction, Arg};

  use std::ffi::{OsStr,OsString};

  use std::os::unix::ffi::{OsStrExt,OsStringExt};

  use std::path::PathBuf;

  

  let m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(PathBuf)))

      .get_matches_from(vec![OsString::from("myprog"),

                              OsString::from("-x"),

                              OsString::from("a"), OsString::from("b"),

                              OsString::from("-x"),

                              OsString::from("c"),

                              // "{0xe9}!"

                              OsString::from_vec(vec![0xe9, b'!'])]);

  let mut itr = m.get_raw_occurrences("x")

      .expect("`-x`is required")

      .map(Iterator::collect::<Vec<_>>);

  assert_eq!(itr.next(), Some(vec![OsStr::new("a"), OsStr::new("b")]));

  assert_eq!(itr.next(), Some(vec![OsStr::new("c"), OsStr::from_bytes(&[0xe9, b'!'])]));

  assert_eq!(itr.next(), None);

  }

  ```

  

  

- <span id="argmatches-remove-one"></span>`fn remove_one<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<T>`

  Returns the value of a specific option or positional argument.

  

  i.e. an argument that `takes an additional value` at runtime.

  

  Returns an error if the wrong type was used.  No item will have been removed.

  

  Returns `None` if the option wasn't present.

  

  <div class="warning">

  

  *NOTE:* This will always return `Some(value)` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_one`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("file")

          .required(true)

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "myprog", "file.txt",

      ]);

  let vals: String = m.remove_one("file")

      .expect("`file`is required");

  assert_eq!(vals, "file.txt");

  ```

  

- <span id="argmatches-remove-many"></span>`fn remove_many<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Values<T>>` — [`Values`](parser/matches/arg_matches/index.md#values)

  Return values of a specific option or positional argument.

  

  i.e. an argument that takes multiple values at runtime.

  

  Returns an error if the wrong type was used.  No item will have been removed.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_many`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("file")

          .action(ArgAction::Append)

          .num_args(1..)

          .required(true))

      .get_matches_from(vec![

          "myprog", "file1.txt", "file2.txt", "file3.txt", "file4.txt",

      ]);

  let vals: Vec<String> = m.remove_many("file")

      .expect("`file`is required")

      .collect();

  assert_eq!(vals, ["file1.txt", "file2.txt", "file3.txt", "file4.txt"]);

  ```

- <span id="argmatches-remove-occurrences"></span>`fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Occurrences<T>>` — [`Occurrences`](parser/matches/arg_matches/index.md#occurrences)

  Return values for each occurrence of an option.

  

  Each item is itself an iterator containing the arguments passed to a single occurrence of

  the option.

  

  If the option doesn't support multiple occurrences, or there was only a single occurrence,

  the iterator will only contain a single item.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_occurrences`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(String)))

      .get_matches_from(vec![

          "myprog", "-x", "a", "b", "-x", "c", "d"]);

  let vals: Vec<Vec<String>> = m.remove_occurrences("x").unwrap().map(Iterator::collect).collect();

  assert_eq!(vals, [["a", "b"], ["c", "d"]]);

  ```

- <span id="argmatches-contains-id"></span>`fn contains_id(&self, id: &str) -> bool`

  Check if values are present for the argument or group id

  

  <div class="warning">

  

  *NOTE:* This will always return `true` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group name (debug builds).  To handle this case programmatically, see

  `ArgMatches::try_contains_id`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "myprog", "-d"

      ]);

  

  assert!(m.contains_id("debug"));

  ```

- <span id="argmatches-ids"></span>`fn ids(&self) -> IdsRef<'_>` — [`IdsRef`](parser/matches/arg_matches/index.md#idsref)

  Iterate over `Arg` and `ArgGroup` [`Id`](util/id/index.md)s via `ArgMatches::ids`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, value_parser};

  

  let m = Command::new("myprog")

      .arg(arg!(--color <when>)

          .value_parser(["auto", "always", "never"]))

      .arg(arg!(--config <path>)

          .value_parser(value_parser!(std::path::PathBuf)))

      .get_matches_from(["myprog", "--config=config.toml", "--color=auto"]);

  assert_eq!(m.ids().len(), 2);

  assert_eq!(

      m.ids()

          .map(|id| id.as_str())

          .collect::<Vec<_>>(),

      ["config", "color"]

  );

  ```

- <span id="argmatches-args-present"></span>`fn args_present(&self) -> bool`

  Check if any `Arg`s were present on the command line

  

  See `ArgMatches::subcommand_name()` or `ArgMatches::subcommand()` to check if a

  subcommand was present on the command line.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let mut cmd = Command::new("myapp")

      .arg(Arg::new("output")

          .action(ArgAction::Set));

  

  let m = cmd

      .try_get_matches_from_mut(vec!["myapp", "something"])

      .unwrap();

  assert!(m.args_present());

  

  let m = cmd

      .try_get_matches_from_mut(vec!["myapp"])

      .unwrap();

  assert!(! m.args_present());

- <span id="argmatches-value-source"></span>`fn value_source(&self, id: &str) -> Option<ValueSource>` — [`ValueSource`](parser/matches/value_source/index.md#valuesource)

  Report where argument value came from

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  use clap::parser::ValueSource;

  let m = Command::new("myprog")

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "myprog", "-d"

      ]);

  

  assert_eq!(m.value_source("debug"), Some(ValueSource::CommandLine));

  ```

- <span id="argmatches-index-of"></span>`fn index_of(&self, id: &str) -> Option<usize>`

  The first index of that an argument showed up.

  

  Indices are similar to argv indices, but are not exactly 1:1.

  

  For flags (i.e. those arguments which don't have an associated value), indices refer

  to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices

  refer to the *values* `-o val` would therefore not represent two distinct indices, only the

  index for `val` would be recorded. This is by design.

  

  Besides the flag/option discrepancy, the primary difference between an argv index and clap

  index, is that clap continues counting once all arguments have properly separated, whereas

  an argv index does not.

  

  The examples should clear this up.

  

  <div class="warning">

  

  *NOTE:* If an argument is allowed multiple times, this method will only give the *first*

  index.  See `ArgMatches::indices_of`.

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  The argv indices are listed in the comments below. See how they correspond to the clap

  indices. Note that if it's not listed in a clap index, this is because it's not saved in

  in an `ArgMatches` struct for querying.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-f", "-o", "val"]);

             // ARGV indices: ^0       ^1    ^2    ^3

             // clap indices:          ^1          ^3

  

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("option"), Some(3));

  ```

  

  Now notice, if we use one of the other styles of options:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-f", "-o=val"]);

             // ARGV indices: ^0       ^1    ^2

             // clap indices:          ^1       ^3

  

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("option"), Some(3));

  ```

  

  Things become much more complicated, or clear if we look at a more complex combination of

  flags. Let's also throw in the final option style for good measure.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag2")

          .short('F')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag3")

          .short('z')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-fzF", "-oval"]);

             // ARGV indices: ^0      ^1       ^2

             // clap indices:         ^1,2,3    ^5

             //

             // clap sees the above as 'myapp -f -z -F -o val'

             //                         ^0    ^1 ^2 ^3 ^4 ^5

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("flag2"), Some(3));

  assert_eq!(m.index_of("flag3"), Some(2));

  assert_eq!(m.index_of("option"), Some(5));

  ```

  

  One final combination of flags/options to see how they combine:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag2")

          .short('F')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag3")

          .short('z')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-fzFoval"]);

             // ARGV indices: ^0       ^1

             // clap indices:          ^1,2,3^5

             //

             // clap sees the above as 'myapp -f -z -F -o val'

             //                         ^0    ^1 ^2 ^3 ^4 ^5

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("flag2"), Some(3));

  assert_eq!(m.index_of("flag3"), Some(2));

  assert_eq!(m.index_of("option"), Some(5));

  ```

  

  The last part to mention is when values are sent in multiple groups with a [delimiter].

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .value_delimiter(',')

          .num_args(1..))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2   ^3   ^4

             //

             // clap sees the above as 'myapp -o val1 val2 val3'

             //                         ^0    ^1 ^2   ^3   ^4

  assert_eq!(m.index_of("option"), Some(2));

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);

  ```

- <span id="argmatches-indices-of"></span>`fn indices_of(&self, id: &str) -> Option<Indices<'_>>` — [`Indices`](parser/matches/arg_matches/index.md#indices)

  All indices an argument appeared at when parsing.

  

  Indices are similar to argv indices, but are not exactly 1:1.

  

  For flags (i.e. those arguments which don't have an associated value), indices refer

  to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices

  refer to the *values* `-o val` would therefore not represent two distinct indices, only the

  index for `val` would be recorded. This is by design.

  

  <div class="warning">

  

  *NOTE:* For more information about how clap indices compared to argv indices, see

  `ArgMatches::index_of`

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .value_delimiter(','))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2   ^3   ^4

             //

             // clap sees the above as 'myapp -o val1 val2 val3'

             //                         ^0    ^1 ^2   ^3   ^4

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);

  ```

  

  Another quick example is when flags and options are used together

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set)

          .action(ArgAction::Append))

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::Count))

      .get_matches_from(vec!["myapp", "-o", "val1", "-f", "-o", "val2", "-f"]);

             // ARGV indices: ^0       ^1    ^2      ^3    ^4    ^5      ^6

             // clap indices:                ^2      ^3          ^5      ^6

  

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 5]);

  assert_eq!(m.indices_of("flag").unwrap().collect::<Vec<_>>(), &[6]);

  ```

  

  One final example, which is an odd case; if we *don't* use  value delimiter as we did with

  the first example above instead of `val1`, `val2` and `val3` all being distinc values, they

  would all be a single value of `val1,val2,val3`, in which case they'd only receive a single

  index.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set)

          .num_args(1..))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2

             //

             // clap sees the above as 'myapp -o "val1,val2,val3"'

             //                         ^0    ^1  ^2

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2]);

  ```

#### Trait Implementations

##### `impl Any for ArgMatches`

- <span id="argmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgMatches`

- <span id="argmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgMatches`

- <span id="argmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

##### `impl CloneToUninit for ArgMatches`

- <span id="argmatches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgMatches`

- <span id="argmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

##### `impl Eq for ArgMatches`

##### `impl<T> From for ArgMatches`

- <span id="argmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgMatches`

- <span id="argmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-partialeq-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](parser/matches/arg_matches/index.md#argmatches)

##### `impl StructuralPartialEq for ArgMatches`

##### `impl ToOwned for ArgMatches`

- <span id="argmatches-toowned-type-owned"></span>`type Owned = T`

- <span id="argmatches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argmatches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgMatches`

- <span id="argmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgMatches`

- <span id="argmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Id`

```rust
struct Id(crate::builder::Str);
```

*Defined in [`clap_builder-4.5.53/src/util/id.rs:11`](../../.source_1765521767/clap_builder-4.5.53/src/util/id.rs#L11)*

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- <span id="id-const-help"></span>`const HELP: &'static str`

- <span id="id-const-version"></span>`const VERSION: &'static str`

- <span id="id-const-external"></span>`const EXTERNAL: &'static str`

- <span id="id-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="id-as-str"></span>`fn as_str(&self) -> &str`

  Get the raw string of the `Id`

- <span id="id-as-internal-str"></span>`fn as_internal_str(&self) -> &Str` — [`Str`](builder/str/index.md#str)

#### Trait Implementations

##### `impl Any for Id`

- <span id="id-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Id`

- <span id="id-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for Id`

- <span id="id-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Id`

- <span id="id-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](util/id/index.md#id)

##### `impl CloneToUninit for Id`

- <span id="id-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Id`

- <span id="id-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- <span id="id-default"></span>`fn default() -> Id` — [`Id`](util/id/index.md#id)

##### `impl Display for Id`

- <span id="id-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl<T> From for Id`

- <span id="id-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Command`

- <span id="command-index-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](util/id/index.md#id)

##### `impl<U> Into for Id`

- <span id="id-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Str`

- <span id="str-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<Id>` — [`Resettable`](builder/resettable/index.md#resettable), [`Id`](util/id/index.md#id)

##### `impl Ord for Id`

- <span id="id-ord-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](util/id/index.md#id)

##### `impl PartialEq for Id`

- <span id="id-partialeq-eq"></span>`fn eq(&self, other: &Id) -> bool` — [`Id`](util/id/index.md#id)

##### `impl PartialOrd for Id`

- <span id="id-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](util/id/index.md#id)

##### `impl StructuralPartialEq for Id`

##### `impl ToOwned for Id`

- <span id="id-toowned-type-owned"></span>`type Owned = T`

- <span id="id-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="id-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Id`

- <span id="id-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Id`

- <span id="id-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="id-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Id`

- <span id="id-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="id-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`clap_builder-4.5.53/src/builder/action.rs:34-353`](../../.source_1765521767/clap_builder-4.5.53/src/builder/action.rs#L34-L353)*

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

  Returns whether this action accepts values on the command-line

  

  `default_values` and `env` may still be

  processed.

- <span id="argaction-max-num-args"></span>`fn max_num_args(&self) -> ValueRange` — [`ValueRange`](builder/range/index.md#valuerange)

- <span id="argaction-default-num-args"></span>`fn default_num_args(&self) -> ValueRange` — [`ValueRange`](builder/range/index.md#valuerange)

- <span id="argaction-default-value"></span>`fn default_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-missing-value"></span>`fn default_missing_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-value-parser"></span>`fn default_value_parser(&self) -> Option<super::ValueParser>` — [`ValueParser`](builder/value_parser/index.md#valueparser)

- <span id="argaction-value-type-id"></span>`fn value_type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](util/any_value/index.md#anyvalueid)

#### Trait Implementations

##### `impl Any for ArgAction`

- <span id="argaction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgAction`

- <span id="argaction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgAction`

- <span id="argaction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgAction`

- <span id="argaction-clone"></span>`fn clone(&self) -> ArgAction` — [`ArgAction`](builder/action/index.md#argaction)

##### `impl CloneToUninit for ArgAction`

- <span id="argaction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgAction`

- <span id="argaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArgAction`

- <span id="argaction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgAction`

- <span id="argaction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Option<crate::builder::ArgAction>`

- <span id="option-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ArgAction>` — [`Resettable`](builder/resettable/index.md#resettable), [`ArgAction`](builder/action/index.md#argaction)

##### `impl ToOwned for ArgAction`

- <span id="argaction-toowned-type-owned"></span>`type Owned = T`

- <span id="argaction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argaction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgAction`

- <span id="argaction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argaction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgAction`

- <span id="argaction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argaction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`clap_builder-4.5.53/src/builder/value_hint.rs:29-68`](../../.source_1765521767/clap_builder-4.5.53/src/builder/value_hint.rs#L29-L68)*

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

##### `impl Any for ValueHint`

- <span id="valuehint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueHint`

- <span id="valuehint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueHint`

- <span id="valuehint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueHint`

- <span id="valuehint-clone"></span>`fn clone(&self) -> ValueHint` — [`ValueHint`](builder/value_hint/index.md#valuehint)

##### `impl CloneToUninit for ValueHint`

- <span id="valuehint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- <span id="valuehint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ValueHint`

- <span id="valuehint-default"></span>`fn default() -> ValueHint` — [`ValueHint`](builder/value_hint/index.md#valuehint)

##### `impl Eq for ValueHint`

##### `impl<T> From for ValueHint`

- <span id="valuehint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for ValueHint`

- <span id="valuehint-fromstr-type-err"></span>`type Err = String`

- <span id="valuehint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- <span id="valuehint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ValueHint`

- <span id="valuehint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Option<crate::builder::ValueHint>`

- <span id="option-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueHint>` — [`Resettable`](builder/resettable/index.md#resettable), [`ValueHint`](builder/value_hint/index.md#valuehint)

##### `impl PartialEq for ValueHint`

- <span id="valuehint-partialeq-eq"></span>`fn eq(&self, other: &ValueHint) -> bool` — [`ValueHint`](builder/value_hint/index.md#valuehint)

##### `impl StructuralPartialEq for ValueHint`

##### `impl ToOwned for ValueHint`

- <span id="valuehint-toowned-type-owned"></span>`type Owned = T`

- <span id="valuehint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuehint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueHint`

- <span id="valuehint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuehint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueHint`

- <span id="valuehint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuehint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ColorChoice`

```rust
enum ColorChoice {
    Auto,
    Always,
    Never,
}
```

*Defined in [`clap_builder-4.5.53/src/util/color.rs:6-58`](../../.source_1765521767/clap_builder-4.5.53/src/util/color.rs#L6-L58)*

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

- <span id="colorchoice-possible-values"></span>`fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](builder/possible_value/index.md#possiblevalue)

  Report all `possible_values`

#### Trait Implementations

##### `impl Any for ColorChoice`

- <span id="colorchoice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ColorChoice`

- <span id="colorchoice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ColorChoice`

- <span id="colorchoice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ColorChoice`

- <span id="colorchoice-clone"></span>`fn clone(&self) -> ColorChoice` — [`ColorChoice`](util/color/index.md#colorchoice)

##### `impl CloneToUninit for ColorChoice`

- <span id="colorchoice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- <span id="colorchoice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ColorChoice`

- <span id="colorchoice-default"></span>`fn default() -> ColorChoice` — [`ColorChoice`](util/color/index.md#colorchoice)

##### `impl Display for ColorChoice`

- <span id="colorchoice-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl<T> From for ColorChoice`

- <span id="colorchoice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for ColorChoice`

- <span id="colorchoice-fromstr-type-err"></span>`type Err = String`

- <span id="colorchoice-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<U> Into for ColorChoice`

- <span id="colorchoice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-partialeq-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](util/color/index.md#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl ToOwned for ColorChoice`

- <span id="colorchoice-toowned-type-owned"></span>`type Owned = T`

- <span id="colorchoice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colorchoice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ColorChoice`

- <span id="colorchoice-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ColorChoice`

- <span id="colorchoice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colorchoice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColorChoice`

- <span id="colorchoice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colorchoice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum for ColorChoice`

- <span id="colorchoice-valueenum-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="colorchoice-valueenum-to-possible-value"></span>`fn to_possible_value(&self) -> Option<PossibleValue>` — [`PossibleValue`](builder/possible_value/index.md#possiblevalue)

## Traits

### `Args`

```rust
trait Args: FromArgMatches + Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:227-246`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L227-L246)*

Parse a set of arguments into a user-defined container.

Implementing this trait lets a parent container delegate argument parsing behavior to `Self`.
with:
- `#[command(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl
  `Args`.
- `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn augment_args(cmd: Command) -> Command`

  Append to [`Command`](builder/command/index.md) so it can instantiate `Self` via

- `fn augment_args_for_update(cmd: Command) -> Command`

  Append to [`Command`](builder/command/index.md) so it can instantiate `self` via

#### Provided Methods

- `fn group_id() -> Option<crate::Id>`

  Report the `ArgGroup::id` for this set of arguments

#### Implementors

- `()`
- `Box<T>`

### `CommandFactory`

```rust
trait CommandFactory: Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:116-125`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L116-L125)*

Create a [`Command`](builder/command/index.md) relevant for a user-defined container.

Derived as part of [`Parser`](derive/index.md).

#### Required Methods

- `fn command() -> Command`

  Build a [`Command`](builder/command/index.md) that can instantiate `Self`.

- `fn command_for_update() -> Command`

  Build a [`Command`](builder/command/index.md) that can update `self`.

#### Implementors

- `Box<T>`

### `FromArgMatches`

```rust
trait FromArgMatches: Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:130-212`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L130-L212)*

Converts an instance of [`ArgMatches`](parser/matches/arg_matches/index.md) to a user-defined container.

Derived as part of [`Parser`](derive/index.md), [`Args`](derive/index.md), and [`Subcommand`](derive/index.md).

#### Required Methods

- `fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error>`

  Instantiate `Self` from [`ArgMatches`](parser/matches/arg_matches/index.md), parsing the arguments as needed.

- `fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error>`

  Assign values from `ArgMatches` to `self`.

#### Provided Methods

- `fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error>`

  Instantiate `Self` from [`ArgMatches`](parser/matches/arg_matches/index.md), parsing the arguments as needed.

- `fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error>`

  Assign values from `ArgMatches` to `self`.

#### Implementors

- `()`
- `Box<T>`
- `std::convert::Infallible`

### `Parser`

```rust
trait Parser: FromArgMatches + CommandFactory + Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:29-111`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L29-L111)*

Parse command-line arguments into `Self`.

The primary one-stop-shop trait used to create an instance of a `clap`
[`Command`](builder/command/index.md), conduct the parsing, and turn the resulting [`ArgMatches`](parser/matches/arg_matches/index.md) back
into concrete instance of the user struct.

This trait is primarily a convenience on top of [`FromArgMatches`](derive/index.md) +
[`CommandFactory`](derive/index.md) which uses those two underlying traits to build the two
fundamental functions `parse` which uses the `std::env::args_os` iterator,
and `parse_from` which allows the consumer to supply the iterator (along
with fallible options for each).

See also [`Subcommand`](derive/index.md) and [`Args`](derive/index.md).

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Provided Methods

- `fn parse() -> Self`

  Parse from `std::env::args_os()`, `exit` on error.

- `fn try_parse() -> Result<Self, Error>`

  Parse from `std::env::args_os()`, return Err on error.

- `fn parse_from<I, T>(itr: I) -> Self`

  Parse from iterator, `exit` on error.

- `fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>`

  Parse from iterator, return Err on error.

- `fn update_from<I, T>(&mut self, itr: I)`

  Update from iterator, `exit` on error.

- `fn try_update_from<I, T>(&mut self, itr: I) -> Result<(), Error>`

  Update from iterator, return Err on error.

#### Implementors

- `Box<T>`

### `Subcommand`

```rust
trait Subcommand: FromArgMatches + Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:262-279`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L262-L279)*

Parse a sub-command into a user-defined enum.

Implementing this trait lets a parent container delegate subcommand behavior to `Self`.
with:
- `#[command(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum
  variants that impl `Subcommand`.
- `#[command(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl
  `Subcommand`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn augment_subcommands(cmd: Command) -> Command`

  Append to [`Command`](builder/command/index.md) so it can instantiate `Self` via

- `fn augment_subcommands_for_update(cmd: Command) -> Command`

  Append to [`Command`](builder/command/index.md) so it can instantiate `self` via

- `fn has_subcommand(name: &str) -> bool`

  Test whether `Self` can parse a specific subcommand

#### Implementors

- `()`
- `Box<T>`
- `std::convert::Infallible`

### `ValueEnum`

```rust
trait ValueEnum: Sized + Clone { ... }
```

*Defined in [`clap_builder-4.5.53/src/derive.rs:293-314`](../../.source_1765521767/clap_builder-4.5.53/src/derive.rs#L293-L314)*

Parse arguments into enums.

When deriving [`Parser`](derive/index.md), a field whose type implements `ValueEnum` can have the attribute
`#[arg(value_enum)]` which will
- Call `EnumValueParser`
- Allowing using the `#[arg(default_value_t)]` attribute without implementing `Display`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn value_variants<'a>() -> &'a [Self]`

  All possible argument values, in display order.

- `fn to_possible_value(&self) -> Option<PossibleValue>`

  The canonical argument value.

#### Provided Methods

- `fn from_str(input: &str, ignore_case: bool) -> Result<Self, String>`

  Parse an argument into `Self`.

#### Implementors

- [`ColorChoice`](util/color/index.md#colorchoice)

## Type Aliases

### `Error`

```rust
type Error = error::Error<error::DefaultFormatter>;
```

*Defined in [`clap_builder-4.5.53/src/lib.rs:30`](../../.source_1765521767/clap_builder-4.5.53/src/lib.rs#L30)*

Command Line Argument Parser Error

See `Command::error` to create an error.


## Constants

### `INTERNAL_ERROR_MSG`
```rust
const INTERNAL_ERROR_MSG: &str;
```

*Defined in [`clap_builder-4.5.53/src/lib.rs:48-49`](../../.source_1765521767/clap_builder-4.5.53/src/lib.rs#L48-L49)*

## Macros

### `command!`

*Defined in [`clap_builder-4.5.53/src/macros.rs:155-162`](../../.source_1765521767/clap_builder-4.5.53/src/macros.rs#L155-L162)*

Requires `cargo` feature flag to be enabled.

### `arg!`

*Defined in [`clap_builder-4.5.53/src/macros.rs:532-563`](../../.source_1765521767/clap_builder-4.5.53/src/macros.rs#L532-L563)*

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

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2626-2632`](../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L2626-L2632)*

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

