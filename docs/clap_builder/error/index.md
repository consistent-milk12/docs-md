*[clap_builder](../index.md) / [error](index.md)*

---

# Module `error`

Error reporting

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

Report [`ErrorKind`](../index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](../index.md).

</div>

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

##### `impl ErrorFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `RichFormatter`

```rust
struct RichFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

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

##### `impl ErrorFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `DefaultFormatter`

```rust
struct DefaultFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

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

##### `impl ErrorFormatter`

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Error<F: ErrorFormatter>`

```rust
struct Error<F: ErrorFormatter> {
    // [REDACTED: Private Fields]
}
```

Command Line Argument Parser Error

See `Command::error` to create an error.


#### Implementations

- `fn raw(kind: ErrorKind, message: impl Display) -> Self`
  Create an unformatted error

- `fn format(self: Self, cmd: &mut Command) -> Self`
  Format the existing message with the Command's context

- `fn new(kind: ErrorKind) -> Self`
  Create an error with a pre-defined message

- `fn with_cmd(self: Self, cmd: &Command) -> Self`
  Apply [`Command`]'s formatting to the error

- `fn apply<EF: ErrorFormatter>(self: Self) -> Error<EF>`
  Apply an alternative formatter to the error

- `fn kind(self: &Self) -> ErrorKind`
  Type of error for programmatic processing

- `fn context(self: &Self) -> impl Iterator<Item = (ContextKind, &ContextValue)>`
  Additional information to further qualify the error

- `fn get(self: &Self, kind: ContextKind) -> Option<&ContextValue>`
  Lookup a piece of context

- `fn insert(self: &mut Self, kind: ContextKind, value: ContextValue) -> Option<ContextValue>`
  Insert a piece of context

- `fn remove(self: &mut Self, kind: ContextKind) -> Option<ContextValue>`
  Remove a piece of context, return the old value if any

- `fn use_stderr(self: &Self) -> bool`
  Should the message be written to `stdout` or not?

- `fn exit_code(self: &Self) -> i32`
  Returns the exit code that `.exit` will exit the process with.

- `fn exit(self: &Self) -> never`
  Prints the error and exits.

- `fn print(self: &Self) -> io::Result<()>`
  Prints formatted and colored error to `stdout` or `stderr` according to its error kind

- `fn render(self: &Self) -> StyledStr`
  Render the error message to a [`StyledStr`].

#### Trait Implementations

##### `impl From<F: ErrorFormatter>`

- `fn from(e: io::Error) -> Self`

##### `impl From<F: ErrorFormatter>`

- `fn from(e: fmt::Error) -> Self`

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

##### `impl Display<F: ErrorFormatter>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Error<F: ErrorFormatter>`

- `fn source(self: &Self) -> Option<&dyn error::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<F: ErrorFormatter>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
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
  # use clap_builder as clap;
  # use clap::{Command, arg, error::ErrorKind};
  let result = Command::new("prog")
      .arg(arg!(--flag "some flag"))
      .try_get_matches_from(vec!["prog", "--other"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::UnknownArgument);
  ```

- **`InvalidSubcommand`**

  Occurs when the user provides an unrecognized [`Subcommand`](../index.md) which meets the threshold for
  being similar enough to an existing subcommand.
  If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
  the more general [`UnknownArgument`](../index.md) error is returned.
  
  # Examples
  
  ```rust
  # #[cfg(feature = "suggestions")] {
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, };
  let result = Command::new("prog")
      .subcommand(Command::new("config")
          .about("Used for configuration")
          .arg(Arg::new("config_file")
              .help("The configuration file to use")))
      .try_get_matches_from(vec!["prog", "confi"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
  # }
  ```
  
  

- **`NoEquals`**

  Occurs when the user doesn't use equals for an option that requires equal
  sign to provide values.
  
  ```rust
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, ArgAction};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, value_parser};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, ArgAction};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, ArgAction};
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
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
  # use clap_builder as clap;
  # use clap::{Command, error::ErrorKind};
  let err = Command::new("prog")
      .subcommand_required(true)
      .subcommand(Command::new("test"))
      .try_get_matches_from(vec![
          "myprog",
      ]);
  assert!(err.is_err());
  assert_eq!(err.unwrap_err().kind(), ErrorKind::MissingSubcommand);
  # ;
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
  # #[cfg(unix)] {
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, ArgAction};
  # use std::os::unix::ffi::OsStringExt;
  # use std::ffi::OsString;
  let result = Command::new("prog")
      .arg(Arg::new("utf8")
          .short('u')
          .action(ArgAction::Set))
      .try_get_matches_from(vec![OsString::from("myprog"),
                                  OsString::from("-u"),
                                  OsString::from_vec(vec![0xE9])]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidUtf8);
  # }
  ```
  

- **`DisplayHelp`**

  Not a true "error" as it means `--help` or similar was used.
  The help message will be sent to `stdout`.
  
  **Note**: If the help is displayed due to an error (such as missing subcommands) it will
  be sent to `stderr` instead of `stdout`.
  
  # Examples
  
  ```rust
  # #[cfg(feature = "help")] {
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .try_get_matches_from(vec!["prog", "--help"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayHelp);
  # }
  ```

- **`DisplayHelpOnMissingArgumentOrSubcommand`**

  Occurs when either an argument or a [`Subcommand`](../index.md) is required, as defined by
  `Command::arg_required_else_help` , but the user did not provide
  one.
  
  # Examples
  
  ```rust
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind, };
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
  # use clap_builder as clap;
  # use clap::{Command, Arg, error::ErrorKind};
  let result = Command::new("prog")
      .version("3.0")
      .try_get_matches_from(vec!["prog", "--version"]);
  assert!(result.is_err());
  assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayVersion);
  ```

- **`Io`**

  Represents an [I/O error].
  Can occur when writing to `stderr` or `stdout` or reading a configuration file.
  
  [I/O error]: std::io::Error

- **`Format`**

  Represents a [Format error] (which is a part of [`Display`](#display)).
  Typically caused by writing to `stderr` or `stdout`.
  
  [Format error]: std::fmt::Error

#### Implementations

- `fn as_str(self: Self) -> Option<&'static str>`
  End-user description of the error case, where relevant

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

- `fn clone(self: &Self) -> ErrorKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ErrorKind) -> bool`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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
  End-user description of the error case, where relevant

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

- `fn clone(self: &Self) -> ContextKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ContextKind) -> bool`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

  [`ContextKind`](../index.md) is self-sufficient, no additional information needed

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

- `fn clone(self: &Self) -> ContextValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ContextValue) -> bool`

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

Short hand for [`Result`](#result) type


