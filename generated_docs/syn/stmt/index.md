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
| [`LocalInit`](#localinit) | struct | The expression assigned in a local `let` binding, including optional diverging `else` block. |
| [`StmtMacro`](#stmtmacro) | struct | A macro invocation in statement position. |
| [`Stmt`](#stmt) | enum | A statement, usually ending in a semicolon. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `Block`

```rust
struct Block {
    pub brace_token: token::Brace,
    pub stmts: Vec<Stmt>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:8-16`](../../../.source_1765633015/syn-2.0.111/src/stmt.rs#L8-L16)*

A braced block containing Rust statements.

#### Fields

- **`stmts`**: `Vec<Stmt>`

  Statements in a block

#### Implementations

- <span id="cratestmtblock-parse-within"></span>`fn parse_within(input: ParseStream<'_>) -> Result<Vec<Stmt>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Stmt`](#stmt)

  Parse the body of a block as zero or more statements, possibly

  including one trailing expression.

  

  # Example

  

  ```rust

  use syn::{braced, token, Attribute, Block, Ident, Result, Stmt, Token};

  use syn::parse::{Parse, ParseStream};

  

  // Parse a function with no generics or parameter list.

  //

  //     fn playground {

  //         let mut x = 1;

  //         x += 1;

  //         println!("{}", x);

  //     }

  struct MiniFunction {

      attrs: Vec<Attribute>,

      fn_token: Token![fn],

      name: Ident,

      brace_token: token::Brace,

      stmts: Vec<Stmt>,

  }

  

  impl Parse for MiniFunction {

      fn parse(input: ParseStream) -> Result<Self> {

          let outer_attrs = input.call(Attribute::parse_outer)?;

          let fn_token: Token![fn] = input.parse()?;

          let name: Ident = input.parse()?;

  

          let content;

          let brace_token = braced!(content in input);

          let inner_attrs = content.call(Attribute::parse_inner)?;

          let stmts = content.call(Block::parse_within)?;

  

          Ok(MiniFunction {

              attrs: {

                  let mut attrs = outer_attrs;

                  attrs.extend(inner_attrs);

                  attrs

              },

              fn_token,

              name,

              brace_token,

              stmts,

          })

      }

  }

  ```

#### Trait Implementations

##### `impl Any for Block`

- <span id="block-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Block`

- <span id="block-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Block`

- <span id="block-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Block`

- <span id="crateblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Block`

- <span id="block-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Block`

- <span id="crateblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Block`

##### `impl<T> From for Block`

- <span id="block-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Block`

- <span id="crateblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Block`

- <span id="block-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::stmt::Block`

- <span id="cratestmtblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Block`

- <span id="crateblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Block`

##### `impl Spanned for Block`

- <span id="block-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Block`

- <span id="block-toowned-type-owned"></span>`type Owned = T`

- <span id="block-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="block-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Block`

- <span id="cratestmtblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Block`

- <span id="block-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="block-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Block`

- <span id="block-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="block-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/stmt.rs:40-50`](../../../.source_1765633015/syn-2.0.111/src/stmt.rs#L40-L50)*

A local `let` binding: `let x: u64 = s.parse()?;`.

#### Implementations

- <span id="cratelocal-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Local`

- <span id="local-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Local`

- <span id="local-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Local`

- <span id="local-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Local`

- <span id="cratelocal-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Local`

- <span id="local-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Local`

- <span id="cratelocal-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Local`

##### `impl<T> From for Local`

- <span id="local-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Local`

- <span id="cratelocal-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Local`

- <span id="local-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Local`

- <span id="cratelocal-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Local`

##### `impl Spanned for Local`

- <span id="local-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Local`

- <span id="local-toowned-type-owned"></span>`type Owned = T`

- <span id="local-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="local-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Local`

- <span id="cratestmtlocal-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Local`

- <span id="local-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="local-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Local`

- <span id="local-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="local-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocalInit`

```rust
struct LocalInit {
    pub eq_token: token::Eq,
    pub expr: Box<crate::expr::Expr>,
    pub diverge: Option<(token::Else, Box<crate::expr::Expr>)>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:52-64`](../../../.source_1765633015/syn-2.0.111/src/stmt.rs#L52-L64)*

The expression assigned in a local `let` binding, including optional
diverging `else` block.

`LocalInit` represents `= s.parse()?` in `let x: u64 = s.parse()?` and
`= r else { return }` in `let Ok(x) = r else { return }`.

#### Trait Implementations

##### `impl Any for LocalInit`

- <span id="localinit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocalInit`

- <span id="localinit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalInit`

- <span id="localinit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::LocalInit`

- <span id="cratelocalinit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LocalInit`

- <span id="localinit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::LocalInit`

- <span id="cratelocalinit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::LocalInit`

##### `impl<T> From for LocalInit`

- <span id="localinit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::LocalInit`

- <span id="cratelocalinit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for LocalInit`

- <span id="localinit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::LocalInit`

- <span id="cratelocalinit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToOwned for LocalInit`

- <span id="localinit-toowned-type-owned"></span>`type Owned = T`

- <span id="localinit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="localinit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocalInit`

- <span id="localinit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localinit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalInit`

- <span id="localinit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localinit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StmtMacro`

```rust
struct StmtMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
    pub semi_token: Option<token::Semi>,
}
```

*Defined in [`syn-2.0.111/src/stmt.rs:66-78`](../../../.source_1765633015/syn-2.0.111/src/stmt.rs#L66-L78)*

A macro invocation in statement position.

Syntactically it's ambiguous which other kind of statement this macro
would expand to. It can be any of local variable (`let`), item, or
expression.

#### Implementations

- <span id="cratestmtmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for StmtMacro`

- <span id="stmtmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StmtMacro`

- <span id="stmtmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StmtMacro`

- <span id="stmtmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::StmtMacro`

- <span id="cratestmtmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StmtMacro`

- <span id="stmtmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::StmtMacro`

- <span id="cratestmtmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::StmtMacro`

##### `impl<T> From for StmtMacro`

- <span id="stmtmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::StmtMacro`

- <span id="cratestmtmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for StmtMacro`

- <span id="stmtmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::StmtMacro`

- <span id="cratestmtmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for StmtMacro`

##### `impl Spanned for StmtMacro`

- <span id="stmtmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for StmtMacro`

- <span id="stmtmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="stmtmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stmtmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::StmtMacro`

- <span id="cratestmtstmtmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for StmtMacro`

- <span id="stmtmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stmtmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StmtMacro`

- <span id="stmtmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stmtmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/stmt.rs:18-38`](../../../.source_1765633015/syn-2.0.111/src/stmt.rs#L18-L38)*

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

##### `impl Any for Stmt`

- <span id="stmt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stmt`

- <span id="stmt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stmt`

- <span id="stmt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Stmt`

- <span id="cratestmt-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Stmt`

- <span id="stmt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Stmt`

- <span id="cratestmt-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Stmt`

##### `impl<T> From for Stmt`

- <span id="stmt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Stmt`

- <span id="cratestmt-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Stmt`

- <span id="stmt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::stmt::Stmt`

- <span id="cratestmtstmt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Stmt`

- <span id="cratestmt-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Stmt`

##### `impl Spanned for Stmt`

- <span id="stmt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Stmt`

- <span id="stmt-toowned-type-owned"></span>`type Owned = T`

- <span id="stmt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stmt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::stmt::Stmt`

- <span id="cratestmtstmt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Stmt`

- <span id="stmt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stmt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Stmt`

- <span id="stmt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stmt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

