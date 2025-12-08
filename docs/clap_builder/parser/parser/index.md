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

- `fn new(cmd: &'cmd mut Command) -> Self` — [`Command`](../../builder/command/index.md)

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

