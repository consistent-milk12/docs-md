*[syn](../index.md) / [path](index.md)*

---

# Module `path`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Path`](#path)
  - [`PathSegment`](#pathsegment)
  - [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments)
  - [`AssocType`](#assoctype)
  - [`AssocConst`](#assocconst)
  - [`Constraint`](#constraint)
  - [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments)
  - [`QSelf`](#qself)
- [Enums](#enums)
  - [`PathArguments`](#patharguments)
  - [`GenericArgument`](#genericargument)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Path`](#path) | struct | A path at which a named item is exported (e.g. `std::collections::HashMap`). |
| [`PathSegment`](#pathsegment) | struct | A segment of a path together with any path arguments on that segment. |
| [`AngleBracketedGenericArguments`](#anglebracketedgenericarguments) | struct | Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K |
| [`AssocType`](#assoctype) | struct | A binding (equality constraint) on an associated type: the `Item = u8` |
| [`AssocConst`](#assocconst) | struct | An equality constraint on an associated constant: the `PANIC = false` in |
| [`Constraint`](#constraint) | struct | An associated type bound: `Iterator<Item: Display>`. |
| [`ParenthesizedGenericArguments`](#parenthesizedgenericarguments) | struct | Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) -> |
| [`QSelf`](#qself) | struct | The explicit Self type in a qualified path: the `T` in `<T as |
| [`PathArguments`](#patharguments) | enum | Angle bracketed or parenthesized arguments of a path segment. |
| [`GenericArgument`](#genericargument) | enum | An individual generic argument, like `'a`, `T`, or `Item = T`. |

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Path`

```rust
struct Path {
    pub leading_colon: Option<token::PathSep>,
    pub segments: crate::punctuated::Punctuated<PathSegment, token::PathSep>,
}
```

A path at which a named item is exported (e.g. `std::collections::HashMap`).

#### Implementations

- <span id="cratepath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Path`

- <span id="cratepath-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Path`

- <span id="cratepath-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Path`

##### `impl Hash for crate::Path`

- <span id="cratepath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::Path`

- <span id="cratepathpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Path`

- <span id="cratepath-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialEq for syn::Path`

##### `impl<T> Sealed for Path`

##### `impl<T> Spanned for Path`

- <span id="path-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Path`

- <span id="cratepathpath-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `PathSegment`

```rust
struct PathSegment {
    pub ident: crate::ident::Ident,
    pub arguments: PathArguments,
}
```

A segment of a path together with any path arguments on that segment.

#### Implementations

- <span id="cratepathpathsegment-parse-helper"></span>`fn parse_helper(input: ParseStream<'_>, expr_style: bool) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::PathSegment`

- <span id="cratepathsegment-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathSegment`

- <span id="cratepathsegment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PathSegment`

##### `impl Hash for crate::PathSegment`

- <span id="cratepathsegment-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::PathSegment`

- <span id="cratepathpathsegment-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::PathSegment`

- <span id="cratepathsegment-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PathSegment`

##### `impl<T> Spanned for PathSegment`

- <span id="pathsegment-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathSegment`

- <span id="cratepathpathsegment-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AngleBracketedGenericArguments`

```rust
struct AngleBracketedGenericArguments {
    pub colon2_token: Option<token::PathSep>,
    pub lt_token: token::Lt,
    pub args: crate::punctuated::Punctuated<GenericArgument, token::Comma>,
    pub gt_token: token::Gt,
}
```

Angle bracketed arguments of a path segment: the `<K, V>` in `HashMap<K,
V>`.

#### Implementations

- <span id="crateanglebracketedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AngleBracketedGenericArguments`

##### `impl Hash for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::AngleBracketedGenericArguments`

- <span id="crateanglebracketedgenericarguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AngleBracketedGenericArguments`

##### `impl<T> Spanned for AngleBracketedGenericArguments`

- <span id="anglebracketedgenericarguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AngleBracketedGenericArguments`

- <span id="cratepathanglebracketedgenericarguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocType`

```rust
struct AssocType {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub ty: crate::ty::Type,
}
```

A binding (equality constraint) on an associated type: the `Item = u8`
in `Iterator<Item = u8>`.

#### Trait Implementations

##### `impl Clone for crate::AssocType`

- <span id="crateassoctype-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocType`

- <span id="crateassoctype-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocType`

##### `impl Hash for crate::AssocType`

- <span id="crateassoctype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocType`

- <span id="crateassoctype-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocType`

##### `impl<T> Spanned for AssocType`

- <span id="assoctype-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocType`

- <span id="cratepathassoctype-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `AssocConst`

```rust
struct AssocConst {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub eq_token: token::Eq,
    pub value: crate::expr::Expr,
}
```

An equality constraint on an associated constant: the `PANIC = false` in
`Trait<PANIC = false>`.

#### Trait Implementations

##### `impl Clone for crate::AssocConst`

- <span id="crateassocconst-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::AssocConst`

- <span id="crateassocconst-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::AssocConst`

##### `impl Hash for crate::AssocConst`

- <span id="crateassocconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::AssocConst`

- <span id="crateassocconst-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for AssocConst`

##### `impl<T> Spanned for AssocConst`

- <span id="assocconst-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::AssocConst`

- <span id="cratepathassocconst-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Constraint`

```rust
struct Constraint {
    pub ident: crate::ident::Ident,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub colon_token: token::Colon,
    pub bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
}
```

An associated type bound: `Iterator<Item: Display>`.

#### Trait Implementations

##### `impl Clone for crate::Constraint`

- <span id="crateconstraint-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Constraint`

- <span id="crateconstraint-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Constraint`

##### `impl Hash for crate::Constraint`

- <span id="crateconstraint-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Constraint`

- <span id="crateconstraint-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Constraint`

##### `impl<T> Spanned for Constraint`

- <span id="constraint-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::Constraint`

- <span id="cratepathconstraint-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `ParenthesizedGenericArguments`

```rust
struct ParenthesizedGenericArguments {
    pub paren_token: token::Paren,
    pub inputs: crate::punctuated::Punctuated<crate::ty::Type, token::Comma>,
    pub output: crate::ty::ReturnType,
}
```

Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->
C`.

#### Fields

- **`inputs`**: `crate::punctuated::Punctuated<crate::ty::Type, token::Comma>`

  `(A, B)`

- **`output`**: `crate::ty::ReturnType`

  `C`

#### Implementations

- <span id="crateparenthesizedgenericarguments-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ParenthesizedGenericArguments`

##### `impl Hash for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::ParenthesizedGenericArguments`

- <span id="crateparenthesizedgenericarguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for ParenthesizedGenericArguments`

##### `impl<T> Spanned for ParenthesizedGenericArguments`

- <span id="parenthesizedgenericarguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::ParenthesizedGenericArguments`

- <span id="cratepathparenthesizedgenericarguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `QSelf`

```rust
struct QSelf {
    pub lt_token: token::Lt,
    pub ty: Box<crate::ty::Type>,
    pub position: usize,
    pub as_token: Option<token::As>,
    pub gt_token: token::Gt,
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

- <span id="crateqself-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::QSelf`

- <span id="crateqself-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::QSelf`

##### `impl Hash for crate::QSelf`

- <span id="crateqself-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::QSelf`

- <span id="crateqself-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for crate::QSelf`

##### `impl Spanned for crate::path::QSelf`

- <span id="cratepathqself-span"></span>`fn span(&self) -> Span`

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

- <span id="patharguments-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patharguments-is-none"></span>`fn is_none(&self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::PathArguments`

- <span id="cratepatharguments-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::PathArguments`

- <span id="cratepatharguments-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathArguments`

- <span id="patharguments-default"></span>`fn default() -> Self`

##### `impl Eq for crate::PathArguments`

##### `impl Hash for crate::PathArguments`

- <span id="cratepatharguments-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::PathArguments`

- <span id="cratepatharguments-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for PathArguments`

##### `impl<T> Spanned for PathArguments`

- <span id="patharguments-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::PathArguments`

- <span id="cratepathpatharguments-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

- <span id="crategenericargument-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::GenericArgument`

- <span id="crategenericargument-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::GenericArgument`

##### `impl Hash for crate::GenericArgument`

- <span id="crategenericargument-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::path::GenericArgument`

- <span id="cratepathgenericargument-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::GenericArgument`

- <span id="crategenericargument-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for GenericArgument`

##### `impl<T> Spanned for GenericArgument`

- <span id="genericargument-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::path::GenericArgument`

- <span id="cratepathgenericargument-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

