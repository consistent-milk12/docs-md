*[itertools](../index.md) / [unique_impl](index.md)*

---

# Module `unique_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`UniqueBy`](#uniqueby) | struct | An iterator adapter to filter out duplicate elements. |
| [`Unique`](#unique) | struct | An iterator adapter to filter out duplicate elements. |
| [`unique_by`](#unique-by) | fn | Create a new `UniqueBy` iterator. |
| [`count_new_keys`](#count-new-keys) | fn |  |
| [`unique`](#unique) | fn |  |

## Structs

### `UniqueBy<I: Iterator, V, F>`

```rust
struct UniqueBy<I: Iterator, V, F> {
    iter: I,
    used: std::collections::HashMap<V, ()>,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:12-19`](../../../.source_1765900590/itertools-0.14.0/src/unique_impl.rs#L12-L19)*

An iterator adapter to filter out duplicate elements.

See [`.unique_by()`](crate::Itertools::unique) for more information.

#### Trait Implementations

##### `impl Any for UniqueBy<I, V, F>`

- <span id="uniqueby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UniqueBy<I, V, F>`

- <span id="uniqueby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UniqueBy<I, V, F>`

- <span id="uniqueby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator, V: clone::Clone, F: clone::Clone> Clone for UniqueBy<I, V, F>`

- <span id="uniqueby-clone"></span>`fn clone(&self) -> UniqueBy<I, V, F>` — [`UniqueBy`](#uniqueby)

##### `impl CloneToUninit for UniqueBy<I, V, F>`

- <span id="uniqueby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, V, F> Debug for UniqueBy<I, V, F>`

- <span id="uniqueby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, V, F> DoubleEndedIterator for UniqueBy<I, V, F>`

- <span id="uniqueby-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for UniqueBy<I, V, F>`

- <span id="uniqueby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, V, F> FusedIterator for UniqueBy<I, V, F>`

##### `impl<U> Into for UniqueBy<I, V, F>`

- <span id="uniqueby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UniqueBy<I, V, F>`

##### `impl<I> IntoIterator for UniqueBy<I, V, F>`

- <span id="uniqueby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="uniqueby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="uniqueby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, V, F> Iterator for UniqueBy<I, V, F>`

- <span id="uniqueby-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="uniqueby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="uniqueby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="uniqueby-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for UniqueBy<I, V, F>`

##### `impl MultiUnzip for UniqueBy<I, V, F>`

- <span id="uniqueby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for UniqueBy<I, V, F>`

- <span id="uniqueby-toowned-type-owned"></span>`type Owned = T`

- <span id="uniqueby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="uniqueby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UniqueBy<I, V, F>`

- <span id="uniqueby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uniqueby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UniqueBy<I, V, F>`

- <span id="uniqueby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uniqueby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unique<I>`

```rust
struct Unique<I>
where
    I: Iterator,
    <I as >::Item: Eq + Hash + Clone {
    iter: UniqueBy<I, <I as >::Item, ()>,
}
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:160-166`](../../../.source_1765900590/itertools-0.14.0/src/unique_impl.rs#L160-L166)*

An iterator adapter to filter out duplicate elements.

See [`.unique()`](crate::Itertools::unique) for more information.

#### Trait Implementations

##### `impl Any for Unique<I>`

- <span id="unique-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unique<I>`

- <span id="unique-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unique<I>`

- <span id="unique-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Unique<I>`

- <span id="unique-clone"></span>`fn clone(&self) -> Unique<I>` — [`Unique`](#unique)

##### `impl CloneToUninit for Unique<I>`

- <span id="unique-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Unique<I>`

- <span id="unique-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I> DoubleEndedIterator for Unique<I>`

- <span id="unique-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for Unique<I>`

- <span id="unique-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Unique<I>`

##### `impl<U> Into for Unique<I>`

- <span id="unique-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Unique<I>`

##### `impl<I> IntoIterator for Unique<I>`

- <span id="unique-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unique-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unique-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Unique<I>`

- <span id="unique-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unique-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="unique-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="unique-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for Unique<I>`

##### `impl MultiUnzip for Unique<I>`

- <span id="unique-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Unique<I>`

- <span id="unique-toowned-type-owned"></span>`type Owned = T`

- <span id="unique-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unique-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unique<I>`

- <span id="unique-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unique-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unique<I>`

- <span id="unique-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unique-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `unique_by`

```rust
fn unique_by<I, V, F>(iter: I, f: F) -> UniqueBy<I, V, F>
where
    V: Eq + Hash,
    F: FnMut(&<I as >::Item) -> V,
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:30-41`](../../../.source_1765900590/itertools-0.14.0/src/unique_impl.rs#L30-L41)*

Create a new `UniqueBy` iterator.

### `count_new_keys`

```rust
fn count_new_keys<I, K>(used: std::collections::HashMap<K, ()>, iterable: I) -> usize
where
    I: IntoIterator<Item = K>,
    K: Hash + Eq
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:44-53`](../../../.source_1765900590/itertools-0.14.0/src/unique_impl.rs#L44-L53)*

### `unique`

```rust
fn unique<I>(iter: I) -> Unique<I>
where
    I: Iterator,
    <I as >::Item: Eq + Hash + Clone
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:176-188`](../../../.source_1765900590/itertools-0.14.0/src/unique_impl.rs#L176-L188)*

