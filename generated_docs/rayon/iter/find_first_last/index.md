*[rayon](../../index.md) / [iter](../index.md) / [find_first_last](index.md)*

---

# Module `find_first_last`

## Contents

- [Structs](#structs)
  - [`FindConsumer`](#findconsumer)
  - [`FindFolder`](#findfolder)
  - [`FindReducer`](#findreducer)
- [Enums](#enums)
  - [`MatchPosition`](#matchposition)
- [Functions](#functions)
  - [`better_position`](#better_position)
  - [`find_first`](#find_first)
  - [`find_last`](#find_last)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FindConsumer`](#findconsumer) | struct |  |
| [`FindFolder`](#findfolder) | struct |  |
| [`FindReducer`](#findreducer) | struct |  |
| [`MatchPosition`](#matchposition) | enum |  |
| [`better_position`](#better_position) | fn | Returns true if pos1 is a better match than pos2 according to MatchPosition |
| [`find_first`](#find_first) | fn |  |
| [`find_last`](#find_last) | fn |  |

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

- <span id="findconsumer-new"></span>`fn new(find_op: &'p P, match_position: MatchPosition, best_found: &'p AtomicUsize) -> Self` — [`MatchPosition`](#matchposition)

- <span id="findconsumer-current-index"></span>`fn current_index(&self) -> usize`

#### Trait Implementations

##### `impl<'p, T, P> Consumer for FindConsumer<'p, P>`

- <span id="findconsumer-folder"></span>`type Folder = FindFolder<'p, T, P>`

- <span id="findconsumer-reducer"></span>`type Reducer = FindReducer`

- <span id="findconsumer-result"></span>`type Result = Option<T>`

- <span id="findconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="findconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="findconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FindConsumer<'p, P>`

##### `impl<T> Pointable for FindConsumer<'p, P>`

- <span id="findconsumer-align"></span>`const ALIGN: usize`

- <span id="findconsumer-init"></span>`type Init = T`

- <span id="findconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'p, T, P> UnindexedConsumer for FindConsumer<'p, P>`

- <span id="findconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="findconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

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

- <span id="findfolder-result"></span>`type Result = Option<T>`

- <span id="findfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="findfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="findfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FindFolder<'p, T, P>`

##### `impl<T> Pointable for FindFolder<'p, T, P>`

- <span id="findfolder-align"></span>`const ALIGN: usize`

- <span id="findfolder-init"></span>`type Init = T`

- <span id="findfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `FindReducer`

```rust
struct FindReducer {
    match_position: MatchPosition,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for FindReducer`

##### `impl<T> Pointable for FindReducer`

- <span id="findreducer-align"></span>`const ALIGN: usize`

- <span id="findreducer-init"></span>`type Init = T`

- <span id="findreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for FindReducer`

- <span id="findreducer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

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

- <span id="matchposition-clone"></span>`fn clone(&self) -> MatchPosition` — [`MatchPosition`](#matchposition)

##### `impl Copy for MatchPosition`

##### `impl<T> IntoEither for MatchPosition`

##### `impl<T> Pointable for MatchPosition`

- <span id="matchposition-align"></span>`const ALIGN: usize`

- <span id="matchposition-init"></span>`type Init = T`

- <span id="matchposition-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="matchposition-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="matchposition-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="matchposition-drop"></span>`unsafe fn drop(ptr: usize)`

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

