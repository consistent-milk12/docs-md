*[itertools](../index.md) / [permutations](index.md)*

---

# Module `permutations`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Permutations`](#permutations) | struct | An iterator adaptor that iterates through all the `k`-permutations of the elements from an iterator. |
| [`PermutationState`](#permutationstate) | enum |  |
| [`permutations`](#permutations) | fn |  |
| [`advance`](#advance) | fn |  |

## Structs

### `Permutations<I: Iterator>`

```rust
struct Permutations<I: Iterator> {
    vals: super::lazy_buffer::LazyBuffer<I>,
    state: PermutationState,
}
```

*Defined in [`itertools-0.14.0/src/permutations.rs:16-19`](../../../.source_1765900590/itertools-0.14.0/src/permutations.rs#L16-L19)*

An iterator adaptor that iterates through all the `k`-permutations of the
elements from an iterator.

See [`.permutations()`](crate::Itertools::permutations) for
more information.

#### Trait Implementations

##### `impl Any for Permutations<I>`

- <span id="permutations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Permutations<I>`

- <span id="permutations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Permutations<I>`

- <span id="permutations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Permutations<I>`

- <span id="permutations-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Permutations<I>`

- <span id="permutations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Permutations<I>`

- <span id="permutations-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Permutations<I>`

- <span id="permutations-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Permutations<I>`

##### `impl<U> Into for Permutations<I>`

- <span id="permutations-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Permutations<I>`

##### `impl<I> IntoIterator for Permutations<I>`

- <span id="permutations-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="permutations-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="permutations-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Permutations<I>`

- <span id="permutations-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="permutations-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="permutations-iterator-count"></span>`fn count(self) -> usize`

- <span id="permutations-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Permutations<I>`

##### `impl ToOwned for Permutations<I>`

- <span id="permutations-toowned-type-owned"></span>`type Owned = T`

- <span id="permutations-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="permutations-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Permutations<I>`

- <span id="permutations-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="permutations-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Permutations<I>`

- <span id="permutations-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="permutations-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `PermutationState`

```rust
enum PermutationState {
    Start {
        k: usize,
    },
    Buffered {
        k: usize,
        min_n: usize,
    },
    Loaded {
        indices: alloc::boxed::Box<[usize]>,
        cycles: alloc::boxed::Box<[usize]>,
    },
    End,
}
```

*Defined in [`itertools-0.14.0/src/permutations.rs:30-42`](../../../.source_1765900590/itertools-0.14.0/src/permutations.rs#L30-L42)*

#### Variants

- **`Start`**

  No permutation generated yet.

- **`Buffered`**

  Values from the iterator are not fully loaded yet so `n` is still unknown.

- **`Loaded`**

  All values from the iterator are known so `n` is known.

- **`End`**

  No permutation left to generate.

#### Implementations

- <span id="permutationstate-size-hint-for"></span>`fn size_hint_for(&self, n: usize) -> (usize, Option<usize>)`

#### Trait Implementations

##### `impl Any for PermutationState`

- <span id="permutationstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PermutationState`

- <span id="permutationstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PermutationState`

- <span id="permutationstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PermutationState`

- <span id="permutationstate-clone"></span>`fn clone(&self) -> PermutationState` — [`PermutationState`](#permutationstate)

##### `impl CloneToUninit for PermutationState`

- <span id="permutationstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PermutationState`

- <span id="permutationstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PermutationState`

- <span id="permutationstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PermutationState`

- <span id="permutationstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PermutationState`

##### `impl ToOwned for PermutationState`

- <span id="permutationstate-toowned-type-owned"></span>`type Owned = T`

- <span id="permutationstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="permutationstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PermutationState`

- <span id="permutationstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="permutationstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PermutationState`

- <span id="permutationstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="permutationstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `permutations`

```rust
fn permutations<I: Iterator>(iter: I, k: usize) -> Permutations<I>
```

*Defined in [`itertools-0.14.0/src/permutations.rs:52-57`](../../../.source_1765900590/itertools-0.14.0/src/permutations.rs#L52-L57)*

### `advance`

```rust
fn advance(indices: &mut [usize], cycles: &mut [usize]) -> bool
```

*Defined in [`itertools-0.14.0/src/permutations.rs:140-156`](../../../.source_1765900590/itertools-0.14.0/src/permutations.rs#L140-L156)*

