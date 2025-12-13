*[serde_derive](../index.md) / [fragment](index.md)*

---

# Module `fragment`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | struct | Interpolate a fragment in place of an expression. |
| [`Stmts`](#stmts) | struct | Interpolate a fragment as the statements of a block. |
| [`Match`](#match) | struct | Interpolate a fragment as the value part of a `match` expression. |
| [`Fragment`](#fragment) | enum |  |
| [`quote_expr!`](#quote-expr) | macro |  |
| [`quote_block!`](#quote-block) | macro |  |

## Structs

### `Expr`

```rust
struct Expr(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:27`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L27)*

Interpolate a fragment in place of an expression. This involves surrounding
Block fragments in curly braces.

#### Trait Implementations

##### `impl Any for Expr`

- <span id="expr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Expr`

- <span id="expr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Expr`

- <span id="expr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Expr`

- <span id="expr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Expr`

- <span id="expr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Expr`

- <span id="expr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Expr`

- <span id="expr-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

##### `impl<U> TryFrom for Expr`

- <span id="expr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="expr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Expr`

- <span id="expr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="expr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Stmts`

```rust
struct Stmts(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:40`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L40)*

Interpolate a fragment as the statements of a block.

#### Trait Implementations

##### `impl Any for Stmts`

- <span id="stmts-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Stmts`

- <span id="stmts-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Stmts`

- <span id="stmts-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Stmts`

- <span id="stmts-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Stmts`

- <span id="stmts-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Stmts`

- <span id="stmts-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Stmts`

- <span id="stmts-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

##### `impl<U> TryFrom for Stmts`

- <span id="stmts-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stmts-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Stmts`

- <span id="stmts-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stmts-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match`

```rust
struct Match(Fragment);
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:52`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L52)*

Interpolate a fragment as the value part of a `match` expression. This
involves putting a comma after expressions and curly braces around blocks.

#### Trait Implementations

##### `impl Any for Match`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Match`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Match`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Spanned for Match`

- <span id="match-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for Match`

- <span id="match-totokens-to-tokens"></span>`fn to_tokens(&self, out: &mut TokenStream)`

##### `impl<U> TryFrom for Match`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Fragment`

```rust
enum Fragment {
    Expr(proc_macro2::TokenStream),
    Block(proc_macro2::TokenStream),
}
```

*Defined in [`serde_derive-1.0.228/src/fragment.rs:5-11`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L5-L11)*

#### Variants

- **`Expr`**

  Tokens that can be used as an expression.

- **`Block`**

  Tokens that can be used inside a block. The surrounding curly braces are
  not part of these tokens.

#### Trait Implementations

##### `impl Any for Fragment`

- <span id="fragment-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AsRef for Fragment`

- <span id="fragment-asref-as-ref"></span>`fn as_ref(&self) -> &TokenStream`

##### `impl<T> Borrow for Fragment`

- <span id="fragment-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fragment`

- <span id="fragment-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Fragment`

- <span id="fragment-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fragment`

- <span id="fragment-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Fragment`

- <span id="fragment-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fragment-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fragment`

- <span id="fragment-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fragment-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `quote_expr!`

*Defined in [`serde_derive-1.0.228/src/fragment.rs:13-17`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L13-L17)*

### `quote_block!`

*Defined in [`serde_derive-1.0.228/src/fragment.rs:19-23`](../../../.source_1765633015/serde_derive-1.0.228/src/fragment.rs#L19-L23)*

