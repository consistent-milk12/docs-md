*[syn](../index.md) / [expr](index.md)*

---

# Module `expr`

## Contents

- [Modules](#modules)
  - [`parsing`](#parsing)
  - [`printing`](#printing)
- [Structs](#structs)
  - [`ExprArray`](#exprarray)
  - [`ExprAssign`](#exprassign)
  - [`ExprAsync`](#exprasync)
  - [`ExprAwait`](#exprawait)
  - [`ExprBinary`](#exprbinary)
  - [`ExprBlock`](#exprblock)
  - [`ExprBreak`](#exprbreak)
  - [`ExprCall`](#exprcall)
  - [`ExprCast`](#exprcast)
  - [`ExprClosure`](#exprclosure)
  - [`ExprConst`](#exprconst)
  - [`ExprContinue`](#exprcontinue)
  - [`ExprField`](#exprfield)
  - [`ExprForLoop`](#exprforloop)
  - [`ExprGroup`](#exprgroup)
  - [`ExprIf`](#exprif)
  - [`ExprIndex`](#exprindex)
  - [`ExprInfer`](#exprinfer)
  - [`ExprLet`](#exprlet)
  - [`ExprLit`](#exprlit)
  - [`ExprLoop`](#exprloop)
  - [`ExprMacro`](#exprmacro)
  - [`ExprMatch`](#exprmatch)
  - [`ExprMethodCall`](#exprmethodcall)
  - [`ExprParen`](#exprparen)
  - [`ExprPath`](#exprpath)
  - [`ExprRange`](#exprrange)
  - [`ExprRawAddr`](#exprrawaddr)
  - [`ExprReference`](#exprreference)
  - [`ExprRepeat`](#exprrepeat)
  - [`ExprReturn`](#exprreturn)
  - [`ExprStruct`](#exprstruct)
  - [`ExprTry`](#exprtry)
  - [`ExprTryBlock`](#exprtryblock)
  - [`ExprTuple`](#exprtuple)
  - [`ExprUnary`](#exprunary)
  - [`ExprUnsafe`](#exprunsafe)
  - [`ExprWhile`](#exprwhile)
  - [`ExprYield`](#expryield)
  - [`Index`](#index)
  - [`FieldValue`](#fieldvalue)
  - [`Label`](#label)
  - [`Arm`](#arm)
- [Enums](#enums)
  - [`Expr`](#expr)
  - [`Member`](#member)
  - [`RangeLimits`](#rangelimits)
  - [`PointerMutability`](#pointermutability)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`ExprArray`](#exprarray) | struct | A slice literal expression: `[a, b, c, d]`. |
| [`ExprAssign`](#exprassign) | struct | An assignment expression: `a = compute()`. |
| [`ExprAsync`](#exprasync) | struct | An async block: `async { ... |
| [`ExprAwait`](#exprawait) | struct | An await expression: `fut.await`. |
| [`ExprBinary`](#exprbinary) | struct | A binary operation: `a + b`, `a += b`. |
| [`ExprBlock`](#exprblock) | struct | A blocked scope: `{ ... |
| [`ExprBreak`](#exprbreak) | struct | A `break`, with an optional label to break and an optional expression. |
| [`ExprCall`](#exprcall) | struct | A function call expression: `invoke(a, b)`. |
| [`ExprCast`](#exprcast) | struct | A cast expression: `foo as f64`. |
| [`ExprClosure`](#exprclosure) | struct | A closure expression: `\|a, b\| a + b`. |
| [`ExprConst`](#exprconst) | struct | A const block: `const { ... |
| [`ExprContinue`](#exprcontinue) | struct | A `continue`, with an optional label. |
| [`ExprField`](#exprfield) | struct | Access of a named struct field (`obj.k`) or unnamed tuple struct field (`obj.0`). |
| [`ExprForLoop`](#exprforloop) | struct | A for loop: `for pat in expr { ... |
| [`ExprGroup`](#exprgroup) | struct | An expression contained within invisible delimiters. |
| [`ExprIf`](#exprif) | struct | An `if` expression with an optional `else` block: `if expr { ... |
| [`ExprIndex`](#exprindex) | struct | A square bracketed indexing expression: `vector[2]`. |
| [`ExprInfer`](#exprinfer) | struct | The inferred value of a const generic argument, denoted `_`. |
| [`ExprLet`](#exprlet) | struct | A `let` guard: `let Some(x) = opt`. |
| [`ExprLit`](#exprlit) | struct | A literal in place of an expression: `1`, `"foo"`. |
| [`ExprLoop`](#exprloop) | struct | Conditionless loop: `loop { ... |
| [`ExprMacro`](#exprmacro) | struct | A macro invocation expression: `format!("{}", q)`. |
| [`ExprMatch`](#exprmatch) | struct | A `match` expression: `match n { Some(n) => {}, None => {} }`. |
| [`ExprMethodCall`](#exprmethodcall) | struct | A method call expression: `x.foo::<T>(a, b)`. |
| [`ExprParen`](#exprparen) | struct | A parenthesized expression: `(a + b)`. |
| [`ExprPath`](#exprpath) | struct | A path like `std::mem::replace` possibly containing generic parameters and a qualified self-type. |
| [`ExprRange`](#exprrange) | struct | A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`. |
| [`ExprRawAddr`](#exprrawaddr) | struct | Address-of operation: `&raw const place` or `&raw mut place`. |
| [`ExprReference`](#exprreference) | struct | A referencing operation: `&a` or `&mut a`. |
| [`ExprRepeat`](#exprrepeat) | struct | An array literal constructed from one repeated element: `[0u8; N]`. |
| [`ExprReturn`](#exprreturn) | struct | A `return`, with an optional value to be returned. |
| [`ExprStruct`](#exprstruct) | struct | A struct literal expression: `Point { x: 1, y: 1 }`. |
| [`ExprTry`](#exprtry) | struct | A try-expression: `expr?`. |
| [`ExprTryBlock`](#exprtryblock) | struct | A try block: `try { ... |
| [`ExprTuple`](#exprtuple) | struct | A tuple expression: `(a, b, c, d)`. |
| [`ExprUnary`](#exprunary) | struct | A unary operation: `!x`, `*x`. |
| [`ExprUnsafe`](#exprunsafe) | struct | An unsafe block: `unsafe { ... |
| [`ExprWhile`](#exprwhile) | struct | A while loop: `while expr { ... |
| [`ExprYield`](#expryield) | struct | A yield expression: `yield expr`. |
| [`Index`](#index) | struct | The index of an unnamed tuple struct field. |
| [`FieldValue`](#fieldvalue) | struct | A field-value pair in a struct literal. |
| [`Label`](#label) | struct | A lifetime labeling a `for`, `while`, or `loop`. |
| [`Arm`](#arm) | struct | One arm of a `match` expression: `0..=10 => { return true; }`. |
| [`Expr`](#expr) | enum | A Rust expression. |
| [`Member`](#member) | enum | A struct or tuple struct field accessed in a struct literal or field expression. |
| [`RangeLimits`](#rangelimits) | enum | Limit types of a range, inclusive or exclusive. |
| [`PointerMutability`](#pointermutability) | enum | Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable isn't the implicit default. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Structs

### `ExprArray`

```rust
struct ExprArray {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:269-277`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L269-L277)*

A slice literal expression: `[a, b, c, d]`.

#### Implementations

- <span id="crateexprarray-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprArray`

- <span id="exprarray-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprArray`

- <span id="exprarray-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprArray`

- <span id="exprarray-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprArray`

- <span id="crateexprarray-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprArray`

- <span id="exprarray-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprArray`

- <span id="crateexprarray-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprArray`

##### `impl<T> From for ExprArray`

- <span id="exprarray-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprArray`

- <span id="crateexprarray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprArray`

- <span id="exprarray-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprArray`

- <span id="crateexprexprarray-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprArray`

- <span id="crateexprarray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprArray`

##### `impl Spanned for ExprArray`

- <span id="exprarray-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprArray`

- <span id="exprarray-toowned-type-owned"></span>`type Owned = T`

- <span id="exprarray-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprarray-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprArray`

- <span id="crateexprexprarray-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprArray`

- <span id="exprarray-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprarray-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprArray`

- <span id="exprarray-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprarray-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAssign`

```rust
struct ExprAssign {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub eq_token: token::Eq,
    pub right: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:279-288`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L279-L288)*

An assignment expression: `a = compute()`.

#### Implementations

- <span id="crateexprassign-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAssign`

- <span id="exprassign-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAssign`

- <span id="exprassign-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAssign`

- <span id="exprassign-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAssign`

- <span id="crateexprassign-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAssign`

- <span id="exprassign-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAssign`

- <span id="crateexprassign-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAssign`

##### `impl<T> From for ExprAssign`

- <span id="exprassign-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAssign`

- <span id="crateexprassign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAssign`

- <span id="exprassign-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAssign`

- <span id="crateexprexprassign-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAssign`

- <span id="crateexprassign-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAssign`

##### `impl Spanned for ExprAssign`

- <span id="exprassign-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAssign`

- <span id="exprassign-toowned-type-owned"></span>`type Owned = T`

- <span id="exprassign-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprassign-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAssign`

- <span id="crateexprexprassign-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAssign`

- <span id="exprassign-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprassign-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAssign`

- <span id="exprassign-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprassign-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAsync`

```rust
struct ExprAsync {
    pub attrs: Vec<crate::attr::Attribute>,
    pub async_token: token::Async,
    pub capture: Option<token::Move>,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:290-299`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L290-L299)*

An async block: `async { ... }`.

#### Implementations

- <span id="crateexprasync-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAsync`

- <span id="exprasync-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAsync`

- <span id="exprasync-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAsync`

- <span id="exprasync-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAsync`

- <span id="crateexprasync-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAsync`

- <span id="exprasync-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAsync`

- <span id="crateexprasync-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAsync`

##### `impl<T> From for ExprAsync`

- <span id="exprasync-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAsync`

- <span id="crateexprasync-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAsync`

- <span id="exprasync-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAsync`

- <span id="crateexprexprasync-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAsync`

- <span id="crateexprasync-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAsync`

##### `impl Spanned for ExprAsync`

- <span id="exprasync-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAsync`

- <span id="exprasync-toowned-type-owned"></span>`type Owned = T`

- <span id="exprasync-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprasync-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAsync`

- <span id="crateexprexprasync-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAsync`

- <span id="exprasync-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprasync-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAsync`

- <span id="exprasync-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprasync-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprAwait`

```rust
struct ExprAwait {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub await_token: token::Await,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:301-310`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L301-L310)*

An await expression: `fut.await`.

#### Implementations

- <span id="crateexprawait-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprAwait`

- <span id="exprawait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprAwait`

- <span id="exprawait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprAwait`

- <span id="exprawait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprAwait`

- <span id="crateexprawait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprAwait`

- <span id="exprawait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprAwait`

- <span id="crateexprawait-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprAwait`

##### `impl<T> From for ExprAwait`

- <span id="exprawait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprAwait`

- <span id="crateexprawait-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprAwait`

- <span id="exprawait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprAwait`

- <span id="crateexprexprawait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprAwait`

- <span id="crateexprawait-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprAwait`

##### `impl Spanned for ExprAwait`

- <span id="exprawait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprAwait`

- <span id="exprawait-toowned-type-owned"></span>`type Owned = T`

- <span id="exprawait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprawait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprAwait`

- <span id="crateexprexprawait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprAwait`

- <span id="exprawait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprawait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprAwait`

- <span id="exprawait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprawait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBinary`

```rust
struct ExprBinary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub left: Box<Expr>,
    pub op: crate::op::BinOp,
    pub right: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:312-321`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L312-L321)*

A binary operation: `a + b`, `a += b`.

#### Implementations

- <span id="crateexprbinary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBinary`

- <span id="exprbinary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBinary`

- <span id="exprbinary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBinary`

- <span id="exprbinary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBinary`

- <span id="crateexprbinary-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBinary`

- <span id="exprbinary-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBinary`

- <span id="crateexprbinary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBinary`

##### `impl<T> From for ExprBinary`

- <span id="exprbinary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBinary`

- <span id="crateexprbinary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBinary`

- <span id="exprbinary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBinary`

- <span id="crateexprbinary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBinary`

##### `impl Spanned for ExprBinary`

- <span id="exprbinary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBinary`

- <span id="exprbinary-toowned-type-owned"></span>`type Owned = T`

- <span id="exprbinary-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprbinary-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBinary`

- <span id="crateexprexprbinary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBinary`

- <span id="exprbinary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprbinary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBinary`

- <span id="exprbinary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprbinary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBlock`

```rust
struct ExprBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:323-331`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L323-L331)*

A blocked scope: `{ ... }`.

#### Implementations

- <span id="crateexprblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBlock`

- <span id="exprblock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBlock`

- <span id="exprblock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBlock`

- <span id="exprblock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBlock`

- <span id="crateexprblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBlock`

- <span id="exprblock-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBlock`

- <span id="crateexprblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBlock`

##### `impl<T> From for ExprBlock`

- <span id="exprblock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBlock`

- <span id="crateexprblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBlock`

- <span id="exprblock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBlock`

- <span id="crateexprexprblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBlock`

- <span id="crateexprblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBlock`

##### `impl Spanned for ExprBlock`

- <span id="exprblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBlock`

- <span id="exprblock-toowned-type-owned"></span>`type Owned = T`

- <span id="exprblock-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprblock-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBlock`

- <span id="crateexprexprblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBlock`

- <span id="exprblock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprblock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBlock`

- <span id="exprblock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprblock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprBreak`

```rust
struct ExprBreak {
    pub attrs: Vec<crate::attr::Attribute>,
    pub break_token: token::Break,
    pub label: Option<crate::lifetime::Lifetime>,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:333-343`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L333-L343)*

A `break`, with an optional label to break and an optional
expression.

#### Implementations

- <span id="crateexprbreak-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprBreak`

- <span id="exprbreak-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprBreak`

- <span id="exprbreak-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprBreak`

- <span id="exprbreak-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprBreak`

- <span id="crateexprbreak-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprBreak`

- <span id="exprbreak-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprBreak`

- <span id="crateexprbreak-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprBreak`

##### `impl<T> From for ExprBreak`

- <span id="exprbreak-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprBreak`

- <span id="crateexprbreak-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprBreak`

- <span id="exprbreak-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprBreak`

- <span id="crateexprbreak-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprBreak`

##### `impl Spanned for ExprBreak`

- <span id="exprbreak-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprBreak`

- <span id="exprbreak-toowned-type-owned"></span>`type Owned = T`

- <span id="exprbreak-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprbreak-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprBreak`

- <span id="crateexprexprbreak-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprBreak`

- <span id="exprbreak-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprbreak-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprBreak`

- <span id="exprbreak-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprbreak-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCall`

```rust
struct ExprCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub func: Box<Expr>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:345-354`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L345-L354)*

A function call expression: `invoke(a, b)`.

#### Implementations

- <span id="crateexprcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprCall`

- <span id="exprcall-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprCall`

- <span id="exprcall-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprCall`

- <span id="exprcall-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprCall`

- <span id="crateexprcall-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprCall`

- <span id="exprcall-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprCall`

- <span id="crateexprcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCall`

##### `impl<T> From for ExprCall`

- <span id="exprcall-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprCall`

- <span id="crateexprcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprCall`

- <span id="exprcall-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprCall`

- <span id="crateexprexprcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprCall`

- <span id="crateexprcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCall`

##### `impl Spanned for ExprCall`

- <span id="exprcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprCall`

- <span id="exprcall-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcall-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcall-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprCall`

- <span id="crateexprexprcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprCall`

- <span id="exprcall-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcall-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprCall`

- <span id="exprcall-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcall-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprCast`

```rust
struct ExprCast {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub as_token: token::As,
    pub ty: Box<crate::ty::Type>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:356-365`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L356-L365)*

A cast expression: `foo as f64`.

#### Implementations

- <span id="crateexprcast-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprCast`

- <span id="exprcast-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprCast`

- <span id="exprcast-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprCast`

- <span id="exprcast-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprCast`

- <span id="crateexprcast-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprCast`

- <span id="exprcast-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprCast`

- <span id="crateexprcast-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprCast`

##### `impl<T> From for ExprCast`

- <span id="exprcast-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprCast`

- <span id="crateexprcast-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprCast`

- <span id="exprcast-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprCast`

- <span id="crateexprexprcast-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprCast`

- <span id="crateexprcast-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprCast`

##### `impl Spanned for ExprCast`

- <span id="exprcast-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprCast`

- <span id="exprcast-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcast-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcast-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprCast`

- <span id="crateexprexprcast-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprCast`

- <span id="exprcast-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcast-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprCast`

- <span id="exprcast-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcast-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprClosure`

```rust
struct ExprClosure {
    pub attrs: Vec<crate::attr::Attribute>,
    pub lifetimes: Option<crate::generics::BoundLifetimes>,
    pub constness: Option<token::Const>,
    pub movability: Option<token::Static>,
    pub asyncness: Option<token::Async>,
    pub capture: Option<token::Move>,
    pub or1_token: token::Or,
    pub inputs: crate::punctuated::Punctuated<crate::pat::Pat, token::Comma>,
    pub or2_token: token::Or,
    pub output: crate::ty::ReturnType,
    pub body: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:367-383`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L367-L383)*

A closure expression: `|a, b| a + b`.

#### Implementations

- <span id="crateexprclosure-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprClosure`

- <span id="exprclosure-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprClosure`

- <span id="exprclosure-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprClosure`

- <span id="exprclosure-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprClosure`

- <span id="crateexprclosure-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprClosure`

- <span id="exprclosure-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprClosure`

- <span id="crateexprclosure-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprClosure`

##### `impl<T> From for ExprClosure`

- <span id="exprclosure-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprClosure`

- <span id="crateexprclosure-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprClosure`

- <span id="exprclosure-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprClosure`

- <span id="crateexprclosure-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprClosure`

##### `impl Spanned for ExprClosure`

- <span id="exprclosure-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprClosure`

- <span id="exprclosure-toowned-type-owned"></span>`type Owned = T`

- <span id="exprclosure-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprclosure-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprClosure`

- <span id="crateexprexprclosure-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprClosure`

- <span id="exprclosure-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprclosure-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprClosure`

- <span id="exprclosure-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprclosure-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprConst`

```rust
struct ExprConst {
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

### `ExprContinue`

```rust
struct ExprContinue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub continue_token: token::Continue,
    pub label: Option<crate::lifetime::Lifetime>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:395-403`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L395-L403)*

A `continue`, with an optional label.

#### Implementations

- <span id="crateexprcontinue-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprContinue`

- <span id="exprcontinue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprContinue`

- <span id="exprcontinue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprContinue`

- <span id="exprcontinue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprContinue`

- <span id="crateexprcontinue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprContinue`

- <span id="exprcontinue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprContinue`

- <span id="crateexprcontinue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprContinue`

##### `impl<T> From for ExprContinue`

- <span id="exprcontinue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprContinue`

- <span id="crateexprcontinue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprContinue`

- <span id="exprcontinue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprContinue`

- <span id="crateexprcontinue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprContinue`

##### `impl Spanned for ExprContinue`

- <span id="exprcontinue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprContinue`

- <span id="exprcontinue-toowned-type-owned"></span>`type Owned = T`

- <span id="exprcontinue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprcontinue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprContinue`

- <span id="crateexprexprcontinue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprContinue`

- <span id="exprcontinue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprcontinue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprContinue`

- <span id="exprcontinue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprcontinue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprField`

```rust
struct ExprField {
    pub attrs: Vec<crate::attr::Attribute>,
    pub base: Box<Expr>,
    pub dot_token: token::Dot,
    pub member: Member,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:405-415`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L405-L415)*

Access of a named struct field (`obj.k`) or unnamed tuple struct
field (`obj.0`).

#### Implementations

- <span id="crateexprfield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprField`

- <span id="exprfield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprField`

- <span id="exprfield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprField`

- <span id="exprfield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprField`

- <span id="crateexprfield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprField`

- <span id="exprfield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprField`

- <span id="crateexprfield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprField`

##### `impl<T> From for ExprField`

- <span id="exprfield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprField`

- <span id="crateexprfield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprField`

- <span id="exprfield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprField`

- <span id="crateexprexprfield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprField`

- <span id="crateexprfield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprField`

##### `impl Spanned for ExprField`

- <span id="exprfield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprField`

- <span id="exprfield-toowned-type-owned"></span>`type Owned = T`

- <span id="exprfield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprfield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprField`

- <span id="crateexprexprfield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprField`

- <span id="exprfield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprfield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprField`

- <span id="exprfield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprfield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprForLoop`

```rust
struct ExprForLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub for_token: token::For,
    pub pat: Box<crate::pat::Pat>,
    pub in_token: token::In,
    pub expr: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:417-429`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L417-L429)*

A for loop: `for pat in expr { ... }`.

#### Implementations

- <span id="crateexprforloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprForLoop`

- <span id="exprforloop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprForLoop`

- <span id="exprforloop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprForLoop`

- <span id="exprforloop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprForLoop`

- <span id="crateexprforloop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprForLoop`

- <span id="exprforloop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprForLoop`

- <span id="crateexprforloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprForLoop`

##### `impl<T> From for ExprForLoop`

- <span id="exprforloop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprForLoop`

- <span id="crateexprforloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprForLoop`

- <span id="exprforloop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprForLoop`

- <span id="crateexprforloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprForLoop`

##### `impl Spanned for ExprForLoop`

- <span id="exprforloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprForLoop`

- <span id="exprforloop-toowned-type-owned"></span>`type Owned = T`

- <span id="exprforloop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprforloop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprForLoop`

- <span id="crateexprexprforloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprForLoop`

- <span id="exprforloop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprforloop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprForLoop`

- <span id="exprforloop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprforloop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprGroup`

```rust
struct ExprGroup {
    pub attrs: Vec<crate::attr::Attribute>,
    pub group_token: token::Group,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:431-443`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L431-L443)*

An expression contained within invisible delimiters.

This variant is important for faithfully representing the precedence
of expressions and is related to `None`-delimited spans in a
`TokenStream`.

#### Implementations

- <span id="crateexprgroup-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprGroup`

- <span id="exprgroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprGroup`

- <span id="exprgroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprGroup`

- <span id="exprgroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprGroup`

- <span id="crateexprgroup-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprGroup`

- <span id="exprgroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprGroup`

- <span id="crateexprgroup-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprGroup`

##### `impl<T> From for ExprGroup`

- <span id="exprgroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprGroup`

- <span id="crateexprgroup-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprGroup`

- <span id="exprgroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::ExprGroup`

- <span id="crateexprgroup-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprGroup`

##### `impl Spanned for ExprGroup`

- <span id="exprgroup-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprGroup`

- <span id="exprgroup-toowned-type-owned"></span>`type Owned = T`

- <span id="exprgroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprgroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprGroup`

- <span id="crateexprexprgroup-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprGroup`

- <span id="exprgroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprgroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprGroup`

- <span id="exprgroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprgroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprIf`

```rust
struct ExprIf {
    pub attrs: Vec<crate::attr::Attribute>,
    pub if_token: token::If,
    pub cond: Box<Expr>,
    pub then_branch: crate::stmt::Block,
    pub else_branch: Option<(token::Else, Box<Expr>)>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:445-459`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L445-L459)*

An `if` expression with an optional `else` block: `if expr { ... }
else { ... }`.

The `else` branch expression may only be an `If` or `Block`
expression, not any of the other types of expression.

#### Implementations

- <span id="crateexprif-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprIf`

- <span id="exprif-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprIf`

- <span id="exprif-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprIf`

- <span id="exprif-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprIf`

- <span id="crateexprif-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprIf`

- <span id="exprif-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprIf`

- <span id="crateexprif-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIf`

##### `impl<T> From for ExprIf`

- <span id="exprif-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprIf`

- <span id="crateexprif-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprIf`

- <span id="exprif-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprIf`

- <span id="crateexprexprif-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprIf`

- <span id="crateexprif-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIf`

##### `impl Spanned for ExprIf`

- <span id="exprif-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprIf`

- <span id="exprif-toowned-type-owned"></span>`type Owned = T`

- <span id="exprif-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprif-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprIf`

- <span id="crateexprexprif-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprIf`

- <span id="exprif-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprif-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprIf`

- <span id="exprif-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprif-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprIndex`

```rust
struct ExprIndex {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub bracket_token: token::Bracket,
    pub index: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:461-470`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L461-L470)*

A square bracketed indexing expression: `vector[2]`.

#### Implementations

- <span id="crateexprindex-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprIndex`

- <span id="exprindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprIndex`

- <span id="exprindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprIndex`

- <span id="exprindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprIndex`

- <span id="crateexprindex-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprIndex`

- <span id="exprindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprIndex`

- <span id="crateexprindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprIndex`

##### `impl<T> From for ExprIndex`

- <span id="exprindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprIndex`

- <span id="crateexprindex-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprIndex`

- <span id="exprindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprIndex`

- <span id="crateexprexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprIndex`

- <span id="crateexprindex-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprIndex`

##### `impl Spanned for ExprIndex`

- <span id="exprindex-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprIndex`

- <span id="exprindex-toowned-type-owned"></span>`type Owned = T`

- <span id="exprindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprIndex`

- <span id="crateexprexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprIndex`

- <span id="exprindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprIndex`

- <span id="exprindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprInfer`

```rust
struct ExprInfer {
    pub attrs: Vec<crate::attr::Attribute>,
    pub underscore_token: token::Underscore,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:472-479`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L472-L479)*

The inferred value of a const generic argument, denoted `_`.

#### Implementations

- <span id="crateexprinfer-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprInfer`

- <span id="exprinfer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprInfer`

- <span id="exprinfer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprInfer`

- <span id="exprinfer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprInfer`

- <span id="crateexprinfer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprInfer`

- <span id="exprinfer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprInfer`

- <span id="crateexprinfer-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprInfer`

##### `impl<T> From for ExprInfer`

- <span id="exprinfer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprInfer`

- <span id="crateexprinfer-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprInfer`

- <span id="exprinfer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprInfer`

- <span id="crateexprinfer-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprInfer`

##### `impl Spanned for ExprInfer`

- <span id="exprinfer-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprInfer`

- <span id="exprinfer-toowned-type-owned"></span>`type Owned = T`

- <span id="exprinfer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprinfer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprInfer`

- <span id="crateexprexprinfer-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprInfer`

- <span id="exprinfer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprinfer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprInfer`

- <span id="exprinfer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprinfer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLet`

```rust
struct ExprLet {
    pub attrs: Vec<crate::attr::Attribute>,
    pub let_token: token::Let,
    pub pat: Box<crate::pat::Pat>,
    pub eq_token: token::Eq,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:481-491`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L481-L491)*

A `let` guard: `let Some(x) = opt`.

#### Implementations

- <span id="crateexprlet-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLet`

- <span id="exprlet-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLet`

- <span id="exprlet-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLet`

- <span id="exprlet-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLet`

- <span id="crateexprlet-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLet`

- <span id="exprlet-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLet`

- <span id="crateexprlet-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLet`

##### `impl<T> From for ExprLet`

- <span id="exprlet-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLet`

- <span id="crateexprlet-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLet`

- <span id="exprlet-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLet`

- <span id="crateexprexprlet-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLet`

- <span id="crateexprlet-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLet`

##### `impl Spanned for ExprLet`

- <span id="exprlet-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLet`

- <span id="exprlet-toowned-type-owned"></span>`type Owned = T`

- <span id="exprlet-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprlet-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLet`

- <span id="crateexprexprlet-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLet`

- <span id="exprlet-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprlet-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLet`

- <span id="exprlet-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprlet-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprLit`

```rust
struct ExprLit {
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

### `ExprLoop`

```rust
struct ExprLoop {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub loop_token: token::Loop,
    pub body: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:502-511`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L502-L511)*

Conditionless loop: `loop { ... }`.

#### Implementations

- <span id="crateexprloop-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprLoop`

- <span id="exprloop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprLoop`

- <span id="exprloop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprLoop`

- <span id="exprloop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprLoop`

- <span id="crateexprloop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprLoop`

- <span id="exprloop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprLoop`

- <span id="crateexprloop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprLoop`

##### `impl<T> From for ExprLoop`

- <span id="exprloop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprLoop`

- <span id="crateexprloop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprLoop`

- <span id="exprloop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprLoop`

- <span id="crateexprexprloop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprLoop`

- <span id="crateexprloop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprLoop`

##### `impl Spanned for ExprLoop`

- <span id="exprloop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprLoop`

- <span id="exprloop-toowned-type-owned"></span>`type Owned = T`

- <span id="exprloop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprloop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprLoop`

- <span id="crateexprexprloop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprLoop`

- <span id="exprloop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprloop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprLoop`

- <span id="exprloop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprloop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMacro`

```rust
struct ExprMacro {
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

### `ExprMatch`

```rust
struct ExprMatch {
    pub attrs: Vec<crate::attr::Attribute>,
    pub match_token: token::Match,
    pub expr: Box<Expr>,
    pub brace_token: token::Brace,
    pub arms: Vec<Arm>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:522-532`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L522-L532)*

A `match` expression: `match n { Some(n) => {}, None => {} }`.

#### Implementations

- <span id="crateexprmatch-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMatch`

- <span id="exprmatch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMatch`

- <span id="exprmatch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMatch`

- <span id="exprmatch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMatch`

- <span id="crateexprmatch-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMatch`

- <span id="exprmatch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMatch`

- <span id="crateexprmatch-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMatch`

##### `impl<T> From for ExprMatch`

- <span id="exprmatch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMatch`

- <span id="crateexprmatch-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMatch`

- <span id="exprmatch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMatch`

- <span id="crateexprmatch-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMatch`

##### `impl Spanned for ExprMatch`

- <span id="exprmatch-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMatch`

- <span id="exprmatch-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmatch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmatch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMatch`

- <span id="crateexprexprmatch-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMatch`

- <span id="exprmatch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmatch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMatch`

- <span id="exprmatch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmatch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprMethodCall`

```rust
struct ExprMethodCall {
    pub attrs: Vec<crate::attr::Attribute>,
    pub receiver: Box<Expr>,
    pub dot_token: token::Dot,
    pub method: crate::ident::Ident,
    pub turbofish: Option<crate::path::AngleBracketedGenericArguments>,
    pub paren_token: token::Paren,
    pub args: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:534-546`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L534-L546)*

A method call expression: `x.foo::<T>(a, b)`.

#### Implementations

- <span id="crateexprmethodcall-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprMethodCall`

- <span id="exprmethodcall-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprMethodCall`

- <span id="exprmethodcall-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprMethodCall`

- <span id="exprmethodcall-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprMethodCall`

- <span id="crateexprmethodcall-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprMethodCall`

- <span id="exprmethodcall-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprMethodCall`

- <span id="crateexprmethodcall-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprMethodCall`

##### `impl<T> From for ExprMethodCall`

- <span id="exprmethodcall-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprMethodCall`

- <span id="crateexprmethodcall-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprMethodCall`

- <span id="exprmethodcall-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprMethodCall`

- <span id="crateexprmethodcall-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprMethodCall`

##### `impl Spanned for ExprMethodCall`

- <span id="exprmethodcall-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprMethodCall`

- <span id="exprmethodcall-toowned-type-owned"></span>`type Owned = T`

- <span id="exprmethodcall-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprmethodcall-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprMethodCall`

- <span id="crateexprexprmethodcall-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprMethodCall`

- <span id="exprmethodcall-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprmethodcall-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprMethodCall`

- <span id="exprmethodcall-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprmethodcall-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprParen`

```rust
struct ExprParen {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:548-556`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L548-L556)*

A parenthesized expression: `(a + b)`.

#### Implementations

- <span id="crateexprparen-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprParen`

- <span id="exprparen-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprParen`

- <span id="exprparen-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprParen`

- <span id="exprparen-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprParen`

- <span id="crateexprparen-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprParen`

- <span id="exprparen-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprParen`

- <span id="crateexprparen-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprParen`

##### `impl<T> From for ExprParen`

- <span id="exprparen-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprParen`

- <span id="crateexprparen-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprParen`

- <span id="exprparen-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprParen`

- <span id="crateexprexprparen-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprParen`

- <span id="crateexprparen-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprParen`

##### `impl Spanned for ExprParen`

- <span id="exprparen-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprParen`

- <span id="exprparen-toowned-type-owned"></span>`type Owned = T`

- <span id="exprparen-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprparen-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprParen`

- <span id="crateexprexprparen-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprParen`

- <span id="exprparen-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprparen-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprParen`

- <span id="exprparen-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprparen-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprPath`

```rust
struct ExprPath {
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

### `ExprRange`

```rust
struct ExprRange {
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

### `ExprRawAddr`

```rust
struct ExprRawAddr {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub raw: token::Raw,
    pub mutability: PointerMutability,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:582-592`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L582-L592)*

Address-of operation: `&raw const place` or `&raw mut place`.

#### Implementations

- <span id="crateexprrawaddr-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRawAddr`

- <span id="exprrawaddr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRawAddr`

- <span id="exprrawaddr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRawAddr`

- <span id="exprrawaddr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRawAddr`

- <span id="crateexprrawaddr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRawAddr`

- <span id="exprrawaddr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRawAddr`

- <span id="crateexprrawaddr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRawAddr`

##### `impl<T> From for ExprRawAddr`

- <span id="exprrawaddr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRawAddr`

- <span id="crateexprrawaddr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRawAddr`

- <span id="exprrawaddr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRawAddr`

- <span id="crateexprrawaddr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRawAddr`

##### `impl Spanned for ExprRawAddr`

- <span id="exprrawaddr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRawAddr`

- <span id="exprrawaddr-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrawaddr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrawaddr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRawAddr`

- <span id="crateexprexprrawaddr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRawAddr`

- <span id="exprrawaddr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrawaddr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRawAddr`

- <span id="exprrawaddr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrawaddr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReference`

```rust
struct ExprReference {
    pub attrs: Vec<crate::attr::Attribute>,
    pub and_token: token::And,
    pub mutability: Option<token::Mut>,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:594-603`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L594-L603)*

A referencing operation: `&a` or `&mut a`.

#### Implementations

- <span id="crateexprreference-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprReference`

- <span id="exprreference-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprReference`

- <span id="exprreference-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprReference`

- <span id="exprreference-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprReference`

- <span id="crateexprreference-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprReference`

- <span id="exprreference-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprReference`

- <span id="crateexprreference-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReference`

##### `impl<T> From for ExprReference`

- <span id="exprreference-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprReference`

- <span id="crateexprreference-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprReference`

- <span id="exprreference-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprReference`

- <span id="crateexprexprreference-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprReference`

- <span id="crateexprreference-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReference`

##### `impl Spanned for ExprReference`

- <span id="exprreference-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprReference`

- <span id="exprreference-toowned-type-owned"></span>`type Owned = T`

- <span id="exprreference-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprreference-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprReference`

- <span id="crateexprexprreference-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprReference`

- <span id="exprreference-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprreference-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprReference`

- <span id="exprreference-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprreference-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprRepeat`

```rust
struct ExprRepeat {
    pub attrs: Vec<crate::attr::Attribute>,
    pub bracket_token: token::Bracket,
    pub expr: Box<Expr>,
    pub semi_token: token::Semi,
    pub len: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:605-615`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L605-L615)*

An array literal constructed from one repeated element: `[0u8; N]`.

#### Implementations

- <span id="crateexprrepeat-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprRepeat`

- <span id="exprrepeat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprRepeat`

- <span id="exprrepeat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprRepeat`

- <span id="exprrepeat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprRepeat`

- <span id="crateexprrepeat-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprRepeat`

- <span id="exprrepeat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprRepeat`

- <span id="crateexprrepeat-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprRepeat`

##### `impl<T> From for ExprRepeat`

- <span id="exprrepeat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprRepeat`

- <span id="crateexprrepeat-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprRepeat`

- <span id="exprrepeat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprRepeat`

- <span id="crateexprrepeat-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprRepeat`

##### `impl Spanned for ExprRepeat`

- <span id="exprrepeat-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprRepeat`

- <span id="exprrepeat-toowned-type-owned"></span>`type Owned = T`

- <span id="exprrepeat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprrepeat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprRepeat`

- <span id="crateexprexprrepeat-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprRepeat`

- <span id="exprrepeat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprrepeat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprRepeat`

- <span id="exprrepeat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprrepeat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprReturn`

```rust
struct ExprReturn {
    pub attrs: Vec<crate::attr::Attribute>,
    pub return_token: token::Return,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:617-625`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L617-L625)*

A `return`, with an optional value to be returned.

#### Implementations

- <span id="crateexprreturn-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprReturn`

- <span id="exprreturn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprReturn`

- <span id="exprreturn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprReturn`

- <span id="exprreturn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprReturn`

- <span id="crateexprreturn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprReturn`

- <span id="exprreturn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprReturn`

- <span id="crateexprreturn-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprReturn`

##### `impl<T> From for ExprReturn`

- <span id="exprreturn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprReturn`

- <span id="crateexprreturn-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprReturn`

- <span id="exprreturn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprReturn`

- <span id="crateexprreturn-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprReturn`

##### `impl Spanned for ExprReturn`

- <span id="exprreturn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprReturn`

- <span id="exprreturn-toowned-type-owned"></span>`type Owned = T`

- <span id="exprreturn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprreturn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprReturn`

- <span id="crateexprexprreturn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprReturn`

- <span id="exprreturn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprreturn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprReturn`

- <span id="exprreturn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprreturn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprStruct`

```rust
struct ExprStruct {
    pub attrs: Vec<crate::attr::Attribute>,
    pub qself: Option<crate::path::QSelf>,
    pub path: crate::path::Path,
    pub brace_token: token::Brace,
    pub fields: crate::punctuated::Punctuated<FieldValue, token::Comma>,
    pub dot2_token: Option<token::DotDot>,
    pub rest: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:627-642`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L627-L642)*

A struct literal expression: `Point { x: 1, y: 1 }`.

The `rest` provides the value of the remaining fields as in `S { a:
1, b: 1, ..rest }`.

#### Implementations

- <span id="crateexprstruct-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprStruct`

- <span id="exprstruct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprStruct`

- <span id="exprstruct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprStruct`

- <span id="exprstruct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprStruct`

- <span id="crateexprstruct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprStruct`

- <span id="exprstruct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprStruct`

- <span id="crateexprstruct-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprStruct`

##### `impl<T> From for ExprStruct`

- <span id="exprstruct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprStruct`

- <span id="crateexprstruct-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprStruct`

- <span id="exprstruct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprStruct`

- <span id="crateexprstruct-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprStruct`

##### `impl Spanned for ExprStruct`

- <span id="exprstruct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprStruct`

- <span id="exprstruct-toowned-type-owned"></span>`type Owned = T`

- <span id="exprstruct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprstruct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprStruct`

- <span id="crateexprexprstruct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprStruct`

- <span id="exprstruct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprstruct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprStruct`

- <span id="exprstruct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprstruct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTry`

```rust
struct ExprTry {
    pub attrs: Vec<crate::attr::Attribute>,
    pub expr: Box<Expr>,
    pub question_token: token::Question,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:644-652`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L644-L652)*

A try-expression: `expr?`.

#### Implementations

- <span id="crateexprtry-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTry`

- <span id="exprtry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTry`

- <span id="exprtry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTry`

- <span id="exprtry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTry`

- <span id="crateexprtry-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTry`

- <span id="exprtry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTry`

- <span id="crateexprtry-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTry`

##### `impl<T> From for ExprTry`

- <span id="exprtry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTry`

- <span id="crateexprtry-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTry`

- <span id="exprtry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTry`

- <span id="crateexprexprtry-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTry`

- <span id="crateexprtry-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTry`

##### `impl Spanned for ExprTry`

- <span id="exprtry-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTry`

- <span id="exprtry-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTry`

- <span id="crateexprexprtry-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTry`

- <span id="exprtry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTry`

- <span id="exprtry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTryBlock`

```rust
struct ExprTryBlock {
    pub attrs: Vec<crate::attr::Attribute>,
    pub try_token: token::Try,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:654-662`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L654-L662)*

A try block: `try { ... }`.

#### Implementations

- <span id="crateexprtryblock-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTryBlock`

- <span id="exprtryblock-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTryBlock`

- <span id="exprtryblock-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTryBlock`

- <span id="exprtryblock-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTryBlock`

- <span id="crateexprtryblock-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTryBlock`

- <span id="exprtryblock-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTryBlock`

- <span id="crateexprtryblock-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTryBlock`

##### `impl<T> From for ExprTryBlock`

- <span id="exprtryblock-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTryBlock`

- <span id="crateexprtryblock-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTryBlock`

- <span id="exprtryblock-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTryBlock`

- <span id="crateexprtryblock-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTryBlock`

##### `impl Spanned for ExprTryBlock`

- <span id="exprtryblock-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTryBlock`

- <span id="exprtryblock-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtryblock-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtryblock-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTryBlock`

- <span id="crateexprexprtryblock-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTryBlock`

- <span id="exprtryblock-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtryblock-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTryBlock`

- <span id="exprtryblock-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtryblock-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprTuple`

```rust
struct ExprTuple {
    pub attrs: Vec<crate::attr::Attribute>,
    pub paren_token: token::Paren,
    pub elems: crate::punctuated::Punctuated<Expr, token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:664-672`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L664-L672)*

A tuple expression: `(a, b, c, d)`.

#### Implementations

- <span id="crateexprtuple-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprTuple`

- <span id="exprtuple-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprTuple`

- <span id="exprtuple-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprTuple`

- <span id="exprtuple-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprTuple`

- <span id="crateexprtuple-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprTuple`

- <span id="exprtuple-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprTuple`

- <span id="crateexprtuple-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprTuple`

##### `impl<T> From for ExprTuple`

- <span id="exprtuple-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprTuple`

- <span id="crateexprtuple-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprTuple`

- <span id="exprtuple-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprTuple`

- <span id="crateexprtuple-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprTuple`

##### `impl Spanned for ExprTuple`

- <span id="exprtuple-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprTuple`

- <span id="exprtuple-toowned-type-owned"></span>`type Owned = T`

- <span id="exprtuple-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprtuple-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprTuple`

- <span id="crateexprexprtuple-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprTuple`

- <span id="exprtuple-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprtuple-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprTuple`

- <span id="exprtuple-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprtuple-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnary`

```rust
struct ExprUnary {
    pub attrs: Vec<crate::attr::Attribute>,
    pub op: crate::op::UnOp,
    pub expr: Box<Expr>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:674-682`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L674-L682)*

A unary operation: `!x`, `*x`.

#### Implementations

- <span id="crateexprunary-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprUnary`

- <span id="exprunary-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprUnary`

- <span id="exprunary-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprUnary`

- <span id="exprunary-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprUnary`

- <span id="crateexprunary-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprUnary`

- <span id="exprunary-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprUnary`

- <span id="crateexprunary-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnary`

##### `impl<T> From for ExprUnary`

- <span id="exprunary-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprUnary`

- <span id="crateexprunary-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprUnary`

- <span id="exprunary-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprUnary`

- <span id="crateexprexprunary-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprUnary`

- <span id="crateexprunary-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnary`

##### `impl Spanned for ExprUnary`

- <span id="exprunary-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprUnary`

- <span id="exprunary-toowned-type-owned"></span>`type Owned = T`

- <span id="exprunary-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprunary-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprUnary`

- <span id="crateexprexprunary-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprUnary`

- <span id="exprunary-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprunary-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprUnary`

- <span id="exprunary-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprunary-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprUnsafe`

```rust
struct ExprUnsafe {
    pub attrs: Vec<crate::attr::Attribute>,
    pub unsafe_token: token::Unsafe,
    pub block: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:684-692`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L684-L692)*

An unsafe block: `unsafe { ... }`.

#### Implementations

- <span id="crateexprunsafe-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprUnsafe`

- <span id="exprunsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprUnsafe`

- <span id="exprunsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprUnsafe`

- <span id="exprunsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprUnsafe`

- <span id="crateexprunsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprUnsafe`

- <span id="exprunsafe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprUnsafe`

- <span id="crateexprunsafe-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprUnsafe`

##### `impl<T> From for ExprUnsafe`

- <span id="exprunsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprUnsafe`

- <span id="crateexprunsafe-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprUnsafe`

- <span id="exprunsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprUnsafe`

- <span id="crateexprunsafe-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprUnsafe`

##### `impl Spanned for ExprUnsafe`

- <span id="exprunsafe-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprUnsafe`

- <span id="exprunsafe-toowned-type-owned"></span>`type Owned = T`

- <span id="exprunsafe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprunsafe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprUnsafe`

- <span id="crateexprexprunsafe-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprUnsafe`

- <span id="exprunsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprunsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprUnsafe`

- <span id="exprunsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprunsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprWhile`

```rust
struct ExprWhile {
    pub attrs: Vec<crate::attr::Attribute>,
    pub label: Option<Label>,
    pub while_token: token::While,
    pub cond: Box<Expr>,
    pub body: crate::stmt::Block,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:694-704`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L694-L704)*

A while loop: `while expr { ... }`.

#### Implementations

- <span id="crateexprwhile-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprWhile`

- <span id="exprwhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprWhile`

- <span id="exprwhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprWhile`

- <span id="exprwhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprWhile`

- <span id="crateexprwhile-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprWhile`

- <span id="exprwhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprWhile`

- <span id="crateexprwhile-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprWhile`

##### `impl<T> From for ExprWhile`

- <span id="exprwhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprWhile`

- <span id="crateexprwhile-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprWhile`

- <span id="exprwhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprWhile`

- <span id="crateexprwhile-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprWhile`

##### `impl Spanned for ExprWhile`

- <span id="exprwhile-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprWhile`

- <span id="exprwhile-toowned-type-owned"></span>`type Owned = T`

- <span id="exprwhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exprwhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprWhile`

- <span id="crateexprexprwhile-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprWhile`

- <span id="exprwhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exprwhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprWhile`

- <span id="exprwhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exprwhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExprYield`

```rust
struct ExprYield {
    pub attrs: Vec<crate::attr::Attribute>,
    pub yield_token: token::Yield,
    pub expr: Option<Box<Expr>>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:706-714`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L706-L714)*

A yield expression: `yield expr`.

#### Implementations

- <span id="crateexpryield-debug"></span>`fn debug(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result`

#### Trait Implementations

##### `impl Any for ExprYield`

- <span id="expryield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExprYield`

- <span id="expryield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExprYield`

- <span id="expryield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::ExprYield`

- <span id="crateexpryield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ExprYield`

- <span id="expryield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::ExprYield`

- <span id="crateexpryield-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::ExprYield`

##### `impl<T> From for ExprYield`

- <span id="expryield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::ExprYield`

- <span id="crateexpryield-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for ExprYield`

- <span id="expryield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::ExprYield`

- <span id="crateexprexpryield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::ExprYield`

- <span id="crateexpryield-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for ExprYield`

##### `impl Spanned for ExprYield`

- <span id="expryield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ExprYield`

- <span id="expryield-toowned-type-owned"></span>`type Owned = T`

- <span id="expryield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="expryield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::ExprYield`

- <span id="crateexprexpryield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for ExprYield`

- <span id="expryield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expryield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExprYield`

- <span id="expryield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expryield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Index`

```rust
struct Index {
    pub index: u32,
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1049-1056`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1049-L1056)*

The index of an unnamed tuple struct field.

#### Trait Implementations

##### `impl Any for Index`

- <span id="index-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Index`

- <span id="index-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Index`

- <span id="index-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Index`

- <span id="crateindex-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Index`

- <span id="index-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Index`

- <span id="crateindex-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Index`

##### `impl<T> From for Index`

- <span id="index-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Index`

- <span id="index-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Index`

- <span id="index-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="index-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl<U> Into for Index`

- <span id="index-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Index`

- <span id="crateexprindex-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Index`

- <span id="index-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Index`

##### `impl Spanned for Index`

- <span id="index-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Index`

- <span id="index-toowned-type-owned"></span>`type Owned = T`

- <span id="index-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="index-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Index`

- <span id="crateexprindex-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Index`

- <span id="index-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="index-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Index`

- <span id="index-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="index-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FieldValue`

```rust
struct FieldValue {
    pub attrs: Vec<crate::attr::Attribute>,
    pub member: Member,
    pub colon_token: Option<token::Colon>,
    pub expr: Expr,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1093-1106`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1093-L1106)*

A field-value pair in a struct literal.

#### Fields

- **`colon_token`**: `Option<token::Colon>`

  The colon in `Struct { x: x }`. If written in shorthand like
  `Struct { x }`, there is no colon.

#### Trait Implementations

##### `impl Any for FieldValue`

- <span id="fieldvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FieldValue`

- <span id="fieldvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FieldValue`

- <span id="fieldvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::FieldValue`

- <span id="cratefieldvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FieldValue`

- <span id="fieldvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::FieldValue`

- <span id="cratefieldvalue-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::FieldValue`

##### `impl<T> From for FieldValue`

- <span id="fieldvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::FieldValue`

- <span id="cratefieldvalue-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for FieldValue`

- <span id="fieldvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::FieldValue`

- <span id="cratefieldvalue-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for FieldValue`

##### `impl Spanned for FieldValue`

- <span id="fieldvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FieldValue`

- <span id="fieldvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="fieldvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fieldvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::FieldValue`

- <span id="crateexprfieldvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for FieldValue`

- <span id="fieldvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fieldvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FieldValue`

- <span id="fieldvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fieldvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Label`

```rust
struct Label {
    pub name: crate::lifetime::Lifetime,
    pub colon_token: token::Colon,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1109-1116`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1109-L1116)*

A lifetime labeling a `for`, `while`, or `loop`.

#### Trait Implementations

##### `impl Any for Label`

- <span id="label-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Label`

- <span id="label-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Label`

- <span id="label-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Label`

- <span id="cratelabel-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Label`

- <span id="label-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Label`

- <span id="cratelabel-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Label`

##### `impl<T> From for Label`

- <span id="label-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Label`

- <span id="cratelabel-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Label`

- <span id="label-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Label`

- <span id="crateexprlabel-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Label`

- <span id="cratelabel-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Label`

##### `impl Spanned for Label`

- <span id="label-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Label`

- <span id="label-toowned-type-owned"></span>`type Owned = T`

- <span id="label-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="label-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Label`

- <span id="crateexprlabel-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Label`

- <span id="label-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="label-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Label`

- <span id="label-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="label-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Arm`

```rust
struct Arm {
    pub attrs: Vec<crate::attr::Attribute>,
    pub pat: crate::pat::Pat,
    pub guard: Option<(token::If, Box<Expr>)>,
    pub fat_arrow_token: token::FatArrow,
    pub body: Box<Expr>,
    pub comma: Option<token::Comma>,
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1119-1146`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1119-L1146)*

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

- <span id="crateexprarm-parse-multiple"></span>`fn parse_multiple(input: ParseStream<'_>) -> Result<Vec<Self>>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for Arm`

- <span id="arm-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Arm`

- <span id="arm-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Arm`

- <span id="arm-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Arm`

- <span id="cratearm-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Arm`

- <span id="arm-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Arm`

- <span id="cratearm-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Arm`

##### `impl<T> From for Arm`

- <span id="arm-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Arm`

- <span id="cratearm-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Arm`

- <span id="arm-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Arm`

- <span id="crateexprarm-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Arm>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Arm`](#arm)

##### `impl PartialEq for crate::Arm`

- <span id="cratearm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Arm`

##### `impl Spanned for Arm`

- <span id="arm-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Arm`

- <span id="arm-toowned-type-owned"></span>`type Owned = T`

- <span id="arm-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arm-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Arm`

- <span id="crateexprarm-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Arm`

- <span id="arm-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arm-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Arm`

- <span id="arm-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arm-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`syn-2.0.111/src/expr.rs:35-267`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L35-L267)*

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

- <span id="expr-const-placeholder"></span>`const PLACEHOLDER: Self`

- <span id="expr-parse-without-eager-brace"></span>`fn parse_without_eager_brace(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Expr`](#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](../parse/index.md)

  trait) for ambiguous syntactic positions in which a trailing brace

  should not be taken as part of the expression.

  

  Rust grammar has an ambiguity where braces sometimes turn a path

  expression into a struct initialization and sometimes do not. In the

  following code, the expression `S {}` is one expression. Presumably

  there is an empty struct `struct S {}` defined somewhere which it is

  instantiating.

  

  ```rust

  struct S;

  impl std::ops::Deref for S {

      type Target = bool;

      fn deref(&self) -> &Self::Target {

          &true

      }

  }

  let _ = *S {};

  

  // parsed by rustc as: `*(S {})`

  ```

  

  We would want to parse the above using `Expr::parse` after the `=`

  token.

  

  But in the following, `S {}` is *not* a struct init expression.

  

  ```rust

  const S: &bool = &true;

  if *S {} {}

  

  // parsed by rustc as:

  //

  //    if (*S) {

  //        /* empty block */

  //    }

  //    {

  //        /* another empty block */

  //    }

  ```

  

  For that reason we would want to parse if-conditions using

  `Expr::parse_without_eager_brace` after the `if` token. Same for similar

  syntactic positions such as the condition expr after a `while` token or

  the expr at the top of a `match`.

  

  The Rust grammar's choices around which way this ambiguity is resolved

  at various syntactic positions is fairly arbitrary. Really either parse

  behavior could work in most positions, and language designers just

  decide each case based on which is more likely to be what the programmer

  had in mind most of the time.

  

  ```rust

  struct S;

  fn doc() -> S {

  if return S {} {}

  unreachable!()

  }

  

  // parsed by rustc as:

  //

  //    if (return (S {})) {

  //    }

  //

  // but could equally well have been this other arbitrary choice:

  //

  //    if (return S) {

  //    }

  //    {}

  ```

  

  Note the grammar ambiguity on trailing braces is distinct from

  precedence and is not captured by assigning a precedence level to the

  braced struct init expr in relation to other operators. This can be

  illustrated by `return 0..S {}` vs `match 0..S {}`. The former parses as

  `return (0..(S {}))` implying tighter precedence for struct init than

  `..`, while the latter parses as `match (0..S) {}` implying tighter

  precedence for `..` than struct init, a contradiction.

- <span id="expr-parse-with-earlier-boundary-rule"></span>`fn parse_with_earlier_boundary_rule(input: ParseStream<'_>) -> Result<Expr>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result), [`Expr`](#expr)

  An alternative to the primary `Expr::parse` parser (from the [`Parse`](../parse/index.md)

  trait) for syntactic positions in which expression boundaries are placed

  more eagerly than done by the typical expression grammar. This includes

  expressions at the head of a statement or in the right-hand side of a

  `match` arm.

  

  Compare the following cases:

  

  1.

    ```rust

    let result = ();

    let guard = false;

    let cond = true;

    let f = true;

    let g = f;

  

    let _ = match result {

        () if guard => if cond { f } else { g }

        () => false,

    };

    ```

  

  2.

    ```rust

    let cond = true;

    let f = ();

    let g = f;

  

    let _ = || {

        if cond { f } else { g }

        ()

    };

    ```

  

  3.

    ```rust

    let cond = true;

    let f = || ();

    let g = f;

  

    let _ = [if cond { f } else { g } ()];

    ```

  

  The same sequence of tokens `if cond { f } else { g } ()` appears in

  expression position 3 times. The first two syntactic positions use eager

  placement of expression boundaries, and parse as `Expr::If`, with the

  adjacent `()` becoming `Pat::Tuple` or `Expr::Tuple`. In contrast, the

  third case uses standard expression boundaries and parses as

  `Expr::Call`.

  

  As with `parse_without_eager_brace`, this ambiguity in the Rust

  grammar is independent of precedence.

- <span id="expr-peek"></span>`fn peek(input: ParseStream<'_>) -> bool` — [`ParseStream`](../parse/index.md#parsestream)

  Returns whether the next token in the parse stream is one that might

  possibly form the beginning of an expr.

  

  This classification is a load-bearing part of the grammar of some Rust

  expressions, notably `return` and `break`. For example `return < …` will

  never parse `<` as a binary operator regardless of what comes after,

  because `<` is a legal starting token for an expression and so it's

  required to be continued as a return value, such as `return <Struct as

  Trait>::CONST`. Meanwhile `return > …` treats the `>` as a binary

  operator because it cannot be a starting token for any Rust expression.

- <span id="expr-replace-attrs"></span>`fn replace_attrs(&mut self, new: Vec<Attribute>) -> Vec<Attribute>` — [`Attribute`](../attr/index.md#attribute)

#### Trait Implementations

##### `impl Any for Expr`

- <span id="expr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expr`

- <span id="expr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expr`

- <span id="expr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Expr`

- <span id="crateexpr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Expr`

- <span id="expr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Expr`

- <span id="crateexpr-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::Expr`

##### `impl<T> From for Expr`

- <span id="expr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::Expr`

- <span id="crateexpr-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for Expr`

- <span id="expr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Expr`

- <span id="crateexprexpr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::Expr`

- <span id="crateexpr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Expr`

##### `impl Spanned for Expr`

- <span id="expr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Expr`

- <span id="expr-toowned-type-owned"></span>`type Owned = T`

- <span id="expr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="expr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Expr`

- <span id="expr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream)`

##### `impl<U> TryFrom for Expr`

- <span id="expr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Expr`

- <span id="expr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Member`

```rust
enum Member {
    Named(crate::ident::Ident),
    Unnamed(Index),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:971-981`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L971-L981)*

A struct or tuple struct field accessed in a struct literal or field
expression.

#### Variants

- **`Named`**

  A named field like `self.x`.

- **`Unnamed`**

  An unnamed field like `self.0`.

#### Implementations

- <span id="member-is-named"></span>`fn is_named(&self) -> bool`

#### Trait Implementations

##### `impl Any for Member`

- <span id="member-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Member`

- <span id="member-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Member`

- <span id="member-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::Member`

- <span id="cratemember-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Member`

- <span id="member-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::Member`

- <span id="cratemember-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Member`

##### `impl<T> From for Member`

- <span id="member-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Member`

- <span id="member-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl IdentFragment for Member`

- <span id="member-identfragment-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="member-identfragment-span"></span>`fn span(&self) -> Option<Span>`

##### `impl<U> Into for Member`

- <span id="member-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::Member`

- <span id="crateexprmember-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Member`

- <span id="member-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for Member`

##### `impl Spanned for Member`

- <span id="member-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Member`

- <span id="member-toowned-type-owned"></span>`type Owned = T`

- <span id="member-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="member-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::Member`

- <span id="crateexprmember-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for Member`

- <span id="member-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="member-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Member`

- <span id="member-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="member-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RangeLimits`

```rust
enum RangeLimits {
    HalfOpen(token::DotDot),
    Closed(token::DotDotEq),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1149-1158`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1149-L1158)*

Limit types of a range, inclusive or exclusive.

#### Variants

- **`HalfOpen`**

  Inclusive at the beginning, exclusive at the end.

- **`Closed`**

  Inclusive at the beginning and end.

#### Implementations

- <span id="crateexprrangelimits-parse-obsolete"></span>`fn parse_obsolete(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

#### Trait Implementations

##### `impl Any for RangeLimits`

- <span id="rangelimits-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeLimits`

- <span id="rangelimits-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeLimits`

- <span id="rangelimits-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::RangeLimits`

- <span id="craterangelimits-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for RangeLimits`

- <span id="rangelimits-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::RangeLimits`

##### `impl Debug for crate::RangeLimits`

- <span id="craterangelimits-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::RangeLimits`

##### `impl<T> From for RangeLimits`

- <span id="rangelimits-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::RangeLimits`

- <span id="craterangelimits-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for RangeLimits`

- <span id="rangelimits-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::RangeLimits`

- <span id="craterangelimits-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for RangeLimits`

##### `impl Spanned for RangeLimits`

- <span id="rangelimits-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for RangeLimits`

- <span id="rangelimits-toowned-type-owned"></span>`type Owned = T`

- <span id="rangelimits-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangelimits-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::RangeLimits`

- <span id="crateexprrangelimits-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for RangeLimits`

- <span id="rangelimits-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangelimits-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeLimits`

- <span id="rangelimits-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangelimits-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PointerMutability`

```rust
enum PointerMutability {
    Const(token::Const),
    Mut(token::Mut),
}
```

*Defined in [`syn-2.0.111/src/expr.rs:1161-1169`](../../../.source_1765633015/syn-2.0.111/src/expr.rs#L1161-L1169)*

Mutability of a raw pointer (`*const T`, `*mut T`), in which non-mutable
isn't the implicit default.

#### Trait Implementations

##### `impl Any for PointerMutability`

- <span id="pointermutability-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PointerMutability`

- <span id="pointermutability-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PointerMutability`

- <span id="pointermutability-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::PointerMutability`

- <span id="cratepointermutability-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PointerMutability`

- <span id="pointermutability-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for crate::PointerMutability`

- <span id="cratepointermutability-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::PointerMutability`

##### `impl<T> From for PointerMutability`

- <span id="pointermutability-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::PointerMutability`

- <span id="cratepointermutability-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for PointerMutability`

- <span id="pointermutability-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::PointerMutability`

- <span id="cratepointermutability-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for PointerMutability`

##### `impl Spanned for PointerMutability`

- <span id="pointermutability-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PointerMutability`

- <span id="pointermutability-toowned-type-owned"></span>`type Owned = T`

- <span id="pointermutability-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pointermutability-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::expr::PointerMutability`

- <span id="crateexprpointermutability-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for PointerMutability`

- <span id="pointermutability-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pointermutability-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PointerMutability`

- <span id="pointermutability-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pointermutability-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

