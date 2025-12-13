*[tracing_core](../../index.md) / [callsite](../index.md) / [dispatchers](index.md)*

---

# Module `dispatchers`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dispatchers`](#dispatchers) | struct |  |
| [`Rebuilder`](#rebuilder) | enum |  |

## Structs

### `Dispatchers`

```rust
struct Dispatchers {
    has_just_one: std::sync::atomic::AtomicBool,
}
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:524-526`](../../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L524-L526)*

#### Implementations

- <span id="dispatchers-new"></span>`const fn new() -> Self`

- <span id="dispatchers-rebuilder"></span>`fn rebuilder(&self) -> Rebuilder<'_>` — [`Rebuilder`](#rebuilder)

- <span id="dispatchers-register-dispatch"></span>`fn register_dispatch(&self, dispatch: &dispatcher::Dispatch) -> Rebuilder<'_>` — [`Dispatch`](../../dispatcher/index.md#dispatch), [`Rebuilder`](#rebuilder)

#### Trait Implementations

##### `impl Any for Dispatchers`

- <span id="dispatchers-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dispatchers`

- <span id="dispatchers-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dispatchers`

- <span id="dispatchers-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Dispatchers`

- <span id="dispatchers-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Dispatchers`

- <span id="dispatchers-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Dispatchers`

- <span id="dispatchers-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dispatchers-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dispatchers`

- <span id="dispatchers-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dispatchers-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Rebuilder<'a>`

```rust
enum Rebuilder<'a> {
    JustOne,
    Read(std::sync::RwLockReadGuard<'a, alloc::vec::Vec<dispatcher::Registrar>>),
    Write(std::sync::RwLockWriteGuard<'a, alloc::vec::Vec<dispatcher::Registrar>>),
}
```

*Defined in [`tracing-core-0.1.35/src/callsite.rs:531-535`](../../../../.source_1765521767/tracing-core-0.1.35/src/callsite.rs#L531-L535)*

#### Implementations

- <span id="rebuilder-for-each"></span>`fn for_each(&self, f: impl FnMut(&dispatcher::Dispatch))` — [`Dispatch`](../../dispatcher/index.md#dispatch)

#### Trait Implementations

##### `impl Any for Rebuilder<'a>`

- <span id="rebuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rebuilder<'a>`

- <span id="rebuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rebuilder<'a>`

- <span id="rebuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Rebuilder<'a>`

- <span id="rebuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Rebuilder<'a>`

- <span id="rebuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Rebuilder<'a>`

- <span id="rebuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rebuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rebuilder<'a>`

- <span id="rebuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rebuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

