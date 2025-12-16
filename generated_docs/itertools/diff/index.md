*[itertools](../index.md) / [diff](index.md)*

---

# Module `diff`

"Diff"ing iterators for caching elements to sequential collections without requiring the new
elements' iterator to be `Clone`.

[`Diff`](#diff) (produced by the [`diff_with`](#diff-with) function)
describes the difference between two non-`Clone` iterators `I` and `J` after breaking ASAP from
a lock-step comparison.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Diff`](#diff) | enum | A type returned by the [`diff_with`] function. |
| [`diff_with`](#diff-with) | fn | Compares every element yielded by both `i` and `j` with the given function in lock-step and returns a [`Diff`] which describes how `j` differs from `i`. |

## Enums

### `Diff<I, J>`

```rust
enum Diff<I, J>
where
    I: Iterator,
    J: Iterator {
    FirstMismatch(usize, crate::structs::PutBack<I>, crate::structs::PutBack<J>),
    Shorter(usize, crate::structs::PutBack<I>),
    Longer(usize, crate::structs::PutBack<J>),
}
```

*Defined in [`itertools-0.14.0/src/diff.rs:17-29`](../../../.source_1765894658/itertools-0.14.0/src/diff.rs#L17-L29)*

A type returned by the [`diff_with`](#diff-with) function.

`Diff` represents the way in which the elements yielded by the iterator `I` differ to some
iterator `J`.

#### Variants

- **`FirstMismatch`**

  The index of the first non-matching element along with both iterator's remaining elements
  starting with the first mis-match.

- **`Shorter`**

  The total number of elements that were in `J` along with the remaining elements of `I`.

- **`Longer`**

  The total number of elements that were in `I` along with the remaining elements of `J`.

#### Trait Implementations

##### `impl Any for Diff<I, J>`

- <span id="diff-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Diff<I, J>`

- <span id="diff-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Diff<I, J>`

- <span id="diff-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J> Clone for Diff<I, J>`

- <span id="diff-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Diff<I, J>`

- <span id="diff-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, J> Debug for Diff<I, J>`

- <span id="diff-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Diff<I, J>`

- <span id="diff-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Diff<I, J>`

- <span id="diff-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Diff<I, J>`

##### `impl ToOwned for Diff<I, J>`

- <span id="diff-toowned-type-owned"></span>`type Owned = T`

- <span id="diff-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="diff-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Diff<I, J>`

- <span id="diff-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diff-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Diff<I, J>`

- <span id="diff-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diff-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `diff_with`

```rust
fn diff_with<I, J, F>(i: I, j: J, is_equal: F) -> Option<Diff<<I as >::IntoIter, <J as >::IntoIter>>
where
    I: IntoIterator,
    J: IntoIterator,
    F: FnMut(&<I as >::Item, &<J as >::Item) -> bool
```

*Defined in [`itertools-0.14.0/src/diff.rs:80-104`](../../../.source_1765894658/itertools-0.14.0/src/diff.rs#L80-L104)*

Compares every element yielded by both `i` and `j` with the given function in lock-step and
returns a [`Diff`](#diff) which describes how `j` differs from `i`.

If the number of elements yielded by `j` is less than the number of elements yielded by `i`,
the number of `j` elements yielded will be returned along with `i`'s remaining elements as
`Diff::Shorter`.

If the two elements of a step differ, the index of those elements along with the remaining
elements of both `i` and `j` are returned as `Diff::FirstMismatch`.

If `i` becomes exhausted before `j` becomes exhausted, the number of elements in `i` along with
the remaining `j` elements will be returned as `Diff::Longer`.

