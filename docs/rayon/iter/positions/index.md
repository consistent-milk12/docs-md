*[rayon](../../index.md) / [iter](../index.md) / [positions](index.md)*

---

# Module `positions`

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

This struct is created by the `positions()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for Positions<I, P>`

- `fn clone(self: &Self) -> Positions<I, P>` — [`Positions`](#positions)

##### `impl<I: Debug, P> Debug for Positions<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Positions<I, P>`

##### `impl<T> IntoParallelIterator for Positions<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for Positions<I, P>`

- `type Item = usize`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Positions<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `PositionsConsumer<'p, C, P>`

```rust
struct PositionsConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    offset: usize,
}
```

#### Implementations

- `fn new(base: C, predicate: &'p P, offset: usize) -> Self`

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for PositionsConsumer<'p, C, P>`

- `type Folder = PositionsFolder<'p, <C as Consumer>::Folder, P>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for PositionsConsumer<'p, C, P>`

##### `impl<T> Pointable for PositionsConsumer<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `type Result = <F as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for PositionsFolder<'p, F, P>`

##### `impl<T> Pointable for PositionsFolder<'p, F, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

