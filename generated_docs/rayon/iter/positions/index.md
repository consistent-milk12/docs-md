*[rayon](../../index.md) / [iter](../index.md) / [positions](index.md)*

---

# Module `positions`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Positions`](#positions) | struct | `Positions` takes a predicate `predicate` and filters out elements that match |
| [`PositionsConsumer`](#positionsconsumer) | struct |  |
| [`PositionsFolder`](#positionsfolder) | struct |  |

## Structs

### `Positions<I, P>`

```rust
struct Positions<I, P> {
    base: I,
    predicate: P,
}
```

`Positions` takes a predicate `predicate` and filters out elements that match,
yielding their indices.

This struct is created by the `positions()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="positions-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Positions<I, P>`

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, P>` — [`Positions`](../index.md)

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- <span id="positions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Positions<I, P>`

##### `impl<T> IntoParallelIterator for Positions<I, P>`

- <span id="positions-iter"></span>`type Iter = T`

- <span id="positions-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="positions-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- <span id="positions-item"></span>`type Item = usize`

- <span id="positions-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Positions<I, P>`

- <span id="positions-align"></span>`const ALIGN: usize`

- <span id="positions-init"></span>`type Init = T`

- <span id="positions-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positions-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positions-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positions-drop"></span>`unsafe fn drop(ptr: usize)`

### `PositionsConsumer<'p, C, P>`

```rust
struct PositionsConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    offset: usize,
}
```

#### Implementations

- <span id="positionsconsumer-new"></span>`fn new(base: C, predicate: &'p P, offset: usize) -> Self`

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-folder"></span>`type Folder = PositionsFolder<'p, <C as Consumer>::Folder, P>`

- <span id="positionsconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="positionsconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="positionsconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="positionsconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="positionsconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for PositionsConsumer<'p, C, P>`

##### `impl<T> Pointable for PositionsConsumer<'p, C, P>`

- <span id="positionsconsumer-align"></span>`const ALIGN: usize`

- <span id="positionsconsumer-init"></span>`type Init = T`

- <span id="positionsconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positionsconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positionsconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positionsconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

### `PositionsFolder<'p, F, P>`

```rust
struct PositionsFolder<'p, F, P> {
    base: F,
    predicate: &'p P,
    offset: usize,
}
```

#### Trait Implementations

##### `impl<F, P, T> Folder for PositionsFolder<'_, F, P>`

- <span id="positionsfolder-result"></span>`type Result = <F as Folder>::Result`

- <span id="positionsfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="positionsfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="positionsfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for PositionsFolder<'p, F, P>`

##### `impl<T> Pointable for PositionsFolder<'p, F, P>`

- <span id="positionsfolder-align"></span>`const ALIGN: usize`

- <span id="positionsfolder-init"></span>`type Init = T`

- <span id="positionsfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="positionsfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="positionsfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="positionsfolder-drop"></span>`unsafe fn drop(ptr: usize)`

