*[rayon](../../index.md) / [iter](../index.md) / [find](index.md)*

---

# Module `find`

## Structs

### `FindConsumer<'p, P>`

```rust
struct FindConsumer<'p, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
}
```

#### Implementations

- `fn new(find_op: &'p P, found: &'p AtomicBool) -> Self`

#### Trait Implementations

##### `impl<'p, T, P> Consumer for FindConsumer<'p, P>`

- `type Folder = FindFolder<'p, T, P>`

- `type Reducer = FindReducer`

- `type Result = Option<T>`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FindConsumer<'p, P>`

##### `impl<T> Pointable for FindConsumer<'p, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, T, P> UnindexedConsumer for FindConsumer<'p, P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FindFolder<'p, T, P>`

```rust
struct FindFolder<'p, T, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
    item: Option<T>,
}
```

#### Trait Implementations

##### `impl<'p, T, P> Folder for FindFolder<'p, T, P>`

- `type Result = Option<T>`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FindFolder<'p, T, P>`

##### `impl<T> Pointable for FindFolder<'p, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FindReducer`

```rust
struct FindReducer;
```

#### Trait Implementations

##### `impl<T> IntoEither for FindReducer`

##### `impl<T> Pointable for FindReducer`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for FindReducer`

- `fn reduce(self: Self, left: Option<T>, right: Option<T>) -> Option<T>`

## Functions

### `find`

```rust
fn find<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

