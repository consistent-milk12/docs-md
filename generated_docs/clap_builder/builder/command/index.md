*[clap_builder](../../index.md) / [builder](../index.md) / [command](index.md)*

---

# Module `command`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Command`](#command) | struct | Build a command-line interface. |
| [`TermWidth`](#termwidth) | struct |  |
| [`MaxTermWidth`](#maxtermwidth) | struct |  |
| [`Captures`](#captures) | trait | A workaround |
| [`AppExt`](#appext) | trait |  |
| [`two_elements_of`](#two_elements_of) | fn | Returns the first two elements of an iterator as an `Option<(T, T)>`. |

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

- <span id="command-no-binary-name"></span>`fn no_binary_name(self, yes: bool) -> Self`

- <span id="command-ignore-errors"></span>`fn ignore_errors(self, yes: bool) -> Self`

- <span id="command-args-override-self"></span>`fn args_override_self(self, yes: bool) -> Self`

- <span id="command-dont-delimit-trailing-values"></span>`fn dont_delimit_trailing_values(self, yes: bool) -> Self`

- <span id="command-color"></span>`fn color(self, color: ColorChoice) -> Self` — [`ColorChoice`](../../index.md)

- <span id="command-styles"></span>`fn styles(self, styles: Styles) -> Self` — [`Styles`](../index.md)

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

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](../../index.md)

##### `impl Debug for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Command`

- <span id="command-default"></span>`fn default() -> Self`

##### `impl Display for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index for Command`

- <span id="command-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](../../index.md)

##### `impl<T> ToString for Command`

- <span id="command-to-string"></span>`fn to_string(&self) -> String`

### `TermWidth`

```rust
struct TermWidth(usize);
```

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

A workaround:
<https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999>

#### Implementors

- `T`

### `AppExt`

```rust
trait AppExt: Extension { ... }
```

#### Implementors

- [`MaxTermWidth`](#maxtermwidth)
- [`Styles`](../index.md)
- [`TermWidth`](#termwidth)

## Functions

### `two_elements_of`

```rust
fn two_elements_of<I, T>(iter: I) -> Option<(T, T)>
where
    I: Iterator<Item = T>
```

Returns the first two elements of an iterator as an `Option<(T, T)>`.

If the iterator has fewer than two elements, it returns `None`.

