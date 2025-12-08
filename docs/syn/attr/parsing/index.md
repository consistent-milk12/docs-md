*[syn](../../index.md) / [attr](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Structs

### `DisplayAttrStyle<'a>`

```rust
struct DisplayAttrStyle<'a>(&'a crate::attr::AttrStyle);
```

#### Trait Implementations

##### `impl<'a> Display for DisplayAttrStyle<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DisplayAttrStyle<'a>`

- `fn to_string(self: &Self) -> String`

### `DisplayPath<'a>`

```rust
struct DisplayPath<'a>(&'a crate::path::Path);
```

#### Trait Implementations

##### `impl<'a> Display for DisplayPath<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DisplayPath<'a>`

- `fn to_string(self: &Self) -> String`

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

