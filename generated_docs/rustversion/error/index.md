*[rustversion](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct |  |
| [`Result`](#result) | type |  |

## Structs

### `Error`

```rust
struct Error {
    begin: proc_macro::Span,
    end: proc_macro::Span,
    msg: String,
}
```

*Defined in [`rustversion-1.0.22/src/error.rs:7-11`](../../../.source_1765521767/rustversion-1.0.22/src/error.rs#L7-L11)*

#### Implementations

- <span id="error-new"></span>`fn new(span: Span, msg: impl Display) -> Self`

- <span id="error-new2"></span>`fn new2(begin: Span, end: Span, msg: impl Display) -> Self`

- <span id="error-group"></span>`fn group(group: Group, msg: impl Display) -> Self`

- <span id="error-into-compile-error"></span>`fn into_compile_error(self) -> TokenStream`

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

*Defined in [`rustversion-1.0.22/src/error.rs:5`](../../../.source_1765521767/rustversion-1.0.22/src/error.rs#L5)*

