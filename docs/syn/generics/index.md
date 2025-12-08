*[syn](../index.md) / [generics](index.md)*

---

# Module `generics`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Generics`

```rust
struct Generics {
    pub lt_token: Option<$crate::token::Lt>,
    pub params: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>,
    pub gt_token: Option<$crate::token::Gt>,
    pub where_clause: Option<WhereClause>,
}
```

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.



#### Implementations

- `fn lifetimes(self: &Self) -> Lifetimes<'_>` — [`Lifetimes`](#lifetimes)

- `fn lifetimes_mut(self: &mut Self) -> LifetimesMut<'_>` — [`LifetimesMut`](#lifetimesmut)

- `fn type_params(self: &Self) -> TypeParams<'_>` — [`TypeParams`](#typeparams)

- `fn type_params_mut(self: &mut Self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](#typeparamsmut)

- `fn const_params(self: &Self) -> ConstParams<'_>` — [`ConstParams`](#constparams)

- `fn const_params_mut(self: &mut Self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](#constparamsmut)

- `fn make_where_clause(self: &mut Self) -> &mut WhereClause` — [`WhereClause`](../index.md)

- `fn split_for_impl(self: &Self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](../index.md), [`TypeGenerics`](../index.md), [`WhereClause`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Generics`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Generics`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- `fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl Hash for crate::Generics`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::Generics`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Generics`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Generics`

##### `impl<T> Spanned for Generics`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::Generics`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>,
}
```

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- `fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::LifetimeParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::LifetimeParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl Hash for crate::LifetimeParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::LifetimeParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::LifetimeParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for LifetimeParam`

##### `impl<T> Spanned for LifetimeParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::LifetimeParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeParam`

```rust
struct TypeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub colon_token: Option<$crate::token::Colon>,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>,
    pub eq_token: Option<$crate::token::Eq>,
    pub default: Option<crate::ty::Type>,
}
```

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl Clone for crate::TypeParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl Hash for crate::TypeParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParam`

##### `impl<T> Spanned for TypeParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TypeParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ConstParam`

```rust
struct ConstParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub ident: crate::ident::Ident,
    pub colon_token: $crate::token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: Option<$crate::token::Eq>,
    pub default: Option<crate::expr::Expr>,
}
```

A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::ConstParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ConstParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl Hash for crate::ConstParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::ConstParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::ConstParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ConstParam`

##### `impl<T> Spanned for ConstParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::ConstParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Lifetimes<'a>`

```rust
struct Lifetimes<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for Lifetimes<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for Lifetimes<'a>`

- `type Item = &'a LifetimeParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `LifetimesMut<'a>`

```rust
struct LifetimesMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for LifetimesMut<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for LifetimesMut<'a>`

- `type Item = &'a mut LifetimeParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `TypeParams<'a>`

```rust
struct TypeParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for TypeParams<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for TypeParams<'a>`

- `type Item = &'a TypeParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `TypeParamsMut<'a>`

```rust
struct TypeParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for TypeParamsMut<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for TypeParamsMut<'a>`

- `type Item = &'a mut TypeParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ConstParams<'a>`

```rust
struct ConstParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for ConstParams<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ConstParams<'a>`

- `type Item = &'a ConstParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ConstParamsMut<'a>`

```rust
struct ConstParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

#### Trait Implementations

##### `impl<I> IntoIterator for ConstParamsMut<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ConstParamsMut<'a>`

- `type Item = &'a mut ConstParam`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl<'a> Clone for ImplGenerics<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for ImplGenerics<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for ImplGenerics<'a>`

##### `impl<'a> Hash for ImplGenerics<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for ImplGenerics<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ImplGenerics<'a>`

##### `impl<T> Spanned for ImplGenerics<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::ImplGenerics<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

Returned by `Generics::split_for_impl`.

#### Implementations

- `fn as_turbofish(self: &Self) -> Turbofish<'a>` — [`Turbofish`](../index.md)

#### Trait Implementations

##### `impl<'a> Clone for TypeGenerics<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for TypeGenerics<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for TypeGenerics<'a>`

##### `impl<'a> Hash for TypeGenerics<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for TypeGenerics<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGenerics<'a>`

##### `impl<T> Spanned for TypeGenerics<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::TypeGenerics<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl<'a> Clone for Turbofish<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Debug for Turbofish<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Eq for Turbofish<'a>`

##### `impl<'a> Hash for Turbofish<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for Turbofish<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Turbofish<'a>`

##### `impl<T> Spanned for Turbofish<'a>`

- `fn span(self: &Self) -> Span`

##### `impl<'a> ToTokens for crate::generics::Turbofish<'a>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: $crate::token::For,
    pub lt_token: $crate::token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

A set of bound lifetimes: `for<'a, 'b, 'c>`.

#### Trait Implementations

##### `impl Clone for crate::BoundLifetimes`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BoundLifetimes`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- `fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl Hash for crate::BoundLifetimes`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::BoundLifetimes`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::BoundLifetimes`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BoundLifetimes`

##### `impl<T> Spanned for BoundLifetimes`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

#### Implementations

- `fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::TraitBound`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TraitBound`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl Hash for crate::TraitBound`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TraitBound`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBound`

##### `impl<T> Spanned for TraitBound`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TraitBound`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: $crate::token::Use,
    pub lt_token: $crate::token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

#### Trait Implementations

##### `impl Clone for crate::PreciseCapture`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PreciseCapture`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl Hash for crate::PreciseCapture`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::PreciseCapture`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::PreciseCapture`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PreciseCapture`

##### `impl<T> Spanned for PreciseCapture`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PreciseCapture`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: $crate::token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, $crate::token::Comma>,
}
```

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

#### Trait Implementations

##### `impl Clone for crate::WhereClause`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::WhereClause`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl Hash for crate::WhereClause`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::WhereClause`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::WhereClause`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for WhereClause`

##### `impl<T> Spanned for WhereClause`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::WhereClause`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, $crate::token::Plus>,
}
```

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

#### Trait Implementations

##### `impl Clone for crate::PredicateLifetime`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PredicateLifetime`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl Hash for crate::PredicateLifetime`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PredicateLifetime`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateLifetime`

##### `impl<T> Spanned for PredicateLifetime`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>,
}
```

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, $crate::token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

#### Trait Implementations

##### `impl Clone for crate::PredicateType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PredicateType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl Hash for crate::PredicateType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PredicateType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PredicateType`

##### `impl<T> Spanned for PredicateType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::PredicateType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

A generic type parameter, lifetime, or const generic: `T: Into<String>`,
`'a: 'b`, `const LEN: usize`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime parameter: `'a: 'b + 'c + 'd`.

- **`Type`**

  A generic type parameter: `T: Into<String>`.

- **`Const`**

  A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::GenericParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::GenericParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl Hash for crate::GenericParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::GenericParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::GenericParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericParam`

##### `impl<T> Spanned for GenericParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for GenericParam`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

A trait or lifetime used as a bound on a type parameter.

#### Implementations

- `fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, $crate::token::Plus>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md), [`Punctuated`](../punctuated/index.md), [`Plus`](../token/index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeParamBound`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParamBound`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl Hash for crate::TypeParamBound`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParamBound`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeParamBound`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParamBound`

##### `impl<T> Spanned for TypeParamBound`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for TypeParamBound`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe($crate::token::Question),
}
```

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

#### Trait Implementations

##### `impl Clone for crate::TraitBoundModifier`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl Hash for crate::TraitBoundModifier`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBoundModifier`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TraitBoundModifier`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TraitBoundModifier`

##### `impl<T> Spanned for TraitBoundModifier`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

Single parameter in a precise capturing bound.

#### Variants

- **`Lifetime`**

  A lifetime parameter in precise capturing bound: `fn f<'a>() -> impl
  Trait + use<'a>`.

- **`Ident`**

  A type parameter or const generic parameter in precise capturing
  bound: `fn f<T>() -> impl Trait + use<T>` or `fn f<const K: T>() ->
  impl Trait + use<K>`.

#### Trait Implementations

##### `impl Clone for crate::CapturedParam`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::CapturedParam`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl Hash for crate::CapturedParam`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::CapturedParam`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::CapturedParam`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for CapturedParam`

##### `impl<T> Spanned for CapturedParam`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::generics::CapturedParam`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl Clone for crate::WherePredicate`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::WherePredicate`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl Hash for crate::WherePredicate`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::generics::WherePredicate`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::WherePredicate`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for WherePredicate`

##### `impl<T> Spanned for WherePredicate`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for WherePredicate`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

## Macros

### `generics_wrapper_impls!`

