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

*Defined in [`syn-2.0.111/src/generics.rs:15-32`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L15-L32)*

Lifetimes and type parameters attached to a declaration of a function,
enum, trait, etc.

This struct represents two distinct optional syntactic elements,
[generic parameters] and [where clause]. In some locations of the
grammar, there may be other tokens in between these two things.



#### Implementations

- <span id="generics-lifetimes"></span>`fn lifetimes(&self) -> Lifetimes<'_>` — [`Lifetimes`](#lifetimes)

  Iterator over the lifetime parameters in `self.params`.

- <span id="generics-lifetimes-mut"></span>`fn lifetimes_mut(&mut self) -> LifetimesMut<'_>` — [`LifetimesMut`](#lifetimesmut)

  Iterator over the lifetime parameters in `self.params`.

- <span id="generics-type-params"></span>`fn type_params(&self) -> TypeParams<'_>` — [`TypeParams`](#typeparams)

  Iterator over the type parameters in `self.params`.

- <span id="generics-type-params-mut"></span>`fn type_params_mut(&mut self) -> TypeParamsMut<'_>` — [`TypeParamsMut`](#typeparamsmut)

  Iterator over the type parameters in `self.params`.

- <span id="generics-const-params"></span>`fn const_params(&self) -> ConstParams<'_>` — [`ConstParams`](#constparams)

  Iterator over the constant parameters in `self.params`.

- <span id="generics-const-params-mut"></span>`fn const_params_mut(&mut self) -> ConstParamsMut<'_>` — [`ConstParamsMut`](#constparamsmut)

  Iterator over the constant parameters in `self.params`.

- <span id="generics-make-where-clause"></span>`fn make_where_clause(&mut self) -> &mut WhereClause` — [`WhereClause`](#whereclause)

  Initializes an empty `where`-clause if there is not one present already.

- <span id="generics-split-for-impl"></span>`fn split_for_impl(&self) -> (ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>)` — [`ImplGenerics`](#implgenerics), [`TypeGenerics`](#typegenerics), [`WhereClause`](#whereclause)

  Split a type's generics into the pieces required for impl'ing a trait

  for that type.

  

  ```rust

  use proc_macro2::{Span, Ident};

  use quote::quote;

  

  let generics: syn::Generics = Default::default();

  let name = Ident::new("MyType", Span::call_site());

  

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {

      impl #impl_generics MyTrait for #name #ty_generics #where_clause {

          // ...

      }

  }

  ;

  ```

#### Trait Implementations

##### `impl Any for Generics`

- <span id="generics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Generics`

- <span id="generics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Generics`

- <span id="generics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Generics`

- <span id="crategenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Generics`

- <span id="generics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Generics`

- <span id="crategenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generics`

- <span id="generics-default"></span>`fn default() -> Self`

##### `impl Eq for crate::Generics`

##### `impl<T> From for Generics`

- <span id="generics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Generics`

- <span id="crategenerics-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Generics`

- <span id="generics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::Generics`

- <span id="crategenericsgenerics-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Generics`

- <span id="crategenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Generics`

##### `impl Spanned for Generics`

- <span id="generics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Generics`

- <span id="generics-toowned-type-owned"></span>`type Owned = T`

- <span id="generics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="generics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::Generics`

- <span id="crategenericsgenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Generics`

- <span id="generics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Generics`

- <span id="generics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LifetimeParam`

```rust
struct LifetimeParam {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: Option<token::Colon>,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:56-65`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L56-L65)*

A lifetime definition: `'a: 'b + 'c + 'd`.

#### Implementations

- <span id="lifetimeparam-new"></span>`fn new(lifetime: Lifetime) -> Self` — [`Lifetime`](../lifetime/index.md#lifetime)

#### Trait Implementations

##### `impl Any for LifetimeParam`

- <span id="lifetimeparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LifetimeParam`

- <span id="lifetimeparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LifetimeParam`

- <span id="lifetimeparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LifetimeParam`

- <span id="cratelifetimeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LifetimeParam`

- <span id="lifetimeparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::LifetimeParam`

- <span id="cratelifetimeparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LifetimeParam`

##### `impl<T> From for LifetimeParam`

- <span id="lifetimeparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LifetimeParam`

- <span id="cratelifetimeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LifetimeParam`

- <span id="lifetimeparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::LifetimeParam`

- <span id="cratelifetimeparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for LifetimeParam`

##### `impl Spanned for LifetimeParam`

- <span id="lifetimeparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LifetimeParam`

- <span id="lifetimeparam-toowned-type-owned"></span>`type Owned = T`

- <span id="lifetimeparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lifetimeparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::LifetimeParam`

- <span id="crategenericslifetimeparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for LifetimeParam`

- <span id="lifetimeparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lifetimeparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LifetimeParam`

- <span id="lifetimeparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lifetimeparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/generics.rs:67-78`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L67-L78)*

A generic type parameter: `T: Into<String>`.

#### Trait Implementations

##### `impl Any for TypeParam`

- <span id="typeparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParam`

- <span id="typeparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParam`

- <span id="typeparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParam`

- <span id="cratetypeparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParam`

- <span id="typeparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParam`

- <span id="cratetypeparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParam`

##### `impl<T> From for TypeParam`

- <span id="typeparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParam`

- <span id="cratetypeparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParam`

- <span id="typeparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TypeParam`

- <span id="crategenericstypeparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeParam`

- <span id="cratetypeparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParam`

##### `impl Spanned for TypeParam`

- <span id="typeparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParam`

- <span id="typeparam-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TypeParam`

- <span id="crategenericstypeparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeParam`

- <span id="typeparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParam`

- <span id="typeparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/generics.rs:80-92`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L80-L92)*

A const generic parameter: `const LENGTH: usize`.

#### Trait Implementations

##### `impl Any for ConstParam`

- <span id="constparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstParam`

- <span id="constparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstParam`

- <span id="constparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ConstParam`

- <span id="crateconstparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ConstParam`

- <span id="constparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ConstParam`

- <span id="crateconstparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ConstParam`

##### `impl<T> From for ConstParam`

- <span id="constparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ConstParam`

- <span id="crateconstparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ConstParam`

- <span id="constparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::ConstParam`

- <span id="crategenericsconstparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ConstParam`

- <span id="crateconstparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ConstParam`

##### `impl Spanned for ConstParam`

- <span id="constparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ConstParam`

- <span id="constparam-toowned-type-owned"></span>`type Owned = T`

- <span id="constparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::ConstParam`

- <span id="crategenericsconstparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ConstParam`

- <span id="constparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstParam`

- <span id="constparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lifetimes<'a>`

```rust
struct Lifetimes<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:185`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L185)*

#### Trait Implementations

##### `impl Any for Lifetimes<'a>`

- <span id="lifetimes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lifetimes<'a>`

- <span id="lifetimes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lifetimes<'a>`

- <span id="lifetimes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Lifetimes<'a>`

- <span id="lifetimes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lifetimes<'a>`

- <span id="lifetimes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Lifetimes<'a>`

- <span id="lifetimes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="lifetimes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="lifetimes-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Lifetimes<'a>`

- <span id="lifetimes-iterator-type-item"></span>`type Item = &'a LifetimeParam`

- <span id="lifetimes-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for Lifetimes<'a>`

- <span id="lifetimes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lifetimes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lifetimes<'a>`

- <span id="lifetimes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lifetimes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LifetimesMut<'a>`

```rust
struct LifetimesMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:199`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L199)*

#### Trait Implementations

##### `impl Any for LifetimesMut<'a>`

- <span id="lifetimesmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LifetimesMut<'a>`

- <span id="lifetimesmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LifetimesMut<'a>`

- <span id="lifetimesmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for LifetimesMut<'a>`

- <span id="lifetimesmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LifetimesMut<'a>`

- <span id="lifetimesmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LifetimesMut<'a>`

- <span id="lifetimesmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="lifetimesmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="lifetimesmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LifetimesMut<'a>`

- <span id="lifetimesmut-iterator-type-item"></span>`type Item = &'a mut LifetimeParam`

- <span id="lifetimesmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for LifetimesMut<'a>`

- <span id="lifetimesmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lifetimesmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LifetimesMut<'a>`

- <span id="lifetimesmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lifetimesmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParams<'a>`

```rust
struct TypeParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:213`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L213)*

#### Trait Implementations

##### `impl Any for TypeParams<'a>`

- <span id="typeparams-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParams<'a>`

- <span id="typeparams-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParams<'a>`

- <span id="typeparams-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TypeParams<'a>`

- <span id="typeparams-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TypeParams<'a>`

- <span id="typeparams-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TypeParams<'a>`

- <span id="typeparams-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="typeparams-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="typeparams-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TypeParams<'a>`

- <span id="typeparams-iterator-type-item"></span>`type Item = &'a TypeParam`

- <span id="typeparams-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for TypeParams<'a>`

- <span id="typeparams-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparams-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParams<'a>`

- <span id="typeparams-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparams-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParamsMut<'a>`

```rust
struct TypeParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:227`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L227)*

#### Trait Implementations

##### `impl Any for TypeParamsMut<'a>`

- <span id="typeparamsmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParamsMut<'a>`

- <span id="typeparamsmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParamsMut<'a>`

- <span id="typeparamsmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TypeParamsMut<'a>`

- <span id="typeparamsmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TypeParamsMut<'a>`

- <span id="typeparamsmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TypeParamsMut<'a>`

- <span id="typeparamsmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="typeparamsmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="typeparamsmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TypeParamsMut<'a>`

- <span id="typeparamsmut-iterator-type-item"></span>`type Item = &'a mut TypeParam`

- <span id="typeparamsmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for TypeParamsMut<'a>`

- <span id="typeparamsmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparamsmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParamsMut<'a>`

- <span id="typeparamsmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparamsmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ConstParams<'a>`

```rust
struct ConstParams<'a>(crate::punctuated::Iter<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:241`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L241)*

#### Trait Implementations

##### `impl Any for ConstParams<'a>`

- <span id="constparams-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstParams<'a>`

- <span id="constparams-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstParams<'a>`

- <span id="constparams-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ConstParams<'a>`

- <span id="constparams-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ConstParams<'a>`

- <span id="constparams-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ConstParams<'a>`

- <span id="constparams-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="constparams-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="constparams-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ConstParams<'a>`

- <span id="constparams-iterator-type-item"></span>`type Item = &'a ConstParam`

- <span id="constparams-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ConstParams<'a>`

- <span id="constparams-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constparams-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstParams<'a>`

- <span id="constparams-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constparams-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ConstParamsMut<'a>`

```rust
struct ConstParamsMut<'a>(crate::punctuated::IterMut<'a, GenericParam>);
```

*Defined in [`syn-2.0.111/src/generics.rs:255`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L255)*

#### Trait Implementations

##### `impl Any for ConstParamsMut<'a>`

- <span id="constparamsmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConstParamsMut<'a>`

- <span id="constparamsmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConstParamsMut<'a>`

- <span id="constparamsmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ConstParamsMut<'a>`

- <span id="constparamsmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ConstParamsMut<'a>`

- <span id="constparamsmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ConstParamsMut<'a>`

- <span id="constparamsmut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="constparamsmut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="constparamsmut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ConstParamsMut<'a>`

- <span id="constparamsmut-iterator-type-item"></span>`type Item = &'a mut ConstParam`

- <span id="constparamsmut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for ConstParamsMut<'a>`

- <span id="constparamsmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constparamsmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConstParamsMut<'a>`

- <span id="constparamsmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constparamsmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ImplGenerics<'a>`

```rust
struct ImplGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:275`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L275)*

Returned by `Generics::split_for_impl`.

#### Trait Implementations

##### `impl Any for ImplGenerics<'a>`

- <span id="implgenerics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ImplGenerics<'a>`

- <span id="implgenerics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ImplGenerics<'a>`

- <span id="implgenerics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ImplGenerics<'a>`

- <span id="implgenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ImplGenerics<'a>`

- <span id="implgenerics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ImplGenerics<'a>`

- <span id="implgenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImplGenerics<'a>`

##### `impl<T> From for ImplGenerics<'a>`

- <span id="implgenerics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ImplGenerics<'a>`

- <span id="implgenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for ImplGenerics<'a>`

- <span id="implgenerics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ImplGenerics<'a>`

- <span id="implgenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ImplGenerics<'a>`

##### `impl Spanned for ImplGenerics<'a>`

- <span id="implgenerics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ImplGenerics<'a>`

- <span id="implgenerics-toowned-type-owned"></span>`type Owned = T`

- <span id="implgenerics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="implgenerics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::ImplGenerics<'a>`

- <span id="crategenericsimplgenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ImplGenerics<'a>`

- <span id="implgenerics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="implgenerics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ImplGenerics<'a>`

- <span id="implgenerics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="implgenerics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeGenerics<'a>`

```rust
struct TypeGenerics<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:283`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L283)*

Returned by `Generics::split_for_impl`.

#### Implementations

- <span id="typegenerics-as-turbofish"></span>`fn as_turbofish(&self) -> Turbofish<'a>` — [`Turbofish`](#turbofish)

  Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`.

#### Trait Implementations

##### `impl Any for TypeGenerics<'a>`

- <span id="typegenerics-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeGenerics<'a>`

- <span id="typegenerics-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeGenerics<'a>`

- <span id="typegenerics-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TypeGenerics<'a>`

- <span id="typegenerics-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeGenerics<'a>`

- <span id="typegenerics-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TypeGenerics<'a>`

- <span id="typegenerics-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeGenerics<'a>`

##### `impl<T> From for TypeGenerics<'a>`

- <span id="typegenerics-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for TypeGenerics<'a>`

- <span id="typegenerics-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for TypeGenerics<'a>`

- <span id="typegenerics-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for TypeGenerics<'a>`

- <span id="typegenerics-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeGenerics<'a>`

##### `impl Spanned for TypeGenerics<'a>`

- <span id="typegenerics-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeGenerics<'a>`

- <span id="typegenerics-toowned-type-owned"></span>`type Owned = T`

- <span id="typegenerics-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typegenerics-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TypeGenerics<'a>`

- <span id="crategenericstypegenerics-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TypeGenerics<'a>`

- <span id="typegenerics-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typegenerics-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeGenerics<'a>`

- <span id="typegenerics-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typegenerics-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Turbofish<'a>`

```rust
struct Turbofish<'a>(&'a Generics);
```

*Defined in [`syn-2.0.111/src/generics.rs:291`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L291)*

Returned by `TypeGenerics::as_turbofish`.

#### Trait Implementations

##### `impl Any for Turbofish<'a>`

- <span id="turbofish-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Turbofish<'a>`

- <span id="turbofish-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Turbofish<'a>`

- <span id="turbofish-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Turbofish<'a>`

- <span id="turbofish-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Turbofish<'a>`

- <span id="turbofish-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Turbofish<'a>`

- <span id="turbofish-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Turbofish<'a>`

##### `impl<T> From for Turbofish<'a>`

- <span id="turbofish-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Turbofish<'a>`

- <span id="turbofish-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<U> Into for Turbofish<'a>`

- <span id="turbofish-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Turbofish<'a>`

- <span id="turbofish-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Turbofish<'a>`

##### `impl Spanned for Turbofish<'a>`

- <span id="turbofish-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Turbofish<'a>`

- <span id="turbofish-toowned-type-owned"></span>`type Owned = T`

- <span id="turbofish-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="turbofish-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::Turbofish<'a>`

- <span id="crategenericsturbofish-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Turbofish<'a>`

- <span id="turbofish-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="turbofish-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Turbofish<'a>`

- <span id="turbofish-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="turbofish-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundLifetimes`

```rust
struct BoundLifetimes {
    pub for_token: token::For,
    pub lt_token: token::Lt,
    pub lifetimes: crate::punctuated::Punctuated<GenericParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:352-361`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L352-L361)*

A set of bound lifetimes: `for<'a, 'b, 'c>`.

#### Trait Implementations

##### `impl Any for BoundLifetimes`

- <span id="boundlifetimes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundLifetimes`

- <span id="boundlifetimes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundLifetimes`

- <span id="boundlifetimes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BoundLifetimes`

- <span id="crateboundlifetimes-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BoundLifetimes`

- <span id="boundlifetimes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::BoundLifetimes`

- <span id="crateboundlifetimes-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoundLifetimes`

- <span id="boundlifetimes-default"></span>`fn default() -> Self`

##### `impl Eq for crate::BoundLifetimes`

##### `impl<T> From for BoundLifetimes`

- <span id="boundlifetimes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BoundLifetimes`

- <span id="crateboundlifetimes-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BoundLifetimes`

- <span id="boundlifetimes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::BoundLifetimes`

- <span id="crateboundlifetimes-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BoundLifetimes`

##### `impl Spanned for BoundLifetimes`

- <span id="boundlifetimes-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BoundLifetimes`

- <span id="boundlifetimes-toowned-type-owned"></span>`type Owned = T`

- <span id="boundlifetimes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boundlifetimes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::BoundLifetimes`

- <span id="crategenericsboundlifetimes-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BoundLifetimes`

- <span id="boundlifetimes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundlifetimes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundLifetimes`

- <span id="boundlifetimes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundlifetimes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBound`

```rust
struct TraitBound {
    pub paren_token: Option<token::Paren>,
    pub modifier: TraitBoundModifier,
    pub lifetimes: Option<BoundLifetimes>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:410-421`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L410-L421)*

A trait used as a bound on a type parameter.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  The `for<'a>` in `for<'a> Foo<&'a T>`

- **`path`**: `crate::path::Path`

  The `Foo<&'a T>` in `for<'a> Foo<&'a T>`

#### Implementations

- <span id="crategenericstraitbound-do-parse"></span>`fn do_parse(input: ParseStream<'_>, allow_const: bool) -> Result<Option<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for TraitBound`

- <span id="traitbound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitBound`

- <span id="traitbound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitBound`

- <span id="traitbound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitBound`

- <span id="cratetraitbound-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitBound`

- <span id="traitbound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TraitBound`

- <span id="cratetraitbound-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBound`

##### `impl<T> From for TraitBound`

- <span id="traitbound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitBound`

- <span id="cratetraitbound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitBound`

- <span id="traitbound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TraitBound`

- <span id="crategenericstraitbound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitBound`

- <span id="cratetraitbound-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBound`

##### `impl Spanned for TraitBound`

- <span id="traitbound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitBound`

- <span id="traitbound-toowned-type-owned"></span>`type Owned = T`

- <span id="traitbound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitbound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TraitBound`

- <span id="crategenericstraitbound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitBound`

- <span id="traitbound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitbound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitBound`

- <span id="traitbound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitbound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PreciseCapture`

```rust
struct PreciseCapture {
    pub use_token: token::Use,
    pub lt_token: token::Lt,
    pub params: crate::punctuated::Punctuated<CapturedParam, token::Comma>,
    pub gt_token: token::Gt,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:433-443`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L433-L443)*

Precise capturing bound: the 'use&lt;&hellip;&gt;' in `impl Trait +
use<'a, T>`.

#### Trait Implementations

##### `impl Any for PreciseCapture`

- <span id="precisecapture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PreciseCapture`

- <span id="precisecapture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PreciseCapture`

- <span id="precisecapture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PreciseCapture`

- <span id="crateprecisecapture-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PreciseCapture`

- <span id="precisecapture-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PreciseCapture`

- <span id="crateprecisecapture-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PreciseCapture`

##### `impl<T> From for PreciseCapture`

- <span id="precisecapture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PreciseCapture`

- <span id="crateprecisecapture-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PreciseCapture`

- <span id="precisecapture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PreciseCapture`

- <span id="crateprecisecapture-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PreciseCapture`

##### `impl Spanned for PreciseCapture`

- <span id="precisecapture-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PreciseCapture`

- <span id="precisecapture-toowned-type-owned"></span>`type Owned = T`

- <span id="precisecapture-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="precisecapture-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PreciseCapture`

- <span id="crategenericsprecisecapture-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PreciseCapture`

- <span id="precisecapture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="precisecapture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PreciseCapture`

- <span id="precisecapture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="precisecapture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhereClause`

```rust
struct WhereClause {
    pub where_token: token::Where,
    pub predicates: crate::punctuated::Punctuated<WherePredicate, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:461-469`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L461-L469)*

A `where` clause in a definition: `where T: Deserialize<'de>, D:
'static`.

#### Trait Implementations

##### `impl Any for WhereClause`

- <span id="whereclause-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhereClause`

- <span id="whereclause-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhereClause`

- <span id="whereclause-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::WhereClause`

- <span id="cratewhereclause-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WhereClause`

- <span id="whereclause-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::WhereClause`

- <span id="cratewhereclause-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WhereClause`

##### `impl<T> From for WhereClause`

- <span id="whereclause-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::WhereClause`

- <span id="cratewhereclause-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for WhereClause`

- <span id="whereclause-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::WhereClause`

- <span id="crategenericswhereclause-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::WhereClause`

- <span id="cratewhereclause-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WhereClause`

##### `impl Spanned for WhereClause`

- <span id="whereclause-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for WhereClause`

- <span id="whereclause-toowned-type-owned"></span>`type Owned = T`

- <span id="whereclause-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whereclause-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::WhereClause`

- <span id="crategenericswhereclause-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for WhereClause`

- <span id="whereclause-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whereclause-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhereClause`

- <span id="whereclause-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whereclause-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateLifetime`

```rust
struct PredicateLifetime {
    pub lifetime: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::lifetime::Lifetime, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:490-498`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L490-L498)*

A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

#### Trait Implementations

##### `impl Any for PredicateLifetime`

- <span id="predicatelifetime-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PredicateLifetime`

- <span id="predicatelifetime-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PredicateLifetime`

- <span id="predicatelifetime-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PredicateLifetime`

- <span id="predicatelifetime-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateLifetime`

##### `impl<T> From for PredicateLifetime`

- <span id="predicatelifetime-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PredicateLifetime`

- <span id="predicatelifetime-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PredicateLifetime`

- <span id="cratepredicatelifetime-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateLifetime`

##### `impl Spanned for PredicateLifetime`

- <span id="predicatelifetime-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PredicateLifetime`

- <span id="predicatelifetime-toowned-type-owned"></span>`type Owned = T`

- <span id="predicatelifetime-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="predicatelifetime-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PredicateLifetime`

- <span id="crategenericspredicatelifetime-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PredicateLifetime`

- <span id="predicatelifetime-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="predicatelifetime-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PredicateLifetime`

- <span id="predicatelifetime-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="predicatelifetime-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PredicateType`

```rust
struct PredicateType {
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: crate::ty::Type,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<TypeParamBound, token::Plus>,
}
```

*Defined in [`syn-2.0.111/src/generics.rs:500-512`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L500-L512)*

A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Fields

- **`lifetimes`**: `Option<BoundLifetimes>`

  Any lifetimes from a `for` binding

- **`bounded_ty`**: `crate::ty::Type`

  The type being bounded

- **`bounds`**: `crate::punctuated::Punctuated<TypeParamBound, token::Plus>`

  Trait and lifetime bounds (`Clone+Send+'static`)

#### Trait Implementations

##### `impl Any for PredicateType`

- <span id="predicatetype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PredicateType`

- <span id="predicatetype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PredicateType`

- <span id="predicatetype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PredicateType`

- <span id="cratepredicatetype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PredicateType`

- <span id="predicatetype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PredicateType`

- <span id="cratepredicatetype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PredicateType`

##### `impl<T> From for PredicateType`

- <span id="predicatetype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PredicateType`

- <span id="cratepredicatetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PredicateType`

- <span id="predicatetype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PredicateType`

- <span id="cratepredicatetype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PredicateType`

##### `impl Spanned for PredicateType`

- <span id="predicatetype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PredicateType`

- <span id="predicatetype-toowned-type-owned"></span>`type Owned = T`

- <span id="predicatetype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="predicatetype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::PredicateType`

- <span id="crategenericspredicatetype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PredicateType`

- <span id="predicatetype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="predicatetype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PredicateType`

- <span id="predicatetype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="predicatetype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `GenericParam`

```rust
enum GenericParam {
    Lifetime(LifetimeParam),
    Type(TypeParam),
    Const(ConstParam),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:34-54`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L34-L54)*

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

##### `impl Any for GenericParam`

- <span id="genericparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GenericParam`

- <span id="genericparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenericParam`

- <span id="genericparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::GenericParam`

- <span id="crategenericparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for GenericParam`

- <span id="genericparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::GenericParam`

- <span id="crategenericparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericParam`

##### `impl<T> From for GenericParam`

- <span id="genericparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::GenericParam`

- <span id="crategenericparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for GenericParam`

- <span id="genericparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::GenericParam`

- <span id="crategenericsgenericparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::GenericParam`

- <span id="crategenericparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for GenericParam`

##### `impl Spanned for GenericParam`

- <span id="genericparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for GenericParam`

- <span id="genericparam-toowned-type-owned"></span>`type Owned = T`

- <span id="genericparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="genericparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for GenericParam`

- <span id="genericparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for GenericParam`

- <span id="genericparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="genericparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenericParam`

- <span id="genericparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="genericparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TypeParamBound`

```rust
enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(crate::lifetime::Lifetime),
    PreciseCapture(PreciseCapture),
    Verbatim(proc_macro2::TokenStream),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:398-408`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L398-L408)*

A trait or lifetime used as a bound on a type parameter.

#### Implementations

- <span id="crategenericstypeparambound-parse-single"></span>`fn parse_single(input: ParseStream<'_>, allow_precise_capture: bool, allow_const: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

- <span id="crategenericstypeparambound-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>, allow_plus: bool, allow_precise_capture: bool, allow_const: bool) -> Result<Punctuated<Self, token::Plus>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Punctuated`](../punctuated/index.md#punctuated), [`Plus`](../token/index.md#plus)

#### Trait Implementations

##### `impl Any for TypeParamBound`

- <span id="typeparambound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TypeParamBound`

- <span id="typeparambound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TypeParamBound`

- <span id="typeparambound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TypeParamBound`

- <span id="cratetypeparambound-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TypeParamBound`

- <span id="typeparambound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::TypeParamBound`

- <span id="cratetypeparambound-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParamBound`

##### `impl<T> From for TypeParamBound`

- <span id="typeparambound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TypeParamBound`

- <span id="cratetypeparambound-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TypeParamBound`

- <span id="typeparambound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TypeParamBound`

- <span id="crategenericstypeparambound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TypeParamBound`

- <span id="cratetypeparambound-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TypeParamBound`

##### `impl Spanned for TypeParamBound`

- <span id="typeparambound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TypeParamBound`

- <span id="typeparambound-toowned-type-owned"></span>`type Owned = T`

- <span id="typeparambound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeparambound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for TypeParamBound`

- <span id="typeparambound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for TypeParamBound`

- <span id="typeparambound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeparambound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TypeParamBound`

- <span id="typeparambound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeparambound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TraitBoundModifier`

```rust
enum TraitBoundModifier {
    None,
    Maybe(token::Question),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:423-431`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L423-L431)*

A modifier on a trait bound, currently only used for the `?` in
`?Sized`.

#### Trait Implementations

##### `impl Any for TraitBoundModifier`

- <span id="traitboundmodifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TraitBoundModifier`

- <span id="traitboundmodifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TraitBoundModifier`

- <span id="traitboundmodifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TraitBoundModifier`

- <span id="traitboundmodifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::TraitBoundModifier`

##### `impl Debug for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TraitBoundModifier`

##### `impl<T> From for TraitBoundModifier`

- <span id="traitboundmodifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for TraitBoundModifier`

- <span id="traitboundmodifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::TraitBoundModifier`

- <span id="cratetraitboundmodifier-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for TraitBoundModifier`

##### `impl Spanned for TraitBoundModifier`

- <span id="traitboundmodifier-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for TraitBoundModifier`

- <span id="traitboundmodifier-toowned-type-owned"></span>`type Owned = T`

- <span id="traitboundmodifier-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="traitboundmodifier-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::TraitBoundModifier`

- <span id="crategenericstraitboundmodifier-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for TraitBoundModifier`

- <span id="traitboundmodifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="traitboundmodifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TraitBoundModifier`

- <span id="traitboundmodifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="traitboundmodifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CapturedParam`

```rust
enum CapturedParam {
    Lifetime(crate::lifetime::Lifetime),
    Ident(crate::ident::Ident),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:446-459`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L446-L459)*

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

##### `impl Any for CapturedParam`

- <span id="capturedparam-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapturedParam`

- <span id="capturedparam-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapturedParam`

- <span id="capturedparam-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::CapturedParam`

- <span id="cratecapturedparam-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for CapturedParam`

- <span id="capturedparam-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::CapturedParam`

- <span id="cratecapturedparam-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::CapturedParam`

##### `impl<T> From for CapturedParam`

- <span id="capturedparam-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::CapturedParam`

- <span id="cratecapturedparam-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for CapturedParam`

- <span id="capturedparam-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::CapturedParam`

- <span id="cratecapturedparam-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for CapturedParam`

##### `impl Spanned for CapturedParam`

- <span id="capturedparam-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for CapturedParam`

- <span id="capturedparam-toowned-type-owned"></span>`type Owned = T`

- <span id="capturedparam-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capturedparam-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::generics::CapturedParam`

- <span id="crategenericscapturedparam-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for CapturedParam`

- <span id="capturedparam-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturedparam-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapturedParam`

- <span id="capturedparam-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturedparam-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WherePredicate`

```rust
enum WherePredicate {
    Lifetime(PredicateLifetime),
    Type(PredicateType),
}
```

*Defined in [`syn-2.0.111/src/generics.rs:471-488`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L471-L488)*

A single predicate in a `where` clause: `T: Deserialize<'de>`.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Lifetime`**

  A lifetime predicate in a `where` clause: `'a: 'b + 'c`.

- **`Type`**

  A type predicate in a `where` clause: `for<'c> Foo<'c>: Trait<'c>`.

#### Trait Implementations

##### `impl Any for WherePredicate`

- <span id="wherepredicate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WherePredicate`

- <span id="wherepredicate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WherePredicate`

- <span id="wherepredicate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::WherePredicate`

- <span id="cratewherepredicate-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WherePredicate`

- <span id="wherepredicate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::WherePredicate`

- <span id="cratewherepredicate-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::WherePredicate`

##### `impl<T> From for WherePredicate`

- <span id="wherepredicate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::WherePredicate`

- <span id="cratewherepredicate-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for WherePredicate`

- <span id="wherepredicate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::generics::WherePredicate`

- <span id="crategenericswherepredicate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::WherePredicate`

- <span id="cratewherepredicate-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for WherePredicate`

##### `impl Spanned for WherePredicate`

- <span id="wherepredicate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for WherePredicate`

- <span id="wherepredicate-toowned-type-owned"></span>`type Owned = T`

- <span id="wherepredicate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="wherepredicate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for WherePredicate`

- <span id="wherepredicate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for WherePredicate`

- <span id="wherepredicate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="wherepredicate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WherePredicate`

- <span id="wherepredicate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="wherepredicate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `generics_wrapper_impls!`

*Defined in [`syn-2.0.111/src/generics.rs:294-335`](../../../.source_1765521767/syn-2.0.111/src/generics.rs#L294-L335)*

