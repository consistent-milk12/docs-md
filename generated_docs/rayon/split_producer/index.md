*[rayon](../index.md) / [split_producer](index.md)*

---

# Module `split_producer`

Common splitter for strings and slices

This module is private, so these items are effectively `pub(super)`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SplitProducer`](#splitproducer) | struct | Common producer for splitting on a predicate. |
| [`Fissile`](#fissile) | trait | Helper trait so `&str`, `&[T]`, and `&mut [T]` can share `SplitProducer`. |
| [`SplitInclusiveProducer`](#splitinclusiveproducer) | type |  |

## Structs

### `SplitProducer<'p, P, V, const INCL: bool>`

```rust
struct SplitProducer<'p, P, V, const INCL: bool> {
    data: V,
    separator: &'p P,
    tail: usize,
}
```

*Defined in [`rayon-1.11.0/src/split_producer.rs:8-14`](../../../.source_1765210505/rayon-1.11.0/src/split_producer.rs#L8-L14)*

Common producer for splitting on a predicate.

#### Fields

- **`tail`**: `usize`

  Marks the endpoint beyond which we've already found no separators.

#### Implementations

- <span id="splitproducer-new"></span>`fn new(data: V, separator: &'p P) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for SplitProducer<'p, P, V, INCL>`

##### `impl<T> Pointable for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-const-align"></span>`const ALIGN: usize`

- <span id="splitproducer-type-init"></span>`type Init = T`

- <span id="splitproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'p, P, V, const INCL: bool> UnindexedProducer for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-type-item"></span>`type Item = V`

- <span id="splitproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="splitproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `Fissile<P>`

```rust
trait Fissile<P>: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/split_producer.rs:19-29`](../../../.source_1765210505/rayon-1.11.0/src/split_producer.rs#L19-L29)*

Helper trait so `&str`, `&[T]`, and `&mut [T]` can share `SplitProducer`.

#### Required Methods

- `fn length(&self) -> usize`

- `fn midpoint(&self, end: usize) -> usize`

- `fn find(&self, separator: &P, start: usize, end: usize) -> Option<usize>`

- `fn rfind(&self, separator: &P, end: usize) -> Option<usize>`

- `fn split_once<const INCL: bool>(self, index: usize) -> (Self, Self)`

- `fn fold_splits<F, const INCL: bool>(self, separator: &P, folder: F, skip_last: bool) -> F`

#### Implementors

- `&[T]`
- `&mut [T]`
- `&str`

## Type Aliases

### `SplitInclusiveProducer<'p, P, V>`

```rust
type SplitInclusiveProducer<'p, P, V> = SplitProducer<'p, P, V, true>;
```

*Defined in [`rayon-1.11.0/src/split_producer.rs:16`](../../../.source_1765210505/rayon-1.11.0/src/split_producer.rs#L16)*

