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

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:23-31`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L23-L31)*

#### Fields

- **`flag_subcmd_at`**: `Option<usize>`

  Index of the previous flag subcommand in a group of flags.

- **`flag_subcmd_skip`**: `usize`

  Counter indicating the number of items to skip
  when revisiting the group of flags which includes the flag subcommand.

#### Implementations

- <span id="parser-new"></span>`fn new(cmd: &'cmd mut Command) -> Self` — [`Command`](../../builder/command/index.md#command)

### `PendingArg`

```rust
struct PendingArg {
    id: crate::util::Id,
    ident: Option<Identifier>,
    raw_vals: Vec<std::ffi::OsString>,
    trailing_idx: Option<usize>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1666-1671`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1666-L1671)*

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

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1629-1633`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1629-L1633)*

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

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1638-1663`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1638-L1663)*

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

*Defined in [`clap_builder-4.5.53/src/parser/parser.rs:1674-1678`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/parser.rs#L1674-L1678)*

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

