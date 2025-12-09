*[syn](../index.md) / [restriction](index.md)*

---

# Module `restriction`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`VisRestricted`](#visrestricted) | struct | A visibility level restricted to some path: `pub(self)` or |
| [`Visibility`](#visibility) | enum | The visibility level of an item: inherited or `pub` or |
| [`FieldMutability`](#fieldmutability) | enum | Unused, but reserved for RFC 3323 restrictions. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `VisRestricted`

```rust
struct VisRestricted {
    pub pub_token: token::Pub,
    pub paren_token: token::Paren,
    pub in_token: Option<token::In>,
    pub path: Box<crate::path::Path>,
}
```

A visibility level restricted to some path: `pub(self)` or
`pub(super)` or `pub(crate)` or `pub(in some::module)`.

#### Implementations

- <span id="cratevisrestricted-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::VisRestricted`

- <span id="cratevisrestricted-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::VisRestricted`

- <span id="cratevisrestricted-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::VisRestricted`

##### `impl Hash for crate::VisRestricted`

- <span id="cratevisrestricted-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::VisRestricted`

- <span id="cratevisrestricted-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for VisRestricted`

##### `impl<T> Spanned for VisRestricted`

- <span id="visrestricted-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::restriction::VisRestricted`

- <span id="craterestrictionvisrestricted-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Visibility`

```rust
enum Visibility {
    Public(token::Pub),
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

- <span id="craterestrictionvisibility-is-inherited"></span>`fn is_inherited(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Visibility`

- <span id="cratevisibility-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Visibility`

- <span id="cratevisibility-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Visibility`

##### `impl Hash for crate::Visibility`

- <span id="cratevisibility-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Visibility`

- <span id="cratevisibility-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Visibility`

##### `impl<T> Spanned for Visibility`

- <span id="visibility-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::restriction::Visibility`

- <span id="craterestrictionvisibility-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldMutability`

```rust
enum FieldMutability {
    None,
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::FieldMutability`

- <span id="cratefieldmutability-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldMutability`

- <span id="cratefieldmutability-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldMutability`

##### `impl Hash for crate::FieldMutability`

- <span id="cratefieldmutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::FieldMutability`

- <span id="cratefieldmutability-eq"></span>`fn eq(&self, other: &Self) -> bool`

