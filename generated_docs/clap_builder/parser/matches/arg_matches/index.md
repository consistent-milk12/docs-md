*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [arg_matches](index.md)*

---

# Module `arg_matches`

## Structs

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

- `fn try_get_one<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<&T>, MatchesError>` — [`MatchesError`](../../index.md)

- `fn try_get_many<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<ValuesRef<'_, T>>, MatchesError>` — [`ValuesRef`](../../index.md), [`MatchesError`](../../index.md)

- `fn try_get_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &Self, id: &str) -> Result<Option<OccurrencesRef<'_, T>>, MatchesError>` — [`OccurrencesRef`](#occurrencesref), [`MatchesError`](../../index.md)

- `fn try_get_raw(self: &Self, id: &str) -> Result<Option<RawValues<'_>>, MatchesError>` — [`RawValues`](../../index.md), [`MatchesError`](../../index.md)

- `fn try_get_raw_occurrences(self: &Self, id: &str) -> Result<Option<RawOccurrences<'_>>, MatchesError>` — [`RawOccurrences`](#rawoccurrences), [`MatchesError`](../../index.md)

- `fn try_remove_one<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<T>, MatchesError>` — [`MatchesError`](../../index.md)

- `fn try_remove_many<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Values<T>>, MatchesError>` — [`Values`](../../index.md), [`MatchesError`](../../index.md)

- `fn try_remove_occurrences<T: Any + Clone + Send + Sync + 'static>(self: &mut Self, id: &str) -> Result<Option<Occurrences<T>>, MatchesError>` — [`Occurrences`](#occurrences), [`MatchesError`](../../index.md)

- `fn try_contains_id(self: &Self, id: &str) -> Result<bool, MatchesError>` — [`MatchesError`](../../index.md)

- `fn try_clear_id(self: &mut Self, id: &str) -> Result<bool, MatchesError>` — [`MatchesError`](../../index.md)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- `fn clone(self: &Self) -> ArgMatches` — [`ArgMatches`](../../../index.md)

##### `impl Debug for ArgMatches`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgMatches`

- `fn default() -> ArgMatches` — [`ArgMatches`](../../../index.md)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- `fn eq(self: &Self, other: &ArgMatches) -> bool` — [`ArgMatches`](../../../index.md)

##### `impl StructuralPartialEq for ArgMatches`

### `SubCommand`

```rust
struct SubCommand {
    name: String,
    matches: ArgMatches,
}
```

#### Trait Implementations

##### `impl Clone for SubCommand`

- `fn clone(self: &Self) -> SubCommand` — [`SubCommand`](#subcommand)

##### `impl Debug for SubCommand`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SubCommand`

##### `impl PartialEq for SubCommand`

- `fn eq(self: &Self, other: &SubCommand) -> bool` — [`SubCommand`](#subcommand)

##### `impl StructuralPartialEq for SubCommand`

### `IdsRef<'a>`

```rust
struct IdsRef<'a> {
    iter: std::slice::Iter<'a, crate::util::Id>,
}
```

Iterate over `Arg` and `ArgGroup` [`Id`](../../../index.md)s via `ArgMatches::ids`.

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

- `fn clone(self: &Self) -> IdsRef<'a>` — [`IdsRef`](../../index.md)

##### `impl<'a> Debug for IdsRef<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for IdsRef<'a>`

- `fn next_back(self: &mut Self) -> Option<&'a Id>` — [`Id`](../../../index.md)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl<I> IntoIterator for IdsRef<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for IdsRef<'a>`

- `type Item = &'a Id`

- `fn next(self: &mut Self) -> Option<&'a Id>` — [`Id`](../../../index.md)

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

- `fn clone(self: &Self) -> Values<T>` — [`Values`](../../index.md)

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

- `fn clone(self: &Self) -> ValuesRef<'a, T>` — [`ValuesRef`](../../index.md)

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

- `fn clone(self: &Self) -> RawValues<'a>` — [`RawValues`](../../index.md)

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

### `Occurrences<T>`

```rust
struct Occurrences<T> {
    iter: std::iter::Map<std::vec::IntoIter<Vec<self::any_value::AnyValue>>, fn(Vec<self::any_value::AnyValue>) -> OccurrenceValues<T>>,
}
```

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Occurrences<T>`

- `fn clone(self: &Self) -> Occurrences<T>` — [`Occurrences`](#occurrences)

##### `impl<T: $crate::fmt::Debug> Debug for Occurrences<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Default for Occurrences<T>`

- `fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Occurrences<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Occurrences<T>`

##### `impl<I> IntoIterator for Occurrences<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for Occurrences<T>`

- `type Item = OccurrenceValues<T>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `OccurrenceValues<T>`

```rust
struct OccurrenceValues<T> {
    iter: std::iter::Map<std::vec::IntoIter<self::any_value::AnyValue>, fn(self::any_value::AnyValue) -> T>,
}
```

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for OccurrenceValues<T>`

- `fn clone(self: &Self) -> OccurrenceValues<T>` — [`OccurrenceValues`](#occurrencevalues)

##### `impl<T: $crate::fmt::Debug> Debug for OccurrenceValues<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> DoubleEndedIterator for OccurrenceValues<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for OccurrenceValues<T>`

##### `impl<I> IntoIterator for OccurrenceValues<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T> Iterator for OccurrenceValues<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `OccurrencesRef<'a, T>`

```rust
struct OccurrencesRef<'a, T> {
    iter: std::iter::Map<std::slice::Iter<'a, Vec<self::any_value::AnyValue>>, fn(&Vec<self::any_value::AnyValue>) -> OccurrenceValuesRef<'_, T>>,
}
```

#### Trait Implementations

##### `impl<'a, T: $crate::clone::Clone> Clone for OccurrencesRef<'a, T>`

- `fn clone(self: &Self) -> OccurrencesRef<'a, T>` — [`OccurrencesRef`](#occurrencesref)

##### `impl<'a, T: $crate::fmt::Debug> Debug for OccurrencesRef<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Default for OccurrencesRef<'_, T>`

- `fn default() -> Self`

##### `impl<'a, T> DoubleEndedIterator for OccurrencesRef<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for OccurrencesRef<'a, T>`

##### `impl<I> IntoIterator for OccurrencesRef<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for OccurrencesRef<'a, T>`

- `type Item = OccurrenceValuesRef<'a, T>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `OccurrenceValuesRef<'a, T>`

```rust
struct OccurrenceValuesRef<'a, T> {
    iter: std::iter::Map<std::slice::Iter<'a, self::any_value::AnyValue>, fn(&self::any_value::AnyValue) -> &T>,
}
```

#### Trait Implementations

##### `impl<'a, T: $crate::clone::Clone> Clone for OccurrenceValuesRef<'a, T>`

- `fn clone(self: &Self) -> OccurrenceValuesRef<'a, T>` — [`OccurrenceValuesRef`](#occurrencevaluesref)

##### `impl<'a, T: $crate::fmt::Debug> Debug for OccurrenceValuesRef<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T> DoubleEndedIterator for OccurrenceValuesRef<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for OccurrenceValuesRef<'a, T>`

##### `impl<I> IntoIterator for OccurrenceValuesRef<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T> Iterator for OccurrenceValuesRef<'a, T>`

- `type Item = &'a T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `RawOccurrences<'a>`

```rust
struct RawOccurrences<'a> {
    iter: std::iter::Map<std::slice::Iter<'a, Vec<std::ffi::OsString>>, fn(&Vec<std::ffi::OsString>) -> RawOccurrenceValues<'_>>,
}
```

#### Trait Implementations

##### `impl<'a> Clone for RawOccurrences<'a>`

- `fn clone(self: &Self) -> RawOccurrences<'a>` — [`RawOccurrences`](#rawoccurrences)

##### `impl<'a> Debug for RawOccurrences<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RawOccurrences<'_>`

- `fn default() -> Self`

##### `impl DoubleEndedIterator for RawOccurrences<'_>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for RawOccurrences<'_>`

##### `impl<I> IntoIterator for RawOccurrences<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for RawOccurrences<'a>`

- `type Item = RawOccurrenceValues<'a>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `RawOccurrenceValues<'a>`

```rust
struct RawOccurrenceValues<'a> {
    iter: std::iter::Map<std::slice::Iter<'a, std::ffi::OsString>, fn(&std::ffi::OsString) -> &std::ffi::OsStr>,
}
```

#### Trait Implementations

##### `impl<'a> Clone for RawOccurrenceValues<'a>`

- `fn clone(self: &Self) -> RawOccurrenceValues<'a>` — [`RawOccurrenceValues`](#rawoccurrencevalues)

##### `impl<'a> Debug for RawOccurrenceValues<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for RawOccurrenceValues<'a>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for RawOccurrenceValues<'_>`

##### `impl<I> IntoIterator for RawOccurrenceValues<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for RawOccurrenceValues<'a>`

- `type Item = &'a OsStr`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

- `fn clone(self: &Self) -> Indices<'a>` — [`Indices`](../../index.md)

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

## Functions

### `unwrap_downcast_ref`

```rust
fn unwrap_downcast_ref<T: Any + Clone + Send + Sync + 'static>(value: &self::any_value::AnyValue) -> &T
```

### `unwrap_downcast_into`

```rust
fn unwrap_downcast_into<T: Any + Clone + Send + Sync + 'static>(value: self::any_value::AnyValue) -> T
```

