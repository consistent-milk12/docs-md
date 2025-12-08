*[clap_builder](../../index.md) / [builder](../index.md) / [command](index.md)*

---

# Module `command`

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

- `fn new(name: impl Into<Str>) -> Self` — [`Str`](../str/index.md)

- `fn arg(self: Self, a: impl Into<Arg>) -> Self` — [`Arg`](../arg/index.md)

- `fn arg_internal(self: &mut Self, arg: Arg)` — [`Arg`](../arg/index.md)

- `fn args(self: Self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](../arg/index.md)

- `fn mut_arg<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`

- `fn mut_args<F>(self: Self, f: F) -> Self`

- `fn mut_group<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`

- `fn mut_subcommand<F>(self: Self, name: impl AsRef<str>, f: F) -> Self`

- `fn mut_subcommands<F>(self: Self, f: F) -> Self`

- `fn group(self: Self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](../arg_group/index.md)

- `fn groups(self: Self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](../arg_group/index.md)

- `fn subcommand(self: Self, subcmd: impl Into<Command>) -> Self` — [`Command`](#command)

- `fn subcommand_internal(self: Self, subcmd: Self) -> Self`

- `fn subcommands(self: Self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`

- `fn defer(self: Self, deferred: fn(Command) -> Command) -> Self` — [`Command`](#command)

- `fn debug_assert(self: Self)`

- `fn error(self: &mut Self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](../../error/kind/index.md), [`Error`](../../index.md)

- `fn get_matches(self: Self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn get_matches_mut(self: &mut Self) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn try_get_matches(self: Self) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md), [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn get_matches_from<I, T>(self: Self, itr: I) -> ArgMatches` — [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn try_get_matches_from<I, T>(self: Self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md), [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn try_get_matches_from_mut<I, T>(self: &mut Self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../../error/index.md), [`ArgMatches`](../../parser/matches/arg_matches/index.md)

- `fn print_help(self: &mut Self) -> io::Result<()>`

- `fn print_long_help(self: &mut Self) -> io::Result<()>`

- `fn render_help(self: &mut Self) -> StyledStr` — [`StyledStr`](../styled_str/index.md)

- `fn render_long_help(self: &mut Self) -> StyledStr` — [`StyledStr`](../styled_str/index.md)

- `fn render_version(self: &Self) -> String`

- `fn render_long_version(self: &Self) -> String`

- `fn render_usage(self: &mut Self) -> StyledStr` — [`StyledStr`](../styled_str/index.md)

- `fn render_usage_(self: &mut Self) -> Option<StyledStr>` — [`StyledStr`](../styled_str/index.md)

#### Trait Implementations

##### `impl Clone for Command`

- `fn clone(self: &Self) -> Command` — [`Command`](#command)

##### `impl Debug for Command`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Command`

- `fn default() -> Self`

##### `impl Display for Command`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output` — [`Id`](../../util/id/index.md)

##### `impl<T> ToString for Command`

- `fn to_string(self: &Self) -> String`

### `TermWidth`

```rust
struct TermWidth(usize);
```

#### Trait Implementations

##### `impl AppExt for TermWidth`

##### `impl Clone for TermWidth`

- `fn clone(self: &Self) -> TermWidth` — [`TermWidth`](#termwidth)

##### `impl Copy for TermWidth`

##### `impl Debug for TermWidth`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for TermWidth`

- `fn default() -> TermWidth` — [`TermWidth`](#termwidth)

### `MaxTermWidth`

```rust
struct MaxTermWidth(usize);
```

#### Trait Implementations

##### `impl AppExt for MaxTermWidth`

##### `impl Clone for MaxTermWidth`

- `fn clone(self: &Self) -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

##### `impl Copy for MaxTermWidth`

##### `impl Debug for MaxTermWidth`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MaxTermWidth`

- `fn default() -> MaxTermWidth` — [`MaxTermWidth`](#maxtermwidth)

## Traits

### `Captures<'a>`

```rust
trait Captures<'a> { ... }
```

A workaround:
<https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999>

### `AppExt`

```rust
trait AppExt: Extension { ... }
```

## Functions

### `two_elements_of`

```rust
fn two_elements_of<I, T>(iter: I) -> Option<(T, T)>
where
    I: Iterator<Item = T>
```

Returns the first two elements of an iterator as an `Option<(T, T)>`.

If the iterator has fewer than two elements, it returns `None`.

