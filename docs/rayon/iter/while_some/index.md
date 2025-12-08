*[rayon](../../index.md) / [iter](../index.md) / [while_some](index.md)*

---

# Module `while_some`

## Structs

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    base: I,
}
```

`WhileSome` is an iterator that yields the `Some` elements of an iterator,
halting as soon as any `None` is produced.

This struct is created by the `while_some()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for WhileSome<I>`

- `fn clone(self: &Self) -> WhileSome<I>` — [`WhileSome`](#whilesome)

##### `impl<I: $crate::fmt::Debug> Debug for WhileSome<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for WhileSome<I>`

##### `impl<T> IntoParallelIterator for WhileSome<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, T> ParallelIterator for WhileSome<I>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for WhileSome<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `WhileSomeConsumer<'f, C>`

```rust
struct WhileSomeConsumer<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'f, T, C> Consumer for WhileSomeConsumer<'f, C>`

- `type Folder = WhileSomeFolder<'f, <C as Consumer>::Folder>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for WhileSomeConsumer<'f, C>`

##### `impl<T> Pointable for WhileSomeConsumer<'f, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, C> UnindexedConsumer for WhileSomeConsumer<'f, C>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `WhileSomeFolder<'f, C>`

```rust
struct WhileSomeFolder<'f, C> {
    base: C,
    full: &'f std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'f, T, C> Folder for WhileSomeFolder<'f, C>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: Option<T>) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for WhileSomeFolder<'f, C>`

##### `impl<T> Pointable for WhileSomeFolder<'f, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

