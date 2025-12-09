*[syn](../index.md) / [ty](index.md)*

---

# Module `ty`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`TypeArray`](#typearray)
  - [`TypeBareFn`](#typebarefn)
  - [`TypeGroup`](#typegroup)
  - [`TypeImplTrait`](#typeimpltrait)
  - [`TypeInfer`](#typeinfer)
  - [`TypeMacro`](#typemacro)
  - [`TypeNever`](#typenever)
  - [`TypeParen`](#typeparen)
  - [`TypePath`](#typepath)
  - [`TypePtr`](#typeptr)
  - [`TypeReference`](#typereference)
  - [`TypeSlice`](#typeslice)
  - [`TypeTraitObject`](#typetraitobject)
  - [`TypeTuple`](#typetuple)
  - [`Abi`](#abi)
  - [`BareFnArg`](#barefnarg)
  - [`BareVariadic`](#barevariadic)
- [Enums](#enums)
  - [`Type`](#type)
  - [`ReturnType`](#returntype)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`TypeArray`](#typearray) | struct | A fixed size array type: `[T; n]`. |
| [`TypeBareFn`](#typebarefn) | struct | A bare function type: `fn(usize) -> bool`. |
| [`TypeGroup`](#typegroup) | struct | A type contained within invisible delimiters. |
| [`TypeImplTrait`](#typeimpltrait) | struct | An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or |
| [`TypeInfer`](#typeinfer) | struct | Indication that a type should be inferred by the compiler: `_`. |
| [`TypeMacro`](#typemacro) | struct | A macro in the type position. |
| [`TypeNever`](#typenever) | struct | The never type: `!`. |
| [`TypeParen`](#typeparen) | struct | A parenthesized type equivalent to the inner type. |
| [`TypePath`](#typepath) | struct | A path like `std::slice::Iter`, optionally qualified with a |
| [`TypePtr`](#typeptr) | struct | A raw pointer type: `*const T` or `*mut T`. |
| [`TypeReference`](#typereference) | struct | A reference type: `&'a T` or `&'a mut T`. |
| [`TypeSlice`](#typeslice) | struct | A dynamically sized slice type: `[T]`. |
| [`TypeTraitObject`](#typetraitobject) | struct | A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a |
| [`TypeTuple`](#typetuple) | struct | A tuple type: `(A, B, C, String)`. |
| [`Abi`](#abi) | struct | The binary interface of a function: `extern "C"`. |
| [`BareFnArg`](#barefnarg) | struct | An argument in a function type: the `usize` in `fn(usize) -> bool`. |
| [`BareVariadic`](#barevariadic) | struct | The variadic argument of a function pointer like `fn(usize, ...)`. |
| [`Type`](#type) | enum | The possible types that a Rust value could have. |
| [`ReturnType`](#returntype) | enum | Return type of a function signature. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: token::Semi,
    pub len: crate::expr::Expr,
}
```

A fixed size array type: `[T; n]`.

#### Implementations

- <span id="cratetypearray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeArray`

- <span id="cratetypearray-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeArray`

- <span id="cratetypearray-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl Hash for crate::TypeArray`

- <span id="cratetypearray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeArray`

- <span id="cratetytypearray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeArray`

- <span id="cratetypearray-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeArray`

##### `impl<T> Spanned for TypeArray`

- <span id="typearray-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeArray`

- <span id="cratetytypearray-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeBareFn`

```rust
struct TypeBareFn {
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub unsafety: Option<token::Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: token::Fn,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<BareFnArg, token::Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
```

A bare function type: `fn(usize) -> bool`.

#### Implementations

- <span id="cratetypebarefn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeBareFn`

- <span id="cratetypebarefn-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeBareFn`

- <span id="cratetypebarefn-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl Hash for crate::TypeBareFn`

- <span id="cratetypebarefn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeBareFn`

- <span id="cratetypebarefn-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeBareFn`

##### `impl<T> Spanned for TypeBareFn`

- <span id="typebarefn-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeBareFn`

- <span id="cratetytypebarefn-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

A type contained within invisible delimiters.

#### Implementations

- <span id="cratetypegroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeGroup`

- <span id="cratetypegroup-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeGroup`

- <span id="cratetypegroup-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl Hash for crate::TypeGroup`

- <span id="cratetypegroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeGroup`

- <span id="cratetytypegroup-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeGroup`

- <span id="cratetypegroup-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGroup`

##### `impl<T> Spanned for TypeGroup`

- <span id="typegroup-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeGroup`

- <span id="cratetytypegroup-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- <span id="cratetytypeimpltrait-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl Hash for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeImplTrait`

- <span id="cratetypeimpltrait-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeImplTrait`

##### `impl<T> Spanned for TypeImplTrait`

- <span id="typeimpltrait-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- <span id="cratetytypeimpltrait-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: token::Underscore,
}
```

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- <span id="cratetypeinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeInfer`

- <span id="cratetypeinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeInfer`

- <span id="cratetypeinfer-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl Hash for crate::TypeInfer`

- <span id="cratetypeinfer-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeInfer`

- <span id="cratetypeinfer-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeInfer`

##### `impl<T> Spanned for TypeInfer`

- <span id="typeinfer-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeInfer`

- <span id="cratetytypeinfer-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

A macro in the type position.

#### Implementations

- <span id="cratetypemacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeMacro`

- <span id="cratetypemacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeMacro`

- <span id="cratetypemacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl Hash for crate::TypeMacro`

- <span id="cratetypemacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeMacro`

- <span id="cratetytypemacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeMacro`

- <span id="cratetypemacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeMacro`

##### `impl<T> Spanned for TypeMacro`

- <span id="typemacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeMacro`

- <span id="cratetytypemacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: token::Not,
}
```

The never type: `!`.

#### Implementations

- <span id="cratetypenever-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeNever`

- <span id="cratetypenever-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeNever`

- <span id="cratetypenever-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl Hash for crate::TypeNever`

- <span id="cratetypenever-hash"></span>`fn hash<H>(&self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeNever`

- <span id="cratetytypenever-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeNever`

- <span id="cratetypenever-eq"></span>`fn eq(&self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeNever`

##### `impl<T> Spanned for TypeNever`

- <span id="typenever-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeNever`

- <span id="cratetytypenever-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

A parenthesized type equivalent to the inner type.

#### Implementations

- <span id="cratetypeparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeParen`

- <span id="cratetypeparen-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeParen`

- <span id="cratetypeparen-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl Hash for crate::TypeParen`

- <span id="cratetypeparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeParen`

- <span id="cratetytypeparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeParen`

- <span id="cratetypeparen-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParen`

##### `impl<T> Spanned for TypeParen`

- <span id="typeparen-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeParen`

- <span id="cratetytypeparen-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypePath`

```rust
struct TypePath {
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::slice::Iter`, optionally qualified with a
self-type as in `<Vec<T> as SomeTrait>::Associated`.

#### Implementations

- <span id="cratetypepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePath`

- <span id="cratetypepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypePath`

- <span id="cratetypepath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl Hash for crate::TypePath`

- <span id="cratetypepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypePath`

- <span id="cratetytypepath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypePath`

- <span id="cratetypepath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePath`

##### `impl<T> Spanned for TypePath`

- <span id="typepath-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypePath`

- <span id="cratetytypepath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: token::Star,
    pub const_token: Option<token::Const>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

A raw pointer type: `*const T` or `*mut T`.

#### Implementations

- <span id="cratetypeptr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePtr`

- <span id="cratetypeptr-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypePtr`

- <span id="cratetypeptr-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl Hash for crate::TypePtr`

- <span id="cratetypeptr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypePtr`

- <span id="cratetytypeptr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypePtr`

- <span id="cratetypeptr-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePtr`

##### `impl<T> Spanned for TypePtr`

- <span id="typeptr-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypePtr`

- <span id="cratetytypeptr-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<token::Mut>,
    pub elem: Box<Type>,
}
```

A reference type: `&'a T` or `&'a mut T`.

#### Implementations

- <span id="cratetypereference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeReference`

- <span id="cratetypereference-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeReference`

- <span id="cratetypereference-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl Hash for crate::TypeReference`

- <span id="cratetypereference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeReference`

- <span id="cratetytypereference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeReference`

- <span id="cratetypereference-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeReference`

##### `impl<T> Spanned for TypeReference`

- <span id="typereference-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeReference`

- <span id="cratetytypereference-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

A dynamically sized slice type: `[T]`.

#### Implementations

- <span id="cratetypeslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeSlice`

- <span id="cratetypeslice-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeSlice`

- <span id="cratetypeslice-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl Hash for crate::TypeSlice`

- <span id="cratetypeslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeSlice`

- <span id="cratetytypeslice-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeSlice`

- <span id="cratetypeslice-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeSlice`

##### `impl<T> Spanned for TypeSlice`

- <span id="typeslice-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeSlice`

- <span id="cratetytypeslice-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- <span id="cratetytypetraitobject-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="cratetytypetraitobject-parse-bounds"></span>`fn parse_bounds(dyn_span: Span, input: ParseStream<'_>, allow_plus: bool) -> Result<Punctuated<TypeParamBound, token::Plus>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md), [`Punctuated`](../punctuated/index.md), [`TypeParamBound`](../index.md), [`Plus`](../token/index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeTraitObject`

- <span id="cratetypetraitobject-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeTraitObject`

- <span id="cratetypetraitobject-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl Hash for crate::TypeTraitObject`

- <span id="cratetypetraitobject-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeTraitObject`

- <span id="cratetypetraitobject-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTraitObject`

##### `impl<T> Spanned for TypeTraitObject`

- <span id="typetraitobject-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- <span id="cratetytypetraitobject-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, token::Comma>,
}
```

A tuple type: `(A, B, C, String)`.

#### Implementations

- <span id="cratetypetuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTuple`

- <span id="cratetypetuple-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::TypeTuple`

- <span id="cratetypetuple-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl Hash for crate::TypeTuple`

- <span id="cratetypetuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTuple`

- <span id="cratetytypetuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeTuple`

- <span id="cratetypetuple-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTuple`

##### `impl<T> Spanned for TypeTuple`

- <span id="typetuple-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::TypeTuple`

- <span id="cratetytypetuple-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Abi`

```rust
struct Abi {
    pub extern_token: token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Clone for crate::Abi`

- <span id="crateabi-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Abi`

- <span id="crateabi-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl Hash for crate::Abi`

- <span id="crateabi-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::Abi`

- <span id="cratetyabi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Abi`

- <span id="crateabi-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Abi`

##### `impl<T> Spanned for Abi`

- <span id="abi-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::Abi`

- <span id="cratetyabi-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub ty: Type,
}
```

An argument in a function type: the `usize` in `fn(usize) -> bool`.

#### Trait Implementations

##### `impl Clone for crate::BareFnArg`

- <span id="cratebarefnarg-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BareFnArg`

- <span id="cratebarefnarg-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl Hash for crate::BareFnArg`

- <span id="cratebarefnarg-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::BareFnArg`

- <span id="cratebarefnarg-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BareFnArg`

##### `impl<T> Spanned for BareFnArg`

- <span id="barefnarg-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::BareFnArg`

- <span id="cratetybarefnarg-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, token::Colon)>,
    pub dots: token::DotDotDot,
    pub comma: Option<token::Comma>,
}
```

The variadic argument of a function pointer like `fn(usize, ...)`.

#### Trait Implementations

##### `impl Clone for crate::BareVariadic`

- <span id="cratebarevariadic-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::BareVariadic`

- <span id="cratebarevariadic-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl Hash for crate::BareVariadic`

- <span id="cratebarevariadic-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::BareVariadic`

- <span id="cratebarevariadic-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for BareVariadic`

##### `impl<T> Spanned for BareVariadic`

- <span id="barevariadic-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::BareVariadic`

- <span id="cratetybarevariadic-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Type`

```rust
enum Type {
    Array(TypeArray),
    BareFn(TypeBareFn),
    Group(TypeGroup),
    ImplTrait(TypeImplTrait),
    Infer(TypeInfer),
    Macro(TypeMacro),
    Never(TypeNever),
    Paren(TypeParen),
    Path(TypePath),
    Ptr(TypePtr),
    Reference(TypeReference),
    Slice(TypeSlice),
    TraitObject(TypeTraitObject),
    Tuple(TypeTuple),
    Verbatim(proc_macro2::TokenStream),
}
```

The possible types that a Rust value could have.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Array`**

  A fixed size array type: `[T; n]`.

- **`BareFn`**

  A bare function type: `fn(usize) -> bool`.

- **`Group`**

  A type contained within invisible delimiters.

- **`ImplTrait`**

  An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
  a lifetime.

- **`Infer`**

  Indication that a type should be inferred by the compiler: `_`.

- **`Macro`**

  A macro in the type position.

- **`Never`**

  The never type: `!`.

- **`Paren`**

  A parenthesized type equivalent to the inner type.

- **`Path`**

  A path like `std::slice::Iter`, optionally qualified with a
  self-type as in `<Vec<T> as SomeTrait>::Associated`.

- **`Ptr`**

  A raw pointer type: `*const T` or `*mut T`.

- **`Reference`**

  A reference type: `&'a T` or `&'a mut T`.

- **`Slice`**

  A dynamically sized slice type: `[T]`.

- **`TraitObject`**

  A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
  trait or a lifetime.

- **`Tuple`**

  A tuple type: `(A, B, C, String)`.

- **`Verbatim`**

  Tokens in type position not interpreted by Syn.

#### Implementations

- <span id="cratetytype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Type`

- <span id="cratetype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Type`

- <span id="cratetype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl Hash for crate::Type`

- <span id="cratetype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::Type`

- <span id="cratetytype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Type`

- <span id="cratetype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Type`

##### `impl<T> Spanned for Type`

- <span id="type-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Type`

- <span id="type-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type(token::RArrow, Box<Type>),
}
```

Return type of a function signature.

#### Variants

- **`Default`**

  Return type is not specified.
  
  Functions default to `()` and closures default to type inference.

- **`Type`**

  A particular type is returned.

#### Implementations

- <span id="cratetyreturntype-without-plus"></span>`fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::ReturnType`

- <span id="cratereturntype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ReturnType`

- <span id="cratereturntype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl Hash for crate::ReturnType`

- <span id="cratereturntype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::ty::ReturnType`

- <span id="cratetyreturntype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::ReturnType`

- <span id="cratereturntype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ReturnType`

##### `impl<T> Spanned for ReturnType`

- <span id="returntype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::ty::ReturnType`

- <span id="cratetyreturntype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

