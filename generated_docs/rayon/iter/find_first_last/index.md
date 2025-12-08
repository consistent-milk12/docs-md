*[rayon](../../index.md) / [iter](../index.md) / [find_first_last](index.md)*

---

# Module `find_first_last`

## Structs

### `FindConsumer<'p, P>`

```rust
struct FindConsumer<'p, P> {
    find_op: &'p P,
    lower_bound: std::cell::Cell<usize>,
    upper_bound: usize,
    match_position: MatchPosition,
    best_found: &'p std::sync::atomic::AtomicUsize,
}
```

#### Implementations

- `fn new(find_op: &'p P, match_position: MatchPosition, best_found: &'p AtomicUsize) -> Self` — [`MatchPosition`](#matchposition)

- `fn current_index(self: &Self) -> usize`

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
    boundary: usize,
    match_position: MatchPosition,
    best_found: &'p std::sync::atomic::AtomicUsize,
    item: Option<T>,
}
```

#### Trait Implementations

##### `impl<'p, P: 'p + Fn(&T) -> bool, T> Folder for FindFolder<'p, T, P>`

- `type Result = Option<T>`

- `fn consume(self: Self, item: T) -> Self`

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
struct FindReducer {
    match_position: MatchPosition,
}
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

## Enums

### `MatchPosition`

```rust
enum MatchPosition {
    Leftmost,
    Rightmost,
}
```

#### Trait Implementations

##### `impl Clone for MatchPosition`

- `fn clone(self: &Self) -> MatchPosition` — [`MatchPosition`](#matchposition)

##### `impl Copy for MatchPosition`

##### `impl<T> IntoEither for MatchPosition`

##### `impl<T> Pointable for MatchPosition`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `better_position`

```rust
fn better_position(pos1: usize, pos2: usize, mp: MatchPosition) -> bool
```

Returns true if pos1 is a better match than pos2 according to MatchPosition

### `find_first`

```rust
fn find_first<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

### `find_last`

```rust
fn find_last<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

