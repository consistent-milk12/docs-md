*[rayon](../../index.md) / [iter](../index.md) / [take_any](index.md)*

---

# Module `take_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeAny`](#takeany) | struct | `TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`. |
| [`TakeAnyConsumer`](#takeanyconsumer) | struct |  |
| [`TakeAnyFolder`](#takeanyfolder) | struct |  |
| [`checked_decrement`](#checked_decrement) | fn |  |

## Structs

### `TakeAny<I>`

```rust
struct TakeAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:11-14`](../../../../.source_1765210505/rayon-1.11.0/src/iter/take_any.rs#L11-L14)*

`TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`.
This struct is created by the `take_any()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="takeany-new"></span>`fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for TakeAny<I>`

- <span id="takeany-clone"></span>`fn clone(&self) -> TakeAny<I>` — [`TakeAny`](#takeany)

##### `impl<I: fmt::Debug> Debug for TakeAny<I>`

- <span id="takeany-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAny<I>`

##### `impl<T> IntoParallelIterator for TakeAny<I>`

- <span id="takeany-type-iter"></span>`type Iter = T`

- <span id="takeany-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeany-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for TakeAny<I>`

- <span id="takeany-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeany-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TakeAny<I>`

- <span id="takeany-const-align"></span>`const ALIGN: usize`

- <span id="takeany-type-init"></span>`type Init = T`

- <span id="takeany-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeany-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeany-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeany-drop"></span>`unsafe fn drop(ptr: usize)`

### `TakeAnyConsumer<'f, C>`

```rust
struct TakeAnyConsumer<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:44-47`](../../../../.source_1765210505/rayon-1.11.0/src/iter/take_any.rs#L44-L47)*

#### Trait Implementations

##### `impl<'f, T, C> Consumer for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-type-folder"></span>`type Folder = TakeAnyFolder<'f, <C as Consumer>::Folder>`

- <span id="takeanyconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="takeanyconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="takeanyconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="takeanyconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="takeanyconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TakeAnyConsumer<'f, C>`

##### `impl<T> Pointable for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-const-align"></span>`const ALIGN: usize`

- <span id="takeanyconsumer-type-init"></span>`type Init = T`

- <span id="takeanyconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanyconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanyconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanyconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C> UnindexedConsumer for TakeAnyConsumer<'f, C>`

- <span id="takeanyconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="takeanyconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TakeAnyFolder<'f, C>`

```rust
struct TakeAnyFolder<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:99-102`](../../../../.source_1765210505/rayon-1.11.0/src/iter/take_any.rs#L99-L102)*

#### Trait Implementations

##### `impl<'f, T, C> Folder for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="takeanyfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="takeanyfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="takeanyfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="takeanyfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TakeAnyFolder<'f, C>`

##### `impl<T> Pointable for TakeAnyFolder<'f, C>`

- <span id="takeanyfolder-const-align"></span>`const ALIGN: usize`

- <span id="takeanyfolder-type-init"></span>`type Init = T`

- <span id="takeanyfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanyfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanyfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanyfolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `checked_decrement`

```rust
fn checked_decrement(u: &std::sync::atomic::AtomicUsize) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/take_any.rs:104-107`](../../../../.source_1765210505/rayon-1.11.0/src/iter/take_any.rs#L104-L107)*

