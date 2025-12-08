*[clap_builder](../index.md) / [util](index.md)*

---

# Module `util`

## Modules

- [`any_value`](any_value/index.md) - 
- [`flat_map`](flat_map/index.md) - 
- [`flat_set`](flat_set/index.md) - 
- [`graph`](graph/index.md) - 
- [`id`](id/index.md) - 
- [`str_to_bool`](str_to_bool/index.md) - 
- [`color`](color/index.md) - 

## Structs

### `Id`

```rust
struct Id(crate::builder::Str);
```

`Arg` or `ArgGroup` identifier

This is used for accessing the value in `ArgMatches` or defining
relationships between `Arg`s and `ArgGroup`s with functions like
`Arg::conflicts_with`.

#### Implementations

- `const HELP: &'static str`

- `const VERSION: &'static str`

- `const EXTERNAL: &'static str`

- `fn from_static_ref(name: &'static str) -> Self`

- `fn as_str(self: &Self) -> &str`

- `fn as_internal_str(self: &Self) -> &Str` — [`Str`](../builder/index.md)

#### Trait Implementations

##### `impl AsRef for Id`

- `fn as_ref(self: &Self) -> &str`

##### `impl Clone for Id`

- `fn clone(self: &Self) -> Id` — [`Id`](../index.md)

##### `impl Debug for Id`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for Id`

- `fn default() -> Id` — [`Id`](../index.md)

##### `impl Display for Id`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for Id`

##### `impl Hash for Id`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<I> IntoResettable for Id`

- `fn into_resettable(self: Self) -> Resettable<Str>` — [`Resettable`](../builder/index.md), [`Str`](../builder/index.md)

##### `impl Ord for Id`

- `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering` — [`Id`](../index.md)

##### `impl PartialEq for Id`

- `fn eq(self: &Self, other: &str) -> bool`

##### `impl PartialOrd for Id`

- `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Id`](../index.md)

##### `impl StructuralPartialEq for Id`

##### `impl<T> ToString for Id`

- `fn to_string(self: &Self) -> String`

## Functions

### `eq_ignore_case`

```rust
fn eq_ignore_case(left: &str, right: &str) -> bool
```

## Constants

### `SUCCESS_CODE`

```rust
const SUCCESS_CODE: i32 = 0i32;
```

### `USAGE_CODE`

```rust
const USAGE_CODE: i32 = 2i32;
```

