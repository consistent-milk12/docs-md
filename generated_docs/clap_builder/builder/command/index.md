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

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:74-113`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L74-L113)*

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

- <span id="command-arg"></span>`fn arg(self, a: impl Into<Arg>) -> Self` — [`Arg`](../arg/index.md#arg)

- <span id="command-arg-internal"></span>`fn arg_internal(&mut self, arg: Arg)` — [`Arg`](../arg/index.md#arg)

- <span id="command-args"></span>`fn args(self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](../arg/index.md#arg)

- <span id="command-mut-arg"></span>`fn mut_arg<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

- <span id="command-mut-args"></span>`fn mut_args<F>(self, f: F) -> Self`

- <span id="command-mut-group"></span>`fn mut_group<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

- <span id="command-mut-subcommand"></span>`fn mut_subcommand<F>(self, name: impl AsRef<str>, f: F) -> Self`

- <span id="command-mut-subcommands"></span>`fn mut_subcommands<F>(self, f: F) -> Self`

- <span id="command-group"></span>`fn group(self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](../arg_group/index.md#arggroup)

- <span id="command-groups"></span>`fn groups(self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](../arg_group/index.md#arggroup)

- <span id="command-subcommand"></span>`fn subcommand(self, subcmd: impl Into<Command>) -> Self` — [`Command`](#command)

- <span id="command-subcommand-internal"></span>`fn subcommand_internal(self, subcmd: Self) -> Self`

- <span id="command-subcommands"></span>`fn subcommands(self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`

- <span id="command-defer"></span>`fn defer(self, deferred: fn(Command) -> Command) -> Self` — [`Command`](#command)

- <span id="command-debug-assert"></span>`fn debug_assert(self)`

- <span id="command-error"></span>`fn error(&mut self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](../../error/kind/index.md#errorkind), [`Error`](../../index.md#error)

- <span id="command-get-matches"></span>`fn get_matches(self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-get-matches-mut"></span>`fn get_matches_mut(&mut self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-try-get-matches"></span>`fn try_get_matches(self) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-get-matches-from"></span>`fn get_matches_from<I, T>(self, itr: I) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-try-get-matches-from"></span>`fn try_get_matches_from<I, T>(self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-try-get-matches-from-mut"></span>`fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md#result), [`ArgMatches`](../../parser/matches/arg_matches/index.md#argmatches)

- <span id="command-print-help"></span>`fn print_help(&mut self) -> io::Result<()>`

- <span id="command-print-long-help"></span>`fn print_long_help(&mut self) -> io::Result<()>`

- <span id="command-render-help"></span>`fn render_help(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

- <span id="command-render-long-help"></span>`fn render_long_help(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

- <span id="command-render-version"></span>`fn render_version(&self) -> String`

- <span id="command-render-long-version"></span>`fn render_long_version(&self) -> String`

- <span id="command-render-usage"></span>`fn render_usage(&mut self) -> StyledStr` — [`StyledStr`](../styled_str/index.md#styledstr)

- <span id="command-render-usage"></span>`fn render_usage_(&mut self) -> Option<StyledStr>` — [`StyledStr`](../styled_str/index.md#styledstr)

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

- <span id="command-index-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](../../util/id/index.md#id)

##### `impl ToString for Command`

- <span id="command-to-string"></span>`fn to_string(&self) -> String`

### `TermWidth`

```rust
struct TermWidth(usize);
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5260`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L5260)*

#### Trait Implementations

##### `impl AppExt for TermWidth`

##### `impl Clone for TermWidth`

- <span id="termwidth-clone"></span>`fn clone(&self) -> TermWidth` — [`TermWidth`](#termwidth)

##### `impl Copy for TermWidth`

##### `impl Debug for TermWidth`

- <span id="termwidth-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TermWidth`

- <span id="termwidth-default"></span>`fn default() -> TermWidth` — [`TermWidth`](#termwidth)

### `MaxTermWidth`

```rust
struct MaxTermWidth(usize);
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5266`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L5266)*

#### Trait Implementations

##### `impl AppExt for MaxTermWidth`

##### `impl Clone for MaxTermWidth`

- <span id="maxtermwidth-clone"></span>`fn clone(&self) -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

##### `impl Copy for MaxTermWidth`

##### `impl Debug for MaxTermWidth`

- <span id="maxtermwidth-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MaxTermWidth`

- <span id="maxtermwidth-default"></span>`fn default() -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

## Traits

### `Captures<'a>`

```rust
trait Captures<'a> { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:4933`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L4933)*

A workaround:
<https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999>

#### Implementors

- `T`

### `AppExt`

```rust
trait AppExt: Extension { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5256`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L5256)*

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

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:5273-5284`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/command.rs#L5273-L5284)*

Returns the first two elements of an iterator as an `Option<(T, T)>`.

If the iterator has fewer than two elements, it returns `None`.

