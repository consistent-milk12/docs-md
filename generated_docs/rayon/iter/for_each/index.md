*[rayon](../../index.md) / [iter](../index.md) / [for_each](index.md)*

---

# Module `for_each`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ForEachConsumer`](#foreachconsumer) | struct |  |
| [`for_each`](#for-each) | fn |  |

## Structs

### `ForEachConsumer<'f, F>`

```rust
struct ForEachConsumer<'f, F> {
    op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/for_each.rs:15-17`](../../../../.source_1765521767/rayon-1.11.0/src/iter/for_each.rs#L15-L17)*

#### Trait Implementations

##### `impl Any for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F, T> Consumer for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-consumer-type-folder"></span>`type Folder = ForEachConsumer<'f, F>`

- <span id="foreachconsumer-consumer-type-reducer"></span>`type Reducer = NoopReducer`

- <span id="foreachconsumer-consumer-type-result"></span>`type Result = ()`

- <span id="foreachconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](../noop/index.md#noopreducer)

- <span id="foreachconsumer-consumer-into-folder"></span>`fn into_folder(self) -> Self`

- <span id="foreachconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<F, T> Folder for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-folder-type-result"></span>`type Result = ()`

- <span id="foreachconsumer-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="foreachconsumer-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="foreachconsumer-folder-complete"></span>`fn complete(self)`

- <span id="foreachconsumer-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ForEachConsumer<'f, F>`

##### `impl Pointable for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foreachconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foreachconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foreachconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foreachconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foreachconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foreachconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foreachconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<F, T> UnindexedConsumer for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foreachconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> NoopReducer` — [`NoopReducer`](../noop/index.md#noopreducer)

## Functions

### `for_each`

```rust
fn for_each<I, F, T>(pi: I, op: &F)
where
    I: ParallelIterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/for_each.rs:5-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/for_each.rs#L5-L13)*

