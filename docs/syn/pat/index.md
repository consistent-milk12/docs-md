*[syn](../index.md) / [pat](index.md)*

---

# Module `pat`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: $crate::token::Const,
    pub block: crate::stmt::Block,
}
```

A const block: `const { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprConst`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprConst`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl Hash for crate::ExprConst`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprConst`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprConst`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprConst`

##### `impl<T> Spanned for ExprConst`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprConst`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl Hash for crate::ExprLit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLit`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprLit`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLit`

##### `impl<T> Spanned for ExprLit`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLit`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl Hash for crate::ExprMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMacro`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMacro`

##### `impl<T> Spanned for ExprMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatPath`

```rust
struct PatPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprPath`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprPath`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl Hash for crate::ExprPath`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprPath`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprPath`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprPath`

##### `impl<T> Spanned for ExprPath`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprPath`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatRange`

```rust
struct PatRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRange`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRange`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl Hash for crate::ExprRange`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRange`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprRange`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRange`

##### `impl<T> Spanned for ExprRange`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRange`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatIdent`

```rust
struct PatIdent {
    pub attrs: Vec<crate::attr::Attribute>,
    pub by_ref: Option<$crate::token::Ref>,
    pub mutability: Option<$crate::token::Mut>,
    pub ident: crate::ident::Ident,
    pub subpat: Option<($crate::token::At, Box<Pat>)>,
}
```

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatIdent`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatIdent`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl Hash for crate::PatIdent`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatIdent`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatIdent`

##### `impl<T> Spanned for PatIdent`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatIdent`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<$crate::token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, $crate::token::Or>,
}
```

A pattern that matches any one of a set of cases.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatOr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatOr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl Hash for crate::PatOr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatOr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatOr`

##### `impl<T> Spanned for PatOr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatOr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

A parenthesized pattern: `(A | B)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl Hash for crate::PatParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatParen`

##### `impl<T> Spanned for PatParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatReference`

```rust
struct PatReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub mutability: Option<$crate::token::Mut>,
    pub pat: Box<Pat>,
}
```

A reference pattern: `&mut var`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl Hash for crate::PatReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatReference`

##### `impl<T> Spanned for PatReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: $crate::token::DotDot,
}
```

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatRest`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatRest`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl Hash for crate::PatRest`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatRest`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatRest`

##### `impl<T> Spanned for PatRest`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatRest`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatSlice`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatSlice`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl Hash for crate::PatSlice`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatSlice`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatSlice`

##### `impl<T> Spanned for PatSlice`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatSlice`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatStruct`

```rust
struct PatStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldPat, $crate::token::Comma>,
    pub rest: Option<PatRest>,
}
```

A struct or struct variant pattern: `Variant { x, y, .. }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl Hash for crate::PatStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatStruct`

##### `impl<T> Spanned for PatStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A tuple pattern: `(a, b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl Hash for crate::PatTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTuple`

##### `impl<T> Spanned for PatTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatTupleStruct`

```rust
struct PatTupleStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, $crate::token::Comma>,
}
```

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatTupleStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatTupleStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl Hash for crate::PatTupleStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatTupleStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatTupleStruct`

##### `impl<T> Spanned for PatTupleStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatType`

```rust
struct PatType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Box<Pat>,
    pub colon_token: $crate::token::Colon,
    pub ty: Box<crate::ty::Type>,
}
```

A type ascription pattern: `foo: f64`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatType`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatType`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl Hash for crate::PatType`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::pat::PatType`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::PatType`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatType`

##### `impl<T> Spanned for PatType`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatType`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

A pattern that matches any value: `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::PatWild`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PatWild`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl Hash for crate::PatWild`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::PatWild`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PatWild`

##### `impl<T> Spanned for PatWild`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::PatWild`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<$crate::token::Colon>,
    pub pat: Box<Pat>,
}
```

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

#### Trait Implementations

##### `impl Clone for crate::FieldPat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldPat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl Hash for crate::FieldPat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::FieldPat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldPat`

##### `impl<T> Spanned for FieldPat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::pat::FieldPat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `Pat`

```rust
enum Pat {
    Const(PatConst),
    Ident(PatIdent),
    Lit(PatLit),
    Macro(PatMacro),
    Or(PatOr),
    Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Rest(PatRest),
    Slice(PatSlice),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Type(PatType),
    Verbatim(proc_macro2::TokenStream),
    Wild(PatWild),
}
```

A pattern in a local binding, function signature, match expression, or
various other places.

# Syntax tree enum

This type is a [syntax tree enum].


#### Variants

- **`Const`**

  A const block: `const { ... }`.

- **`Ident`**

  A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

- **`Lit`**

  A literal pattern: `0`.

- **`Macro`**

  A macro in pattern position.

- **`Or`**

  A pattern that matches any one of a set of cases.

- **`Paren`**

  A parenthesized pattern: `(A | B)`.

- **`Path`**

  A path pattern like `Color::Red`, optionally qualified with a
  self-type.
  
  Unqualified path patterns can legally refer to variants, structs,
  constants or associated constants. Qualified path patterns like
  `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
  associated constants.

- **`Range`**

  A range pattern: `1..=2`.

- **`Reference`**

  A reference pattern: `&mut var`.

- **`Rest`**

  The dots in a tuple or slice pattern: `[0, 1, ..]`.

- **`Slice`**

  A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

- **`Struct`**

  A struct or struct variant pattern: `Variant { x, y, .. }`.

- **`Tuple`**

  A tuple pattern: `(a, b)`.

- **`TupleStruct`**

  A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

- **`Type`**

  A type ascription pattern: `foo: f64`.

- **`Verbatim`**

  Tokens in pattern position not interpreted by Syn.

- **`Wild`**

  A pattern that matches any value: `_`.

#### Implementations

- `fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

- `fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

- `fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::Pat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Pat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl Hash for crate::Pat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Pat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Pat`

##### `impl<T> Spanned for Pat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Pat`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

