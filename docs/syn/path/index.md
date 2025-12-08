*[syn](../index.md) / [path](index.md)*

---

# Module `path`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Path`

```rust
struct Path {
    pub leading_colon: Option<$crate::token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, $crate::token::PathSep>,
}
```

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- `fn is_ident<I>(self: &Self, ident: &I) -> bool`

- `fn get_ident(self: &Self) -> Option<&Ident>`

- `fn require_ident(self: &Self) -> Result<&Ident>` — [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::Path`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Path`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl Hash for crate::Path`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::Path`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Path`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl<T> Sealed for Path`

##### `impl<T> Spanned for Path`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::Path`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Implementations

- `fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::PathSegment`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PathSegment`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl Hash for crate::PathSegment`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::PathSegment`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::PathSegment`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PathSegment`

##### `impl<T> Spanned for PathSegment`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::PathSegment`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<$crate::token::PathSep>,
    pub lt_token: $crate::token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, $crate::token::Comma>,
    pub gt_token: $crate::token::Gt,
}
```

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- `fn parse_turbofish(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

- `fn do_parse(colon2_token: Option<$crate::token::PathSep>, input: ParseStream<'_>) -> Result<Self>` — [`PathSep`](../token/index.md), [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::AngleBracketedGenericArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl Hash for crate::AngleBracketedGenericArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AngleBracketedGenericArguments`

##### `impl<T> Spanned for AngleBracketedGenericArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: $crate::token::Eq,
    pub ty: crate::ty::Type,
}
```

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Clone for crate::AssocType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AssocType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl Hash for crate::AssocType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::AssocType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocType`

##### `impl<T> Spanned for AssocType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AssocType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: $crate::token::Eq,
    pub value: crate::expr::Expr,
}
```

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Clone for crate::AssocConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::AssocConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl Hash for crate::AssocConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::AssocConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocConst`

##### `impl<T> Spanned for AssocConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::AssocConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: $crate::token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, $crate::token::Plus>,
}
```

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::Constraint`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Constraint`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl Hash for crate::Constraint`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Constraint`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Constraint`

##### `impl<T> Spanned for Constraint`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::Constraint`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, $crate::token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, $crate::token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ParenthesizedGenericArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl Hash for crate::ParenthesizedGenericArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ParenthesizedGenericArguments`

##### `impl<T> Spanned for ParenthesizedGenericArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: $crate::token::Lt,
    pub ty: Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<$crate::token::As>,
    pub gt_token: $crate::token::Gt,
}
```

The explicit Self type in a qualified path: the `T` in `<T as
Display>::fmt`.

The actual path, including the trait and the associated item, is stored
separately. The `position` field represents the index of the associated
item qualified with this Self type.

```text
<Vec<T> as a::b::Trait>::AssociatedItem
 ^~~~~~    ~~~~~~~~~~~~~~^
 ty        position = 3

<Vec<T>>::AssociatedItem
 ^~~~~~   ^
 ty       position = 0
```

#### Trait Implementations

##### `impl Clone for crate::QSelf`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::QSelf`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl Hash for crate::QSelf`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::QSelf`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- `fn span(self: &Self) -> Span`

## Enums

### `PathArguments`

```rust
enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
```

Angle bracketed or parenthesized arguments of a path segment.

## Angle bracketed

The `<'a, T>` in `std::slice::iter<'a, T>`.

## Parenthesized

The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Variants

- **`AngleBracketed`**

  The `<'a, T>` in `std::slice::iter<'a, T>`.

- **`Parenthesized`**

  The `(A, B) -> C` in `Fn(A, B) -> C`.

#### Implementations

- `fn is_empty(self: &Self) -> bool`

- `fn is_none(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::PathArguments`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PathArguments`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- `fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl Hash for crate::PathArguments`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PathArguments`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PathArguments`

##### `impl<T> Spanned for PathArguments`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::PathArguments`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `GenericArgument`

```rust
enum GenericArgument {
    Lifetime(crate::lifetime::Lifetime),
    Type(crate::ty::Type),
    Const(crate::expr::Expr),
    AssocType(AssocType),
    AssocConst(AssocConst),
    Constraint(Constraint),
}
```

An individual generic argument, like `'a`, `T`, or `Item = T`.

#### Variants

- **`Lifetime`**

  A lifetime argument.

- **`Type`**

  A type argument.

- **`Const`**

  A const expression. Must be inside of a block.
  
  NOTE: Identity expressions are represented as Type arguments, as
  they are indistinguishable syntactically.

- **`AssocType`**

  A binding (equality constraint) on an associated type: the `Item =
  u8` in `Iterator<Item = u8>`.

- **`AssocConst`**

  An equality constraint on an associated constant: the `PANIC =
  false` in `Trait<PANIC = false>`.

- **`Constraint`**

  An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::GenericArgument`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::GenericArgument`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl Hash for crate::GenericArgument`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::path::GenericArgument`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::GenericArgument`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericArgument`

##### `impl<T> Spanned for GenericArgument`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::path::GenericArgument`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

