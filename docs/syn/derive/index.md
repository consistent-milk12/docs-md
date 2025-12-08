*[syn](../index.md) / [derive](index.md)*

---

# Module `derive`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `DeriveInput`

```rust
struct DeriveInput {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub data: Data,
}
```

Data structure sent to a `proc_macro_derive` macro.

#### Trait Implementations

##### `impl Clone for crate::DeriveInput`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DeriveInput`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DeriveInput`

##### `impl Hash for crate::DeriveInput`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::derive::DeriveInput`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::DeriveInput`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for DeriveInput`

##### `impl<T> Spanned for DeriveInput`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::derive::DeriveInput`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `DataStruct`

```rust
struct DataStruct {
    pub struct_token: $crate::token::Struct,
    pub fields: crate::data::Fields,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A struct input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataStruct`

##### `impl Hash for crate::DataStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

### `DataEnum`

```rust
struct DataEnum {
    pub enum_token: $crate::token::Enum,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>,
}
```

An enum input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataEnum`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataEnum`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataEnum`

##### `impl Hash for crate::DataEnum`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataEnum`

- `fn eq(self: &Self, other: &Self) -> bool`

### `DataUnion`

```rust
struct DataUnion {
    pub union_token: $crate::token::Union,
    pub fields: crate::data::FieldsNamed,
}
```

An untagged union input to a `proc_macro_derive` macro.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::DataUnion`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::DataUnion`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::DataUnion`

##### `impl Hash for crate::DataUnion`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::DataUnion`

- `fn eq(self: &Self, other: &Self) -> bool`

## Enums

### `Data`

```rust
enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
```

The storage of a struct, enum or union data structure.

# Syntax tree enum

This type is a [syntax tree enum].


#### Trait Implementations

##### `impl Clone for crate::Data`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Data`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Data`

##### `impl Hash for crate::Data`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Data`

- `fn eq(self: &Self, other: &Self) -> bool`

