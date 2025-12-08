*[rayon](../../index.md) / [iter](../index.md) / [skip_any_while](index.md)*

---

# Module `skip_any_while`

## Structs

### `SkipAnyWhile<I, P>`

```rust
struct SkipAnyWhile<I, P> {
    base: I,
    predicate: P,
}
```

`SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`
until the callback returns `false`.
This struct is created by the `skip_any_while()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, predicate: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for SkipAnyWhile<I, P>`

- `fn clone(self: &Self) -> SkipAnyWhile<I, P>` — [`SkipAnyWhile`](#skipanywhile)

##### `impl<I: fmt::Debug, P> Debug for SkipAnyWhile<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SkipAnyWhile<I, P>`

##### `impl<T> IntoParallelIterator for SkipAnyWhile<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P> ParallelIterator for SkipAnyWhile<I, P>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for SkipAnyWhile<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SkipAnyWhileConsumer<'p, C, P>`

```rust
struct SkipAnyWhileConsumer<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for SkipAnyWhileConsumer<'p, C, P>`

- `type Folder = SkipAnyWhileFolder<'p, <C as Consumer>::Folder, P>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for SkipAnyWhileConsumer<'p, C, P>`

##### `impl<T> Pointable for SkipAnyWhileConsumer<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, T, C, P> UnindexedConsumer for SkipAnyWhileConsumer<'p, C, P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `SkipAnyWhileFolder<'p, C, P>`

```rust
struct SkipAnyWhileFolder<'p, C, P> {
    base: C,
    predicate: &'p P,
    skipping: &'p std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'p, T, C, P> Folder for SkipAnyWhileFolder<'p, C, P>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for SkipAnyWhileFolder<'p, C, P>`

##### `impl<T> Pointable for SkipAnyWhileFolder<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `skip`

```rust
fn skip<T>(item: &T, skipping: &std::sync::atomic::AtomicBool, predicate: &impl Fn(&T) -> bool) -> bool
```

