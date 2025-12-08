*[rayon](../../index.md) / [iter](../index.md) / [for_each](index.md)*

---

# Module `for_each`

## Structs

### `ForEachConsumer<'f, F>`

```rust
struct ForEachConsumer<'f, F> {
    op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, F, T> Consumer for ForEachConsumer<'f, F>`

- `type Folder = ForEachConsumer<'f, F>`

- `type Reducer = NoopReducer`

- `type Result = ()`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](../noop/index.md)

- `fn into_folder(self: Self) -> Self`

- `fn full(self: &Self) -> bool`

##### `impl<'f, F, T> Folder for ForEachConsumer<'f, F>`

- `type Result = ()`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self)`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ForEachConsumer<'f, F>`

##### `impl<T> Pointable for ForEachConsumer<'f, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, F, T> UnindexedConsumer for ForEachConsumer<'f, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> NoopReducer` — [`NoopReducer`](../noop/index.md)

## Functions

### `for_each`

```rust
fn for_each<I, F, T>(pi: I, op: &F)
where
    I: ParallelIterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send
```

