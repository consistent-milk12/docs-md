*[syn](../../index.md) / [attr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`DisplayAttrStyle`](#displayattrstyle)
  - [`DisplayPath`](#displaypath)
- [Functions](#functions)
  - [`parse_inner`](#parse-inner)
  - [`single_parse_inner`](#single-parse-inner)
  - [`single_parse_outer`](#single-parse-outer)
  - [`parse_outermost_meta_path`](#parse-outermost-meta-path)
  - [`parse_meta_after_path`](#parse-meta-after-path)
  - [`parse_meta_list_after_path`](#parse-meta-list-after-path)
  - [`parse_meta_name_value_after_path`](#parse-meta-name-value-after-path)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DisplayAttrStyle`](#displayattrstyle) | struct |  |
| [`DisplayPath`](#displaypath) | struct |  |
| [`parse_inner`](#parse-inner) | fn |  |
| [`single_parse_inner`](#single-parse-inner) | fn |  |
| [`single_parse_outer`](#single-parse-outer) | fn |  |
| [`parse_outermost_meta_path`](#parse-outermost-meta-path) | fn |  |
| [`parse_meta_after_path`](#parse-meta-after-path) | fn |  |
| [`parse_meta_list_after_path`](#parse-meta-list-after-path) | fn |  |
| [`parse_meta_name_value_after_path`](#parse-meta-name-value-after-path) | fn |  |

## Structs

### `DisplayAttrStyle<'a>`

```rust
struct DisplayAttrStyle<'a>(&'a crate::attr::AttrStyle);
```

*Defined in [`syn-2.0.111/src/attr.rs:762`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L762)*

#### Trait Implementations

##### `impl Any for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="displayattrstyle-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="displayattrstyle-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DisplayPath<'a>`

```rust
struct DisplayPath<'a>(&'a crate::path::Path);
```

*Defined in [`syn-2.0.111/src/attr.rs:773`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L773)*

#### Trait Implementations

##### `impl Any for DisplayPath<'a>`

- <span id="displaypath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DisplayPath<'a>`

- <span id="displaypath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DisplayPath<'a>`

- <span id="displaypath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for DisplayPath<'a>`

- <span id="displaypath-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DisplayPath<'a>`

- <span id="displaypath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DisplayPath<'a>`

- <span id="displaypath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for DisplayPath<'a>`

- <span id="displaypath-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DisplayPath<'a>`

- <span id="displaypath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="displaypath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DisplayPath<'a>`

- <span id="displaypath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="displaypath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse_inner`

```rust
fn parse_inner(input: crate::parse::ParseStream<'_>, attrs: &mut Vec<crate::attr::Attribute>) -> crate::error::Result<()>
```

*Defined in [`syn-2.0.111/src/attr.rs:659-664`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L659-L664)*

### `single_parse_inner`

```rust
fn single_parse_inner(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Attribute>
```

*Defined in [`syn-2.0.111/src/attr.rs:666-674`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L666-L674)*

### `single_parse_outer`

```rust
fn single_parse_outer(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Attribute>
```

*Defined in [`syn-2.0.111/src/attr.rs:676-684`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L676-L684)*

### `parse_outermost_meta_path`

```rust
fn parse_outermost_meta_path(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::path::Path>
```

*Defined in [`syn-2.0.111/src/attr.rs:712-719`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L712-L719)*

### `parse_meta_after_path`

```rust
fn parse_meta_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Meta>
```

*Defined in [`syn-2.0.111/src/attr.rs:721-729`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L721-L729)*

### `parse_meta_list_after_path`

```rust
fn parse_meta_list_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::MetaList>
```

*Defined in [`syn-2.0.111/src/attr.rs:731-738`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L731-L738)*

### `parse_meta_name_value_after_path`

```rust
fn parse_meta_name_value_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::MetaNameValue>
```

*Defined in [`syn-2.0.111/src/attr.rs:740-760`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L740-L760)*

