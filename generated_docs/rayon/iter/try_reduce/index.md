*[rayon](../../index.md) / [iter](../index.md) / [try_reduce](index.md)*

---

# Module `try_reduce`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReduceConsumer`](#tryreduceconsumer) | struct |  |
| [`TryReduceFolder`](#tryreducefolder) | struct |  |
| [`try_reduce`](#try_reduce) | fn |  |

## Structs

### `TryReduceConsumer<'r, R, ID>`

```rust
struct TryReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R, ID> Clone for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'r, R, ID, T> Consumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-folder"></span>`type Folder = TryReduceFolder<'r, R, T>`

- <span id="tryreduceconsumer-reducer"></span>`type Reducer = TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-result"></span>`type Result = T`

- <span id="tryreduceconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="tryreduceconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="tryreduceconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<'r, R, ID> Copy for TryReduceConsumer<'r, R, ID>`

##### `impl<T> IntoEither for TryReduceConsumer<'r, R, ID>`

##### `impl<T> Pointable for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-align"></span>`const ALIGN: usize`

- <span id="tryreduceconsumer-init"></span>`type Init = T`

- <span id="tryreduceconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreduceconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreduceconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreduceconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, R, ID, T> Reducer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl<'r, R, ID, T> UnindexedConsumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryreduceconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TryReduceFolder<'r, R, T: Try>`

```rust
struct TryReduceFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    control: std::ops::ControlFlow<<T as >::Residual, <T as >::Output>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R, T> Folder for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-result"></span>`type Result = T`

- <span id="tryreducefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryreducefolder-complete"></span>`fn complete(self) -> T`

- <span id="tryreducefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryReduceFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-align"></span>`const ALIGN: usize`

- <span id="tryreducefolder-init"></span>`type Init = T`

- <span id="tryreducefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducefolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `try_reduce`

```rust
fn try_reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(<T as >::Output, <T as >::Output) -> T + Sync,
    ID: Fn() -> <T as >::Output + Sync,
    T: Try + Send
```

