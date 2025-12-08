*[syn](../index.md) / [op](index.md)*

---

# Module `op`

## Modules

- [`parsing`](parsing/index.md) - 
- [`printing`](printing/index.md) - 

## Enums

### `BinOp`

```rust
enum BinOp {
    Add($crate::token::Plus),
    Sub($crate::token::Minus),
    Mul($crate::token::Star),
    Div($crate::token::Slash),
    Rem($crate::token::Percent),
    And($crate::token::AndAnd),
    Or($crate::token::OrOr),
    BitXor($crate::token::Caret),
    BitAnd($crate::token::And),
    BitOr($crate::token::Or),
    Shl($crate::token::Shl),
    Shr($crate::token::Shr),
    Eq($crate::token::EqEq),
    Lt($crate::token::Lt),
    Le($crate::token::Le),
    Ne($crate::token::Ne),
    Ge($crate::token::Ge),
    Gt($crate::token::Gt),
    AddAssign($crate::token::PlusEq),
    SubAssign($crate::token::MinusEq),
    MulAssign($crate::token::StarEq),
    DivAssign($crate::token::SlashEq),
    RemAssign($crate::token::PercentEq),
    BitXorAssign($crate::token::CaretEq),
    BitAndAssign($crate::token::AndEq),
    BitOrAssign($crate::token::OrEq),
    ShlAssign($crate::token::ShlEq),
    ShrAssign($crate::token::ShrEq),
}
```

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

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::BinOp`

##### `impl Debug for crate::BinOp`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::BinOp`

##### `impl Hash for crate::BinOp`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::op::BinOp`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::BinOp`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for BinOp`

##### `impl<T> Spanned for BinOp`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::op::BinOp`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `UnOp`

```rust
enum UnOp {
    Deref($crate::token::Star),
    Not($crate::token::Not),
    Neg($crate::token::Minus),
}
```

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

- `fn clone(self: &Self) -> Self`

##### `impl Copy for crate::UnOp`

##### `impl Debug for crate::UnOp`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for crate::UnOp`

##### `impl Hash for crate::UnOp`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl Parse for crate::op::UnOp`

- `fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md), [`Result`](../error/index.md)

##### `impl PartialEq for crate::UnOp`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T> Sealed for UnOp`

##### `impl<T> Spanned for UnOp`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for crate::op::UnOp`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

