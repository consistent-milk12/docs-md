*[syn](../index.md) / [precedence](index.md)*

---

# Module `precedence`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Precedence`](#precedence) | enum |  |

## Enums

### `Precedence`

```rust
enum Precedence {
    Jump,
    Assign,
    Range,
    Or,
    And,
    Let,
    Compare,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    Sum,
    Product,
    Cast,
    Prefix,
    Unambiguous,
}
```

*Defined in [`syn-2.0.111/src/precedence.rs:18-54`](../../../.source_1765521767/syn-2.0.111/src/precedence.rs#L18-L54)*

#### Implementations

- <span id="precedence-const-min"></span>`const MIN: Self`

- <span id="precedence-of-binop"></span>`fn of_binop(op: &BinOp) -> Self` — [`BinOp`](../op/index.md#binop)

- <span id="precedence-of"></span>`fn of(e: &Expr) -> Self` — [`Expr`](../expr/index.md#expr)

#### Trait Implementations

##### `impl Any for Precedence`

- <span id="precedence-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Precedence`

- <span id="precedence-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Precedence`

- <span id="precedence-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Precedence`

- <span id="precedence-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Precedence`

- <span id="precedence-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Precedence`

##### `impl<T> From for Precedence`

- <span id="precedence-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Precedence`

- <span id="precedence-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Precedence`

- <span id="precedence-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl PartialOrd for Precedence`

- <span id="precedence-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl ToOwned for Precedence`

- <span id="precedence-toowned-type-owned"></span>`type Owned = T`

- <span id="precedence-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="precedence-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Precedence`

- <span id="precedence-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="precedence-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Precedence`

- <span id="precedence-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="precedence-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

