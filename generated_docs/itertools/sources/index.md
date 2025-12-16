*[itertools](../index.md) / [sources](index.md)*

---

# Module `sources`

Iterators that are sources (produce elements from parameters,
not from another iterator).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unfold`](#unfold) | struct | See [`unfold`](crate::unfold) for more information. |
| [`Iterate`](#iterate) | struct | An iterator that infinitely applies function to value and yields results. |
| [`unfold`](#unfold) | fn | Creates a new unfold source with the specified closure as the "iterator function" and an initial state to eventually pass to the closure |
| [`iterate`](#iterate) | fn | Creates a new iterator that infinitely applies function to value and yields results. |

## Structs

### `Unfold<St, F>`

```rust
struct Unfold<St, F> {
    f: F,
    pub state: St,
}
```

*Defined in [`itertools-0.14.0/src/sources.rs:72-76`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L72-L76)*

See [`unfold`](crate::unfold) for more information.

#### Fields

- **`state`**: `St`

  Internal state that will be passed to the closure on the next iteration

#### Trait Implementations

##### `impl Any for Unfold<St, F>`

- <span id="unfold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unfold<St, F>`

- <span id="unfold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unfold<St, F>`

- <span id="unfold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<St: clone::Clone, F: clone::Clone> Clone for Unfold<St, F>`

- <span id="unfold-clone"></span>`fn clone(&self) -> Unfold<St, F>` — [`Unfold`](#unfold)

##### `impl CloneToUninit for Unfold<St, F>`

- <span id="unfold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<St, F> Debug for Unfold<St, F>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Unfold<St, F>`

- <span id="unfold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unfold<St, F>`

- <span id="unfold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Unfold<St, F>`

##### `impl IntoIterator for Unfold<St, F>`

- <span id="unfold-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unfold-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unfold-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St, F> Iterator for Unfold<St, F>`

- <span id="unfold-iterator-type-item"></span>`type Item = A`

- <span id="unfold-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Unfold<St, F>`

##### `impl MultiUnzip for Unfold<St, F>`

- <span id="unfold-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Unfold<St, F>`

- <span id="unfold-toowned-type-owned"></span>`type Owned = T`

- <span id="unfold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unfold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unfold<St, F>`

- <span id="unfold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unfold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unfold<St, F>`

- <span id="unfold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unfold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iterate<St, F>`

```rust
struct Iterate<St, F> {
    state: St,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/sources.rs:96-99`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L96-L99)*

An iterator that infinitely applies function to value and yields results.

This `struct` is created by the [`iterate()`](crate::iterate) function.
See its documentation for more.

#### Trait Implementations

##### `impl Any for Iterate<St, F>`

- <span id="iterate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iterate<St, F>`

- <span id="iterate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iterate<St, F>`

- <span id="iterate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<St: clone::Clone, F: clone::Clone> Clone for Iterate<St, F>`

- <span id="iterate-clone"></span>`fn clone(&self) -> Iterate<St, F>` — [`Iterate`](#iterate)

##### `impl CloneToUninit for Iterate<St, F>`

- <span id="iterate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<St, F> Debug for Iterate<St, F>`

- <span id="iterate-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Iterate<St, F>`

- <span id="iterate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iterate<St, F>`

- <span id="iterate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Iterate<St, F>`

##### `impl IntoIterator for Iterate<St, F>`

- <span id="iterate-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterate-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterate-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St, F> Iterator for Iterate<St, F>`

- <span id="iterate-iterator-type-item"></span>`type Item = St`

- <span id="iterate-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterate-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Iterate<St, F>`

##### `impl MultiUnzip for Iterate<St, F>`

- <span id="iterate-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Iterate<St, F>`

- <span id="iterate-toowned-type-owned"></span>`type Owned = T`

- <span id="iterate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iterate<St, F>`

- <span id="iterate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iterate<St, F>`

- <span id="iterate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `unfold`

```rust
fn unfold<A, St, F>(initial_state: St, f: F) -> Unfold<St, F>
where
    F: FnMut(&mut St) -> Option<A>
```

*Defined in [`itertools-0.14.0/src/sources.rs:48-56`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L48-L56)*

Creates a new unfold source with the specified closure as the "iterator
function" and an initial state to eventually pass to the closure

`unfold` is a general iterator builder: it has a mutable state value,
and a closure with access to the state that produces the next value.

This more or less equivalent to a regular struct with an `Iterator`
implementation, and is useful for one-off iterators.

```rust
// an iterator that yields sequential Fibonacci numbers,
// and stops at the maximum representable value.

use itertools::unfold;

let mut fibonacci = unfold((1u32, 1u32), |(x1, x2)| {
    // Attempt to get the next Fibonacci number
    let next = x1.saturating_add(*x2);

    // Shift left: ret <- x1 <- x2 <- next
    let ret = *x1;
    *x1 = *x2;
    *x2 = next;

    // If addition has saturated at the maximum, we are finished
    if ret == *x1 && ret > 1 {
        None
    } else {
        Some(ret)
    }
});

itertools::assert_equal(fibonacci.by_ref().take(8),
                        vec![1, 1, 2, 3, 5, 8, 13, 21]);
assert_eq!(fibonacci.last(), Some(2_971_215_073))
```

### `iterate`

```rust
fn iterate<St, F>(initial_value: St, f: F) -> Iterate<St, F>
where
    F: FnMut(&St) -> St
```

*Defined in [`itertools-0.14.0/src/sources.rs:145-153`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L145-L153)*

Creates a new iterator that infinitely applies function to value and yields results.

```rust
use itertools::iterate;

itertools::assert_equal(iterate(1, |i| i % 3 + 1).take(5), vec![1, 2, 3, 1, 2]);
```

**Panics** if compute the next value does.

```should_panic
use itertools::iterate;
let mut it = iterate(25u32, |x| x - 10).take_while(|&x| x > 10);
assert_eq!(it.next(), Some(25)); // `Iterate` holds 15.
assert_eq!(it.next(), Some(15)); // `Iterate` holds 5.
it.next(); // `5 - 10` overflows.
```

You can alternatively use `core::iter::successors` as it better describes a finite iterator.

