*[rayon](../../index.md) / [iter](../index.md) / [sum](index.md)*

---

# Module `sum`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SumConsumer`](#sumconsumer) | struct |  |
| [`SumFolder`](#sumfolder) | struct |  |
| [`sum`](#sum) | fn |  |
| [`add`](#add) | fn |  |

## Structs

### `SumConsumer<S: Send>`

```rust
struct SumConsumer<S: Send> {
    _marker: std::marker::PhantomData<*const S>,
}
```

#### Implementations

- <span id="sumconsumer-new"></span>`fn new() -> SumConsumer<S>` — [`SumConsumer`](#sumconsumer)

#### Trait Implementations

##### `impl<S, T> Consumer for SumConsumer<S>`

- <span id="sumconsumer-folder"></span>`type Folder = SumFolder<S>`

- <span id="sumconsumer-reducer"></span>`type Reducer = SumConsumer<S>`

- <span id="sumconsumer-result"></span>`type Result = S`

- <span id="sumconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="sumconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="sumconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for SumConsumer<S>`

##### `impl<T> Pointable for SumConsumer<S>`

- <span id="sumconsumer-align"></span>`const ALIGN: usize`

- <span id="sumconsumer-init"></span>`type Init = T`

- <span id="sumconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sumconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sumconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sumconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<S> Reducer for SumConsumer<S>`

- <span id="sumconsumer-reduce"></span>`fn reduce(self, left: S, right: S) -> S`

##### `impl<S: Send> Send for SumConsumer<S>`

##### `impl<S, T> UnindexedConsumer for SumConsumer<S>`

- <span id="sumconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="sumconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `SumFolder<S>`

```rust
struct SumFolder<S> {
    sum: S,
}
```

#### Trait Implementations

##### `impl<S, T> Folder for SumFolder<S>`

- <span id="sumfolder-result"></span>`type Result = S`

- <span id="sumfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="sumfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="sumfolder-complete"></span>`fn complete(self) -> S`

- <span id="sumfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for SumFolder<S>`

##### `impl<T> Pointable for SumFolder<S>`

- <span id="sumfolder-align"></span>`const ALIGN: usize`

- <span id="sumfolder-init"></span>`type Init = T`

- <span id="sumfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sumfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sumfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sumfolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `sum`

```rust
fn sum<PI, S>(pi: PI) -> S
where
    PI: ParallelIterator,
    S: Send + Sum<<PI as >::Item> + Sum
```

### `add`

```rust
fn add<T: Sum>(left: T, right: T) -> T
```

