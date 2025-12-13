*[hashbrown](../index.md) / [scopeguard](index.md)*

---

# Module `scopeguard`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ScopeGuard`](#scopeguard) | struct |  |
| [`guard`](#guard) | fn |  |

## Structs

### `ScopeGuard<T, F>`

```rust
struct ScopeGuard<T, F>
where
    F: FnMut(&mut T) {
    dropfn: F,
    value: T,
}
```

*Defined in [`hashbrown-0.16.1/src/scopeguard.rs:8-14`](../../../.source_1765521767/hashbrown-0.16.1/src/scopeguard.rs#L8-L14)*

#### Implementations

- <span id="scopeguard-into-inner"></span>`fn into_inner(guard: Self) -> T`

#### Trait Implementations

##### `impl<T> Any for ScopeGuard<T, F>`

- <span id="scopeguard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ScopeGuard<T, F>`

- <span id="scopeguard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ScopeGuard<T, F>`

- <span id="scopeguard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, F> Deref for ScopeGuard<T, F>`

- <span id="scopeguard-deref-type-target"></span>`type Target = T`

- <span id="scopeguard-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, F> DerefMut for ScopeGuard<T, F>`

- <span id="scopeguard-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T, F> Drop for ScopeGuard<T, F>`

- <span id="scopeguard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for ScopeGuard<T, F>`

- <span id="scopeguard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ScopeGuard<T, F>`

- <span id="scopeguard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for ScopeGuard<T, F>`

- <span id="scopeguard-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for ScopeGuard<T, F>`

- <span id="scopeguard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="scopeguard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ScopeGuard<T, F>`

- <span id="scopeguard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="scopeguard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `guard`

```rust
fn guard<T, F>(value: T, dropfn: F) -> ScopeGuard<T, F>
where
    F: FnMut(&mut T)
```

*Defined in [`hashbrown-0.16.1/src/scopeguard.rs:17-22`](../../../.source_1765521767/hashbrown-0.16.1/src/scopeguard.rs#L17-L22)*

