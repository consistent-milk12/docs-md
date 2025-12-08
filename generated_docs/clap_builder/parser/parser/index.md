*[clap_builder](../../index.md) / [parser](../index.md) / [parser](index.md)*

---

# Module `parser`

## Structs

### `Parser<'cmd>`

```rust
struct Parser<'cmd> {
    cmd: &'cmd mut crate::builder::Command,
    cur_idx: std::cell::Cell<usize>,
    flag_subcmd_at: Option<usize>,
    flag_subcmd_skip: usize,
}
```

#### Fields

- **`flag_subcmd_at`**: `Option<usize>`

  Index of the previous flag subcommand in a group of flags.

- **`flag_subcmd_skip`**: `usize`

  Counter indicating the number of items to skip
  when revisiting the group of flags which includes the flag subcommand.

#### Implementations

- `fn get_matches_with(self: &mut Self, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn parse(self: &mut Self, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn match_arg_error(self: &Self, arg_os: &clap_lex::ParsedArg<'_>, valid_arg_found: bool, trailing_values: bool, matcher: &ArgMatcher) -> ClapError` — [`ArgMatcher`](../arg_matcher/index.md), [`Error`](../../error/index.md)

- `fn possible_subcommand(self: &Self, arg: Result<&str, &OsStr>, valid_arg_found: bool) -> Option<&str>`

- `fn possible_long_flag_subcommand(self: &Self, arg: &str) -> Option<&str>`

- `fn parse_help_subcommand(self: &Self, cmds: impl Iterator<Item = &'cmd OsStr>) -> ClapResult<std::convert::Infallible>` — [`Result`](../../error/index.md)

- `fn is_new_arg(self: &Self, next: &clap_lex::ParsedArg<'_>, current_positional: &Arg) -> bool` — [`Arg`](../../index.md)

- `fn parse_subcommand(self: &mut Self, sc_name: &str, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor, keep_state: bool) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn parse_long_arg(self: &mut Self, matcher: &mut ArgMatcher, long_arg: Result<&str, &OsStr>, long_value: Option<&OsStr>, parse_state: &ParseState, pos_counter: usize, valid_arg_found: &mut bool) -> ClapResult<ParseResult>` — [`ArgMatcher`](../arg_matcher/index.md), [`ParseState`](#parsestate), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- `fn parse_short_arg(self: &mut Self, matcher: &mut ArgMatcher, short_arg: clap_lex::ShortFlags<'_>, parse_state: &ParseState, pos_counter: usize, valid_arg_found: &mut bool) -> ClapResult<ParseResult>` — [`ArgMatcher`](../arg_matcher/index.md), [`ParseState`](#parsestate), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- `fn parse_opt_value(self: &Self, ident: Identifier, attached_value: Option<&OsStr>, arg: &Arg, matcher: &mut ArgMatcher, has_eq: bool) -> ClapResult<ParseResult>` — [`Identifier`](#identifier), [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- `fn check_terminator(self: &Self, arg: &Arg, val: &OsStr) -> Option<ParseResult>` — [`Arg`](../../index.md), [`ParseResult`](#parseresult)

- `fn push_arg_values(self: &Self, arg: &Arg, raw_vals: Vec<OsString>, source: ValueSource, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`Arg`](../../index.md), [`ValueSource`](../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn resolve_pending(self: &Self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn react(self: &Self, ident: Option<Identifier>, source: ValueSource, arg: &Arg, raw_vals: Vec<OsString>, trailing_idx: Option<usize>, matcher: &mut ArgMatcher) -> ClapResult<ParseResult>` — [`Identifier`](#identifier), [`ValueSource`](../index.md), [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- `fn verify_num_args(self: &Self, arg: &Arg, raw_vals: &[OsString]) -> ClapResult<()>` — [`Arg`](../../index.md), [`Result`](../../error/index.md)

- `fn remove_overrides(self: &Self, arg: &Arg, matcher: &mut ArgMatcher)` — [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md)

- `fn add_defaults(self: &Self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn add_default_value(self: &Self, arg: &Arg, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn start_custom_arg(self: &Self, matcher: &mut ArgMatcher, arg: &Arg, source: ValueSource)` — [`ArgMatcher`](../arg_matcher/index.md), [`Arg`](../../index.md), [`ValueSource`](../index.md)

### `PendingArg`

```rust
struct PendingArg {
    id: crate::util::Id,
    ident: Option<Identifier>,
    raw_vals: Vec<std::ffi::OsString>,
    trailing_idx: Option<usize>,
}
```

#### Trait Implementations

##### `impl Clone for PendingArg`

- `fn clone(self: &Self) -> PendingArg` — [`PendingArg`](#pendingarg)

##### `impl Debug for PendingArg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for PendingArg`

##### `impl PartialEq for PendingArg`

- `fn eq(self: &Self, other: &PendingArg) -> bool` — [`PendingArg`](#pendingarg)

##### `impl StructuralPartialEq for PendingArg`

## Enums

### `ParseState`

```rust
enum ParseState {
    ValuesDone,
    Opt(crate::util::Id),
    Pos(crate::util::Id),
}
```

#### Trait Implementations

##### `impl Debug for ParseState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ParseState`

##### `impl PartialEq for ParseState`

- `fn eq(self: &Self, other: &ParseState) -> bool` — [`ParseState`](#parsestate)

##### `impl StructuralPartialEq for ParseState`

### `ParseResult`

```rust
enum ParseResult {
    FlagSubCommand(String),
    Opt(crate::util::Id),
    ValuesDone,
    AttachedValueNotConsumed,
    UnneededAttachedValue {
        rest: String,
        used: Vec<crate::util::Id>,
        arg: String,
    },
    MaybeHyphenValue,
    EqualsNotProvided {
        arg: String,
    },
    NoMatchingArg {
        arg: String,
    },
    NoArg,
}
```

Recoverable Parsing results.

#### Variants

- **`AttachedValueNotConsumed`**

  Value attached to the short flag is not consumed(e.g. 'u' for `-cu` is
  not consumed).

- **`UnneededAttachedValue`**

  This long flag doesn't need a value but is provided one.

- **`MaybeHyphenValue`**

  This flag might be an hyphen Value.

- **`EqualsNotProvided`**

  Equals required but not provided.

- **`NoMatchingArg`**

  Failed to match a Arg.

- **`NoArg`**

  No argument found e.g. parser is given `-` when parsing a flag.

#### Trait Implementations

##### `impl Clone for ParseResult`

- `fn clone(self: &Self) -> ParseResult` — [`ParseResult`](#parseresult)

##### `impl Debug for ParseResult`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PartialEq for ParseResult`

- `fn eq(self: &Self, other: &ParseResult) -> bool` — [`ParseResult`](#parseresult)

##### `impl StructuralPartialEq for ParseResult`

### `Identifier`

```rust
enum Identifier {
    Short,
    Long,
    Index,
}
```

#### Trait Implementations

##### `impl Clone for Identifier`

- `fn clone(self: &Self) -> Identifier` — [`Identifier`](#identifier)

##### `impl Copy for Identifier`

##### `impl Debug for Identifier`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Identifier`

##### `impl PartialEq for Identifier`

- `fn eq(self: &Self, other: &Identifier) -> bool` — [`Identifier`](#identifier)

##### `impl StructuralPartialEq for Identifier`

