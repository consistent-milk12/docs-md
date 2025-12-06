*[clap_builder](../index.md) / [parser](index.md)*

---

# Module `parser`

`Command` line argument parser

## Structs

### `IdsRef<'a>`

```rust
struct IdsRef<'a> {
    iter: std::slice::Iter<'a, crate::util::Id>,
}
```

Iterate over `Arg` and `ArgGroup` [`Id`](../util/id/index.md)s via `ArgMatches::ids`.

# Examples

```rust
use clap_builder as clap;
use clap::{Command, arg, value_parser};

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

##### `impl<'a> Clone for IdsRef<'a>`

- `fn clone(self: &Self) -> IdsRef<'a>` — [`IdsRef`](matches/arg_matches/index.md)

##### `impl<'a> Debug for IdsRef<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for IdsRef<'a>`

- `fn next_back(self: &mut Self) -> Option<&'a Id>` — [`Id`](../util/id/index.md)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl<I> IntoIterator for IdsRef<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for IdsRef<'a>`

- `type Item = &'a Id`

- `fn next(self: &mut Self) -> Option<&'a Id>` — [`Id`](../util/id/index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `RawValues<'a>`

```rust
struct RawValues<'a> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<std::ffi::OsString>>>, fn(&std::ffi::OsString) -> &std::ffi::OsStr>,
    len: usize,
}
```

Iterate over raw argument values via `ArgMatches::get_raw`.

# Examples

```rust
#[cfg(unix)] {
use clap_builder as clap;
use clap::{Command, arg, value_parser};
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
}
```

#### Trait Implementations

##### `impl<'a> Clone for RawValues<'a>`

- `fn clone(self: &Self) -> RawValues<'a>` — [`RawValues`](matches/arg_matches/index.md)

##### `impl<'a> Debug for RawValues<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RawValues<'_>`

- `fn default() -> Self`

##### `impl<'a> DoubleEndedIterator for RawValues<'a>`

- `fn next_back(self: &mut Self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator for RawValues<'_>`

##### `impl<I> IntoIterator for RawValues<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for RawValues<'a>`

- `type Item = &'a OsStr`

- `fn next(self: &mut Self) -> Option<&'a OsStr>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Values<T>`

```rust
struct Values<T> {
    iter: std::iter::Map<std::iter::Flatten<std::vec::IntoIter<Vec<self::any_value::AnyValue>>>, fn(self::any_value::AnyValue) -> T>,
    len: usize,
}
```

Iterate over multiple values for an argument via `ArgMatches::remove_many`.

# Examples

```rust
use clap_builder as clap;
use clap::{Command, Arg, ArgAction};
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

##### `impl<T: $crate::clone::Clone> Clone for Values<T>`

- `fn clone(self: &Self) -> Values<T>` — [`Values`](matches/arg_matches/index.md)

##### `impl<T: $crate::fmt::Debug> Debug for Values<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Default for Values<T>`

- `fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Values<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Values<T>`

##### `impl<I> IntoIterator for Values<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for Values<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ValuesRef<'a, T>`

```rust
struct ValuesRef<'a, T> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<self::any_value::AnyValue>>>, fn(&self::any_value::AnyValue) -> &T>,
    len: usize,
}
```

Iterate over multiple values for an argument via `ArgMatches::get_many`.

# Examples

```rust
use clap_builder as clap;
use clap::{Command, Arg, ArgAction};
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

##### `impl<'a, T: $crate::clone::Clone> Clone for ValuesRef<'a, T>`

- `fn clone(self: &Self) -> ValuesRef<'a, T>` — [`ValuesRef`](matches/arg_matches/index.md)

##### `impl<'a, T: $crate::fmt::Debug> Debug for ValuesRef<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T: 'a> Default for ValuesRef<'a, T>`

- `fn default() -> Self`

##### `impl<'a, T: 'a> DoubleEndedIterator for ValuesRef<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a> ExactSizeIterator for ValuesRef<'a, T>`

##### `impl<I> IntoIterator for ValuesRef<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: 'a> Iterator for ValuesRef<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `ArgMatches`

```rust
struct ArgMatches {
    valid_args: Vec<crate::util::Id>,
    valid_subcommands: Vec<crate::builder::Str>,
    args: self::flat_map::FlatMap<crate::util::Id, matched_arg::MatchedArg>,
    subcommand: Option<Box<SubCommand>>,
}
```

Container for parse results.

Used to get information about the arguments that were supplied to the program at runtime by
the user. New instances of this struct are obtained by using the `Command::get_matches` family of
methods.

# Examples

```no_run
use clap_builder as clap;
use clap::{Command, Arg, ArgAction};
use clap::parser::ValueSource;
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

- `fn try_get_arg(self: &Self, arg: &str) -> Result<Option<&MatchedArg>, MatchesError>` — [`MatchedArg`](matches/matched_arg/index.md), [`MatchesError`](error/index.md)

- `fn try_get_arg_t<T: Any + Send + Sync + 'static>(self: &Self, arg: &str) -> Result<Option<&MatchedArg>, MatchesError>` — [`MatchedArg`](matches/matched_arg/index.md), [`MatchesError`](error/index.md)

- `fn try_remove_arg_t<T: Any + Send + Sync + 'static>(self: &mut Self, arg: &str) -> Result<Option<MatchedArg>, MatchesError>` — [`MatchedArg`](matches/matched_arg/index.md), [`MatchesError`](error/index.md)

- `fn verify_arg_t<T: Any + Send + Sync + 'static>(self: &Self, arg: &MatchedArg) -> Result<(), MatchesError>` — [`MatchedArg`](matches/matched_arg/index.md), [`MatchesError`](error/index.md)

- `fn verify_arg(self: &Self, _arg: &str) -> Result<(), MatchesError>` — [`MatchesError`](error/index.md)

- `fn get_arg<'s>(self: &'s Self, arg: &str) -> Option<&'s MatchedArg>` — [`MatchedArg`](matches/matched_arg/index.md)

- `fn get_subcommand(self: &Self, name: &str) -> Option<&SubCommand>` — [`SubCommand`](matches/arg_matches/index.md)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- `fn clone(self: &Self) -> ArgMatches` — [`ArgMatches`](matches/arg_matches/index.md)

##### `impl Debug for ArgMatches`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgMatches`

- `fn default() -> ArgMatches` — [`ArgMatches`](matches/arg_matches/index.md)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- `fn eq(self: &Self, other: &ArgMatches) -> bool` — [`ArgMatches`](matches/arg_matches/index.md)

##### `impl StructuralPartialEq for ArgMatches`

### `Indices<'a>`

```rust
struct Indices<'a> {
    iter: std::iter::Cloned<std::slice::Iter<'a, usize>>,
    len: usize,
}
```

Iterate over indices for where an argument appeared when parsing, via `ArgMatches::indices_of`

# Examples

```rust
use clap_builder as clap;
use clap::{Command, Arg, ArgAction};
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

##### `impl<'a> Clone for Indices<'a>`

- `fn clone(self: &Self) -> Indices<'a>` — [`Indices`](matches/arg_matches/index.md)

##### `impl<'a> Debug for Indices<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Indices<'_>`

- `fn default() -> Self`

##### `impl DoubleEndedIterator for Indices<'_>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl ExactSizeIterator for Indices<'_>`

##### `impl<I> IntoIterator for Indices<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for Indices<'_>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

#### Implementations

- `fn is_explicit(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone for ValueSource`

- `fn clone(self: &Self) -> ValueSource` — [`ValueSource`](matches/value_source/index.md)

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ValueSource`

##### `impl Ord for ValueSource`

- `fn cmp(self: &Self, other: &ValueSource) -> $crate::cmp::Ordering` — [`ValueSource`](matches/value_source/index.md)

##### `impl PartialEq for ValueSource`

- `fn eq(self: &Self, other: &ValueSource) -> bool` — [`ValueSource`](matches/value_source/index.md)

##### `impl PartialOrd for ValueSource`

- `fn partial_cmp(self: &Self, other: &ValueSource) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ValueSource`](matches/value_source/index.md)

##### `impl StructuralPartialEq for ValueSource`

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

#### Implementations

- `fn unwrap<T>(id: &str, r: Result<T, MatchesError>) -> T` — [`MatchesError`](error/index.md)

#### Trait Implementations

##### `impl Clone for MatchesError`

- `fn clone(self: &Self) -> MatchesError` — [`MatchesError`](error/index.md)

##### `impl Debug for MatchesError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for MatchesError`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for MatchesError`

##### `impl<T> ToString for MatchesError`

- `fn to_string(self: &Self) -> String`

