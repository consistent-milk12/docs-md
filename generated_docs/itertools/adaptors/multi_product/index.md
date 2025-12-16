*[itertools](../../index.md) / [adaptors](../index.md) / [multi_product](index.md)*

---

# Module `multi_product`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiProduct`](#multiproduct) | struct | An iterator adaptor that iterates over the cartesian product of multiple iterators of type `I`. |
| [`MultiProductInner`](#multiproductinner) | struct | Internals for `MultiProduct`. |
| [`MultiProductIter`](#multiproductiter) | struct | Holds the state of a single iterator within a `MultiProduct`. |
| [`multi_cartesian_product`](#multi-cartesian-product) | fn | Create a new cartesian product iterator over an arbitrary number of iterators of the same type. |

## Structs

### `MultiProduct<I>`

```rust
struct MultiProduct<I>(Option<MultiProductInner<I>>)
where
    I: Iterator + Clone,
    <I as >::Item: Clone;
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:18-21`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/multi_product.rs#L18-L21)*

An iterator adaptor that iterates over the cartesian product of
multiple iterators of type `I`.

An iterator element type is `Vec<I::Item>`.

See [`.multi_cartesian_product()`](crate::Itertools::multi_cartesian_product)
for more information.

#### Trait Implementations

##### `impl Any for MultiProduct<I>`

- <span id="multiproduct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProduct<I>`

- <span id="multiproduct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProduct<I>`

- <span id="multiproduct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProduct<I>`

- <span id="multiproduct-clone"></span>`fn clone(&self) -> MultiProduct<I>` — [`MultiProduct`](../index.md#multiproduct)

##### `impl CloneToUninit for MultiProduct<I>`

- <span id="multiproduct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProduct<I>`

- <span id="multiproduct-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MultiProduct<I>`

- <span id="multiproduct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for MultiProduct<I>`

##### `impl<U> Into for MultiProduct<I>`

- <span id="multiproduct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProduct<I>`

##### `impl<I> IntoIterator for MultiProduct<I>`

- <span id="multiproduct-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multiproduct-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="multiproduct-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for MultiProduct<I>`

- <span id="multiproduct-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="multiproduct-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="multiproduct-iterator-count"></span>`fn count(self) -> usize`

- <span id="multiproduct-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="multiproduct-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

##### `impl Itertools for MultiProduct<I>`

##### `impl ToOwned for MultiProduct<I>`

- <span id="multiproduct-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproduct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproduct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProduct<I>`

- <span id="multiproduct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproduct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProduct<I>`

- <span id="multiproduct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproduct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MultiProductInner<I>`

```rust
struct MultiProductInner<I>
where
    I: Iterator + Clone,
    <I as >::Item: Clone {
    iters: alloc::vec::Vec<MultiProductIter<I>>,
    cur: Option<alloc::vec::Vec<<I as >::Item>>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:25-34`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/multi_product.rs#L25-L34)*

Internals for `MultiProduct`.

#### Fields

- **`iters`**: `alloc::vec::Vec<MultiProductIter<I>>`

  Holds the iterators.

- **`cur`**: `Option<alloc::vec::Vec<<I as >::Item>>`

  Not populated at the beginning then it holds the current item of each iterator.

#### Trait Implementations

##### `impl Any for MultiProductInner<I>`

- <span id="multiproductinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProductInner<I>`

- <span id="multiproductinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProductInner<I>`

- <span id="multiproductinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProductInner<I>`

- <span id="multiproductinner-clone"></span>`fn clone(&self) -> MultiProductInner<I>` — [`MultiProductInner`](#multiproductinner)

##### `impl CloneToUninit for MultiProductInner<I>`

- <span id="multiproductinner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProductInner<I>`

- <span id="multiproductinner-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MultiProductInner<I>`

- <span id="multiproductinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProductInner<I>`

- <span id="multiproductinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProductInner<I>`

##### `impl ToOwned for MultiProductInner<I>`

- <span id="multiproductinner-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproductinner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproductinner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProductInner<I>`

- <span id="multiproductinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproductinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProductInner<I>`

- <span id="multiproductinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproductinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MultiProductIter<I>`

```rust
struct MultiProductIter<I>
where
    I: Iterator + Clone,
    <I as >::Item: Clone {
    iter: I,
    iter_orig: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:74-81`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/multi_product.rs#L74-L81)*

Holds the state of a single iterator within a `MultiProduct`.

#### Implementations

- <span id="multiproductiter-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl Any for MultiProductIter<I>`

- <span id="multiproductiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProductIter<I>`

- <span id="multiproductiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProductIter<I>`

- <span id="multiproductiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProductIter<I>`

- <span id="multiproductiter-clone"></span>`fn clone(&self) -> MultiProductIter<I>` — [`MultiProductIter`](#multiproductiter)

##### `impl CloneToUninit for MultiProductIter<I>`

- <span id="multiproductiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProductIter<I>`

- <span id="multiproductiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MultiProductIter<I>`

- <span id="multiproductiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProductIter<I>`

- <span id="multiproductiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProductIter<I>`

##### `impl ToOwned for MultiProductIter<I>`

- <span id="multiproductiter-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproductiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproductiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProductIter<I>`

- <span id="multiproductiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproductiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProductIter<I>`

- <span id="multiproductiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproductiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `multi_cartesian_product`

```rust
fn multi_cartesian_product<H>(iters: H) -> MultiProduct<<<H as >::Item as IntoIterator>::IntoIter>
where
    H: Iterator,
    <H as >::Item: IntoIterator,
    <<H as >::Item as IntoIterator>::IntoIter: Clone,
    <<H as >::Item as IntoIterator>::Item: Clone
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:56-70`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/multi_product.rs#L56-L70)*

Create a new cartesian product iterator over an arbitrary number
of iterators of the same type.

Iterator element is of type `Vec<H::Item::Item>`.

