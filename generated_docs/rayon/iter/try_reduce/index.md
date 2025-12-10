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

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:24-28`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce.rs#L24-L28)*

#### Trait Implementations

##### `impl<'r, R, ID> Clone for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'r, R, ID, T> Consumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-type-folder"></span>`type Folder = TryReduceFolder<'r, R, T>`

- <span id="tryreduceconsumer-type-reducer"></span>`type Reducer = TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-type-result"></span>`type Result = T`

- <span id="tryreduceconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="tryreduceconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryreduceconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<'r, R, ID> Copy for TryReduceConsumer<'r, R, ID>`

##### `impl<T> IntoEither for TryReduceConsumer<'r, R, ID>`

##### `impl<T> Pointable for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-const-align"></span>`const ALIGN: usize`

- <span id="tryreduceconsumer-type-init"></span>`type Init = T`

- <span id="tryreduceconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreduceconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreduceconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreduceconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, R, ID, T> Reducer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl<'r, R, ID, T> UnindexedConsumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryreduceconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TryReduceFolder<'r, R, T: Try>`

```rust
struct TryReduceFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    control: std::ops::ControlFlow<<T as >::Residual, <T as >::Output>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:93-97`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce.rs#L93-L97)*

#### Trait Implementations

##### `impl<'r, R, T> Folder for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-type-result"></span>`type Result = T`

- <span id="tryreducefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryreducefolder-complete"></span>`fn complete(self) -> T`

- <span id="tryreducefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryReduceFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-const-align"></span>`const ALIGN: usize`

- <span id="tryreducefolder-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:8-22`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce.rs#L8-L22)*

