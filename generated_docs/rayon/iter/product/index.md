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

*Defined in [`rayon-1.11.0/src/iter/product.rs:19-21`](../../../../.source_1765521767/rayon-1.11.0/src/iter/product.rs#L19-L21)*

#### Implementations

- <span id="productconsumer-new"></span>`fn new() -> ProductConsumer<P>` — [`ProductConsumer`](#productconsumer)

#### Trait Implementations

##### `impl Any for ProductConsumer<P>`

- <span id="productconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProductConsumer<P>`

- <span id="productconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProductConsumer<P>`

- <span id="productconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P, T> Consumer for ProductConsumer<P>`

- <span id="productconsumer-consumer-type-folder"></span>`type Folder = ProductFolder<P>`

- <span id="productconsumer-consumer-type-reducer"></span>`type Reducer = ProductConsumer<P>`

- <span id="productconsumer-consumer-type-result"></span>`type Result = P`

- <span id="productconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="productconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="productconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ProductConsumer<P>`

- <span id="productconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProductConsumer<P>`

- <span id="productconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ProductConsumer<P>`

##### `impl Pointable for ProductConsumer<P>`

- <span id="productconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="productconsumer-pointable-type-init"></span>`type Init = T`

- <span id="productconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="productconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="productconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="productconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Reducer for ProductConsumer<P>`

- <span id="productconsumer-reducer-reduce"></span>`fn reduce(self, left: P, right: P) -> P`

##### `impl<P: Send> Send for ProductConsumer<P>`

##### `impl<U> TryFrom for ProductConsumer<P>`

- <span id="productconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="productconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProductConsumer<P>`

- <span id="productconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="productconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P, T> UnindexedConsumer for ProductConsumer<P>`

- <span id="productconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="productconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ProductFolder<P>`

```rust
struct ProductFolder<P> {
    product: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/product.rs:82-84`](../../../../.source_1765521767/rayon-1.11.0/src/iter/product.rs#L82-L84)*

#### Trait Implementations

##### `impl Any for ProductFolder<P>`

- <span id="productfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProductFolder<P>`

- <span id="productfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProductFolder<P>`

- <span id="productfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P, T> Folder for ProductFolder<P>`

- <span id="productfolder-folder-type-result"></span>`type Result = P`

- <span id="productfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="productfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="productfolder-folder-complete"></span>`fn complete(self) -> P`

- <span id="productfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ProductFolder<P>`

- <span id="productfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProductFolder<P>`

- <span id="productfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ProductFolder<P>`

##### `impl Pointable for ProductFolder<P>`

- <span id="productfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="productfolder-pointable-type-init"></span>`type Init = T`

- <span id="productfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="productfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="productfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="productfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ProductFolder<P>`

- <span id="productfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="productfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProductFolder<P>`

- <span id="productfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="productfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `product`

```rust
fn product<PI, P>(pi: PI) -> P
where
    PI: ParallelIterator,
    P: Send + Product<<PI as >::Item> + Product
```

*Defined in [`rayon-1.11.0/src/iter/product.rs:7-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/product.rs#L7-L13)*

### `mul`

```rust
fn mul<T: Product>(left: T, right: T) -> T
```

*Defined in [`rayon-1.11.0/src/iter/product.rs:15-17`](../../../../.source_1765521767/rayon-1.11.0/src/iter/product.rs#L15-L17)*

