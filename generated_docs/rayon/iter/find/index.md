*[rayon](../../index.md) / [iter](../index.md) / [find](index.md)*

---

# Module `find`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FindConsumer`](#findconsumer) | struct |  |
| [`FindFolder`](#findfolder) | struct |  |
| [`FindReducer`](#findreducer) | struct |  |
| [`find`](#find) | fn |  |

## Structs

### `FindConsumer<'p, P>`

```rust
struct FindConsumer<'p, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:15-18`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L15-L18)*

#### Implementations

- <span id="findconsumer-new"></span>`fn new(find_op: &'p P, found: &'p AtomicBool) -> Self`

#### Trait Implementations

##### `impl<T, P> Consumer for FindConsumer<'p, P>`

- <span id="findconsumer-consumer-type-folder"></span>`type Folder = FindFolder<'p, T, P>`

- <span id="findconsumer-consumer-type-reducer"></span>`type Reducer = FindReducer`

- <span id="findconsumer-consumer-type-result"></span>`type Result = Option<T>`

- <span id="findconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FindConsumer<'p, P>`

##### `impl Pointable for FindConsumer<'p, P>`

- <span id="findconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findconsumer-pointable-type-init"></span>`type Init = T`

- <span id="findconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, P> UnindexedConsumer for FindConsumer<'p, P>`

- <span id="findconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="findconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FindFolder<'p, T, P>`

```rust
struct FindFolder<'p, T, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
    item: Option<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:66-70`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L66-L70)*

#### Trait Implementations

##### `impl<T, P> Folder for FindFolder<'p, T, P>`

- <span id="findfolder-folder-type-result"></span>`type Result = Option<T>`

- <span id="findfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="findfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="findfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="findfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FindFolder<'p, T, P>`

##### `impl<T> Pointable for FindFolder<'p, T, P>`

- <span id="findfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findfolder-pointable-type-init"></span>`type Init = T`

- <span id="findfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `FindReducer`

```rust
struct FindReducer;
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:114`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L114)*

#### Trait Implementations

##### `impl IntoEither for FindReducer`

##### `impl Pointable for FindReducer`

- <span id="findreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findreducer-pointable-type-init"></span>`type Init = T`

- <span id="findreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for FindReducer`

- <span id="findreducer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

## Functions

### `find`

```rust
fn find<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:5-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L5-L13)*

