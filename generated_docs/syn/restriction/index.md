*[syn](../index.md) / [restriction](index.md)*

---

# Module `restriction`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: $crate::token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<$crate::token::In>,
    pub path: Box<crate::path::Path>,
}
```

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::VisRestricted`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::VisRestricted`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl Hash for crate::VisRestricted`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::VisRestricted`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for VisRestricted`

##### `impl<T> Spanned for VisRestricted`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::restriction::VisRestricted`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `Visibility`

```rust
enum Visibility {
    Public($crate::token::Pub),
    Restricted(VisRestricted),
    Inherited,
}
```

The visibility level of an item: inherited or `pub` or
`pub(restricted)`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Public`**

  A public visibility level: `pub`.

- **`Restricted`**

  A visibility level restricted to some path: `pub(self)` or
  `pub(super)` or `pub(crate)` or `pub(in some::module)`.

- **`Inherited`**

  An inherited visibility, which usually means private.

#### Implementations

- `fn parse_pub(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn is_some(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Visibility`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Visibility`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl Hash for crate::Visibility`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::restriction::Visibility`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Visibility`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Visibility`

##### `impl<T> Spanned for Visibility`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::restriction::Visibility`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::FieldMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl Hash for crate::FieldMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::FieldMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

