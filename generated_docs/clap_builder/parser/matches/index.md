*[clap_builder](../../index.md) / [parser](../index.md) / [matches](index.md)*

---

# Module `matches`

## Contents

- [Modules](#modules)
  - [`arg_matches`](#arg_matches)
  - [`matched_arg`](#matched_arg)
  - [`value_source`](#value_source)
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
| [`arg_matches`](#arg_matches) | mod |  |
| [`matched_arg`](#matched_arg) | mod |  |
| [`value_source`](#value_source) | mod |  |
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

Iterate over `Arg` and `ArgGroup` [`Id`](../../index.md)s via `ArgMatches::ids`.

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

- <span id="idsref-clone"></span>`fn clone(&self) -> IdsRef<'a>` — [`IdsRef`](../index.md)

##### `impl<'a> Debug for IdsRef<'a>`

- <span id="idsref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> DoubleEndedIterator for IdsRef<'a>`

- <span id="idsref-next-back"></span>`fn next_back(&mut self) -> Option<&'a Id>` — [`Id`](../../index.md)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl<I> IntoIterator for IdsRef<'a>`

- <span id="idsref-item"></span>`type Item = <I as Iterator>::Item`

- <span id="idsref-intoiter"></span>`type IntoIter = I`

- <span id="idsref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for IdsRef<'a>`

- <span id="idsref-item"></span>`type Item = &'a Id`

- <span id="idsref-next"></span>`fn next(&mut self) -> Option<&'a Id>` — [`Id`](../../index.md)

- <span id="idsref-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="rawvalues-clone"></span>`fn clone(&self) -> RawValues<'a>` — [`RawValues`](../index.md)

##### `impl<'a> Debug for RawValues<'a>`

- <span id="rawvalues-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawValues<'_>`

- <span id="rawvalues-default"></span>`fn default() -> Self`

##### `impl<'a> DoubleEndedIterator for RawValues<'a>`

- <span id="rawvalues-next-back"></span>`fn next_back(&mut self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator for RawValues<'_>`

##### `impl<I> IntoIterator for RawValues<'a>`

- <span id="rawvalues-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawvalues-intoiter"></span>`type IntoIter = I`

- <span id="rawvalues-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for RawValues<'a>`

- <span id="rawvalues-item"></span>`type Item = &'a OsStr`

- <span id="rawvalues-next"></span>`fn next(&mut self) -> Option<&'a OsStr>`

- <span id="rawvalues-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

##### `impl<T: clone::Clone> Clone for Values<T>`

- <span id="values-clone"></span>`fn clone(&self) -> Values<T>` — [`Values`](../index.md)

##### `impl<T: fmt::Debug> Debug for Values<T>`

- <span id="values-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Values<T>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Values<T>`

- <span id="values-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Values<T>`

##### `impl<I> IntoIterator for Values<T>`

- <span id="values-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiter"></span>`type IntoIter = I`

- <span id="values-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Values<T>`

- <span id="values-item"></span>`type Item = T`

- <span id="values-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

##### `impl<'a, T: clone::Clone> Clone for ValuesRef<'a, T>`

- <span id="valuesref-clone"></span>`fn clone(&self) -> ValuesRef<'a, T>` — [`ValuesRef`](../index.md)

##### `impl<'a, T: fmt::Debug> Debug for ValuesRef<'a, T>`

- <span id="valuesref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: 'a> Default for ValuesRef<'a, T>`

- <span id="valuesref-default"></span>`fn default() -> Self`

##### `impl<'a, T: 'a> DoubleEndedIterator for ValuesRef<'a, T>`

- <span id="valuesref-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a> ExactSizeIterator for ValuesRef<'a, T>`

##### `impl<I> IntoIterator for ValuesRef<'a, T>`

- <span id="valuesref-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesref-intoiter"></span>`type IntoIter = I`

- <span id="valuesref-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, T: 'a> Iterator for ValuesRef<'a, T>`

- <span id="valuesref-item"></span>`type Item = &'a T`

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

- <span id="argmatches-subcommand"></span>`fn subcommand(&self) -> Option<(&str, &ArgMatches)>` — [`ArgMatches`](../../index.md)

- <span id="argmatches-remove-subcommand"></span>`fn remove_subcommand(&mut self) -> Option<(String, ArgMatches)>` — [`ArgMatches`](../../index.md)

- <span id="argmatches-subcommand-matches"></span>`fn subcommand_matches(&self, name: &str) -> Option<&ArgMatches>` — [`ArgMatches`](../../index.md)

- <span id="argmatches-subcommand-name"></span>`fn subcommand_name(&self) -> Option<&str>`

#### Trait Implementations

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](../../index.md)

##### `impl Debug for ArgMatches`

- <span id="argmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](../../index.md)

##### `impl Eq for ArgMatches`

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](../../index.md)

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

- <span id="indices-clone"></span>`fn clone(&self) -> Indices<'a>` — [`Indices`](../index.md)

##### `impl<'a> Debug for Indices<'a>`

- <span id="indices-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Indices<'_>`

- <span id="indices-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Indices<'_>`

- <span id="indices-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl ExactSizeIterator for Indices<'_>`

##### `impl<I> IntoIterator for Indices<'a>`

- <span id="indices-item"></span>`type Item = <I as Iterator>::Item`

- <span id="indices-intoiter"></span>`type IntoIter = I`

- <span id="indices-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Indices<'_>`

- <span id="indices-item"></span>`type Item = usize`

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

- <span id="valuesource-clone"></span>`fn clone(&self) -> ValueSource` — [`ValueSource`](../index.md)

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- <span id="valuesource-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueSource`

##### `impl Ord for ValueSource`

- <span id="valuesource-cmp"></span>`fn cmp(&self, other: &ValueSource) -> cmp::Ordering` — [`ValueSource`](../index.md)

##### `impl PartialEq for ValueSource`

- <span id="valuesource-eq"></span>`fn eq(&self, other: &ValueSource) -> bool` — [`ValueSource`](../index.md)

##### `impl PartialOrd for ValueSource`

- <span id="valuesource-partial-cmp"></span>`fn partial_cmp(&self, other: &ValueSource) -> option::Option<cmp::Ordering>` — [`ValueSource`](../index.md)

##### `impl StructuralPartialEq for ValueSource`

