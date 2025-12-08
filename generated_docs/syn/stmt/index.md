*[syn](../index.md) / [stmt](index.md)*

---

# Module `stmt`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`Block`](#block)
  - [`Local`](#local)
  - [`LocalInit`](#localinit)
  - [`StmtMacro`](#stmtmacro)
- [Enums](#enums)
  - [`Stmt`](#stmt)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`Block`](#block) | struct | A braced block containing Rust statements. |
| [`Local`](#local) | struct | A local `let` binding: `let x: u64 = s.parse()?;`. |
| [`LocalInit`](#localinit) | struct | The expression assigned in a local `let` binding, including optional |
| [`StmtMacro`](#stmtmacro) | struct | A macro invocation in statement position. |
| [`Stmt`](#stmt) | enum | A statement, usually ending in a semicolon. |

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

- <span id="cratestmtblock-parse-within"></span>`fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md), [`Stmt`](../index.md)

#### Trait Implementations

##### `impl Clone for crate::Block`

- <span id="crateblock-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Block`

- <span id="crateblock-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl Hash for crate::Block`

- <span id="crateblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::stmt::Block`

- <span id="cratestmtblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Block`

- <span id="crateblock-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Block`

##### `impl<T> Spanned for Block`

- <span id="block-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Block`

- <span id="cratestmtblock-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `Local`

```rust
struct Local {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: token::Let,
    pub pat: crate::pat::Pat,
    pub init: Option<LocalInit>,
    pub semi_token: token::Semi,
}
```

A local `let` binding: `let x: u64 = s.parse()?;`.

#### Implementations

- <span id="cratelocal-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::Local`

- <span id="cratelocal-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Local`

- <span id="cratelocal-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl Hash for crate::Local`

- <span id="cratelocal-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::Local`

- <span id="cratelocal-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Local`

##### `impl<T> Spanned for Local`

- <span id="local-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Local`

- <span id="cratestmtlocal-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<(token::Else, Box<crate::expr::Expr>)>,
}
```

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

#### Trait Implementations

##### `impl Clone for crate::LocalInit`

- <span id="cratelocalinit-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::LocalInit`

- <span id="cratelocalinit-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl Hash for crate::LocalInit`

- <span id="cratelocalinit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::LocalInit`

- <span id="cratelocalinit-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

#### Implementations

- <span id="cratestmtmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Clone for crate::StmtMacro`

- <span id="cratestmtmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::StmtMacro`

- <span id="cratestmtmacro-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl Hash for crate::StmtMacro`

- <span id="cratestmtmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for crate::StmtMacro`

- <span id="cratestmtmacro-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for StmtMacro`

##### `impl<T> Spanned for StmtMacro`

- <span id="stmtmacro-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::StmtMacro`

- <span id="cratestmtstmtmacro-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `Stmt`

```rust
enum Stmt {
    Local(Local),
    Item(crate::item::Item),
    Expr(crate::expr::Expr, Option<token::Semi>),
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

- <span id="cratestmt-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for crate::Stmt`

- <span id="cratestmt-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl Hash for crate::Stmt`

- <span id="cratestmt-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::stmt::Stmt`

- <span id="cratestmtstmt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../index.md)

##### `impl PartialEq for crate::Stmt`

- <span id="cratestmt-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> Sealed for Stmt`

##### `impl<T> Spanned for Stmt`

- <span id="stmt-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::stmt::Stmt`

- <span id="cratestmtstmt-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

