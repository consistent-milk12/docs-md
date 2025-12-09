*[rayon](../../index.md) / [iter](../index.md) / [reduce](index.md)*

---

# Module `reduce`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReduceConsumer`](#reduceconsumer) | struct |  |
| [`ReduceFolder`](#reducefolder) | struct |  |
| [`reduce`](#reduce) | fn |  |

## Structs

### `ReduceConsumer<'r, R, ID>`

```rust
struct ReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
}
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:18-21`](../../../../.source_1765210505/rayon-1.11.0/src/iter/reduce.rs#L18-L21)*

#### Trait Implementations

##### `impl<'r, R, ID> Clone for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'r, R, ID, T> Consumer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-type-folder"></span>`type Folder = ReduceFolder<'r, R, T>`

- <span id="reduceconsumer-type-reducer"></span>`type Reducer = ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-type-result"></span>`type Result = T`

- <span id="reduceconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="reduceconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="reduceconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<'r, R, ID> Copy for ReduceConsumer<'r, R, ID>`

##### `impl<T> IntoEither for ReduceConsumer<'r, R, ID>`

##### `impl<T> Pointable for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-const-align"></span>`const ALIGN: usize`

- <span id="reduceconsumer-type-init"></span>`type Init = T`

- <span id="reduceconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="reduceconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="reduceconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="reduceconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, R, ID, T> Reducer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl<'r, R, ID, T> UnindexedConsumer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="reduceconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ReduceFolder<'r, R, T>`

```rust
struct ReduceFolder<'r, R, T> {
    reduce_op: &'r R,
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:81-84`](../../../../.source_1765210505/rayon-1.11.0/src/iter/reduce.rs#L81-L84)*

#### Trait Implementations

##### `impl<'r, R, T> Folder for ReduceFolder<'r, R, T>`

- <span id="reducefolder-type-result"></span>`type Result = T`

- <span id="reducefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="reducefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="reducefolder-complete"></span>`fn complete(self) -> T`

- <span id="reducefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ReduceFolder<'r, R, T>`

##### `impl<T> Pointable for ReduceFolder<'r, R, T>`

- <span id="reducefolder-const-align"></span>`const ALIGN: usize`

- <span id="reducefolder-type-init"></span>`type Init = T`

- <span id="reducefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="reducefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="reducefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="reducefolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `reduce`

```rust
fn reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(T, T) -> T + Sync,
    ID: Fn() -> T + Sync,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:4-16`](../../../../.source_1765210505/rayon-1.11.0/src/iter/reduce.rs#L4-L16)*

