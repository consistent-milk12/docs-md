*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [utils](index.md)*

---

# Module `utils`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CachePadded`](#cachepadded) | struct | Pads and aligns a value to the length of a cache line. |
| [`Backoff`](#backoff) | struct | Performs exponential backoff in spin loops. |
| [`SPIN_LIMIT`](#spin-limit) | const |  |

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:92-94`](../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L92-L94)*

Pads and aligns a value to the length of a cache line.

#### Implementations

- <span id="cachepadded-new"></span>`const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Any for CachePadded<T>`

- <span id="cachepadded-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CachePadded<T>`

- <span id="cachepadded-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CachePadded<T>`

- <span id="cachepadded-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Deref for CachePadded<T>`

- <span id="cachepadded-deref-type-target"></span>`type Target = T`

- <span id="cachepadded-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> From for CachePadded<T>`

- <span id="cachepadded-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CachePadded<T>`

- <span id="cachepadded-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> Receiver for CachePadded<T>`

- <span id="cachepadded-receiver-type-target"></span>`type Target = T`

##### `impl<T, U> TryFrom for CachePadded<T>`

- <span id="cachepadded-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cachepadded-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CachePadded<T>`

- <span id="cachepadded-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cachepadded-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Backoff`

```rust
struct Backoff {
    step: u32,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:115-117`](../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L115-L117)*

Performs exponential backoff in spin loops.

#### Implementations

- <span id="backoff-new"></span>`const fn new() -> Self`

- <span id="backoff-snooze"></span>`fn snooze(&mut self)`

#### Trait Implementations

##### `impl Any for Backoff`

- <span id="backoff-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Backoff`

- <span id="backoff-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Backoff`

- <span id="backoff-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Backoff`

- <span id="backoff-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Backoff`

- <span id="backoff-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Backoff`

- <span id="backoff-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="backoff-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Backoff`

- <span id="backoff-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="backoff-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `SPIN_LIMIT`
```rust
const SPIN_LIMIT: u32 = 4u32;
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:120`](../../../../../.source_1765633015/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L120)*

