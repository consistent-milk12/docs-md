*[rayon](../../index.md) / [iter](../index.md) / [interleave_shortest](index.md)*

---

# Module `interleave_shortest`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InterleaveShortest`](#interleaveshortest) | struct | `InterleaveShortest` is an iterator that works similarly to `Interleave`, but this version stops returning elements once one of the iterators run out. |

## Structs

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave_shortest.rs:14-16`](../../../../.source_1765210505/rayon-1.11.0/src/iter/interleave_shortest.rs#L14-L16)*

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](../index.md).


#### Implementations

- <span id="interleaveshortest-new"></span>`fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, J: clone::Clone> Clone for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](#interleaveshortest)

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for InterleaveShortest<I, J>`

- <span id="interleaveshortest-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="interleaveshortest-len"></span>`fn len(&self) -> usize`

- <span id="interleaveshortest-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl IntoEither for InterleaveShortest<I, J>`

##### `impl IntoParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="interleaveshortest-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleaveshortest-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleaveshortest-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="interleaveshortest-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for InterleaveShortest<I, J>`

- <span id="interleaveshortest-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleaveshortest-pointable-type-init"></span>`type Init = T`

- <span id="interleaveshortest-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveshortest-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveshortest-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveshortest-drop"></span>`unsafe fn drop(ptr: usize)`

