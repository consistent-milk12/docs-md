*[rustversion](../index.md) / [bound](index.md)*

---

# Module `bound`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bound`](#bound) | enum |  |
| [`parse`](#parse) | fn |  |

## Enums

### `Bound`

```rust
enum Bound {
    Nightly(crate::date::Date),
    Stable(crate::release::Release),
}
```

*Defined in [`rustversion-1.0.22/src/bound.rs:10-13`](../../../.source_1765633015/rustversion-1.0.22/src/bound.rs#L10-L13)*

#### Trait Implementations

##### `impl Any for Bound`

- <span id="bound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bound`

- <span id="bound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bound`

- <span id="bound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Bound`

- <span id="bound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Bound`

- <span id="bound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::version::Version`

- <span id="crateversionversion-partialeq-eq"></span>`fn eq(&self, rhs: &Bound) -> bool` — [`Bound`](#bound)

##### `impl PartialOrd for crate::version::Version`

- <span id="crateversionversion-partialord-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](#bound)

##### `impl<U> TryFrom for Bound`

- <span id="bound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bound`

- <span id="bound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Bound, Error>
```

*Defined in [`rustversion-1.0.22/src/bound.rs:15-31`](../../../.source_1765633015/rustversion-1.0.22/src/bound.rs#L15-L31)*

