*[clap_builder](../index.md) / [builder](index.md)*

---

# Module `builder`

Define [`Command`](../index.md) line [arguments][`Arg`](../index.md)

## Modules

- [`styling`](styling/index.md) - Terminal [`Styles`] for help and error output

## Structs

### `Str`

```rust
struct Str {
    // [REDACTED: Private Fields]
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `String`), enable the `string`
feature

</div>

#### Implementations

- `fn as_str(self: &Self) -> &str`
  Get the raw string of the `Str`

#### Trait Implementations

##### `impl From`

- `fn from(name: Id) -> Self`

##### `impl From`

- `fn from(name: &'static str) -> Self`

##### `impl From`

- `fn from(name: &&'static str) -> Self`

##### `impl From`

- `fn from(id: &Str) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<Str>`

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<String>`

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<Id>`

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<OsStr>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &std::ffi::OsStr`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &[u8]`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &std::path::Path`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Str`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Str) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &std::ffi::OsStr) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Id) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&std::ffi::OsStr) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Str) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default`

- `fn default() -> Str`

##### `impl Deref`

- `type Target = str`

- `fn deref(self: &Self) -> &str`

### `Arg`

```rust
struct Arg {
    // [REDACTED: Private Fields]
}
```

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](../index.md)s, using the builder pattern and setting options
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

- `fn action(self: Self, action: impl IntoResettable<ArgAction>) -> Self`
  Specify how to react to an argument when parsing it.

- `fn value_parser(self: Self, parser: impl IntoResettable<super::ValueParser>) -> Self`
  Specify the typed behavior of the argument.

- `fn num_args(self: Self, qty: impl IntoResettable<ValueRange>) -> Self`
  Specifies the number of arguments parsed per occurrence

- `fn value_name(self: Self, name: impl IntoResettable<Str>) -> Self`
  Placeholder for the argument's value in the help message / usage.

- `fn value_names(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Placeholders for the argument's values in the help message / usage.

- `fn value_hint(self: Self, value_hint: impl IntoResettable<ValueHint>) -> Self`
  Provide the shell a hint about how to complete this argument.

- `fn ignore_case(self: Self, yes: bool) -> Self`
  Match values against [`PossibleValuesParser`][crate::builder::PossibleValuesParser] without matching case.

- `fn allow_hyphen_values(self: Self, yes: bool) -> Self`
  Allows values which start with a leading hyphen (`-`)

- `fn allow_negative_numbers(self: Self, yes: bool) -> Self`
  Allows negative numbers to pass as values.

- `fn require_equals(self: Self, yes: bool) -> Self`
  Requires that options use the `--option=val` syntax

- `fn value_delimiter(self: Self, d: impl IntoResettable<char>) -> Self`
  Allow grouping of multiple values via a delimiter.

- `fn value_terminator(self: Self, term: impl IntoResettable<Str>) -> Self`
  Sentinel to **stop** parsing multiple values of a given argument.

- `fn raw(self: Self, yes: bool) -> Self`
  Consume all following arguments.

- `fn default_value(self: Self, val: impl IntoResettable<OsStr>) -> Self`
  Value for the argument when not present.

- `fn default_values(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when not present.

- `fn default_missing_value(self: Self, val: impl IntoResettable<OsStr>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_value_os(self: Self, val: impl Into<OsStr>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_values(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn default_missing_values_os(self: Self, vals: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Value for the argument when the flag is present but no value is specified.

- `fn new(id: impl Into<Id>) -> Self`
  Create a new [`Arg`] with a unique name.

- `fn id(self: Self, id: impl Into<Id>) -> Self`
  Set the identifier used for referencing this argument in the clap API.

- `fn short(self: Self, s: impl IntoResettable<char>) -> Self`
  Sets the short version of the argument without the preceding `-`.

- `fn long(self: Self, l: impl IntoResettable<Str>) -> Self`
  Sets the long version of the argument without the preceding `--`.

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a hidden long flag.

- `fn short_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as a hidden short flag.

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as hidden long flags.

- `fn short_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which functions as a hidden short flag.

- `fn visible_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a visible long flag.

- `fn visible_short_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as a visible short flag.

- `fn visible_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as visible long flags.

- `fn visible_short_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as visible short flags.

- `fn index(self: Self, idx: impl IntoResettable<usize>) -> Self`
  Specifies the index of a positional argument **starting at** 1.

- `fn trailing_var_arg(self: Self, yes: bool) -> Self`
  This is a "var arg" and everything that follows should be captured by it, as if the user had

- `fn last(self: Self, yes: bool) -> Self`
  This arg is the last, or final, positional argument (i.e. has the highest

- `fn required(self: Self, yes: bool) -> Self`
  Specifies that the argument must be present.

- `fn requires(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Sets an argument that is required when this one is present

- `fn exclusive(self: Self, yes: bool) -> Self`
  This argument must be passed alone; it conflicts with all other arguments.

- `fn global(self: Self, yes: bool) -> Self`
  Specifies that an argument can be matched to all child [`Subcommand`]s.

- `fn help(self: Self, h: impl IntoResettable<StyledStr>) -> Self`
  Sets the description of the argument for short help (`-h`).

- `fn long_help(self: Self, h: impl IntoResettable<StyledStr>) -> Self`
  Sets the description of the argument for long help (`--help`).

- `fn display_order(self: Self, ord: impl IntoResettable<usize>) -> Self`
  Allows custom ordering of args within the help message.

- `fn help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Override the `--help` section this appears in.

- `fn next_line_help(self: Self, yes: bool) -> Self`
  Render the [help][Arg::help] on the line after the argument.

- `fn hide(self: Self, yes: bool) -> Self`
  Do not display the argument in help message.

- `fn hide_possible_values(self: Self, yes: bool) -> Self`
  Do not display the [possible values][crate::builder::ValueParser::possible_values] in the help message.

- `fn hide_default_value(self: Self, yes: bool) -> Self`
  Do not display the default value of the argument in the help message.

- `fn hide_short_help(self: Self, yes: bool) -> Self`
  Hides an argument from short help (`-h`).

- `fn hide_long_help(self: Self, yes: bool) -> Self`
  Hides an argument from long help (`--help`).

- `fn get_id(self: &Self) -> &Id`
  Get the name of the argument

- `fn get_help(self: &Self) -> Option<&StyledStr>`
  Get the help specified for this argument, if any

- `fn get_long_help(self: &Self) -> Option<&StyledStr>`
  Get the long help specified for this argument, if any

- `fn get_display_order(self: &Self) -> usize`
  Get the placement within help

- `fn get_help_heading(self: &Self) -> Option<&str>`
  Get the help heading specified for this argument, if any

- `fn get_short(self: &Self) -> Option<char>`
  Get the short option name for this argument, if any

- `fn get_visible_short_aliases(self: &Self) -> Option<Vec<char>>`
  Get visible short aliases for this argument, if any

- `fn get_all_short_aliases(self: &Self) -> Option<Vec<char>>`
  Get *all* short aliases for this argument, if any, both visible and hidden.

- `fn get_short_and_visible_aliases(self: &Self) -> Option<Vec<char>>`
  Get the short option name and its visible aliases, if any

- `fn get_long(self: &Self) -> Option<&str>`
  Get the long option name for this argument, if any

- `fn get_visible_aliases(self: &Self) -> Option<Vec<&str>>`
  Get visible aliases for this argument, if any

- `fn get_all_aliases(self: &Self) -> Option<Vec<&str>>`
  Get *all* aliases for this argument, if any, both visible and hidden.

- `fn get_long_and_visible_aliases(self: &Self) -> Option<Vec<&str>>`
  Get the long option name and its visible aliases, if any

- `fn get_aliases(self: &Self) -> Option<Vec<&str>>`
  Get hidden aliases for this argument, if any

- `fn get_possible_values(self: &Self) -> Vec<PossibleValue>`
  Get the names of possible values for this argument. Only useful for user

- `fn get_value_names(self: &Self) -> Option<&[Str]>`
  Get the names of values for this argument.

- `fn get_num_args(self: &Self) -> Option<ValueRange>`
  Get the number of values for this argument.

- `fn get_value_delimiter(self: &Self) -> Option<char>`
  Get the delimiter between multiple values

- `fn get_value_terminator(self: &Self) -> Option<&Str>`
  Get the value terminator for this argument. The `value_terminator` is a value

- `fn get_index(self: &Self) -> Option<usize>`
  Get the index of this argument, if any

- `fn get_value_hint(self: &Self) -> ValueHint`
  Get the value hint of this argument

- `fn get_default_values(self: &Self) -> &[OsStr]`
  Get the default values specified for this argument, if any

- `fn is_positional(self: &Self) -> bool`
  Checks whether this argument is a positional or not.

- `fn is_required_set(self: &Self) -> bool`
  Reports whether [`Arg::required`] is set

- `fn is_allow_hyphen_values_set(self: &Self) -> bool`
  Report whether [`Arg::allow_hyphen_values`] is set

- `fn is_allow_negative_numbers_set(self: &Self) -> bool`
  Report whether [`Arg::allow_negative_numbers`] is set

- `fn get_action(self: &Self) -> &ArgAction`
  Behavior when parsing the argument

- `fn get_value_parser(self: &Self) -> &super::ValueParser`
  Configured parser for argument values

- `fn is_global_set(self: &Self) -> bool`
  Report whether [`Arg::global`] is set

- `fn is_next_line_help_set(self: &Self) -> bool`
  Report whether [`Arg::next_line_help`] is set

- `fn is_hide_set(self: &Self) -> bool`
  Report whether [`Arg::hide`] is set

- `fn is_hide_default_value_set(self: &Self) -> bool`
  Report whether [`Arg::hide_default_value`] is set

- `fn is_hide_possible_values_set(self: &Self) -> bool`
  Report whether [`Arg::hide_possible_values`] is set

- `fn is_hide_short_help_set(self: &Self) -> bool`
  Report whether [`Arg::hide_short_help`] is set

- `fn is_hide_long_help_set(self: &Self) -> bool`
  Report whether [`Arg::hide_long_help`] is set

- `fn is_require_equals_set(self: &Self) -> bool`
  Report whether [`Arg::require_equals`] is set

- `fn is_exclusive_set(self: &Self) -> bool`
  Reports whether [`Arg::exclusive`] is set

- `fn is_trailing_var_arg_set(self: &Self) -> bool`
  Report whether [`Arg::trailing_var_arg`] is set

- `fn is_last_set(self: &Self) -> bool`
  Reports whether [`Arg::last`] is set

- `fn is_ignore_case_set(self: &Self) -> bool`
  Reports whether [`Arg::ignore_case`] is set

- `fn group(self: Self, group_id: impl IntoResettable<Id>) -> Self`
  The name of the [`ArgGroup`] the argument belongs to.

- `fn groups(self: Self, group_ids: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  The names of [`ArgGroup`]'s the argument belongs to.

- `fn default_value_if(self: Self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, default: impl IntoResettable<OsStr>) -> Self`
  Specifies the value of the argument if `arg` has been used at runtime.

- `fn default_values_if(self: Self, arg_id: impl Into<Id>, predicate: impl Into<ArgPredicate>, defaults: impl IntoIterator<Item = impl Into<OsStr>>) -> Self`
  Specifies the values of the argument if `arg` has been used at runtime.

- `fn default_value_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>)>) -> Self`
  Specifies multiple values and conditions in the same manner as [`Arg::default_value_if`].

- `fn default_values_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoIterator<Item = impl Into<OsStr>>)>) -> Self`
  Specifies multiple values and conditions in the same manner as [`Arg::default_values_if`].

- `fn required_unless_present(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Set this arg as [required] as long as the specified argument is not present at runtime.

- `fn required_unless_present_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets this arg as [required] unless *all* of the specified arguments are present at runtime.

- `fn required_unless_present_any(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets this arg as [required] unless *any* of the specified arguments are present at runtime.

- `fn required_if_eq(self: Self, arg_id: impl Into<Id>, val: impl Into<OsStr>) -> Self`
  This argument is [required] only if the specified `arg` is present at runtime and its value

- `fn required_if_eq_any(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self`
  Specify this argument is [required] based on multiple conditions.

- `fn required_if_eq_all(self: Self, ifs: impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>) -> Self`
  Specify this argument is [required] based on multiple conditions.

- `fn requires_if(self: Self, val: impl Into<ArgPredicate>, arg_id: impl Into<Id>) -> Self`
  Require another argument if this arg matches the [`ArgPredicate`]

- `fn requires_ifs(self: Self, ifs: impl IntoIterator<Item = (impl Into<ArgPredicate>, impl Into<Id>)>) -> Self`
  Allows multiple conditional requirements.

- `fn conflicts_with(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  This argument is mutually exclusive with the specified argument.

- `fn conflicts_with_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  This argument is mutually exclusive with the specified arguments.

- `fn overrides_with(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Sets an overridable argument.

- `fn overrides_with_all(self: Self, names: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Sets multiple mutually overridable arguments by name.

#### Trait Implementations

##### `impl From`

- `fn from(a: &Arg) -> Self`

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

- `fn clone(self: &Self) -> Arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Arg) -> Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Arg) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

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

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default`

- `fn default() -> Arg`

### `ArgGroup`

```rust
struct ArgGroup {
    // [REDACTED: Private Fields]
}
```

Specifies a logical group of [arguments](#arguments)

You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts](#conflicts) or [requirements](#requirements)
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
[arguments](#arguments): crate::Arg
[conflicts](#conflicts): crate::Arg::conflicts_with()
[requirements](#requirements): crate::Arg::requires()

#### Implementations

- `fn new(id: impl Into<Id>) -> Self`
  Create a `ArgGroup` using a unique name.

- `fn id(self: Self, id: impl Into<Id>) -> Self`
  Sets the group name.

- `fn arg(self: Self, arg_id: impl IntoResettable<Id>) -> Self`
  Adds an [argument] to this group by name

- `fn args(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Adds multiple [arguments] to this group by name

- `fn get_args(self: &Self) -> impl Iterator<Item = &Id>`
  Getters for all args. It will return a vector of `Id`

- `fn multiple(self: Self, yes: bool) -> Self`
  Allows more than one of the [`Arg`]s in this group to be used. (Default: `false`)

- `fn is_multiple(self: &mut Self) -> bool`
  Return true if the group allows more than one of the arguments

- `fn required(self: Self, yes: bool) -> Self`
  Require an argument from the group to be present when parsing.

- `fn requires(self: Self, id: impl IntoResettable<Id>) -> Self`
  Specify an argument or group that must be present when this group is.

- `fn requires_all(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Specify arguments or groups that must be present when this group is.

- `fn conflicts_with(self: Self, id: impl IntoResettable<Id>) -> Self`
  Specify an argument or group that must **not** be present when this group is.

- `fn conflicts_with_all(self: Self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self`
  Specify arguments or groups that must **not** be present when this group is.

- `fn get_id(self: &Self) -> &Id`
  Get the name of the group

- `fn is_required_set(self: &Self) -> bool`
  Reports whether [`ArgGroup::required`] is set

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(g: &ArgGroup) -> Self`

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

- `fn clone(self: &Self) -> ArgGroup`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgGroup) -> bool`

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

- `fn default() -> ArgGroup`

### `Command`

```rust
struct Command {
    // [REDACTED: Private Fields]
}
```

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

- `fn name(self: Self, name: impl Into<Str>) -> Self`
  (Re)Sets the program's name.

- `fn bin_name(self: Self, name: impl IntoResettable<String>) -> Self`
  Overrides the runtime-determined name of the binary for help and error messages.

- `fn display_name(self: Self, name: impl IntoResettable<String>) -> Self`
  Overrides the runtime-determined display name of the program for help and error messages.

- `fn author(self: Self, author: impl IntoResettable<Str>) -> Self`
  Sets the author(s) for the help message.

- `fn about(self: Self, about: impl IntoResettable<StyledStr>) -> Self`
  Sets the program's description for the short help (`-h`).

- `fn long_about(self: Self, long_about: impl IntoResettable<StyledStr>) -> Self`
  Sets the program's description for the long help (`--help`).

- `fn after_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for after auto-generated short help (`-h`).

- `fn after_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for after auto-generated long help (`--help`).

- `fn before_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for before auto-generated short help (`-h`).

- `fn before_long_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Free-form help text for before auto-generated long help (`--help`).

- `fn version(self: Self, ver: impl IntoResettable<Str>) -> Self`
  Sets the version for the short version (`-V`) and help messages.

- `fn long_version(self: Self, ver: impl IntoResettable<Str>) -> Self`
  Sets the version for the long version (`--version`) and help messages.

- `fn override_usage(self: Self, usage: impl IntoResettable<StyledStr>) -> Self`
  Overrides the `clap` generated usage string for help and error messages.

- `fn override_help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Overrides the `clap` generated help message (both `-h` and `--help`).

- `fn help_template(self: Self, s: impl IntoResettable<StyledStr>) -> Self`
  Sets the help template to be used, overriding the default format.

- `fn flatten_help(self: Self, yes: bool) -> Self`
  Flatten subcommand help into the current command's help

- `fn next_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Set the default section heading for future args.

- `fn next_display_order(self: Self, disp_ord: impl IntoResettable<usize>) -> Self`
  Change the starting value for assigning future display orders for args.

- `fn arg_required_else_help(self: Self, yes: bool) -> Self`
  Exit gracefully if no arguments are present (e.g. `$ myprog`).

- `fn allow_missing_positional(self: Self, yes: bool) -> Self`
  Allows one to implement two styles of CLIs where positionals can be used out of order.

- `fn new(name: impl Into<Str>) -> Self`
  Creates a new instance of an `Command`.

- `fn arg(self: Self, a: impl Into<Arg>) -> Self`
  Adds an [argument] to the list of valid possibilities.

- `fn args(self: Self, args: impl IntoIterator<Item = impl Into<Arg>>) -> Self`
  Adds multiple [arguments] to the list of valid possibilities.

- `fn mut_arg<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate an [`Arg`] after it's been added to a [`Command`].

- `fn mut_args<F>(self: Self, f: F) -> Self`
  Allows one to mutate all [`Arg`]s after they've been added to a [`Command`].

- `fn mut_group<F>(self: Self, arg_id: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate an [`ArgGroup`] after it's been added to a [`Command`].

- `fn mut_subcommand<F>(self: Self, name: impl AsRef<str>, f: F) -> Self`
  Allows one to mutate a [`Command`] after it's been added as a subcommand.

- `fn mut_subcommands<F>(self: Self, f: F) -> Self`
  Allows one to mutate all [`Command`]s after they've been added as subcommands.

- `fn group(self: Self, group: impl Into<ArgGroup>) -> Self`
  Adds an [`ArgGroup`] to the application.

- `fn groups(self: Self, groups: impl IntoIterator<Item = impl Into<ArgGroup>>) -> Self`
  Adds multiple [`ArgGroup`]s to the [`Command`] at once.

- `fn subcommand(self: Self, subcmd: impl Into<Command>) -> Self`
  Adds a subcommand to the list of valid possibilities.

- `fn subcommands(self: Self, subcmds: impl IntoIterator<Item = impl Into<Self>>) -> Self`
  Adds multiple subcommands to the list of valid possibilities.

- `fn defer(self: Self, deferred: fn(Command) -> Command) -> Self`
  Delay initialization for parts of the `Command`

- `fn debug_assert(self: Self)`
  Catch problems earlier in the development cycle.

- `fn error(self: &mut Self, kind: ErrorKind, message: impl fmt::Display) -> Error`
  Custom error message for post-parsing validation

- `fn get_matches(self: Self) -> ArgMatches`
  Parse [`env::args_os`], [exiting][Error::exit] on failure.

- `fn get_matches_mut(self: &mut Self) -> ArgMatches`
  Parse [`env::args_os`], [exiting][Error::exit] on failure.

- `fn try_get_matches(self: Self) -> ClapResult<ArgMatches>`
  Parse [`env::args_os`], returning a [`clap::Result`] on failure.

- `fn get_matches_from<I, T>(self: Self, itr: I) -> ArgMatches`
  Parse the specified arguments, [exiting][Error::exit] on failure.

- `fn try_get_matches_from<I, T>(self: Self, itr: I) -> ClapResult<ArgMatches>`
  Parse the specified arguments, returning a [`clap::Result`] on failure.

- `fn try_get_matches_from_mut<I, T>(self: &mut Self, itr: I) -> ClapResult<ArgMatches>`
  Parse the specified arguments, returning a [`clap::Result`] on failure.

- `fn print_help(self: &mut Self) -> io::Result<()>`
  Prints the short help message (`-h`) to [`io::stdout()`].

- `fn print_long_help(self: &mut Self) -> io::Result<()>`
  Prints the long help message (`--help`) to [`io::stdout()`].

- `fn render_help(self: &mut Self) -> StyledStr`
  Render the short help message (`-h`) to a [`StyledStr`]

- `fn render_long_help(self: &mut Self) -> StyledStr`
  Render the long help message (`--help`) to a [`StyledStr`].

- `fn render_version(self: &Self) -> String`
  Version message rendered as if the user ran `-V`.

- `fn render_long_version(self: &Self) -> String`
  Version message rendered as if the user ran `--version`.

- `fn render_usage(self: &mut Self) -> StyledStr`
  Usage statement

- `fn get_display_name(self: &Self) -> Option<&str>`
  Get the name of the binary.

- `fn get_bin_name(self: &Self) -> Option<&str>`
  Get the name of the binary.

- `fn set_bin_name(self: &mut Self, name: impl Into<String>)`
  Set binary name. Uses `&mut self` instead of `self`.

- `fn get_name(self: &Self) -> &str`
  Get the name of the cmd.

- `fn get_name_and_visible_aliases(self: &Self) -> Vec<&str>`
  Get all known names of the cmd (i.e. primary name and visible aliases).

- `fn get_version(self: &Self) -> Option<&str>`
  Get the version of the cmd.

- `fn get_long_version(self: &Self) -> Option<&str>`
  Get the long version of the cmd.

- `fn get_display_order(self: &Self) -> usize`
  Get the placement within help

- `fn get_author(self: &Self) -> Option<&str>`
  Get the authors of the cmd.

- `fn get_short_flag(self: &Self) -> Option<char>`
  Get the short flag of the subcommand.

- `fn get_long_flag(self: &Self) -> Option<&str>`
  Get the long flag of the subcommand.

- `fn get_about(self: &Self) -> Option<&StyledStr>`
  Get the help message specified via [`Command::about`].

- `fn get_long_about(self: &Self) -> Option<&StyledStr>`
  Get the help message specified via [`Command::long_about`].

- `fn is_flatten_help_set(self: &Self) -> bool`
  Get the custom section heading specified via [`Command::flatten_help`].

- `fn get_next_help_heading(self: &Self) -> Option<&str>`
  Get the custom section heading specified via [`Command::next_help_heading`].

- `fn get_visible_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *visible* aliases for this subcommand.

- `fn get_visible_short_flag_aliases(self: &Self) -> impl Iterator<Item = char> + '_`
  Iterate through the *visible* short aliases for this subcommand.

- `fn get_visible_long_flag_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *visible* long aliases for this subcommand.

- `fn get_all_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the set of *all* the aliases for this subcommand, both visible and hidden.

- `fn get_all_short_flag_aliases(self: &Self) -> impl Iterator<Item = char> + '_`
  Iterate through the set of *all* the short aliases for this subcommand, both visible and hidden.

- `fn get_all_long_flag_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the set of *all* the long aliases for this subcommand, both visible and hidden.

- `fn get_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Iterate through the *hidden* aliases for this subcommand.

- `fn get_color(self: &Self) -> ColorChoice`
  Should we color the output?

- `fn get_styles(self: &Self) -> &Styles`
  Return the current `Styles` for the `Command`

- `fn get_subcommands(self: &Self) -> impl Iterator<Item = &Command>`
  Iterate through the set of subcommands, getting a reference to each.

- `fn get_subcommands_mut(self: &mut Self) -> impl Iterator<Item = &mut Command>`
  Iterate through the set of subcommands, getting a mutable reference to each.

- `fn has_subcommands(self: &Self) -> bool`
  Returns `true` if this `Command` has subcommands.

- `fn get_subcommand_help_heading(self: &Self) -> Option<&str>`
  Returns the help heading for listing subcommands.

- `fn get_subcommand_value_name(self: &Self) -> Option<&str>`
  Returns the subcommand value name.

- `fn get_before_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_before_long_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_after_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn get_after_long_help(self: &Self) -> Option<&StyledStr>`
  Returns the help heading for listing subcommands.

- `fn find_subcommand(self: &Self, name: impl AsRef<std::ffi::OsStr>) -> Option<&Command>`
  Find subcommand such that its name or one of aliases equals `name`.

- `fn find_subcommand_mut(self: &mut Self, name: impl AsRef<std::ffi::OsStr>) -> Option<&mut Command>`
  Find subcommand such that its name or one of aliases equals `name`, returning

- `fn get_groups(self: &Self) -> impl Iterator<Item = &ArgGroup>`
  Iterate through the set of groups.

- `fn get_arguments(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the set of arguments.

- `fn get_positionals(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the *positionals* arguments.

- `fn get_opts(self: &Self) -> impl Iterator<Item = &Arg>`
  Iterate through the *options*.

- `fn get_arg_conflicts_with(self: &Self, arg: &Arg) -> Vec<&Arg>`
  Get a list of all arguments the given argument conflicts with.

- `fn is_no_binary_name_set(self: &Self) -> bool`
  Report whether [`Command::no_binary_name`] is set

- `fn is_dont_delimit_trailing_values_set(self: &Self) -> bool`
  Report whether [`Command::dont_delimit_trailing_values`] is set

- `fn is_disable_version_flag_set(self: &Self) -> bool`
  Report whether [`Command::disable_version_flag`] is set

- `fn is_propagate_version_set(self: &Self) -> bool`
  Report whether [`Command::propagate_version`] is set

- `fn is_next_line_help_set(self: &Self) -> bool`
  Report whether [`Command::next_line_help`] is set

- `fn is_disable_help_flag_set(self: &Self) -> bool`
  Report whether [`Command::disable_help_flag`] is set

- `fn is_disable_help_subcommand_set(self: &Self) -> bool`
  Report whether [`Command::disable_help_subcommand`] is set

- `fn is_disable_colored_help_set(self: &Self) -> bool`
  Report whether [`Command::disable_colored_help`] is set

- `fn is_arg_required_else_help_set(self: &Self) -> bool`
  Report whether [`Command::arg_required_else_help`] is set

- `fn is_allow_missing_positional_set(self: &Self) -> bool`
  Report whether [`Command::allow_missing_positional`] is set

- `fn is_hide_set(self: &Self) -> bool`
  Report whether [`Command::hide`] is set

- `fn is_subcommand_required_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_required`] is set

- `fn is_allow_external_subcommands_set(self: &Self) -> bool`
  Report whether [`Command::allow_external_subcommands`] is set

- `fn get_external_subcommand_value_parser(self: &Self) -> Option<&super::ValueParser>`
  Configured parser for values passed to an external subcommand

- `fn is_args_conflicts_with_subcommands_set(self: &Self) -> bool`
  Report whether [`Command::args_conflicts_with_subcommands`] is set

- `fn is_subcommand_precedence_over_arg_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_precedence_over_arg`] is set

- `fn is_subcommand_negates_reqs_set(self: &Self) -> bool`
  Report whether [`Command::subcommand_negates_reqs`] is set

- `fn is_multicall_set(self: &Self) -> bool`
  Report whether [`Command::multicall`] is set

- `fn build(self: &mut Self)`
  Prepare for introspecting on all included [`Command`]s

- `fn no_binary_name(self: Self, yes: bool) -> Self`
  Specifies that the parser should not assume the first argument passed is the binary name.

- `fn ignore_errors(self: Self, yes: bool) -> Self`
  Try not to fail on parse errors, like missing option values.

- `fn args_override_self(self: Self, yes: bool) -> Self`
  Replace prior occurrences of arguments rather than error

- `fn dont_delimit_trailing_values(self: Self, yes: bool) -> Self`
  Disables the automatic [delimiting of values][Arg::value_delimiter] after `--` or when [`Arg::trailing_var_arg`]

- `fn color(self: Self, color: ColorChoice) -> Self`
  Sets when to color output.

- `fn styles(self: Self, styles: Styles) -> Self`
  Sets the [`Styles`] for terminal output

- `fn term_width(self: Self, width: usize) -> Self`
  Sets the terminal width at which to wrap help messages.

- `fn max_term_width(self: Self, width: usize) -> Self`
  Limit the line length for wrapping help when using the current terminal's width.

- `fn disable_version_flag(self: Self, yes: bool) -> Self`
  Disables `-V` and `--version` flag.

- `fn propagate_version(self: Self, yes: bool) -> Self`
  Specifies to use the version of the current command for all [`subcommands`].

- `fn next_line_help(self: Self, yes: bool) -> Self`
  Places the help string for all arguments and subcommands on the line after them.

- `fn disable_help_flag(self: Self, yes: bool) -> Self`
  Disables `-h` and `--help` flag.

- `fn disable_help_subcommand(self: Self, yes: bool) -> Self`
  Disables the `help` [`subcommand`].

- `fn disable_colored_help(self: Self, yes: bool) -> Self`
  Disables colorized help messages.

- `fn help_expected(self: Self, yes: bool) -> Self`
   Panic if help descriptions are omitted.

- `fn hide_possible_values(self: Self, yes: bool) -> Self`
  Tells `clap` *not* to print possible values when displaying help information.

- `fn infer_long_args(self: Self, yes: bool) -> Self`
  Allow partial matches of long arguments or their [aliases].

- `fn infer_subcommands(self: Self, yes: bool) -> Self`
  Allow partial matches of [subcommand] names and their [aliases].

- `fn short_flag(self: Self, short: impl IntoResettable<char>) -> Self`
  Sets the short version of the subcommand flag without the preceding `-`.

- `fn long_flag(self: Self, long: impl Into<Str>) -> Self`
  Sets the long version of the subcommand flag without the preceding `--`.

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Sets a hidden alias to this subcommand.

- `fn short_flag_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as  "hidden" short flag subcommand

- `fn long_flag_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a "hidden" long flag subcommand.

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Sets multiple hidden aliases to this subcommand.

- `fn short_flag_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as "hidden" short flag subcommands.

- `fn long_flag_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as "hidden" long flag subcommands.

- `fn visible_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Sets a visible alias to this subcommand.

- `fn visible_short_flag_alias(self: Self, name: impl IntoResettable<char>) -> Self`
  Add an alias, which functions as  "visible" short flag subcommand

- `fn visible_long_flag_alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Add an alias, which functions as a "visible" long flag subcommand.

- `fn visible_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Sets multiple visible aliases to this subcommand.

- `fn visible_short_flag_aliases(self: Self, names: impl IntoIterator<Item = char>) -> Self`
  Add aliases, which function as *visible* short flag subcommands.

- `fn visible_long_flag_aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Add aliases, which function as *visible* long flag subcommands.

- `fn display_order(self: Self, ord: impl IntoResettable<usize>) -> Self`
  Set the placement of this subcommand within the help.

- `fn hide(self: Self, yes: bool) -> Self`
  Specifies that this [`subcommand`] should be hidden from help messages

- `fn subcommand_required(self: Self, yes: bool) -> Self`
  If no [`subcommand`] is present at runtime, error and exit gracefully.

- `fn allow_external_subcommands(self: Self, yes: bool) -> Self`
  Assume unexpected positional arguments are a [`subcommand`].

- `fn external_subcommand_value_parser(self: Self, parser: impl IntoResettable<super::ValueParser>) -> Self`
  Specifies how to parse external subcommand arguments.

- `fn args_conflicts_with_subcommands(self: Self, yes: bool) -> Self`
  Specifies that use of an argument prevents the use of [`subcommands`].

- `fn subcommand_precedence_over_arg(self: Self, yes: bool) -> Self`
  Prevent subcommands from being consumed as an arguments value.

- `fn subcommand_negates_reqs(self: Self, yes: bool) -> Self`
  Allows [`subcommands`] to override all requirements of the parent command.

- `fn multicall(self: Self, yes: bool) -> Self`
  Multiple-personality program dispatched on the binary name (`argv[0]`)

- `fn subcommand_value_name(self: Self, value_name: impl IntoResettable<Str>) -> Self`
  Sets the value name used for subcommands when printing usage and help.

- `fn subcommand_help_heading(self: Self, heading: impl IntoResettable<Str>) -> Self`
  Sets the help heading used for subcommands when printing usage and help.

#### Trait Implementations

##### `impl From`

- `fn from(cmd: &Command) -> Self`

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

- `fn clone(self: &Self) -> Command`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Index`

- `type Output = Arg`

- `fn index(self: &Self, key: &Id) -> &<Self as >::Output`

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

##### `impl Default`

- `fn default() -> Self`

### `OsStr`

```rust
struct OsStr {
    // [REDACTED: Private Fields]
}
```

A UTF-8-encoded fixed string

<div class="warning">

**NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`
feature

</div>

#### Implementations

- `fn as_os_str(self: &Self) -> &std::ffi::OsStr`
  Get the raw string as an `std::ffi::OsStr`

- `fn to_os_string(self: &Self) -> std::ffi::OsString`
  Get the raw string as an `OsString`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(id: &Str) -> Self`

##### `impl From`

- `fn from(id: Str) -> Self`

##### `impl From`

- `fn from(name: &'static str) -> Self`

##### `impl From`

- `fn from(id: &OsStr) -> Self`

##### `impl From`

- `fn from(name: &'static std::ffi::OsStr) -> Self`

##### `impl From`

- `fn from(name: &&'static str) -> Self`

##### `impl From`

- `fn from(name: &&'static std::ffi::OsStr) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<OsStr>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &std::path::Path`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &std::ffi::OsStr`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl Borrow`

- `fn borrow(self: &Self) -> &std::ffi::OsStr`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OsStr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &OsStr) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &std::ffi::OsString) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &OsStr) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&str) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &&std::ffi::OsStr) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &String) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &OsStr) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

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

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default`

- `fn default() -> OsStr`

##### `impl Deref`

- `type Target = OsStr`

- `fn deref(self: &Self) -> &std::ffi::OsStr`

### `PossibleValue`

```rust
struct PossibleValue {
    // [REDACTED: Private Fields]
}
```

A possible value of an argument.

This is used for specifying [possible values] of [Args].

See also `PossibleValuesParser`

<div class="warning">

**NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required
to [hide](#hide) single values from help messages and shell completions or to attach [help](#help) to
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

[Args]: crate::Arg
[possible values]: crate::builder::ValueParser::possible_values
[hide](#hide): PossibleValue::hide()
[help](#help): PossibleValue::help()

#### Implementations

- `fn new(name: impl Into<Str>) -> Self`
  Create a [`PossibleValue`] with its name.

- `fn help(self: Self, help: impl IntoResettable<StyledStr>) -> Self`
  Sets the help description of the value.

- `fn hide(self: Self, yes: bool) -> Self`
  Hides this value from help and shell completions.

- `fn alias(self: Self, name: impl IntoResettable<Str>) -> Self`
  Sets a *hidden* alias for this argument value.

- `fn aliases(self: Self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self`
  Sets multiple *hidden* aliases for this argument value.

- `fn get_name(self: &Self) -> &str`
  Get the name of the argument value

- `fn get_help(self: &Self) -> Option<&StyledStr>`
  Get the help specified for this argument, if any

- `fn is_hide_set(self: &Self) -> bool`
  Report if [`PossibleValue::hide`] is set

- `fn get_name_and_aliases(self: &Self) -> impl Iterator<Item = &str> + '_`
  Returns all valid values of the argument value.

- `fn matches(self: &Self, value: &str, ignore_case: bool) -> bool`
  Tests if the value is valid for this argument value

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<S: Into<crate::builder::Str>>`

- `fn from(s: S) -> Self`

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

- `fn clone(self: &Self) -> PossibleValue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PossibleValue) -> bool`

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

- `fn default() -> PossibleValue`

### `ValueRange`

```rust
struct ValueRange {
    // [REDACTED: Private Fields]
}
```

Values per occurrence for an argument

#### Implementations

- `const EMPTY: Self`

- `const SINGLE: Self`

- `fn new(range: impl Into<Self>) -> Self`
  Create a range

- `fn min_values(self: &Self) -> usize`
  Fewest number of values the argument accepts

- `fn max_values(self: &Self) -> usize`
  Most number of values the argument accepts

- `fn takes_values(self: &Self) -> bool`
  Report whether the argument takes any values (ie is a flag)

#### Trait Implementations

##### `impl From`

- `fn from(range: std::ops::RangeInclusive<usize>) -> Self`

##### `impl From`

- `fn from(range: std::ops::Range<usize>) -> Self`

##### `impl From`

- `fn from(_: std::ops::RangeFull) -> Self`

##### `impl From`

- `fn from(fixed: usize) -> Self`

##### `impl From`

- `fn from(range: std::ops::RangeFrom<usize>) -> Self`

##### `impl From`

- `fn from(range: std::ops::RangeTo<usize>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(range: std::ops::RangeToInclusive<usize>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueRange>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ValueRange`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ValueRange) -> bool`

##### `impl RangeBounds`

- `fn start_bound(self: &Self) -> std::ops::Bound<&usize>`

- `fn end_bound(self: &Self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq`

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

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `StyledStr`

```rust
struct StyledStr();
```

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

- `const fn new() -> Self`
  Create an empty buffer

- `fn ansi(self: &Self) -> impl std::fmt::Display + '_`
  Display using [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) styling

- `fn push_str(self: &mut Self, msg: &str)`
  Appends a given string slice onto the end of this `StyledStr`.

#### Trait Implementations

##### `impl From`

- `fn from(name: &String) -> Self`

##### `impl From`

- `fn from(name: &'static str) -> Self`

##### `impl From`

- `fn from(name: String) -> Self`

##### `impl From`

- `fn from(cow: Cow<'static, str>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(name: &&'static str) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<StyledStr>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StyledStr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &StyledStr) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StyledStr) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StyledStr) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

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

##### `impl Write`

- `fn write_str(self: &mut Self, s: &str) -> Result<(), std::fmt::Error>`

- `fn write_char(self: &mut Self, c: char) -> Result<(), std::fmt::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> StyledStr`

### `Styles`

```rust
struct Styles {
    // [REDACTED: Private Fields]
}
```

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

- `const fn plain() -> Self`
  No terminal styling

- `const fn styled() -> Self`
  Default terminal styling

- `const fn header(self: Self, style: Style) -> Self`
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- `const fn error(self: Self, style: Style) -> Self`
  Error heading

- `const fn usage(self: Self, style: Style) -> Self`
  Usage heading

- `const fn literal(self: Self, style: Style) -> Self`
  Literal command-line syntax, e.g. `--help`

- `const fn placeholder(self: Self, style: Style) -> Self`
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- `const fn valid(self: Self, style: Style) -> Self`
  Highlight suggested usage

- `const fn invalid(self: Self, style: Style) -> Self`
  Highlight invalid usage

- `const fn context(self: Self, style: Style) -> Self`
  Highlight all specified contexts, e.g. `[default: false]`

- `const fn context_value(self: Self, style: Style) -> Self`
  Highlight values within all of the context, e.g. the `false` in `[default: false]`

- `const fn get_header(self: &Self) -> &Style`
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- `const fn get_error(self: &Self) -> &Style`
  Error heading

- `const fn get_usage(self: &Self) -> &Style`
  Usage heading

- `const fn get_literal(self: &Self) -> &Style`
  Literal command-line syntax, e.g. `--help`

- `const fn get_placeholder(self: &Self) -> &Style`
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- `const fn get_valid(self: &Self) -> &Style`
  Highlight suggested usage

- `const fn get_invalid(self: &Self) -> &Style`
  Highlight invalid usage

- `const fn get_context(self: &Self) -> &Style`
  Highlight all specified contexts, e.g. `[default: false]`

- `const fn get_context_value(self: &Self) -> &Style`
  Highlight values within all of the context, e.g. the `false` in `[default: false]`

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

- `fn clone(self: &Self) -> Styles`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

- `fn default() -> Self`

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](../index.md)s

#### Implementations

- `fn new() -> Self`
  Implementation for [`ValueParser::bool`]

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> BoolValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

Parse bool-like string values

See also:
- `ValueParser::bool` for different human readable bool representations
- [`FalseyValueParser`](../index.md) for assuming non-false is true

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

- `fn new() -> Self`
  Parse bool-like string values

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> BoolishValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>();
```

Parse an `ValueEnum` value.

See also:
- [`PossibleValuesParser`](../index.md)

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

- `fn new() -> Self`
  Parse an [`ValueEnum`][crate::ValueEnum]

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<E: $crate::clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static>`

- `fn clone(self: &Self) -> EnumValueParser<E>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl TypedValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

- `type Value = E`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug<E: $crate::fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

- `fn default() -> Self`

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

Parse false-like string values, everything else is `true`

See also:
- `ValueParser::bool` for assuming non-false is true
- [`BoolishValueParser`](../index.md) for different human readable bool representations

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

- `fn new() -> Self`
  Parse false-like string values, everything else is `true`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> FalseyValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `MapValueParser<P, F>`

```rust
struct MapValueParser<P, F> {
    // [REDACTED: Private Fields]
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::map`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<P: $crate::clone::Clone, F: $crate::clone::Clone>`

- `fn clone(self: &Self) -> MapValueParser<P, F>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl TypedValueParser<P, F, T>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug<P: $crate::fmt::Debug, F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `NonEmptyStringValueParser`

```rust
struct NonEmptyStringValueParser {
}
```

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

- `fn new() -> Self`
  Parse non-empty string values

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> NonEmptyStringValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](../index.md)s

#### Implementations

- `fn new() -> Self`
  Implementation for [`ValueParser::os_string`]

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OsStringValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = OsString`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse(self: &Self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](../index.md)s

#### Implementations

- `fn new() -> Self`
  Implementation for [`ValueParser::path_buf`]

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PathBufValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = PathBuf`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `PossibleValuesParser`

```rust
struct PossibleValuesParser();
```

Verify the value is from an enumerated set of `PossibleValue`.

See also:
- [`EnumValueParser`](../index.md) for directly supporting `ValueEnum` types
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

- `fn new(values: impl Into<PossibleValuesParser>) -> Self`
  Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<I, T>`

- `fn from(values: I) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PossibleValuesParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl TypedValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync>`

```rust
struct RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync> {
    // [REDACTED: Private Fields]
}
```

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

- `fn new() -> Self`
  Select full range of `i64`

- `fn range<B: RangeBounds<i64>>(self: Self, range: B) -> Self`
  Narrow the supported range

#### Trait Implementations

##### `impl From<T: TryFrom<i64> + Clone + Send + Sync, B: RangeBounds<i64>>`

- `fn from(range: B) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone + TryFrom<i64> + Clone + Send + Sync>`

- `fn clone(self: &Self) -> RangedI64ValueParser<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T: $crate::marker::Copy + TryFrom<i64> + Clone + Send + Sync>`

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

##### `impl TypedValueParser<T: TryFrom<i64> + Clone + Send + Sync + 'static>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug<T: $crate::fmt::Debug + TryFrom<i64> + Clone + Send + Sync>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<T: TryFrom<i64> + Clone + Send + Sync>`

- `fn default() -> Self`

### `RangedU64ValueParser<T: TryFrom<u64>>`

```rust
struct RangedU64ValueParser<T: TryFrom<u64>> {
    // [REDACTED: Private Fields]
}
```

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

- `fn new() -> Self`
  Select full range of `u64`

- `fn range<B: RangeBounds<u64>>(self: Self, range: B) -> Self`
  Narrow the supported range

#### Trait Implementations

##### `impl From<T: TryFrom<u64>, B: RangeBounds<u64>>`

- `fn from(range: B) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone + TryFrom<u64>>`

- `fn clone(self: &Self) -> RangedU64ValueParser<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T: $crate::marker::Copy + TryFrom<u64>>`

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

##### `impl TypedValueParser<T: TryFrom<u64> + Clone + Send + Sync + 'static>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug<T: $crate::fmt::Debug + TryFrom<u64>>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<T: TryFrom<u64>>`

- `fn default() -> Self`

### `StringValueParser`

```rust
struct StringValueParser {
}
```

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](../index.md)s

#### Implementations

- `fn new() -> Self`
  Implementation for [`ValueParser::string`]

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StringValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

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

##### `impl TypedValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

### `TryMapValueParser<P, F>`

```rust
struct TryMapValueParser<P, F> {
    // [REDACTED: Private Fields]
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::try_map`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<P: $crate::clone::Clone, F: $crate::clone::Clone>`

- `fn clone(self: &Self) -> TryMapValueParser<P, F>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl TypedValueParser<P, F, T, E>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

##### `impl Debug<P: $crate::fmt::Debug, F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `UnknownArgumentValueParser`

```rust
struct UnknownArgumentValueParser {
    // [REDACTED: Private Fields]
}
```

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

- `fn suggest_arg(arg: impl Into<Str>) -> Self`
  Suggest an alternative argument

- `fn suggest(text: impl Into<StyledStr>) -> Self`
  Provide a general suggestion

- `fn and_suggest(self: Self, text: impl Into<StyledStr>) -> Self`
  Extend the suggestions

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> UnknownArgumentValueParser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl TypedValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

- `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ValueParser`

```rust
struct ValueParser();
```

Parse/validate argument values

Specified with `Arg::value_parser`.

`ValueParser` defines how to convert a raw argument value into a validated and typed value for
use within an application.

See
- `value_parser!` for automatically selecting an implementation for a given type
- `ValueParser::new` for additional [`TypedValueParser`](../index.md) that can be used

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

- `fn type_id(self: &Self) -> AnyValueId`
  Describes the content of `AnyValue`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`
  Reflect on enumerated value properties

- `fn new<P>(other: P) -> Self`
  Custom parser for argument values

- `const fn bool() -> Self`
  [`bool`] parser for argument values

- `const fn string() -> Self`
  [`String`] parser for argument values

- `const fn os_string() -> Self`
  [`OsString`][std::ffi::OsString] parser for argument values

- `const fn path_buf() -> Self`
  [`PathBuf`][std::path::PathBuf] parser for argument values

#### Trait Implementations

##### `impl From`

- `fn from(value: std::ops::Range<i64>) -> Self`

##### `impl From`

- `fn from(value: std::ops::RangeInclusive<i64>) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: std::ops::RangeToInclusive<i64>) -> Self`

##### `impl From`

- `fn from(value: std::ops::RangeFrom<i64>) -> Self`

##### `impl From`

- `fn from(value: std::ops::RangeFull) -> Self`

##### `impl From<P>`

- `fn from(values: Vec<P>) -> Self`

##### `impl From`

- `fn from(value: std::ops::RangeTo<i64>) -> Self`

##### `impl From<P, const C: usize>`

- `fn from(values: [P; C]) -> Self`

##### `impl From<P>`

- `fn from(p: P) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<I>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

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

- `fn takes_values(self: &Self) -> bool`
  Returns whether this action accepts values on the command-line

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable`

- `fn into_resettable(self: Self) -> Resettable<ArgAction>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ArgAction`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `ArgPredicate`

```rust
enum ArgPredicate {
    IsPresent,
    Equals(crate::builder::OsStr),
}
```

Operations to perform on argument values

These do not apply to `ValueSource::DefaultValue`

#### Variants

- **`IsPresent`**

  Is the argument present?

- **`Equals`**

  Does the argument match the specified value?

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<S: Into<crate::builder::OsStr>>`

- `fn from(other: S) -> Self`

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

- `fn clone(self: &Self) -> ArgPredicate`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgPredicate) -> bool`

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

### `Resettable<T>`

```rust
enum Resettable<T> {
    Value(T),
    Reset,
}
```

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

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: never) -> T`

##### `impl From<T>`

- `fn from(other: T) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<T>`

- `fn from(other: Option<T>) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable<T>`

- `fn into_resettable(self: Self) -> Resettable<T>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Resettable<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T: $crate::marker::Copy>`

##### `impl Eq<T: $crate::cmp::Eq>`

##### `impl Hash<T: $crate::hash::Hash>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<T: $crate::cmp::Ord>`

- `fn cmp(self: &Self, other: &Resettable<T>) -> $crate::cmp::Ordering`

##### `impl PartialEq<T: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &Resettable<T>) -> bool`

##### `impl PartialOrd<T: $crate::cmp::PartialOrd>`

- `fn partial_cmp(self: &Self, other: &Resettable<T>) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq<T>`

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
  [`.num_args(1..)`](#num-args1) and Command must use `Command::trailing_var_arg(true)`. The result is that the
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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromStr`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoResettable`

- `fn into_resettable(self: Self) -> Resettable<ValueHint>`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ValueHint`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ValueHint) -> bool`

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

- `fn default() -> ValueHint`

## Traits

