*[clap_builder](../index.md) / [util](index.md)*

---

# Module `util`

## Contents

- [Modules](#modules)
  - [`any_value`](#any_value)
  - [`flat_map`](#flat_map)
  - [`flat_set`](#flat_set)
  - [`graph`](#graph)
  - [`id`](#id)
  - [`str_to_bool`](#str_to_bool)
  - [`color`](#color)
- [Structs](#structs)
  - [`Id`](#id)
- [Functions](#functions)
  - [`eq_ignore_case`](#eq_ignore_case)
- [Constants](#constants)
  - [`SUCCESS_CODE`](#success_code)
  - [`USAGE_CODE`](#usage_code)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`any_value`](#any_value) | mod |  |
| [`flat_map`](#flat_map) | mod |  |
| [`flat_set`](#flat_set) | mod |  |
| [`graph`](#graph) | mod |  |
| [`id`](#id) | mod |  |
| [`str_to_bool`](#str_to_bool) | mod |  |
| [`color`](#color) | mod |  |
| [`Id`](#id) | struct |  |
| [`eq_ignore_case`](#eq_ignore_case) | fn |  |
| [`SUCCESS_CODE`](#success_code) | const |  |
| [`USAGE_CODE`](#usage_code) | const |  |

## Modules

- [`any_value`](any_value/index.md)
- [`flat_map`](flat_map/index.md)
- [`flat_set`](flat_set/index.md)
- [`graph`](graph/index.md)
- [`id`](id/index.md)
- [`str_to_bool`](str_to_bool/index.md)
- [`color`](color/index.md)

## Structs

### `Id`

```rust
struct Id(crate::builder::Str);
```

*Defined in [`clap_builder-4.5.53/src/util/id.rs:11`](../../../.source_1765210505/clap_builder-4.5.53/src/util/id.rs#L11)*

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- <span id="id-const-help"></span>`const HELP: &'static str`

- <span id="id-const-version"></span>`const VERSION: &'static str`

- <span id="id-const-external"></span>`const EXTERNAL: &'static str`

- <span id="id-from-static-ref"></span>`fn from_static_ref(name: &'static str) -> Self`

- <span id="id-as-str"></span>`fn as_str(&self) -> &str`

- <span id="id-as-internal-str"></span>`fn as_internal_str(&self) -> &Str` — [`Str`](../builder/str/index.md)

#### Trait Implementations

##### `impl AsRef for Id`

- <span id="id-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for Id`

- <span id="id-clone"></span>`fn clone(&self) -> Id` — [`Id`](id/index.md)

##### `impl Debug for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- <span id="id-default"></span>`fn default() -> Id` — [`Id`](id/index.md)

##### `impl Display for Id`

- <span id="id-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl Hash for Id`

- <span id="id-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for Command`

- <span id="command-type-output"></span>`type Output = Arg`

- <span id="command-index"></span>`fn index(&self, key: &Id) -> &<Self as >::Output` — [`Id`](id/index.md)

##### `impl IntoResettable for Str`

- <span id="str-into-resettable"></span>`fn into_resettable(self) -> Resettable<Id>` — [`Resettable`](../builder/resettable/index.md), [`Id`](id/index.md)

##### `impl Ord for Id`

- <span id="id-cmp"></span>`fn cmp(&self, other: &Id) -> cmp::Ordering` — [`Id`](id/index.md)

##### `impl PartialEq for Id`

- <span id="id-eq"></span>`fn eq(&self, other: &Id) -> bool` — [`Id`](id/index.md)

##### `impl PartialOrd for Id`

- <span id="id-partial-cmp"></span>`fn partial_cmp(&self, other: &Id) -> option::Option<cmp::Ordering>` — [`Id`](id/index.md)

##### `impl StructuralPartialEq for Id`

##### `impl ToString for Id`

- <span id="id-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `eq_ignore_case`

```rust
fn eq_ignore_case(left: &str, right: &str) -> bool
```

*Defined in [`clap_builder-4.5.53/src/util/mod.rs:33-35`](../../../.source_1765210505/clap_builder-4.5.53/src/util/mod.rs#L33-L35)*

## Constants

### `SUCCESS_CODE`
```rust
const SUCCESS_CODE: i32 = 0i32;
```

*Defined in [`clap_builder-4.5.53/src/util/mod.rs:24`](../../../.source_1765210505/clap_builder-4.5.53/src/util/mod.rs#L24)*

### `USAGE_CODE`
```rust
const USAGE_CODE: i32 = 2i32;
```

*Defined in [`clap_builder-4.5.53/src/util/mod.rs:30`](../../../.source_1765210505/clap_builder-4.5.53/src/util/mod.rs#L30)*

