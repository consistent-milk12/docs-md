*[rayon](../../index.md) / [iter](../index.md) / [multizip](index.md)*

---

# Module `multizip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiZip`](#multizip) | struct | `MultiZip` is an iterator that zips up a tuple of parallel iterators to produce tuples of their items. |
| [`reduce!`](#reduce) | macro |  |
| [`nest!`](#nest) | macro |  |
| [`flatten!`](#flatten) | macro |  |
| [`multizip_impls!`](#multizip-impls) | macro |  |

## Structs

### `MultiZip<T>`

```rust
struct MultiZip<T> {
    tuple: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:79-81`](../../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L79-L81)*

`MultiZip` is an iterator that zips up a tuple of parallel iterators to
produce tuples of their items.

It is created by calling `into_par_iter()` on a tuple of types that
implement `IntoParallelIterator`, or `par_iter()`/`par_iter_mut()` with
types that are iterable by reference.

The implementation currently support tuples up to length 12.

# Examples

```rust
use rayon::prelude::*;

// This will iterate `r` by mutable reference, like `par_iter_mut()`, while
// ranges are all iterated by value like `into_par_iter()`.
// Note that the zipped iterator is only as long as the shortest input.
let mut r = vec![0; 3];
(&mut r, 1..10, 10..100, 100..1000).into_par_iter()
    .for_each(|(r, x, y, z)| *r = x * y + z);

assert_eq!(&r, &[1 * 10 + 100, 2 * 11 + 101, 3 * 12 + 102]);
```

For a group that should all be iterated by reference, you can use a tuple reference.

```rust
use rayon::prelude::*;

let xs: Vec<_> = (1..10).collect();
let ys: Vec<_> = (10..100).collect();
let zs: Vec<_> = (100..1000).collect();

// Reference each input separately with `IntoParallelIterator`:
let r1: Vec<_> = (&xs, &ys, &zs).into_par_iter()
    .map(|(x, y, z)| x * y + z)
    .collect();

// Reference them all together with `IntoParallelRefIterator`:
let r2: Vec<_> = (xs, ys, zs).par_iter()
    .map(|(x, y, z)| x * y + z)
    .collect();

assert_eq!(r1, r2);
```

Mutable references to a tuple will work similarly.

```rust
use rayon::prelude::*;

let mut xs: Vec<_> = (1..4).collect();
let mut ys: Vec<_> = (-4..-1).collect();
let mut zs = vec![0; 3];

// Mutably reference each input separately with `IntoParallelIterator`:
(&mut xs, &mut ys, &mut zs).into_par_iter().for_each(|(x, y, z)| {
    *z += *x + *y;
    std::mem::swap(x, y);
});

assert_eq!(xs, (vec![-4, -3, -2]));
assert_eq!(ys, (vec![1, 2, 3]));
assert_eq!(zs, (vec![-3, -1, 1]));

// Mutably reference them all together with `IntoParallelRefMutIterator`:
let mut tuple = (xs, ys, zs);
tuple.par_iter_mut().for_each(|(x, y, z)| {
    *z += *x + *y;
    std::mem::swap(x, y);
});

assert_eq!(tuple, (vec![1, 2, 3], vec![-4, -3, -2], vec![-6, -2, 2]));
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for MultiZip<T>`

- <span id="multizip-clone"></span>`fn clone(&self) -> MultiZip<T>` — [`MultiZip`](#multizip)

##### `impl<T: fmt::Debug> Debug for MultiZip<T>`

- <span id="multizip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> IndexedParallelIterator for MultiZip<(A)>`

- <span id="multizip-drive"></span>`fn drive<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="multizip-len"></span>`fn len(&self) -> usize`

- <span id="multizip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for MultiZip<T>`

##### `impl<T> IntoParallelIterator for MultiZip<T>`

- <span id="multizip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="multizip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="multizip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A> ParallelIterator for MultiZip<(A)>`

- <span id="multizip-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item)`

- <span id="multizip-drive-unindexed"></span>`fn drive_unindexed<CONSUMER>(self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="multizip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MultiZip<T>`

- <span id="multizip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multizip-pointable-type-init"></span>`type Init = T`

- <span id="multizip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multizip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multizip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multizip-drop"></span>`unsafe fn drop(ptr: usize)`

## Macros

### `reduce!`

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:93-106`](../../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L93-L106)*

### `nest!`

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:108-117`](../../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L108-L117)*

### `flatten!`

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:119-127`](../../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L119-L127)*

### `multizip_impls!`

*Defined in [`rayon-1.11.0/src/iter/multizip.rs:129-230`](../../../../.source_1765521767/rayon-1.11.0/src/iter/multizip.rs#L129-L230)*

