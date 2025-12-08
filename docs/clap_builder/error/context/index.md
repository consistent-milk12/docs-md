*[clap_builder](../../index.md) / [error](../index.md) / [context](index.md)*

---

# Module `context`

## Enums

### `ContextKind`

```rust
enum ContextKind {
    InvalidSubcommand,
    InvalidArg,
    PriorArg,
    ValidSubcommand,
    ValidValue,
    InvalidValue,
    ActualNumValues,
    ExpectedNumValues,
    MinValues,
    SuggestedCommand,
    SuggestedSubcommand,
    SuggestedArg,
    SuggestedValue,
    TrailingArg,
    Suggested,
    Usage,
    Custom,
}
```

Semantics for a piece of error information

#### Variants

- **`InvalidSubcommand`**

  The cause of the error

- **`InvalidArg`**

  The cause of the error

- **`PriorArg`**

  Existing arguments

- **`ValidSubcommand`**

  Accepted subcommands

- **`ValidValue`**

  Accepted values

- **`InvalidValue`**

  Rejected values

- **`ActualNumValues`**

  Number of values present

- **`ExpectedNumValues`**

  Number of allowed values

- **`MinValues`**

  Minimum number of allowed values

- **`SuggestedCommand`**

  Potential fix for the user

- **`SuggestedSubcommand`**

  Potential fix for the user

- **`SuggestedArg`**

  Potential fix for the user

- **`SuggestedValue`**

  Potential fix for the user

- **`TrailingArg`**

  Trailing argument

- **`Suggested`**

  Potential fix for the user

- **`Usage`**

  A usage string

- **`Custom`**

  An opaque message to the user

#### Implementations

- `fn as_str(self: Self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ContextKind`

- `fn clone(self: &Self) -> ContextKind` — [`ContextKind`](#contextkind)

##### `impl Copy for ContextKind`

##### `impl Debug for ContextKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ContextKind`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextKind`

##### `impl Hash for ContextKind`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for ContextKind`

- `fn eq(self: &Self, other: &ContextKind) -> bool` — [`ContextKind`](#contextkind)

##### `impl StructuralPartialEq for ContextKind`

##### `impl<T> ToString for ContextKind`

- `fn to_string(self: &Self) -> String`

### `ContextValue`

```rust
enum ContextValue {
    None,
    Bool(bool),
    String(String),
    Strings(Vec<String>),
    StyledStr(crate::builder::StyledStr),
    StyledStrs(Vec<crate::builder::StyledStr>),
    Number(isize),
}
```

A piece of error information

#### Variants

- **`None`**

  [`ContextKind`](#contextkind) is self-sufficient, no additional information needed

- **`Bool`**

  A single value

- **`String`**

  A single value

- **`Strings`**

  Many values

- **`StyledStr`**

  A single value

- **`StyledStrs`**

  many value

- **`Number`**

  A single value

#### Trait Implementations

##### `impl Clone for ContextValue`

- `fn clone(self: &Self) -> ContextValue` — [`ContextValue`](#contextvalue)

##### `impl Debug for ContextValue`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for ContextValue`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextValue`

##### `impl PartialEq for ContextValue`

- `fn eq(self: &Self, other: &ContextValue) -> bool` — [`ContextValue`](#contextvalue)

##### `impl StructuralPartialEq for ContextValue`

##### `impl<T> ToString for ContextValue`

- `fn to_string(self: &Self) -> String`

