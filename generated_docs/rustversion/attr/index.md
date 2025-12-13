*[rustversion](../index.md) / [attr](index.md)*

---

# Module `attr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Args`](#args) | struct |  |
| [`Then`](#then) | enum |  |
| [`parse`](#parse) | fn |  |

## Structs

### `Args`

```rust
struct Args {
    pub condition: crate::expr::Expr,
    pub then: Then,
}
```

*Defined in [`rustversion-1.0.22/src/attr.rs:6-9`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L6-L9)*

#### Trait Implementations

##### `impl Any for Args`

- <span id="args-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Args`

- <span id="args-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Args`

- <span id="args-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Args`

- <span id="args-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Args`

- <span id="args-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Args`

- <span id="args-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="args-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Args`

- <span id="args-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="args-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Then`

```rust
enum Then {
    Const(proc_macro::Span),
    Attribute(proc_macro::TokenStream),
}
```

*Defined in [`rustversion-1.0.22/src/attr.rs:11-14`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L11-L14)*

#### Trait Implementations

##### `impl Any for Then`

- <span id="then-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Then`

- <span id="then-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Then`

- <span id="then-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Then`

- <span id="then-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Then`

- <span id="then-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Then`

- <span id="then-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="then-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Then`

- <span id="then-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="then-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(input: proc_macro::TokenStream) -> std::result::Result<Args, Error>
```

*Defined in [`rustversion-1.0.22/src/attr.rs:16-35`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L16-L35)*

