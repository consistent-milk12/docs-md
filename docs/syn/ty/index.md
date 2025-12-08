*[syn](../index.md) / [ty](index.md)*

---

# Module `ty`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `TypeArray`

```rust
struct TypeArray {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
    pub semi_token: $crate::token::Semi,
    pub len: crate::expr::Expr,
}
```

A fixed size array type: `[T; n]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeArray`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeArray`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeArray`

##### `impl Hash for crate::TypeArray`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeArray`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeArray`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeArray`

##### `impl<T> Spanned for TypeArray`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeArray`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeBareFn`

```rust
struct TypeBareFn {
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub unsafety: Option<$crate::token::Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: $crate::token::Fn,
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<BareFnArg, $crate::token::Comma>,
    pub variadic: Option<BareVariadic>,
    pub output: ReturnType,
}
```

A bare function type: `fn(usize) -> bool`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeBareFn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeBareFn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeBareFn`

##### `impl Hash for crate::TypeBareFn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeBareFn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeBareFn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeBareFn`

##### `impl<T> Spanned for TypeBareFn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeBareFn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeGroup`

```rust
struct TypeGroup {
    pub group_token: token::Group,
    pub elem: Box<Type>,
}
```

A type contained within invisible delimiters.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeGroup`

##### `impl Hash for crate::TypeGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeGroup`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeGroup`

##### `impl<T> Spanned for TypeGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeImplTrait`

```rust
struct TypeImplTrait {
    pub impl_token: $crate::token::Impl,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
a lifetime.

#### Implementations

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeImplTrait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeImplTrait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeImplTrait`

##### `impl Hash for crate::TypeImplTrait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeImplTrait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeImplTrait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeImplTrait`

##### `impl<T> Spanned for TypeImplTrait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeImplTrait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeInfer`

```rust
struct TypeInfer {
    pub underscore_token: $crate::token::Underscore,
}
```

Indication that a type should be inferred by the compiler: `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeInfer`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeInfer`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeInfer`

##### `impl Hash for crate::TypeInfer`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeInfer`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeInfer`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeInfer`

##### `impl<T> Spanned for TypeInfer`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeInfer`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeMacro`

```rust
struct TypeMacro {
    pub mac: crate::mac::Macro,
}
```

A macro in the type position.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeMacro`

##### `impl Hash for crate::TypeMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeMacro`

##### `impl<T> Spanned for TypeMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeNever`

```rust
struct TypeNever {
    pub bang_token: $crate::token::Not,
}
```

The never type: `!`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeNever`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeNever`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeNever`

##### `impl Hash for crate::TypeNever`

- `fn hash<H>(self: &Self, _state: &mut H)`

##### `impl Parse for crate::ty::TypeNever`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeNever`

- `fn eq(self: &Self, _other: &Self) -> bool`

##### `impl<T> Sealed for TypeNever`

##### `impl<T> Spanned for TypeNever`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeNever`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeParen`

```rust
struct TypeParen {
    pub paren_token: token::Paren,
    pub elem: Box<Type>,
}
```

A parenthesized type equivalent to the inner type.

#### Implementations

- `fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::TypeParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeParen`

##### `impl Hash for crate::TypeParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeParen`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeParen`

##### `impl<T> Spanned for TypeParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypePath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePath`

##### `impl Hash for crate::TypePath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypePath`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypePath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePath`

##### `impl<T> Spanned for TypePath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypePath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypePtr`

```rust
struct TypePtr {
    pub star_token: $crate::token::Star,
    pub const_token: Option<$crate::token::Const>,
    pub mutability: Option<$crate::token::Mut>,
    pub elem: Box<Type>,
}
```

A raw pointer type: `*const T` or `*mut T`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypePtr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypePtr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypePtr`

##### `impl Hash for crate::TypePtr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypePtr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypePtr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypePtr`

##### `impl<T> Spanned for TypePtr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypePtr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeReference`

```rust
struct TypeReference {
    pub and_token: $crate::token::And,
    pub lifetime: Option<crate::lifetime::Lifetime>,
    pub mutability: Option<$crate::token::Mut>,
    pub elem: Box<Type>,
}
```

A reference type: `&'a T` or `&'a mut T`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeReference`

##### `impl Hash for crate::TypeReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeReference`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeReference`

##### `impl<T> Spanned for TypeReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeSlice`

```rust
struct TypeSlice {
    pub bracket_token: token::Bracket,
    pub elem: Box<Type>,
}
```

A dynamically sized slice type: `[T]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeSlice`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeSlice`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeSlice`

##### `impl Hash for crate::TypeSlice`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeSlice`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeSlice`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeSlice`

##### `impl<T> Spanned for TypeSlice`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeSlice`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeTraitObject`

```rust
struct TypeTraitObject {
    pub dyn_token: Option<$crate::token::Dyn>,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
trait or a lifetime.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTraitObject`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeTraitObject`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTraitObject`

##### `impl Hash for crate::TypeTraitObject`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTraitObject`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeTraitObject`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTraitObject`

##### `impl<T> Spanned for TypeTraitObject`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeTraitObject`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `TypeTuple`

```rust
struct TypeTuple {
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Type, $crate::token::Comma>,
}
```

A tuple type: `(A, B, C, String)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::TypeTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::TypeTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::TypeTuple`

##### `impl Hash for crate::TypeTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::TypeTuple`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::TypeTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for TypeTuple`

##### `impl<T> Spanned for TypeTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::TypeTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Abi`

```rust
struct Abi {
    pub extern_token: $crate::token::Extern,
    pub name: Option<crate::lit::LitStr>,
}
```

The binary interface of a function: `extern "C"`.

#### Trait Implementations

##### `impl Clone for crate::Abi`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Abi`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Abi`

##### `impl Hash for crate::Abi`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::Abi`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Abi`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Abi`

##### `impl<T> Spanned for Abi`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::Abi`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `BareFnArg`

```rust
struct BareFnArg {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, $crate::token::Colon)>,
    pub ty: Type,
}
```

An argument in a function type: the `usize` in `fn(usize) -> bool`.

#### Trait Implementations

##### `impl Clone for crate::BareFnArg`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BareFnArg`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareFnArg`

##### `impl Hash for crate::BareFnArg`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::BareFnArg`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::BareFnArg`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BareFnArg`

##### `impl<T> Spanned for BareFnArg`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::BareFnArg`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `BareVariadic`

```rust
struct BareVariadic {
    pub attrs: Vec<crate::attr::Attribute>,
    pub name: Option<(crate::ident::Ident, $crate::token::Colon)>,
    pub dots: $crate::token::DotDotDot,
    pub comma: Option<$crate::token::Comma>,
}
```

The variadic argument of a function pointer like `fn(usize, ...)`.

#### Trait Implementations

##### `impl Clone for crate::BareVariadic`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::BareVariadic`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BareVariadic`

##### `impl Hash for crate::BareVariadic`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::BareVariadic`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BareVariadic`

##### `impl<T> Spanned for BareVariadic`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::BareVariadic`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

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

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Type`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Type`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Type`

##### `impl Hash for crate::Type`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::Type`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Type`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Type`

##### `impl<T> Spanned for Type`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Type`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `ReturnType`

```rust
enum ReturnType {
    Default,
    Type($crate::token::RArrow, Box<Type>),
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

- `fn without_plus(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

- `fn parse(input: ParseStream<'_>, allow_plus: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::ReturnType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ReturnType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ReturnType`

##### `impl Hash for crate::ReturnType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::ty::ReturnType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::ReturnType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ReturnType`

##### `impl<T> Spanned for ReturnType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::ty::ReturnType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

