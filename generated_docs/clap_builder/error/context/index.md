*[clap_builder](../../index.md) / [error](../index.md) / [context](index.md)*

---

# Module `context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ContextKind`](#contextkind) | enum | Semantics for a piece of error information |
| [`ContextValue`](#contextvalue) | enum | A piece of error information |

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

*Defined in [`clap_builder-4.5.53/src/error/context.rs:5-40`](../../../../.source_1765210505/clap_builder-4.5.53/src/error/context.rs#L5-L40)*

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

- <span id="contextkind-as-str"></span>`fn as_str(self) -> Option<&'static str>`

#### Trait Implementations

##### `impl Clone for ContextKind`

- <span id="contextkind-clone"></span>`fn clone(&self) -> ContextKind` — [`ContextKind`](#contextkind)

##### `impl Copy for ContextKind`

##### `impl Debug for ContextKind`

- <span id="contextkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextKind`

- <span id="contextkind-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextKind`

##### `impl Hash for ContextKind`

- <span id="contextkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ContextKind`

- <span id="contextkind-eq"></span>`fn eq(&self, other: &ContextKind) -> bool` — [`ContextKind`](#contextkind)

##### `impl StructuralPartialEq for ContextKind`

##### `impl ToString for ContextKind`

- <span id="contextkind-to-string"></span>`fn to_string(&self) -> String`

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

*Defined in [`clap_builder-4.5.53/src/error/context.rs:77-92`](../../../../.source_1765210505/clap_builder-4.5.53/src/error/context.rs#L77-L92)*

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

- <span id="contextvalue-clone"></span>`fn clone(&self) -> ContextValue` — [`ContextValue`](#contextvalue)

##### `impl Debug for ContextValue`

- <span id="contextvalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextValue`

- <span id="contextvalue-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextValue`

##### `impl PartialEq for ContextValue`

- <span id="contextvalue-eq"></span>`fn eq(&self, other: &ContextValue) -> bool` — [`ContextValue`](#contextvalue)

##### `impl StructuralPartialEq for ContextValue`

##### `impl ToString for ContextValue`

- <span id="contextvalue-to-string"></span>`fn to_string(&self) -> String`

