*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [into_iter](index.md)*

---

# Module `into_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of a vector. |

## Structs

### `IntoIter<T, A: Allocator>`

```rust
struct IntoIter<T, A: Allocator> {
    buf: core::ptr::NonNull<T>,
    phantom: core::marker::PhantomData<T>,
    cap: usize,
    alloc: core::mem::ManuallyDrop<A>,
    ptr: *const T,
    end: *const T,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/into_iter.rs:27-36`](../../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/into_iter.rs#L27-L36)*

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the `IntoIterator` trait).

# Example

```rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

#### Implementations

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  let _ = into_iter.next().unwrap();

  assert_eq!(into_iter.as_slice(), &['b', 'c']);

  ```

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Returns the remaining items of this iterator as a mutable slice.

  

  # Examples

  

  ```rust

  let vec = vec!['a', 'b', 'c'];

  let mut into_iter = vec.into_iter();

  assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);

  into_iter.as_mut_slice()[2] = 'z';

  assert_eq!(into_iter.next().unwrap(), 'a');

  assert_eq!(into_iter.next().unwrap(), 'b');

  assert_eq!(into_iter.next().unwrap(), 'z');

  ```

- <span id="intoiter-allocator"></span>`fn allocator(&self) -> &A`

  Returns a reference to the underlying allocator.

- <span id="intoiter-as-raw-mut-slice"></span>`fn as_raw_mut_slice(&mut self) -> *mut [T]`

#### Trait Implementations

##### `impl<T> Any for IntoIter<T, A>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- <span id="intoiter-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Borrow for IntoIter<T, A>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T, A>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for IntoIter<T, A>`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T> From for IntoIter<T, A>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl<T, U> Into for IntoIter<T, A>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IntoIter<T, A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-iterator-count"></span>`fn count(self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

##### `impl<T> ToOwned for IntoIter<T, A>`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IntoIter<T, A>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T, A>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

