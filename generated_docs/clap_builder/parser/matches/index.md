*[clap_builder](../../index.md) / [parser](../index.md) / [matches](index.md)*

---

# Module `matches`

## Contents

- [Modules](#modules)
  - [`arg_matches`](#arg-matches)
  - [`matched_arg`](#matched-arg)
  - [`value_source`](#value-source)
- [Structs](#structs)
  - [`IdsRef`](#idsref)
  - [`RawValues`](#rawvalues)
  - [`Values`](#values)
  - [`ValuesRef`](#valuesref)
  - [`ArgMatches`](#argmatches)
  - [`Indices`](#indices)
- [Enums](#enums)
  - [`ValueSource`](#valuesource)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arg_matches`](#arg-matches) | mod |  |
| [`matched_arg`](#matched-arg) | mod |  |
| [`value_source`](#value-source) | mod |  |
| [`IdsRef`](#idsref) | struct |  |
| [`RawValues`](#rawvalues) | struct |  |
| [`Values`](#values) | struct |  |
| [`ValuesRef`](#valuesref) | struct |  |
| [`ArgMatches`](#argmatches) | struct |  |
| [`Indices`](#indices) | struct |  |
| [`ValueSource`](#valuesource) | enum |  |

## Modules

- [`arg_matches`](arg_matches/index.md)
- [`matched_arg`](matched_arg/index.md)
- [`value_source`](value_source/index.md)

## Structs

### `IdsRef<'a>`

```rust
struct IdsRef<'a> {
    iter: std::slice::Iter<'a, crate::util::Id>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1384-1386`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1384-L1386)*

Iterate over `Arg` and `ArgGroup` [`Id`](../../util/id/index.md)s via `ArgMatches::ids`.

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

- <span id="idsref-clone"></span>`fn clone(&self) -> IdsRef<'a>` — [`IdsRef`](arg_matches/index.md#idsref)

##### `impl Debug for IdsRef<'a>`

- <span id="idsref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IdsRef<'a>`

- <span id="idsref-next-back"></span>`fn next_back(&mut self) -> Option<&'a Id>` — [`Id`](../../util/id/index.md#id)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl IntoIterator for IdsRef<'a>`

- <span id="idsref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="idsref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="idsref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IdsRef<'a>`

- <span id="idsref-iterator-type-item"></span>`type Item = &'a Id`

- <span id="idsref-next"></span>`fn next(&mut self) -> Option<&'a Id>` — [`Id`](../../util/id/index.md#id)

- <span id="idsref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawValues<'a>`

```rust
struct RawValues<'a> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<std::ffi::OsString>>>, fn(&std::ffi::OsString) -> &std::ffi::OsStr>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1569-1573`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1569-L1573)*

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

- <span id="rawvalues-clone"></span>`fn clone(&self) -> RawValues<'a>` — [`RawValues`](arg_matches/index.md#rawvalues)

##### `impl Debug for RawValues<'a>`

- <span id="rawvalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawValues<'_>`

- <span id="rawvalues-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for RawValues<'a>`

- <span id="rawvalues-next-back"></span>`fn next_back(&mut self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator for RawValues<'_>`

##### `impl IntoIterator for RawValues<'a>`

- <span id="rawvalues-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawvalues-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawvalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawValues<'a>`

- <span id="rawvalues-iterator-type-item"></span>`type Item = &'a OsStr`

- <span id="rawvalues-next"></span>`fn next(&mut self) -> Option<&'a OsStr>`

- <span id="rawvalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Values<T>`

```rust
struct Values<T> {
    iter: std::iter::Map<std::iter::Flatten<std::vec::IntoIter<Vec<self::any_value::AnyValue>>>, fn(self::any_value::AnyValue) -> T>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1428-1432`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1428-L1432)*

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

- <span id="values-clone"></span>`fn clone(&self) -> Values<T>` — [`Values`](arg_matches/index.md#values)

##### `impl<T: fmt::Debug> Debug for Values<T>`

- <span id="values-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Values<T>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Values<T>`

- <span id="values-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Values<T>`

##### `impl IntoIterator for Values<T>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Values<T>`

- <span id="values-iterator-type-item"></span>`type Item = T`

- <span id="values-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ValuesRef<'a, T>`

```rust
struct ValuesRef<'a, T> {
    iter: std::iter::Map<std::iter::Flatten<std::slice::Iter<'a, Vec<self::any_value::AnyValue>>>, fn(&self::any_value::AnyValue) -> &T>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1496-1500`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1496-L1500)*

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

##### `impl<T: clone::Clone> Clone for ValuesRef<'a, T>`

- <span id="valuesref-clone"></span>`fn clone(&self) -> ValuesRef<'a, T>` — [`ValuesRef`](arg_matches/index.md#valuesref)

##### `impl<T: fmt::Debug> Debug for ValuesRef<'a, T>`

- <span id="valuesref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a> Default for ValuesRef<'a, T>`

- <span id="valuesref-default"></span>`fn default() -> Self`

##### `impl<T: 'a> DoubleEndedIterator for ValuesRef<'a, T>`

- <span id="valuesref-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: 'a> ExactSizeIterator for ValuesRef<'a, T>`

##### `impl IntoIterator for ValuesRef<'a, T>`

- <span id="valuesref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a> Iterator for ValuesRef<'a, T>`

- <span id="valuesref-iterator-type-item"></span>`type Item = &'a T`

- <span id="valuesref-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ArgMatches`

```rust
struct ArgMatches {
    valid_args: Vec<crate::util::Id>,
    valid_subcommands: Vec<crate::builder::Str>,
    args: self::flat_map::FlatMap<crate::util::Id, matched_arg::MatchedArg>,
    subcommand: Option<Box<SubCommand>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:67-74`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L67-L74)*

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

- <span id="argmatches-get-many"></span>`fn get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<ValuesRef<'_, T>>` — [`ValuesRef`](arg_matches/index.md#valuesref)

- <span id="argmatches-get-occurrences"></span>`fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<OccurrencesRef<'_, T>>` — [`OccurrencesRef`](arg_matches/index.md#occurrencesref)

- <span id="argmatches-get-raw"></span>`fn get_raw(&self, id: &str) -> Option<RawValues<'_>>` — [`RawValues`](arg_matches/index.md#rawvalues)

- <span id="argmatches-get-raw-occurrences"></span>`fn get_raw_occurrences(&self, id: &str) -> Option<RawOccurrences<'_>>` — [`RawOccurrences`](arg_matches/index.md#rawoccurrences)

- <span id="argmatches-remove-one"></span>`fn remove_one<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<T>`

- <span id="argmatches-remove-many"></span>`fn remove_many<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Values<T>>` — [`Values`](arg_matches/index.md#values)

- <span id="argmatches-remove-occurrences"></span>`fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Occurrences<T>>` — [`Occurrences`](arg_matches/index.md#occurrences)

- <span id="argmatches-contains-id"></span>`fn contains_id(&self, id: &str) -> bool`

- <span id="argmatches-ids"></span>`fn ids(&self) -> IdsRef<'_>` — [`IdsRef`](arg_matches/index.md#idsref)

- <span id="argmatches-args-present"></span>`fn args_present(&self) -> bool`

- <span id="argmatches-value-source"></span>`fn value_source(&self, id: &str) -> Option<ValueSource>` — [`ValueSource`](value_source/index.md#valuesource)

- <span id="argmatches-index-of"></span>`fn index_of(&self, id: &str) -> Option<usize>`

- <span id="argmatches-indices-of"></span>`fn indices_of(&self, id: &str) -> Option<Indices<'_>>` — [`Indices`](arg_matches/index.md#indices)

#### Trait Implementations

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl Debug for ArgMatches`

- <span id="argmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl StructuralPartialEq for ArgMatches`

### `Indices<'a>`

```rust
struct Indices<'a> {
    iter: std::iter::Cloned<std::slice::Iter<'a, usize>>,
    len: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/arg_matches.rs:1842-1845`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/arg_matches.rs#L1842-L1845)*

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

- <span id="indices-clone"></span>`fn clone(&self) -> Indices<'a>` — [`Indices`](arg_matches/index.md#indices)

##### `impl Debug for Indices<'a>`

- <span id="indices-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Indices<'_>`

- <span id="indices-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Indices<'_>`

- <span id="indices-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl ExactSizeIterator for Indices<'_>`

##### `impl IntoIterator for Indices<'a>`

- <span id="indices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="indices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="indices-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Indices<'_>`

- <span id="indices-iterator-type-item"></span>`type Item = usize`

- <span id="indices-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="indices-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `ValueSource`

```rust
enum ValueSource {
    DefaultValue,
    EnvVariable,
    CommandLine,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/matches/value_source.rs:4-11`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/matches/value_source.rs#L4-L11)*

Origin of the argument's value

#### Variants

- **`DefaultValue`**

  Value came `Arg::default_value`

- **`EnvVariable`**

  Value came `Arg::env`

- **`CommandLine`**

  Value was passed in on the command-line

#### Implementations

- <span id="valuesource-is-explicit"></span>`fn is_explicit(self) -> bool`

#### Trait Implementations

##### `impl Clone for ValueSource`

- <span id="valuesource-clone"></span>`fn clone(&self) -> ValueSource` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- <span id="valuesource-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueSource`

##### `impl Ord for ValueSource`

- <span id="valuesource-cmp"></span>`fn cmp(&self, other: &ValueSource) -> cmp::Ordering` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl PartialEq for ValueSource`

- <span id="valuesource-eq"></span>`fn eq(&self, other: &ValueSource) -> bool` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl PartialOrd for ValueSource`

- <span id="valuesource-partial-cmp"></span>`fn partial_cmp(&self, other: &ValueSource) -> option::Option<cmp::Ordering>` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl StructuralPartialEq for ValueSource`

