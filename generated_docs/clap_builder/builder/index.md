*[clap_builder](../index.md) / [builder](index.md)*

---

# Module `builder`

Define [`Command`](command/index.md) line [arguments][`Arg`](arg/index.md)

## Contents

- [Modules](#modules)
  - [`action`](#action)
  - [`app_settings`](#app-settings)
  - [`arg`](#arg)
  - [`arg_group`](#arg-group)
  - [`arg_predicate`](#arg-predicate)
  - [`arg_settings`](#arg-settings)
  - [`command`](#command)
  - [`ext`](#ext)
  - [`os_str`](#os-str)
  - [`possible_value`](#possible-value)
  - [`range`](#range)
  - [`resettable`](#resettable)
  - [`str`](#str)
  - [`styled_str`](#styled-str)
  - [`value_hint`](#value-hint)
  - [`value_parser`](#value-parser)
  - [`debug_asserts`](#debug-asserts)
  - [`styling`](#styling)
- [Structs](#structs)
  - [`Str`](#str)
  - [`Arg`](#arg)
  - [`ArgGroup`](#arggroup)
  - [`Command`](#command)
  - [`OsStr`](#osstr)
  - [`PossibleValue`](#possiblevalue)
  - [`ValueRange`](#valuerange)
  - [`StyledStr`](#styledstr)
  - [`Styles`](#styles)
  - [`BoolValueParser`](#boolvalueparser)
  - [`BoolishValueParser`](#boolishvalueparser)
  - [`EnumValueParser`](#enumvalueparser)
  - [`FalseyValueParser`](#falseyvalueparser)
  - [`MapValueParser`](#mapvalueparser)
  - [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
  - [`OsStringValueParser`](#osstringvalueparser)
  - [`PathBufValueParser`](#pathbufvalueparser)
  - [`PossibleValuesParser`](#possiblevaluesparser)
  - [`RangedI64ValueParser`](#rangedi64valueparser)
  - [`RangedU64ValueParser`](#rangedu64valueparser)
  - [`StringValueParser`](#stringvalueparser)
  - [`TryMapValueParser`](#trymapvalueparser)
  - [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
  - [`ValueParser`](#valueparser)
- [Enums](#enums)
  - [`ArgAction`](#argaction)
  - [`ArgPredicate`](#argpredicate)
  - [`Resettable`](#resettable)
  - [`ValueHint`](#valuehint)
- [Traits](#traits)
  - [`IntoResettable`](#intoresettable)
  - [`TypedValueParser`](#typedvalueparser)
  - [`ValueParserFactory`](#valueparserfactory)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`action`](#action) | mod |  |
| [`app_settings`](#app-settings) | mod |  |
| [`arg`](#arg) | mod |  |
| [`arg_group`](#arg-group) | mod |  |
| [`arg_predicate`](#arg-predicate) | mod |  |
| [`arg_settings`](#arg-settings) | mod |  |
| [`command`](#command) | mod |  |
| [`ext`](#ext) | mod |  |
| [`os_str`](#os-str) | mod |  |
| [`possible_value`](#possible-value) | mod |  |
| [`range`](#range) | mod |  |
| [`resettable`](#resettable) | mod |  |
| [`str`](#str) | mod |  |
| [`styled_str`](#styled-str) | mod |  |
| [`value_hint`](#value-hint) | mod |  |
| [`value_parser`](#value-parser) | mod |  |
| [`debug_asserts`](#debug-asserts) | mod |  |
| [`styling`](#styling) | mod | Terminal [`Styles`] for help and error output |
| [`Str`](#str) | struct |  |
| [`Arg`](#arg) | struct |  |
| [`ArgGroup`](#arggroup) | struct |  |
| [`Command`](#command) | struct |  |
| [`OsStr`](#osstr) | struct |  |
| [`PossibleValue`](#possiblevalue) | struct |  |
| [`ValueRange`](#valuerange) | struct |  |
| [`StyledStr`](#styledstr) | struct |  |
| [`Styles`](#styles) | struct |  |
| [`BoolValueParser`](#boolvalueparser) | struct |  |
| [`BoolishValueParser`](#boolishvalueparser) | struct |  |
| [`EnumValueParser`](#enumvalueparser) | struct |  |
| [`FalseyValueParser`](#falseyvalueparser) | struct |  |
| [`MapValueParser`](#mapvalueparser) | struct |  |
| [`NonEmptyStringValueParser`](#nonemptystringvalueparser) | struct |  |
| [`OsStringValueParser`](#osstringvalueparser) | struct |  |
| [`PathBufValueParser`](#pathbufvalueparser) | struct |  |
| [`PossibleValuesParser`](#possiblevaluesparser) | struct |  |
| [`RangedI64ValueParser`](#rangedi64valueparser) | struct |  |
| [`RangedU64ValueParser`](#rangedu64valueparser) | struct |  |
| [`StringValueParser`](#stringvalueparser) | struct |  |
| [`TryMapValueParser`](#trymapvalueparser) | struct |  |
| [`UnknownArgumentValueParser`](#unknownargumentvalueparser) | struct |  |
| [`ValueParser`](#valueparser) | struct |  |
| [`ArgAction`](#argaction) | enum |  |
| [`ArgPredicate`](#argpredicate) | enum |  |
| [`Resettable`](#resettable) | enum |  |
| [`ValueHint`](#valuehint) | enum |  |
| [`IntoResettable`](#intoresettable) | trait |  |
| [`TypedValueParser`](#typedvalueparser) | trait |  |
| [`ValueParserFactory`](#valueparserfactory) | trait |  |

## Modules

- [`action`](action/index.md)
- [`app_settings`](app_settings/index.md)
- [`arg`](arg/index.md)
- [`arg_group`](arg_group/index.md)
- [`arg_predicate`](arg_predicate/index.md)
- [`arg_settings`](arg_settings/index.md)
- [`command`](command/index.md)
- [`ext`](ext/index.md)
- [`os_str`](os_str/index.md)
- [`possible_value`](possible_value/index.md)
- [`range`](range/index.md)
- [`resettable`](resettable/index.md)
- [`str`](str/index.md)
- [`styled_str`](styled_str/index.md)
- [`value_hint`](value_hint/index.md)
- [`value_parser`](value_parser/index.md)
- [`debug_asserts`](debug_asserts/index.md)
- [`styling`](styling/index.md) — Terminal [`Styles`] for help and error output

## Structs

### `Str`

```rust
struct Str {
    name: inner::Inner,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/str.rs:13-15`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/str.rs#L13-L15)*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

#### Implementations

- <span id="str-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="str-into-inner"></span>`fn into_inner(self) -> Inner` — [`Inner`](str/inner/index.md#inner)

- <span id="str-as-str"></span>`fn as_str(&self) -> &str`

  Get the raw string of the `Str`

#### Trait Implementations

##### `impl Any for Str`

- <span id="str-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Str`

- <span id="str-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T> Borrow for Str`

- <span id="str-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Str`

- <span id="str-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Str`

- <span id="str-clone"></span>`fn clone(&self) -> Str` — [`Str`](str/index.md#str)

##### `impl CloneToUninit for Str`

- <span id="str-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Str`

- <span id="str-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Str`

- <span id="str-default"></span>`fn default() -> Str` — [`Str`](str/index.md#str)

##### `impl Deref for Str`

- <span id="str-deref-type-target"></span>`type Target = str`

- <span id="str-deref"></span>`fn deref(&self) -> &str`

##### `impl Display for Str`

- <span id="str-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Str`

##### `impl<T> From for Str`

- <span id="str-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Str`

- <span id="str-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Str`

- <span id="str-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Str`

- <span id="str-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<String>` — [`Resettable`](resettable/index.md#resettable)

##### `impl Ord for Str`

- <span id="str-ord-cmp"></span>`fn cmp(&self, other: &Str) -> cmp::Ordering` — [`Str`](str/index.md#str)

##### `impl PartialEq for Str`

- <span id="str-partialeq-eq"></span>`fn eq(&self, other: &Str) -> bool` — [`Str`](str/index.md#str)

##### `impl PartialOrd for Str`

- <span id="str-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Str) -> option::Option<cmp::Ordering>` — [`Str`](str/index.md#str)

##### `impl Receiver for Str`

- <span id="str-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Str`

##### `impl ToOwned for Str`

- <span id="str-toowned-type-owned"></span>`type Owned = T`

- <span id="str-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="str-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Str`

- <span id="str-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Str`

- <span id="str-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="str-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Str`

- <span id="str-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="str-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Arg`

```rust
struct Arg {
    id: crate::Id,
    help: Option<crate::builder::StyledStr>,
    long_help: Option<crate::builder::StyledStr>,
    action: Option<crate::ArgAction>,
    value_parser: Option<super::ValueParser>,
    blacklist: Vec<crate::Id>,
    settings: arg_settings::ArgFlags,
    overrides: Vec<crate::Id>,
    groups: Vec<crate::Id>,
    requires: Vec<(crate::builder::ArgPredicate, crate::Id)>,
    r_ifs: Vec<(crate::Id, crate::builder::OsStr)>,
    r_ifs_all: Vec<(crate::Id, crate::builder::OsStr)>,
    r_unless: Vec<crate::Id>,
    r_unless_all: Vec<crate::Id>,
    short: Option<char>,
    long: Option<crate::builder::Str>,
    aliases: Vec<(crate::builder::Str, bool)>,
    short_aliases: Vec<(char, bool)>,
    disp_ord: Option<usize>,
    val_names: Vec<crate::builder::Str>,
    num_vals: Option<crate::builder::ValueRange>,
    val_delim: Option<char>,
    default_vals: Vec<crate::builder::OsStr>,
    default_vals_ifs: Vec<(crate::Id, crate::builder::ArgPredicate, Option<Vec<crate::builder::OsStr>>)>,
    default_missing_vals: Vec<crate::builder::OsStr>,
    terminator: Option<crate::builder::Str>,
    index: Option<usize>,
    help_heading: Option<Option<crate::builder::Str>>,
    ext: crate::builder::ext::Extensions,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg.rs:60-92`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg.rs#L60-L92)*

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](arg/index.md)s, using the builder pattern and setting options
manually, or using a usage string which is far less verbose but has fewer options. You can also
use a combination of the two methods to achieve the best of both worlds.

- [Basic API][crate::Arg#basic-api]
- [Value Handling][crate::Arg#value-handling]
- [Help][crate::Arg#help-1]
- [Advanced Argument Relations][crate::Arg#advanced-argument-relations]
- [Reflection][crate::Arg#reflection]

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, arg, ArgAction};
// Using the traditional builder pattern and setting each option manually
let cfg = Arg::new("config")
      .short('c')
      .long("config")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("Provides a config file to myprog");
// Using a usage string (setting a similar argument to the one above)
let input = arg!(-i --input <FILE> "Provides an input file to the program");
```

#### Implementations

- <span id="arg-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../util/id/index.md#id)

  Create a new [`Arg`](arg/index.md) with a unique name.

  

  The name is used to check whether or not the argument was used at

  runtime, get values, set relationships with other args, etc..

  

  By default, an `Arg` is

  - Positional, see `Arg::short` or `Arg::long` turn it into an option

  - Accept a single value, see `Arg::action` to override this

  

  <div class="warning">

  

  **NOTE:** In the case of arguments that take values (i.e. `Arg::action(ArgAction::Set)`)

  and positional arguments (i.e. those without a preceding `-` or `--`) the name will also

  be displayed when the user prints the usage/help information of the program.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  Arg::new("config")

  ;

  ```

- <span id="arg-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../util/id/index.md#id)

  Set the identifier used for referencing this argument in the clap API.

  

  See `Arg::new` for more details.

- <span id="arg-short"></span>`fn short(self, s: impl IntoResettable<char>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable)

  Sets the short version of the argument without the preceding `-`.

  

  By default `V` and `h` are used by the auto-generated `version` and `help` arguments,

  respectively. You will need to disable the auto-generated flags

  (`disable_help_flag`,

  `disable_version_flag`) and define your own.

  

  # Examples

  

  When calling `short`, use a single valid UTF-8 character which will allow using the

  argument via a single hyphen (`-`) such as `-c`:

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg,  ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("config")

          .short('c')

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "prog", "-c", "file.toml"

      ]);

  

  assert_eq!(m.get_one::<String>("config").map(String::as_str), Some("file.toml"));

  ```

  

  To use `-h` for your own flag and still have help:

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg,  ArgAction};

  let m = Command::new("prog")

      .disable_help_flag(true)

      .arg(Arg::new("host")

          .short('h')

          .long("host"))

      .arg(Arg::new("help")

          .long("help")

          .global(true)

          .action(ArgAction::Help))

      .get_matches_from(vec![

          "prog", "-h", "wikipedia.org"

      ]);

  

  assert_eq!(m.get_one::<String>("host").map(String::as_str), Some("wikipedia.org"));

  ```

- <span id="arg-long"></span>`fn long(self, l: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Str`](str/index.md#str)

  Sets the long version of the argument without the preceding `--`.

  

  By default `version` and `help` are used by the auto-generated `version` and `help`

  arguments, respectively. You may use the word `version` or `help` for the long form of your

  own arguments, in which case `clap` simply will not assign those to the auto-generated

  `version` or `help` arguments.

  

  <div class="warning">

  

  **NOTE:** Any leading `-` characters will be stripped

  

  </div>

  

  # Examples

  

  To set `long` use a word containing valid UTF-8. If you supply a double leading

  `--` such as `--config` they will be stripped. Hyphens in the middle of the word, however,

  will *not* be stripped (i.e. `config-file` is allowed).

  

  Setting `long` allows using the argument via a double hyphen (`--`) such as `--config`

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("cfg")

          .long("config")

          .action(ArgAction::Set))

      .get_matches_from(vec![

          "prog", "--config", "file.toml"

      ]);

  

  assert_eq!(m.get_one::<String>("cfg").map(String::as_str), Some("file.toml"));

  ```

- <span id="arg-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Str`](str/index.md#str)

  Add an alias, which functions as a hidden long flag.

  

  This is more efficient, and easier than creating multiple hidden arguments as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

              .long("test")

              .alias("alias")

              .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "--alias", "cool"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "cool");

  ```

- <span id="arg-short-alias"></span>`fn short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable)

  Add an alias, which functions as a hidden short flag.

  

  This is more efficient, and easier than creating multiple hidden arguments as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

              .short('t')

              .short_alias('e')

              .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "-e", "cool"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "cool");

  ```

- <span id="arg-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](str/index.md#str)

  Add aliases, which function as hidden long flags.

  

  This is more efficient, and easier than creating multiple hidden subcommands as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                      .long("test")

                      .aliases(["do-stuff", "do-tests", "tests"])

                      .action(ArgAction::SetTrue)

                      .help("the file to add")

                      .required(false))

              .get_matches_from(vec![

                  "prog", "--do-tests"

              ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-short-aliases"></span>`fn short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

  Add aliases, which functions as a hidden short flag.

  

  This is more efficient, and easier than creating multiple hidden subcommands as one only

  needs to check for the existence of this command, and not all variants.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                      .short('t')

                      .short_aliases(['e', 's'])

                      .action(ArgAction::SetTrue)

                      .help("the file to add")

                      .required(false))

              .get_matches_from(vec![

                  "prog", "-s"

              ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-visible-alias"></span>`fn visible_alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Str`](str/index.md#str)

  Add an alias, which functions as a visible long flag.

  

  Like `Arg::alias`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .visible_alias("something-awesome")

                  .long("test")

                  .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "--something-awesome", "coffee"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "coffee");

  ```

- <span id="arg-visible-short-alias"></span>`fn visible_short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable)

  Add an alias, which functions as a visible short flag.

  

  Like `Arg::short_alias`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .visible_short_alias('t')

                  .action(ArgAction::Set))

         .get_matches_from(vec![

              "prog", "-t", "coffee"

          ]);

  assert_eq!(m.get_one::<String>("test").unwrap(), "coffee");

  ```

- <span id="arg-visible-aliases"></span>`fn visible_aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](str/index.md#str)

  Add aliases, which function as visible long flags.

  

  Like `Arg::aliases`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .action(ArgAction::SetTrue)

                  .visible_aliases(["something", "awesome", "cool"]))

         .get_matches_from(vec![

              "prog", "--awesome"

          ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-visible-short-aliases"></span>`fn visible_short_aliases(self, names: impl IntoIterator<Item = char>) -> Self`

  Add aliases, which function as visible short flags.

  

  Like `Arg::short_aliases`, except that they are visible inside the help message.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

              .arg(Arg::new("test")

                  .long("test")

                  .action(ArgAction::SetTrue)

                  .visible_short_aliases(['t', 'e']))

         .get_matches_from(vec![

              "prog", "-t"

          ]);

  assert_eq!(m.get_flag("test"), true);

  ```

- <span id="arg-index"></span>`fn index(self, idx: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable)

  Specifies the index of a positional argument **starting at** 1.

  

  <div class="warning">

  

  **NOTE:** The index refers to position according to **other positional argument**. It does

  not define position in the argument list as a whole.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** You can optionally leave off the `index` method, and the index will be

  assigned in order of evaluation. Utilizing the `index` method allows for setting

  indexes out of order

  

  </div>

  

  <div class="warning">

  

  **NOTE:** This is only meant to be used for positional arguments and shouldn't to be used

  with `Arg::short` or `Arg::long`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** When utilized with `Arg::num_args(1..)`, only the **last** positional argument

  may be defined as having a variable number of arguments (i.e. with the highest index)

  

  </div>

  

  # Panics

  

  [`Command`](command/index.md) will [`panic!`](../../miette/panic/index.md) if indexes are skipped (such as defining `index(1)` and `index(3)`

  but not `index(2)`, or a positional argument is defined as multiple and is not the highest

  index (debug builds)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg};

  Arg::new("config")

      .index(1)

  ;

  ```

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("mode")

          .index(1))

      .arg(Arg::new("debug")

          .long("debug")

          .action(ArgAction::SetTrue))

      .get_matches_from(vec![

          "prog", "--debug", "fast"

      ]);

  

  assert!(m.contains_id("mode"));

  assert_eq!(m.get_one::<String>("mode").unwrap(), "fast"); // notice index(1) means "first positional"

                                                            // *not* first argument

  ```

  

  

  

- <span id="arg-trailing-var-arg"></span>`fn trailing_var_arg(self, yes: bool) -> Self`

  This is a "var arg" and everything that follows should be captured by it, as if the user had

  used a `--`.

  

  <div class="warning">

  

  **NOTE:** To start the trailing "var arg" on unknown flags (and not just a positional

  value), set `allow_hyphen_values`.  Either way, users still

  have the option to explicitly escape ambiguous arguments with `--`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** `Arg::value_delimiter` still applies if set.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** Setting this requires `Arg::num_args(..)`.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  let m = Command::new("myprog")

      .arg(arg!(<cmd> ... "commands to run").trailing_var_arg(true))

      .get_matches_from(vec!["myprog", "arg1", "-r", "val1"]);

  

  let trail: Vec<_> = m.get_many::<String>("cmd").unwrap().collect();

  assert_eq!(trail, ["arg1", "-r", "val1"]);

  ```

- <span id="arg-last"></span>`fn last(self, yes: bool) -> Self`

  This arg is the last, or final, positional argument (i.e. has the highest

  index) and is *only* able to be accessed via the `--` syntax (i.e. `$ prog args --

  last_arg`).

  

  Even, if no other arguments are left to parse, if the user omits the `--` syntax

  they will receive an [`UnknownArgument`](../index.md) error. Setting an argument to `.last(true)` also

  allows one to access this arg early using the `--` syntax. Accessing an arg early, even with

  the `--` syntax is otherwise not possible.

  

  <div class="warning">

  

  **NOTE:** This will change the usage string to look like `$ prog [OPTIONS] [-- <ARG>]` if

  `ARG` is marked as `.last(true)`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** This setting will imply [`crate::Command::dont_collapse_args_in_usage`](command/index.md#dont-collapse-args-in-usage) because failing

  to set this can make the usage string very confusing.

  

  </div>

  

  <div class="warning">

  

  **NOTE**: This setting only applies to positional arguments, and has no effect on OPTIONS

  

  </div>

  

  <div class="warning">

  

  **NOTE:** Setting this requires `taking values`

  

  </div>

  

  <div class="warning">

  

  **WARNING:** Using this setting *and* having child subcommands is not

  recommended with the exception of *also* using

  [`crate::Command::args_conflicts_with_subcommands`](command/index.md#args-conflicts-with-subcommands)

  (or [`crate::Command::subcommand_negates_reqs`](command/index.md#subcommand-negates-reqs) if the argument marked `Last` is also

  marked `Arg::required`)

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Arg, ArgAction};

  Arg::new("args")

      .action(ArgAction::Set)

      .last(true)

  ;

  ```

  

  Setting `last` ensures the arg has the highest [`index`](../../gimli/read/index/index.md) of all positional args

  and requires that the `--` syntax be used to access it early.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("first"))

      .arg(Arg::new("second"))

      .arg(Arg::new("third")

          .action(ArgAction::Set)

          .last(true))

      .try_get_matches_from(vec![

          "prog", "one", "--", "three"

      ]);

  

  assert!(res.is_ok());

  let m = res.unwrap();

  assert_eq!(m.get_one::<String>("third").unwrap(), "three");

  assert_eq!(m.get_one::<String>("second"), None);

  ```

  

  Even if the positional argument marked `Last` is the only argument left to parse,

  failing to use the `--` syntax results in an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("first"))

      .arg(Arg::new("second"))

      .arg(Arg::new("third")

          .action(ArgAction::Set)

          .last(true))

      .try_get_matches_from(vec![

          "prog", "one", "two", "three"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);

  ```

  

- <span id="arg-required"></span>`fn required(self, yes: bool) -> Self`

  Specifies that the argument must be present.

  

  Required by default means it is required, when no other conflicting rules or overrides have

  been evaluated. Conflicting rules take precedence over being required.

  

  **Pro tip:** Flags (i.e. not positional, or arguments that take values) shouldn't be

  required by default. This is because if a flag were to be required, it should simply be

  implied. No additional information is required from user. Flags by their very nature are

  simply boolean on/off switches. The only time a user *should* be required to use a flag

  is if the operation is destructive in nature, and the user is essentially proving to you,

  "Yes, I know what I'm doing."

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .required(true)

  ;

  ```

  

  Setting required requires that the argument be used at runtime.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .required(true)

          .action(ArgAction::Set)

          .long("config"))

      .try_get_matches_from(vec![

          "prog", "--config", "file.conf",

      ]);

  

  assert!(res.is_ok());

  ```

  

  Setting required and then *not* supplying that argument at runtime is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .required(true)

          .action(ArgAction::Set)

          .long("config"))

      .try_get_matches_from(vec![

          "prog"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

  ```

- <span id="arg-requires"></span>`fn requires(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Id`](../util/id/index.md#id)

  Sets an argument that is required when this one is present

  

  i.e. when using this argument, the following argument *must* be present.

  

  <div class="warning">

  

  **NOTE:** [Conflicting] rules and [override] rules take precedence over being required

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .requires("input")

  ;

  ```

  

  Setting `Arg::requires(name)` requires that the argument be used at runtime if the

  defining argument is used. If the defining argument isn't used, the other argument isn't

  required

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .action(ArgAction::Set)

          .requires("input")

          .long("config"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog"

      ]);

  

  assert!(res.is_ok()); // We didn't use cfg, so input wasn't required

  ```

  

  Setting `Arg::requires(name)` and *not* supplying that argument is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("cfg")

          .action(ArgAction::Set)

          .requires("input")

          .long("config"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog", "--config", "file.conf"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

  ```

  

  

- <span id="arg-exclusive"></span>`fn exclusive(self, yes: bool) -> Self`

  This argument must be passed alone; it conflicts with all other arguments.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Arg;

  Arg::new("config")

      .exclusive(true)

  ;

  ```

  

  Setting an exclusive argument and having any other arguments present at runtime

  is an error.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, error::ErrorKind, ArgAction};

  let res = Command::new("prog")

      .arg(Arg::new("exclusive")

          .action(ArgAction::Set)

          .exclusive(true)

          .long("exclusive"))

      .arg(Arg::new("debug")

          .long("debug"))

      .arg(Arg::new("input"))

      .try_get_matches_from(vec![

          "prog", "--exclusive", "file.conf", "file.txt"

      ]);

  

  assert!(res.is_err());

  assert_eq!(res.unwrap_err().kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arg-global"></span>`fn global(self, yes: bool) -> Self`

  Specifies that an argument can be matched to all child [`Subcommand`](../derive/index.md)s.

  

  <div class="warning">

  

  **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands), however

  their values once a user uses them will be propagated back up to parents. In effect, this

  means one should *define* all global arguments at the top level, however it doesn't matter

  where the user *uses* the global argument.

  

  </div>

  

  # Examples

  

  Assume an application with two subcommands, and you'd like to define a

  `--verbose` flag that can be called on any of the subcommands and parent, but you don't

  want to clutter the source with three duplicate [`Arg`](arg/index.md) definitions.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  let m = Command::new("prog")

      .arg(Arg::new("verb")

          .long("verbose")

          .short('v')

          .action(ArgAction::SetTrue)

          .global(true))

      .subcommand(Command::new("test"))

      .subcommand(Command::new("do-stuff"))

      .get_matches_from(vec![

          "prog", "do-stuff", "--verbose"

      ]);

  

  assert_eq!(m.subcommand_name(), Some("do-stuff"));

  let sub_m = m.subcommand_matches("do-stuff").unwrap();

  assert_eq!(sub_m.get_flag("verb"), true);

  ```

- <span id="arg-is-set"></span>`fn is_set(&self, s: ArgSettings) -> bool` — [`ArgSettings`](arg_settings/index.md#argsettings)

- <span id="arg-setting"></span>`fn setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](arg_settings/index.md#argsettings)

- <span id="arg-unset-setting"></span>`fn unset_setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](arg_settings/index.md#argsettings)

#### Trait Implementations

##### `impl Any for Arg`

- <span id="arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Arg`

- <span id="arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Arg`

- <span id="arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Arg`

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](arg/index.md#arg)

##### `impl CloneToUninit for Arg`

- <span id="arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Arg`

- <span id="arg-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](arg/index.md#arg)

##### `impl Display for Arg`

- <span id="arg-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl<T> From for Arg`

- <span id="arg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Arg`

- <span id="arg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Arg`

- <span id="arg-ord-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](arg/index.md#arg)

##### `impl PartialEq for Arg`

- <span id="arg-partialeq-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](arg/index.md#arg)

##### `impl PartialOrd for Arg`

- <span id="arg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl ToOwned for Arg`

- <span id="arg-toowned-type-owned"></span>`type Owned = T`

- <span id="arg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Arg`

- <span id="arg-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Arg`

- <span id="arg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Arg`

- <span id="arg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArgGroup`

```rust
struct ArgGroup {
    id: crate::util::Id,
    args: Vec<crate::util::Id>,
    required: bool,
    requires: Vec<crate::util::Id>,
    conflicts: Vec<crate::util::Id>,
    multiple: bool,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_group.rs:68-75`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_group.rs#L68-L75)*

Specifies a logical group of [arguments]

You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts] or [requirements]
- check which argument in a group was specified on the command-line

For visually grouping arguments in help, see instead
`Arg::help_heading`.

# Examples

The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
the arguments from the specified group is present at runtime.

```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, error::ErrorKind};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor", "patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major", "--patch"]);
// Because we used two args in the group it's an error
assert!(result.is_err());
let err = result.unwrap_err();
assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
```

This next example shows a passing parse of the same scenario
```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, Id};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor","patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major"]);
assert!(result.is_ok());
let matches = result.unwrap();
// We may not know which of the args was used, so we can test for the group...
assert!(matches.contains_id("vers"));
// We can also ask the group which arg was used
assert_eq!(matches
    .get_one::<Id>("vers")
    .expect("`vers` is required")
    .as_str(),
    "major"
);
// we could also alternatively check each arg individually (not shown here)
```




#### Implementations

- <span id="arggroup-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../util/id/index.md#id)

  Create a `ArgGroup` using a unique name.

  

  The name will be used to get values from the group or refer to the group inside of conflict

  and requirement rules.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::new("config")

  ;

  ```

- <span id="arggroup-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../util/id/index.md#id)

  Sets the group name.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::default().id("config")

  ;

  ```

- <span id="arggroup-arg"></span>`fn arg(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Id`](../util/id/index.md#id)

  Adds an [argument] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .arg("flag")

          .arg("color"))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-args"></span>`fn args(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../util/id/index.md#id)

  Adds multiple [arguments] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-get-args"></span>`fn get_args(&self) -> impl Iterator<Item = &Id>` — [`Id`](../util/id/index.md#id)

  Getters for all args. It will return a vector of `Id`

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let args: Vec<&str> = vec!["a1".into(), "a4".into()];

  let grp = ArgGroup::new("program").args(&args);

  

  for (pos, arg) in grp.get_args().enumerate() {

      assert_eq!(*arg, args[pos]);

  }

  ```

- <span id="arggroup-multiple"></span>`fn multiple(self, yes: bool) -> Self`

  Allows more than one of the [`Arg`](../index.md)s in this group to be used. (Default: `false`)

  

  # Examples

  

  Notice in this example we use *both* the `-f` and `-c` flags which are both part of the

  group

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .multiple(true))

      .get_matches_from(vec!["myprog", "-f", "-c"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  ```

  In this next example, we show the default behavior (i.e. `multiple(false)`) which will throw

  an error if more than one of the args in the group was used.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .try_get_matches_from(vec!["myprog", "-f", "-c"]);

  // Because we used both args in the group it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-is-multiple"></span>`fn is_multiple(&mut self) -> bool`

  Return true if the group allows more than one of the arguments

  in this group to be used. (Default: `false`)

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let mut group = ArgGroup::new("myprog")

      .args(["f", "c"])

      .multiple(true);

  

  assert!(group.is_multiple());

  ```

- <span id="arggroup-required"></span>`fn required(self, yes: bool) -> Self`

  Require an argument from the group to be present when parsing.

  

  This is unless conflicting with another argument.  A required group will be displayed in

  the usage string of the application in the format `<arg|arg2|arg3>`.

  

  <div class="warning">

  

  **NOTE:** This setting only applies to the current [`Command`](command/index.md) / [`Subcommand`](../derive/index.md)s, and not

  globally.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** By default, `ArgGroup::multiple` is set to `false` which when combined with

  `ArgGroup::required(true)` states, "One and *only one* arg must be used from this group.

  Use of more than one arg is an error." Vice setting `ArgGroup::multiple(true)` which

  states, '*At least* one arg from this group must be used. Using multiple is OK."

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .required(true))

      .try_get_matches_from(vec!["myprog"]);

  // Because we didn't use any of the args in the group, it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

  

- <span id="arggroup-requires"></span>`fn requires(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Id`](../util/id/index.md#id)

  Specify an argument or group that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when any one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires("debug"))

      .try_get_matches_from(vec!["myprog", "-c"]);

  // because we used an arg from the group, and the group requires "-d" to be used, it's an

  // error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-requires-all"></span>`fn requires_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../util/id/index.md#id)

  Specify arguments or groups that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group requires "-d" and "-v" to be used,

  // yet we only used "-d" it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-conflicts-with"></span>`fn conflicts_with(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Id`](../util/id/index.md#id)

  Specify an argument or group that must **not** be present when this group is.

  

  Exclusion (aka conflict) rules function just like [argument exclusion rules], you can name

  other arguments or groups that must *not* be present when one of the arguments from this

  group are used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with("debug"))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group conflicts with "-d", it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-conflicts-with-all"></span>`fn conflicts_with_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../util/id/index.md#id)

  Specify arguments or groups that must **not** be present when this group is.

  

  Exclusion rules function just like [argument exclusion rules], you can name other arguments

  or groups that must *not* be present when one of the arguments from this group are used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-v"]);

  // because we used an arg from the group, and the group conflicts with either "-v" or "-d"

  // it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

#### Trait Implementations

##### `impl Any for ArgGroup`

- <span id="arggroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgGroup`

- <span id="arggroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgGroup`

- <span id="arggroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](arg_group/index.md#arggroup)

##### `impl CloneToUninit for ArgGroup`

- <span id="arggroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgGroup`

- <span id="arggroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](arg_group/index.md#arggroup)

##### `impl Eq for ArgGroup`

##### `impl<T> From for ArgGroup`

- <span id="arggroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgGroup`

- <span id="arggroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-partialeq-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](arg_group/index.md#arggroup)

##### `impl StructuralPartialEq for ArgGroup`

##### `impl ToOwned for ArgGroup`

- <span id="arggroup-toowned-type-owned"></span>`type Owned = T`

- <span id="arggroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arggroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgGroup`

- <span id="arggroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arggroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgGroup`

- <span id="arggroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arggroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Command`

```rust
struct Command {
    name: crate::builder::Str,
    long_flag: Option<crate::builder::Str>,
    short_flag: Option<char>,
    display_name: Option<String>,
    bin_name: Option<String>,
    author: Option<crate::builder::Str>,
    version: Option<crate::builder::Str>,
    long_version: Option<crate::builder::Str>,
    about: Option<crate::builder::StyledStr>,
    long_about: Option<crate::builder::StyledStr>,
    before_help: Option<crate::builder::StyledStr>,
    before_long_help: Option<crate::builder::StyledStr>,
    after_help: Option<crate::builder::StyledStr>,
    after_long_help: Option<crate::builder::StyledStr>,
    aliases: Vec<(crate::builder::Str, bool)>,
    short_flag_aliases: Vec<(char, bool)>,
    long_flag_aliases: Vec<(crate::builder::Str, bool)>,
    usage_str: Option<crate::builder::StyledStr>,
    usage_name: Option<String>,
    help_str: Option<crate::builder::StyledStr>,
    disp_ord: Option<usize>,
    template: Option<crate::builder::StyledStr>,
    settings: crate::builder::app_settings::AppFlags,
    g_settings: crate::builder::app_settings::AppFlags,
    args: crate::mkeymap::MKeyMap,
    subcommands: Vec<Command>,
    groups: Vec<crate::builder::ArgGroup>,
    current_help_heading: Option<crate::builder::Str>,
    current_disp_ord: Option<usize>,
    subcommand_value_name: Option<crate::builder::Str>,
    subcommand_heading: Option<crate::builder::Str>,
    external_value_parser: Option<super::ValueParser>,
    long_help_exists: bool,
    deferred: Option<fn(Command) -> Command>,
    app_ext: crate::builder::ext::Extensions,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/command.rs:74-113`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/command.rs#L74-L113)*

Build a command-line interface.

This includes defining arguments, subcommands, parser behavior, and help output.
Once all configuration is complete,
the `Command::get_matches` family of methods starts the runtime-parsing
process. These methods then return information about the user supplied
arguments (or lack thereof).

When deriving a `Parser`, you can use
`CommandFactory::command` to access the
`Command`.

- [Basic API][crate::Command#basic-api]
- [Application-wide Settings][crate::Command#application-wide-settings]
- [Command-specific Settings][crate::Command#command-specific-settings]
- [Subcommand-specific Settings][crate::Command#subcommand-specific-settings]
- [Reflection][crate::Command#reflection]

# Examples

```no_run
use clap_builder as clap;
use clap::{Command, Arg};
let m = Command::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(
        Arg::new("in_file")
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();

// Your program logic starts here...
```


#### Implementations

- <span id="command-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](str/index.md#str)

  Creates a new instance of an `Command`.

  

  It is common, but not required, to use binary name as the `name`. This

  name will only be displayed to the user when they request to print

  version or help and usage information.

  

  See also [`command!`](crate::command!) and [`crate_name!`](crate::crate_name!).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  Command::new("My Program")

  ;

  ```

- <span id="command-arg"></span>`fn arg(self, a: impl Into<Arg>) -> Self` — [`Arg`](arg/index.md#arg)

  Adds an [argument] to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, Arg};

  Command::new("myprog")

      // Adding a single "flag" argument with a short and help text, using Arg::new()

      .arg(

          Arg::new("debug")

             .short('d')

             .help("turns on debugging mode")

      )

      // Adding a single "option" argument with a short, a long, and help text using the less

      // verbose Arg::from()

      .arg(

          arg!(-c --config <CONFIG> "Optionally sets a config file to use")

      )

  ;

  ```

- <span id="command-arg-internal"></span>`fn arg_internal(&mut self, arg: Arg)` — [`Arg`](arg/index.md#arg)

- <span id="command-args"></span>`fn args(self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self` — [`Arg`](arg/index.md#arg)

  Adds multiple [arguments] to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, Arg};

  Command::new("myprog")

      .args([

          arg!(-d --debug "turns on debugging info"),

          Arg::new("input").help("the input file to use")

      ])

  ;

  ```

- <span id="command-mut-arg"></span>`fn mut_arg<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate an [`Arg`](arg/index.md) after it's been added to a [`Command`](command/index.md).

  

  # Panics

  

  If the argument is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .arg(Arg::new("bar")

          .short('b')

          .action(ArgAction::SetTrue))

      .mut_arg("bar", |a| a.short('B'));

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "-b"]);

  

  // Since we changed `bar`'s short to "B" this should err as there

  // is no `-b` anymore, only `-B`

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "-B"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-args"></span>`fn mut_args<F>(self, f: F) -> Self`

  Allows one to mutate all [`Arg`](arg/index.md)s after they've been added to a [`Command`](command/index.md).

  

  This does not affect the built-in `--help` or `--version` arguments.

  

  # Examples

  

  ```ignore

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .arg(Arg::new("bar")

          .long("bar")

          .action(ArgAction::SetTrue))

      .arg(Arg::new("baz")

          .long("baz")

          .action(ArgAction::SetTrue))

      .mut_args(|a| {

          if let Some(l) = a.get_long().map(|l| format!("prefix-{l}")) {

              a.long(l)

          } else {

              a

          }

      });

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "--bar"]);

  

  // Since we changed `bar`'s long to "prefix-bar" this should err as there

  // is no `--bar` anymore, only `--prefix-bar`.

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "--prefix-bar"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-group"></span>`fn mut_group<F>(self, arg_id: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate an [`ArgGroup`](arg_group/index.md) after it's been added to a [`Command`](command/index.md).

  

  # Panics

  

  If the argument is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  

  Command::new("foo")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major "auto increase major"))

      .arg(arg!(--minor "auto increase minor"))

      .arg(arg!(--patch "auto increase patch"))

      .group(ArgGroup::new("vers")

           .args(["set-ver", "major", "minor","patch"])

           .required(true))

      .mut_group("vers", |a| a.required(false));

  ```

- <span id="command-mut-subcommand"></span>`fn mut_subcommand<F>(self, name: impl AsRef<str>, f: F) -> Self`

  Allows one to mutate a [`Command`](command/index.md) after it's been added as a subcommand.

  

  This can be useful for modifying auto-generated arguments of nested subcommands with

  `Command::mut_arg`.

  

  # Panics

  

  If the subcommand is undefined

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  

  let mut cmd = Command::new("foo")

          .subcommand(Command::new("bar"))

          .mut_subcommand("bar", |subcmd| subcmd.disable_help_flag(true));

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "bar", "--help"]);

  

  // Since we disabled the help flag on the "bar" subcommand, this should err.

  

  assert!(res.is_err());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "bar"]);

  assert!(res.is_ok());

  ```

- <span id="command-mut-subcommands"></span>`fn mut_subcommands<F>(self, f: F) -> Self`

  Allows one to mutate all [`Command`](command/index.md)s after they've been added as subcommands.

  

  This does not affect the built-in `--help` or `--version` arguments.

  

  # Examples

  

  ```ignore

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  

  let mut cmd = Command::new("foo")

      .subcommands([

          Command::new("fetch"),

          Command::new("push"),

      ])

      // Allow title-case subcommands

      .mut_subcommands(|sub| {

          let name = sub.get_name();

          let alias = name.chars().enumerate().map(|(i, c)| {

              if i == 0 {

                  c.to_ascii_uppercase()

              } else {

                  c

              }

          }).collect::<String>();

          sub.alias(alias)

      });

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "fetch"]);

  assert!(res.is_ok());

  

  let res = cmd.try_get_matches_from_mut(vec!["foo", "Fetch"]);

  assert!(res.is_ok());

  ```

- <span id="command-group"></span>`fn group(self, group: impl Into<ArgGroup>) -> Self` — [`ArgGroup`](arg_group/index.md#arggroup)

  Adds an [`ArgGroup`](arg_group/index.md) to the application.

  

  [`ArgGroup`](arg_group/index.md)s are a family of related arguments.

  By placing them in a logical group, you can build easier requirement and exclusion rules.

  

  Example use cases:

  - Make an entire [`ArgGroup`](arg_group/index.md) required, meaning that one (and *only*

    one) argument from that group must be present at runtime.

  - Name an [`ArgGroup`](arg_group/index.md) as a conflict to another argument.

    Meaning any of the arguments that belong to that group will cause a failure if present with

    the conflicting argument.

  - Ensure exclusion between arguments.

  - Extract a value from a group instead of determining exactly which argument was used.

  

  # Examples

  

  The following example demonstrates using an [`ArgGroup`](arg_group/index.md) to ensure that one, and only one,

  of the arguments from the specified group is present at runtime.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  Command::new("cmd")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major "auto increase major"))

      .arg(arg!(--minor "auto increase minor"))

      .arg(arg!(--patch "auto increase patch"))

      .group(ArgGroup::new("vers")

           .args(["set-ver", "major", "minor","patch"])

           .required(true))

  ;

  ```

- <span id="command-groups"></span>`fn groups(self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self` — [`ArgGroup`](arg_group/index.md#arggroup)

  Adds multiple [`ArgGroup`](arg_group/index.md)s to the [`Command`](command/index.md) at once.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg, ArgGroup};

  Command::new("cmd")

      .arg(arg!(--"set-ver" <ver> "set the version manually").required(false))

      .arg(arg!(--major         "auto increase major"))

      .arg(arg!(--minor         "auto increase minor"))

      .arg(arg!(--patch         "auto increase patch"))

      .arg(arg!(-c <FILE>       "a config file").required(false))

      .arg(arg!(-i <IFACE>      "an interface").required(false))

      .groups([

          ArgGroup::new("vers")

              .args(["set-ver", "major", "minor","patch"])

              .required(true),

          ArgGroup::new("input")

              .args(["c", "i"])

      ])

  ;

  ```

- <span id="command-subcommand"></span>`fn subcommand(self, subcmd: impl Into<Command>) -> Self` — [`Command`](command/index.md#command)

  Adds a subcommand to the list of valid possibilities.

  

  Subcommands are effectively sub-[`Command`](command/index.md)s, because they can contain their own arguments,

  subcommands, version, usage, etc. They also function just like [`Command`](command/index.md)s, in that they get

  their own auto generated help, version, and usage.

  

  A subcommand's `Command::name` will be used for:

  - The argument the user passes in

  - Programmatically looking up the subcommand

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  Command::new("myprog")

      .subcommand(Command::new("config")

          .about("Controls configuration features")

          .arg(arg!(<config> "Required configuration file to use")))

  ;

  ```

- <span id="command-subcommand-internal"></span>`fn subcommand_internal(self, subcmd: Self) -> Self`

- <span id="command-subcommands"></span>`fn subcommands(self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`

  Adds multiple subcommands to the list of valid possibilities.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, };

  Command::new("myprog")

  .subcommands( [

         Command::new("config").about("Controls configuration functionality")

                                  .arg(Arg::new("config_file")),

         Command::new("debug").about("Controls debug functionality")])

  ;

  ```

- <span id="command-defer"></span>`fn defer(self, deferred: fn(Command) -> Command) -> Self` — [`Command`](command/index.md#command)

  Delay initialization for parts of the `Command`

  

  This is useful for large applications to delay definitions of subcommands until they are

  being invoked.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, arg};

  Command::new("myprog")

      .subcommand(Command::new("config")

          .about("Controls configuration features")

          .defer(|cmd| {

              cmd.arg(arg!(<config> "Required configuration file to use"))

          })

      )

  ;

  ```

- <span id="command-debug-assert"></span>`fn debug_assert(self)`

  Catch problems earlier in the development cycle.

  

  Most error states are handled as asserts under the assumption they are programming mistake

  and not something to handle at runtime.  Rather than relying on tests (manual or automated)

  that exhaustively test your CLI to ensure the asserts are evaluated, this will run those

  asserts in a way convenient for running as a test.

  

  **Note:** This will not help with asserts in [`ArgMatches`](../parser/matches/arg_matches/index.md), those will need exhaustive

  testing of your CLI.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgAction};

  fn cmd() -> Command {

      Command::new("foo")

          .arg(

              Arg::new("bar").short('b').action(ArgAction::SetTrue)

          )

  }

  

  #[test]

  fn verify_app() {

      cmd().debug_assert();

  }

  

  fn main() {

      let m = cmd().get_matches_from(vec!["foo", "-b"]);

      println!("{}", m.get_flag("bar"));

  }

  ```

- <span id="command-error"></span>`fn error(&mut self, kind: ErrorKind, message: impl fmt::Display) -> Error` — [`ErrorKind`](../error/kind/index.md#errorkind), [`Error`](../index.md#error)

  Custom error message for post-parsing validation

  

  **Note:** this will ensure the `Command` has been sufficiently `built` for any

  relevant context, including usage.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, error::ErrorKind};

  let mut cmd = Command::new("myprog");

  let err = cmd.error(ErrorKind::InvalidValue, "Some failure case");

  ```

- <span id="command-get-matches"></span>`fn get_matches(self) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, `exiting` on failure.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let matches = Command::new("myprog")

      // Args and options go here...

      .get_matches();

  ```

  

- <span id="command-get-matches-mut"></span>`fn get_matches_mut(&mut self) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, `exiting` on failure.

  

  Like `Command::get_matches` but doesn't consume the `Command`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let mut cmd = Command::new("myprog")

      // Args and options go here...

      ;

  let matches = cmd.get_matches_mut();

  ```

  

- <span id="command-try-get-matches"></span>`fn try_get_matches(self) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md#result), [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse `env::args_os`, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../error/kind/index.md) is a

  `ErrorKind::DisplayHelp` or `ErrorKind::DisplayVersion` respectively. You must call

  `Error::exit` or perform a `std::process::exit`.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let matches = Command::new("myprog")

      // Args and options go here...

      .try_get_matches()

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

  

  

  

  

- <span id="command-get-matches-from"></span>`fn get_matches_from<I, T>(self, itr: I) -> ArgMatches` — [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, `exiting` on failure.

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let matches = Command::new("myprog")

      // Args and options go here...

      .get_matches_from(arg_vec);

  ```

  

  

- <span id="command-try-get-matches-from"></span>`fn try_get_matches_from<I, T>(self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md#result), [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../error/kind/index.md) is a `ErrorKind::DisplayHelp`

  or `ErrorKind::DisplayVersion` respectively. You must call `Error::exit` or

  perform a `std::process::exit` yourself.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let matches = Command::new("myprog")

      // Args and options go here...

      .try_get_matches_from(arg_vec)

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

  

  

  

  

  

  

- <span id="command-try-get-matches-from-mut"></span>`fn try_get_matches_from_mut<I, T>(&mut self, itr: I) -> ClapResult<ArgMatches>` — [`Result`](../error/index.md#result), [`ArgMatches`](../parser/matches/arg_matches/index.md#argmatches)

  Parse the specified arguments, returning a `clap::Result` on failure.

  

  Like `Command::try_get_matches_from` but doesn't consume the `Command`.

  

  <div class="warning">

  

  **NOTE:** This method WILL NOT exit when `--help` or `--version` (or short versions) are

  used. It will return a `clap::Error`, where the [`kind`](../error/kind/index.md) is a [`ErrorKind::DisplayHelp`](../index.md)

  or [`ErrorKind::DisplayVersion`](../index.md) respectively. You must call `Error::exit` or

  perform a [`std::process::exit`](../../libc/index.md) yourself.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** The first argument will be parsed as the binary name unless

  `Command::no_binary_name` is used.

  

  </div>

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```no_run

  use clap_builder as clap;

  use clap::{Command, Arg};

  let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

  

  let mut cmd = Command::new("myprog");

      // Args and options go here...

  let matches = cmd.try_get_matches_from_mut(arg_vec)

      .unwrap_or_else(|e| e.exit());

  ```

  

  

  

- <span id="command-print-help"></span>`fn print_help(&mut self) -> io::Result<()>`

  Prints the short help message (`-h`) to `io::stdout()`.

  

  See also `Command::print_long_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  let mut cmd = Command::new("myprog");

  cmd.print_help();

  ```

- <span id="command-print-long-help"></span>`fn print_long_help(&mut self) -> io::Result<()>`

  Prints the long help message (`--help`) to `io::stdout()`.

  

  See also `Command::print_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  let mut cmd = Command::new("myprog");

  cmd.print_long_help();

  ```

  

  

  

- <span id="command-render-help"></span>`fn render_help(&mut self) -> StyledStr` — [`StyledStr`](styled_str/index.md#styledstr)

  Render the short help message (`-h`) to a [`StyledStr`](styled_str/index.md)

  

  See also `Command::render_long_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  let mut out = io::stdout();

  let help = cmd.render_help();

  println!("{help}");

  ```

  

  

- <span id="command-render-long-help"></span>`fn render_long_help(&mut self) -> StyledStr` — [`StyledStr`](styled_str/index.md#styledstr)

  Render the long help message (`--help`) to a [`StyledStr`](styled_str/index.md).

  

  See also `Command::render_help`.

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  let mut out = io::stdout();

  let help = cmd.render_long_help();

  println!("{help}");

  ```

  

  

- <span id="command-render-version"></span>`fn render_version(&self) -> String`

  Version message rendered as if the user ran `-V`.

  

  See also `Command::render_long_version`.

  

  ### Coloring

  

  This function does not try to color the message nor it inserts any [ANSI escape codes].

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let cmd = Command::new("myprog");

  println!("{}", cmd.render_version());

  ```

  

  

  

- <span id="command-render-long-version"></span>`fn render_long_version(&self) -> String`

  Version message rendered as if the user ran `--version`.

  

  See also `Command::render_version`.

  

  ### Coloring

  

  This function does not try to color the message nor it inserts any [ANSI escape codes].

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let cmd = Command::new("myprog");

  println!("{}", cmd.render_long_version());

  ```

  

  

  

- <span id="command-render-usage"></span>`fn render_usage(&mut self) -> StyledStr` — [`StyledStr`](styled_str/index.md#styledstr)

  Usage statement

  

  **Note:** this will ensure the `Command` has been sufficiently `built`.

  

  # Panics

  

  If contradictory arguments or settings exist (debug builds).

  

  ### Examples

  

  ```rust

  use clap_builder as clap;

  use clap::Command;

  use std::io;

  let mut cmd = Command::new("myprog");

  println!("{}", cmd.render_usage());

  ```

- <span id="command-render-usage"></span>`fn render_usage_(&mut self) -> Option<StyledStr>` — [`StyledStr`](styled_str/index.md#styledstr)

#### Trait Implementations

##### `impl Any for Command`

- <span id="command-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Command`

- <span id="command-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Command`

- <span id="command-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Command`

- <span id="command-clone"></span>`fn clone(&self) -> Command` — [`Command`](command/index.md#command)

##### `impl CloneToUninit for Command`

- <span id="command-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Command`

- <span id="command-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Command`

- <span id="command-default"></span>`fn default() -> Self`

##### `impl Display for Command`

- <span id="command-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Command`

- <span id="command-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Index for Command`

- <span id="command-index-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](../util/id/index.md#id)

##### `impl<U> Into for Command`

- <span id="command-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Command`

- <span id="command-toowned-type-owned"></span>`type Owned = T`

- <span id="command-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="command-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Command`

- <span id="command-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Command`

- <span id="command-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="command-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Command`

- <span id="command-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="command-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OsStr`

```rust
struct OsStr {
    name: inner::Inner,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/os_str.rs:14-16`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/os_str.rs#L14-L16)*

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- <span id="osstr-from-static-ref"></span>`fn from_static_ref(name: &'static std::ffi::OsStr) -> Self`

- <span id="osstr-as-os-str"></span>`fn as_os_str(&self) -> &std::ffi::OsStr`

  Get the raw string as an `std::ffi::OsStr`

- <span id="osstr-to-os-string"></span>`fn to_os_string(&self) -> std::ffi::OsString`

  Get the raw string as an `OsString`

#### Trait Implementations

##### `impl Any for OsStr`

- <span id="osstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for OsStr`

- <span id="osstr-asref-as-ref"></span>`fn as_ref(&self) -> &std::ffi::OsStr`

##### `impl<T> Borrow for OsStr`

- <span id="osstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OsStr`

- <span id="osstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OsStr`

- <span id="osstr-clone"></span>`fn clone(&self) -> OsStr` — [`OsStr`](os_str/index.md#osstr)

##### `impl CloneToUninit for OsStr`

- <span id="osstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OsStr`

- <span id="osstr-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for OsStr`

- <span id="osstr-default"></span>`fn default() -> OsStr` — [`OsStr`](os_str/index.md#osstr)

##### `impl Deref for OsStr`

- <span id="osstr-deref-type-target"></span>`type Target = OsStr`

- <span id="osstr-deref"></span>`fn deref(&self) -> &std::ffi::OsStr`

##### `impl Eq for OsStr`

##### `impl<T> From for OsStr`

- <span id="osstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for OsStr`

- <span id="osstr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for OsStr`

- <span id="osstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for OsStr`

- <span id="osstr-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<OsStr>` — [`Resettable`](resettable/index.md#resettable), [`OsStr`](os_str/index.md#osstr)

##### `impl Ord for OsStr`

- <span id="osstr-ord-cmp"></span>`fn cmp(&self, other: &OsStr) -> cmp::Ordering` — [`OsStr`](os_str/index.md#osstr)

##### `impl PartialEq for OsStr`

- <span id="osstr-partialeq-eq"></span>`fn eq(&self, other: &OsStr) -> bool` — [`OsStr`](os_str/index.md#osstr)

##### `impl PartialOrd for OsStr`

- <span id="osstr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &OsStr) -> option::Option<cmp::Ordering>` — [`OsStr`](os_str/index.md#osstr)

##### `impl Receiver for OsStr`

- <span id="osstr-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for OsStr`

##### `impl ToOwned for OsStr`

- <span id="osstr-toowned-type-owned"></span>`type Owned = T`

- <span id="osstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="osstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OsStr`

- <span id="osstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="osstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OsStr`

- <span id="osstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="osstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PossibleValue`

```rust
struct PossibleValue {
    name: crate::builder::Str,
    help: Option<crate::builder::StyledStr>,
    aliases: Vec<crate::builder::Str>,
    hide: bool,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/possible_value.rs:40-45`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/possible_value.rs#L40-L45)*

A possible value of an argument.

This is used for specifying [possible values] of [Args].

See also `PossibleValuesParser`

<div class="warning">

**NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required
to [hide] single values from help messages and shell completions or to attach [`help`](../output/help/index.md) to
possible values.

</div>

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, builder::PossibleValue, ArgAction};
let cfg = Arg::new("config")
    .action(ArgAction::Set)
    .value_name("FILE")
    .value_parser([
        PossibleValue::new("fast"),
        PossibleValue::new("slow").help("slower than fast"),
        PossibleValue::new("secret speed").hide(true)
    ]);
```





#### Implementations

- <span id="possiblevalue-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](str/index.md#str)

  Create a [`PossibleValue`](possible_value/index.md) with its name.

  

  The name will be used to decide whether this value was provided by the user to an argument.

  

  <div class="warning">

  

  **NOTE:** In case it is not [hidden] it will also be shown in help messages for arguments

  that use it as a [possible value] and have not hidden them through `Arg::hide_possible_values(true)`.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("fast")

  ;

  ```

  

  

- <span id="possiblevalue-help"></span>`fn help(self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`StyledStr`](styled_str/index.md#styledstr)

  Sets the help description of the value.

  

  This is typically displayed in completions (where supported) and should be a short, one-line

  description.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .help("not fast")

  ;

  ```

- <span id="possiblevalue-hide"></span>`fn hide(self, yes: bool) -> Self`

  Hides this value from help and shell completions.

  

  This is an alternative to hiding through `Arg::hide_possible_values(true)`, if you only

  want to hide some values.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("secret")

      .hide(true)

  ;

  ```

- <span id="possiblevalue-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](resettable/index.md#intoresettable), [`Str`](str/index.md#str)

  Sets a *hidden* alias for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .alias("not-fast")

  ;

  ```

- <span id="possiblevalue-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](str/index.md#str)

  Sets multiple *hidden* aliases for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .aliases(["not-fast", "snake-like"])

  ;

  ```

#### Trait Implementations

##### `impl Any for PossibleValue`

- <span id="possiblevalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PossibleValue`

- <span id="possiblevalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PossibleValue`

- <span id="possiblevalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PossibleValue`

- <span id="possiblevalue-clone"></span>`fn clone(&self) -> PossibleValue` — [`PossibleValue`](possible_value/index.md#possiblevalue)

##### `impl CloneToUninit for PossibleValue`

- <span id="possiblevalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PossibleValue`

- <span id="possiblevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PossibleValue`

- <span id="possiblevalue-default"></span>`fn default() -> PossibleValue` — [`PossibleValue`](possible_value/index.md#possiblevalue)

##### `impl Eq for PossibleValue`

##### `impl<T> From for PossibleValue`

- <span id="possiblevalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PossibleValue`

- <span id="possiblevalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PossibleValue`

- <span id="possiblevalue-partialeq-eq"></span>`fn eq(&self, other: &PossibleValue) -> bool` — [`PossibleValue`](possible_value/index.md#possiblevalue)

##### `impl StructuralPartialEq for PossibleValue`

##### `impl ToOwned for PossibleValue`

- <span id="possiblevalue-toowned-type-owned"></span>`type Owned = T`

- <span id="possiblevalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="possiblevalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PossibleValue`

- <span id="possiblevalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="possiblevalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PossibleValue`

- <span id="possiblevalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="possiblevalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ValueRange`

```rust
struct ValueRange {
    start_inclusive: usize,
    end_inclusive: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/range.rs:3-6`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/range.rs#L3-L6)*

Values per occurrence for an argument

#### Implementations

- <span id="valuerange-const-empty"></span>`const EMPTY: Self`

- <span id="valuerange-const-single"></span>`const SINGLE: Self`

- <span id="valuerange-const-optional"></span>`const OPTIONAL: Self`

- <span id="valuerange-const-full"></span>`const FULL: Self`

- <span id="valuerange-new"></span>`fn new(range: impl Into<Self>) -> Self`

  Create a range

  

  # Panics

  

  If the end is less than the start (debug builds)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(5);

  let range = ValueRange::new(5..10);

  let range = ValueRange::new(5..=10);

  let range = ValueRange::new(5..);

  let range = ValueRange::new(..10);

  let range = ValueRange::new(..=10);

  ```

  

  While this will panic:

  ```should_panic

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(10..5);  // Panics!

  ```

- <span id="valuerange-raw"></span>`fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- <span id="valuerange-min-values"></span>`fn min_values(&self) -> usize`

  Fewest number of values the argument accepts

- <span id="valuerange-max-values"></span>`fn max_values(&self) -> usize`

  Most number of values the argument accepts

- <span id="valuerange-takes-values"></span>`fn takes_values(&self) -> bool`

  Report whether the argument takes any values (ie is a flag)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(5);

  assert!(range.takes_values());

  

  let range = ValueRange::new(0);

  assert!(!range.takes_values());

  ```

- <span id="valuerange-is-unbounded"></span>`fn is_unbounded(&self) -> bool`

- <span id="valuerange-is-fixed"></span>`fn is_fixed(&self) -> bool`

- <span id="valuerange-is-multiple"></span>`fn is_multiple(&self) -> bool`

- <span id="valuerange-num-values"></span>`fn num_values(&self) -> Option<usize>`

- <span id="valuerange-accepts-more"></span>`fn accepts_more(&self, current: usize) -> bool`

#### Trait Implementations

##### `impl Any for ValueRange`

- <span id="valuerange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueRange`

- <span id="valuerange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueRange`

- <span id="valuerange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueRange`

- <span id="valuerange-clone"></span>`fn clone(&self) -> ValueRange` — [`ValueRange`](range/index.md#valuerange)

##### `impl CloneToUninit for ValueRange`

- <span id="valuerange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- <span id="valuerange-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- <span id="valuerange-default"></span>`fn default() -> Self`

##### `impl Display for ValueRange`

- <span id="valuerange-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl<T> From for ValueRange`

- <span id="valuerange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ValueRange`

- <span id="valuerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ValueRange`

- <span id="valuerange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for ValueRange`

- <span id="valuerange-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueRange>` — [`Resettable`](resettable/index.md#resettable), [`ValueRange`](range/index.md#valuerange)

##### `impl PartialEq for ValueRange`

- <span id="valuerange-partialeq-eq"></span>`fn eq(&self, other: &ValueRange) -> bool` — [`ValueRange`](range/index.md#valuerange)

##### `impl RangeBounds for ValueRange`

- <span id="valuerange-rangebounds-start-bound"></span>`fn start_bound(&self) -> std::ops::Bound<&usize>`

- <span id="valuerange-rangebounds-end-bound"></span>`fn end_bound(&self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl ToOwned for ValueRange`

- <span id="valuerange-toowned-type-owned"></span>`type Owned = T`

- <span id="valuerange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuerange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ValueRange`

- <span id="valuerange-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ValueRange`

- <span id="valuerange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuerange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueRange`

- <span id="valuerange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuerange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StyledStr`

```rust
struct StyledStr(String);
```

*Defined in [`clap_builder-4.5.53/src/builder/styled_str.rs:25`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/styled_str.rs#L25)*

Terminal-styling container

Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)

# Examples

```rust
use clap_builder as clap;
// `cstr!` converts tags to ANSI codes
let after_help: &'static str = color_print::cstr!(
r#"<bold><underline>Examples</underline></bold>

  <dim>$</dim> <bold>mybin --input file.toml</bold>
"#);

let cmd = clap::Command::new("mybin")
    .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
    // ...
  ;
```

#### Implementations

- <span id="styledstr-new"></span>`const fn new() -> Self`

  Create an empty buffer

- <span id="styledstr-ansi"></span>`fn ansi(&self) -> impl std::fmt::Display + '_`

  Display using [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) styling

- <span id="styledstr-push-string"></span>`fn push_string(&mut self, msg: String)`

  May allow the compiler to consolidate the `Drop`s for `msg`, reducing code size compared to

  `styled.push_str(&msg)`

- <span id="styledstr-push-str"></span>`fn push_str(&mut self, msg: &str)`

  Appends a given string slice onto the end of this `StyledStr`.

- <span id="styledstr-trim-start-lines"></span>`fn trim_start_lines(&mut self)`

- <span id="styledstr-trim-end"></span>`fn trim_end(&mut self)`

- <span id="styledstr-replace-newline-var"></span>`fn replace_newline_var(&mut self)`

- <span id="styledstr-indent"></span>`fn indent(&mut self, initial: &str, trailing: &str)`

- <span id="styledstr-wrap"></span>`fn wrap(&mut self, _hard_width: usize)`

- <span id="styledstr-display-width"></span>`fn display_width(&self) -> usize`

- <span id="styledstr-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="styledstr-as-styled-str"></span>`fn as_styled_str(&self) -> &str`

- <span id="styledstr-iter-text"></span>`fn iter_text(&self) -> impl Iterator<Item = &str>`

- <span id="styledstr-push-styled"></span>`fn push_styled(&mut self, other: &Self)`

- <span id="styledstr-write-to"></span>`fn write_to(&self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for StyledStr`

- <span id="styledstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyledStr`

- <span id="styledstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyledStr`

- <span id="styledstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyledStr`

- <span id="styledstr-clone"></span>`fn clone(&self) -> StyledStr` — [`StyledStr`](styled_str/index.md#styledstr)

##### `impl CloneToUninit for StyledStr`

- <span id="styledstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StyledStr`

- <span id="styledstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyledStr`

- <span id="styledstr-default"></span>`fn default() -> StyledStr` — [`StyledStr`](styled_str/index.md#styledstr)

##### `impl Display for StyledStr`

- <span id="styledstr-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<T> From for StyledStr`

- <span id="styledstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyledStr`

- <span id="styledstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for StyledStr`

- <span id="styledstr-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<StyledStr>` — [`Resettable`](resettable/index.md#resettable), [`StyledStr`](styled_str/index.md#styledstr)

##### `impl Ord for StyledStr`

- <span id="styledstr-ord-cmp"></span>`fn cmp(&self, other: &StyledStr) -> cmp::Ordering` — [`StyledStr`](styled_str/index.md#styledstr)

##### `impl PartialEq for StyledStr`

- <span id="styledstr-partialeq-eq"></span>`fn eq(&self, other: &StyledStr) -> bool` — [`StyledStr`](styled_str/index.md#styledstr)

##### `impl PartialOrd for StyledStr`

- <span id="styledstr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StyledStr) -> option::Option<cmp::Ordering>` — [`StyledStr`](styled_str/index.md#styledstr)

##### `impl StructuralPartialEq for StyledStr`

##### `impl ToOwned for StyledStr`

- <span id="styledstr-toowned-type-owned"></span>`type Owned = T`

- <span id="styledstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styledstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StyledStr`

- <span id="styledstr-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StyledStr`

- <span id="styledstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyledStr`

- <span id="styledstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for StyledStr`

- <span id="styledstr-write-write-str"></span>`fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error>`

- <span id="styledstr-write-write-char"></span>`fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error>`

### `Styles`

```rust
struct Styles {
    header: Style,
    error: Style,
    usage: Style,
    literal: Style,
    placeholder: Style,
    valid: Style,
    invalid: Style,
    context: Style,
    context_value: Option<Style>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/styling.rs:23-33`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/styling.rs#L23-L33)*

Terminal styling definitions

See also `Command::styles`.

# Example

clap v3 styling
```rust
use clap_builder as clap;
use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

#### Implementations

- <span id="styles-plain"></span>`const fn plain() -> Self`

  No terminal styling

- <span id="styles-styled"></span>`const fn styled() -> Self`

  Default terminal styling

- <span id="styles-header"></span>`const fn header(self, style: Style) -> Self`

  General Heading style, e.g. `help_heading`

- <span id="styles-error"></span>`const fn error(self, style: Style) -> Self`

  Error heading

- <span id="styles-usage"></span>`const fn usage(self, style: Style) -> Self`

  Usage heading

- <span id="styles-literal"></span>`const fn literal(self, style: Style) -> Self`

  Literal command-line syntax, e.g. `--help`

- <span id="styles-placeholder"></span>`const fn placeholder(self, style: Style) -> Self`

  Descriptions within command-line syntax, e.g. `value_name`

- <span id="styles-valid"></span>`const fn valid(self, style: Style) -> Self`

  Highlight suggested usage

- <span id="styles-invalid"></span>`const fn invalid(self, style: Style) -> Self`

  Highlight invalid usage

- <span id="styles-context"></span>`const fn context(self, style: Style) -> Self`

  Highlight all specified contexts, e.g. `[default: false]`

  

  To specialize the style of the value within the context, see `Styles::context_value`

- <span id="styles-context-value"></span>`const fn context_value(self, style: Style) -> Self`

  Highlight values within all of the context, e.g. the `false` in `[default: false]`

  

  If not explicitly set, falls back to `context`'s style.

#### Trait Implementations

##### `impl Any for Styles`

- <span id="styles-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AppExt for Styles`

##### `impl<T> Borrow for Styles`

- <span id="styles-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Styles`

- <span id="styles-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Styles`

- <span id="styles-clone"></span>`fn clone(&self) -> Styles` — [`Styles`](styling/index.md#styles)

##### `impl CloneToUninit for Styles`

- <span id="styles-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Styles`

- <span id="styles-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Styles`

- <span id="styles-default"></span>`fn default() -> Self`

##### `impl<T> From for Styles`

- <span id="styles-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Styles`

- <span id="styles-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Styles`

- <span id="styles-toowned-type-owned"></span>`type Owned = T`

- <span id="styles-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styles-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Styles`

- <span id="styles-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styles-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Styles`

- <span id="styles-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styles-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1677`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1677)*

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- <span id="boolvalueparser-new"></span>`fn new() -> Self`

  Implementation for `ValueParser::bool`

- <span id="boolvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

#### Trait Implementations

##### `impl Any for BoolValueParser`

- <span id="boolvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoolValueParser`

- <span id="boolvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoolValueParser`

- <span id="boolvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BoolValueParser`

- <span id="boolvalueparser-clone"></span>`fn clone(&self) -> BoolValueParser` — [`BoolValueParser`](value_parser/index.md#boolvalueparser)

##### `impl CloneToUninit for BoolValueParser`

- <span id="boolvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BoolValueParser`

##### `impl Debug for BoolValueParser`

- <span id="boolvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolValueParser`

- <span id="boolvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for BoolValueParser`

- <span id="boolvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoolValueParser`

- <span id="boolvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for BoolValueParser`

- <span id="boolvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for BoolValueParser`

- <span id="boolvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="boolvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boolvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BoolValueParser`

- <span id="boolvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boolvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoolValueParser`

- <span id="boolvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boolvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for BoolValueParser`

- <span id="boolvalueparser-typedvalueparser-type-value"></span>`type Value = bool`

- <span id="boolvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="boolvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1877`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1877)*

Parse bool-like string values

See also:
- `ValueParser::bool` for different human readable bool representations
- [`FalseyValueParser`](value_parser/index.md) for assuming non-false is true

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::BoolishValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::BoolishValueParser::new();
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("true")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("Yes")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oN")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("1")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- <span id="boolishvalueparser-new"></span>`fn new() -> Self`

  Parse bool-like string values

- <span id="boolishvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

#### Trait Implementations

##### `impl Any for BoolishValueParser`

- <span id="boolishvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoolishValueParser`

- <span id="boolishvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoolishValueParser`

- <span id="boolishvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BoolishValueParser`

- <span id="boolishvalueparser-clone"></span>`fn clone(&self) -> BoolishValueParser` — [`BoolishValueParser`](value_parser/index.md#boolishvalueparser)

##### `impl CloneToUninit for BoolishValueParser`

- <span id="boolishvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BoolishValueParser`

##### `impl Debug for BoolishValueParser`

- <span id="boolishvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolishValueParser`

- <span id="boolishvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for BoolishValueParser`

- <span id="boolishvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoolishValueParser`

- <span id="boolishvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for BoolishValueParser`

- <span id="boolishvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for BoolishValueParser`

- <span id="boolishvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="boolishvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boolishvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BoolishValueParser`

- <span id="boolishvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boolishvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoolishValueParser`

- <span id="boolishvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boolishvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for BoolishValueParser`

- <span id="boolishvalueparser-typedvalueparser-type-value"></span>`type Value = bool`

- <span id="boolishvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="boolishvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(std::marker::PhantomData<E>);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1079-1081`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1079-L1081)*

Parse an `ValueEnum` value.

See also:
- [`PossibleValuesParser`](value_parser/index.md)

# Example

```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::ColorChoice;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;

// Usage
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::EnumValueParser::<ColorChoice>::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: ColorChoice = *m.get_one("color")
    .expect("required");
assert_eq!(port, ColorChoice::Always);

// Semantics
let value_parser = clap::builder::EnumValueParser::<ColorChoice>::new();
// or
let value_parser = clap::value_parser!(ColorChoice);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), ColorChoice::Always);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), ColorChoice::Auto);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), ColorChoice::Never);
```

#### Implementations

- <span id="enumvalueparser-new"></span>`fn new() -> Self`

  Parse an `ValueEnum`

#### Trait Implementations

##### `impl Any for EnumValueParser<E>`

- <span id="enumvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EnumValueParser<E>`

- <span id="enumvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EnumValueParser<E>`

- <span id="enumvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static> Clone for EnumValueParser<E>`

- <span id="enumvalueparser-clone"></span>`fn clone(&self) -> EnumValueParser<E>` — [`EnumValueParser`](value_parser/index.md#enumvalueparser)

##### `impl CloneToUninit for EnumValueParser<E>`

- <span id="enumvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<E: fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static> Debug for EnumValueParser<E>`

- <span id="enumvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> Default for EnumValueParser<E>`

- <span id="enumvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for EnumValueParser<E>`

- <span id="enumvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for EnumValueParser<E>`

- <span id="enumvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for EnumValueParser<E>`

- <span id="enumvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for EnumValueParser<E>`

- <span id="enumvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="enumvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enumvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EnumValueParser<E>`

- <span id="enumvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enumvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EnumValueParser<E>`

- <span id="enumvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enumvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> TypedValueParser for EnumValueParser<E>`

- <span id="enumvalueparser-typedvalueparser-type-value"></span>`type Value = E`

- <span id="enumvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="enumvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1778`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1778)*

Parse false-like string values, everything else is `true`

See also:
- `ValueParser::bool` for assuming non-false is true
- [`BoolishValueParser`](value_parser/index.md) for different human readable bool representations

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::FalseyValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::FalseyValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- <span id="falseyvalueparser-new"></span>`fn new() -> Self`

  Parse false-like string values, everything else is `true`

- <span id="falseyvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

#### Trait Implementations

##### `impl Any for FalseyValueParser`

- <span id="falseyvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FalseyValueParser`

- <span id="falseyvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FalseyValueParser`

- <span id="falseyvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FalseyValueParser`

- <span id="falseyvalueparser-clone"></span>`fn clone(&self) -> FalseyValueParser` — [`FalseyValueParser`](value_parser/index.md#falseyvalueparser)

##### `impl CloneToUninit for FalseyValueParser`

- <span id="falseyvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FalseyValueParser`

##### `impl Debug for FalseyValueParser`

- <span id="falseyvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FalseyValueParser`

- <span id="falseyvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for FalseyValueParser`

- <span id="falseyvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FalseyValueParser`

- <span id="falseyvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for FalseyValueParser`

- <span id="falseyvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for FalseyValueParser`

- <span id="falseyvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="falseyvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="falseyvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FalseyValueParser`

- <span id="falseyvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="falseyvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FalseyValueParser`

- <span id="falseyvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="falseyvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for FalseyValueParser`

- <span id="falseyvalueparser-typedvalueparser-type-value"></span>`type Value = bool`

- <span id="falseyvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="falseyvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `MapValueParser<P, F>`

```rust
struct MapValueParser<P, F> {
    parser: P,
    func: F,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2014-2017`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L2014-L2017)*

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::map`

#### Implementations

- <span id="mapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl Any for MapValueParser<P, F>`

- <span id="mapvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapValueParser<P, F>`

- <span id="mapvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapValueParser<P, F>`

- <span id="mapvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone, F: clone::Clone> Clone for MapValueParser<P, F>`

- <span id="mapvalueparser-clone"></span>`fn clone(&self) -> MapValueParser<P, F>` — [`MapValueParser`](value_parser/index.md#mapvalueparser)

##### `impl CloneToUninit for MapValueParser<P, F>`

- <span id="mapvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for MapValueParser<P, F>`

- <span id="mapvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MapValueParser<P, F>`

- <span id="mapvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapValueParser<P, F>`

- <span id="mapvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for MapValueParser<P, F>`

- <span id="mapvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for MapValueParser<P, F>`

- <span id="mapvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="mapvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapValueParser<P, F>`

- <span id="mapvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapValueParser<P, F>`

- <span id="mapvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, F> TypedValueParser for MapValueParser<P, F>`

- <span id="mapvalueparser-typedvalueparser-type-value"></span>`type Value = T`

- <span id="mapvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="mapvalueparser-typedvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="mapvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `NonEmptyStringValueParser`

```rust
struct NonEmptyStringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1968`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1968)*

Parse non-empty string values

See also:
- `ValueParser::string`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: &String = m.get_one("append")
    .expect("required");
assert_eq!(port, "true");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::NonEmptyStringValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), "random");
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
```

#### Implementations

- <span id="nonemptystringvalueparser-new"></span>`fn new() -> Self`

  Parse non-empty string values

#### Trait Implementations

##### `impl Any for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-clone"></span>`fn clone(&self) -> NonEmptyStringValueParser` — [`NonEmptyStringValueParser`](value_parser/index.md#nonemptystringvalueparser)

##### `impl CloneToUninit for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NonEmptyStringValueParser`

##### `impl Debug for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="nonemptystringvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nonemptystringvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nonemptystringvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nonemptystringvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-typedvalueparser-type-value"></span>`type Value = String`

- <span id="nonemptystringvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:953`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L953)*

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- <span id="osstringvalueparser-new"></span>`fn new() -> Self`

  Implementation for `ValueParser::os_string`

#### Trait Implementations

##### `impl Any for OsStringValueParser`

- <span id="osstringvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OsStringValueParser`

- <span id="osstringvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OsStringValueParser`

- <span id="osstringvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OsStringValueParser`

- <span id="osstringvalueparser-clone"></span>`fn clone(&self) -> OsStringValueParser` — [`OsStringValueParser`](value_parser/index.md#osstringvalueparser)

##### `impl CloneToUninit for OsStringValueParser`

- <span id="osstringvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OsStringValueParser`

##### `impl Debug for OsStringValueParser`

- <span id="osstringvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OsStringValueParser`

- <span id="osstringvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for OsStringValueParser`

- <span id="osstringvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OsStringValueParser`

- <span id="osstringvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for OsStringValueParser`

- <span id="osstringvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for OsStringValueParser`

- <span id="osstringvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="osstringvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="osstringvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OsStringValueParser`

- <span id="osstringvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="osstringvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OsStringValueParser`

- <span id="osstringvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="osstringvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for OsStringValueParser`

- <span id="osstringvalueparser-typedvalueparser-type-value"></span>`type Value = OsString`

- <span id="osstringvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="osstringvalueparser-typedvalueparser-parse"></span>`fn parse(&self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:995`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L995)*

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- <span id="pathbufvalueparser-new"></span>`fn new() -> Self`

  Implementation for `ValueParser::path_buf`

#### Trait Implementations

##### `impl Any for PathBufValueParser`

- <span id="pathbufvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathBufValueParser`

- <span id="pathbufvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathBufValueParser`

- <span id="pathbufvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PathBufValueParser`

- <span id="pathbufvalueparser-clone"></span>`fn clone(&self) -> PathBufValueParser` — [`PathBufValueParser`](value_parser/index.md#pathbufvalueparser)

##### `impl CloneToUninit for PathBufValueParser`

- <span id="pathbufvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PathBufValueParser`

##### `impl Debug for PathBufValueParser`

- <span id="pathbufvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathBufValueParser`

- <span id="pathbufvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for PathBufValueParser`

- <span id="pathbufvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PathBufValueParser`

- <span id="pathbufvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for PathBufValueParser`

- <span id="pathbufvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for PathBufValueParser`

- <span id="pathbufvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="pathbufvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pathbufvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PathBufValueParser`

- <span id="pathbufvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathbufvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathBufValueParser`

- <span id="pathbufvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathbufvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for PathBufValueParser`

- <span id="pathbufvalueparser-typedvalueparser-type-value"></span>`type Value = PathBuf`

- <span id="pathbufvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="pathbufvalueparser-typedvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `PossibleValuesParser`

```rust
struct PossibleValuesParser(Vec<super::PossibleValue>);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1196`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1196)*

Verify the value is from an enumerated set of `PossibleValue`.

See also:
- [`EnumValueParser`](value_parser/index.md) for directly supporting `ValueEnum` types
- `TypedValueParser::map` for adapting values to a more specialized type, like an external
  enums that can't implement `ValueEnum`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::PossibleValuesParser::new(["always", "auto", "never"]))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: &String = m.get_one("color")
    .expect("required");
assert_eq!(port, "always");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::PossibleValuesParser::new(["always", "auto", "never"]);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), "always");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), "auto");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), "never");
```

#### Implementations

- <span id="possiblevaluesparser-new"></span>`fn new(values: impl Into<PossibleValuesParser>) -> Self` — [`PossibleValuesParser`](value_parser/index.md#possiblevaluesparser)

  Verify the value is from an enumerated set of `PossibleValue`.

#### Trait Implementations

##### `impl Any for PossibleValuesParser`

- <span id="possiblevaluesparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PossibleValuesParser`

- <span id="possiblevaluesparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PossibleValuesParser`

- <span id="possiblevaluesparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PossibleValuesParser`

- <span id="possiblevaluesparser-clone"></span>`fn clone(&self) -> PossibleValuesParser` — [`PossibleValuesParser`](value_parser/index.md#possiblevaluesparser)

##### `impl CloneToUninit for PossibleValuesParser`

- <span id="possiblevaluesparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PossibleValuesParser`

- <span id="possiblevaluesparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PossibleValuesParser`

- <span id="possiblevaluesparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PossibleValuesParser`

- <span id="possiblevaluesparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for PossibleValuesParser`

- <span id="possiblevaluesparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for PossibleValuesParser`

- <span id="possiblevaluesparser-toowned-type-owned"></span>`type Owned = T`

- <span id="possiblevaluesparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="possiblevaluesparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PossibleValuesParser`

- <span id="possiblevaluesparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="possiblevaluesparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PossibleValuesParser`

- <span id="possiblevaluesparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="possiblevaluesparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for PossibleValuesParser`

- <span id="possiblevaluesparser-typedvalueparser-type-value"></span>`type Value = String`

- <span id="possiblevaluesparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="possiblevaluesparser-typedvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`Error`](../index.md#error)

- <span id="possiblevaluesparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync>`

```rust
struct RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync> {
    bounds: (std::ops::Bound<i64>, std::ops::Bound<i64>),
    target: std::marker::PhantomData<T>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1315-1318`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1315-L1318)*

Parse number that fall within a range of values

<div class="warning">

**NOTE:** To capture negative values, you will also need to set
`Arg::allow_negative_numbers` or
`Arg::allow_hyphen_values`.

</div>

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedI64ValueParser::<i32>::new().range(-1..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).unwrap(), -1);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- <span id="rangedi64valueparser-new"></span>`fn new() -> Self`

  Select full range of `i64`

- <span id="rangedi64valueparser-range"></span>`fn range<B: RangeBounds<i64>>(self, range: B) -> Self`

  Narrow the supported range

- <span id="rangedi64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T> Any for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + TryFrom<i64> + Clone + Send + Sync> Clone for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-clone"></span>`fn clone(&self) -> RangedI64ValueParser<T>` — [`RangedI64ValueParser`](value_parser/index.md#rangedi64valueparser)

##### `impl<T> CloneToUninit for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy + TryFrom<i64> + Clone + Send + Sync> Copy for RangedI64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<i64> + Clone + Send + Sync> Debug for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync> Default for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl<T> ToOwned for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="rangedi64valueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangedi64valueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangedi64valueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangedi64valueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync + 'static> TypedValueParser for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-typedvalueparser-type-value"></span>`type Value = T`

- <span id="rangedi64valueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `RangedU64ValueParser<T: TryFrom<u64>>`

```rust
struct RangedU64ValueParser<T: TryFrom<u64>> {
    bounds: (std::ops::Bound<u64>, std::ops::Bound<u64>),
    target: std::marker::PhantomData<T>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1514-1517`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L1514-L1517)*

Parse number that fall within a range of values

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u64).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u64 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedU64ValueParser::<u32>::new().range(0..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- <span id="rangedu64valueparser-new"></span>`fn new() -> Self`

  Select full range of `u64`

- <span id="rangedu64valueparser-range"></span>`fn range<B: RangeBounds<u64>>(self, range: B) -> Self`

  Narrow the supported range

- <span id="rangedu64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T> Any for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone + TryFrom<u64>> Clone for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-clone"></span>`fn clone(&self) -> RangedU64ValueParser<T>` — [`RangedU64ValueParser`](value_parser/index.md#rangedu64valueparser)

##### `impl<T> CloneToUninit for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy + TryFrom<u64>> Copy for RangedU64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<u64>> Debug for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<u64>> Default for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl<T> ToOwned for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="rangedu64valueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangedu64valueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangedu64valueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangedu64valueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: TryFrom<u64> + Clone + Send + Sync + 'static> TypedValueParser for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-typedvalueparser-type-value"></span>`type Value = T`

- <span id="rangedu64valueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `StringValueParser`

```rust
struct StringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:905`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L905)*

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](value_parser/index.md)s

#### Implementations

- <span id="stringvalueparser-new"></span>`fn new() -> Self`

  Implementation for `ValueParser::string`

#### Trait Implementations

##### `impl Any for StringValueParser`

- <span id="stringvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StringValueParser`

- <span id="stringvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StringValueParser`

- <span id="stringvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StringValueParser`

- <span id="stringvalueparser-clone"></span>`fn clone(&self) -> StringValueParser` — [`StringValueParser`](value_parser/index.md#stringvalueparser)

##### `impl CloneToUninit for StringValueParser`

- <span id="stringvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StringValueParser`

##### `impl Debug for StringValueParser`

- <span id="stringvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StringValueParser`

- <span id="stringvalueparser-default"></span>`fn default() -> Self`

##### `impl<T> From for StringValueParser`

- <span id="stringvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StringValueParser`

- <span id="stringvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for StringValueParser`

- <span id="stringvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for StringValueParser`

- <span id="stringvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="stringvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stringvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StringValueParser`

- <span id="stringvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stringvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StringValueParser`

- <span id="stringvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stringvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for StringValueParser`

- <span id="stringvalueparser-typedvalueparser-type-value"></span>`type Value = String`

- <span id="stringvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="stringvalueparser-typedvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `TryMapValueParser<P, F>`

```rust
struct TryMapValueParser<P, F> {
    parser: P,
    func: F,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2073-2076`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L2073-L2076)*

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::try_map`

#### Implementations

- <span id="trymapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl Any for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone, F: clone::Clone> Clone for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-clone"></span>`fn clone(&self) -> TryMapValueParser<P, F>` — [`TryMapValueParser`](value_parser/index.md#trymapvalueparser)

##### `impl CloneToUninit for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="trymapvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="trymapvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trymapvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trymapvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, F> TypedValueParser for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-typedvalueparser-type-value"></span>`type Value = T`

- <span id="trymapvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="trymapvalueparser-typedvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](possible_value/index.md#possiblevalue)

### `UnknownArgumentValueParser`

```rust
struct UnknownArgumentValueParser {
    arg: Option<crate::builder::Str>,
    suggestions: Vec<crate::builder::StyledStr>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2159-2162`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L2159-L2162)*

When encountered, report `ErrorKind::UnknownArgument`

Useful to help users migrate, either from old versions or similar tools.

# Examples

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
let cmd = Command::new("mycmd")
    .args([
        Arg::new("current-dir")
            .short('C'),
        Arg::new("current-dir-unknown")
            .long("cwd")
            .aliases(["current-dir", "directory", "working-directory", "root"])
            .value_parser(clap::builder::UnknownArgumentValueParser::suggest_arg("-C"))
            .hide(true),
    ]);

// Use a supported version of the argument
let matches = cmd.clone().try_get_matches_from(["mycmd", "-C", ".."]).unwrap();
assert!(matches.contains_id("current-dir"));
assert_eq!(
    matches.get_many::<String>("current-dir").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    vec![".."]
);

// Use one of the invalid versions
let err = cmd.try_get_matches_from(["mycmd", "--cwd", ".."]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
```

#### Implementations

- <span id="unknownargumentvalueparser-suggest-arg"></span>`fn suggest_arg(arg: impl Into<Str>) -> Self` — [`Str`](str/index.md#str)

  Suggest an alternative argument

- <span id="unknownargumentvalueparser-suggest"></span>`fn suggest(text: impl Into<StyledStr>) -> Self` — [`StyledStr`](styled_str/index.md#styledstr)

  Provide a general suggestion

- <span id="unknownargumentvalueparser-and-suggest"></span>`fn and_suggest(self, text: impl Into<StyledStr>) -> Self` — [`StyledStr`](styled_str/index.md#styledstr)

  Extend the suggestions

#### Trait Implementations

##### `impl Any for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-clone"></span>`fn clone(&self) -> UnknownArgumentValueParser` — [`UnknownArgumentValueParser`](value_parser/index.md#unknownargumentvalueparser)

##### `impl CloneToUninit for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="unknownargumentvalueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unknownargumentvalueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unknownargumentvalueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unknownargumentvalueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl TypedValueParser for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-typedvalueparser-type-value"></span>`type Value = String`

- <span id="unknownargumentvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

- <span id="unknownargumentvalueparser-typedvalueparser-parse-ref"></span>`fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>` — [`Command`](command/index.md#command), [`Arg`](arg/index.md#arg), [`ValueSource`](../parser/matches/value_source/index.md#valuesource), [`TypedValueParser`](value_parser/index.md#typedvalueparser), [`Error`](../index.md#error)

### `ValueParser`

```rust
struct ValueParser(ValueParserInner);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:63`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L63)*

Parse/validate argument values

Specified with `Arg::value_parser`.

`ValueParser` defines how to convert a raw argument value into a validated and typed value for
use within an application.

See
- `value_parser!` for automatically selecting an implementation for a given type
- `ValueParser::new` for additional [`TypedValueParser`](value_parser/index.md) that can be used

# Example

```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .long("color")
            .value_parser(["always", "auto", "never"])
            .default_value("auto")
    )
    .arg(
        clap::Arg::new("hostname")
            .long("hostname")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .action(clap::ArgAction::Set)
            .required(true)
    )
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(
    ["cmd", "--hostname", "rust-lang.org", "--port", "3001"]
).unwrap();

let color: &String = m.get_one("color")
    .expect("default");
assert_eq!(color, "auto");

let hostname: &String = m.get_one("hostname")
    .expect("required");
assert_eq!(hostname, "rust-lang.org");

let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

#### Implementations

- <span id="valueparser-new"></span>`fn new<P>(other: P) -> Self`

  Custom parser for argument values

  

  Pre-existing [`TypedValueParser`](value_parser/index.md) implementations include:

  - `Fn(&str) -> Result<T, E>`

  - [`EnumValueParser`](value_parser/index.md) and  [`PossibleValuesParser`](value_parser/index.md) for static enumerated values

  - [`BoolishValueParser`](value_parser/index.md) and [`FalseyValueParser`](value_parser/index.md) for alternative `bool` implementations

  - [`RangedI64ValueParser`](value_parser/index.md) and [`RangedU64ValueParser`](value_parser/index.md)

  - [`NonEmptyStringValueParser`](value_parser/index.md)

  

  # Example

  

  ```rust

  use clap_builder as clap;

  type EnvVar = (String, Option<String>);

  fn parse_env_var(env: &str) -> Result<EnvVar, std::io::Error> {

      if let Some((var, value)) = env.split_once('=') {

          Ok((var.to_owned(), Some(value.to_owned())))

      } else {

          Ok((env.to_owned(), None))

      }

  }

  

  let mut cmd = clap::Command::new("raw")

      .arg(

          clap::Arg::new("env")

              .value_parser(clap::builder::ValueParser::new(parse_env_var))

              .required(true)

      );

  

  let m = cmd.try_get_matches_from_mut(["cmd", "key=value"]).unwrap();

  let port: &EnvVar = m.get_one("env")

      .expect("required");

  assert_eq!(*port, ("key".into(), Some("value".into())));

  ```

- <span id="valueparser-bool"></span>`const fn bool() -> Self`

  `bool` parser for argument values

  

  See also:

  - [`BoolishValueParser`](value_parser/index.md) for different human readable bool representations

  - [`FalseyValueParser`](value_parser/index.md) for assuming non-false is true

  

  # Example

  

  ```rust

  use clap_builder as clap;

  let mut cmd = clap::Command::new("raw")

      .arg(

          clap::Arg::new("download")

              .value_parser(clap::value_parser!(bool))

              .required(true)

      );

  

  let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();

  let port: bool = *m.get_one("download")

      .expect("required");

  assert_eq!(port, true);

  

  assert!(cmd.try_get_matches_from_mut(["cmd", "forever"]).is_err());

  ```

- <span id="valueparser-string"></span>`const fn string() -> Self`

  [`String`](../../cargo_platform/index.md) parser for argument values

  

  See also:

  - [`NonEmptyStringValueParser`](value_parser/index.md)

  

  # Example

  

  ```rust

  use clap_builder as clap;

  let mut cmd = clap::Command::new("raw")

      .arg(

          clap::Arg::new("port")

              .value_parser(clap::value_parser!(String))

              .required(true)

      );

  

  let m = cmd.try_get_matches_from_mut(["cmd", "80"]).unwrap();

  let port: &String = m.get_one("port")

      .expect("required");

  assert_eq!(port, "80");

  ```

- <span id="valueparser-os-string"></span>`const fn os_string() -> Self`

  `OsString` parser for argument values

  

  # Example

  

  ```rust

  #[cfg(unix)] {

  use clap_builder as clap;

  use clap::{Command, Arg, builder::ValueParser};

  use std::ffi::OsString;

  use std::os::unix::ffi::{OsStrExt,OsStringExt};

  let r = Command::new("myprog")

      .arg(

          Arg::new("arg")

          .required(true)

          .value_parser(ValueParser::os_string())

      )

      .try_get_matches_from(vec![

          OsString::from("myprog"),

          OsString::from_vec(vec![0xe9])

      ]);

  

  assert!(r.is_ok());

  let m = r.unwrap();

  let arg: &OsString = m.get_one("arg")

      .expect("required");

  assert_eq!(arg.as_bytes(), &[0xe9]);

  }

  ```

- <span id="valueparser-path-buf"></span>`const fn path_buf() -> Self`

  `PathBuf` parser for argument values

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use std::path::PathBuf;

  use std::path::Path;

  let mut cmd = clap::Command::new("raw")

      .arg(

          clap::Arg::new("output")

              .value_parser(clap::value_parser!(PathBuf))

              .required(true)

      );

  

  let m = cmd.try_get_matches_from_mut(["cmd", "hello.txt"]).unwrap();

  let port: &PathBuf = m.get_one("output")

      .expect("required");

  assert_eq!(port, Path::new("hello.txt"));

  

  assert!(cmd.try_get_matches_from_mut(["cmd", ""]).is_err());

  ```

#### Trait Implementations

##### `impl Any for ValueParser`

- <span id="valueparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueParser`

- <span id="valueparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueParser`

- <span id="valueparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueParser`

- <span id="valueparser-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ValueParser`

- <span id="valueparser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ValueParser`

- <span id="valueparser-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl<T> From for ValueParser`

- <span id="valueparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ValueParser`

- <span id="valueparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for ValueParser`

- <span id="valueparser-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](resettable/index.md#resettable), [`ValueParser`](value_parser/index.md#valueparser)

##### `impl ToOwned for ValueParser`

- <span id="valueparser-toowned-type-owned"></span>`type Owned = T`

- <span id="valueparser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valueparser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueParser`

- <span id="valueparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valueparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueParser`

- <span id="valueparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valueparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ArgAction`

```rust
enum ArgAction {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
    Help,
    HelpShort,
    HelpLong,
    Version,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/action.rs:34-353`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/action.rs#L34-L353)*

Behavior of arguments when they are encountered while parsing

# Examples

```rust
#[cfg(feature = "help")] {
use clap_builder as clap;
use clap::Command;
use clap::Arg;
let cmd = Command::new("mycmd")
    .arg(
        Arg::new("special-help")
            .short('?')
            .action(clap::ArgAction::Help)
    );

// Existing help still exists
let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);

// New help available
let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}
```

#### Variants

- **`Set`**

  When encountered, store the associated value(s) in `ArgMatches`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::Set)
      );
  
  let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
      vec!["value"]
  );
  ```

- **`Append`**

  When encountered, store the associated value(s) in `ArgMatches`
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::Append)
      );
  
  let matches = cmd.try_get_matches_from(["mycmd", "--flag", "value1", "--flag", "value2"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_many::<String>("flag").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
      vec!["value1", "value2"]
  );
  ```

- **`SetTrue`**

  When encountered, act as if `"true"` was encountered on the command-line
  
  If no `default_value` is set, it will be `false`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
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
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      false
  );
  ```
  
  You can use `TypedValueParser::map` to have the
  flag control an application-specific type:
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  use clap::builder::TypedValueParser as _;
  use clap::builder::BoolishValueParser;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::SetTrue)
              .value_parser(
                  BoolishValueParser::new()
                  .map(|b| -> usize {
                      if b { 10 } else { 5 }
                  })
              )
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_one::<usize>("flag").copied(),
      Some(10)
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_one::<usize>("flag").copied(),
      Some(5)
  );
  ```

- **`SetFalse`**

  When encountered, act as if `"false"` was encountered on the command-line
  
  If no `default_value` is set, it will be `true`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
  <div class="warning">
  
  **NOTE:** If the argument has previously been seen, it will result in a
  `ArgumentConflict` unless
  `Command::args_override_self(true)` is set.
  
  </div>
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("flag")
              .long("flag")
              .action(clap::ArgAction::SetFalse)
      );
  
  let matches = cmd.clone().try_get_matches_from(["mycmd", "--flag"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      false
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_flag("flag"),
      true
  );
  ```

- **`Count`**

  When encountered, increment a `u8` counter starting from `0`.
  
  If no `default_value` is set, it will be `0`.
  
  No value is allowed. To optionally accept a value, see
  `Arg::default_missing_value`
  
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
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_count("flag"),
      2
  );
  
  let matches = cmd.try_get_matches_from(["mycmd"]).unwrap();
  assert!(matches.contains_id("flag"));
  assert_eq!(
      matches.get_count("flag"),
      0
  );
  ```

- **`Help`**

  When encountered, display `Command::print_help`
  
  Depending on the flag, `Command::print_long_help` may be shown
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::Help)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`HelpShort`**

  When encountered, display `Command::print_help`
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::HelpShort)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`HelpLong`**

  When encountered, display `Command::print_long_help`
  
  # Examples
  
  ```rust
  #[cfg(feature = "help")] {
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .arg(
          Arg::new("special-help")
              .short('?')
              .action(clap::ArgAction::HelpLong)
      );
  
  // Existing help still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  
  // New help available
  let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
  }
  ```

- **`Version`**

  When encountered, display `Command::version`
  
  Depending on the flag, `Command::long_version` may be shown
  
  # Examples
  
  ```rust
  use clap_builder as clap;
  use clap::Command;
  use clap::Arg;
  let cmd = Command::new("mycmd")
      .version("1.0.0")
      .arg(
          Arg::new("special-version")
              .long("special-version")
              .action(clap::ArgAction::Version)
      );
  
  // Existing version still exists
  let err = cmd.clone().try_get_matches_from(["mycmd", "--version"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
  
  // New version available
  let err = cmd.try_get_matches_from(["mycmd", "--special-version"]).unwrap_err();
  assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
  ```

#### Implementations

- <span id="argaction-takes-values"></span>`fn takes_values(&self) -> bool`

  Returns whether this action accepts values on the command-line

  

  `default_values` and `env` may still be

  processed.

- <span id="argaction-max-num-args"></span>`fn max_num_args(&self) -> ValueRange` — [`ValueRange`](range/index.md#valuerange)

- <span id="argaction-default-num-args"></span>`fn default_num_args(&self) -> ValueRange` — [`ValueRange`](range/index.md#valuerange)

- <span id="argaction-default-value"></span>`fn default_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-missing-value"></span>`fn default_missing_value(&self) -> Option<&'static std::ffi::OsStr>`

- <span id="argaction-default-value-parser"></span>`fn default_value_parser(&self) -> Option<super::ValueParser>` — [`ValueParser`](value_parser/index.md#valueparser)

- <span id="argaction-value-type-id"></span>`fn value_type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](../util/any_value/index.md#anyvalueid)

#### Trait Implementations

##### `impl Any for ArgAction`

- <span id="argaction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgAction`

- <span id="argaction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgAction`

- <span id="argaction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgAction`

- <span id="argaction-clone"></span>`fn clone(&self) -> ArgAction` — [`ArgAction`](action/index.md#argaction)

##### `impl CloneToUninit for ArgAction`

- <span id="argaction-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgAction`

- <span id="argaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ArgAction`

- <span id="argaction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgAction`

- <span id="argaction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Option<crate::builder::ArgAction>`

- <span id="option-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ArgAction>` — [`Resettable`](resettable/index.md#resettable), [`ArgAction`](action/index.md#argaction)

##### `impl ToOwned for ArgAction`

- <span id="argaction-toowned-type-owned"></span>`type Owned = T`

- <span id="argaction-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argaction-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgAction`

- <span id="argaction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argaction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgAction`

- <span id="argaction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argaction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArgPredicate`

```rust
enum ArgPredicate {
    IsPresent,
    Equals(crate::builder::OsStr),
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_predicate.rs:8-13`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_predicate.rs#L8-L13)*

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl Any for ArgPredicate`

- <span id="argpredicate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgPredicate`

- <span id="argpredicate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgPredicate`

- <span id="argpredicate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgPredicate`

- <span id="argpredicate-clone"></span>`fn clone(&self) -> ArgPredicate` — [`ArgPredicate`](arg_predicate/index.md#argpredicate)

##### `impl CloneToUninit for ArgPredicate`

- <span id="argpredicate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgPredicate`

- <span id="argpredicate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgPredicate`

##### `impl<T> From for ArgPredicate`

- <span id="argpredicate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgPredicate`

- <span id="argpredicate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgPredicate`

- <span id="argpredicate-partialeq-eq"></span>`fn eq(&self, other: &ArgPredicate) -> bool` — [`ArgPredicate`](arg_predicate/index.md#argpredicate)

##### `impl StructuralPartialEq for ArgPredicate`

##### `impl ToOwned for ArgPredicate`

- <span id="argpredicate-toowned-type-owned"></span>`type Owned = T`

- <span id="argpredicate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argpredicate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgPredicate`

- <span id="argpredicate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argpredicate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgPredicate`

- <span id="argpredicate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argpredicate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Resettable<T>`

```rust
enum Resettable<T> {
    Value(T),
    Reset,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/resettable.rs:33-38`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/resettable.rs#L33-L38)*

Clearable builder value

This allows a builder function to both accept any value that can `Into::into` `T` (like
`&str` into `OsStr`) as well as `None` to reset it to the default.  This is needed to
workaround a limitation where you can't have a function argument that is `impl Into<Option<T>>`
where `T` is `impl Into<S>` accept `None` as its type is ambiguous.

# Example

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
fn common() -> Command {
    Command::new("cli")
        .arg(Arg::new("input").short('i').long("input"))
}
let mut command = common();
command.mut_arg("input", |arg| arg.short(None));
```

#### Variants

- **`Value`**

  Overwrite builder value

- **`Reset`**

  Reset builder value

#### Implementations

- <span id="resettable-into-option"></span>`fn into_option(self) -> Option<T>`

#### Trait Implementations

##### `impl<T> Any for Resettable<T>`

- <span id="resettable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Resettable<T>`

- <span id="resettable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Resettable<T>`

- <span id="resettable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Resettable<T>`

- <span id="resettable-clone"></span>`fn clone(&self) -> Resettable<T>` — [`Resettable`](resettable/index.md#resettable)

##### `impl<T> CloneToUninit for Resettable<T>`

- <span id="resettable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for Resettable<T>`

##### `impl<T: fmt::Debug> Debug for Resettable<T>`

- <span id="resettable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Resettable<T>`

##### `impl<T> From for Resettable<T>`

- <span id="resettable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for Resettable<T>`

- <span id="resettable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for Resettable<T>`

- <span id="resettable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoResettable for Resettable<T>`

- <span id="resettable-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<T>` — [`Resettable`](resettable/index.md#resettable)

##### `impl<T: cmp::Ord> Ord for Resettable<T>`

- <span id="resettable-ord-cmp"></span>`fn cmp(&self, other: &Resettable<T>) -> cmp::Ordering` — [`Resettable`](resettable/index.md#resettable)

##### `impl<T: cmp::PartialEq> PartialEq for Resettable<T>`

- <span id="resettable-partialeq-eq"></span>`fn eq(&self, other: &Resettable<T>) -> bool` — [`Resettable`](resettable/index.md#resettable)

##### `impl<T: cmp::PartialOrd> PartialOrd for Resettable<T>`

- <span id="resettable-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Resettable<T>) -> option::Option<cmp::Ordering>` — [`Resettable`](resettable/index.md#resettable)

##### `impl<T> StructuralPartialEq for Resettable<T>`

##### `impl<T> ToOwned for Resettable<T>`

- <span id="resettable-toowned-type-owned"></span>`type Owned = T`

- <span id="resettable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resettable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Resettable<T>`

- <span id="resettable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resettable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Resettable<T>`

- <span id="resettable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resettable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ValueHint`

```rust
enum ValueHint {
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_hint.rs:29-68`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_hint.rs#L29-L68)*

Provide shell with hint on how to complete an argument.

See `Arg::value_hint` to set this on an argument.

See the `clap_complete` crate for completion script generation.

Overview of which hints are supported by which shell:

| Hint                   | zsh | fish[^1] | dynamic |
| ---------------------- | --- | ---------|---------|
| `AnyPath`              | Yes | Yes      | Yes     |
| `FilePath`             | Yes | Yes      | Yes     |
| `DirPath`              | Yes | Yes      | Yes     |
| `ExecutablePath`       | Yes | Partial  | Yes     |
| `CommandName`          | Yes | Yes      | No      |
| `CommandString`        | Yes | Partial  | No      |
| `CommandWithArguments` | Yes |          | No      |
| `Username`             | Yes | Yes      | No      |
| `Hostname`             | Yes | Yes      | No      |
| `Url`                  | Yes |          | No      |
| `EmailAddress`         | Yes |          | No      |

[^1]: fish completions currently only support named arguments (e.g. -o or --opt), not
      positional arguments.

#### Variants

- **`Unknown`**

  Default value if hint is not specified. Follows shell default behavior, which is usually
  auto-completing filenames.

- **`Other`**

  None of the hints below apply. Disables shell completion for this argument.

- **`AnyPath`**

  Any existing path.

- **`FilePath`**

  Path to a file.

- **`DirPath`**

  Path to a directory.

- **`ExecutablePath`**

  Path to an executable file.

- **`CommandName`**

  Name of a command, without arguments. May be relative to PATH, or full path to executable.

- **`CommandString`**

  A single string containing a command and its arguments.

- **`CommandWithArguments`**

  Capture the remaining arguments as a command name and arguments for that command. This is
  common when writing shell wrappers that execute anther command, for example `sudo` or `env`.
  
  This hint is special, the argument must be a positional argument and have
  `.num_args(1..)` and Command must use `Command::trailing_var_arg(true)`. The result is that the
  command line `my_app ls -la /` will be parsed as `["ls", "-la", "/"]` and clap won't try to
  parse the `-la` argument itself.
  
  

- **`Username`**

  Name of a local operating system user.

- **`Hostname`**

  Host name of a computer.
  Shells usually parse `/etc/hosts` and `.ssh/known_hosts` to complete hostnames.

- **`Url`**

  Complete web address.

- **`EmailAddress`**

  Email address.

#### Trait Implementations

##### `impl Any for ValueHint`

- <span id="valuehint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueHint`

- <span id="valuehint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueHint`

- <span id="valuehint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueHint`

- <span id="valuehint-clone"></span>`fn clone(&self) -> ValueHint` — [`ValueHint`](value_hint/index.md#valuehint)

##### `impl CloneToUninit for ValueHint`

- <span id="valuehint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- <span id="valuehint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ValueHint`

- <span id="valuehint-default"></span>`fn default() -> ValueHint` — [`ValueHint`](value_hint/index.md#valuehint)

##### `impl Eq for ValueHint`

##### `impl<T> From for ValueHint`

- <span id="valuehint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for ValueHint`

- <span id="valuehint-fromstr-type-err"></span>`type Err = String`

- <span id="valuehint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- <span id="valuehint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ValueHint`

- <span id="valuehint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Option<crate::builder::ValueHint>`

- <span id="option-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueHint>` — [`Resettable`](resettable/index.md#resettable), [`ValueHint`](value_hint/index.md#valuehint)

##### `impl PartialEq for ValueHint`

- <span id="valuehint-partialeq-eq"></span>`fn eq(&self, other: &ValueHint) -> bool` — [`ValueHint`](value_hint/index.md#valuehint)

##### `impl StructuralPartialEq for ValueHint`

##### `impl ToOwned for ValueHint`

- <span id="valuehint-toowned-type-owned"></span>`type Owned = T`

- <span id="valuehint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuehint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueHint`

- <span id="valuehint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuehint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueHint`

- <span id="valuehint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuehint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IntoResettable<T>`

```rust
trait IntoResettable<T> { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/resettable.rs:65-68`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/resettable.rs#L65-L68)*

Convert to the intended resettable type

#### Required Methods

- `fn into_resettable(self) -> Resettable<T>`

  Convert to the intended resettable type

#### Implementors

- [`ArgAction`](action/index.md#argaction)
- [`Resettable`](resettable/index.md#resettable)
- [`ValueHint`](value_hint/index.md#valuehint)
- `I`
- `Option<&'static str>`
- `Option<char>`
- `Option<crate::builder::ArgAction>`
- `Option<crate::builder::ValueHint>`
- `Option<crate::builder::ValueParser>`
- `Option<usize>`
- `char`
- `usize`

### `TypedValueParser`

```rust
trait TypedValueParser: Clone + Send + Sync + 'static { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:711-868`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L711-L868)*

Parse/validate argument values

As alternatives to implementing `TypedValueParser`,
- Use `Fn(&str) -> Result<T, E>` which implements `TypedValueParser`
- `TypedValueParser::map` or `TypedValueParser::try_map` to adapt an existing `TypedValueParser`

See `ValueParserFactory` to register `TypedValueParser::Value` with
`value_parser!`.

# Example

```rust
#[cfg(feature = "error-context")] {
use clap_builder as clap;
use clap::error::ErrorKind;
use clap::error::ContextKind;
use clap::error::ContextValue;
#[derive(Clone)]
struct Custom(u32);

#[derive(Clone)]
struct CustomValueParser;

impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;

        const INVALID_VALUE: u32 = 10;
        if val == INVALID_VALUE {
            let mut err = clap::Error::new(ErrorKind::ValueValidation)
                .with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(ContextKind::InvalidArg, ContextValue::String(arg.to_string()));
            }
            err.insert(ContextKind::InvalidValue, ContextValue::String(INVALID_VALUE.to_string()));
            return Err(err);
        }

        Ok(Custom(val))
    }
}
}
```

#### Associated Types

- `type Value: 3`

#### Required Methods

- `fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

#### Provided Methods

- `fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

  Reflect on enumerated value properties

- `fn map<T, F>(self, func: F) -> MapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

- `fn try_map<T, E, F>(self, func: F) -> TryMapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

#### Implementors

- [`BoolValueParser`](value_parser/index.md#boolvalueparser)
- [`BoolishValueParser`](value_parser/index.md#boolishvalueparser)
- [`EnumValueParser`](value_parser/index.md#enumvalueparser)
- [`FalseyValueParser`](value_parser/index.md#falseyvalueparser)
- [`MapValueParser`](value_parser/index.md#mapvalueparser)
- [`NonEmptyStringValueParser`](value_parser/index.md#nonemptystringvalueparser)
- [`OsStringValueParser`](value_parser/index.md#osstringvalueparser)
- [`PathBufValueParser`](value_parser/index.md#pathbufvalueparser)
- [`PossibleValuesParser`](value_parser/index.md#possiblevaluesparser)
- [`RangedI64ValueParser`](value_parser/index.md#rangedi64valueparser)
- [`RangedU64ValueParser`](value_parser/index.md#rangedu64valueparser)
- [`StringValueParser`](value_parser/index.md#stringvalueparser)
- [`TryMapValueParser`](value_parser/index.md#trymapvalueparser)
- [`UnknownArgumentValueParser`](value_parser/index.md#unknownargumentvalueparser)
- `F`

### `ValueParserFactory`

```rust
trait ValueParserFactory { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2276-2285`](../../../.source_1765521767/clap_builder-4.5.53/src/builder/value_parser.rs#L2276-L2285)*

Register a type with [`value_parser!`][crate::value_parser!]

# Example

```rust
use clap_builder as clap;
#[derive(Copy, Clone, Debug)]
pub struct Custom(u32);

impl clap::builder::ValueParserFactory for Custom {
    type Parser = CustomValueParser;
    fn value_parser() -> Self::Parser {
        CustomValueParser
    }
}

#[derive(Clone, Debug)]
pub struct CustomValueParser;
impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Custom(val))
    }
}

let parser: CustomValueParser = clap::value_parser!(Custom);
```

#### Associated Types

- `type Parser`

#### Required Methods

- `fn value_parser() -> <Self as >::Parser`

  Create the specified `Self::Parser`

#### Implementors

- `Box<T>`
- `Box<std::ffi::OsStr>`
- `Box<std::path::Path>`
- `Box<str>`
- `String`
- `bool`
- `i16`
- `i32`
- `i64`
- `i8`
- `std::ffi::OsString`
- `std::num::Saturating<T>`
- `std::num::Wrapping<T>`
- `std::path::PathBuf`
- `std::sync::Arc<T>`
- `u16`
- `u32`
- `u64`
- `u8`

