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

##### `impl Display for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-to-string"></span>`fn to_string(&self) -> String`

### `DisplayPath<'a>`

```rust
struct DisplayPath<'a>(&'a crate::path::Path);
```

*Defined in [`syn-2.0.111/src/attr.rs:773`](../../../../.source_1765521767/syn-2.0.111/src/attr.rs#L773)*

#### Trait Implementations

##### `impl Display for DisplayPath<'a>`

- <span id="displaypath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for DisplayPath<'a>`

- <span id="displaypath-to-string"></span>`fn to_string(&self) -> String`

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

