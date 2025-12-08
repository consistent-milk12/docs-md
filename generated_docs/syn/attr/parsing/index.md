*[syn](../../index.md) / [attr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`DisplayAttrStyle`](#displayattrstyle)
  - [`DisplayPath`](#displaypath)
- [Functions](#functions)
  - [`parse_inner`](#parse_inner)
  - [`single_parse_inner`](#single_parse_inner)
  - [`single_parse_outer`](#single_parse_outer)
  - [`parse_outermost_meta_path`](#parse_outermost_meta_path)
  - [`parse_meta_after_path`](#parse_meta_after_path)
  - [`parse_meta_list_after_path`](#parse_meta_list_after_path)
  - [`parse_meta_name_value_after_path`](#parse_meta_name_value_after_path)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DisplayAttrStyle`](#displayattrstyle) | struct |  |
| [`DisplayPath`](#displaypath) | struct |  |
| [`parse_inner`](#parse_inner) | fn |  |
| [`single_parse_inner`](#single_parse_inner) | fn |  |
| [`single_parse_outer`](#single_parse_outer) | fn |  |
| [`parse_outermost_meta_path`](#parse_outermost_meta_path) | fn |  |
| [`parse_meta_after_path`](#parse_meta_after_path) | fn |  |
| [`parse_meta_list_after_path`](#parse_meta_list_after_path) | fn |  |
| [`parse_meta_name_value_after_path`](#parse_meta_name_value_after_path) | fn |  |

## Structs

### `DisplayAttrStyle<'a>`

```rust
struct DisplayAttrStyle<'a>(&'a crate::attr::AttrStyle);
```

#### Trait Implementations

##### `impl<'a> Display for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DisplayAttrStyle<'a>`

- <span id="displayattrstyle-to-string"></span>`fn to_string(&self) -> String`

### `DisplayPath<'a>`

```rust
struct DisplayPath<'a>(&'a crate::path::Path);
```

#### Trait Implementations

##### `impl<'a> Display for DisplayPath<'a>`

- <span id="displaypath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DisplayPath<'a>`

- <span id="displaypath-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `parse_inner`

```rust
fn parse_inner(input: crate::parse::ParseStream<'_>, attrs: &mut Vec<crate::attr::Attribute>) -> crate::error::Result<()>
```

### `single_parse_inner`

```rust
fn single_parse_inner(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Attribute>
```

### `single_parse_outer`

```rust
fn single_parse_outer(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Attribute>
```

### `parse_outermost_meta_path`

```rust
fn parse_outermost_meta_path(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::path::Path>
```

### `parse_meta_after_path`

```rust
fn parse_meta_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::Meta>
```

### `parse_meta_list_after_path`

```rust
fn parse_meta_list_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::MetaList>
```

### `parse_meta_name_value_after_path`

```rust
fn parse_meta_name_value_after_path(path: crate::path::Path, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::attr::MetaNameValue>
```

