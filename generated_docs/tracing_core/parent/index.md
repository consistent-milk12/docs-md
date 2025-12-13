*[tracing_core](../index.md) / [parent](index.md)*

---

# Module `parent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parent`](#parent) | enum |  |

## Enums

### `Parent`

```rust
enum Parent {
    Root,
    Current,
    Explicit(crate::span::Id),
}
```

*Defined in [`tracing-core-0.1.35/src/parent.rs:4-11`](../../../.source_1765633015/tracing-core-0.1.35/src/parent.rs#L4-L11)*

#### Variants

- **`Root`**

  The new span will be a root span.

- **`Current`**

  The new span will be rooted in the current span.

- **`Explicit`**

  The new span has an explicitly-specified parent.

#### Trait Implementations

##### `impl Any for Parent`

- <span id="parent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parent`

- <span id="parent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parent`

- <span id="parent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Parent`

- <span id="parent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Parent`

- <span id="parent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parent`

- <span id="parent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Parent`

- <span id="parent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parent`

- <span id="parent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

