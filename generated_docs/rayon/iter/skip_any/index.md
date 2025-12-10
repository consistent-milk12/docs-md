*[rayon](../../index.md) / [iter](../index.md) / [skip_any](index.md)*

---

# Module `skip_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SkipAny`](#skipany) | struct | `SkipAny` is an iterator that skips over `n` elements from anywhere in `I`. |
| [`SkipAnyConsumer`](#skipanyconsumer) | struct |  |
| [`SkipAnyFolder`](#skipanyfolder) | struct |  |
| [`checked_decrement`](#checked_decrement) | fn |  |

## Structs

### `SkipAny<I>`

```rust
struct SkipAny<I> {
    base: I,
    count: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:11-14`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any.rs#L11-L14)*

`SkipAny` is an iterator that skips over `n` elements from anywhere in `I`.
This struct is created by the `skip_any()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="skipany-new"></span>`fn new(base: I, count: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for SkipAny<I>`

- <span id="skipany-clone"></span>`fn clone(&self) -> SkipAny<I>` — [`SkipAny`](#skipany)

##### `impl<I: fmt::Debug> Debug for SkipAny<I>`

- <span id="skipany-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SkipAny<I>`

##### `impl<T> IntoParallelIterator for SkipAny<I>`

- <span id="skipany-type-iter"></span>`type Iter = T`

- <span id="skipany-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skipany-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for SkipAny<I>`

- <span id="skipany-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skipany-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl<T> Pointable for SkipAny<I>`

- <span id="skipany-const-align"></span>`const ALIGN: usize`

- <span id="skipany-type-init"></span>`type Init = T`

- <span id="skipany-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipany-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipany-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipany-drop"></span>`unsafe fn drop(ptr: usize)`

### `SkipAnyConsumer<'f, C>`

```rust
struct SkipAnyConsumer<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:44-47`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any.rs#L44-L47)*

#### Trait Implementations

##### `impl<'f, T, C> Consumer for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-type-folder"></span>`type Folder = SkipAnyFolder<'f, <C as Consumer>::Folder>`

- <span id="skipanyconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="skipanyconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="skipanyconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanyconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skipanyconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for SkipAnyConsumer<'f, C>`

##### `impl<T> Pointable for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-const-align"></span>`const ALIGN: usize`

- <span id="skipanyconsumer-type-init"></span>`type Init = T`

- <span id="skipanyconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanyconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanyconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanyconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C> UnindexedConsumer for SkipAnyConsumer<'f, C>`

- <span id="skipanyconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="skipanyconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `SkipAnyFolder<'f, C>`

```rust
struct SkipAnyFolder<'f, C> {
    base: C,
    count: &'f std::sync::atomic::AtomicUsize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:99-102`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any.rs#L99-L102)*

#### Trait Implementations

##### `impl<'f, T, C> Folder for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="skipanyfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="skipanyfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="skipanyfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="skipanyfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for SkipAnyFolder<'f, C>`

##### `impl<T> Pointable for SkipAnyFolder<'f, C>`

- <span id="skipanyfolder-const-align"></span>`const ALIGN: usize`

- <span id="skipanyfolder-type-init"></span>`type Init = T`

- <span id="skipanyfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skipanyfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skipanyfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skipanyfolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `checked_decrement`

```rust
fn checked_decrement(u: &std::sync::atomic::AtomicUsize) -> bool
```

*Defined in [`rayon-1.11.0/src/iter/skip_any.rs:104-107`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip_any.rs#L104-L107)*

