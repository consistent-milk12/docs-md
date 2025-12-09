*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [arg_matches](index.md)*

---

# Module `arg_matches`

## Contents

- [Structs](#structs)
  - [`ArgMatches`](#argmatches)
  - [`SubCommand`](#subcommand)
  - [`IdsRef`](#idsref)
  - [`Values`](#values)
  - [`ValuesRef`](#valuesref)
  - [`RawValues`](#rawvalues)
  - [`Occurrences`](#occurrences)
  - [`OccurrenceValues`](#occurrencevalues)
  - [`OccurrencesRef`](#occurrencesref)
  - [`OccurrenceValuesRef`](#occurrencevaluesref)
  - [`RawOccurrences`](#rawoccurrences)
  - [`RawOccurrenceValues`](#rawoccurrencevalues)
  - [`Indices`](#indices)
- [Functions](#functions)
  - [`unwrap_downcast_ref`](#unwrap_downcast_ref)
  - [`unwrap_downcast_into`](#unwrap_downcast_into)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgMatches`](#argmatches) | struct | Container for parse results. |
| [`SubCommand`](#subcommand) | struct |  |
| [`IdsRef`](#idsref) | struct | Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`]. |
| [`Values`](#values) | struct | Iterate over multiple values for an argument via [`ArgMatches::remove_many`]. |
| [`ValuesRef`](#valuesref) | struct | Iterate over multiple values for an argument via [`ArgMatches::get_many`]. |
| [`RawValues`](#rawvalues) | struct | Iterate over raw argument values via [`ArgMatches::get_raw`]. |
| [`Occurrences`](#occurrences) | struct |  |
| [`OccurrenceValues`](#occurrencevalues) | struct |  |
| [`OccurrencesRef`](#occurrencesref) | struct |  |
| [`OccurrenceValuesRef`](#occurrencevaluesref) | struct |  |
| [`RawOccurrences`](#rawoccurrences) | struct |  |
| [`RawOccurrenceValues`](#rawoccurrencevalues) | struct |  |
| [`Indices`](#indices) | struct | Iterate over indices for where an argument appeared when parsing, via [`ArgMatches::indices_of`] |
| [`unwrap_downcast_ref`](#unwrap_downcast_ref) | fn |  |
| [`unwrap_downcast_into`](#unwrap_downcast_into) | fn |  |

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

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:67-74`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L67-L74)*

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

- <span id="argmatches-get-one"></span>`fn get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<&T>`

- <span id="argmatches-get-count"></span>`fn get_count(&self, id: &str) -> u8`

- <span id="argmatches-get-flag"></span>`fn get_flag(&self, id: &str) -> bool`

- <span id="argmatches-get-many"></span>`fn get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<ValuesRef<'_, T>>` — [`ValuesRef`](#valuesref)

- <span id="argmatches-get-occurrences"></span>`fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<OccurrencesRef<'_, T>>` — [`OccurrencesRef`](#occurrencesref)

- <span id="argmatches-get-raw"></span>`fn get_raw(&self, id: &str) -> Option<RawValues<'_>>` — [`RawValues`](#rawvalues)

- <span id="argmatches-get-raw-occurrences"></span>`fn get_raw_occurrences(&self, id: &str) -> Option<RawOccurrences<'_>>` — [`RawOccurrences`](#rawoccurrences)

- <span id="argmatches-remove-one"></span>`fn remove_one<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<T>`

- <span id="argmatches-remove-many"></span>`fn remove_many<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Values<T>>` — [`Values`](#values)

- <span id="argmatches-remove-occurrences"></span>`fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Occurrences<T>>` — [`Occurrences`](#occurrences)

- <span id="argmatches-contains-id"></span>`fn contains_id(&self, id: &str) -> bool`

- <span id="argmatches-ids"></span>`fn ids(&self) -> IdsRef<'_>` — [`IdsRef`](#idsref)

- <span id="argmatches-args-present"></span>`fn args_present(&self) -> bool`

- <span id="argmatches-value-source"></span>`fn value_source(&self, id: &str) -> Option<ValueSource>` — [`ValueSource`](../value_source/index.md)

- <span id="argmatches-index-of"></span>`fn index_of(&self, id: &str) -> Option<usize>`

- <span id="argmatches-indices-of"></span>`fn indices_of(&self, id: &str) -> Option<Indices<'_>>` — [`Indices`](#indices)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](#argmatches)

##### `impl Debug for ArgMatches`

- <span id="argmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](#argmatches)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](#argmatches)

##### `impl StructuralPartialEq for ArgMatches`

### `SubCommand`

```rust
struct SubCommand {
    name: String,
    matches: ArgMatches,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1357-1360`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1357-L1360)*

#### Trait Implementations

##### `impl Clone for SubCommand`

- <span id="subcommand-clone"></span>`fn clone(&self) -> SubCommand` — [`SubCommand`](#subcommand)

##### `impl Debug for SubCommand`

- <span id="subcommand-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubCommand`

##### `impl PartialEq for SubCommand`

- <span id="subcommand-eq"></span>`fn eq(&self, other: &SubCommand) -> bool` — [`SubCommand`](#subcommand)

##### `impl StructuralPartialEq for SubCommand`

### `IdsRef<'a>`

```rust
struct IdsRef<'a> {
    iter: std::slice::Iter<'a, crate::util::Id>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1384-1386`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1384-L1386)*

Iterate over `Arg` and `ArgGroup` [`Id`](../../../util/id/index.md)s via `ArgMatches::ids`.

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

##### `impl Clone for IdsRef<'a>`

- <span id="idsref-clone"></span>`fn clone(&self) -> IdsRef<'a>` — [`IdsRef`](#idsref)

##### `impl Debug for IdsRef<'a>`

- <span id="idsref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IdsRef<'a>`

- <span id="idsref-next-back"></span>`fn next_back(&mut self) -> Option<&'a Id>` — [`Id`](../../../util/id/index.md)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl IntoIterator for IdsRef<'a>`

- <span id="idsref-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="idsref-type-intoiter"></span>`type IntoIter = I`

- <span id="idsref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IdsRef<'a>`

- <span id="idsref-type-item"></span>`type Item = &'a Id`

- <span id="idsref-next"></span>`fn next(&mut self) -> Option<&'a Id>` — [`Id`](../../../util/id/index.md)

- <span id="idsref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Values<T>`

```rust
struct Values<T> {
    iter: std::iter::Map<std::iter::Flatten<std::vec::IntoIter<Vec<self::any_value::AnyValue>>>, fn(self::any_value::AnyValue) -> T>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1428-1432`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1428-L1432)*

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

##### `impl<T: clone::Clone> Clone for Values<T>`

- <span id="values-clone"></span>`fn clone(&self) -> Values<T>` — [`Values`](#values)

##### `impl<T: fmt::Debug> Debug for Values<T>`

- <span id="values-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Values<T>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Values<T>`

- <span id="values-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Values<T>`

##### `impl<I> IntoIterator for Values<T>`

- <span id="values-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-type-intoiter"></span>`type IntoIter = I`

- <span id="values-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Values<T>`

- <span id="values-type-item"></span>`type Item = T`

- <span id="values-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ValuesRef<'a, T>`

```rust
struct ValuesRef<'a, T> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<self::any_value::AnyValue>>>, fn(&self::any_value::AnyValue) -> &T>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1496-1500`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1496-L1500)*

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

##### `impl<'a, T: clone::Clone> Clone for ValuesRef<'a, T>`

- <span id="valuesref-clone"></span>`fn clone(&self) -> ValuesRef<'a, T>` — [`ValuesRef`](#valuesref)

##### `impl<'a, T: fmt::Debug> Debug for ValuesRef<'a, T>`

- <span id="valuesref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: 'a> Default for ValuesRef<'a, T>`

- <span id="valuesref-default"></span>`fn default() -> Self`

##### `impl<'a, T: 'a> DoubleEndedIterator for ValuesRef<'a, T>`

- <span id="valuesref-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a> ExactSizeIterator for ValuesRef<'a, T>`

##### `impl<I> IntoIterator for ValuesRef<'a, T>`

- <span id="valuesref-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesref-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T: 'a> Iterator for ValuesRef<'a, T>`

- <span id="valuesref-type-item"></span>`type Item = &'a T`

- <span id="valuesref-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawValues<'a>`

```rust
struct RawValues<'a> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<std::ffi::OsString>>>, fn(&std::ffi::OsString) -> &std::ffi::OsStr>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1569-1573`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1569-L1573)*

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

##### `impl Clone for RawValues<'a>`

- <span id="rawvalues-clone"></span>`fn clone(&self) -> RawValues<'a>` — [`RawValues`](#rawvalues)

##### `impl Debug for RawValues<'a>`

- <span id="rawvalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawValues<'_>`

- <span id="rawvalues-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for RawValues<'a>`

- <span id="rawvalues-next-back"></span>`fn next_back(&mut self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator for RawValues<'_>`

##### `impl IntoIterator for RawValues<'a>`

- <span id="rawvalues-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawvalues-type-intoiter"></span>`type IntoIter = I`

- <span id="rawvalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawValues<'a>`

- <span id="rawvalues-type-item"></span>`type Item = &'a OsStr`

- <span id="rawvalues-next"></span>`fn next(&mut self) -> Option<&'a OsStr>`

- <span id="rawvalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Occurrences<T>`

```rust
struct Occurrences<T> {
    iter: std::iter::Map<std::vec::IntoIter<Vec<self::any_value::AnyValue>>, fn(Vec<self::any_value::AnyValue>) -> OccurrenceValues<T>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1621-1624`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1621-L1624)*

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Occurrences<T>`

- <span id="occurrences-clone"></span>`fn clone(&self) -> Occurrences<T>` — [`Occurrences`](#occurrences)

##### `impl<T: fmt::Debug> Debug for Occurrences<T>`

- <span id="occurrences-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Occurrences<T>`

- <span id="occurrences-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Occurrences<T>`

- <span id="occurrences-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Occurrences<T>`

##### `impl<I> IntoIterator for Occurrences<T>`

- <span id="occurrences-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="occurrences-type-intoiter"></span>`type IntoIter = I`

- <span id="occurrences-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Occurrences<T>`

- <span id="occurrences-type-item"></span>`type Item = OccurrenceValues<T>`

- <span id="occurrences-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="occurrences-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `OccurrenceValues<T>`

```rust
struct OccurrenceValues<T> {
    iter: std::iter::Map<std::vec::IntoIter<self::any_value::AnyValue>, fn(self::any_value::AnyValue) -> T>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1656-1659`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1656-L1659)*

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for OccurrenceValues<T>`

- <span id="occurrencevalues-clone"></span>`fn clone(&self) -> OccurrenceValues<T>` — [`OccurrenceValues`](#occurrencevalues)

##### `impl<T: fmt::Debug> Debug for OccurrenceValues<T>`

- <span id="occurrencevalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for OccurrenceValues<T>`

- <span id="occurrencevalues-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for OccurrenceValues<T>`

##### `impl<I> IntoIterator for OccurrenceValues<T>`

- <span id="occurrencevalues-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="occurrencevalues-type-intoiter"></span>`type IntoIter = I`

- <span id="occurrencevalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for OccurrenceValues<T>`

- <span id="occurrencevalues-type-item"></span>`type Item = T`

- <span id="occurrencevalues-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="occurrencevalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `OccurrencesRef<'a, T>`

```rust
struct OccurrencesRef<'a, T> {
    iter: std::iter::Map<std::slice::Iter<'a, Vec<self::any_value::AnyValue>>, fn(&Vec<self::any_value::AnyValue>) -> OccurrenceValuesRef<'_, T>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1682-1685`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1682-L1685)*

#### Trait Implementations

##### `impl<'a, T: clone::Clone> Clone for OccurrencesRef<'a, T>`

- <span id="occurrencesref-clone"></span>`fn clone(&self) -> OccurrencesRef<'a, T>` — [`OccurrencesRef`](#occurrencesref)

##### `impl<'a, T: fmt::Debug> Debug for OccurrencesRef<'a, T>`

- <span id="occurrencesref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for OccurrencesRef<'_, T>`

- <span id="occurrencesref-default"></span>`fn default() -> Self`

##### `impl<'a, T> DoubleEndedIterator for OccurrencesRef<'a, T>`

- <span id="occurrencesref-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for OccurrencesRef<'a, T>`

##### `impl<I> IntoIterator for OccurrencesRef<'a, T>`

- <span id="occurrencesref-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="occurrencesref-type-intoiter"></span>`type IntoIter = I`

- <span id="occurrencesref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for OccurrencesRef<'a, T>`

- <span id="occurrencesref-type-item"></span>`type Item = OccurrenceValuesRef<'a, T>`

- <span id="occurrencesref-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="occurrencesref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `OccurrenceValuesRef<'a, T>`

```rust
struct OccurrenceValuesRef<'a, T> {
    iter: std::iter::Map<std::slice::Iter<'a, self::any_value::AnyValue>, fn(&self::any_value::AnyValue) -> &T>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1722-1725`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1722-L1725)*

#### Trait Implementations

##### `impl<'a, T: clone::Clone> Clone for OccurrenceValuesRef<'a, T>`

- <span id="occurrencevaluesref-clone"></span>`fn clone(&self) -> OccurrenceValuesRef<'a, T>` — [`OccurrenceValuesRef`](#occurrencevaluesref)

##### `impl<'a, T: fmt::Debug> Debug for OccurrenceValuesRef<'a, T>`

- <span id="occurrencevaluesref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T> DoubleEndedIterator for OccurrenceValuesRef<'a, T>`

- <span id="occurrencevaluesref-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T> ExactSizeIterator for OccurrenceValuesRef<'a, T>`

##### `impl<I> IntoIterator for OccurrenceValuesRef<'a, T>`

- <span id="occurrencevaluesref-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="occurrencevaluesref-type-intoiter"></span>`type IntoIter = I`

- <span id="occurrencevaluesref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T> Iterator for OccurrenceValuesRef<'a, T>`

- <span id="occurrencevaluesref-type-item"></span>`type Item = &'a T`

- <span id="occurrencevaluesref-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="occurrencevaluesref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawOccurrences<'a>`

```rust
struct RawOccurrences<'a> {
    iter: std::iter::Map<std::slice::Iter<'a, Vec<std::ffi::OsString>>, fn(&Vec<std::ffi::OsString>) -> RawOccurrenceValues<'_>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1754-1757`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1754-L1757)*

#### Trait Implementations

##### `impl Clone for RawOccurrences<'a>`

- <span id="rawoccurrences-clone"></span>`fn clone(&self) -> RawOccurrences<'a>` — [`RawOccurrences`](#rawoccurrences)

##### `impl Debug for RawOccurrences<'a>`

- <span id="rawoccurrences-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawOccurrences<'_>`

- <span id="rawoccurrences-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for RawOccurrences<'_>`

- <span id="rawoccurrences-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for RawOccurrences<'_>`

##### `impl IntoIterator for RawOccurrences<'a>`

- <span id="rawoccurrences-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawoccurrences-type-intoiter"></span>`type IntoIter = I`

- <span id="rawoccurrences-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawOccurrences<'a>`

- <span id="rawoccurrences-type-item"></span>`type Item = RawOccurrenceValues<'a>`

- <span id="rawoccurrences-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rawoccurrences-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawOccurrenceValues<'a>`

```rust
struct RawOccurrenceValues<'a> {
    iter: std::iter::Map<std::slice::Iter<'a, std::ffi::OsString>, fn(&std::ffi::OsString) -> &std::ffi::OsStr>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1789-1792`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1789-L1792)*

#### Trait Implementations

##### `impl Clone for RawOccurrenceValues<'a>`

- <span id="rawoccurrencevalues-clone"></span>`fn clone(&self) -> RawOccurrenceValues<'a>` — [`RawOccurrenceValues`](#rawoccurrencevalues)

##### `impl Debug for RawOccurrenceValues<'a>`

- <span id="rawoccurrencevalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for RawOccurrenceValues<'a>`

- <span id="rawoccurrencevalues-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl ExactSizeIterator for RawOccurrenceValues<'_>`

##### `impl IntoIterator for RawOccurrenceValues<'a>`

- <span id="rawoccurrencevalues-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawoccurrencevalues-type-intoiter"></span>`type IntoIter = I`

- <span id="rawoccurrencevalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawOccurrenceValues<'a>`

- <span id="rawoccurrencevalues-type-item"></span>`type Item = &'a OsStr`

- <span id="rawoccurrencevalues-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rawoccurrencevalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Indices<'a>`

```rust
struct Indices<'a> {
    iter: std::iter::Cloned<std::slice::Iter<'a, usize>>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1842-1845`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1842-L1845)*

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

##### `impl Clone for Indices<'a>`

- <span id="indices-clone"></span>`fn clone(&self) -> Indices<'a>` — [`Indices`](#indices)

##### `impl Debug for Indices<'a>`

- <span id="indices-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Indices<'_>`

- <span id="indices-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Indices<'_>`

- <span id="indices-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl ExactSizeIterator for Indices<'_>`

##### `impl IntoIterator for Indices<'a>`

- <span id="indices-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="indices-type-intoiter"></span>`type IntoIter = I`

- <span id="indices-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Indices<'_>`

- <span id="indices-type-item"></span>`type Item = usize`

- <span id="indices-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="indices-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Functions

### `unwrap_downcast_ref`

```rust
fn unwrap_downcast_ref<T: Any + Clone + Send + Sync + 'static>(value: &self::any_value::AnyValue) -> &T
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1889-1891`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1889-L1891)*

### `unwrap_downcast_into`

```rust
fn unwrap_downcast_into<T: Any + Clone + Send + Sync + 'static>(value: self::any_value::AnyValue) -> T
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1894-1896`](../../../../../.source_1765210505/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1894-L1896)*

