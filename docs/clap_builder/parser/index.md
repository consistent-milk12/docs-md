*[clap_builder](../index.md) / [parser](index.md)*

---

# Module `parser`

`Command` line argument parser

## Structs

### `IdsRef<'a>`

```rust
struct IdsRef<'a> {
    // [REDACTED: Private Fields]
}
```

Iterate over `Arg` and `ArgGroup` [`Id`](../index.md)s via `ArgMatches::ids`.

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, arg, value_parser};

let m = Command::new("myprog")
    .arg(arg!(--color <when>)
        .value_parser(["auto", "always", "never"]))
    .arg(arg!(--config <path>)
        .value_parser(value_parser!(std::path::PathBuf)))
    .get_matches_from(["myprog", "--config=config.toml", "--color=auto"]);
assert_eq!(
    m.ids()
        .map(|id| id.as_str())
        .collect::<Vec<_>>(),
    ["config", "color"]
);
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> IdsRef<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<&'a Id>`

##### `impl ExactSizeIterator`

##### `impl Iterator<'a>`

- `type Item = &'a Id`

- `fn next(self: &mut Self) -> Option<&'a Id>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RawValues<'a>`

```rust
struct RawValues<'a> {
    // [REDACTED: Private Fields]
}
```

Iterate over raw argument values via `ArgMatches::get_raw`.

# Examples

```rust
# #[cfg(unix)] {
# use clap_builder as clap;
# use clap::{Command, arg, value_parser};
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt,OsStringExt};

let m = Command::new("utf8")
    .arg(arg!(<arg> "some arg")
        .value_parser(value_parser!(OsString)))
    .get_matches_from(vec![OsString::from("myprog"),
                            // "Hi {0xe9}!"
                            OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
assert_eq!(
    &*m.get_raw("arg")
        .unwrap()
        .next().unwrap()
        .as_bytes(),
    [b'H', b'i', b' ', 0xe9, b'!']
);
# }
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> RawValues<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<'a>`

- `fn next_back(self: &mut Self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator`

##### `impl Iterator<'a>`

- `type Item = &'a OsStr`

- `fn next(self: &mut Self) -> Option<&'a OsStr>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `Values<T>`

```rust
struct Values<T> {
    // [REDACTED: Private Fields]
}
```

Iterate over multiple values for an argument via `ArgMatches::remove_many`.

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let mut m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .action(ArgAction::Append))
    .get_matches_from(vec!["myapp", "-o", "val1", "-o", "val2"]);

let mut values = m.remove_many::<String>("output")
    .unwrap();

assert_eq!(values.next(), Some(String::from("val1")));
assert_eq!(values.next(), Some(String::from("val2")));
assert_eq!(values.next(), None);
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Values<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<T>`

##### `impl Iterator<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

### `ValuesRef<'a, T>`

```rust
struct ValuesRef<'a, T> {
    // [REDACTED: Private Fields]
}
```

Iterate over multiple values for an argument via `ArgMatches::get_many`.

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .action(ArgAction::Append))
    .get_matches_from(vec!["myapp", "-o", "val1", "-o", "val2"]);

let mut values = m.get_many::<String>("output")
    .unwrap()
    .map(|s| s.as_str());

assert_eq!(values.next(), Some("val1"));
assert_eq!(values.next(), Some("val2"));
assert_eq!(values.next(), None);
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a, T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> ValuesRef<'a, T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator<'a, T: 'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator<'a, T: 'a>`

##### `impl Iterator<'a, T: 'a>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<'a, T: 'a>`

- `fn default() -> Self`

### `ArgMatches`

```rust
struct ArgMatches {
    // [REDACTED: Private Fields]
}
```

Container for parse results.

Used to get information about the arguments that were supplied to the program at runtime by
the user. New instances of this struct are obtained by using the `Command::get_matches` family of
methods.

# Examples

```no_run
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
# use clap::parser::ValueSource;
let matches = Command::new("MyApp")
    .arg(Arg::new("out")
        .long("output")
        .required(true)
        .action(ArgAction::Set)
        .default_value("-"))
    .arg(Arg::new("cfg")
        .short('c')
        .action(ArgAction::Set))
    .get_matches(); // builds the instance of ArgMatches

// to get information about the "cfg" argument we created, such as the value supplied we use
// various ArgMatches methods, such as [ArgMatches::get_one]
if let Some(c) = matches.get_one::<String>("cfg") {
    println!("Value for -c: {c}");
}

// The ArgMatches::get_one method returns an Option because the user may not have supplied
// that argument at runtime. But if we specified that the argument was "required" as we did
// with the "out" argument, we can safely unwrap because `clap` verifies that was actually
// used at runtime.
println!("Value for --output: {}", matches.get_one::<String>("out").unwrap());

// You can check the presence of an argument's values
if matches.contains_id("out") {
    // However, if you want to know where the value came from
    if matches.value_source("out").expect("checked contains_id") == ValueSource::CommandLine {
        println!("`out` set by user");
    } else {
        println!("`out` is defaulted");
    }
}
```


#### Implementations

- `fn get_one<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<&T>`
  Gets the value of a specific option or positional argument.

- `fn get_count(self: &Self, id: &str) -> u8`
  Gets the value of a specific [`ArgAction::Count`][crate::ArgAction::Count] flag

- `fn get_flag(self: &Self, id: &str) -> bool`
  Gets the value of a specific [`ArgAction::SetTrue`][crate::ArgAction::SetTrue] or [`ArgAction::SetFalse`][crate::ArgAction::SetFalse] flag

- `fn get_many<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<ValuesRef<'_, T>>`
  Iterate over values of a specific option or positional argument.

- `fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Option<OccurrencesRef<'_, T>>`
  Iterate over the values passed to each occurrence of an option.

- `fn get_raw(self: &Self, id: &str) -> Option<RawValues<'_>>`
  Iterate over the original argument values.

- `fn get_raw_occurrences(self: &Self, id: &str) -> Option<RawOccurrences<'_>>`
  Iterate over the original values for each occurrence of an option.

- `fn remove_one<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<T>`
  Returns the value of a specific option or positional argument.

- `fn remove_many<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<Values<T>>`
  Return values of a specific option or positional argument.

- `fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Option<Occurrences<T>>`
  Return values for each occurrence of an option.

- `fn contains_id(self: &Self, id: &str) -> bool`
  Check if values are present for the argument or group id

- `fn ids(self: &Self) -> IdsRef<'_>`
  Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`].

- `fn args_present(self: &Self) -> bool`
  Check if any [`Arg`][crate::Arg]s were present on the command line

- `fn value_source(self: &Self, id: &str) -> Option<ValueSource>`
  Report where argument value came from

- `fn index_of(self: &Self, id: &str) -> Option<usize>`
  The first index of that an argument showed up.

- `fn indices_of(self: &Self, id: &str) -> Option<Indices<'_>>`
  All indices an argument appeared at when parsing.

- `fn try_get_one<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<&T>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_one`]

- `fn try_get_many<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<ValuesRef<'_, T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_many`]

- `fn try_get_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<OccurrencesRef<'_, T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_occurrences`]

- `fn try_get_raw(self: &Self, id: &str) -> Result<Option<RawValues<'_>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_raw`]

- `fn try_get_raw_occurrences(self: &Self, id: &str) -> Result<Option<RawOccurrences<'_>>, MatchesError>`
  Non-panicking version of [`ArgMatches::get_raw_occurrences`]

- `fn try_remove_one<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<T>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_one`]

- `fn try_remove_many<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Values<T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_many`]

- `fn try_remove_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Occurrences<T>>, MatchesError>`
  Non-panicking version of [`ArgMatches::remove_occurrences`]

- `fn try_contains_id(self: &Self, id: &str) -> Result<bool, MatchesError>`
  Non-panicking version of [`ArgMatches::contains_id`]

- `fn try_clear_id(self: &mut Self, id: &str) -> Result<bool, MatchesError>`
  Clears the values for the given `id`

- `fn subcommand(self: &Self) -> Option<(&str, &ArgMatches)>`
  The name and `ArgMatches` of the current [subcommand].

- `fn remove_subcommand(self: &mut Self) -> Option<(String, ArgMatches)>`
  Return the name and `ArgMatches` of the current [subcommand].

- `fn subcommand_matches(self: &Self, name: &str) -> Option<&ArgMatches>`
  The `ArgMatches` for the current [subcommand].

- `fn subcommand_name(self: &Self) -> Option<&str>`
  The name of the current [subcommand].

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

- `fn clone(self: &Self) -> ArgMatches`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgMatches) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ArgMatches`

### `Indices<'a>`

```rust
struct Indices<'a> {
    // [REDACTED: Private Fields]
}
```

Iterate over indices for where an argument appeared when parsing, via `ArgMatches::indices_of`

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .num_args(1..)
        .action(ArgAction::Set))
    .get_matches_from(vec!["myapp", "-o", "val1", "val2"]);

let mut indices = m.indices_of("output").unwrap();

assert_eq!(indices.next(), Some(2));
assert_eq!(indices.next(), Some(3));
assert_eq!(indices.next(), None);
```


#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Indices<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl DoubleEndedIterator`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl ExactSizeIterator`

##### `impl Iterator`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

## Enums

### `ValueSource`

```rust
enum ValueSource {
    DefaultValue,
    EnvVariable,
    CommandLine,
}
```

Origin of the argument's value

#### Variants

- **`DefaultValue`**

  Value came `Arg::default_value`

- **`EnvVariable`**

  Value came `Arg::env`

- **`CommandLine`**

  Value was passed in on the command-line

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

- `fn clone(self: &Self) -> ValueSource`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &ValueSource) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ValueSource) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ValueSource) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `MatchesError`

```rust
enum MatchesError {
    Downcast {
        actual: self::any_value::AnyValueId,
        expected: self::any_value::AnyValueId,
    },
    UnknownArgument {
    },
}
```

Violation of `ArgMatches` assumptions

#### Variants

- **`Downcast`**

  Failed to downcast `AnyValue` to the specified type

- **`UnknownArgument`**

  Argument not defined in `Command`

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

- `fn clone(self: &Self) -> MatchesError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error`

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

