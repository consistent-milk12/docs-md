*[rayon](../index.md) / [split_producer](index.md)*

---

# Module `split_producer`

Common splitter for strings and slices

This module is private, so these items are effectively `pub(super)`

## Structs

### `SplitProducer<'p, P, V, const INCL: bool>`

```rust
struct SplitProducer<'p, P, V, const INCL: bool> {
    data: V,
    separator: &'p P,
    tail: usize,
}
```

Common producer for splitting on a predicate.

#### Fields

- **`tail`**: `usize`

  Marks the endpoint beyond which we've already found no separators.

#### Implementations

- `fn new(data: V, separator: &'p P) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for SplitProducer<'p, P, V, INCL>`

##### `impl<T> Pointable for SplitProducer<'p, P, V, INCL>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, P, V, const INCL: bool> UnindexedProducer for SplitProducer<'p, P, V, INCL>`

- `type Item = V`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

## Traits

### `Fissile<P>`

```rust
trait Fissile<P>: Sized { ... }
```

Helper trait so `&str`, `&[T]`, and `&mut [T]` can share `SplitProducer`.

#### Required Methods

- `fn length(self: &Self) -> usize`

- `fn midpoint(self: &Self, end: usize) -> usize`

- `fn find(self: &Self, separator: &P, start: usize, end: usize) -> Option<usize>`

- `fn rfind(self: &Self, separator: &P, end: usize) -> Option<usize>`

- `fn split_once<const INCL: bool>(self: Self, index: usize) -> (Self, Self)`

- `fn fold_splits<F, const INCL: bool>(self: Self, separator: &P, folder: F, skip_last: bool) -> F`

## Type Aliases

### `SplitInclusiveProducer<'p, P, V>`

```rust
type SplitInclusiveProducer<'p, P, V> = SplitProducer<'p, P, V, true>;
```

