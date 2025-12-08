*[syn](../index.md) / [data](index.md)*

---

# Module `data`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Variant`

```rust
struct Variant {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub fields: Fields,
    pub discriminant: Option<($crate::token::Eq, crate::expr::Expr)>,
}
```

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<($crate::token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

#### Trait Implementations

##### `impl Clone for crate::Variant`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Variant`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl Hash for crate::Variant`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::Variant`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Variant`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Variant`

##### `impl<T> Spanned for Variant`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::Variant`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsNamed`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldsNamed`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl Hash for crate::FieldsNamed`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::FieldsNamed`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::FieldsNamed`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsNamed`

##### `impl<T> Spanned for FieldsNamed`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::FieldsNamed`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, $crate::token::Comma>,
}
```

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsUnnamed`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldsUnnamed`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl Hash for crate::FieldsUnnamed`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::data::FieldsUnnamed`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::FieldsUnnamed`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsUnnamed`

##### `impl<T> Spanned for FieldsUnnamed`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Field`

```rust
struct Field {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub mutability: crate::restriction::FieldMutability,
    pub ident: Option<crate::ident::Ident>,
    pub colon_token: Option<$crate::token::Colon>,
    pub ty: crate::ty::Type,
}
```

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- `fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

- `fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::Field`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Field`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl Hash for crate::Field`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Field`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Field`

##### `impl<T> Spanned for Field`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::data::Field`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Members<'a>`

```rust
struct Members<'a> {
    fields: punctuated::Iter<'a, Field>,
    index: u32,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Members<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<I> IntoIterator for Members<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Members<'a>`

- `type Item = Member`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Enums

### `Fields`

```rust
enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
```

Data stored within an enum variant or struct.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Named`**

  Named fields of a struct or struct variant such as `Point { x: f64,
  y: f64 }`.

- **`Unnamed`**

  Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

- **`Unit`**

  Unit struct or unit variant such as `None`.

#### Implementations

- `fn iter(self: &Self) -> punctuated::Iter<'_, Field>` — [`Iter`](../punctuated/index.md), [`Field`](#field)

- `fn iter_mut(self: &mut Self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](../punctuated/index.md), [`Field`](#field)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn members(self: &Self) -> Members<'_>` — [`Members`](#members)

#### Trait Implementations

##### `impl Clone for crate::Fields`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Fields`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl Hash for crate::Fields`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl IntoIterator for Fields`

- `type Item = Field`

- `type IntoIter = IntoIter<Field>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Fields`

##### `impl<T> Spanned for Fields`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Fields`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

