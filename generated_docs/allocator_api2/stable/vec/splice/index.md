*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [splice](index.md)*

---

# Module `splice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Splice`](#splice) | struct | A splicing iterator for `Vec`. |

## Structs

### `Splice<'a, I: Iterator + 'a, A: Allocator + 'a>`

```rust
struct Splice<'a, I: Iterator + 'a, A: Allocator + 'a> {
    drain: super::Drain<'a, <I as >::Item, A>,
    replace_with: I,
}
```

*Defined in [`allocator-api2-0.2.21/src/stable/vec/splice.rs:21-24`](../../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/vec/splice.rs#L21-L24)*

A splicing iterator for `Vec`.

This struct is created by `Vec::splice()`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let new = [7, 8];
let iter: std::vec::Splice<_> = v.splice(1.., new);
```

#### Trait Implementations

##### `impl Any for Splice<'a, I, A>`

- <span id="splice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Splice<'a, I, A>`

- <span id="splice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Splice<'a, I, A>`

- <span id="splice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug + Iterator + 'a, A: fmt::Debug + Allocator + 'a> Debug for Splice<'a, I, A>`

- <span id="splice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'_, I, A>`

- <span id="splice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I: Iterator, A: Allocator> Drop for Splice<'_, I, A>`

- <span id="splice-drop"></span>`fn drop(&mut self)`

##### `impl<I: Iterator, A: Allocator> ExactSizeIterator for Splice<'_, I, A>`

##### `impl<T> From for Splice<'a, I, A>`

- <span id="splice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Splice<'a, I, A>`

- <span id="splice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for Splice<'a, I, A>`

- <span id="splice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator, A: Allocator> Iterator for Splice<'_, I, A>`

- <span id="splice-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splice-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="splice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for Splice<'a, I, A>`

- <span id="splice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Splice<'a, I, A>`

- <span id="splice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

