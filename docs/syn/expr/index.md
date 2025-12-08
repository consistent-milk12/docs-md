*[syn](../index.md) / [expr](index.md)*

---

# Module `expr`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprArray`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprArray`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl Hash for crate::ExprArray`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprArray`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprArray`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprArray`

##### `impl<T> Spanned for ExprArray`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprArray`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub eq_token: $crate::token::Eq,
    pub right: Box<Expr>,
}
```

An assignment expression: `a = compute()`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAssign`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAssign`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl Hash for crate::ExprAssign`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAssign`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprAssign`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAssign`

##### `impl<T> Spanned for ExprAssign`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAssign`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: Vec<crate::attr::Attribute>,
    pub async_token: $crate::token::Async,
    pub capture: Option<$crate::token::Move>,
    pub block: crate::stmt::Block,
}
```

An async block: `async { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAsync`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAsync`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl Hash for crate::ExprAsync`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAsync`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprAsync`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAsync`

##### `impl<T> Spanned for ExprAsync`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAsync`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub await_token: $crate::token::Await,
}
```

An await expression: `fut.await`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprAwait`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprAwait`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl Hash for crate::ExprAwait`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprAwait`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprAwait`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprAwait`

##### `impl<T> Spanned for ExprAwait`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprAwait`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: Box<Expr>,
}
```

A binary operation: `a + b`, `a += b`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBinary`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBinary`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl Hash for crate::ExprBinary`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBinary`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprBinary`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBinary`

##### `impl<T> Spanned for ExprBinary`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBinary`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

A blocked scope: `{ ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBlock`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBlock`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl Hash for crate::ExprBlock`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBlock`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprBlock`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBlock`

##### `impl<T> Spanned for ExprBlock`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBlock`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: Vec<crate::attr::Attribute>,
    pub break_token: $crate::token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<Box<Expr>>,
}
```

A `break`, with an optional label to break and an optional
expression.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprBreak`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprBreak`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl Hash for crate::ExprBreak`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprBreak`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprBreak`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprBreak`

##### `impl<T> Spanned for ExprBreak`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprBreak`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub func: Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A function call expression: `invoke(a, b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCall`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprCall`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl Hash for crate::ExprCall`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCall`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprCall`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCall`

##### `impl<T> Spanned for ExprCall`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprCall`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub as_token: $crate::token::As,
    pub ty: Box<crate::ty::Type>,
}
```

A cast expression: `foo as f64`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprCast`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprCast`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl Hash for crate::ExprCast`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprCast`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprCast`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprCast`

##### `impl<T> Spanned for ExprCast`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprCast`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprClosure`

```rust
struct ExprClosure {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub constness: Option<$crate::token::Const>,
    pub movability: Option<$crate::token::Static>,
    pub asyncness: Option<$crate::token::Async>,
    pub capture: Option<$crate::token::Move>,
    pub or1_token: $crate::token::Or,
    pub inputs: crate::punctuated::Punctuated<crate::pat::Pat, $crate::token::Comma>,
    pub or2_token: $crate::token::Or,
    pub output: crate::ty::ReturnType,
    pub body: Box<Expr>,
}
```

A closure expression: `|a, b| a + b`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprClosure`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprClosure`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl Hash for crate::ExprClosure`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprClosure`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprClosure`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprClosure`

##### `impl<T> Spanned for ExprClosure`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprClosure`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprConst`

```rust
struct ExprConst {
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

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: $crate::token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

A `continue`, with an optional label.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprContinue`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprContinue`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl Hash for crate::ExprContinue`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprContinue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprContinue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprContinue`

##### `impl<T> Spanned for ExprContinue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprContinue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprField`

```rust
struct ExprField {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub member: Member,
}
```

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprField`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprField`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl Hash for crate::ExprField`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprField`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprField`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprField`

##### `impl<T> Spanned for ExprField`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprField`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprForLoop`

```rust
struct ExprForLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub for_token: $crate::token::For,
    pub pat: Box<crate::pat::Pat>,
    pub in_token: $crate::token::In,
    pub expr: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A for loop: `for pat in expr { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprForLoop`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprForLoop`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl Hash for crate::ExprForLoop`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprForLoop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprForLoop`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprForLoop`

##### `impl<T> Spanned for ExprForLoop`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprForLoop`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: Box<Expr>,
}
```

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprGroup`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprGroup`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl Hash for crate::ExprGroup`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::ExprGroup`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprGroup`

##### `impl<T> Spanned for ExprGroup`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprGroup`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprIf`

```rust
struct ExprIf {
    pub attrs: Vec<crate::attr::Attribute>,
    pub if_token: $crate::token::If,
    pub cond: Box<Expr>,
    pub then_branch: crate::stmt::Block,
    pub else_branch: Option<($crate::token::Else, Box<Expr>)>,
}
```

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIf`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprIf`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl Hash for crate::ExprIf`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIf`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprIf`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIf`

##### `impl<T> Spanned for ExprIf`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprIf`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: Box<Expr>,
}
```

A square bracketed indexing expression: `vector[2]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprIndex`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprIndex`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl Hash for crate::ExprIndex`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprIndex`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprIndex`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprIndex`

##### `impl<T> Spanned for ExprIndex`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprIndex`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: $crate::token::Underscore,
}
```

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprInfer`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprInfer`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl Hash for crate::ExprInfer`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprInfer`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprInfer`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprInfer`

##### `impl<T> Spanned for ExprInfer`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprInfer`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprLet`

```rust
struct ExprLet {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: $crate::token::Let,
    pub pat: Box<crate::pat::Pat>,
    pub eq_token: $crate::token::Eq,
    pub expr: Box<Expr>,
}
```

A `let` guard: `let Some(x) = opt`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLet`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLet`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl Hash for crate::ExprLet`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLet`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprLet`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLet`

##### `impl<T> Spanned for ExprLet`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLet`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprLit`

```rust
struct ExprLit {
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

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: $crate::token::Loop,
    pub body: crate::stmt::Block,
}
```

Conditionless loop: `loop { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprLoop`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprLoop`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl Hash for crate::ExprLoop`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprLoop`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprLoop`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprLoop`

##### `impl<T> Spanned for ExprLoop`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprLoop`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprMacro`

```rust
struct ExprMacro {
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

### `ExprMatch`

```rust
struct ExprMatch {
    pub attrs: Vec<crate::attr::Attribute>,
    pub match_token: $crate::token::Match,
    pub expr: Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: Vec<Arm>,
}
```

A `match` expression: `match n { Some(n) => {}, None => {} }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMatch`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMatch`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl Hash for crate::ExprMatch`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMatch`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprMatch`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMatch`

##### `impl<T> Spanned for ExprMatch`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMatch`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprMethodCall`

```rust
struct ExprMethodCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub receiver: Box<Expr>,
    pub dot_token: $crate::token::Dot,
    pub method: crate::ident::Ident,
    pub turbofish: Option<crate::path::AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A method call expression: `x.foo::<T>(a, b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprMethodCall`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprMethodCall`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl Hash for crate::ExprMethodCall`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprMethodCall`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprMethodCall`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprMethodCall`

##### `impl<T> Spanned for ExprMethodCall`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

A parenthesized expression: `(a + b)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprParen`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprParen`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl Hash for crate::ExprParen`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprParen`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprParen`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprParen`

##### `impl<T> Spanned for ExprParen`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprParen`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprPath`

```rust
struct ExprPath {
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

### `ExprRange`

```rust
struct ExprRange {
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

### `ExprRawAddr`

```rust
struct ExprRawAddr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub raw: $crate::token::Raw,
    pub mutability: PointerMutability,
    pub expr: Box<Expr>,
}
```

Address-of operation: `&raw const place` or `&raw mut place`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRawAddr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRawAddr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl Hash for crate::ExprRawAddr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRawAddr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprRawAddr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRawAddr`

##### `impl<T> Spanned for ExprRawAddr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: $crate::token::And,
    pub mutability: Option<$crate::token::Mut>,
    pub expr: Box<Expr>,
}
```

A referencing operation: `&a` or `&mut a`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReference`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprReference`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl Hash for crate::ExprReference`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReference`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprReference`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReference`

##### `impl<T> Spanned for ExprReference`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprReference`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprRepeat`

```rust
struct ExprRepeat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub expr: Box<Expr>,
    pub semi_token: $crate::token::Semi,
    pub len: Box<Expr>,
}
```

An array literal constructed from one repeated element: `[0u8; N]`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprRepeat`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprRepeat`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl Hash for crate::ExprRepeat`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprRepeat`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprRepeat`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprRepeat`

##### `impl<T> Spanned for ExprRepeat`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprRepeat`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: $crate::token::Return,
    pub expr: Option<Box<Expr>>,
}
```

A `return`, with an optional value to be returned.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprReturn`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprReturn`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl Hash for crate::ExprReturn`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprReturn`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprReturn`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprReturn`

##### `impl<T> Spanned for ExprReturn`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprReturn`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprStruct`

```rust
struct ExprStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldValue, $crate::token::Comma>,
    pub dot2_token: Option<$crate::token::DotDot>,
    pub rest: Option<Box<Expr>>,
}
```

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprStruct`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprStruct`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl Hash for crate::ExprStruct`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprStruct`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprStruct`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprStruct`

##### `impl<T> Spanned for ExprStruct`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprStruct`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: $crate::token::Question,
}
```

A try-expression: `expr?`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTry`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTry`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl Hash for crate::ExprTry`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTry`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprTry`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTry`

##### `impl<T> Spanned for ExprTry`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTry`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: $crate::token::Try,
    pub block: crate::stmt::Block,
}
```

A try block: `try { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTryBlock`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTryBlock`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl Hash for crate::ExprTryBlock`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTryBlock`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprTryBlock`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTryBlock`

##### `impl<T> Spanned for ExprTryBlock`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, $crate::token::Comma>,
}
```

A tuple expression: `(a, b, c, d)`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprTuple`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprTuple`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl Hash for crate::ExprTuple`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprTuple`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprTuple`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprTuple`

##### `impl<T> Spanned for ExprTuple`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprTuple`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

A unary operation: `!x`, `*x`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnary`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprUnary`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl Hash for crate::ExprUnary`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnary`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprUnary`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnary`

##### `impl<T> Spanned for ExprUnary`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnary`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: $crate::token::Unsafe,
    pub block: crate::stmt::Block,
}
```

An unsafe block: `unsafe { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprUnsafe`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprUnsafe`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl Hash for crate::ExprUnsafe`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprUnsafe`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprUnsafe`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprUnsafe`

##### `impl<T> Spanned for ExprUnsafe`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprWhile`

```rust
struct ExprWhile {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub while_token: $crate::token::While,
    pub cond: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

A while loop: `while expr { ... }`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprWhile`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprWhile`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl Hash for crate::ExprWhile`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprWhile`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprWhile`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprWhile`

##### `impl<T> Spanned for ExprWhile`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprWhile`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: $crate::token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

A yield expression: `yield expr`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::ExprYield`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::ExprYield`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl Hash for crate::ExprYield`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::ExprYield`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::ExprYield`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for ExprYield`

##### `impl<T> Spanned for ExprYield`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::ExprYield`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Clone for crate::Index`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Index`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl Hash for Index`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment for Index`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse for crate::expr::Index`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for Index`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Index`

##### `impl<T> Spanned for Index`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Index`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<$crate::token::Colon>,
    pub expr: Expr,
}
```

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<$crate::token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

#### Trait Implementations

##### `impl Clone for crate::FieldValue`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::FieldValue`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl Hash for crate::FieldValue`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::FieldValue`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::FieldValue`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for FieldValue`

##### `impl<T> Spanned for FieldValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::FieldValue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: $crate::token::Colon,
}
```

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Clone for crate::Label`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Label`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl Hash for crate::Label`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Label`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Label`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Label`

##### `impl<T> Spanned for Label`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Label`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Arm`

```rust
struct Arm {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: crate::pat::Pat,
    pub guard: Option<($crate::token::If, Box<Expr>)>,
    pub fat_arrow_token: $crate::token::FatArrow,
    pub body: Box<Expr>,
    pub comma: Option<$crate::token::Comma>,
}
```

One arm of a `match` expression: `0..=10 => { return true; }`.

As in:

```rust
fn f() -> bool {
    let n = 0;
match n {
    0..=10 => {
        return true;
    }
    // ...
    _ => {}
}
  false
}
```

#### Implementations

- `fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::Arm`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Arm`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl Hash for crate::Arm`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Arm`

- `fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md), [`Arm`](#arm)

##### `impl PartialEq for crate::Arm`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Arm`

##### `impl<T> Spanned for Arm`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Arm`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `Expr`

```rust
enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    RawAddr(ExprRawAddr),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(proc_macro2::TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
}
```

A Rust expression.

*This type is available only if Syn is built with the `"derive"` or `"full"`
feature, but most of the variants are not available unless "full" is enabled.*

# Syntax tree enums

This type is a syntax tree enum. In Syn this and other syntax tree enums
are designed to be traversed using the following rebinding idiom.

```rust
use syn::Expr;

fn example(expr: Expr) {
const IGNORE: &str = stringify! {
let expr: Expr = /* ... */;
};
match expr {
    Expr::MethodCall(expr) => {
        /* ... */
    }
    Expr::Cast(expr) => {
        /* ... */
    }
    Expr::If(expr) => {
        /* ... */
    }

    /* ... */
    _ => {}
}
}
```

We begin with a variable `expr` of type `Expr` that has no fields
(because it is an enum), and by matching on it and rebinding a variable
with the same name `expr` we effectively imbue our variable with all of
the data fields provided by the variant that it turned out to be. So for
example above if we ended up in the `MethodCall` case then we get to use
`expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.

This approach avoids repeating the variant names twice on every line.

```rust
use syn::{Expr, ExprMethodCall};

fn example(expr: Expr) {
// Repetitive; recommend not doing this.
match expr {
    Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
}
_ => {}
}
}
```

In general, the name to which a syntax tree enum variant is bound should
be a suitable name for the complete syntax tree enum type.

```rust
use syn::{Expr, ExprField};

fn example(discriminant: ExprField) {
// Binding is called `base` which is the name I would use if I were
// assigning `*discriminant.base` without an `if let`.
if let Expr::Tuple(base) = *discriminant.base {
}
}
```

A sign that you may not be choosing the right variable names is if you
see names getting repeated in your code, like accessing
`receiver.receiver` or `pat.pat` or `cond.cond`.

#### Variants

- **`Array`**

  A slice literal expression: `[a, b, c, d]`.

- **`Assign`**

  An assignment expression: `a = compute()`.

- **`Async`**

  An async block: `async { ... }`.

- **`Await`**

  An await expression: `fut.await`.

- **`Binary`**

  A binary operation: `a + b`, `a += b`.

- **`Block`**

  A blocked scope: `{ ... }`.

- **`Break`**

  A `break`, with an optional label to break and an optional
  expression.

- **`Call`**

  A function call expression: `invoke(a, b)`.

- **`Cast`**

  A cast expression: `foo as f64`.

- **`Closure`**

  A closure expression: `|a, b| a + b`.

- **`Const`**

  A const block: `const { ... }`.

- **`Continue`**

  A `continue`, with an optional label.

- **`Field`**

  Access of a named struct field (`obj.k`) or unnamed tuple struct
  field (`obj.0`).

- **`ForLoop`**

  A for loop: `for pat in expr { ... }`.

- **`Group`**

  An expression contained within invisible delimiters.
  
  This variant is important for faithfully representing the precedence
  of expressions and is related to `None`-delimited spans in a
  `TokenStream`.

- **`If`**

  An `if` expression with an optional `else` block: `if expr { ... }
  else { ... }`.
  
  The `else` branch expression may only be an `If` or `Block`
  expression, not any of the other types of expression.

- **`Index`**

  A square bracketed indexing expression: `vector[2]`.

- **`Infer`**

  The inferred value of a const generic argument, denoted `_`.

- **`Let`**

  A `let` guard: `let Some(x) = opt`.

- **`Lit`**

  A literal in place of an expression: `1`, `"foo"`.

- **`Loop`**

  Conditionless loop: `loop { ... }`.

- **`Macro`**

  A macro invocation expression: `format!("{}", q)`.

- **`Match`**

  A `match` expression: `match n { Some(n) => {}, None => {} }`.

- **`MethodCall`**

  A method call expression: `x.foo::<T>(a, b)`.

- **`Paren`**

  A parenthesized expression: `(a + b)`.

- **`Path`**

  A path like `std::mem::replace` possibly containing generic
  parameters and a qualified self-type.
  
  A plain identifier like `x` is a path of length 1.

- **`Range`**

  A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

- **`RawAddr`**

  Address-of operation: `&raw const place` or `&raw mut place`.

- **`Reference`**

  A referencing operation: `&a` or `&mut a`.

- **`Repeat`**

  An array literal constructed from one repeated element: `[0u8; N]`.

- **`Return`**

  A `return`, with an optional value to be returned.

- **`Struct`**

  A struct literal expression: `Point { x: 1, y: 1 }`.
  
  The `rest` provides the value of the remaining fields as in `S { a:
  1, b: 1, ..rest }`.

- **`Try`**

  A try-expression: `expr?`.

- **`TryBlock`**

  A try block: `try { ... }`.

- **`Tuple`**

  A tuple expression: `(a, b, c, d)`.

- **`Unary`**

  A unary operation: `!x`, `*x`.

- **`Unsafe`**

  An unsafe block: `unsafe { ... }`.

- **`Verbatim`**

  Tokens in expression position not interpreted by Syn.

- **`While`**

  A while loop: `while expr { ... }`.

- **`Yield`**

  A yield expression: `yield expr`.

#### Implementations

- `const PLACEHOLDER: Self`

- `fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md), [`Expr`](#expr)

- `fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md), [`Expr`](#expr)

- `fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](../parse/index.md)

- `fn replace_attrs(self: &mut Self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md)

#### Trait Implementations

##### `impl Clone for crate::Expr`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Expr`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl Hash for crate::Expr`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::Expr`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Expr`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Expr`

##### `impl<T> Spanned for Expr`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Expr`

- `fn to_tokens(self: &Self, tokens: &mut ::proc_macro2::TokenStream)`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

A struct or tuple struct field accessed in a struct literal or field
expression.

#### Variants

- **`Named`**

  A named field like `self.x`.

- **`Unnamed`**

  An unnamed field like `self.0`.

#### Implementations

- `fn is_named(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for crate::Member`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Member`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl Hash for Member`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl IdentFragment for Member`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn span(self: &Self) -> Option<Span>`

##### `impl Parse for crate::expr::Member`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for Member`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Member`

##### `impl<T> Spanned for Member`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::Member`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen($crate::token::DotDot),
    Closed($crate::token::DotDotEq),
}
```

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

#### Implementations

- `fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

#### Trait Implementations

##### `impl Clone for crate::RangeLimits`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl Hash for crate::RangeLimits`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::RangeLimits`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::RangeLimits`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for RangeLimits`

##### `impl<T> Spanned for RangeLimits`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::RangeLimits`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `PointerMutability`

```rust
enum PointerMutability {
    Const($crate::token::Const),
    Mut($crate::token::Mut),
}
```

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

#### Trait Implementations

##### `impl Clone for crate::PointerMutability`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::PointerMutability`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl Hash for crate::PointerMutability`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::expr::PointerMutability`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::PointerMutability`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for PointerMutability`

##### `impl<T> Spanned for PointerMutability`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::expr::PointerMutability`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

