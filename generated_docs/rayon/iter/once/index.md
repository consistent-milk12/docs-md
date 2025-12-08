*[rayon](../../index.md) / [iter](../index.md) / [once](index.md)*

---

# Module `once`

## Structs

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Once<T>`

- `fn clone(self: &Self) -> Once<T>` — [`Once`](../index.md)

##### `impl<T: $crate::fmt::Debug> Debug for Once<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `once`

```rust
fn once<T: Send>(item: T) -> Once<T>
```

Creates a parallel iterator that produces an element exactly once.

This admits no parallelism on its own, but it could be chained to existing
parallel iterators to extend their contents, or otherwise used for any code
that deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::once;

let pi = (0..1234).into_par_iter()
    .chain(once(-1))
    .chain(1234..10_000);

assert_eq!(pi.clone().count(), 10_001);
assert_eq!(pi.clone().filter(|&x| x < 0).count(), 1);
assert_eq!(pi.position_any(|x| x < 0), Some(1234));
```

