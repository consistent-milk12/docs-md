*[rayon](../../index.md) / [iter](../index.md) / [take_any_while](index.md)*

---

# Module `take_any_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeAnyWhile`](#takeanywhile) | struct | `TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I` |
| [`TakeAnyWhileConsumer`](#takeanywhileconsumer) | struct |  |
| [`TakeAnyWhileFolder`](#takeanywhilefolder) | struct |  |
| [`take`](#take) | fn |  |

## Structs

### `TakeAnyWhile<I, P>`

```rust
struct TakeAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

`TakeAnyWhile` is an iterator that iterates over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `take_any_while()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="takeanywhile-new"></span>`fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for TakeAnyWhile<I, P>`

- <span id="takeanywhile-clone"></span>`fn clone(&self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](../index.md)

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- <span id="takeanywhile-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-iter"></span>`type Iter = T`

- <span id="takeanywhile-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="takeanywhile-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- <span id="takeanywhile-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="takeanywhile-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TakeAnyWhile<I, P>`

- <span id="takeanywhile-align"></span>`const ALIGN: usize`

- <span id="takeanywhile-init"></span>`type Init = T`

- <span id="takeanywhile-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhile-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhile-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhile-drop"></span>`unsafe fn drop(ptr: usize)`

### `TakeAnyWhileConsumer<'p, C, P>`

```rust
struct TakeAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-folder"></span>`type Folder = TakeAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- <span id="takeanywhileconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="takeanywhileconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="takeanywhileconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="takeanywhileconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="takeanywhileconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TakeAnyWhileConsumer<'p, C, P>`

##### `impl<T> Pointable for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-align"></span>`const ALIGN: usize`

- <span id="takeanywhileconsumer-init"></span>`type Init = T`

- <span id="takeanywhileconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhileconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhileconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhileconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'p, T, C, P> UnindexedConsumer for TakeAnyWhileConsumer<'p, C, P>`

- <span id="takeanywhileconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="takeanywhileconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TakeAnyWhileFolder<'p, C, P>`

```rust
struct TakeAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    taking: &'p std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'p, T, C, P> Folder for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="takeanywhilefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="takeanywhilefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="takeanywhilefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="takeanywhilefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for TakeAnyWhileFolder<'p, C, P>`

##### `impl<T> Pointable for TakeAnyWhileFolder<'p, C, P>`

- <span id="takeanywhilefolder-align"></span>`const ALIGN: usize`

- <span id="takeanywhilefolder-init"></span>`type Init = T`

- <span id="takeanywhilefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="takeanywhilefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="takeanywhilefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="takeanywhilefolder-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `take`

```rust
fn take<T>(item: &T, taking: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

