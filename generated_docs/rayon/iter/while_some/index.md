*[rayon](../../index.md) / [iter](../index.md) / [while_some](index.md)*

---

# Module `while_some`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WhileSome`](#whilesome) | struct | `WhileSome` is an iterator that yields the `Some` elements of an iterator, halting as soon as any `None` is produced. |
| [`WhileSomeConsumer`](#whilesomeconsumer) | struct |  |
| [`WhileSomeFolder`](#whilesomefolder) | struct |  |

## Structs

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:13-15`](../../../../.source_1765210505/rayon-1.11.0/src/iter/while_some.rs#L13-L15)*

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="whilesome-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for WhileSome<I>`

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](#whilesome)

##### `impl<I: fmt::Debug> Debug for WhileSome<I>`

- <span id="whilesome-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for WhileSome<I>`

##### `impl IntoParallelIterator for WhileSome<I>`

- <span id="whilesome-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="whilesome-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="whilesome-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for WhileSome<I>`

- <span id="whilesome-paralleliterator-type-item"></span>`type Item = T`

- <span id="whilesome-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for WhileSome<I>`

- <span id="whilesome-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesome-pointable-type-init"></span>`type Init = T`

- <span id="whilesome-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesome-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesome-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesome-drop"></span>`unsafe fn drop(ptr: usize)`

### `WhileSomeConsumer<'f, C>`

```rust
struct WhileSomeConsumer<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:47-50`](../../../../.source_1765210505/rayon-1.11.0/src/iter/while_some.rs#L47-L50)*

#### Trait Implementations

##### `impl<T, C> Consumer for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-consumer-type-folder"></span>`type Folder = WhileSomeFolder<'f, <C as Consumer>::Folder>`

- <span id="whilesomeconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="whilesomeconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="whilesomeconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="whilesomeconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="whilesomeconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for WhileSomeConsumer<'f, C>`

##### `impl Pointable for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesomeconsumer-pointable-type-init"></span>`type Init = T`

- <span id="whilesomeconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesomeconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesomeconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesomeconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for WhileSomeConsumer<'f, C>`

- <span id="whilesomeconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="whilesomeconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `WhileSomeFolder<'f, C>`

```rust
struct WhileSomeFolder<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/while_some.rs:102-105`](../../../../.source_1765210505/rayon-1.11.0/src/iter/while_some.rs#L102-L105)*

#### Trait Implementations

##### `impl<T, C> Folder for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="whilesomefolder-consume"></span>`fn consume(self, item: Option<T>) -> Self`

- <span id="whilesomefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="whilesomefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="whilesomefolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for WhileSomeFolder<'f, C>`

##### `impl Pointable for WhileSomeFolder<'f, C>`

- <span id="whilesomefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="whilesomefolder-pointable-type-init"></span>`type Init = T`

- <span id="whilesomefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="whilesomefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="whilesomefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="whilesomefolder-drop"></span>`unsafe fn drop(ptr: usize)`

