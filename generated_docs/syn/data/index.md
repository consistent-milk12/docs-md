*[syn](../index.md) / [data](index.md)*

---

# Module `data`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Variant`](#variant)
  - [`FieldsNamed`](#fieldsnamed)
  - [`FieldsUnnamed`](#fieldsunnamed)
  - [`Field`](#field)
  - [`Members`](#members)
- [Enums](#enums)
  - [`Fields`](#fields)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Variant`](#variant) | struct | An enum variant. |
| [`FieldsNamed`](#fieldsnamed) | struct | Named fields of a struct or struct variant such as `Point { x: f64 |
| [`FieldsUnnamed`](#fieldsunnamed) | struct | Unnamed fields of a tuple struct or tuple variant such as `Some(T)`. |
| [`Field`](#field) | struct | A field of a struct or enum variant. |
| [`Members`](#members) | struct |  |
| [`Fields`](#fields) | enum | Data stored within an enum variant or struct. |

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
    pub discriminant: Option<(token::Eq, crate::expr::Expr)>,
}
```

An enum variant.

#### Fields

- **`ident`**: `crate::ident::Ident`

  Name of the variant.

- **`fields`**: `Fields`

  Content stored in the variant.

- **`discriminant`**: `Option<(token::Eq, crate::expr::Expr)>`

  Explicit discriminant: `Variant = 1`

#### Trait Implementations

##### `impl Clone for crate::Variant`

- <span id="cratevariant-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Variant`

- <span id="cratevariant-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variant`

##### `impl Hash for crate::Variant`

- <span id="cratevariant-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::Variant`

- <span id="cratedatavariant-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Variant`

- <span id="cratevariant-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Variant`

##### `impl<T> Spanned for Variant`

- <span id="variant-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::Variant`

- <span id="cratedatavariant-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldsNamed`

```rust
struct FieldsNamed {
    pub brace_token: token::Brace,
    pub named: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

Named fields of a struct or struct variant such as `Point { x: f64,
y: f64 }`.

#### Implementations

- <span id="cratefieldsnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsNamed`

- <span id="cratefieldsnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldsNamed`

- <span id="cratefieldsnamed-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsNamed`

##### `impl Hash for crate::FieldsNamed`

- <span id="cratefieldsnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::FieldsNamed`

- <span id="cratefieldsnamed-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsNamed`

##### `impl<T> Spanned for FieldsNamed`

- <span id="fieldsnamed-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::FieldsNamed`

- <span id="cratedatafieldsnamed-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `FieldsUnnamed`

```rust
struct FieldsUnnamed {
    pub paren_token: token::Paren,
    pub unnamed: crate::punctuated::Punctuated<Field, token::Comma>,
}
```

Unnamed fields of a tuple struct or tuple variant such as `Some(T)`.

#### Implementations

- <span id="cratefieldsunnamed-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldsUnnamed`

##### `impl Hash for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::FieldsUnnamed`

- <span id="cratefieldsunnamed-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldsUnnamed`

##### `impl<T> Spanned for FieldsUnnamed`

- <span id="fieldsunnamed-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::FieldsUnnamed`

- <span id="cratedatafieldsunnamed-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Field`

```rust
struct Field {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub mutability: crate::restriction::FieldMutability,
    pub ident: Option<crate::ident::Ident>,
    pub colon_token: Option<token::Colon>,
    pub ty: crate::ty::Type,
}
```

A field of a struct or enum variant.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  Name of the field, if any.
  
  Fields of tuple structs have no names.

#### Implementations

- <span id="cratedatafield-parse-named"></span>`fn parse_named(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="cratedatafield-parse-unnamed"></span>`fn parse_unnamed(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Field`

- <span id="cratefield-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Field`

- <span id="cratefield-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Field`

##### `impl Hash for crate::Field`

- <span id="cratefield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Field`

- <span id="cratefield-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Field`

##### `impl<T> Spanned for Field`

- <span id="field-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::data::Field`

- <span id="cratedatafield-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Members<'a>`

```rust
struct Members<'a> {
    fields: punctuated::Iter<'a, Field>,
    index: u32,
}
```

#### Trait Implementations

##### `impl<'a> Clone for Members<'a>`

- <span id="members-clone"></span>`fn clone(&self) -> Self`

##### `impl<I> IntoIterator for Members<'a>`

- <span id="members-item"></span>`type Item = <I as Iterator>::Item`

- <span id="members-intoiter"></span>`type IntoIter = I`

- <span id="members-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for Members<'a>`

- <span id="members-item"></span>`type Item = Member`

- <span id="members-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

- <span id="fields-iter"></span>`fn iter(&self) -> punctuated::Iter<'_, Field>` — [`Iter`](../punctuated/index.md), [`Field`](../index.md)

- <span id="fields-iter-mut"></span>`fn iter_mut(&mut self) -> punctuated::IterMut<'_, Field>` — [`IterMut`](../punctuated/index.md), [`Field`](../index.md)

- <span id="fields-len"></span>`fn len(&self) -> usize`

- <span id="fields-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="fields-members"></span>`fn members(&self) -> Members<'_>` — [`Members`](#members)

#### Trait Implementations

##### `impl Clone for crate::Fields`

- <span id="cratefields-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Fields`

- <span id="cratefields-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Fields`

##### `impl Hash for crate::Fields`

- <span id="cratefields-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl IntoIterator for Fields`

- <span id="fields-item"></span>`type Item = Field`

- <span id="fields-intoiter"></span>`type IntoIter = IntoIter<Field>`

- <span id="fields-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for crate::Fields`

- <span id="cratefields-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Fields`

##### `impl<T> Spanned for Fields`

- <span id="fields-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Fields`

- <span id="fields-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

