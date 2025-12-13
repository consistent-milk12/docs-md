*[regex_automata](../../../index.md) / [util](../../index.md) / [lazy](../index.md) / [lazy](index.md)*

---

# Module `lazy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lazy`](#lazy) | struct | A non-std lazy initialized value. |

## Structs

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    data: core::sync::atomic::AtomicPtr<T>,
    create: F,
    owned: core::marker::PhantomData<alloc::boxed::Box<T>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/lazy.rs:120-131`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/lazy.rs#L120-L131)*

A non-std lazy initialized value.

This might run the initialization function more than once, but will
never block.

I wish I could get these semantics into the non-alloc non-std Lazy
type below, but I'm not sure how to do it. If you can do an alloc,
then the implementation becomes very simple if you don't care about
redundant work precisely because a pointer can be atomically swapped.

Perhaps making this approach work in the non-alloc non-std case
requires asking the caller for a pointer? It would make the API less
convenient I think.

#### Implementations

- <span id="lazy-new"></span>`const fn new(create: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

  Create a new alloc but non-std lazy value that is racily

  initialized. That is, the 'create' function may be called more than

  once.

#### Trait Implementations

##### `impl<T> Any for Lazy<T, F>`

- <span id="lazy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lazy<T, F>`

- <span id="lazy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lazy<T, F>`

- <span id="lazy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, F: Fn() -> T> Debug for Lazy<T, F>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, F> Drop for Lazy<T, F>`

- <span id="lazy-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Lazy<T, F>`

- <span id="lazy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Lazy<T, F>`

- <span id="lazy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: Send + Sync, F: Send + Sync> Sync for Lazy<T, F>`

##### `impl<T, U> TryFrom for Lazy<T, F>`

- <span id="lazy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Lazy<T, F>`

- <span id="lazy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

