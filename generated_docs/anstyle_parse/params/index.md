*[anstyle_parse](../index.md) / [params](index.md)*

---

# Module `params`

Fixed size parameters list with optional subparameters.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Params`](#params) | struct |  |
| [`ParamsIter`](#paramsiter) | struct | Immutable subparameter iterator. |
| [`MAX_PARAMS`](#max-params) | const |  |

## Structs

### `Params`

```rust
struct Params {
    subparams: [u8; 32],
    params: [u16; 32],
    current_subparams: u8,
    len: usize,
}
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:8-25`](../../../.source_1765521767/anstyle-parse-0.2.7/src/params.rs#L8-L25)*

#### Fields

- **`subparams`**: `[u8; 32]`

  Number of subparameters for each parameter.
  
  For each entry in the `params` slice, this stores the length of the param as number of
  subparams at the same index as the param in the `params` slice.
  
  At the subparam positions the length will always be `0`.

- **`params`**: `[u16; 32]`

  All parameters and subparameters.

- **`current_subparams`**: `u8`

  Number of suparameters in the current parameter.

- **`len`**: `usize`

  Total number of parameters and subparameters.

#### Implementations

- <span id="params-len"></span>`fn len(&self) -> usize`

  Returns the number of parameters.

- <span id="params-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if there are no parameters present.

- <span id="params-iter"></span>`fn iter(&self) -> ParamsIter<'_>` — [`ParamsIter`](#paramsiter)

  Returns an iterator over all parameters and subparameters.

- <span id="params-is-full"></span>`fn is_full(&self) -> bool`

  Returns `true` if there is no more space for additional parameters.

- <span id="params-clear"></span>`fn clear(&mut self)`

  Clear all parameters.

- <span id="params-push"></span>`fn push(&mut self, item: u16)`

  Add an additional parameter.

- <span id="params-extend"></span>`fn extend(&mut self, item: u16)`

  Add an additional subparameter to the current parameter.

#### Trait Implementations

##### `impl Any for Params`

- <span id="params-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Params`

- <span id="params-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Params`

- <span id="params-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Params`

- <span id="params-clone"></span>`fn clone(&self) -> Params` — [`Params`](#params)

##### `impl CloneToUninit for Params`

- <span id="params-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Params`

- <span id="params-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default for Params`

- <span id="params-default"></span>`fn default() -> Params` — [`Params`](#params)

##### `impl Eq for Params`

##### `impl<T> From for Params`

- <span id="params-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Params`

- <span id="params-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for &'a Params`

- <span id="a-params-intoiterator-type-intoiter"></span>`type IntoIter = ParamsIter<'a>`

- <span id="a-params-intoiterator-type-item"></span>`type Item = &'a [u16]`

- <span id="a-params-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Params`

- <span id="params-partialeq-eq"></span>`fn eq(&self, other: &Params) -> bool` — [`Params`](#params)

##### `impl StructuralPartialEq for Params`

##### `impl ToOwned for Params`

- <span id="params-toowned-type-owned"></span>`type Owned = T`

- <span id="params-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="params-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Params`

- <span id="params-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="params-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Params`

- <span id="params-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="params-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
    params: &'a Params,
    index: usize,
}
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:88-91`](../../../.source_1765521767/anstyle-parse-0.2.7/src/params.rs#L88-L91)*

Immutable subparameter iterator.

#### Implementations

- <span id="paramsiter-new"></span>`fn new(params: &'a Params) -> Self` — [`Params`](#params)

#### Trait Implementations

##### `impl Any for ParamsIter<'a>`

- <span id="paramsiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParamsIter<'a>`

- <span id="paramsiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParamsIter<'a>`

- <span id="paramsiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ParamsIter<'a>`

- <span id="paramsiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParamsIter<'a>`

- <span id="paramsiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ParamsIter<'a>`

- <span id="paramsiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="paramsiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="paramsiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ParamsIter<'a>`

- <span id="paramsiter-iterator-type-item"></span>`type Item = &'a [u16]`

- <span id="paramsiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="paramsiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for ParamsIter<'a>`

- <span id="paramsiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="paramsiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParamsIter<'a>`

- <span id="paramsiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="paramsiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `MAX_PARAMS`
```rust
const MAX_PARAMS: usize = 32usize;
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:5`](../../../.source_1765521767/anstyle-parse-0.2.7/src/params.rs#L5)*

