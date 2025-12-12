*[syn](../index.md) / [generics](index.md)*

---

# Module `generics`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Generics`](#generics)
  - [`LifetimeParam`](#lifetimeparam)
  - [`TypeParam`](#typeparam)
  - [`ConstParam`](#constparam)
  - [`Lifetimes`](#lifetimes)
  - [`LifetimesMut`](#lifetimesmut)
  - [`TypeParams`](#typeparams)
  - [`TypeParamsMut`](#typeparamsmut)
  - [`ConstParams`](#constparams)
  - [`ConstParamsMut`](#constparamsmut)
  - [`ImplGenerics`](#implgenerics)
  - [`TypeGenerics`](#typegenerics)
  - [`Turbofish`](#turbofish)
  - [`BoundLifetimes`](#boundlifetimes)
  - [`TraitBound`](#traitbound)
  - [`PreciseCapture`](#precisecapture)
  - [`WhereClause`](#whereclause)
  - [`PredicateLifetime`](#predicatelifetime)
  - [`PredicateType`](#predicatetype)
- [Enums](#enums)
  - [`GenericParam`](#genericparam)
  - [`TypeParamBound`](#typeparambound)
  - [`TraitBoundModifier`](#traitboundmodifier)
  - [`CapturedParam`](#capturedparam)
  - [`WherePredicate`](#wherepredicate)
- [Macros](#macros)
  - [`generics_wrapper_impls!`](#generics-wrapper-impls)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Generics`](#generics) | struct | Lifetimes and type parameters attached to a declaration of a function, enum, trait, etc. |
| [`LifetimeParam`](#lifetimeparam) | struct | A lifetime definition: `'a: 'b + 'c + 'd`. |
| [`TypeParam`](#typeparam) | struct | A generic type parameter: `T: Into<String>`. |
| [`ConstParam`](#constparam) | struct | A const generic parameter: `const LENGTH: usize`. |
| [`Lifetimes`](#lifetimes) | struct |  |
| [`LifetimesMut`](#lifetimesmut) | struct |  |
| [`TypeParams`](#typeparams) | struct |  |
| [`TypeParamsMut`](#typeparamsmut) | struct |  |
| [`ConstParams`](#constparams) | struct |  |
| [`ConstParamsMut`](#constparamsmut) | struct |  |
| [`ImplGenerics`](#implgenerics) | struct | Returned by `Generics::split_for_impl`. |
| [`TypeGenerics`](#typegenerics) | struct | Returned by `Generics::split_for_impl`. |
| [`Turbofish`](#turbofish) | struct | Returned by `TypeGenerics::as_turbofish`. |
| [`BoundLifetimes`](#boundlifetimes) | struct | A set of bound lifetimes: `for<'a, 'b, 'c>`. |
| [`TraitBound`](#traitbound) | struct | A trait used as a bound on a type parameter. |
| [`PreciseCapture`](#precisecapture) | struct | Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait + use<'a, T>`. |
| [`WhereClause`](#whereclause) | struct | A `where` clause in a definition: `where T: Deserialize<'de>, D: 'static`. |
| [`PredicateLifetime`](#predicatelifetime) | struct | A lifetime predicate in a `where` clause: `'a: 'b + 'c`. |
| [`PredicateType`](#predicatetype) | struct | A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`. |
| [`GenericParam`](#genericparam) | enum | A generic type parameter, lifetime, or const generic: `T: Into<String>`, `'a: 'b`, `const LEN: usize`. |
| [`TypeParamBound`](#typeparambound) | enum | A trait or lifetime used as a bound on a type parameter. |
| [`TraitBoundModifier`](#traitboundmodifier) | enum | A modifier on a trait bound, currently only used for the `?` in `?Sized`. |
| [`CapturedParam`](#capturedparam) | enum | Single parameter in a precise capturing bound. |
| [`WherePredicate`](#wherepredicate) | enum | A single predicate in a `where` clause: `T: Deserialize<'de>`. |
| [`generics_wrapper_impls!`](#generics-wrapper-impls) | macro |  |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Generics`

```rust
struct Generics {
    pub lt_token: Option<token::Lt>,
    pub params: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: Option<token::Gt>,
    pub where_clause: Option<WhereClause>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:15-32`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L15-L32)*

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.



#### Implementations

- <span id="generics-lifetimes"></span>`fn lifetimes(&self) -> Lifetimes<'_>` — [`Lifetimes`](#lifetimes)

- <span id="generics-lifetimes-mut"></span>`fn lifetimes_mut(&mut self) -> LifetimesMut<'_>` — [`LifetimesMut`](#lifetimesmut)

- <span id="generics-type-params"></span>`fn type_params(&self) -> TypeParams<'_>` — [`TypeParams`](#typeparams)

- <span id="generics-type-params-mut"></span>`fn type_params_mut(&mut self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](#typeparamsmut)

- <span id="generics-const-params"></span>`fn const_params(&self) -> ConstParams<'_>` — [`ConstParams`](#constparams)

- <span id="generics-const-params-mut"></span>`fn const_params_mut(&mut self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](#constparamsmut)

- <span id="generics-make-where-clause"></span>`fn make_where_clause(&mut self) -> &mut WhereClause` — [`WhereClause`](#whereclause)

- <span id="generics-split-for-impl"></span>`fn split_for_impl(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](#implgenerics), [`TypeGenerics`](#typegenerics), [`WhereClause`](#whereclause)

#### Trait Implementations

##### `impl Clone for crate::Generics`

- <span id="crategenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Generics`

- <span id="crategenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- <span id="generics-default"></span>`fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl Hash for crate::Generics`

- <span id="crategenerics-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::Generics`

- <span id="crategenericsgenerics-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Generics`

- <span id="crategenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Generics`

##### `impl Spanned for Generics`

- <span id="generics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::Generics`

- <span id="crategenericsgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:56-65`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L56-L65)*

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- <span id="lifetimeparam-new"></span>`fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](../lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Clone for crate::LifetimeParam`

- <span id="cratelifetimeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::LifetimeParam`

- <span id="cratelifetimeparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl Hash for crate::LifetimeParam`

- <span id="cratelifetimeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::LifetimeParam`

- <span id="cratelifetimeparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LifetimeParam`

##### `impl Spanned for LifetimeParam`

- <span id="lifetimeparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeParam`

```rust
struct TypeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub ident: crate::ident::Ident,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
    pub eq_token: Option<token::Eq>,
    pub default: Option<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:67-78`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L67-L78)*

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl Clone for crate::TypeParam`

- <span id="cratetypeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParam`

- <span id="cratetypeparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl Hash for crate::TypeParam`

- <span id="cratetypeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParam`

- <span id="crategenericstypeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeParam`

- <span id="cratetypeparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParam`

##### `impl Spanned for TypeParam`

- <span id="typeparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TypeParam`

- <span id="crategenericstypeparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ConstParam`

```rust
struct ConstParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub ident: crate::ident::Ident,
    pub colon_token: token::Colon,
    pub ty: crate::ty::Type,
    pub eq_token: Option<token::Eq>,
    pub default: Option<crate::expr::Expr>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:80-92`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L80-L92)*

A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Clone for crate::ConstParam`

- <span id="crateconstparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ConstParam`

- <span id="crateconstparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl Hash for crate::ConstParam`

- <span id="crateconstparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::ConstParam`

- <span id="crategenericsconstparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ConstParam`

- <span id="crateconstparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ConstParam`

##### `impl Spanned for ConstParam`

- <span id="constparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::ConstParam`

- <span id="crategenericsconstparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Lifetimes<'a>`

```rust
struct Lifetimes<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:185`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L185)*

#### Trait Implementations

##### `impl IntoIterator for Lifetimes<'a>`

- <span id="lifetimes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="lifetimes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="lifetimes-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Lifetimes<'a>`

- <span id="lifetimes-iterator-type-item"></span>`type Item = &'a LifetimeParam`

- <span id="lifetimes-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LifetimesMut<'a>`

```rust
struct LifetimesMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:199`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L199)*

#### Trait Implementations

##### `impl IntoIterator for LifetimesMut<'a>`

- <span id="lifetimesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="lifetimesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="lifetimesmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LifetimesMut<'a>`

- <span id="lifetimesmut-iterator-type-item"></span>`type Item = &'a mut LifetimeParam`

- <span id="lifetimesmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `TypeParams<'a>`

```rust
struct TypeParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:213`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L213)*

#### Trait Implementations

##### `impl IntoIterator for TypeParams<'a>`

- <span id="typeparams-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="typeparams-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="typeparams-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TypeParams<'a>`

- <span id="typeparams-iterator-type-item"></span>`type Item = &'a TypeParam`

- <span id="typeparams-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `TypeParamsMut<'a>`

```rust
struct TypeParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:227`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L227)*

#### Trait Implementations

##### `impl IntoIterator for TypeParamsMut<'a>`

- <span id="typeparamsmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="typeparamsmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="typeparamsmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TypeParamsMut<'a>`

- <span id="typeparamsmut-iterator-type-item"></span>`type Item = &'a mut TypeParam`

- <span id="typeparamsmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ConstParams<'a>`

```rust
struct ConstParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:241`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L241)*

#### Trait Implementations

##### `impl IntoIterator for ConstParams<'a>`

- <span id="constparams-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="constparams-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="constparams-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ConstParams<'a>`

- <span id="constparams-iterator-type-item"></span>`type Item = &'a ConstParam`

- <span id="constparams-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ConstParamsMut<'a>`

```rust
struct ConstParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:255`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L255)*

#### Trait Implementations

##### `impl IntoIterator for ConstParamsMut<'a>`

- <span id="constparamsmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="constparamsmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="constparamsmut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ConstParamsMut<'a>`

- <span id="constparamsmut-iterator-type-item"></span>`type Item = &'a mut ConstParam`

- <span id="constparamsmut-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:275`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L275)*

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl Clone for ImplGenerics<'a>`

- <span id="implgenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for ImplGenerics<'a>`

- <span id="implgenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImplGenerics<'a>`

##### `impl Hash for ImplGenerics<'a>`

- <span id="implgenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for ImplGenerics<'a>`

- <span id="implgenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplGenerics<'a>`

##### `impl Spanned for ImplGenerics<'a>`

- <span id="implgenerics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::ImplGenerics<'a>`

- <span id="crategenericsimplgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:283`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L283)*

Returned by `Generics::split_for_impl`.

#### Implementations

- <span id="typegenerics-as-turbofish"></span>`fn as_turbofish(&self) -> Turbofish<'a>` — [`Turbofish`](#turbofish)

#### Trait Implementations

##### `impl Clone for TypeGenerics<'a>`

- <span id="typegenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for TypeGenerics<'a>`

- <span id="typegenerics-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeGenerics<'a>`

##### `impl Hash for TypeGenerics<'a>`

- <span id="typegenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for TypeGenerics<'a>`

- <span id="typegenerics-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeGenerics<'a>`

##### `impl Spanned for TypeGenerics<'a>`

- <span id="typegenerics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TypeGenerics<'a>`

- <span id="crategenericstypegenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:291`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L291)*

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl Clone for Turbofish<'a>`

- <span id="turbofish-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Turbofish<'a>`

- <span id="turbofish-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Turbofish<'a>`

##### `impl Hash for Turbofish<'a>`

- <span id="turbofish-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl PartialEq for Turbofish<'a>`

- <span id="turbofish-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Turbofish<'a>`

##### `impl Spanned for Turbofish<'a>`

- <span id="turbofish-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::Turbofish<'a>`

- <span id="crategenericsturbofish-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: token::For,
    pub lt_token: token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:352-361`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L352-L361)*

A set of bound lifetimes: `for<'a, 'b, 'c>`.

#### Trait Implementations

##### `impl Clone for crate::BoundLifetimes`

- <span id="crateboundlifetimes-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BoundLifetimes`

- <span id="crateboundlifetimes-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- <span id="boundlifetimes-default"></span>`fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl Hash for crate::BoundLifetimes`

- <span id="crateboundlifetimes-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::BoundLifetimes`

- <span id="crateboundlifetimes-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BoundLifetimes`

##### `impl Spanned for BoundLifetimes`

- <span id="boundlifetimes-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:410-421`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L410-L421)*

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

#### Implementations

- <span id="crategenericstraitbound-do-parse"></span>`fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Clone for crate::TraitBound`

- <span id="cratetraitbound-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TraitBound`

- <span id="cratetraitbound-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl Hash for crate::TraitBound`

- <span id="cratetraitbound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBound`

- <span id="crategenericstraitbound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitBound`

- <span id="cratetraitbound-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBound`

##### `impl Spanned for TraitBound`

- <span id="traitbound-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TraitBound`

- <span id="crategenericstraitbound-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: token::Use,
    pub lt_token: token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:433-443`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L433-L443)*

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

#### Trait Implementations

##### `impl Clone for crate::PreciseCapture`

- <span id="crateprecisecapture-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PreciseCapture`

- <span id="crateprecisecapture-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl Hash for crate::PreciseCapture`

- <span id="crateprecisecapture-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PreciseCapture`

- <span id="crateprecisecapture-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PreciseCapture`

##### `impl Spanned for PreciseCapture`

- <span id="precisecapture-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:461-469`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L461-L469)*

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

#### Trait Implementations

##### `impl Clone for crate::WhereClause`

- <span id="cratewhereclause-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::WhereClause`

- <span id="cratewhereclause-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl Hash for crate::WhereClause`

- <span id="cratewhereclause-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::WhereClause`

- <span id="crategenericswhereclause-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::WhereClause`

- <span id="cratewhereclause-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WhereClause`

##### `impl Spanned for WhereClause`

- <span id="whereclause-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::WhereClause`

- <span id="crategenericswhereclause-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:490-498`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L490-L498)*

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

#### Trait Implementations

##### `impl Clone for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl Hash for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateLifetime`

##### `impl Spanned for PredicateLifetime`

- <span id="predicatelifetime-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- <span id="crategenericspredicatelifetime-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:500-512`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L500-L512)*

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

#### Trait Implementations

##### `impl Clone for crate::PredicateType`

- <span id="cratepredicatetype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PredicateType`

- <span id="cratepredicatetype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl Hash for crate::PredicateType`

- <span id="cratepredicatetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PredicateType`

- <span id="cratepredicatetype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateType`

##### `impl Spanned for PredicateType`

- <span id="predicatetype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::PredicateType`

- <span id="crategenericspredicatetype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:34-54`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L34-L54)*

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

- <span id="crategenericparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::GenericParam`

- <span id="crategenericparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl Hash for crate::GenericParam`

- <span id="crategenericparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::GenericParam`

- <span id="crategenericsgenericparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::GenericParam`

- <span id="crategenericparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericParam`

##### `impl Spanned for GenericParam`

- <span id="genericparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for GenericParam`

- <span id="genericparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:398-408`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L398-L408)*

A trait or lifetime used as a bound on a type parameter.

#### Implementations

- <span id="crategenericstypeparambound-parse-single"></span>`fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="crategenericstypeparambound-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, token::Plus>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Punctuated`](../punctuated/index.md#punctuated), [`Plus`](../token/index.md#plus)

#### Trait Implementations

##### `impl Clone for crate::TypeParamBound`

- <span id="cratetypeparambound-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParamBound`

- <span id="cratetypeparambound-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl Hash for crate::TypeParamBound`

- <span id="cratetypeparambound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TypeParamBound`

- <span id="crategenericstypeparambound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeParamBound`

- <span id="cratetypeparambound-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParamBound`

##### `impl Spanned for TypeParamBound`

- <span id="typeparambound-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for TypeParamBound`

- <span id="typeparambound-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe(token::Question),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:423-431`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L423-L431)*

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

#### Trait Implementations

##### `impl Clone for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl Hash for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBoundModifier`

##### `impl Spanned for TraitBoundModifier`

- <span id="traitboundmodifier-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:446-459`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L446-L459)*

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

- <span id="cratecapturedparam-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::CapturedParam`

- <span id="cratecapturedparam-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl Hash for crate::CapturedParam`

- <span id="cratecapturedparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::CapturedParam`

- <span id="cratecapturedparam-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for CapturedParam`

##### `impl Spanned for CapturedParam`

- <span id="capturedparam-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:471-488`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L471-L488)*

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

- <span id="cratewherepredicate-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::WherePredicate`

- <span id="cratewherepredicate-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl Hash for crate::WherePredicate`

- <span id="cratewherepredicate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::generics::WherePredicate`

- <span id="crategenericswherepredicate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::WherePredicate`

- <span id="cratewherepredicate-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WherePredicate`

##### `impl Spanned for WherePredicate`

- <span id="wherepredicate-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for WherePredicate`

- <span id="wherepredicate-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

## Macros

### `generics_wrapper_impls!`

*Defined in [`syn-2.0.111/src/generics.rs:294-335`](../../../.source_1765210505/syn-2.0.111/src/generics.rs#L294-L335)*

