*[clap_builder](../index.md) / [error](index.md)*

---

# Module `error`

Error reporting

## Modules

- [`context`](context/index.md) - 
- [`format`](format/index.md) - 
- [`kind`](kind/index.md) - 

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

Report [`ErrorKind`](kind/index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](format/index.md).

</div>

#### Trait Implementations

##### `impl ErrorFormatter for KindFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md)

### `RichFormatter`

```rust
struct RichFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md)

### `DefaultFormatter`

```rust
struct DefaultFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](#error), [`StyledStr`](../builder/styled_str/index.md)

### `Error<F: ErrorFormatter>`

```rust
struct Error<F: ErrorFormatter> {
    inner: Box<ErrorInner>,
    phantom: std::marker::PhantomData<F>,
}
```

Command Line Argument Parser Error

See `Command::error` to create an error.


#### Implementations

- `fn raw(kind: ErrorKind, message: impl Display) -> Self` — [`ErrorKind`](kind/index.md)

- `fn format(self: Self, cmd: &mut Command) -> Self` — [`Command`](../builder/command/index.md)

- `fn new(kind: ErrorKind) -> Self` — [`ErrorKind`](kind/index.md)

- `fn with_cmd(self: Self, cmd: &Command) -> Self` — [`Command`](../builder/command/index.md)

- `fn apply<EF: ErrorFormatter>(self: Self) -> Error<EF>` — [`Error`](#error)

- `fn kind(self: &Self) -> ErrorKind` — [`ErrorKind`](kind/index.md)

- `fn context(self: &Self) -> impl Iterator<Item = (ContextKind, &ContextValue)>` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn get(self: &Self, kind: ContextKind) -> Option<&ContextValue>` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn insert(self: &mut Self, kind: ContextKind, value: ContextValue) -> Option<ContextValue>` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn remove(self: &mut Self, kind: ContextKind) -> Option<ContextValue>` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn use_stderr(self: &Self) -> bool`

- `fn stream(self: &Self) -> Stream` — [`Stream`](../output/fmt/index.md)

- `fn exit_code(self: &Self) -> i32`

- `fn exit(self: &Self) -> never`

- `fn print(self: &Self) -> io::Result<()>`

- `fn render(self: &Self) -> StyledStr` — [`StyledStr`](../builder/styled_str/index.md)

- `fn for_app(kind: ErrorKind, cmd: &Command, styled: StyledStr) -> Self` — [`ErrorKind`](kind/index.md), [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn set_message(self: Self, message: impl Into<Message>) -> Self` — [`Message`](#message)

- `fn set_source(self: Self, source: Box<dyn error::Error + Send + Sync>) -> Self`

- `fn set_styles(self: Self, styles: Styles) -> Self` — [`Styles`](../builder/styling/index.md)

- `fn set_color(self: Self, color_when: ColorChoice) -> Self` — [`ColorChoice`](../util/color/index.md)

- `fn set_colored_help(self: Self, color_help_when: ColorChoice) -> Self` — [`ColorChoice`](../util/color/index.md)

- `fn set_help_flag(self: Self, help_flag: Option<Cow<'static, str>>) -> Self`

- `fn insert_context_unchecked(self: Self, kind: ContextKind, value: ContextValue) -> Self` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn extend_context_unchecked<const N: usize>(self: Self, context: [(ContextKind, ContextValue); N]) -> Self` — [`ContextKind`](context/index.md), [`ContextValue`](context/index.md)

- `fn display_help(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn display_help_error(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn display_version(cmd: &Command, styled: StyledStr) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn argument_conflict(cmd: &Command, arg: String, others: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn subcommand_conflict(cmd: &Command, sub: String, others: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn empty_value(cmd: &Command, good_vals: &[String], arg: String) -> Self` — [`Command`](../builder/command/index.md)

- `fn no_equals(cmd: &Command, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn invalid_value(cmd: &Command, bad_val: String, good_vals: &[String], arg: String) -> Self` — [`Command`](../builder/command/index.md)

- `fn invalid_subcommand(cmd: &Command, subcmd: String, did_you_mean: Vec<String>, name: String, suggested_trailing_arg: bool, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn unrecognized_subcommand(cmd: &Command, subcmd: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn missing_required_argument(cmd: &Command, required: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn missing_subcommand(cmd: &Command, parent: String, available: Vec<String>, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn invalid_utf8(cmd: &Command, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn too_many_values(cmd: &Command, val: String, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn too_few_values(cmd: &Command, arg: String, min_vals: usize, curr_vals: usize, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn value_validation(arg: String, val: String, err: Box<dyn error::Error + Send + Sync>) -> Self`

- `fn wrong_number_of_values(cmd: &Command, arg: String, num_vals: usize, curr_vals: usize, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn unknown_argument(cmd: &Command, arg: String, did_you_mean: Option<(String, Option<String>)>, suggested_trailing_arg: bool, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn unnecessary_double_dash(cmd: &Command, arg: String, usage: Option<StyledStr>) -> Self` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn formatted(self: &Self) -> Cow<'_, StyledStr>` — [`StyledStr`](../builder/styled_str/index.md)

#### Trait Implementations

##### `impl<F: ErrorFormatter> Debug for Error<F>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>` — [`Result`](#result)

##### `impl<F: ErrorFormatter> Display for Error<F>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<F: ErrorFormatter> Error for Error<F>`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl<T> ToString for Error<F>`

- `fn to_string(self: &Self) -> String`

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

#### Trait Implementations

##### `impl Debug for ErrorInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Backtrace`

```rust
struct Backtrace;
```

#### Implementations

- `fn new() -> Option<Self>`

#### Trait Implementations

##### `impl Debug for Backtrace`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Backtrace`

- `fn fmt(self: &Self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for Backtrace`

- `fn to_string(self: &Self) -> String`

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

- `fn as_str(self: Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ErrorKind`

- `fn clone(self: &Self) -> ErrorKind` — [`ErrorKind`](kind/index.md)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ErrorKind`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl Hash for ErrorKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ErrorKind`

- `fn eq(self: &Self, other: &ErrorKind) -> bool` — [`ErrorKind`](kind/index.md)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl<T> ToString for ErrorKind`

- `fn to_string(self: &Self) -> String`

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

- `fn as_str(self: Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ContextKind`

- `fn clone(self: &Self) -> ContextKind` — [`ContextKind`](context/index.md)

##### `impl Copy for ContextKind`

##### `impl Debug for ContextKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ContextKind`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextKind`

##### `impl Hash for ContextKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ContextKind`

- `fn eq(self: &Self, other: &ContextKind) -> bool` — [`ContextKind`](context/index.md)

##### `impl StructuralPartialEq for ContextKind`

##### `impl<T> ToString for ContextKind`

- `fn to_string(self: &Self) -> String`

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

- `fn clone(self: &Self) -> ContextValue` — [`ContextValue`](context/index.md)

##### `impl Debug for ContextValue`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ContextValue`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextValue`

##### `impl PartialEq for ContextValue`

- `fn eq(self: &Self, other: &ContextValue) -> bool` — [`ContextValue`](context/index.md)

##### `impl StructuralPartialEq for ContextValue`

##### `impl<T> ToString for ContextValue`

- `fn to_string(self: &Self) -> String`

### `Message`

```rust
enum Message {
    Raw(String),
    Formatted(crate::builder::StyledStr),
}
```

#### Implementations

- `fn format(self: &mut Self, cmd: &Command, usage: Option<StyledStr>)` — [`Command`](../builder/command/index.md), [`StyledStr`](../builder/styled_str/index.md)

- `fn formatted(self: &Self, styles: &Styles) -> Cow<'_, StyledStr>` — [`Styles`](../builder/styling/index.md), [`StyledStr`](../builder/styled_str/index.md)

#### Trait Implementations

##### `impl Clone for Message`

- `fn clone(self: &Self) -> Message` — [`Message`](#message)

##### `impl Debug for Message`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

Short hand for [`Result`](#result) type


