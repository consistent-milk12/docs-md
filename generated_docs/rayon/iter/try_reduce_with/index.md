*[rayon](../../index.md) / [iter](../index.md) / [try_reduce_with](index.md)*

---

# Module `try_reduce_with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReduceWithConsumer`](#tryreducewithconsumer) | struct |  |
| [`TryReduceWithFolder`](#tryreducewithfolder) | struct |  |
| [`try_reduce_with`](#try_reduce_with) | fn |  |

## Structs

### `TryReduceWithConsumer<'r, R>`

```rust
struct TryReduceWithConsumer<'r, R> {
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:22-25`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce_with.rs#L22-L25)*

#### Trait Implementations

##### `impl<'r, R> Clone for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl<'r, R, T> Consumer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-type-folder"></span>`type Folder = TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithconsumer-type-reducer"></span>`type Reducer = TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-type-result"></span>`type Result = Option<T>`

- <span id="tryreducewithconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="tryreducewithconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="tryreducewithconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<'r, R> Copy for TryReduceWithConsumer<'r, R>`

##### `impl<T> IntoEither for TryReduceWithConsumer<'r, R>`

##### `impl<T> Pointable for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-const-align"></span>`const ALIGN: usize`

- <span id="tryreducewithconsumer-type-init"></span>`type Init = T`

- <span id="tryreducewithconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducewithconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducewithconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducewithconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'r, R, T> Reducer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

##### `impl<'r, R, T> UnindexedConsumer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryreducewithconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TryReduceWithFolder<'r, R, T: Try>`

```rust
struct TryReduceWithFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    opt_control: Option<std::ops::ControlFlow<<T as >::Residual, <T as >::Output>>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:92-96`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce_with.rs#L92-L96)*

#### Trait Implementations

##### `impl<'r, R, T> Folder for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-type-result"></span>`type Result = Option<T>`

- <span id="tryreducewithfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryreducewithfolder-complete"></span>`fn complete(self) -> Option<T>`

- <span id="tryreducewithfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TryReduceWithFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-const-align"></span>`const ALIGN: usize`

- <span id="tryreducewithfolder-type-init"></span>`type Init = T`

- <span id="tryreducewithfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducewithfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducewithfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducewithfolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `try_reduce_with`

```rust
fn try_reduce_with<PI, R, T>(pi: PI, reduce_op: R) -> Option<T>
where
    PI: ParallelIterator<Item = T>,
    R: Fn(<T as >::Output, <T as >::Output) -> T + Sync,
    T: Try + Send
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:8-20`](../../../../.source_1765210505/rayon-1.11.0/src/iter/try_reduce_with.rs#L8-L20)*

