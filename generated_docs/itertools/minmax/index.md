*[itertools](../index.md) / [minmax](index.md)*

---

# Module `minmax`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MinMaxResult`](#minmaxresult) | enum | `MinMaxResult` is an enum returned by `minmax`. |
| [`minmax_impl`](#minmax-impl) | fn | Implementation guts for `minmax` and `minmax_by_key`. |

## Enums

### `MinMaxResult<T>`

```rust
enum MinMaxResult<T> {
    NoElements,
    OneElement(T),
    MinMax(T, T),
}
```

*Defined in [`itertools-0.14.0/src/minmax.rs:5-15`](../../../.source_1765900590/itertools-0.14.0/src/minmax.rs#L5-L15)*

`MinMaxResult` is an enum returned by `minmax`.

See [`.minmax()`](crate::Itertools::minmax) for more detail.

#### Variants

- **`NoElements`**

  Empty iterator

- **`OneElement`**

  Iterator with one element, so the minimum and maximum are the same

- **`MinMax`**

  More than one element in the iterator, the first element is not larger
  than the second

#### Implementations

- <span id="minmaxresult-into-option"></span>`fn into_option(self) -> Option<(T, T)>`

  `into_option` creates an `Option` of type `(T, T)`. The returned `Option`
  has variant `None` if and only if the `MinMaxResult` has variant
  `NoElements`. Otherwise `Some((x, y))` is returned where `x <= y`.
  If the `MinMaxResult` has variant `OneElement(x)`, performing this
  operation will make one clone of `x`.
  
  ##### Examples
  
  ```rust
  use itertools::MinMaxResult::{self, NoElements, OneElement, MinMax};
  
  let r: MinMaxResult<i32> = NoElements;
  assert_eq!(r.into_option(), None);
  
  let r = OneElement(1);
  assert_eq!(r.into_option(), Some((1, 1)));
  
  let r = MinMax(1, 2);
  assert_eq!(r.into_option(), Some((1, 2)));
  ```

#### Trait Implementations

##### `impl<T> Any for MinMaxResult<T>`

- <span id="minmaxresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinMaxResult<T>`

- <span id="minmaxresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinMaxResult<T>`

- <span id="minmaxresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for MinMaxResult<T>`

- <span id="minmaxresult-clone"></span>`fn clone(&self) -> MinMaxResult<T>` — [`MinMaxResult`](#minmaxresult)

##### `impl<T> CloneToUninit for MinMaxResult<T>`

- <span id="minmaxresult-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for MinMaxResult<T>`

##### `impl<T: fmt::Debug> Debug for MinMaxResult<T>`

- <span id="minmaxresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for MinMaxResult<T>`

##### `impl<T> From for MinMaxResult<T>`

- <span id="minmaxresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for MinMaxResult<T>`

- <span id="minmaxresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MinMaxResult<T>`

##### `impl<T: cmp::PartialEq> PartialEq for MinMaxResult<T>`

- <span id="minmaxresult-partialeq-eq"></span>`fn eq(&self, other: &MinMaxResult<T>) -> bool` — [`MinMaxResult`](#minmaxresult)

##### `impl<T> StructuralPartialEq for MinMaxResult<T>`

##### `impl<T> ToOwned for MinMaxResult<T>`

- <span id="minmaxresult-toowned-type-owned"></span>`type Owned = T`

- <span id="minmaxresult-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minmaxresult-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MinMaxResult<T>`

- <span id="minmaxresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minmaxresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MinMaxResult<T>`

- <span id="minmaxresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minmaxresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `minmax_impl`

```rust
fn minmax_impl<I, K, F, L>(it: I, key_for: F, lt: L) -> MinMaxResult<<I as >::Item>
where
    I: Iterator,
    F: FnMut(&<I as >::Item) -> K,
    L: FnMut(&<I as >::Item, &<I as >::Item, &K, &K) -> bool
```

*Defined in [`itertools-0.14.0/src/minmax.rs:48-116`](../../../.source_1765900590/itertools-0.14.0/src/minmax.rs#L48-L116)*

Implementation guts for `minmax` and `minmax_by_key`.

