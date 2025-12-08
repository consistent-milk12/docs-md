*[clap_builder](../index.md) / [derive](index.md)*

---

# Module `derive`

This module contains traits that are usable with the `#[derive(...)]`
macros in `clap_derive`.

## Traits

### `Parser`

```rust
trait Parser: FromArgMatches + CommandFactory + Sized { ... }
```

Parse command-line arguments into `Self`.

The primary one-stop-shop trait used to create an instance of a `clap`
[`Command`](../builder/command/index.md), conduct the parsing, and turn the resulting [`ArgMatches`](../parser/matches/arg_matches/index.md) back
into concrete instance of the user struct.

This trait is primarily a convenience on top of [`FromArgMatches`](#fromargmatches) +
[`CommandFactory`](#commandfactory) which uses those two underlying traits to build the two
fundamental functions `parse` which uses the `std::env::args_os` iterator,
and `parse_from` which allows the consumer to supply the iterator (along
with fallible options for each).

See also [`Subcommand`](#subcommand) and [`Args`](#args).

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn parse() -> Self`

  Parse from `std::env::args_os()`, `exit` on error.

- `fn try_parse() -> Result<Self, Error>`

  Parse from `std::env::args_os()`, return Err on error.

- `fn parse_from<I, T>(itr: I) -> Self`

  Parse from iterator, `exit` on error.

- `fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>`

  Parse from iterator, return Err on error.

- `fn update_from<I, T>(self: &mut Self, itr: I)`

  Update from iterator, `exit` on error.

- `fn try_update_from<I, T>(self: &mut Self, itr: I) -> Result<(), Error>`

  Update from iterator, return Err on error.

### `CommandFactory`

```rust
trait CommandFactory: Sized { ... }
```

Create a [`Command`](../builder/command/index.md) relevant for a user-defined container.

Derived as part of [`Parser`](#parser).

#### Required Methods

- `fn command() -> Command`

  Build a [`Command`](../builder/command/index.md) that can instantiate `Self`.

- `fn command_for_update() -> Command`

  Build a [`Command`](../builder/command/index.md) that can update `self`.

### `FromArgMatches`

```rust
trait FromArgMatches: Sized { ... }
```

Converts an instance of [`ArgMatches`](../parser/matches/arg_matches/index.md) to a user-defined container.

Derived as part of [`Parser`](#parser), [`Args`](#args), and [`Subcommand`](#subcommand).

#### Required Methods

- `fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error>`

  Instantiate `Self` from [`ArgMatches`](../parser/matches/arg_matches/index.md), parsing the arguments as needed.

- `fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error>`

  Instantiate `Self` from [`ArgMatches`](../parser/matches/arg_matches/index.md), parsing the arguments as needed.

- `fn update_from_arg_matches(self: &mut Self, matches: &ArgMatches) -> Result<(), Error>`

  Assign values from `ArgMatches` to `self`.

- `fn update_from_arg_matches_mut(self: &mut Self, matches: &mut ArgMatches) -> Result<(), Error>`

  Assign values from `ArgMatches` to `self`.

### `Args`

```rust
trait Args: FromArgMatches + Sized { ... }
```

Parse a set of arguments into a user-defined container.

Implementing this trait lets a parent container delegate argument parsing behavior to `Self`.
with:
- `#[command(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl
  `Args`.
- `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn group_id() -> Option<crate::Id>`

  Report the `ArgGroup::id` for this set of arguments

- `fn augment_args(cmd: Command) -> Command`

  Append to [`Command`](../builder/command/index.md) so it can instantiate `Self` via

- `fn augment_args_for_update(cmd: Command) -> Command`

  Append to [`Command`](../builder/command/index.md) so it can instantiate `self` via

### `Subcommand`

```rust
trait Subcommand: FromArgMatches + Sized { ... }
```

Parse a sub-command into a user-defined enum.

Implementing this trait lets a parent container delegate subcommand behavior to `Self`.
with:
- `#[command(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum
  variants that impl `Subcommand`.
- `#[command(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl
  `Subcommand`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn augment_subcommands(cmd: Command) -> Command`

  Append to [`Command`](../builder/command/index.md) so it can instantiate `Self` via

- `fn augment_subcommands_for_update(cmd: Command) -> Command`

  Append to [`Command`](../builder/command/index.md) so it can instantiate `self` via

- `fn has_subcommand(name: &str) -> bool`

  Test whether `Self` can parse a specific subcommand

### `ValueEnum`

```rust
trait ValueEnum: Sized + Clone { ... }
```

Parse arguments into enums.

When deriving [`Parser`](#parser), a field whose type implements `ValueEnum` can have the attribute
`#[arg(value_enum)]` which will
- Call `EnumValueParser`
- Allowing using the `#[arg(default_value_t)]` attribute without implementing `Display`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

#### Required Methods

- `fn value_variants<'a>() -> &'a [Self]`

  All possible argument values, in display order.

- `fn from_str(input: &str, ignore_case: bool) -> Result<Self, String>`

  Parse an argument into `Self`.

- `fn to_possible_value(self: &Self) -> Option<PossibleValue>`

  The canonical argument value.

## Functions

### `format_error`

```rust
fn format_error<I: CommandFactory>(err: crate::Error) -> crate::Error
```

