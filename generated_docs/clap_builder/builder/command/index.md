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

- <span id="command-get-usage-name"></span>`fn get_usage_name(&self) -> Option<&str>`

- <span id="command-get-usage-name-fallback"></span>`fn get_usage_name_fallback(&self) -> &str`

- <span id="command-get-display-name"></span>`fn get_display_name(&self) -> Option<&str>`

- <span id="command-get-bin-name"></span>`fn get_bin_name(&self) -> Option<&str>`

- <span id="command-get-bin-name-fallback"></span>`fn get_bin_name_fallback(&self) -> &str`

- <span id="command-set-bin-name"></span>`fn set_bin_name(&mut self, name: impl Into<String>)`

- <span id="command-get-name"></span>`fn get_name(&self) -> &str`

- <span id="command-get-name-str"></span>`fn get_name_str(&self) -> &Str` — [`Str`](../index.md)

- <span id="command-get-name-and-visible-aliases"></span>`fn get_name_and_visible_aliases(&self) -> Vec<&str>`

- <span id="command-get-version"></span>`fn get_version(&self) -> Option<&str>`

- <span id="command-get-long-version"></span>`fn get_long_version(&self) -> Option<&str>`

- <span id="command-get-display-order"></span>`fn get_display_order(&self) -> usize`

- <span id="command-get-author"></span>`fn get_author(&self) -> Option<&str>`

- <span id="command-get-short-flag"></span>`fn get_short_flag(&self) -> Option<char>`

- <span id="command-get-long-flag"></span>`fn get_long_flag(&self) -> Option<&str>`

- <span id="command-get-about"></span>`fn get_about(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-get-long-about"></span>`fn get_long_about(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-is-flatten-help-set"></span>`fn is_flatten_help_set(&self) -> bool`

- <span id="command-get-next-help-heading"></span>`fn get_next_help_heading(&self) -> Option<&str>`

- <span id="command-get-visible-aliases"></span>`fn get_visible_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-visible-short-flag-aliases"></span>`fn get_visible_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_`

- <span id="command-get-visible-long-flag-aliases"></span>`fn get_visible_long_flag_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-all-aliases"></span>`fn get_all_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-all-short-flag-aliases"></span>`fn get_all_short_flag_aliases(&self) -> impl Iterator<Item = char> + '_`

- <span id="command-get-all-long-flag-aliases"></span>`fn get_all_long_flag_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-get-aliases"></span>`fn get_aliases(&self) -> impl Iterator<Item = &str> + '_`

- <span id="command-is-set"></span>`fn is_set(&self, s: AppSettings) -> bool` — [`AppSettings`](../app_settings/index.md)

- <span id="command-get-color"></span>`fn get_color(&self) -> ColorChoice` — [`ColorChoice`](../../index.md)

- <span id="command-get-styles"></span>`fn get_styles(&self) -> &Styles` — [`Styles`](../index.md)

- <span id="command-get-subcommands"></span>`fn get_subcommands(&self) -> impl Iterator<Item = &Command>` — [`Command`](../../index.md)

- <span id="command-get-subcommands-mut"></span>`fn get_subcommands_mut(&mut self) -> impl Iterator<Item = &mut Command>` — [`Command`](../../index.md)

- <span id="command-has-subcommands"></span>`fn has_subcommands(&self) -> bool`

- <span id="command-get-subcommand-help-heading"></span>`fn get_subcommand_help_heading(&self) -> Option<&str>`

- <span id="command-get-subcommand-value-name"></span>`fn get_subcommand_value_name(&self) -> Option<&str>`

- <span id="command-get-before-help"></span>`fn get_before_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-get-before-long-help"></span>`fn get_before_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-get-after-help"></span>`fn get_after_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-get-after-long-help"></span>`fn get_after_long_help(&self) -> Option<&StyledStr>` — [`StyledStr`](../index.md)

- <span id="command-find-subcommand"></span>`fn find_subcommand(&self, name: impl AsRef<std::ffi::OsStr>) -> Option<&Command>` — [`Command`](../../index.md)

- <span id="command-find-subcommand-mut"></span>`fn find_subcommand_mut(&mut self, name: impl AsRef<std::ffi::OsStr>) -> Option<&mut Command>` — [`Command`](../../index.md)

- <span id="command-get-groups"></span>`fn get_groups(&self) -> impl Iterator<Item = &ArgGroup>` — [`ArgGroup`](../../index.md)

- <span id="command-get-arguments"></span>`fn get_arguments(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](../../index.md)

- <span id="command-get-positionals"></span>`fn get_positionals(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](../../index.md)

- <span id="command-get-opts"></span>`fn get_opts(&self) -> impl Iterator<Item = &Arg>` — [`Arg`](../../index.md)

- <span id="command-get-arg-conflicts-with"></span>`fn get_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg>` — [`Arg`](../../index.md)

- <span id="command-get-global-arg-conflicts-with"></span>`fn get_global_arg_conflicts_with(&self, arg: &Arg) -> Vec<&Arg>` — [`Arg`](../../index.md)

- <span id="command-get-subcommands-containing"></span>`fn get_subcommands_containing(&self, arg: &Arg) -> Vec<&Self>` — [`Arg`](../../index.md)

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

- <span id="command-get-external-subcommand-value-parser"></span>`fn get_external_subcommand_value_parser(&self) -> Option<&super::ValueParser>` — [`ValueParser`](../index.md)

- <span id="command-is-args-conflicts-with-subcommands-set"></span>`fn is_args_conflicts_with_subcommands_set(&self) -> bool`

- <span id="command-is-subcommand-precedence-over-arg-set"></span>`fn is_subcommand_precedence_over_arg_set(&self) -> bool`

- <span id="command-is-subcommand-negates-reqs-set"></span>`fn is_subcommand_negates_reqs_set(&self) -> bool`

- <span id="command-is-multicall-set"></span>`fn is_multicall_set(&self) -> bool`

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

