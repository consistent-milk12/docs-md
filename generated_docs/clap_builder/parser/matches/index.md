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

##### `impl Any for IdsRef<'a>`

- <span id="idsref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IdsRef<'a>`

- <span id="idsref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IdsRef<'a>`

- <span id="idsref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for IdsRef<'a>`

- <span id="idsref-clone"></span>`fn clone(&self) -> IdsRef<'a>` — [`IdsRef`](arg_matches/index.md#idsref)

##### `impl CloneToUninit for IdsRef<'a>`

- <span id="idsref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for IdsRef<'a>`

- <span id="idsref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for IdsRef<'a>`

- <span id="idsref-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<&'a Id>` — [`Id`](../../util/id/index.md#id)

##### `impl ExactSizeIterator for IdsRef<'_>`

##### `impl<T> From for IdsRef<'a>`

- <span id="idsref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IdsRef<'a>`

- <span id="idsref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IdsRef<'a>`

- <span id="idsref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="idsref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="idsref-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IdsRef<'a>`

- <span id="idsref-iterator-type-item"></span>`type Item = &'a Id`

- <span id="idsref-iterator-next"></span>`fn next(&mut self) -> Option<&'a Id>` — [`Id`](../../util/id/index.md#id)

- <span id="idsref-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for IdsRef<'a>`

- <span id="idsref-toowned-type-owned"></span>`type Owned = T`

- <span id="idsref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="idsref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IdsRef<'a>`

- <span id="idsref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="idsref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IdsRef<'a>`

- <span id="idsref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="idsref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for RawValues<'a>`

- <span id="rawvalues-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawValues<'a>`

- <span id="rawvalues-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawValues<'a>`

- <span id="rawvalues-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RawValues<'a>`

- <span id="rawvalues-clone"></span>`fn clone(&self) -> RawValues<'a>` — [`RawValues`](arg_matches/index.md#rawvalues)

##### `impl CloneToUninit for RawValues<'a>`

- <span id="rawvalues-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RawValues<'a>`

- <span id="rawvalues-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawValues<'_>`

- <span id="rawvalues-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for RawValues<'a>`

- <span id="rawvalues-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<&'a OsStr>`

##### `impl ExactSizeIterator for RawValues<'_>`

##### `impl<T> From for RawValues<'a>`

- <span id="rawvalues-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawValues<'a>`

- <span id="rawvalues-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RawValues<'a>`

- <span id="rawvalues-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawvalues-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawvalues-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawValues<'a>`

- <span id="rawvalues-iterator-type-item"></span>`type Item = &'a OsStr`

- <span id="rawvalues-iterator-next"></span>`fn next(&mut self) -> Option<&'a OsStr>`

- <span id="rawvalues-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for RawValues<'a>`

- <span id="rawvalues-toowned-type-owned"></span>`type Owned = T`

- <span id="rawvalues-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawvalues-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawValues<'a>`

- <span id="rawvalues-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawvalues-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawValues<'a>`

- <span id="rawvalues-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawvalues-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl<T> Any for Values<T>`

- <span id="values-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Values<T>`

- <span id="values-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Values<T>`

- <span id="values-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Values<T>`

- <span id="values-clone"></span>`fn clone(&self) -> Values<T>` — [`Values`](arg_matches/index.md#values)

##### `impl<T> CloneToUninit for Values<T>`

- <span id="values-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Values<T>`

- <span id="values-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Values<T>`

- <span id="values-default"></span>`fn default() -> Self`

##### `impl<T> DoubleEndedIterator for Values<T>`

- <span id="values-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Values<T>`

##### `impl<T> From for Values<T>`

- <span id="values-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Values<T>`

- <span id="values-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Values<T>`

- <span id="values-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="values-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="values-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Values<T>`

- <span id="values-iterator-type-item"></span>`type Item = T`

- <span id="values-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="values-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToOwned for Values<T>`

- <span id="values-toowned-type-owned"></span>`type Owned = T`

- <span id="values-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="values-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Values<T>`

- <span id="values-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="values-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Values<T>`

- <span id="values-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="values-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl<T> Any for ValuesRef<'a, T>`

- <span id="valuesref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValuesRef<'a, T>`

- <span id="valuesref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValuesRef<'a, T>`

- <span id="valuesref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for ValuesRef<'a, T>`

- <span id="valuesref-clone"></span>`fn clone(&self) -> ValuesRef<'a, T>` — [`ValuesRef`](arg_matches/index.md#valuesref)

##### `impl<T> CloneToUninit for ValuesRef<'a, T>`

- <span id="valuesref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for ValuesRef<'a, T>`

- <span id="valuesref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a> Default for ValuesRef<'a, T>`

- <span id="valuesref-default"></span>`fn default() -> Self`

##### `impl<T: 'a> DoubleEndedIterator for ValuesRef<'a, T>`

- <span id="valuesref-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: 'a> ExactSizeIterator for ValuesRef<'a, T>`

##### `impl<T> From for ValuesRef<'a, T>`

- <span id="valuesref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ValuesRef<'a, T>`

- <span id="valuesref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ValuesRef<'a, T>`

- <span id="valuesref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="valuesref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="valuesref-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a> Iterator for ValuesRef<'a, T>`

- <span id="valuesref-iterator-type-item"></span>`type Item = &'a T`

- <span id="valuesref-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="valuesref-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToOwned for ValuesRef<'a, T>`

- <span id="valuesref-toowned-type-owned"></span>`type Owned = T`

- <span id="valuesref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuesref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ValuesRef<'a, T>`

- <span id="valuesref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuesref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ValuesRef<'a, T>`

- <span id="valuesref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuesref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Gets the value of a specific option or positional argument.

  

  i.e. an argument that `takes an additional value` at runtime.

  

  Returns an error if the wrong type was used.

  

  Returns `None` if the option wasn't present.

  

  <div class="warning">

  

  *NOTE:* This will always return `Some(value)` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_one`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("port")

          .value_parser(value_parser!(usize))

          .action(ArgAction::Set)

          .required(true))

      .get_matches_from(vec!["myapp", "2020"]);

  

  let port: usize = *m

      .get_one("port")

      .expect("`port`is required");

  assert_eq!(port, 2020);

  ```

  

- <span id="argmatches-get-count"></span>`fn get_count(&self, id: &str) -> u8`

  Gets the value of a specific `ArgAction::Count` flag

  

  # Panic

  

  If the argument's action is not `ArgAction::Count`

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use clap::Arg;

  let cmd = Command::new("mycmd")

      .arg(

          Arg::new("flag")

              .long("flag")

              .action(clap::ArgAction::Count)

      );

  

  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag", "--flag"]).unwrap();

  assert_eq!(

      matches.get_count("flag"),

      2

  );

  ```

- <span id="argmatches-get-flag"></span>`fn get_flag(&self, id: &str) -> bool`

  Gets the value of a specific `ArgAction::SetTrue` or `ArgAction::SetFalse` flag

  

  # Panic

  

  If the argument's action is not `ArgAction::SetTrue` or `ArgAction::SetFalse`

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use clap::Arg;

  let cmd = Command::new("mycmd")

      .arg(

          Arg::new("flag")

              .long("flag")

              .action(clap::ArgAction::SetTrue)

      );

  

  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();

  assert!(matches.contains_id("flag"));

  assert_eq!(

      matches.get_flag("flag"),

      true

  );

  ```

- <span id="argmatches-get-many"></span>`fn get_many<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<ValuesRef<'_, T>>` — [`ValuesRef`](arg_matches/index.md#valuesref)

  Iterate over values of a specific option or positional argument.

  

  i.e. an argument that takes multiple values at runtime.

  

  Returns an error if the wrong type was used.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_many`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("ports")

          .action(ArgAction::Append)

          .value_parser(value_parser!(usize))

          .short('p')

          .required(true))

      .get_matches_from(vec![

          "myprog", "-p", "22", "-p", "80", "-p", "2020"

      ]);

  let vals: Vec<usize> = m.get_many("ports")

      .expect("`port`is required")

      .copied()

      .collect();

  assert_eq!(vals, [22, 80, 2020]);

  ```

- <span id="argmatches-get-occurrences"></span>`fn get_occurrences<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<OccurrencesRef<'_, T>>` — [`OccurrencesRef`](arg_matches/index.md#occurrencesref)

  Iterate over the values passed to each occurrence of an option.

  

  Each item is itself an iterator containing the arguments passed to a single occurrence

  of the option.

  

  If the option doesn't support multiple occurrences, or there was only a single occurrence,

  the iterator will only contain a single item.

  

  Returns `None` if the option wasn't present.

  

  # Panics

  

  If the argument definition and access mismatch (debug builds). To handle this case programmatically, see

  `ArgMatches::try_get_occurrences`.

  

  # Examples

  ```rust

  use clap_builder as clap;

  use clap::{Command,Arg, ArgAction, value_parser};

  let m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(String)))

      .get_matches_from(vec![

          "myprog", "-x", "a", "b", "-x", "c", "d"]);

  let vals: Vec<Vec<&String>> = m.get_occurrences("x").unwrap().map(Iterator::collect).collect();

  assert_eq!(vals, [["a", "b"], ["c", "d"]]);

  ```

- <span id="argmatches-get-raw"></span>`fn get_raw(&self, id: &str) -> Option<RawValues<'_>>` — [`RawValues`](arg_matches/index.md#rawvalues)

  Iterate over the original argument values.

  

  An `OsStr` on Unix-like systems is any series of bytes, regardless of whether or not they

  contain valid UTF-8. Since [`String`](../../index.md)s in Rust are guaranteed to be valid UTF-8, a valid

  filename on a Unix system as an argument value may contain invalid UTF-8.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_raw`.

  

  # Examples

  

  ```rust

  #[cfg(unix)] {

  use clap_builder as clap;

  use clap::{Command, arg, value_parser};

  use std::ffi::{OsStr,OsString};

  use std::os::unix::ffi::{OsStrExt,OsStringExt};

  use std::path::PathBuf;

  

  let m = Command::new("utf8")

      .arg(arg!(<arg> ... "some arg").value_parser(value_parser!(PathBuf)))

      .get_matches_from(vec![OsString::from("myprog"),

                                  // "Hi"

                                  OsString::from_vec(vec![b'H', b'i']),

                                  // "{0xe9}!"

                                  OsString::from_vec(vec![0xe9, b'!'])]);

  

  let mut itr = m.get_raw("arg")

      .expect("`port`is required")

      .into_iter();

  assert_eq!(itr.next(), Some(OsStr::new("Hi")));

  assert_eq!(itr.next(), Some(OsStr::from_bytes(&[0xe9, b'!'])));

  assert_eq!(itr.next(), None);

  }

  ```

  

  

- <span id="argmatches-get-raw-occurrences"></span>`fn get_raw_occurrences(&self, id: &str) -> Option<RawOccurrences<'_>>` — [`RawOccurrences`](arg_matches/index.md#rawoccurrences)

  Iterate over the original values for each occurrence of an option.

  

  Similar to `ArgMatches::get_occurrences` but returns raw values.

  

  An `OsStr` on Unix-like systems is any series of bytes, regardless of whether or not they

  contain valid UTF-8. Since [`String`](../../index.md)s in Rust are guaranteed to be valid UTF-8, a valid

  filename on a Unix system as an argument value may contain invalid UTF-8.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_get_raw_occurrences`.

  

  # Examples

  

  ```rust

  #[cfg(unix)] {

  use clap_builder as clap;

  use clap::{Command, arg, value_parser, ArgAction, Arg};

  use std::ffi::{OsStr,OsString};

  use std::os::unix::ffi::{OsStrExt,OsStringExt};

  use std::path::PathBuf;

  

  let m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(PathBuf)))

      .get_matches_from(vec![OsString::from("myprog"),

                              OsString::from("-x"),

                              OsString::from("a"), OsString::from("b"),

                              OsString::from("-x"),

                              OsString::from("c"),

                              // "{0xe9}!"

                              OsString::from_vec(vec![0xe9, b'!'])]);

  let mut itr = m.get_raw_occurrences("x")

      .expect("`-x`is required")

      .map(Iterator::collect::<Vec<_>>);

  assert_eq!(itr.next(), Some(vec![OsStr::new("a"), OsStr::new("b")]));

  assert_eq!(itr.next(), Some(vec![OsStr::new("c"), OsStr::from_bytes(&[0xe9, b'!'])]));

  assert_eq!(itr.next(), None);

  }

  ```

  

  

- <span id="argmatches-remove-one"></span>`fn remove_one<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<T>`

  Returns the value of a specific option or positional argument.

  

  i.e. an argument that `takes an additional value` at runtime.

  

  Returns an error if the wrong type was used.  No item will have been removed.

  

  Returns `None` if the option wasn't present.

  

  <div class="warning">

  

  *NOTE:* This will always return `Some(value)` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_one`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("file")

          .required(true)

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "myprog", "file.txt",

      ]);

  let vals: String = m.remove_one("file")

      .expect("`file`is required");

  assert_eq!(vals, "file.txt");

  ```

  

- <span id="argmatches-remove-many"></span>`fn remove_many<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Values<T>>` — [`Values`](arg_matches/index.md#values)

  Return values of a specific option or positional argument.

  

  i.e. an argument that takes multiple values at runtime.

  

  Returns an error if the wrong type was used.  No item will have been removed.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_many`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("file")

          .action(ArgAction::Append)

          .num_args(1..)

          .required(true))

      .get_matches_from(vec![

          "myprog", "file1.txt", "file2.txt", "file3.txt", "file4.txt",

      ]);

  let vals: Vec<String> = m.remove_many("file")

      .expect("`file`is required")

      .collect();

  assert_eq!(vals, ["file1.txt", "file2.txt", "file3.txt", "file4.txt"]);

  ```

- <span id="argmatches-remove-occurrences"></span>`fn remove_occurrences<T: Any + Clone + Send + Sync + 'static>(&mut self, id: &str) -> Option<Occurrences<T>>` — [`Occurrences`](arg_matches/index.md#occurrences)

  Return values for each occurrence of an option.

  

  Each item is itself an iterator containing the arguments passed to a single occurrence of

  the option.

  

  If the option doesn't support multiple occurrences, or there was only a single occurrence,

  the iterator will only contain a single item.

  

  Returns `None` if the option wasn't present.

  

  # Panic

  

  If the argument definition and access mismatch.  To handle this case programmatically, see

  `ArgMatches::try_remove_occurrences`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, value_parser, ArgAction};

  let mut m = Command::new("myprog")

      .arg(Arg::new("x")

          .short('x')

          .num_args(2)

          .action(ArgAction::Append)

          .value_parser(value_parser!(String)))

      .get_matches_from(vec![

          "myprog", "-x", "a", "b", "-x", "c", "d"]);

  let vals: Vec<Vec<String>> = m.remove_occurrences("x").unwrap().map(Iterator::collect).collect();

  assert_eq!(vals, [["a", "b"], ["c", "d"]]);

  ```

- <span id="argmatches-contains-id"></span>`fn contains_id(&self, id: &str) -> bool`

  Check if values are present for the argument or group id

  

  <div class="warning">

  

  *NOTE:* This will always return `true` if `default_value` has been set.

  `ArgMatches::value_source` can be used to check if a value is present at runtime.

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group name (debug builds).  To handle this case programmatically, see

  `ArgMatches::try_contains_id`.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "myprog", "-d"

      ]);

  

  assert!(m.contains_id("debug"));

  ```

- <span id="argmatches-ids"></span>`fn ids(&self) -> IdsRef<'_>` — [`IdsRef`](arg_matches/index.md#idsref)

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

  assert_eq!(m.ids().len(), 2);

  assert_eq!(

      m.ids()

          .map(|id| id.as_str())

          .collect::<Vec<_>>(),

      ["config", "color"]

  );

  ```

- <span id="argmatches-args-present"></span>`fn args_present(&self) -> bool`

  Check if any `Arg`s were present on the command line

  

  See `ArgMatches::subcommand_name()` or `ArgMatches::subcommand()` to check if a

  subcommand was present on the command line.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let mut cmd = Command::new("myapp")

      .arg(Arg::new("output")

          .action(ArgAction::Set));

  

  let m = cmd

      .try_get_matches_from_mut(vec!["myapp", "something"])

      .unwrap();

  assert!(m.args_present());

  

  let m = cmd

      .try_get_matches_from_mut(vec!["myapp"])

      .unwrap();

  assert!(! m.args_present());

- <span id="argmatches-value-source"></span>`fn value_source(&self, id: &str) -> Option<ValueSource>` — [`ValueSource`](value_source/index.md#valuesource)

  Report where argument value came from

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  use clap::parser::ValueSource;

  let m = Command::new("myprog")

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "myprog", "-d"

      ]);

  

  assert_eq!(m.value_source("debug"), Some(ValueSource::CommandLine));

  ```

- <span id="argmatches-index-of"></span>`fn index_of(&self, id: &str) -> Option<usize>`

  The first index of that an argument showed up.

  

  Indices are similar to argv indices, but are not exactly 1:1.

  

  For flags (i.e. those arguments which don't have an associated value), indices refer

  to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices

  refer to the *values* `-o val` would therefore not represent two distinct indices, only the

  index for `val` would be recorded. This is by design.

  

  Besides the flag/option discrepancy, the primary difference between an argv index and clap

  index, is that clap continues counting once all arguments have properly separated, whereas

  an argv index does not.

  

  The examples should clear this up.

  

  <div class="warning">

  

  *NOTE:* If an argument is allowed multiple times, this method will only give the *first*

  index.  See `ArgMatches::indices_of`.

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  The argv indices are listed in the comments below. See how they correspond to the clap

  indices. Note that if it's not listed in a clap index, this is because it's not saved in

  in an `ArgMatches` struct for querying.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-f", "-o", "val"]);

             // ARGV indices: ^0       ^1    ^2    ^3

             // clap indices:          ^1          ^3

  

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("option"), Some(3));

  ```

  

  Now notice, if we use one of the other styles of options:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-f", "-o=val"]);

             // ARGV indices: ^0       ^1    ^2

             // clap indices:          ^1       ^3

  

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("option"), Some(3));

  ```

  

  Things become much more complicated, or clear if we look at a more complex combination of

  flags. Let's also throw in the final option style for good measure.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag2")

          .short('F')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag3")

          .short('z')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-fzF", "-oval"]);

             // ARGV indices: ^0      ^1       ^2

             // clap indices:         ^1,2,3    ^5

             //

             // clap sees the above as 'myapp -f -z -F -o val'

             //                         ^0    ^1 ^2 ^3 ^4 ^5

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("flag2"), Some(3));

  assert_eq!(m.index_of("flag3"), Some(2));

  assert_eq!(m.index_of("option"), Some(5));

  ```

  

  One final combination of flags/options to see how they combine:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag2")

          .short('F')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("flag3")

          .short('z')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set))

      .get_matches_from(vec!["myapp", "-fzFoval"]);

             // ARGV indices: ^0       ^1

             // clap indices:          ^1,2,3^5

             //

             // clap sees the above as 'myapp -f -z -F -o val'

             //                         ^0    ^1 ^2 ^3 ^4 ^5

  assert_eq!(m.index_of("flag"), Some(1));

  assert_eq!(m.index_of("flag2"), Some(3));

  assert_eq!(m.index_of("flag3"), Some(2));

  assert_eq!(m.index_of("option"), Some(5));

  ```

  

  The last part to mention is when values are sent in multiple groups with a [delimiter].

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .value_delimiter(',')

          .num_args(1..))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2   ^3   ^4

             //

             // clap sees the above as 'myapp -o val1 val2 val3'

             //                         ^0    ^1 ^2   ^3   ^4

  assert_eq!(m.index_of("option"), Some(2));

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);

  ```

- <span id="argmatches-indices-of"></span>`fn indices_of(&self, id: &str) -> Option<Indices<'_>>` — [`Indices`](arg_matches/index.md#indices)

  All indices an argument appeared at when parsing.

  

  Indices are similar to argv indices, but are not exactly 1:1.

  

  For flags (i.e. those arguments which don't have an associated value), indices refer

  to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices

  refer to the *values* `-o val` would therefore not represent two distinct indices, only the

  index for `val` would be recorded. This is by design.

  

  <div class="warning">

  

  *NOTE:* For more information about how clap indices compared to argv indices, see

  `ArgMatches::index_of`

  

  </div>

  

  # Panics

  

  If `id` is not a valid argument or group id (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .value_delimiter(','))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2   ^3   ^4

             //

             // clap sees the above as 'myapp -o val1 val2 val3'

             //                         ^0    ^1 ^2   ^3   ^4

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);

  ```

  

  Another quick example is when flags and options are used together

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set)

          .action(ArgAction::Append))

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::Count))

      .get_matches_from(vec!["myapp", "-o", "val1", "-f", "-o", "val2", "-f"]);

             // ARGV indices: ^0       ^1    ^2      ^3    ^4    ^5      ^6

             // clap indices:                ^2      ^3          ^5      ^6

  

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 5]);

  assert_eq!(m.indices_of("flag").unwrap().collect::<Vec<_>>(), &[6]);

  ```

  

  One final example, which is an odd case; if we *don't* use  value delimiter as we did with

  the first example above instead of `val1`, `val2` and `val3` all being distinc values, they

  would all be a single value of `val1,val2,val3`, in which case they'd only receive a single

  index.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("myapp")

      .arg(Arg::new("option")

          .short('o')

          .action(ArgAction::Set)

          .num_args(1..))

      .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);

             // ARGV indices: ^0       ^1

             // clap indices:             ^2

             //

             // clap sees the above as 'myapp -o "val1,val2,val3"'

             //                         ^0    ^1  ^2

  assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2]);

  ```

#### Trait Implementations

##### `impl Any for ArgMatches`

- <span id="argmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgMatches`

- <span id="argmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgMatches`

- <span id="argmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgMatches`

- <span id="argmatches-clone"></span>`fn clone(&self) -> ArgMatches` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl CloneToUninit for ArgMatches`

- <span id="argmatches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgMatches`

- <span id="argmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatches`

- <span id="argmatches-default"></span>`fn default() -> ArgMatches` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl Eq for ArgMatches`

##### `impl<T> From for ArgMatches`

- <span id="argmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgMatches`

- <span id="argmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgMatches`

- <span id="argmatches-partialeq-eq"></span>`fn eq(&self, other: &ArgMatches) -> bool` — [`ArgMatches`](arg_matches/index.md#argmatches)

##### `impl StructuralPartialEq for ArgMatches`

##### `impl ToOwned for ArgMatches`

- <span id="argmatches-toowned-type-owned"></span>`type Owned = T`

- <span id="argmatches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argmatches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgMatches`

- <span id="argmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgMatches`

- <span id="argmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for Indices<'a>`

- <span id="indices-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Indices<'a>`

- <span id="indices-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Indices<'a>`

- <span id="indices-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Indices<'a>`

- <span id="indices-clone"></span>`fn clone(&self) -> Indices<'a>` — [`Indices`](arg_matches/index.md#indices)

##### `impl CloneToUninit for Indices<'a>`

- <span id="indices-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Indices<'a>`

- <span id="indices-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Indices<'_>`

- <span id="indices-default"></span>`fn default() -> Self`

##### `impl DoubleEndedIterator for Indices<'_>`

- <span id="indices-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl ExactSizeIterator for Indices<'_>`

##### `impl<T> From for Indices<'a>`

- <span id="indices-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Indices<'a>`

- <span id="indices-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Indices<'a>`

- <span id="indices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="indices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="indices-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Indices<'_>`

- <span id="indices-iterator-type-item"></span>`type Item = usize`

- <span id="indices-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="indices-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Indices<'a>`

- <span id="indices-toowned-type-owned"></span>`type Owned = T`

- <span id="indices-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="indices-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Indices<'a>`

- <span id="indices-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="indices-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Indices<'a>`

- <span id="indices-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="indices-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for ValueSource`

- <span id="valuesource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueSource`

- <span id="valuesource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueSource`

- <span id="valuesource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueSource`

- <span id="valuesource-clone"></span>`fn clone(&self) -> ValueSource` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl CloneToUninit for ValueSource`

- <span id="valuesource-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueSource`

##### `impl Debug for ValueSource`

- <span id="valuesource-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueSource`

##### `impl<T> From for ValueSource`

- <span id="valuesource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueSource`

- <span id="valuesource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ValueSource`

- <span id="valuesource-ord-cmp"></span>`fn cmp(&self, other: &ValueSource) -> cmp::Ordering` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl PartialEq for ValueSource`

- <span id="valuesource-partialeq-eq"></span>`fn eq(&self, other: &ValueSource) -> bool` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl PartialOrd for ValueSource`

- <span id="valuesource-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ValueSource) -> option::Option<cmp::Ordering>` — [`ValueSource`](value_source/index.md#valuesource)

##### `impl StructuralPartialEq for ValueSource`

##### `impl ToOwned for ValueSource`

- <span id="valuesource-toowned-type-owned"></span>`type Owned = T`

- <span id="valuesource-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuesource-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueSource`

- <span id="valuesource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuesource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueSource`

- <span id="valuesource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuesource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

