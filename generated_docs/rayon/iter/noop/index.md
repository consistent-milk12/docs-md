*[rayon](../../index.md) / [iter](../index.md) / [noop](index.md)*

---

# Module `noop`

## Structs

### `NoopConsumer`

```rust
struct NoopConsumer;
```

#### Trait Implementations

##### `impl<T> Consumer for NoopConsumer`

- `type Folder = NoopConsumer`

- `type Reducer = NoopReducer`

- `type Result = ()`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](#noopreducer)

- `fn into_folder(self: Self) -> Self`

- `fn full(self: &Self) -> bool`

##### `impl<T> Folder for NoopConsumer`

- `type Result = ()`

- `fn consume(self: Self, _item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self)`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for NoopConsumer`

##### `impl<T> Pointable for NoopConsumer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> UnindexedConsumer for NoopConsumer`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> NoopReducer` — [`NoopReducer`](#noopreducer)

### `NoopReducer`

```rust
struct NoopReducer;
```

#### Trait Implementations

##### `impl<T> IntoEither for NoopReducer`

##### `impl<T> Pointable for NoopReducer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Reducer for NoopReducer`

- `fn reduce(self: Self, _left: (), _right: ())`

