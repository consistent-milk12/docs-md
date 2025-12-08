*[rayon](../../index.md) / [iter](../index.md) / [noop](index.md)*

---

# Module `noop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoopConsumer`](#noopconsumer) | struct |  |
| [`NoopReducer`](#noopreducer) | struct |  |

## Structs

### `NoopConsumer`

```rust
struct NoopConsumer;
```

#### Trait Implementations

##### `impl<T> Consumer for NoopConsumer`

- <span id="noopconsumer-folder"></span>`type Folder = NoopConsumer`

- <span id="noopconsumer-reducer"></span>`type Reducer = NoopReducer`

- <span id="noopconsumer-result"></span>`type Result = ()`

- <span id="noopconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](#noopreducer)

- <span id="noopconsumer-into-folder"></span>`fn into_folder(self) -> Self`

- <span id="noopconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> Folder for NoopConsumer`

- <span id="noopconsumer-result"></span>`type Result = ()`

- <span id="noopconsumer-consume"></span>`fn consume(self, _item: T) -> Self`

- <span id="noopconsumer-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="noopconsumer-complete"></span>`fn complete(self)`

- <span id="noopconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for NoopConsumer`

##### `impl<T> Pointable for NoopConsumer`

- <span id="noopconsumer-align"></span>`const ALIGN: usize`

- <span id="noopconsumer-init"></span>`type Init = T`

- <span id="noopconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="noopconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="noopconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="noopconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> UnindexedConsumer for NoopConsumer`

- <span id="noopconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="noopconsumer-to-reducer"></span>`fn to_reducer(&self) -> NoopReducer` — [`NoopReducer`](#noopreducer)

### `NoopReducer`

```rust
struct NoopReducer;
```

#### Trait Implementations

##### `impl<T> IntoEither for NoopReducer`

##### `impl<T> Pointable for NoopReducer`

- <span id="noopreducer-align"></span>`const ALIGN: usize`

- <span id="noopreducer-init"></span>`type Init = T`

- <span id="noopreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="noopreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="noopreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="noopreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Reducer for NoopReducer`

- <span id="noopreducer-reduce"></span>`fn reduce(self, _left: (), _right: ())`

