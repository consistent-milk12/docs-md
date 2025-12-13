*[proc_macro2](../index.md) / [rcvec](index.md)*

---

# Module `rcvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RcVec`](#rcvec) | struct |  |
| [`RcVecBuilder`](#rcvecbuilder) | struct |  |
| [`RcVecMut`](#rcvecmut) | struct |  |
| [`RcVecIntoIter`](#rcvecintoiter) | struct |  |

## Structs

### `RcVec<T>`

```rust
struct RcVec<T> {
    inner: alloc::rc::Rc<Vec<T>>,
}
```

*Defined in [`proc-macro2-1.0.103/src/rcvec.rs:7-9`](../../../.source_1765521767/proc-macro2-1.0.103/src/rcvec.rs#L7-L9)*

#### Implementations

- <span id="rcvec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="rcvec-len"></span>`fn len(&self) -> usize`

- <span id="rcvec-iter"></span>`fn iter(&self) -> slice::Iter<'_, T>`

- <span id="rcvec-make-mut"></span>`fn make_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvec-get-mut"></span>`fn get_mut(&mut self) -> Option<RcVecMut<'_, T>>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvec-make-owned"></span>`fn make_owned(self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

#### Trait Implementations

##### `impl<T> Any for RcVec<T>`

- <span id="rcvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcVec<T>`

- <span id="rcvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcVec<T>`

- <span id="rcvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for RcVec<T>`

- <span id="rcvec-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for RcVec<T>`

- <span id="rcvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for RcVec<T>`

- <span id="rcvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RcVec<T>`

- <span id="rcvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> RefUnwindSafe for RcVec<T>`

##### `impl<T> ToOwned for RcVec<T>`

- <span id="rcvec-toowned-type-owned"></span>`type Owned = T`

- <span id="rcvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rcvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RcVec<T>`

- <span id="rcvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rcvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RcVec<T>`

- <span id="rcvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rcvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RcVecBuilder<T>`

```rust
struct RcVecBuilder<T> {
    inner: Vec<T>,
}
```

*Defined in [`proc-macro2-1.0.103/src/rcvec.rs:11-13`](../../../.source_1765521767/proc-macro2-1.0.103/src/rcvec.rs#L11-L13)*

#### Implementations

- <span id="rcvecbuilder-new"></span>`fn new() -> Self`

- <span id="rcvecbuilder-with-capacity"></span>`fn with_capacity(cap: usize) -> Self`

- <span id="rcvecbuilder-push"></span>`fn push(&mut self, element: T)`

- <span id="rcvecbuilder-extend"></span>`fn extend(&mut self, iter: impl IntoIterator<Item = T>)`

- <span id="rcvecbuilder-as-mut"></span>`fn as_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvecbuilder-build"></span>`fn build(self) -> RcVec<T>` — [`RcVec`](#rcvec)

#### Trait Implementations

##### `impl<T> Any for RcVecBuilder<T>`

- <span id="rcvecbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcVecBuilder<T>`

- <span id="rcvecbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcVecBuilder<T>`

- <span id="rcvecbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RcVecBuilder<T>`

- <span id="rcvecbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RcVecBuilder<T>`

- <span id="rcvecbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoIterator for RcVecBuilder<T>`

- <span id="rcvecbuilder-intoiterator-type-item"></span>`type Item = T`

- <span id="rcvecbuilder-intoiterator-type-intoiter"></span>`type IntoIter = RcVecIntoIter<T>`

- <span id="rcvecbuilder-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, U> TryFrom for RcVecBuilder<T>`

- <span id="rcvecbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rcvecbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RcVecBuilder<T>`

- <span id="rcvecbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rcvecbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RcVecMut<'a, T>`

```rust
struct RcVecMut<'a, T> {
    inner: &'a mut Vec<T>,
}
```

*Defined in [`proc-macro2-1.0.103/src/rcvec.rs:15-17`](../../../.source_1765521767/proc-macro2-1.0.103/src/rcvec.rs#L15-L17)*

#### Implementations

- <span id="rcvecmut-push"></span>`fn push(&mut self, element: T)`

- <span id="rcvecmut-extend"></span>`fn extend(&mut self, iter: impl IntoIterator<Item = T>)`

- <span id="rcvecmut-as-mut"></span>`fn as_mut(&mut self) -> RcVecMut<'_, T>` — [`RcVecMut`](#rcvecmut)

- <span id="rcvecmut-take"></span>`fn take(self) -> RcVecBuilder<T>` — [`RcVecBuilder`](#rcvecbuilder)

#### Trait Implementations

##### `impl<T> Any for RcVecMut<'a, T>`

- <span id="rcvecmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcVecMut<'a, T>`

- <span id="rcvecmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcVecMut<'a, T>`

- <span id="rcvecmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RcVecMut<'a, T>`

- <span id="rcvecmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RcVecMut<'a, T>`

- <span id="rcvecmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T, U> TryFrom for RcVecMut<'a, T>`

- <span id="rcvecmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rcvecmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RcVecMut<'a, T>`

- <span id="rcvecmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rcvecmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RcVecIntoIter<T>`

```rust
struct RcVecIntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

*Defined in [`proc-macro2-1.0.103/src/rcvec.rs:20-22`](../../../.source_1765521767/proc-macro2-1.0.103/src/rcvec.rs#L20-L22)*

#### Trait Implementations

##### `impl<T> Any for RcVecIntoIter<T>`

- <span id="rcvecintoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcVecIntoIter<T>`

- <span id="rcvecintoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcVecIntoIter<T>`

- <span id="rcvecintoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for RcVecIntoIter<T>`

- <span id="rcvecintoiter-clone"></span>`fn clone(&self) -> RcVecIntoIter<T>` — [`RcVecIntoIter`](#rcvecintoiter)

##### `impl<T> CloneToUninit for RcVecIntoIter<T>`

- <span id="rcvecintoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for RcVecIntoIter<T>`

- <span id="rcvecintoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RcVecIntoIter<T>`

- <span id="rcvecintoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for RcVecIntoIter<T>`

- <span id="rcvecintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rcvecintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rcvecintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RcVecIntoIter<T>`

- <span id="rcvecintoiter-iterator-type-item"></span>`type Item = T`

- <span id="rcvecintoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rcvecintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> ToOwned for RcVecIntoIter<T>`

- <span id="rcvecintoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="rcvecintoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rcvecintoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RcVecIntoIter<T>`

- <span id="rcvecintoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rcvecintoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RcVecIntoIter<T>`

- <span id="rcvecintoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rcvecintoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

