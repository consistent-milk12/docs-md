*[rayon](../../index.md) / [iter](../index.md) / [interleave_shortest](index.md)*

---

# Module `interleave_shortest`

## Structs

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J> {
    interleave: Interleave<Take<I>, Take<J>>,
}
```

`InterleaveShortest` is an iterator that works similarly to
`Interleave`, but this version stops returning elements once one
of the iterators run out.

This struct is created by the `interleave_shortest()` method on
[`IndexedParallelIterator`](../index.md).


#### Implementations

- `fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, J: $crate::clone::Clone> Clone for InterleaveShortest<I, J>`

- `fn clone(self: &Self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](#interleaveshortest)

##### `impl<I: $crate::fmt::Debug, J: $crate::fmt::Debug> Debug for InterleaveShortest<I, J>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I, J> IndexedParallelIterator for InterleaveShortest<I, J>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for InterleaveShortest<I, J>`

##### `impl<T> IntoParallelIterator for InterleaveShortest<I, J>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, J> ParallelIterator for InterleaveShortest<I, J>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for InterleaveShortest<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

