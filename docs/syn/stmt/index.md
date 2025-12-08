*[syn](../index.md) / [stmt](index.md)*

---

# Module `stmt`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Structs

### `Block`

```rust
struct Block {
    pub brace_token: token::Brace,
    pub stmts: Vec<Stmt>,
}
```

A braced block containing Rust statements.

#### Fields

- **`stmts`**: `Vec<Stmt>`

  Statements in a block

#### Implementations

- `fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md), [`Stmt`](#stmt)

#### Trait Implementations

##### `impl Clone for crate::Block`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Block`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl Hash for crate::Block`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::stmt::Block`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Block`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Block`

##### `impl<T> Spanned for Block`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Block`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `Local`

```rust
struct Local {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: $crate::token::Let,
    pub pat: crate::pat::Pat,
    pub init: Option<LocalInit>,
    pub semi_token: $crate::token::Semi,
}
```

A local `let` binding: `let x: u64 = s.parse()?;`.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Local`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Local`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl Hash for crate::Local`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::Local`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Local`

##### `impl<T> Spanned for Local`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Local`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: $crate::token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<($crate::token::Else, Box<crate::expr::Expr>)>,
}
```

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

#### Trait Implementations

##### `impl Clone for crate::LocalInit`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::LocalInit`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl Hash for crate::LocalInit`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::LocalInit`

- `fn eq(self: &Self, other: &Self) -> bool`

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<$crate::token::Semi>,
}
```

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

#### Implementations

- `fn debug(self: &Self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::StmtMacro`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::StmtMacro`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl Hash for crate::StmtMacro`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq for crate::StmtMacro`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for StmtMacro`

##### `impl<T> Spanned for StmtMacro`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::StmtMacro`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Enums

### `Stmt`

```rust
enum Stmt {
    Local(Local),
    Item(crate::item::Item),
    Expr(crate::expr::Expr, Option<$crate::token::Semi>),
    Macro(StmtMacro),
}
```

A statement, usually ending in a semicolon.

#### Variants

- **`Local`**

  A local (let) binding.

- **`Item`**

  An item definition.

- **`Expr`**

  Expression, with or without trailing semicolon.

- **`Macro`**

  A macro invocation in statement position.
  
  Syntactically it's ambiguous which other kind of statement this
  macro would expand to. It can be any of local variable (`let`),
  item, or expression.

#### Trait Implementations

##### `impl Clone for crate::Stmt`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for crate::Stmt`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl Hash for crate::Stmt`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::stmt::Stmt`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::Stmt`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for Stmt`

##### `impl<T> Spanned for Stmt`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::stmt::Stmt`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

