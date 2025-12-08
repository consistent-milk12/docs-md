*[rayon](../../index.md) / [iter](../index.md) / [product](index.md)*

---

# Module `product`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProductConsumer`](#productconsumer) | struct |  |
| [`ProductFolder`](#productfolder) | struct |  |
| [`product`](#product) | fn |  |
| [`mul`](#mul) | fn |  |

## Structs

### `ProductConsumer<P: Send>`

```rust
struct ProductConsumer<P: Send> {
    _marker: std::marker::PhantomData<*const P>,
}
```

#### Implementations

- <span id="productconsumer-new"></span>`fn new() -> ProductConsumer<P>` — [`ProductConsumer`](#productconsumer)

#### Trait Implementations

##### `impl<P, T> Consumer for ProductConsumer<P>`

- <span id="productconsumer-folder"></span>`type Folder = ProductFolder<P>`

- <span id="productconsumer-reducer"></span>`type Reducer = ProductConsumer<P>`

- <span id="productconsumer-result"></span>`type Result = P`

- <span id="productconsumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="productconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="productconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ProductConsumer<P>`

##### `impl<T> Pointable for ProductConsumer<P>`

- <span id="productconsumer-align"></span>`const ALIGN: usize`

- <span id="productconsumer-init"></span>`type Init = T`

- <span id="productconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="productconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="productconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="productconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Reducer for ProductConsumer<P>`

- <span id="productconsumer-reduce"></span>`fn reduce(self, left: P, right: P) -> P`

##### `impl<P: Send> Send for ProductConsumer<P>`

##### `impl<P, T> UnindexedConsumer for ProductConsumer<P>`

- <span id="productconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="productconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ProductFolder<P>`

```rust
struct ProductFolder<P> {
    product: P,
}
```

#### Trait Implementations

##### `impl<P, T> Folder for ProductFolder<P>`

- <span id="productfolder-result"></span>`type Result = P`

- <span id="productfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="productfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="productfolder-complete"></span>`fn complete(self) -> P`

- <span id="productfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ProductFolder<P>`

##### `impl<T> Pointable for ProductFolder<P>`

- <span id="productfolder-align"></span>`const ALIGN: usize`

- <span id="productfolder-init"></span>`type Init = T`

- <span id="productfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="productfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="productfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="productfolder-drop"></span>`unsafe fn drop(ptr: usize)`

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

