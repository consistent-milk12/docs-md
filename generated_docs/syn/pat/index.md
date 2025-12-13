*[syn](../index.md) / [pat](index.md)*

---

# Module `pat`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`PatConst`](#patconst)
  - [`PatLit`](#patlit)
  - [`PatMacro`](#patmacro)
  - [`PatPath`](#patpath)
  - [`PatRange`](#patrange)
  - [`PatIdent`](#patident)
  - [`PatOr`](#pator)
  - [`PatParen`](#patparen)
  - [`PatReference`](#patreference)
  - [`PatRest`](#patrest)
  - [`PatSlice`](#patslice)
  - [`PatStruct`](#patstruct)
  - [`PatTuple`](#pattuple)
  - [`PatTupleStruct`](#pattuplestruct)
  - [`PatType`](#pattype)
  - [`PatWild`](#patwild)
  - [`FieldPat`](#fieldpat)
- [Enums](#enums)
  - [`Pat`](#pat)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`PatConst`](#patconst) | struct |  |
| [`PatLit`](#patlit) | struct |  |
| [`PatMacro`](#patmacro) | struct |  |
| [`PatPath`](#patpath) | struct |  |
| [`PatRange`](#patrange) | struct |  |
| [`PatIdent`](#patident) | struct | A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`. |
| [`PatOr`](#pator) | struct | A pattern that matches any one of a set of cases. |
| [`PatParen`](#patparen) | struct | A parenthesized pattern: `(A \| B)`. |
| [`PatReference`](#patreference) | struct | A reference pattern: `&mut var`. |
| [`PatRest`](#patrest) | struct | The dots in a tuple or slice pattern: `[0, 1, ..]`. |
| [`PatSlice`](#patslice) | struct | A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`. |
| [`PatStruct`](#patstruct) | struct | A struct or struct variant pattern: `Variant { x, y, .. |
| [`PatTuple`](#pattuple) | struct | A tuple pattern: `(a, b)`. |
| [`PatTupleStruct`](#pattuplestruct) | struct | A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`. |
| [`PatType`](#pattype) | struct | A type ascription pattern: `foo: f64`. |
| [`PatWild`](#patwild) | struct | A pattern that matches any value: `_`. |
| [`FieldPat`](#fieldpat) | struct | A single field in a struct pattern. |
| [`Pat`](#pat) | enum | A pattern in a local binding, function signature, match expression, or various other places. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `PatConst`

```rust
struct PatConst {
    pub attrs: Vec<crate::attr::Attribute>,
    pub const_token: token::Const,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:385-393`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L385-L393)*

A const block: `const { ... }`.

#### Implementations

- <span id="crateexprconst-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprConst`

- <span id="exprconst-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprConst`

- <span id="exprconst-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprConst`

- <span id="exprconst-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprConst`

- <span id="crateexprconst-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprConst`

- <span id="exprconst-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprConst`

- <span id="crateexprconst-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprConst`

##### `impl<T> From for ExprConst`

- <span id="exprconst-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprConst`

- <span id="crateexprconst-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprConst`

- <span id="exprconst-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprConst`

- <span id="crateexprexprconst-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprConst`

- <span id="crateexprconst-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprConst`

##### `impl Spanned for ExprConst`

- <span id="exprconst-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprConst`

- <span id="exprconst-toowned-type-owned"></span>`type Owned = T`

- <span id="exprconst-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprconst-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprConst`

- <span id="crateexprexprconst-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprConst`

- <span id="exprconst-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprconst-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprConst`

- <span id="exprconst-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprconst-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatLit`

```rust
struct PatLit {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lit: crate::lit::Lit,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:493-500`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L493-L500)*

A literal in place of an expression: `1`, `"foo"`.

#### Implementations

- <span id="crateexprlit-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLit`

- <span id="exprlit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLit`

- <span id="exprlit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLit`

- <span id="exprlit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLit`

- <span id="crateexprlit-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLit`

- <span id="exprlit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLit`

- <span id="crateexprlit-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLit`

##### `impl<T> From for ExprLit`

- <span id="exprlit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLit`

- <span id="crateexprlit-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLit`

- <span id="exprlit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLit`

- <span id="crateexprexprlit-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLit`

- <span id="crateexprlit-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLit`

##### `impl Spanned for ExprLit`

- <span id="exprlit-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLit`

- <span id="exprlit-toowned-type-owned"></span>`type Owned = T`

- <span id="exprlit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprlit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLit`

- <span id="crateexprexprlit-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLit`

- <span id="exprlit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprlit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLit`

- <span id="exprlit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprlit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatMacro`

```rust
struct PatMacro {
    pub attrs: Vec<crate::attr::Attribute>,
    pub mac: crate::mac::Macro,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:513-520`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L513-L520)*

A macro invocation expression: `format!("{}", q)`.

#### Implementations

- <span id="crateexprmacro-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMacro`

- <span id="exprmacro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMacro`

- <span id="exprmacro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMacro`

- <span id="exprmacro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMacro`

- <span id="crateexprmacro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMacro`

- <span id="exprmacro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMacro`

- <span id="crateexprmacro-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMacro`

##### `impl<T> From for ExprMacro`

- <span id="exprmacro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMacro`

- <span id="crateexprmacro-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMacro`

- <span id="exprmacro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMacro`

- <span id="crateexprmacro-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMacro`

##### `impl Spanned for ExprMacro`

- <span id="exprmacro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMacro`

- <span id="exprmacro-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmacro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmacro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMacro`

- <span id="crateexprexprmacro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMacro`

- <span id="exprmacro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmacro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMacro`

- <span id="exprmacro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmacro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatPath`

```rust
struct PatPath {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:558-569`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L558-L569)*

A path like `std::mem::replace` possibly containing generic
parameters and a qualified self-type.

A plain identifier like `x` is a path of length 1.

#### Implementations

- <span id="crateexprpath-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprPath`

- <span id="exprpath-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprPath`

- <span id="exprpath-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprPath`

- <span id="exprpath-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprPath`

- <span id="crateexprpath-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprPath`

- <span id="exprpath-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprPath`

- <span id="crateexprpath-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprPath`

##### `impl<T> From for ExprPath`

- <span id="exprpath-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprPath`

- <span id="crateexprpath-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprPath`

- <span id="exprpath-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprPath`

- <span id="crateexprexprpath-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprPath`

- <span id="crateexprpath-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprPath`

##### `impl Spanned for ExprPath`

- <span id="exprpath-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprPath`

- <span id="exprpath-toowned-type-owned"></span>`type Owned = T`

- <span id="exprpath-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprpath-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprPath`

- <span id="crateexprexprpath-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprPath`

- <span id="exprpath-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprpath-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprPath`

- <span id="exprpath-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprpath-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatRange`

```rust
struct PatRange {
    pub attrs: Vec<crate::attr::Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:571-580`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L571-L580)*

A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.

#### Implementations

- <span id="crateexprrange-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRange`

- <span id="exprrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRange`

- <span id="exprrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRange`

- <span id="exprrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRange`

- <span id="crateexprrange-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRange`

- <span id="exprrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRange`

- <span id="crateexprrange-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRange`

##### `impl<T> From for ExprRange`

- <span id="exprrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRange`

- <span id="crateexprrange-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRange`

- <span id="exprrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRange`

- <span id="crateexprexprrange-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRange`

- <span id="crateexprrange-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRange`

##### `impl Spanned for ExprRange`

- <span id="exprrange-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRange`

- <span id="exprrange-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRange`

- <span id="crateexprexprrange-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRange`

- <span id="exprrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRange`

- <span id="exprrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatIdent`

```rust
struct PatIdent {
    pub attrs: Vec<crate::attr::Attribute>,
    pub by_ref: Option<token::Ref>,
    pub mutability: Option<token::Mut>,
    pub ident: crate::ident::Ident,
    pub subpat: Option<(token::At, Box<Pat>)>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:104-117`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L104-L117)*

A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.

It may also be a unit struct or struct variant (e.g. `None`), or a
constant; these cannot be distinguished syntactically.

#### Implementations

- <span id="cratepatident-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatIdent`

- <span id="patident-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatIdent`

- <span id="patident-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatIdent`

- <span id="patident-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatIdent`

- <span id="cratepatident-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatIdent`

- <span id="patident-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatIdent`

- <span id="cratepatident-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatIdent`

##### `impl<T> From for PatIdent`

- <span id="patident-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatIdent`

- <span id="cratepatident-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatIdent`

- <span id="patident-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatIdent`

- <span id="cratepatident-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatIdent`

##### `impl Spanned for PatIdent`

- <span id="patident-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatIdent`

- <span id="patident-toowned-type-owned"></span>`type Owned = T`

- <span id="patident-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patident-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatIdent`

- <span id="cratepatpatident-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatIdent`

- <span id="patident-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patident-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatIdent`

- <span id="patident-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patident-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatOr`

```rust
struct PatOr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub leading_vert: Option<token::Or>,
    pub cases: crate::punctuated::Punctuated<Pat, token::Or>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:119-127`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L119-L127)*

A pattern that matches any one of a set of cases.

#### Implementations

- <span id="cratepator-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatOr`

- <span id="pator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatOr`

- <span id="pator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatOr`

- <span id="pator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatOr`

- <span id="cratepator-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatOr`

- <span id="pator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatOr`

- <span id="cratepator-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatOr`

##### `impl<T> From for PatOr`

- <span id="pator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatOr`

- <span id="cratepator-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatOr`

- <span id="pator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatOr`

- <span id="cratepator-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatOr`

##### `impl Spanned for PatOr`

- <span id="pator-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatOr`

- <span id="pator-toowned-type-owned"></span>`type Owned = T`

- <span id="pator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatOr`

- <span id="cratepatpator-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatOr`

- <span id="pator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatOr`

- <span id="pator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatParen`

```rust
struct PatParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:129-137`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L129-L137)*

A parenthesized pattern: `(A | B)`.

#### Implementations

- <span id="cratepatparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatParen`

- <span id="patparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatParen`

- <span id="patparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatParen`

- <span id="patparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatParen`

- <span id="cratepatparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatParen`

- <span id="patparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatParen`

- <span id="cratepatparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatParen`

##### `impl<T> From for PatParen`

- <span id="patparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatParen`

- <span id="cratepatparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatParen`

- <span id="patparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatParen`

- <span id="cratepatparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatParen`

##### `impl Spanned for PatParen`

- <span id="patparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatParen`

- <span id="patparen-toowned-type-owned"></span>`type Owned = T`

- <span id="patparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatParen`

- <span id="cratepatpatparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatParen`

- <span id="patparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatParen`

- <span id="patparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatReference`

```rust
struct PatReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:139-148`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L139-L148)*

A reference pattern: `&mut var`.

#### Implementations

- <span id="cratepatreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatReference`

- <span id="patreference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatReference`

- <span id="patreference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatReference`

- <span id="patreference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatReference`

- <span id="cratepatreference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatReference`

- <span id="patreference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatReference`

- <span id="cratepatreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatReference`

##### `impl<T> From for PatReference`

- <span id="patreference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatReference`

- <span id="cratepatreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatReference`

- <span id="patreference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatReference`

- <span id="cratepatreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatReference`

##### `impl Spanned for PatReference`

- <span id="patreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatReference`

- <span id="patreference-toowned-type-owned"></span>`type Owned = T`

- <span id="patreference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patreference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatReference`

- <span id="cratepatpatreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatReference`

- <span id="patreference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patreference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatReference`

- <span id="patreference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patreference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatRest`

```rust
struct PatRest {
    pub attrs: Vec<crate::attr::Attribute>,
    pub dot2_token: token::DotDot,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:150-157`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L150-L157)*

The dots in a tuple or slice pattern: `[0, 1, ..]`.

#### Implementations

- <span id="cratepatrest-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatRest`

- <span id="patrest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatRest`

- <span id="patrest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatRest`

- <span id="patrest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatRest`

- <span id="cratepatrest-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatRest`

- <span id="patrest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatRest`

- <span id="cratepatrest-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatRest`

##### `impl<T> From for PatRest`

- <span id="patrest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatRest`

- <span id="cratepatrest-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatRest`

- <span id="patrest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatRest`

- <span id="cratepatrest-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatRest`

##### `impl Spanned for PatRest`

- <span id="patrest-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatRest`

- <span id="patrest-toowned-type-owned"></span>`type Owned = T`

- <span id="patrest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patrest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatRest`

- <span id="cratepatpatrest-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatRest`

- <span id="patrest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patrest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatRest`

- <span id="patrest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patrest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatSlice`

```rust
struct PatSlice {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:159-167`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L159-L167)*

A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.

#### Implementations

- <span id="cratepatslice-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatSlice`

- <span id="patslice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatSlice`

- <span id="patslice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatSlice`

- <span id="patslice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatSlice`

- <span id="cratepatslice-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatSlice`

- <span id="patslice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatSlice`

- <span id="cratepatslice-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatSlice`

##### `impl<T> From for PatSlice`

- <span id="patslice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatSlice`

- <span id="cratepatslice-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatSlice`

- <span id="patslice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatSlice`

- <span id="cratepatslice-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatSlice`

##### `impl Spanned for PatSlice`

- <span id="patslice-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatSlice`

- <span id="patslice-toowned-type-owned"></span>`type Owned = T`

- <span id="patslice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patslice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatSlice`

- <span id="cratepatpatslice-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatSlice`

- <span id="patslice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patslice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatSlice`

- <span id="patslice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patslice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatStruct`

```rust
struct PatStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldPat, token::Comma>,
    pub rest: Option<PatRest>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:169-180`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L169-L180)*

A struct or struct variant pattern: `Variant { x, y, .. }`.

#### Implementations

- <span id="cratepatstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatStruct`

- <span id="patstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatStruct`

- <span id="patstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatStruct`

- <span id="patstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatStruct`

- <span id="cratepatstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatStruct`

- <span id="patstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatStruct`

- <span id="cratepatstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatStruct`

##### `impl<T> From for PatStruct`

- <span id="patstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatStruct`

- <span id="cratepatstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatStruct`

- <span id="patstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatStruct`

- <span id="cratepatstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatStruct`

##### `impl Spanned for PatStruct`

- <span id="patstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatStruct`

- <span id="patstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="patstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatStruct`

- <span id="cratepatpatstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatStruct`

- <span id="patstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatStruct`

- <span id="patstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatTuple`

```rust
struct PatTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:182-190`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L182-L190)*

A tuple pattern: `(a, b)`.

#### Implementations

- <span id="cratepattuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatTuple`

- <span id="pattuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatTuple`

- <span id="pattuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatTuple`

- <span id="pattuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatTuple`

- <span id="cratepattuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatTuple`

- <span id="pattuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatTuple`

- <span id="cratepattuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTuple`

##### `impl<T> From for PatTuple`

- <span id="pattuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatTuple`

- <span id="cratepattuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatTuple`

- <span id="pattuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatTuple`

- <span id="cratepattuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTuple`

##### `impl Spanned for PatTuple`

- <span id="pattuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatTuple`

- <span id="pattuple-toowned-type-owned"></span>`type Owned = T`

- <span id="pattuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatTuple`

- <span id="cratepatpattuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatTuple`

- <span id="pattuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatTuple`

- <span id="pattuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatTupleStruct`

```rust
struct PatTupleStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Pat, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:192-202`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L192-L202)*

A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.

#### Implementations

- <span id="cratepattuplestruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatTupleStruct`

- <span id="pattuplestruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatTupleStruct`

- <span id="pattuplestruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatTupleStruct`

- <span id="pattuplestruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatTupleStruct`

- <span id="cratepattuplestruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatTupleStruct`

- <span id="pattuplestruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatTupleStruct`

- <span id="cratepattuplestruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatTupleStruct`

##### `impl<T> From for PatTupleStruct`

- <span id="pattuplestruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatTupleStruct`

- <span id="cratepattuplestruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatTupleStruct`

- <span id="pattuplestruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatTupleStruct`

- <span id="cratepattuplestruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatTupleStruct`

##### `impl Spanned for PatTupleStruct`

- <span id="pattuplestruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatTupleStruct`

- <span id="pattuplestruct-toowned-type-owned"></span>`type Owned = T`

- <span id="pattuplestruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattuplestruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatTupleStruct`

- <span id="cratepatpattuplestruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatTupleStruct`

- <span id="pattuplestruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattuplestruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatTupleStruct`

- <span id="pattuplestruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattuplestruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatType`

```rust
struct PatType {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: Box<Pat>,
    pub colon_token: token::Colon,
    pub ty: Box<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:204-213`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L204-L213)*

A type ascription pattern: `foo: f64`.

#### Implementations

- <span id="cratepattype-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatType`

- <span id="pattype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatType`

- <span id="pattype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatType`

- <span id="pattype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatType`

- <span id="cratepattype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatType`

- <span id="pattype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatType`

- <span id="cratepattype-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatType`

##### `impl<T> From for PatType`

- <span id="pattype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatType`

- <span id="cratepattype-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatType`

- <span id="pattype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::pat::PatType`

- <span id="cratepatpattype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PatType`

- <span id="cratepattype-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatType`

##### `impl Spanned for PatType`

- <span id="pattype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatType`

- <span id="pattype-toowned-type-owned"></span>`type Owned = T`

- <span id="pattype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatType`

- <span id="cratepatpattype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatType`

- <span id="pattype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatType`

- <span id="pattype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatWild`

```rust
struct PatWild {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:215-222`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L215-L222)*

A pattern that matches any value: `_`.

#### Implementations

- <span id="cratepatwild-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for PatWild`

- <span id="patwild-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatWild`

- <span id="patwild-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatWild`

- <span id="patwild-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PatWild`

- <span id="cratepatwild-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PatWild`

- <span id="patwild-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PatWild`

- <span id="cratepatwild-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PatWild`

##### `impl<T> From for PatWild`

- <span id="patwild-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PatWild`

- <span id="cratepatwild-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PatWild`

- <span id="patwild-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::PatWild`

- <span id="cratepatwild-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PatWild`

##### `impl Spanned for PatWild`

- <span id="patwild-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PatWild`

- <span id="patwild-toowned-type-owned"></span>`type Owned = T`

- <span id="patwild-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patwild-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::PatWild`

- <span id="cratepatpatwild-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PatWild`

- <span id="patwild-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patwild-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatWild`

- <span id="patwild-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patwild-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldPat`

```rust
struct FieldPat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: crate::expr::Member,
    pub colon_token: Option<token::Colon>,
    pub pat: Box<Pat>,
}
```

*Defined in [`syn-2.0.111/src/pat.rs:224-236`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L224-L236)*

A single field in a struct pattern.

Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.

#### Trait Implementations

##### `impl Any for FieldPat`

- <span id="fieldpat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldPat`

- <span id="fieldpat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldPat`

- <span id="fieldpat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldPat`

- <span id="cratefieldpat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldPat`

- <span id="fieldpat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldPat`

- <span id="cratefieldpat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldPat`

##### `impl<T> From for FieldPat`

- <span id="fieldpat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldPat`

- <span id="cratefieldpat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldPat`

- <span id="fieldpat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::FieldPat`

- <span id="cratefieldpat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldPat`

##### `impl Spanned for FieldPat`

- <span id="fieldpat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldPat`

- <span id="fieldpat-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldpat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldpat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::pat::FieldPat`

- <span id="cratepatfieldpat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldPat`

- <span id="fieldpat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldpat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldPat`

- <span id="fieldpat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldpat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/pat.rs:15-102`](../../../.source_1765633015/syn-2.0.111/src/pat.rs#L15-L102)*

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

- <span id="cratepatpat-parse-single"></span>`fn parse_single(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern that does _not_ involve `|` at the top level.

  

  This parser matches the behavior of the `$:pat_param` macro_rules

  matcher, and on editions prior to Rust 2021, the behavior of

  `$:pat`.

  

  In Rust syntax, some examples of where this syntax would occur are

  in the argument pattern of functions and closures. Patterns using

  `|` are not allowed to occur in these positions.

  

  ```compile_fail

  fn f(Some(_) | None: Option<T>) {

      let _ = |Some(_) | None: Option<T>| {};

      //       ^^^^^^^^^^^^^^^^^^^^^^^^^??? :(

  }

  ```

  

  ```console

  error: top-level or-patterns are not allowed in function parameters

   --> src/main.rs:1:6

    |

  1 | fn f(Some(_) | None: Option<T>) {

    |      ^^^^^^^^^^^^^^ help: wrap the pattern in parentheses: `(Some(_) | None)`

  ```

- <span id="cratepatpat-parse-multi"></span>`fn parse_multi(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern, possibly involving `|`, but not a leading `|`.

- <span id="cratepatpat-parse-multi-with-leading-vert"></span>`fn parse_multi_with_leading_vert(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

  Parse a pattern, possibly involving `|`, possibly including a

  leading `|`.

  

  This parser matches the behavior of the Rust 2021 edition's `$:pat`

  macro_rules matcher.

  

  In Rust syntax, an example of where this syntax would occur is in

  the pattern of a `match` arm, where the language permits an optional

  leading `|`, although it is not idiomatic to write one there in

  handwritten code.

  

  ```rust

  let wat = None;

  match wat {

      | None | Some(false) => {}

      | Some(true) => {}

  }

  ```

  

  The compiler accepts it only to facilitate some situations in

  macro-generated code where a macro author might need to write:

  

  ```rust

  macro_rules! doc {

      ($value:expr, ($($conditions1:pat),*), ($($conditions2:pat),*), $then:expr) => {

  match $value {

      $(| $conditions1)* $(| $conditions2)* => $then

  }

      };

  }

  

  doc!(true, (true), (false), {});

  doc!(true, (), (true, false), {});

  doc!(true, (true, false), (), {});

  ```

  

  Expressing the same thing correctly in the case that either one (but

  not both) of `$conditions1` and `$conditions2` might be empty,

  without leading `|`, is complex.

  

  Use `Pat::parse_multi` instead if you are not intending to support

  macro-generated macro input.

#### Trait Implementations

##### `impl Any for Pat`

- <span id="pat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pat`

- <span id="pat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pat`

- <span id="pat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Pat`

- <span id="cratepat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Pat`

- <span id="pat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Pat`

- <span id="cratepat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Pat`

##### `impl<T> From for Pat`

- <span id="pat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Pat`

- <span id="cratepat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Pat`

- <span id="pat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::Pat`

- <span id="cratepat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Pat`

##### `impl Spanned for Pat`

- <span id="pat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Pat`

- <span id="pat-toowned-type-owned"></span>`type Owned = T`

- <span id="pat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Pat`

- <span id="pat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Pat`

- <span id="pat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pat`

- <span id="pat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

