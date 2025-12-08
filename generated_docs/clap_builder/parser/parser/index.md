*[clap_builder](../../index.md) / [parser](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parser`](#parser) | struct |  |
| [`PendingArg`](#pendingarg) | struct |  |
| [`ParseState`](#parsestate) | enum |  |
| [`ParseResult`](#parseresult) | enum | Recoverable Parsing results. |
| [`Identifier`](#identifier) | enum |  |

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

- <span id="parser-get-matches-with"></span>`fn get_matches_with(&mut self, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-parse"></span>`fn parse(&mut self, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-match-arg-error"></span>`fn match_arg_error(&self, arg_os: &clap_lex::ParsedArg<'_>, valid_arg_found: bool, trailing_values: bool, matcher: &ArgMatcher) -> ClapError` — [`ArgMatcher`](../arg_matcher/index.md), [`Error`](../../error/index.md)

- <span id="parser-possible-subcommand"></span>`fn possible_subcommand(&self, arg: Result<&str, &OsStr>, valid_arg_found: bool) -> Option<&str>`

- <span id="parser-possible-long-flag-subcommand"></span>`fn possible_long_flag_subcommand(&self, arg: &str) -> Option<&str>`

- <span id="parser-parse-help-subcommand"></span>`fn parse_help_subcommand(&self, cmds: impl Iterator<Item = &'cmd OsStr>) -> ClapResult<std::convert::Infallible>` — [`Result`](../../error/index.md)

- <span id="parser-is-new-arg"></span>`fn is_new_arg(&self, next: &clap_lex::ParsedArg<'_>, current_positional: &Arg) -> bool` — [`Arg`](../../index.md)

- <span id="parser-parse-subcommand"></span>`fn parse_subcommand(&mut self, sc_name: &str, matcher: &mut ArgMatcher, raw_args: &mut clap_lex::RawArgs, args_cursor: clap_lex::ArgCursor, keep_state: bool) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-parse-long-arg"></span>`fn parse_long_arg(&mut self, matcher: &mut ArgMatcher, long_arg: Result<&str, &OsStr>, long_value: Option<&OsStr>, parse_state: &ParseState, pos_counter: usize, valid_arg_found: &mut bool) -> ClapResult<ParseResult>` — [`ArgMatcher`](../arg_matcher/index.md), [`ParseState`](#parsestate), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- <span id="parser-parse-short-arg"></span>`fn parse_short_arg(&mut self, matcher: &mut ArgMatcher, short_arg: clap_lex::ShortFlags<'_>, parse_state: &ParseState, pos_counter: usize, valid_arg_found: &mut bool) -> ClapResult<ParseResult>` — [`ArgMatcher`](../arg_matcher/index.md), [`ParseState`](#parsestate), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- <span id="parser-parse-opt-value"></span>`fn parse_opt_value(&self, ident: Identifier, attached_value: Option<&OsStr>, arg: &Arg, matcher: &mut ArgMatcher, has_eq: bool) -> ClapResult<ParseResult>` — [`Identifier`](#identifier), [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- <span id="parser-check-terminator"></span>`fn check_terminator(&self, arg: &Arg, val: &OsStr) -> Option<ParseResult>` — [`Arg`](../../index.md), [`ParseResult`](#parseresult)

- <span id="parser-push-arg-values"></span>`fn push_arg_values(&self, arg: &Arg, raw_vals: Vec<OsString>, source: ValueSource, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`Arg`](../../index.md), [`ValueSource`](../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-resolve-pending"></span>`fn resolve_pending(&self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-react"></span>`fn react(&self, ident: Option<Identifier>, source: ValueSource, arg: &Arg, raw_vals: Vec<OsString>, trailing_idx: Option<usize>, matcher: &mut ArgMatcher) -> ClapResult<ParseResult>` — [`Identifier`](#identifier), [`ValueSource`](../index.md), [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md), [`ParseResult`](#parseresult)

- <span id="parser-verify-num-args"></span>`fn verify_num_args(&self, arg: &Arg, raw_vals: &[OsString]) -> ClapResult<()>` — [`Arg`](../../index.md), [`Result`](../../error/index.md)

- <span id="parser-remove-overrides"></span>`fn remove_overrides(&self, arg: &Arg, matcher: &mut ArgMatcher)` — [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md)

- <span id="parser-add-defaults"></span>`fn add_defaults(&self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-add-default-value"></span>`fn add_default_value(&self, arg: &Arg, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- <span id="parser-start-custom-arg"></span>`fn start_custom_arg(&self, matcher: &mut ArgMatcher, arg: &Arg, source: ValueSource)` — [`ArgMatcher`](../arg_matcher/index.md), [`Arg`](../../index.md), [`ValueSource`](../index.md)

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

- <span id="pendingarg-clone"></span>`fn clone(&self) -> PendingArg` — [`PendingArg`](#pendingarg)

##### `impl Debug for PendingArg`

- <span id="pendingarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PendingArg`

##### `impl PartialEq for PendingArg`

- <span id="pendingarg-eq"></span>`fn eq(&self, other: &PendingArg) -> bool` — [`PendingArg`](#pendingarg)

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

- <span id="parsestate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseState`

##### `impl PartialEq for ParseState`

- <span id="parsestate-eq"></span>`fn eq(&self, other: &ParseState) -> bool` — [`ParseState`](#parsestate)

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

- <span id="parseresult-clone"></span>`fn clone(&self) -> ParseResult` — [`ParseResult`](#parseresult)

##### `impl Debug for ParseResult`

- <span id="parseresult-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for ParseResult`

- <span id="parseresult-eq"></span>`fn eq(&self, other: &ParseResult) -> bool` — [`ParseResult`](#parseresult)

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

- <span id="identifier-clone"></span>`fn clone(&self) -> Identifier` — [`Identifier`](#identifier)

##### `impl Copy for Identifier`

##### `impl Debug for Identifier`

- <span id="identifier-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Identifier`

##### `impl PartialEq for Identifier`

- <span id="identifier-eq"></span>`fn eq(&self, other: &Identifier) -> bool` — [`Identifier`](#identifier)

##### `impl StructuralPartialEq for Identifier`

