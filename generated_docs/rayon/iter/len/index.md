*[rayon](../../index.md) / [iter](../index.md) / [len](index.md)*

---

# Module `len`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MinLen`](#minlen) | struct | `MinLen` is an iterator that imposes a minimum length on iterator splits. |
| [`MinLenProducer`](#minlenproducer) | struct |  |
| [`MaxLen`](#maxlen) | struct | `MaxLen` is an iterator that imposes a maximum length on iterator splits. |
| [`MaxLenProducer`](#maxlenproducer) | struct |  |

## Structs

### `MinLen<I>`

```rust
struct MinLen<I> {
    base: I,
    min: usize,
}
```

`MinLen` is an iterator that imposes a minimum length on iterator splits.
This struct is created by the `with_min_len()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="minlen-new"></span>`fn new(base: I, min: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for MinLen<I>`

- <span id="minlen-clone"></span>`fn clone(&self) -> MinLen<I>` — [`MinLen`](../index.md)

##### `impl<I: fmt::Debug> Debug for MinLen<I>`

- <span id="minlen-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for MinLen<I>`

- <span id="minlen-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="minlen-len"></span>`fn len(&self) -> usize`

- <span id="minlen-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MinLen<I>`

##### `impl<T> IntoParallelIterator for MinLen<I>`

- <span id="minlen-iter"></span>`type Iter = T`

- <span id="minlen-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="minlen-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MinLen<I>`

- <span id="minlen-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="minlen-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="minlen-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MinLen<I>`

- <span id="minlen-align"></span>`const ALIGN: usize`

- <span id="minlen-init"></span>`type Init = T`

- <span id="minlen-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlen-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlen-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlen-drop"></span>`unsafe fn drop(ptr: usize)`

### `MinLenProducer<P>`

```rust
struct MinLenProducer<P> {
    base: P,
    min: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MinLenProducer<P>`

##### `impl<T> Pointable for MinLenProducer<P>`

- <span id="minlenproducer-align"></span>`const ALIGN: usize`

- <span id="minlenproducer-init"></span>`type Init = T`

- <span id="minlenproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="minlenproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="minlenproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="minlenproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MinLenProducer<P>`

- <span id="minlenproducer-item"></span>`type Item = <P as Producer>::Item`

- <span id="minlenproducer-intoiter"></span>`type IntoIter = <P as Producer>::IntoIter`

- <span id="minlenproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="minlenproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="minlenproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="minlenproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="minlenproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `MaxLen<I>`

```rust
struct MaxLen<I> {
    base: I,
    max: usize,
}
```

`MaxLen` is an iterator that imposes a maximum length on iterator splits.
This struct is created by the `with_max_len()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="maxlen-new"></span>`fn new(base: I, max: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for MaxLen<I>`

- <span id="maxlen-clone"></span>`fn clone(&self) -> MaxLen<I>` — [`MaxLen`](../index.md)

##### `impl<I: fmt::Debug> Debug for MaxLen<I>`

- <span id="maxlen-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for MaxLen<I>`

- <span id="maxlen-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="maxlen-len"></span>`fn len(&self) -> usize`

- <span id="maxlen-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MaxLen<I>`

##### `impl<T> IntoParallelIterator for MaxLen<I>`

- <span id="maxlen-iter"></span>`type Iter = T`

- <span id="maxlen-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="maxlen-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for MaxLen<I>`

- <span id="maxlen-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="maxlen-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="maxlen-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MaxLen<I>`

- <span id="maxlen-align"></span>`const ALIGN: usize`

- <span id="maxlen-init"></span>`type Init = T`

- <span id="maxlen-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlen-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlen-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlen-drop"></span>`unsafe fn drop(ptr: usize)`

### `MaxLenProducer<P>`

```rust
struct MaxLenProducer<P> {
    base: P,
    max: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MaxLenProducer<P>`

##### `impl<T> Pointable for MaxLenProducer<P>`

- <span id="maxlenproducer-align"></span>`const ALIGN: usize`

- <span id="maxlenproducer-init"></span>`type Init = T`

- <span id="maxlenproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="maxlenproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="maxlenproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="maxlenproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for MaxLenProducer<P>`

- <span id="maxlenproducer-item"></span>`type Item = <P as Producer>::Item`

- <span id="maxlenproducer-intoiter"></span>`type IntoIter = <P as Producer>::IntoIter`

- <span id="maxlenproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="maxlenproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="maxlenproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="maxlenproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="maxlenproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

