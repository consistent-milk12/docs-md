*[clap_builder](../../index.md) / [error](../index.md) / [kind](index.md)*

---

# Module `kind`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorKind`](#errorkind) | enum | Command line argument parser kind of error |

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

*Defined in [`clap_builder-4.5.53/src/error/kind.rs:4-330`](../../../../.source_1765521767/clap_builder-4.5.53/src/error/kind.rs#L4-L330)*

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

  Occurs when the user provides an unrecognized [`Subcommand`](../../derive/index.md) which meets the threshold for
  being similar enough to an existing subcommand.
  If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
  the more general [`UnknownArgument`](../../index.md) error is returned.
  
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

  Occurs when either an argument or a [`Subcommand`](../../derive/index.md) is required, as defined by
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

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind` — [`ErrorKind`](#errorkind)

##### `impl Copy for ErrorKind`

##### `impl Debug for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ErrorKind`

- <span id="errorkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ErrorKind`

##### `impl Hash for ErrorKind`

- <span id="errorkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ErrorKind`

- <span id="errorkind-eq"></span>`fn eq(&self, other: &ErrorKind) -> bool` — [`ErrorKind`](#errorkind)

##### `impl StructuralPartialEq for ErrorKind`

##### `impl ToString for ErrorKind`

- <span id="errorkind-to-string"></span>`fn to_string(&self) -> String`

