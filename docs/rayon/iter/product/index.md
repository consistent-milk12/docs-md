*[rayon](../../index.md) / [iter](../index.md) / [product](index.md)*

---

# Module `product`

## Structs

### `ProductConsumer<P: Send>`

```rust
struct ProductConsumer<P: Send> {
    _marker: std::marker::PhantomData<*const P>,
}
```

#### Implementations

- `fn new() -> ProductConsumer<P>` — [`ProductConsumer`](#productconsumer)

#### Trait Implementations

##### `impl<P, T> Consumer for ProductConsumer<P>`

- `type Folder = ProductFolder<P>`

- `type Reducer = ProductConsumer<P>`

- `type Result = P`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, Self)`

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ProductConsumer<P>`

##### `impl<T> Pointable for ProductConsumer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Reducer for ProductConsumer<P>`

- `fn reduce(self: Self, left: P, right: P) -> P`

##### `impl<P: Send> Send for ProductConsumer<P>`

##### `impl<P, T> UnindexedConsumer for ProductConsumer<P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ProductFolder<P>`

```rust
struct ProductFolder<P> {
    product: P,
}
```

#### Trait Implementations

##### `impl<P, T> Folder for ProductFolder<P>`

- `type Result = P`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> P`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ProductFolder<P>`

##### `impl<T> Pointable for ProductFolder<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `product`

```rust
fn product<PI, P>(pi: PI) -> P
where
    PI: ParallelIterator,
    P: Send + Product<<PI as >::Item> + Product
```

### `mul`

```rust
fn mul<T: Product>(left: T, right: T) -> T
```

