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

*Defined in [`rayon-1.11.0/src/split_producer.rs:8-14`](../../../.source_1765521767/rayon-1.11.0/src/split_producer.rs#L8-L14)*

Common producer for splitting on a predicate.

#### Fields

- **`tail`**: `usize`

  Marks the endpoint beyond which we've already found no separators.

#### Implementations

- <span id="splitproducer-new"></span>`fn new(data: V, separator: &'p P) -> Self`

#### Trait Implementations

##### `impl Any for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SplitProducer<'p, P, V, INCL>`

##### `impl Pointable for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitproducer-pointable-type-init"></span>`type Init = T`

- <span id="splitproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, V> UnindexedProducer for SplitProducer<'p, P, V, INCL>`

- <span id="splitproducer-unindexedproducer-type-item"></span>`type Item = V`

- <span id="splitproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="splitproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `Fissile<P>`

```rust
trait Fissile<P>: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/split_producer.rs:19-29`](../../../.source_1765521767/rayon-1.11.0/src/split_producer.rs#L19-L29)*

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

*Defined in [`rayon-1.11.0/src/split_producer.rs:16`](../../../.source_1765521767/rayon-1.11.0/src/split_producer.rs#L16)*

