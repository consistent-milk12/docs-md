*[rayon](../../index.md) / [iter](../index.md) / [zip](index.md)*

---

# Module `zip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zip`](#zip) | struct | `Zip` is an iterator that zips up `a` and `b` into a single iterator of pairs. |
| [`ZipProducer`](#zipproducer) | struct |  |

## Structs

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:12-15`](../../../../.source_1765210505/rayon-1.11.0/src/iter/zip.rs#L12-L15)*

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="zip-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Zip<A, B>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<A, B>` — [`Zip`](#zip)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Zip<A, B>`

- <span id="zip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- <span id="zip-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zip-len"></span>`fn len(&self) -> usize`

- <span id="zip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl IntoEither for Zip<A, B>`

##### `impl IntoParallelIterator for Zip<A, B>`

- <span id="zip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="zip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- <span id="zip-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zip-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Zip<A, B>`

- <span id="zip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zip-pointable-type-init"></span>`type Init = T`

- <span id="zip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zip-drop"></span>`unsafe fn drop(ptr: usize)`

### `ZipProducer<A: Producer, B: Producer>`

```rust
struct ZipProducer<A: Producer, B: Producer> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:118-121`](../../../../.source_1765210505/rayon-1.11.0/src/iter/zip.rs#L118-L121)*

#### Trait Implementations

##### `impl IntoEither for ZipProducer<A, B>`

##### `impl Pointable for ZipProducer<A, B>`

- <span id="zipproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zipproducer-pointable-type-init"></span>`type Init = T`

- <span id="zipproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A: Producer, B: Producer> Producer for ZipProducer<A, B>`

- <span id="zipproducer-producer-type-item"></span>`type Item = (<A as Producer>::Item, <B as Producer>::Item)`

- <span id="zipproducer-producer-type-intoiter"></span>`type IntoIter = Zip<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- <span id="zipproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="zipproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="zipproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="zipproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

