*[rayon](../../index.md) / [iter](../index.md) / [multizip](index.md)*

---

# Module `multizip`

## Structs

### `MultiZip<T>`

```rust
struct MultiZip<T> {
    tuple: T,
}
```

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

##### `impl<T: $crate::clone::Clone> Clone for MultiZip<T>`

- `fn clone(self: &Self) -> MultiZip<T>` — [`MultiZip`](#multizip)

##### `impl<T: $crate::fmt::Debug> Debug for MultiZip<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B, C, D, E, F> IndexedParallelIterator for MultiZip<(A, B, C, D, E, F)>`

- `fn drive<CONSUMER>(self: Self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MultiZip<T>`

##### `impl<T> IntoParallelIterator for MultiZip<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B, C, D, E, F, G, H, I, J> ParallelIterator for MultiZip<(A, B, C, D, E, F, G, H, I, J)>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item, <C as ParallelIterator>::Item, <D as ParallelIterator>::Item, <E as ParallelIterator>::Item, <F as ParallelIterator>::Item, <G as ParallelIterator>::Item, <H as ParallelIterator>::Item, <I as ParallelIterator>::Item, <J as ParallelIterator>::Item)`

- `fn drive_unindexed<CONSUMER>(self: Self, consumer: CONSUMER) -> <CONSUMER as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MultiZip<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Macros

### `reduce!`

### `nest!`

### `flatten!`

### `multizip_impls!`

