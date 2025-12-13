*[clap_builder](../../index.md) / [builder](../index.md) / [arg](index.md)*

---

# Module `arg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Arg`](#arg) | struct | The abstract representation of a command line argument. |

## Structs

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

*Defined in [`clap_builder-4.5.53/src/builder/arg.rs:60-92`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/arg.rs#L60-L92)*

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](#arg)s, using the builder pattern and setting options
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

- <span id="arg-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

  Create a new [`Arg`](#arg) with a unique name.

  

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

- <span id="arg-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

  Set the identifier used for referencing this argument in the clap API.

  

  See `Arg::new` for more details.

- <span id="arg-short"></span>`fn short(self, s: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

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

- <span id="arg-long"></span>`fn long(self, l: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

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

- <span id="arg-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

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

- <span id="arg-short-alias"></span>`fn short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

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

- <span id="arg-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

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

- <span id="arg-visible-alias"></span>`fn visible_alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

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

- <span id="arg-visible-short-alias"></span>`fn visible_short_alias(self, name: impl IntoResettable<char>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

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

- <span id="arg-visible-aliases"></span>`fn visible_aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

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

- <span id="arg-index"></span>`fn index(self, idx: impl IntoResettable<usize>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable)

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

  

  [`Command`](../../index.md) will [`panic!`](../../../miette/panic/index.md) if indexes are skipped (such as defining `index(1)` and `index(3)`

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

  they will receive an [`UnknownArgument`](../../index.md) error. Setting an argument to `.last(true)` also

  allows one to access this arg early using the `--` syntax. Accessing an arg early, even with

  the `--` syntax is otherwise not possible.

  

  <div class="warning">

  

  **NOTE:** This will change the usage string to look like `$ prog [OPTIONS] [-- <ARG>]` if

  `ARG` is marked as `.last(true)`.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** This setting will imply [`crate::Command::dont_collapse_args_in_usage`](../../index.md#dont-collapse-args-in-usage) because failing

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

  [`crate::Command::args_conflicts_with_subcommands`](../../index.md#args-conflicts-with-subcommands)

  (or [`crate::Command::subcommand_negates_reqs`](../../index.md#subcommand-negates-reqs) if the argument marked `Last` is also

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

  

  Setting `last` ensures the arg has the highest [`index`](../../../gimli/read/index/index.md) of all positional args

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

- <span id="arg-requires"></span>`fn requires(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Id`](../../util/id/index.md#id)

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

  Specifies that an argument can be matched to all child [`Subcommand`](../../derive/index.md)s.

  

  <div class="warning">

  

  **NOTE:** Global arguments *only* propagate down, **not** up (to parent commands), however

  their values once a user uses them will be propagated back up to parents. In effect, this

  means one should *define* all global arguments at the top level, however it doesn't matter

  where the user *uses* the global argument.

  

  </div>

  

  # Examples

  

  Assume an application with two subcommands, and you'd like to define a

  `--verbose` flag that can be called on any of the subcommands and parent, but you don't

  want to clutter the source with three duplicate [`Arg`](#arg) definitions.

  

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

- <span id="arg-is-set"></span>`fn is_set(&self, s: ArgSettings) -> bool` — [`ArgSettings`](../arg_settings/index.md#argsettings)

- <span id="arg-setting"></span>`fn setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](../arg_settings/index.md#argsettings)

- <span id="arg-unset-setting"></span>`fn unset_setting(self, setting: ArgSettings) -> Self` — [`ArgSettings`](../arg_settings/index.md#argsettings)

#### Trait Implementations

##### `impl Any for Arg`

- <span id="arg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Arg`

- <span id="arg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Arg`

- <span id="arg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Arg`

- <span id="arg-clone"></span>`fn clone(&self) -> Arg` — [`Arg`](#arg)

##### `impl CloneToUninit for Arg`

- <span id="arg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Arg`

- <span id="arg-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- <span id="arg-default"></span>`fn default() -> Arg` — [`Arg`](#arg)

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

- <span id="arg-ord-cmp"></span>`fn cmp(&self, other: &Arg) -> Ordering` — [`Arg`](#arg)

##### `impl PartialEq for Arg`

- <span id="arg-partialeq-eq"></span>`fn eq(&self, other: &Arg) -> bool` — [`Arg`](#arg)

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

