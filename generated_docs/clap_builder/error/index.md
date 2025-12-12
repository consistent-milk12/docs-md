*[clap_builder](../index.md) / [error](index.md)*

---

# Module `error`

Error reporting

## Contents

- [Modules](#modules)
  - [`context`](#context)
  - [`format`](#format)
  - [`kind`](#kind)
- [Structs](#structs)
  - [`KindFormatter`](#kindformatter)
  - [`RichFormatter`](#richformatter)
  - [`DefaultFormatter`](#defaultformatter)
  - [`Error`](#error)
  - [`ErrorInner`](#errorinner)
  - [`Backtrace`](#backtrace)
- [Enums](#enums)
  - [`ErrorKind`](#errorkind)
  - [`ContextKind`](#contextkind)
  - [`ContextValue`](#contextvalue)
  - [`Message`](#message)
- [Traits](#traits)
  - [`ErrorFormatter`](#errorformatter)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`context`](#context) | mod |  |
| [`format`](#format) | mod |  |
| [`kind`](#kind) | mod |  |
| [`KindFormatter`](#kindformatter) | struct |  |
| [`RichFormatter`](#richformatter) | struct |  |
| [`DefaultFormatter`](#defaultformatter) | struct |  |
| [`Error`](#error) | struct | Command Line Argument Parser Error |
| [`ErrorInner`](#errorinner) | struct |  |
| [`Backtrace`](#backtrace) | struct |  |
| [`ErrorKind`](#errorkind) | enum |  |
| [`ContextKind`](#contextkind) | enum |  |
| [`ContextValue`](#contextvalue) | enum |  |
| [`Message`](#message) | enum |  |
| [`ErrorFormatter`](#errorformatter) | trait |  |
| [`Result`](#result) | type | Short hand for [`Result`] type |

## Modules

- [`context`](context/index.md)
- [`format`](format/index.md)
- [`kind`](kind/index.md)

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:36`](../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L36)*

Report [`ErrorKind`](kind/index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](format/index.md).

</div>

#### Trait Implementations

##### `impl ErrorFormatter for KindFormatter`

- <span id="kindformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md#styledstr)

### `RichFormatter`

```rust
struct RichFormatter;
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:62`](../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L62)*

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- <span id="richformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md#styledstr)

### `DefaultFormatter`

```rust
struct DefaultFormatter;
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:62`](../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L62)*

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- <span id="richformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md#styledstr)

### `Error<F: ErrorFormatter>`

```rust
struct Error<F: ErrorFormatter> {
    inner: Box<ErrorInner>,
    phantom: std::marker::PhantomData<F>,
}
```

*Defined in [`clap_builder-4.5.53/src/error/mod.rs:60-63`](../../../.source_1765521767/clap_builder-4.5.53/src/error/mod.rs#L60-L63)*

Command Line Argument Parser Error

See `Command::error` to create an error.


#### Implementations

- <span id="error-raw"></span>`fn raw(kind: ErrorKind, message: impl Display) -> Self` — [`ErrorKind`](kind/index.md#errorkind)

- <span id="error-format"></span>`fn format(self, cmd: &mut Command) -> Self` — [`Command`](../builder/command/index.md#command)

- <span id="error-new"></span>`fn new(kind: ErrorKind) -> Self` — [`ErrorKind`](kind/index.md#errorkind)

- <span id="error-with-cmd"></span>`fn with_cmd(self, cmd: &Command) -> Self` — [`Command`](../builder/command/index.md#command)

- <span id="error-apply"></span>`fn apply<EF: ErrorFormatter>(self) -> Error<EF>` — [`Error`](#error)

- <span id="error-kind"></span>`fn kind(&self) -> ErrorKind` — [`ErrorKind`](kind/index.md#errorkind)

- <span id="error-context"></span>`fn context(&self) -> impl Iterator<Item = (ContextKind, &ContextValue)>` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-get"></span>`fn get(&self, kind: ContextKind) -> Option<&ContextValue>` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-insert"></span>`fn insert(&mut self, kind: ContextKind, value: ContextValue) -> Option<ContextValue>` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-remove"></span>`fn remove(&mut self, kind: ContextKind) -> Option<ContextValue>` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-use-stderr"></span>`fn use_stderr(&self) -> bool`

- <span id="error-stream"></span>`fn stream(&self) -> Stream` — [`Stream`](../output/fmt/index.md#stream)

- <span id="error-exit-code"></span>`fn exit_code(&self) -> i32`

- <span id="error-exit"></span>`fn exit(&self) -> never`

- <span id="error-print"></span>`fn print(&self) -> io::Result<()>`

- <span id="error-render"></span>`fn render(&self) -> StyledStr` — [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-for-app"></span>`fn for_app(kind: ErrorKind, cmd: &Command, styled: StyledStr) -> Self` — [`ErrorKind`](kind/index.md#errorkind), [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-set-message"></span>`fn set_message(self, message: impl Into<Message>) -> Self` — [`Message`](#message)

- <span id="error-set-source"></span>`fn set_source(self, source: Box<dyn error::Error + Send + Sync>) -> Self`

- <span id="error-set-styles"></span>`fn set_styles(self, styles: Styles) -> Self` — [`Styles`](../builder/styling/index.md#styles)

- <span id="error-set-color"></span>`fn set_color(self, color_when: ColorChoice) -> Self` — [`ColorChoice`](../util/color/index.md#colorchoice)

- <span id="error-set-colored-help"></span>`fn set_colored_help(self, color_help_when: ColorChoice) -> Self` — [`ColorChoice`](../util/color/index.md#colorchoice)

- <span id="error-set-help-flag"></span>`fn set_help_flag(self, help_flag: Option<Cow<'static, str>>) -> Self`

- <span id="error-insert-context-unchecked"></span>`fn insert_context_unchecked(self, kind: ContextKind, value: ContextValue) -> Self` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-extend-context-unchecked"></span>`fn extend_context_unchecked<const N: usize>(self, context: [(ContextKind, ContextValue); N]) -> Self` — [`ContextKind`](context/index.md#contextkind), [`ContextValue`](context/index.md#contextvalue)

- <span id="error-display-help"></span>`fn display_help(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-display-help-error"></span>`fn display_help_error(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-display-version"></span>`fn display_version(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-argument-conflict"></span>`fn argument_conflict(cmd: &Command, arg: String, others: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-subcommand-conflict"></span>`fn subcommand_conflict(cmd: &Command, sub: String, others: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-empty-value"></span>`fn empty_value(cmd: &Command, good_vals: &[String], arg: String) -> Self` — [`Command`](../builder/command/index.md#command)

- <span id="error-no-equals"></span>`fn no_equals(cmd: &Command, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-invalid-value"></span>`fn invalid_value(cmd: &Command, bad_val: String, good_vals: &[String], arg: String) -> Self` — [`Command`](../builder/command/index.md#command)

- <span id="error-invalid-subcommand"></span>`fn invalid_subcommand(cmd: &Command, subcmd: String, did_you_mean: Vec<String>, name: String, suggested_trailing_arg: bool, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-unrecognized-subcommand"></span>`fn unrecognized_subcommand(cmd: &Command, subcmd: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-missing-required-argument"></span>`fn missing_required_argument(cmd: &Command, required: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-missing-subcommand"></span>`fn missing_subcommand(cmd: &Command, parent: String, available: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-invalid-utf8"></span>`fn invalid_utf8(cmd: &Command, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-too-many-values"></span>`fn too_many_values(cmd: &Command, val: String, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-too-few-values"></span>`fn too_few_values(cmd: &Command, arg: String, min_vals: usize, curr_vals: usize, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-value-validation"></span>`fn value_validation(arg: String, val: String, err: Box<dyn error::Error + Send + Sync>) -> Self`

- <span id="error-wrong-number-of-values"></span>`fn wrong_number_of_values(cmd: &Command, arg: String, num_vals: usize, curr_vals: usize, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-unknown-argument"></span>`fn unknown_argument(cmd: &Command, arg: String, did_you_mean: Option<(String, Option<String>)>, suggested_trailing_arg: bool, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-unnecessary-double-dash"></span>`fn unnecessary_double_dash(cmd: &Command, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="error-formatted"></span>`fn formatted(&self) -> Cow<'_, StyledStr>` — [`StyledStr`](../builder/styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl<F: ErrorFormatter> Debug for Error<F>`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>` — [`Result`](#result)

##### `impl<F: ErrorFormatter> Display for Error<F>`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<F: ErrorFormatter> Error for Error<F>`

- <span id="error-source"></span>`fn source(&self) -> Option<&dyn error::Error>`

##### `impl ToString for Error<F>`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

### `ErrorInner`

```rust
struct ErrorInner {
    kind: ErrorKind,
    context: self::flat_map::FlatMap<ContextKind, ContextValue>,
    message: Option<Message>,
    source: Option<Box<dyn error::Error + Send + Sync>>,
    help_flag: Option<std::borrow::Cow<'static, str>>,
    styles: crate::builder::Styles,
    color_when: crate::util::color::ColorChoice,
    color_help_when: crate::util::color::ColorChoice,
    backtrace: Option<Backtrace>,
}
```

*Defined in [`clap_builder-4.5.53/src/error/mod.rs:66-77`](../../../.source_1765521767/clap_builder-4.5.53/src/error/mod.rs#L66-L77)*

#### Trait Implementations

##### `impl Debug for ErrorInner`

- <span id="errorinner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Backtrace`

```rust
struct Backtrace;
```

*Defined in [`clap_builder-4.5.53/src/error/mod.rs:927`](../../../.source_1765521767/clap_builder-4.5.53/src/error/mod.rs#L927)*

#### Implementations

- <span id="backtrace-new"></span>`fn new() -> Option<Self>`

#### Trait Implementations

##### `impl Debug for Backtrace`

- <span id="backtrace-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Backtrace`

- <span id="backtrace-fmt"></span>`fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl ToString for Backtrace`

- <span id="backtrace-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ErrorKind`

```rust
enum ErrorKind {
    InvalidValue,
    UnknownArgument,
    InvalidSubcommand,
    NoEquals,
    ValueValidation,
    TooManyValues,
    TooFewValues,
    WrongNumberOfValues,
    ArgumentConflict,
    MissingRequiredArgument,
    MissingSubcommand,
    InvalidUtf8,
    DisplayHelp,
    DisplayHelpOnMissingArgumentOrSubcommand,
    DisplayVersion,
    Io,
    Format,
}
```

*Defined in [`clap_builder-4.5.53/src/error/kind.rs:4-330`](../../../.source_1765521767/clap_builder-4.5.53/src/error/kind.rs#L4-L330)*

Command line argument parser kind of error

#### Variants

- **`InvalidValue`**

  Occurs when an `Arg` has a set of possible values,
  and the user provides a value which isn't in that set.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(Arg::new("speed")
          .value_parser(["fast", "slow"]))
      .try_get_matches_from(vec!["prog", "other"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
  ```

- **`UnknownArgument`**

  Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(arg!(--flag "some flag"))
      .try_get_matches_from(vec!["prog", "--other"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::UnknownArgument);
  ```

- **`InvalidSubcommand`**

  Occurs when the user provides an unrecognized [`Subcommand`](../derive/index.md) which meets the threshold for
  being similar enough to an existing subcommand.
  If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
  the more general [`UnknownArgument`](../index.md) error is returned.
  
  # Examples
  
  ```rust
  #[cfg(feature = "suggestions")] {
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, };
  let result = Command::new("prog")
      .subcommand(Command::new("config")
          .about("Used for configuration")
          .arg(Arg::new("config_file")
              .help("The configuration file to use")))
      .try_get_matches_from(vec!["prog", "confi"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
  }
  ```
  
  

- **`NoEquals`**

  Occurs when the user doesn't use equals for an option that requires equal
  sign to provide values.
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, ArgAction};
  let res = Command::new("prog")
      .arg(Arg::new("color")
           .action(ArgAction::Set)
           .require_equals(true)
           .long("color"))
      .try_get_matches_from(vec!["prog", "--color", "red"]);
  assert!(res.is_err());
  assert_eq!(res.unwrap_err().kind(), ErrorKind::NoEquals);
  ```

- **`ValueValidation`**

  Occurs when the user provides a value for an argument with a custom validation and the
  value fails that validation.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, value_parser};
  fn is_numeric(val: &str) -> Result<(), String> {
      match val.parse::<i64>() {
          Ok(..) => Ok(()),
          Err(..) => Err(String::from("value wasn't a number!")),
      }
  }
  
  let result = Command::new("prog")
      .arg(Arg::new("num")
           .value_parser(value_parser!(u8)))
      .try_get_matches_from(vec!["prog", "NotANumber"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
  ```

- **`TooManyValues`**

  Occurs when a user provides more values for an argument than were defined by setting
  `Arg::num_args`.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(Arg::new("arg")
          .num_args(1..=2))
      .try_get_matches_from(vec!["prog", "too", "many", "values"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::TooManyValues);
  ```
  

- **`TooFewValues`**

  Occurs when the user provides fewer values for an argument than were defined by setting
  `Arg::num_args`.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(Arg::new("some_opt")
          .long("opt")
          .num_args(3..))
      .try_get_matches_from(vec!["prog", "--opt", "too", "few"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::TooFewValues);
  ```
  

- **`WrongNumberOfValues`**

  Occurs when the user provides a different number of values for an argument than what's
  been defined by setting `Arg::num_args` or than was implicitly set by
  `Arg::value_names`.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, ArgAction};
  let result = Command::new("prog")
      .arg(Arg::new("some_opt")
          .long("opt")
          .action(ArgAction::Set)
          .num_args(2))
      .try_get_matches_from(vec!["prog", "--opt", "wrong"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
  ```
  
  

- **`ArgumentConflict`**

  Occurs when the user provides two values which conflict with each other and can't be used
  together.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, ArgAction};
  let result = Command::new("prog")
      .arg(Arg::new("debug")
          .long("debug")
          .action(ArgAction::SetTrue)
          .conflicts_with("color"))
      .arg(Arg::new("color")
          .long("color")
          .action(ArgAction::SetTrue))
      .try_get_matches_from(vec!["prog", "--debug", "--color"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
  ```

- **`MissingRequiredArgument`**

  Occurs when the user does not provide one or more required arguments.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(Arg::new("debug")
          .required(true))
      .try_get_matches_from(vec!["prog"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
  ```

- **`MissingSubcommand`**

  Occurs when a subcommand is required (as defined by `Command::subcommand_required`),
  but the user does not provide one.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, error::ErrorKind};
  let err = Command::new("prog")
      .subcommand_required(true)
      .subcommand(Command::new("test"))
      .try_get_matches_from(vec![
          "myprog",
      ]);
  assert!(err.is_err());
  assert_eq!(err.unwrap_err().kind(), ErrorKind::MissingSubcommand);
  ;
  ```
  

- **`InvalidUtf8`**

  Occurs when the user provides a value containing invalid UTF-8.
  
  To allow arbitrary data
  - Set `Arg::value_parser(value_parser!(OsString))` for argument values
  - Set `Command::external_subcommand_value_parser` for external-subcommand
    values
  
  # Platform Specific
  
  Non-Windows platforms only (such as Linux, Unix, OSX, etc.)
  
  # Examples
  
  ```rust
  #[cfg(unix)] {
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, ArgAction};
  use std::os::unix::ffi::OsStringExt;
  use std::ffi::OsString;
  let result = Command::new("prog")
      .arg(Arg::new("utf8")
          .short('u')
          .action(ArgAction::Set))
      .try_get_matches_from(vec![OsString::from("myprog"),
                                  OsString::from("-u"),
                                  OsString::from_vec(vec![0xE9])]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidUtf8);
  }
  ```
  

- **`DisplayHelp`**

  Not a true "error" as it means `--help` or similar was used.
  The help message will be sent to `stdout`.
  
  **Note**: If the help is displayed due to an error (such as missing subcommands) it will
  be sent to `stderr` instead of `stdout`.
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .try_get_matches_from(vec!["prog", "--help"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayHelp);
  }
  ```

- **`DisplayHelpOnMissingArgumentOrSubcommand`**

  Occurs when either an argument or a [`Subcommand`](../derive/index.md) is required, as defined by
  `Command::arg_required_else_help` , but the user did not provide
  one.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind, };
  let result = Command::new("prog")
      .arg_required_else_help(true)
      .subcommand(Command::new("config")
          .about("Used for configuration")
          .arg(Arg::new("config_file")
              .help("The configuration file to use")))
      .try_get_matches_from(vec!["prog"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
  ```
  
  

- **`DisplayVersion`**

  Not a true "error" as it means `--version` or similar was used.
  The message will be sent to `stdout`.
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .version("3.0")
      .try_get_matches_from(vec!["prog", "--version"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayVersion);
  ```

- **`Io`**

  Represents an [I/O error].
  Can occur when writing to `stderr` or `stdout` or reading a configuration file.
  

- **`Format`**

  Represents a [Format error] (which is a part of `Display`).
  Typically caused by writing to `stderr` or `stdout`.
  
  

#### Implementations

- <span id="errorkind-as-str"></span>`fn as_str(self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ErrorKind`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](kind/index.md#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl Hash for ErrorKind`

- <span id="errorkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](kind/index.md#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl ToString for ErrorKind`

- <span id="errorkind-to-string"></span>`fn to_string(&self) -> String`

### `ContextKind`

```rust
enum ContextKind {
    InvalidSubcommand,
    InvalidArg,
    PriorArg,
    ValidSubcommand,
    ValidValue,
    InvalidValue,
    ActualNumValues,
    ExpectedNumValues,
    MinValues,
    SuggestedCommand,
    SuggestedSubcommand,
    SuggestedArg,
    SuggestedValue,
    TrailingArg,
    Suggested,
    Usage,
    Custom,
}
```

*Defined in [`clap_builder-4.5.53/src/error/context.rs:5-40`](../../../.source_1765521767/clap_builder-4.5.53/src/error/context.rs#L5-L40)*

Semantics for a piece of error information

#### Variants

- **`InvalidSubcommand`**

  The cause of the error

- **`InvalidArg`**

  The cause of the error

- **`PriorArg`**

  Existing arguments

- **`ValidSubcommand`**

  Accepted subcommands

- **`ValidValue`**

  Accepted values

- **`InvalidValue`**

  Rejected values

- **`ActualNumValues`**

  Number of values present

- **`ExpectedNumValues`**

  Number of allowed values

- **`MinValues`**

  Minimum number of allowed values

- **`SuggestedCommand`**

  Potential fix for the user

- **`SuggestedSubcommand`**

  Potential fix for the user

- **`SuggestedArg`**

  Potential fix for the user

- **`SuggestedValue`**

  Potential fix for the user

- **`TrailingArg`**

  Trailing argument

- **`Suggested`**

  Potential fix for the user

- **`Usage`**

  A usage string

- **`Custom`**

  An opaque message to the user

#### Implementations

- <span id="contextkind-as-str"></span>`fn as_str(self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ContextKind`

- <span id="contextkind-clone"></span>`fn clone(&self) -> ContextKind` — [`ContextKind`](context/index.md#contextkind)

##### `impl Copy for ContextKind`

##### `impl Debug for ContextKind`

- <span id="contextkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextKind`

- <span id="contextkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextKind`

##### `impl Hash for ContextKind`

- <span id="contextkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ContextKind`

- <span id="contextkind-eq"></span>`fn eq(&self, other: &ContextKind) -> bool` — [`ContextKind`](context/index.md#contextkind)

##### `impl StructuralPartialEq for ContextKind`

##### `impl ToString for ContextKind`

- <span id="contextkind-to-string"></span>`fn to_string(&self) -> String`

### `ContextValue`

```rust
enum ContextValue {
    None,
    Bool(bool),
    String(String),
    Strings(Vec<String>),
    StyledStr(crate::builder::StyledStr),
    StyledStrs(Vec<crate::builder::StyledStr>),
    Number(isize),
}
```

*Defined in [`clap_builder-4.5.53/src/error/context.rs:77-92`](../../../.source_1765521767/clap_builder-4.5.53/src/error/context.rs#L77-L92)*

A piece of error information

#### Variants

- **`None`**

  [`ContextKind`](context/index.md) is self-sufficient, no additional information needed

- **`Bool`**

  A single value

- **`String`**

  A single value

- **`Strings`**

  Many values

- **`StyledStr`**

  A single value

- **`StyledStrs`**

  many value

- **`Number`**

  A single value

#### Trait Implementations

##### `impl Clone for ContextValue`

- <span id="contextvalue-clone"></span>`fn clone(&self) -> ContextValue` — [`ContextValue`](context/index.md#contextvalue)

##### `impl Debug for ContextValue`

- <span id="contextvalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextValue`

- <span id="contextvalue-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextValue`

##### `impl PartialEq for ContextValue`

- <span id="contextvalue-eq"></span>`fn eq(&self, other: &ContextValue) -> bool` — [`ContextValue`](context/index.md#contextvalue)

##### `impl StructuralPartialEq for ContextValue`

##### `impl ToString for ContextValue`

- <span id="contextvalue-to-string"></span>`fn to_string(&self) -> String`

### `Message`

```rust
enum Message {
    Raw(String),
    Formatted(crate::builder::StyledStr),
}
```

*Defined in [`clap_builder-4.5.53/src/error/mod.rs:857-860`](../../../.source_1765521767/clap_builder-4.5.53/src/error/mod.rs#L857-L860)*

#### Implementations

- <span id="message-format"></span>`fn format(&mut self, cmd: &Command, usage: Option<StyledStr>)` — [`Command`](../builder/command/index.md#command), [`StyledStr`](../builder/styled_str/index.md#styledstr)

- <span id="message-formatted"></span>`fn formatted(&self, styles: &Styles) -> Cow<'_, StyledStr>` — [`Styles`](../builder/styling/index.md#styles), [`StyledStr`](../builder/styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl Clone for Message`

- <span id="message-clone"></span>`fn clone(&self) -> Message` — [`Message`](#message)

##### `impl Debug for Message`

- <span id="message-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ErrorFormatter`

```rust
trait ErrorFormatter: Sized { ... }
```

*Defined in [`clap_builder-4.5.53/src/error/format.rs:20-23`](../../../.source_1765521767/clap_builder-4.5.53/src/error/format.rs#L20-L23)*

Defines how to format an error for displaying to the user

#### Required Methods

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

  Stylize the error for the terminal

#### Implementors

- [`KindFormatter`](format/index.md#kindformatter)
- [`RichFormatter`](format/index.md#richformatter)

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

*Defined in [`clap_builder-4.5.53/src/error/mod.rs:53`](../../../.source_1765521767/clap_builder-4.5.53/src/error/mod.rs#L53)*

Short hand for [`Result`](#result) type


