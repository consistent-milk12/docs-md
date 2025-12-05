*[rayon](../index.md) / [array](index.md)*

---

# Module `array`

Parallel iterator types for [arrays](#arrays) (`[T; N]`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[arrays](#arrays): primitive@array

## Structs

### `IntoIter<T, const N: usize>`

```rust
struct IntoIter<T, const N: usize> {
    array: [T; N],
}
```

Parallel iterator that moves out of an array.

#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone, const N: usize>`

- `fn clone(self: &Self) -> IntoIter<T, N>` — [`IntoIter`](../../array/index.md)

##### `impl Debug<T: $crate::fmt::Debug, const N: usize>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send, const N: usize>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<T: Send, const N: usize>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

