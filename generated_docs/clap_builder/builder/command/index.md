*[clap_builder](../../index.md) / [builder](../index.md) / [command](index.md)*

---

# Module `command`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Command`](#command) | struct | Build a command-line interface. |
| [`TermWidth`](#termwidth) | struct |  |
| [`MaxTermWidth`](#maxtermwidth) | struct |  |
| [`Captures`](#captures) | trait | A workaround: <https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999> |
| [`AppExt`](#appext) | trait |  |
| [`two_elements_of`](#two-elements-of) | fn | Returns the first two elements of an iterator as an `Option<(T, T)>`. |

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

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:74-113`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L74-L113)*

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

- <span id="command-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](../str/index.md#str)

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

- <span id="command-arg"></span>`fn arg(self, a: impl Into<Arg>) -> Self` — [`Arg`](../arg/index.md#arg)

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

- <span id="command-arg-internal"></span>`fn arg_internal(&mut self, arg: Arg)` — [`Arg`](../arg/index.md#arg)

- <span id="command-args"></span>`fn args(self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](../arg/index.md#arg)

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

  Allows one to mutate an [`Arg`](../arg/index.md) after it's been added to a [`Command`](#command).

  

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

  Allows one to mutate all [`Arg`](../arg/index.md)s after they've been added to a [`Command`](#command).

  

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

  Allows one to mutate an [`ArgGroup`](../arg_group/index.md) after it's been added to a [`Command`](#command).

  

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

  Allows one to mutate a [`Command`](#command) after it's been added as a subcommand.

  

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

  Allows one to mutate all [`Command`](#command)s after they've been added as subcommands.

  

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

- <span id="command-group"></span>`fn group(self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](../arg_group/index.md#arggroup)

  Adds an [`ArgGroup`](../arg_group/index.md) to the application.

  

  [`ArgGroup`](../arg_group/index.md)s are a family of related arguments.

  By placing them in a logical group, you can build easier requirement and exclusion rules.

  

  Example use cases:

  - Make an entire [`ArgGroup`](../arg_group/index.md) required, meaning that one (and *only*

    one) argument from that group must be present at runtime.

  - Name an [`ArgGroup`](../arg_group/index.md) as a conflict to another argument.

    Meaning any of the arguments that belong to that group will cause a failure if present with

    the conflicting argument.

  - Ensure exclusion between arguments.

  - Extract a value from a group instead of determining exactly which argument was used.

  

  # Examples

  

  The following example demonstrates using an [`ArgGroup`](../arg_group/index.md) to ensure that one, and only one,

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

- <span id="command-groups"></span>`fn groups(self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](../arg_group/index.md#arggroup)

  Adds multiple [`ArgGroup`](../arg_group/index.md)s to the [`Command`](#command) at once.

  

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

- <span id="command-subcommand"></span>`fn subcommand(self, subcmd: impl Into<Command>) -> Self` — [`Command`](#command)

  Adds a subcommand to the list of valid possibilities.

  

  Subcommands are effectively sub-[`Command`](#command)s, because they can contain their own arguments,

  subcommands, version, usage, etc. They also function just like [`Command`](#command)s, in that they get

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

- <span id="command-defer"></span>`fn defer(self, deferred: fn(Command) -> Command) -> Self` — [`Command`](#command)

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

  

  **Note:** This will not help with asserts in [`ArgMatches`](../../parser/matches/arg_matches/index.md), those will need exhaustive

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

- <span id="command-error"></span>`fn error(&mut self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](../../error/kind/index.md#errorkind), [`Error`](../../index.md#error)

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

- <span id="command-get-matches"></span>`fn get_matches(self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

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

  

- <span id="command-get-matches-mut"></span>`fn get_matches_mut(&mut self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

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

  

- <span id="command-try-get-matches"></span>`fn try_get_matches(self) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../../error/kind/index.md) is a

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

  

  

  

  

  

  

  

- <span id="command-get-matches-from"></span>`fn get_matches_from<I, T>(self, itr: I) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

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

  

  

- <span id="command-try-get-matches-from"></span>`fn try_get_matches_from<I, T>(self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../../error/kind/index.md) is a `ErrorKind::DisplayHelp`

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

  

  

  

  

  

  

  

  

  

- <span id="command-try-get-matches-from-mut"></span>`fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  Like `Command::try_get_matches_from` but doesn't consume the `Command`.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../../error/kind/index.md) is a [`ErrorKind::DisplayHelp`](../../index.md)

  or [`ErrorKind::DisplayVersion`](../../index.md) respectively. You must call `Error::exit` or

  perform a [`std::process::exit`](../../../libc/index.md) yourself.

  

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

  

  

  

- <span id="command-render-help"></span>`fn render_help(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

  Render the short help message (`-h`) to a [`StyledStr`](../styled_str/index.md)

  

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

  

  

- <span id="command-render-long-help"></span>`fn render_long_help(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

  Render the long help message (`--help`) to a [`StyledStr`](../styled_str/index.md).

  

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

  

  

  

- <span id="command-render-usage"></span>`fn render_usage(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

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

- <span id="command-render-usage"></span>`fn render_usage_(&mut self) -> Option<StyledStr>` — [`StyledStr`](../styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl Any for Command`

- <span id="command-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Command`

- <span id="command-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Command`

- <span id="command-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Command`

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](#command)

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

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](../../util/id/index.md#id)

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

### `TermWidth`

```rust
struct TermWidth(usize);
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5260`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L5260)*

#### Trait Implementations

##### `impl Any for TermWidth`

- <span id="termwidth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AppExt for TermWidth`

##### `impl<T> Borrow for TermWidth`

- <span id="termwidth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TermWidth`

- <span id="termwidth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TermWidth`

- <span id="termwidth-clone"></span>`fn clone(&self) -> TermWidth` — [`TermWidth`](#termwidth)

##### `impl CloneToUninit for TermWidth`

- <span id="termwidth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TermWidth`

##### `impl Debug for TermWidth`

- <span id="termwidth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TermWidth`

- <span id="termwidth-default"></span>`fn default() -> TermWidth` — [`TermWidth`](#termwidth)

##### `impl<T> From for TermWidth`

- <span id="termwidth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TermWidth`

- <span id="termwidth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TermWidth`

- <span id="termwidth-toowned-type-owned"></span>`type Owned = T`

- <span id="termwidth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="termwidth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TermWidth`

- <span id="termwidth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termwidth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TermWidth`

- <span id="termwidth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termwidth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MaxTermWidth`

```rust
struct MaxTermWidth(usize);
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5266`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L5266)*

#### Trait Implementations

##### `impl Any for MaxTermWidth`

- <span id="maxtermwidth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AppExt for MaxTermWidth`

##### `impl<T> Borrow for MaxTermWidth`

- <span id="maxtermwidth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MaxTermWidth`

- <span id="maxtermwidth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MaxTermWidth`

- <span id="maxtermwidth-clone"></span>`fn clone(&self) -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

##### `impl CloneToUninit for MaxTermWidth`

- <span id="maxtermwidth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MaxTermWidth`

##### `impl Debug for MaxTermWidth`

- <span id="maxtermwidth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MaxTermWidth`

- <span id="maxtermwidth-default"></span>`fn default() -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

##### `impl<T> From for MaxTermWidth`

- <span id="maxtermwidth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MaxTermWidth`

- <span id="maxtermwidth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for MaxTermWidth`

- <span id="maxtermwidth-toowned-type-owned"></span>`type Owned = T`

- <span id="maxtermwidth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="maxtermwidth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MaxTermWidth`

- <span id="maxtermwidth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="maxtermwidth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MaxTermWidth`

- <span id="maxtermwidth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="maxtermwidth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Captures<'a>`

```rust
trait Captures<'a> { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:4933`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L4933)*

A workaround:
<https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999>

#### Implementors

- `T`

### `AppExt`

```rust
trait AppExt: Extension { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5256`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L5256)*

#### Implementors

- [`MaxTermWidth`](#maxtermwidth)
- [`Styles`](../styling/index.md#styles)
- [`TermWidth`](#termwidth)

## Functions

### `two_elements_of`

```rust
fn two_elements_of<I, T>(iter: I) -> Option<(T, T)>
where
    I: Iterator<Item = T>
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5273-5284`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L5273-L5284)*

Returns the first two elements of an iterator as an `Option<(T, T)>`.

If the iterator has fewer than two elements, it returns `None`.

