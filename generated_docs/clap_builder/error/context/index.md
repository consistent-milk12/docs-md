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

*Defined in [`clap_builder-4.5.53/src/error/context.rs:5-40`](../../../../.source_1765633015/clap_builder-4.5.53/src/error/context.rs#L5-L40)*

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

  End-user description of the error case, where relevant

#### Trait Implementations

##### `impl Any for ContextKind`

- <span id="contextkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ContextKind`

- <span id="contextkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ContextKind`

- <span id="contextkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ContextKind`

- <span id="contextkind-clone"></span>`fn clone(&self) -> ContextKind` — [`ContextKind`](#contextkind)

##### `impl CloneToUninit for ContextKind`

- <span id="contextkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ContextKind`

##### `impl Debug for ContextKind`

- <span id="contextkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextKind`

- <span id="contextkind-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextKind`

##### `impl<T> From for ContextKind`

- <span id="contextkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ContextKind`

- <span id="contextkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ContextKind`

- <span id="contextkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ContextKind`

- <span id="contextkind-partialeq-eq"></span>`fn eq(&self, other: &ContextKind) -> bool` — [`ContextKind`](#contextkind)

##### `impl StructuralPartialEq for ContextKind`

##### `impl ToOwned for ContextKind`

- <span id="contextkind-toowned-type-owned"></span>`type Owned = T`

- <span id="contextkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="contextkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ContextKind`

- <span id="contextkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ContextKind`

- <span id="contextkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="contextkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ContextKind`

- <span id="contextkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="contextkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`clap_builder-4.5.53/src/error/context.rs:77-92`](../../../../.source_1765633015/clap_builder-4.5.53/src/error/context.rs#L77-L92)*

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

##### `impl Any for ContextValue`

- <span id="contextvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ContextValue`

- <span id="contextvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ContextValue`

- <span id="contextvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ContextValue`

- <span id="contextvalue-clone"></span>`fn clone(&self) -> ContextValue` — [`ContextValue`](#contextvalue)

##### `impl CloneToUninit for ContextValue`

- <span id="contextvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ContextValue`

- <span id="contextvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContextValue`

- <span id="contextvalue-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ContextValue`

##### `impl<T> From for ContextValue`

- <span id="contextvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ContextValue`

- <span id="contextvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ContextValue`

- <span id="contextvalue-partialeq-eq"></span>`fn eq(&self, other: &ContextValue) -> bool` — [`ContextValue`](#contextvalue)

##### `impl StructuralPartialEq for ContextValue`

##### `impl ToOwned for ContextValue`

- <span id="contextvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="contextvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="contextvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ContextValue`

- <span id="contextvalue-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ContextValue`

- <span id="contextvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="contextvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ContextValue`

- <span id="contextvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="contextvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

