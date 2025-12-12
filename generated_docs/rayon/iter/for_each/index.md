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

*Defined in [`rayon-1.11.0/src/iter/for_each.rs:15-17`](../../../../.source_1765210505/rayon-1.11.0/src/iter/for_each.rs#L15-L17)*

#### Trait Implementations

##### `impl<F, T> Consumer for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-consumer-type-folder"></span>`type Folder = ForEachConsumer<'f, F>`

- <span id="foreachconsumer-consumer-type-reducer"></span>`type Reducer = NoopReducer`

- <span id="foreachconsumer-consumer-type-result"></span>`type Result = ()`

- <span id="foreachconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](../noop/index.md#noopreducer)

- <span id="foreachconsumer-into-folder"></span>`fn into_folder(self) -> Self`

- <span id="foreachconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<F, T> Folder for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-folder-type-result"></span>`type Result = ()`

- <span id="foreachconsumer-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="foreachconsumer-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="foreachconsumer-complete"></span>`fn complete(self)`

- <span id="foreachconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for ForEachConsumer<'f, F>`

##### `impl Pointable for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foreachconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foreachconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foreachconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foreachconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foreachconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<F, T> UnindexedConsumer for ForEachConsumer<'f, F>`

- <span id="foreachconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foreachconsumer-to-reducer"></span>`fn to_reducer(&self) -> NoopReducer` — [`NoopReducer`](../noop/index.md#noopreducer)

## Functions

### `for_each`

```rust
fn for_each<I, F, T>(pi: I, op: &F)
where
    I: ParallelIterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/for_each.rs:5-13`](../../../../.source_1765210505/rayon-1.11.0/src/iter/for_each.rs#L5-L13)*

