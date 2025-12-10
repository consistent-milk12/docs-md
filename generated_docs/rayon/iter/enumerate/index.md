*[rayon](../../index.md) / [iter](../index.md) / [enumerate](index.md)*

---

# Module `enumerate`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Enumerate`](#enumerate) | struct | `Enumerate` is an iterator that returns the current count along with the element. |
| [`EnumerateProducer`](#enumerateproducer) | struct |  |

## Structs

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:12-14`](../../../../.source_1765210505/rayon-1.11.0/src/iter/enumerate.rs#L12-L14)*

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="enumerate-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Enumerate<I>`

- <span id="enumerate-clone"></span>`fn clone(&self) -> Enumerate<I>` — [`Enumerate`](#enumerate)

##### `impl<I: fmt::Debug> Debug for Enumerate<I>`

- <span id="enumerate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- <span id="enumerate-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="enumerate-len"></span>`fn len(&self) -> usize`

- <span id="enumerate-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Enumerate<I>`

##### `impl<T> IntoParallelIterator for Enumerate<I>`

- <span id="enumerate-type-iter"></span>`type Iter = T`

- <span id="enumerate-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="enumerate-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Enumerate<I>`

- <span id="enumerate-type-item"></span>`type Item = (usize, <I as ParallelIterator>::Item)`

- <span id="enumerate-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="enumerate-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Enumerate<I>`

- <span id="enumerate-const-align"></span>`const ALIGN: usize`

- <span id="enumerate-type-init"></span>`type Init = T`

- <span id="enumerate-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enumerate-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enumerate-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enumerate-drop"></span>`unsafe fn drop(ptr: usize)`

### `EnumerateProducer<P>`

```rust
struct EnumerateProducer<P> {
    base: P,
    offset: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/enumerate.rs:82-85`](../../../../.source_1765210505/rayon-1.11.0/src/iter/enumerate.rs#L82-L85)*

#### Trait Implementations

##### `impl<T> IntoEither for EnumerateProducer<P>`

##### `impl<T> Pointable for EnumerateProducer<P>`

- <span id="enumerateproducer-const-align"></span>`const ALIGN: usize`

- <span id="enumerateproducer-type-init"></span>`type Init = T`

- <span id="enumerateproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="enumerateproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="enumerateproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="enumerateproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for EnumerateProducer<P>`

- <span id="enumerateproducer-type-item"></span>`type Item = (usize, <P as Producer>::Item)`

- <span id="enumerateproducer-type-intoiter"></span>`type IntoIter = Zip<Range<usize>, <P as Producer>::IntoIter>`

- <span id="enumerateproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="enumerateproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="enumerateproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="enumerateproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

