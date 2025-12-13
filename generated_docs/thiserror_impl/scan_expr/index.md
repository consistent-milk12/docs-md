*[thiserror_impl](../index.md) / [scan_expr](index.md)*

---

# Module `scan_expr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | enum |  |
| [`Action`](#action) | enum |  |
| [`scan_expr`](#scan-expr) | fn |  |

## Enums

### `Input`

```rust
enum Input {
    Keyword(&'static str),
    Punct(&'static str),
    ConsumeAny,
    ConsumeBinOp,
    ConsumeBrace,
    ConsumeDelimiter,
    ConsumeIdent,
    ConsumeLifetime,
    ConsumeLiteral,
    ConsumeNestedBrace,
    ExpectPath,
    ExpectTurbofish,
    ExpectType,
    CanBeginExpr,
    Otherwise,
    Empty,
}
```

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:6-23`](../../../.source_1765521767/thiserror-impl-2.0.17/src/scan_expr.rs#L6-L23)*

#### Trait Implementations

##### `impl Any for Input`

- <span id="input-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Input`

- <span id="input-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Input`

- <span id="input-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Input`

- <span id="input-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Input`

- <span id="input-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Input`

- <span id="input-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="input-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Input`

- <span id="input-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="input-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Action`

```rust
enum Action {
    SetState(&'static [(Input, Action)]),
    IncDepth,
    DecDepth,
    Finish,
}
```

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:25-30`](../../../.source_1765521767/thiserror-impl-2.0.17/src/scan_expr.rs#L25-L30)*

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `scan_expr`

```rust
fn scan_expr(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<()>
```

*Defined in [`thiserror-impl-2.0.17/src/scan_expr.rs:192-264`](../../../.source_1765521767/thiserror-impl-2.0.17/src/scan_expr.rs#L192-L264)*

