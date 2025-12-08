*[rayon](../../index.md) / [iter](../index.md) / [filter](index.md)*

---

# Module `filter`

## Structs

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for Filter<I, P>`

- `fn clone(self: &Self) -> Filter<I, P>` — [`Filter`](../index.md)

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Filter<I, P>`

##### `impl<T> IntoParallelIterator for Filter<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Filter<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FilterConsumer<'p, C, P>`

```rust
struct FilterConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Implementations

- `fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for FilterConsumer<'p, C, P>`

- `type Folder = FilterFolder<'p, <C as Consumer>::Folder, P>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FilterConsumer<'p, C, P>`

##### `impl<T> Pointable for FilterConsumer<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, T, C, P> UnindexedConsumer for FilterConsumer<'p, C, P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FilterFolder<'p, C, P>`

```rust
struct FilterFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Trait Implementations

##### `impl<'p, C, P, T> Folder for FilterFolder<'p, C, P>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FilterFolder<'p, C, P>`

##### `impl<T> Pointable for FilterFolder<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

