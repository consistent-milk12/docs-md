*[rayon](../../index.md) / [iter](../index.md) / [blocks](index.md)*

---

# Module `blocks`

## Structs

### `BlocksCallback<S, C>`

```rust
struct BlocksCallback<S, C> {
    sizes: S,
    consumer: C,
    len: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for BlocksCallback<S, C>`

##### `impl<T> Pointable for BlocksCallback<S, C>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T, S, C> ProducerCallback for BlocksCallback<S, C>`

- `type Output = <C as Consumer>::Result`

- `fn callback<P: Producer<Item = T>>(self: Self, producer: P) -> <Self as >::Output` — [`ProducerCallback`](../plumbing/index.md)

### `ExponentialBlocks<I>`

```rust
struct ExponentialBlocks<I> {
    base: I,
}
```

`ExponentialBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of increasing sizes (exponentially).

This struct is created by the `by_exponential_blocks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for ExponentialBlocks<I>`

- `fn clone(self: &Self) -> ExponentialBlocks<I>` — [`ExponentialBlocks`](../index.md)

##### `impl<I: $crate::fmt::Debug> Debug for ExponentialBlocks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for ExponentialBlocks<I>`

##### `impl<T> IntoParallelIterator for ExponentialBlocks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for ExponentialBlocks<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for ExponentialBlocks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `UniformBlocks<I>`

```rust
struct UniformBlocks<I> {
    base: I,
    block_size: usize,
}
```

`UniformBlocks` is a parallel iterator that consumes itself as a sequence
of parallel blocks of constant sizes.

This struct is created by the `by_uniform_blocks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, block_size: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for UniformBlocks<I>`

- `fn clone(self: &Self) -> UniformBlocks<I>` — [`UniformBlocks`](../index.md)

##### `impl<I: $crate::fmt::Debug> Debug for UniformBlocks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for UniformBlocks<I>`

##### `impl<T> IntoParallelIterator for UniformBlocks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for UniformBlocks<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for UniformBlocks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `exponential_size`

```rust
fn exponential_size(size: &usize) -> Option<usize>
```

