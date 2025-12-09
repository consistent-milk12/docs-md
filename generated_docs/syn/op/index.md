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

*Defined in [`syn-2.0.111/src/op.rs:1-63`](../../../.source_1765210505/syn-2.0.111/src/op.rs#L1-L63)*

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

##### `impl Clone for crate::BinOp`

- <span id="cratebinop-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- <span id="cratebinop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl Hash for crate::BinOp`

- <span id="cratebinop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::op::BinOp`

- <span id="crateopbinop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::BinOp`

- <span id="cratebinop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for BinOp`

##### `impl Spanned for BinOp`

- <span id="binop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::op::BinOp`

- <span id="crateopbinop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `UnOp`

```rust
enum UnOp {
    Deref(token::Star),
    Not(token::Not),
    Neg(token::Minus),
}
```

*Defined in [`syn-2.0.111/src/op.rs:65-77`](../../../.source_1765210505/syn-2.0.111/src/op.rs#L65-L77)*

A unary operator: `*`, `!`, `-`.

#### Variants

- **`Deref`**

  The `*` operator for dereferencing

- **`Not`**

  The `!` operator for logical inversion

- **`Neg`**

  The `-` operator for negation

#### Trait Implementations

##### `impl Clone for crate::UnOp`

- <span id="crateunop-clone"></span>`fn clone(&self) -> Self`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- <span id="crateunop-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl Hash for crate::UnOp`

- <span id="crateunop-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl Parse for crate::op::UnOp`

- <span id="crateopunop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::UnOp`

- <span id="crateunop-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for UnOp`

##### `impl Spanned for UnOp`

- <span id="unop-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for crate::op::UnOp`

- <span id="crateopunop-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

