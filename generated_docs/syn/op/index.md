*[syn](../index.md) / [op](index.md)*

---

# Module `op`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parsing`](#parsing) | mod |  |
| [`printing`](#printing) | mod |  |
| [`BinOp`](#binop) | enum | A binary operator: `+`, `+=`, `&`. |
| [`UnOp`](#unop) | enum | A unary operator: `*`, `!`, `-`. |

## Modules

- [`parsing`](parsing/index.md)
- [`printing`](printing/index.md)

## Enums

### `BinOp`

```rust
enum BinOp {
    Add(token::Plus),
    Sub(token::Minus),
    Mul(token::Star),
    Div(token::Slash),
    Rem(token::Percent),
    And(token::AndAnd),
    Or(token::OrOr),
    BitXor(token::Caret),
    BitAnd(token::And),
    BitOr(token::Or),
    Shl(token::Shl),
    Shr(token::Shr),
    Eq(token::EqEq),
    Lt(token::Lt),
    Le(token::Le),
    Ne(token::Ne),
    Ge(token::Ge),
    Gt(token::Gt),
    AddAssign(token::PlusEq),
    SubAssign(token::MinusEq),
    MulAssign(token::StarEq),
    DivAssign(token::SlashEq),
    RemAssign(token::PercentEq),
    BitXorAssign(token::CaretEq),
    BitAndAssign(token::AndEq),
    BitOrAssign(token::OrEq),
    ShlAssign(token::ShlEq),
    ShrAssign(token::ShrEq),
}
```

*Defined in [`syn-2.0.111/src/op.rs:1-63`](../../../.source_1765521767/syn-2.0.111/src/op.rs#L1-L63)*

A binary operator: `+`, `+=`, `&`.

#### Variants

- **`Add`**

  The `+` operator (addition)

- **`Sub`**

  The `-` operator (subtraction)

- **`Mul`**

  The `*` operator (multiplication)

- **`Div`**

  The `/` operator (division)

- **`Rem`**

  The `%` operator (modulus)

- **`And`**

  The `&&` operator (logical and)

- **`Or`**

  The `||` operator (logical or)

- **`BitXor`**

  The `^` operator (bitwise xor)

- **`BitAnd`**

  The `&` operator (bitwise and)

- **`BitOr`**

  The `|` operator (bitwise or)

- **`Shl`**

  The `<<` operator (shift left)

- **`Shr`**

  The `>>` operator (shift right)

- **`Eq`**

  The `==` operator (equality)

- **`Lt`**

  The `<` operator (less than)

- **`Le`**

  The `<=` operator (less than or equal to)

- **`Ne`**

  The `!=` operator (not equal to)

- **`Ge`**

  The `>=` operator (greater than or equal to)

- **`Gt`**

  The `>` operator (greater than)

- **`AddAssign`**

  The `+=` operator

- **`SubAssign`**

  The `-=` operator

- **`MulAssign`**

  The `*=` operator

- **`DivAssign`**

  The `/=` operator

- **`RemAssign`**

  The `%=` operator

- **`BitXorAssign`**

  The `^=` operator

- **`BitAndAssign`**

  The `&=` operator

- **`BitOrAssign`**

  The `|=` operator

- **`ShlAssign`**

  The `<<=` operator

- **`ShrAssign`**

  The `>>=` operator

#### Trait Implementations

##### `impl Any for BinOp`

- <span id="binop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinOp`

- <span id="binop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinOp`

- <span id="binop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::BinOp`

- <span id="cratebinop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for BinOp`

- <span id="binop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- <span id="cratebinop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl<T> From for BinOp`

- <span id="binop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::BinOp`

- <span id="cratebinop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for BinOp`

- <span id="binop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::op::BinOp`

- <span id="crateopbinop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::BinOp`

- <span id="cratebinop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BinOp`

##### `impl Spanned for BinOp`

- <span id="binop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for BinOp`

- <span id="binop-toowned-type-owned"></span>`type Owned = T`

- <span id="binop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="binop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::op::BinOp`

- <span id="crateopbinop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for BinOp`

- <span id="binop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinOp`

- <span id="binop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UnOp`

```rust
enum UnOp {
    Deref(token::Star),
    Not(token::Not),
    Neg(token::Minus),
}
```

*Defined in [`syn-2.0.111/src/op.rs:65-77`](../../../.source_1765521767/syn-2.0.111/src/op.rs#L65-L77)*

A unary operator: `*`, `!`, `-`.

#### Variants

- **`Deref`**

  The `*` operator for dereferencing

- **`Not`**

  The `!` operator for logical inversion

- **`Neg`**

  The `-` operator for negation

#### Trait Implementations

##### `impl Any for UnOp`

- <span id="unop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnOp`

- <span id="unop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnOp`

- <span id="unop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for crate::UnOp`

- <span id="crateunop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for UnOp`

- <span id="unop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- <span id="crateunop-debug-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl<T> From for UnOp`

- <span id="unop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::UnOp`

- <span id="crateunop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<U> Into for UnOp`

- <span id="unop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for crate::op::UnOp`

- <span id="crateopunop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for crate::UnOp`

- <span id="crateunop-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UnOp`

##### `impl Spanned for UnOp`

- <span id="unop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for UnOp`

- <span id="unop-toowned-type-owned"></span>`type Owned = T`

- <span id="unop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for crate::op::UnOp`

- <span id="crateopunop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl<U> TryFrom for UnOp`

- <span id="unop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnOp`

- <span id="unop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

