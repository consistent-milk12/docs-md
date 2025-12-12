*[rayon](../../index.md) / [iter](../index.md) / [once](index.md)*

---

# Module `once`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Once`](#once) | struct | Iterator adaptor for [the `once()` function]. |
| [`once`](#once) | fn | Creates a parallel iterator that produces an element exactly once. |

## Structs

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:32-34`](../../../../.source_1765521767/rayon-1.11.0/src/iter/once.rs#L32-L34)*

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Once<T>`

- <span id="once-clone"></span>`fn clone(&self) -> Once<T>` — [`Once`](#once)

##### `impl<T: fmt::Debug> Debug for Once<T>`

- <span id="once-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- <span id="once-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="once-len"></span>`fn len(&self) -> usize`

- <span id="once-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- <span id="once-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="once-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="once-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- <span id="once-paralleliterator-type-item"></span>`type Item = T`

- <span id="once-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="once-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- <span id="once-pointable-const-align"></span>`const ALIGN: usize`

- <span id="once-pointable-type-init"></span>`type Init = T`

- <span id="once-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="once-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="once-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="once-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `once`

```rust
fn once<T: Send>(item: T) -> Once<T>
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:24-26`](../../../../.source_1765521767/rayon-1.11.0/src/iter/once.rs#L24-L26)*

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

