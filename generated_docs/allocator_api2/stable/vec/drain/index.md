*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [drain](index.md)*

---

# Module `drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | A draining iterator for `Vec<T>`. |

## Structs

### `Drain<'a, T: 'a, A: Allocator + 'a>`

```rust
struct Drain<'a, T: 'a, A: Allocator + 'a> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::Vec<T, A>>,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/drain.rs:22-30`](../../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/drain.rs#L22-L30)*

A draining iterator for `Vec<T>`.

This `struct` is created by `Vec::drain`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let iter: std::vec::Drain<_> = v.drain(..);
```

#### Fields

- **`tail_start`**: `usize`

  Index of tail to preserve

- **`tail_len`**: `usize`

  Length of tail

- **`iter`**: `slice::Iter<'a, T>`

  Current remaining range to remove

#### Implementations

- <span id="superdrain-fill"></span>`unsafe fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool`

  The range from `self.vec.len` to `self.tail_start` contains elements

  that have been moved out.

  Fill that range as much as possible with new elements from the `replace_with` iterator.

  Returns `true` if we filled the entire range. (`replace_with.next()` didn’t return `None`.)

- <span id="superdrain-move-tail"></span>`unsafe fn move_tail(&mut self, additional: usize)`

  Makes room for inserting more elements before the tail.

#### Trait Implementations

##### `impl<T> Any for Drain<'a, T, A>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T, A: Allocator> AsRef for Drain<'a, T, A>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Borrow for Drain<'a, T, A>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, T, A>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for Drain<'_, T, A>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

##### `impl<T> From for Drain<'a, T, A>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<T, U> Into for Drain<'a, T, A>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Drain<'a, T, A>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>`

##### `impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A>`

##### `impl<T, U> TryFrom for Drain<'a, T, A>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Drain<'a, T, A>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

