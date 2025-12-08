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

- `fn name(self: Self, name: impl Into<Str>) -> Self` — [`Str`](../index.md)

- `fn bin_name(self: Self, name: impl IntoResettable<String>) -> Self` — [`IntoResettable`](../index.md)

- `fn display_name(self: Self, name: impl IntoResettable<String>) -> Self` — [`IntoResettable`](../index.md)

- `fn author(self: Self, author: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../index.md), [`Str`](../index.md)

- `fn about(self: Self, about: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn long_about(self: Self, long_about: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn after_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn after_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn before_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn before_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn version(self: Self, ver: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../index.md), [`Str`](../index.md)

- `fn long_version(self: Self, ver: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../index.md), [`Str`](../index.md)

- `fn override_usage(self: Self, usage: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn override_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn help_template(self: Self, s: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../index.md), [`StyledStr`](../index.md)

- `fn setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](../app_settings/index.md)

- `fn unset_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](../app_settings/index.md)

- `fn global_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](../app_settings/index.md)

- `fn unset_global_setting(self: Self, setting: AppSettings) -> Self` — [`AppSettings`](../app_settings/index.md)

- `fn flatten_help(self: Self, yes: bool) -> Self`

- `fn next_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../index.md), [`Str`](../index.md)

- `fn next_display_order(self: Self, disp_ord: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](../index.md)

- `fn arg_required_else_help(self: Self, yes: bool) -> Self`

- `fn allow_missing_positional(self: Self, yes: bool) -> Self`

#### Trait Implementations

##### `impl Clone for Command`

- `fn clone(self: &Self) -> Command` — [`Command`](../../index.md)

##### `impl Debug for Command`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Command`

- `fn default() -> Self`

##### `impl Display for Command`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output` — [`Id`](../../index.md)

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

