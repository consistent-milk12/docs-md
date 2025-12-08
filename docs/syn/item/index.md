*[syn](../index.md) / [item](index.md)*

---

# Module `item`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `ItemConst`

```rust
struct ItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: $crate::token::Semi,
}
```

A constant item: `const MAX: u16 = 65535`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemConst`

##### `impl Hash for crate::ItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemConst`

##### `impl<T> Spanned for ItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemEnum`

```rust
struct ItemEnum {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub enum_token: $crate::token::Enum,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub brace_token: token::Brace,
    pub variants: crate::punctuated::Punctuated<crate::data::Variant, $crate::token::Comma>,
}
```

An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemEnum`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemEnum`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemEnum`

##### `impl Hash for crate::ItemEnum`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemEnum`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemEnum`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemEnum`

##### `impl<T> Spanned for ItemEnum`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemEnum`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemExternCrate`

```rust
struct ItemExternCrate {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub extern_token: $crate::token::Extern,
    pub crate_token: $crate::token::Crate,
    pub ident: crate::ident::Ident,
    pub rename: Option<($crate::token::As, crate::ident::Ident)>,
    pub semi_token: $crate::token::Semi,
}
```

An `extern crate` item: `extern crate serde`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemExternCrate`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemExternCrate`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemExternCrate`

##### `impl Hash for crate::ItemExternCrate`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemExternCrate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemExternCrate`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemExternCrate`

##### `impl<T> Spanned for ItemExternCrate`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemExternCrate`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemFn`

```rust
struct ItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub block: Box<crate::stmt::Block>,
}
```

A free-standing function: `fn process(n: usize) -> Result<()> { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemFn`

##### `impl Hash for crate::ItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemFn`

##### `impl<T> Spanned for ItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemForeignMod`

```rust
struct ItemForeignMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: crate::ty::Abi,
    pub brace_token: token::Brace,
    pub items: Vec<ForeignItem>,
}
```

A block of foreign items: `extern "C" { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemForeignMod`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemForeignMod`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemForeignMod`

##### `impl Hash for crate::ItemForeignMod`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemForeignMod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemForeignMod`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemForeignMod`

##### `impl<T> Spanned for ItemForeignMod`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemForeignMod`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemImpl`

```rust
struct ItemImpl {
    pub attrs: Vec<crate::attr::Attribute>,
    pub defaultness: Option<$crate::token::Default>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub impl_token: $crate::token::Impl,
    pub generics: crate::generics::Generics,
    pub trait_: Option<(Option<$crate::token::Not>, crate::path::Path, $crate::token::For)>,
    pub self_ty: Box<crate::ty::Type>,
    pub brace_token: token::Brace,
    pub items: Vec<ImplItem>,
}
```

An impl block providing trait or associated items: `impl<A> Trait
for Data<A> { ... }`.

#### Fields

- **`trait_`**: `Option<(Option<$crate::token::Not>, crate::path::Path, $crate::token::For)>`

  Trait this impl implements.

- **`self_ty`**: `Box<crate::ty::Type>`

  The Self type of the impl.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemImpl`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemImpl`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemImpl`

##### `impl Hash for crate::ItemImpl`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemImpl`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemImpl`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemImpl`

##### `impl<T> Spanned for ItemImpl`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemImpl`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemMacro`

```rust
struct ItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: Option<crate::ident::Ident>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation, which includes `macro_rules!` definitions.

#### Fields

- **`ident`**: `Option<crate::ident::Ident>`

  The `example` in `macro_rules! example { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMacro`

##### `impl Hash for crate::ItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMacro`

##### `impl<T> Spanned for ItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemMod`

```rust
struct ItemMod {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub mod_token: $crate::token::Mod,
    pub ident: crate::ident::Ident,
    pub content: Option<(token::Brace, Vec<Item>)>,
    pub semi: Option<$crate::token::Semi>,
}
```

A module or module declaration: `mod m` or `mod m { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemMod`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemMod`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemMod`

##### `impl Hash for crate::ItemMod`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemMod`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemMod`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemMod`

##### `impl<T> Spanned for ItemMod`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemMod`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemStatic`

```rust
struct ItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: $crate::token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub semi_token: $crate::token::Semi,
}
```

A static item: `static BIKE: Shed = Shed(42)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStatic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemStatic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStatic`

##### `impl Hash for crate::ItemStatic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemStatic`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemStatic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStatic`

##### `impl<T> Spanned for ItemStatic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemStatic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemStruct`

```rust
struct ItemStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub struct_token: $crate::token::Struct,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::Fields,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A struct definition: `struct Foo<A> { x: A }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemStruct`

##### `impl Hash for crate::ItemStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemStruct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemStruct`

##### `impl<T> Spanned for ItemStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemTrait`

```rust
struct ItemTrait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub auto_token: Option<$crate::token::Auto>,
    pub restriction: Option<ImplRestriction>,
    pub trait_token: $crate::token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<$crate::token::Colon>,
    pub supertraits: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub brace_token: token::Brace,
    pub items: Vec<TraitItem>,
}
```

A trait definition: `pub trait Iterator { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTrait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemTrait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTrait`

##### `impl Hash for crate::ItemTrait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemTrait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemTrait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTrait`

##### `impl<T> Spanned for ItemTrait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemTrait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemTraitAlias`

```rust
struct ItemTraitAlias {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub trait_token: $crate::token::Trait,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub semi_token: $crate::token::Semi,
}
```

A trait alias: `pub trait SharableIterator = Iterator + Sync`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemTraitAlias`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemTraitAlias`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemTraitAlias`

##### `impl Hash for crate::ItemTraitAlias`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemTraitAlias`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemTraitAlias`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemTraitAlias`

##### `impl<T> Spanned for ItemTraitAlias`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemTraitAlias`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemType`

```rust
struct ItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: $crate::token::Semi,
}
```

A type alias: `type Result<T> = std::result::Result<T, MyError>`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemType`

##### `impl Hash for crate::ItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemType`

##### `impl<T> Spanned for ItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemUnion`

```rust
struct ItemUnion {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub union_token: $crate::token::Union,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub fields: crate::data::FieldsNamed,
}
```

A union definition: `union Foo<A, B> { x: A, y: B }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUnion`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemUnion`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUnion`

##### `impl Hash for crate::ItemUnion`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemUnion`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemUnion`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUnion`

##### `impl<T> Spanned for ItemUnion`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemUnion`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ItemUse`

```rust
struct ItemUse {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub use_token: $crate::token::Use,
    pub leading_colon: Option<$crate::token::PathSep>,
    pub tree: UseTree,
    pub semi_token: $crate::token::Semi,
}
```

A use declaration: `use std::collections::HashMap`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ItemUse`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ItemUse`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ItemUse`

##### `impl Hash for crate::ItemUse`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ItemUse`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ItemUse`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ItemUse`

##### `impl<T> Spanned for ItemUse`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ItemUse`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UsePath`

```rust
struct UsePath {
    pub ident: crate::ident::Ident,
    pub colon2_token: $crate::token::PathSep,
    pub tree: Box<UseTree>,
}
```

A path prefix of imports in a `use` item: `std::...`.

#### Trait Implementations

##### `impl Clone for crate::UsePath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UsePath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UsePath`

##### `impl Hash for crate::UsePath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UsePath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UsePath`

##### `impl<T> Spanned for UsePath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UsePath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseName`

```rust
struct UseName {
    pub ident: crate::ident::Ident,
}
```

An identifier imported by a `use` item: `HashMap`.

#### Trait Implementations

##### `impl Clone for crate::UseName`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseName`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseName`

##### `impl Hash for crate::UseName`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseName`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseName`

##### `impl<T> Spanned for UseName`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseName`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseRename`

```rust
struct UseRename {
    pub ident: crate::ident::Ident,
    pub as_token: $crate::token::As,
    pub rename: crate::ident::Ident,
}
```

An renamed identifier imported by a `use` item: `HashMap as Map`.

#### Trait Implementations

##### `impl Clone for crate::UseRename`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseRename`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseRename`

##### `impl Hash for crate::UseRename`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseRename`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseRename`

##### `impl<T> Spanned for UseRename`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseRename`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseGlob`

```rust
struct UseGlob {
    pub star_token: $crate::token::Star,
}
```

A glob import in a `use` item: `*`.

#### Trait Implementations

##### `impl Clone for crate::UseGlob`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseGlob`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGlob`

##### `impl Hash for crate::UseGlob`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl PartialEq for crate::UseGlob`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for UseGlob`

##### `impl<T> Spanned for UseGlob`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseGlob`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UseGroup`

```rust
struct UseGroup {
    pub brace_token: token::Brace,
    pub items: crate::punctuated::Punctuated<UseTree, $crate::token::Comma>,
}
```

A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseGroup`

##### `impl Hash for crate::UseGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::UseGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseGroup`

##### `impl<T> Spanned for UseGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::UseGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ForeignItemFn`

```rust
struct ForeignItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub sig: Signature,
    pub semi_token: $crate::token::Semi,
}
```

A foreign function in an `extern` block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemFn`

##### `impl Hash for crate::ForeignItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ForeignItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemFn`

##### `impl<T> Spanned for ForeignItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ForeignItemStatic`

```rust
struct ForeignItemStatic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub static_token: $crate::token::Static,
    pub mutability: StaticMutability,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
    pub semi_token: $crate::token::Semi,
}
```

A foreign static item in an `extern` block: `static ext: u8`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemStatic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemStatic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemStatic`

##### `impl Hash for crate::ForeignItemStatic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemStatic`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ForeignItemStatic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemStatic`

##### `impl<T> Spanned for ForeignItemStatic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemStatic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ForeignItemType`

```rust
struct ForeignItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub semi_token: $crate::token::Semi,
}
```

A foreign type in an `extern` block: `type void`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemType`

##### `impl Hash for crate::ForeignItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ForeignItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemType`

##### `impl<T> Spanned for ForeignItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ForeignItemMacro`

```rust
struct ForeignItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an extern block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ForeignItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItemMacro`

##### `impl Hash for crate::ForeignItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ForeignItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItemMacro`

##### `impl<T> Spanned for ForeignItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ForeignItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitItemConst`

```rust
struct TraitItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub default: Option<($crate::token::Eq, crate::expr::Expr)>,
    pub semi_token: $crate::token::Semi,
}
```

An associated constant within the definition of a trait.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemConst`

##### `impl Hash for crate::TraitItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::TraitItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemConst`

##### `impl<T> Spanned for TraitItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitItemFn`

```rust
struct TraitItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub sig: Signature,
    pub default: Option<crate::stmt::Block>,
    pub semi_token: Option<$crate::token::Semi>,
}
```

An associated function within the definition of a trait.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemFn`

##### `impl Hash for crate::TraitItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::TraitItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemFn`

##### `impl<T> Spanned for TraitItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitItemType`

```rust
struct TraitItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
    pub default: Option<($crate::token::Eq, crate::ty::Type)>,
    pub semi_token: $crate::token::Semi,
}
```

An associated type within the definition of a trait.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemType`

##### `impl Hash for crate::TraitItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::TraitItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemType`

##### `impl<T> Spanned for TraitItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitItemMacro`

```rust
struct TraitItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within the definition of a trait.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TraitItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItemMacro`

##### `impl Hash for crate::TraitItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::TraitItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItemMacro`

##### `impl<T> Spanned for TraitItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::TraitItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplItemConst`

```rust
struct ImplItemConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: $crate::token::Eq,
    pub expr: crate::expr::Expr,
    pub semi_token: $crate::token::Semi,
}
```

An associated constant within an impl block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemConst`

##### `impl Hash for crate::ImplItemConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ImplItemConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemConst`

##### `impl<T> Spanned for ImplItemConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplItemFn`

```rust
struct ImplItemFn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub sig: Signature,
    pub block: crate::stmt::Block,
}
```

An associated function within an impl block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemFn`

##### `impl Hash for crate::ImplItemFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ImplItemFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemFn`

##### `impl<T> Spanned for ImplItemFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplItemType`

```rust
struct ImplItemType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub vis: crate::restriction::Visibility,
    pub defaultness: Option<$crate::token::Default>,
    pub type_token: $crate::token::Type,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub eq_token: $crate::token::Eq,
    pub ty: crate::ty::Type,
    pub semi_token: $crate::token::Semi,
}
```

An associated type within an impl block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemType`

##### `impl Hash for crate::ImplItemType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ImplItemType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemType`

##### `impl<T> Spanned for ImplItemType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplItemMacro`

```rust
struct ImplItemMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation within an impl block.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ImplItemMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItemMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItemMacro`

##### `impl Hash for crate::ImplItemMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItemMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ImplItemMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItemMacro`

##### `impl<T> Spanned for ImplItemMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::ImplItemMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Signature`

```rust
struct Signature {
    pub constness: Option<$crate::token::Const>,
    pub asyncness: Option<$crate::token::Async>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: Option<crate::ty::Abi>,
    pub fn_token: $crate::token::Fn,
    pub ident: crate::ident::Ident,
    pub generics: crate::generics::Generics,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<FnArg, $crate::token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: crate::ty::ReturnType,
}
```

A function signature in a trait or implementation: `unsafe fn
initialize(&self)`.

#### Implementations

- `fn receiver(self: &Self) -> Option<&Receiver>` — [`Receiver`](#receiver)

#### Trait Implementations

##### `impl Clone for crate::Signature`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Signature`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Signature`

##### `impl Hash for crate::Signature`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Signature`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Signature`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Signature`

##### `impl<T> Spanned for Signature`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Signature`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Receiver`

```rust
struct Receiver {
    pub attrs: Vec<crate::attr::Attribute>,
    pub reference: Option<($crate::token::And, Option<crate::lifetime::Lifetime>)>,
    pub mutability: Option<$crate::token::Mut>,
    pub self_token: $crate::token::SelfValue,
    pub colon_token: Option<$crate::token::Colon>,
    pub ty: Box<crate::ty::Type>,
}
```

The `self` argument of an associated method.

If `colon_token` is present, the receiver is written with an explicit
type such as `self: Box<Self>`. If `colon_token` is absent, the receiver
is written in shorthand such as `self` or `&self` or `&mut self`. In the
shorthand case, the type in `ty` is reconstructed as one of `Self`,
`&Self`, or `&mut Self`.

#### Implementations

- `fn lifetime(self: &Self) -> Option<&Lifetime>` — [`Lifetime`](../lifetime/index.md)

#### Trait Implementations

##### `impl Clone for crate::Receiver`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Receiver`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Receiver`

##### `impl Hash for crate::Receiver`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Receiver`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Receiver`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Receiver`

##### `impl<T> Spanned for Receiver`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Receiver`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Variadic`

```rust
struct Variadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Option<(Box<crate::pat::Pat>, $crate::token::Colon)>,
    pub dots: $crate::token::DotDotDot,
    pub comma: Option<$crate::token::Comma>,
}
```

The variadic argument of a foreign function.

```rust
struct c_char;
struct c_int;

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    //                               ^^^
}
```

#### Trait Implementations

##### `impl Clone for crate::Variadic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Variadic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Variadic`

##### `impl Hash for crate::Variadic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Variadic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Variadic`

##### `impl<T> Spanned for Variadic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::Variadic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `Item`

```rust
enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(proc_macro2::TokenStream),
}
```

Things that can appear directly inside of a module or scope.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A constant item: `const MAX: u16 = 65535`.

- **`Enum`**

  An enum definition: `enum Foo<A, B> { A(A), B(B) }`.

- **`ExternCrate`**

  An `extern crate` item: `extern crate serde`.

- **`Fn`**

  A free-standing function: `fn process(n: usize) -> Result<()> { ...
  }`.

- **`ForeignMod`**

  A block of foreign items: `extern "C" { ... }`.

- **`Impl`**

  An impl block providing trait or associated items: `impl<A> Trait
  for Data<A> { ... }`.

- **`Macro`**

  A macro invocation, which includes `macro_rules!` definitions.

- **`Mod`**

  A module or module declaration: `mod m` or `mod m { ... }`.

- **`Static`**

  A static item: `static BIKE: Shed = Shed(42)`.

- **`Struct`**

  A struct definition: `struct Foo<A> { x: A }`.

- **`Trait`**

  A trait definition: `pub trait Iterator { ... }`.

- **`TraitAlias`**

  A trait alias: `pub trait SharableIterator = Iterator + Sync`.

- **`Type`**

  A type alias: `type Result<T> = std::result::Result<T, MyError>`.

- **`Union`**

  A union definition: `union Foo<A, B> { x: A, y: B }`.

- **`Use`**

  A use declaration: `use std::collections::HashMap`.

- **`Verbatim`**

  Tokens forming an item not interpreted by Syn.

#### Implementations

- `fn replace_attrs(self: &mut Self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md)

#### Trait Implementations

##### `impl Clone for crate::Item`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Item`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Item`

##### `impl Hash for crate::Item`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::Item`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Item`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Item`

##### `impl<T> Spanned for Item`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Item`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `UseTree`

```rust
enum UseTree {
    Path(UsePath),
    Name(UseName),
    Rename(UseRename),
    Glob(UseGlob),
    Group(UseGroup),
}
```

A suffix of an import tree in a `use` item: `Type as Renamed` or `*`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Path`**

  A path prefix of imports in a `use` item: `std::...`.

- **`Name`**

  An identifier imported by a `use` item: `HashMap`.

- **`Rename`**

  An renamed identifier imported by a `use` item: `HashMap as Map`.

- **`Glob`**

  A glob import in a `use` item: `*`.

- **`Group`**

  A braced group of imports in a `use` item: `{A, B, C}`.

#### Trait Implementations

##### `impl Clone for crate::UseTree`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::UseTree`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UseTree`

##### `impl Hash for crate::UseTree`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::UseTree`

- `fn parse(input: ParseStream<'_>) -> Result<UseTree>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md), [`UseTree`](#usetree)

##### `impl PartialEq for crate::UseTree`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UseTree`

##### `impl<T> Spanned for UseTree`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for UseTree`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `ForeignItem`

```rust
enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an `extern` block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Fn`**

  A foreign function in an `extern` block.

- **`Static`**

  A foreign static item in an `extern` block: `static ext: u8`.

- **`Type`**

  A foreign type in an `extern` block: `type void`.

- **`Macro`**

  A macro invocation within an extern block.

- **`Verbatim`**

  Tokens in an `extern` block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ForeignItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ForeignItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ForeignItem`

##### `impl Hash for crate::ForeignItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ForeignItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ForeignItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ForeignItem`

##### `impl<T> Spanned for ForeignItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ForeignItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `TraitItem`

```rust
enum TraitItem {
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item declaration within the definition of a trait.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within the definition of a trait.

- **`Fn`**

  An associated function within the definition of a trait.

- **`Type`**

  An associated type within the definition of a trait.

- **`Macro`**

  A macro invocation within the definition of a trait.

- **`Verbatim`**

  Tokens within the definition of a trait not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::TraitItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitItem`

##### `impl Hash for crate::TraitItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::TraitItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::TraitItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitItem`

##### `impl<T> Spanned for TraitItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for TraitItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `ImplItem`

```rust
enum ImplItem {
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(proc_macro2::TokenStream),
}
```

An item within an impl block.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  An associated constant within an impl block.

- **`Fn`**

  An associated function within an impl block.

- **`Type`**

  An associated type within an impl block.

- **`Macro`**

  A macro invocation within an impl block.

- **`Verbatim`**

  Tokens within an impl block not interpreted by Syn.

#### Trait Implementations

##### `impl Clone for crate::ImplItem`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplItem`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplItem`

##### `impl Hash for crate::ImplItem`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::ImplItem`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ImplItem`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplItem`

##### `impl<T> Spanned for ImplItem`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for ImplItem`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `FnArg`

```rust
enum FnArg {
    Receiver(Receiver),
    Typed(crate::pat::PatType),
}
```

An argument in a function signature: the `n: usize` in `fn f(n: usize)`.

#### Variants

- **`Receiver`**

  The `self` argument of an associated method.

- **`Typed`**

  A function argument accepted by pattern and type.

#### Trait Implementations

##### `impl Clone for crate::FnArg`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FnArg`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FnArg`

##### `impl Hash for crate::FnArg`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::FnArg`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::FnArg`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FnArg`

##### `impl<T> Spanned for FnArg`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for FnArg`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `StaticMutability`

```rust
enum StaticMutability {
    Mut($crate::token::Mut),
    None,
}
```

The mutability of an `Item::Static` or `ForeignItem::Static`.

#### Trait Implementations

##### `impl Clone for crate::StaticMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::StaticMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StaticMutability`

##### `impl Hash for crate::StaticMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::item::StaticMutability`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::StaticMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for StaticMutability`

##### `impl<T> Spanned for StaticMutability`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::item::StaticMutability`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ImplRestriction`

```rust
enum ImplRestriction {
}
```

Unused, but reserved for RFC 3323 restrictions.

#### Trait Implementations

##### `impl Clone for crate::ImplRestriction`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ImplRestriction`

- `fn fmt(self: &Self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ImplRestriction`

##### `impl Hash for crate::ImplRestriction`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl PartialEq for crate::ImplRestriction`

- `fn eq(self: &Self, _other: &Self) -> bool`

