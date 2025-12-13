*[clap_builder](../../index.md) / [builder](../index.md) / [range](index.md)*

---

# Module `range`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueRange`](#valuerange) | struct | Values per occurrence for an argument |

## Structs

### `ValueRange`

```rust
struct ValueRange {
    start_inclusive: usize,
    end_inclusive: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/range.rs:3-6`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/range.rs#L3-L6)*

Values per occurrence for an argument

#### Implementations

- <span id="valuerange-const-empty"></span>`const EMPTY: Self`

- <span id="valuerange-const-single"></span>`const SINGLE: Self`

- <span id="valuerange-const-optional"></span>`const OPTIONAL: Self`

- <span id="valuerange-const-full"></span>`const FULL: Self`

- <span id="valuerange-new"></span>`fn new(range: impl Into<Self>) -> Self`

  Create a range

  

  # Panics

  

  If the end is less than the start (debug builds)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(5);

  let range = ValueRange::new(5..10);

  let range = ValueRange::new(5..=10);

  let range = ValueRange::new(5..);

  let range = ValueRange::new(..10);

  let range = ValueRange::new(..=10);

  ```

  

  While this will panic:

  ```should_panic

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(10..5);  // Panics!

  ```

- <span id="valuerange-raw"></span>`fn raw(start_inclusive: usize, end_inclusive: usize) -> Self`

- <span id="valuerange-min-values"></span>`fn min_values(&self) -> usize`

  Fewest number of values the argument accepts

- <span id="valuerange-max-values"></span>`fn max_values(&self) -> usize`

  Most number of values the argument accepts

- <span id="valuerange-takes-values"></span>`fn takes_values(&self) -> bool`

  Report whether the argument takes any values (ie is a flag)

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::ValueRange;

  let range = ValueRange::new(5);

  assert!(range.takes_values());

  

  let range = ValueRange::new(0);

  assert!(!range.takes_values());

  ```

- <span id="valuerange-is-unbounded"></span>`fn is_unbounded(&self) -> bool`

- <span id="valuerange-is-fixed"></span>`fn is_fixed(&self) -> bool`

- <span id="valuerange-is-multiple"></span>`fn is_multiple(&self) -> bool`

- <span id="valuerange-num-values"></span>`fn num_values(&self) -> Option<usize>`

- <span id="valuerange-accepts-more"></span>`fn accepts_more(&self, current: usize) -> bool`

#### Trait Implementations

##### `impl Any for ValueRange`

- <span id="valuerange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueRange`

- <span id="valuerange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueRange`

- <span id="valuerange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueRange`

- <span id="valuerange-clone"></span>`fn clone(&self) -> ValueRange` — [`ValueRange`](#valuerange)

##### `impl CloneToUninit for ValueRange`

- <span id="valuerange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueRange`

##### `impl Debug for ValueRange`

- <span id="valuerange-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Default for ValueRange`

- <span id="valuerange-default"></span>`fn default() -> Self`

##### `impl Display for ValueRange`

- <span id="valuerange-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ValueRange`

##### `impl<T> From for ValueRange`

- <span id="valuerange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ValueRange`

- <span id="valuerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ValueRange`

- <span id="valuerange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for ValueRange`

- <span id="valuerange-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueRange>` — [`Resettable`](../resettable/index.md#resettable), [`ValueRange`](#valuerange)

##### `impl PartialEq for ValueRange`

- <span id="valuerange-partialeq-eq"></span>`fn eq(&self, other: &ValueRange) -> bool` — [`ValueRange`](#valuerange)

##### `impl RangeBounds for ValueRange`

- <span id="valuerange-rangebounds-start-bound"></span>`fn start_bound(&self) -> std::ops::Bound<&usize>`

- <span id="valuerange-rangebounds-end-bound"></span>`fn end_bound(&self) -> std::ops::Bound<&usize>`

##### `impl StructuralPartialEq for ValueRange`

##### `impl ToOwned for ValueRange`

- <span id="valuerange-toowned-type-owned"></span>`type Owned = T`

- <span id="valuerange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuerange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ValueRange`

- <span id="valuerange-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ValueRange`

- <span id="valuerange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuerange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueRange`

- <span id="valuerange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuerange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

