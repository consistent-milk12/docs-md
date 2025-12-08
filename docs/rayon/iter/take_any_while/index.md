*[rayon](../../index.md) / [iter](../index.md) / [take_any_while](index.md)*

---

# Module `take_any_while`

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
This struct is created by the `take_any_while()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for TakeAnyWhile<I, P>`

- `fn clone(self: &Self) -> TakeAnyWhile<I, P>` — [`TakeAnyWhile`](#takeanywhile)

##### `impl<I: fmt::Debug, P> Debug for TakeAnyWhile<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for TakeAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for TakeAnyWhile<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for TakeAnyWhile<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for TakeAnyWhile<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `type Folder = TakeAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TakeAnyWhileConsumer<'p, C, P>`

##### `impl<T> Pointable for TakeAnyWhileConsumer<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, T, C, P> UnindexedConsumer for TakeAnyWhileConsumer<'p, C, P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

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

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TakeAnyWhileFolder<'p, C, P>`

##### `impl<T> Pointable for TakeAnyWhileFolder<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `take`

```rust
fn take<T>(item: &T, taking: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

