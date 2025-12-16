*[itertools](../index.md) / [cons_tuples_impl](index.md)*

---

# Module `cons_tuples_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ConsTuplesFn`](#constuplesfn) | struct |  |
| [`cons_tuples`](#cons-tuples) | fn | Create an iterator that maps for example iterators of `((A, B), C)` to `(A, B, C)`. |
| [`ConsTuples`](#constuples) | type | An iterator that maps an iterator of tuples like `((A, B), C)` to an iterator of `(A, B, C)`. |
| [`impl_cons_iter!`](#impl-cons-iter) | macro |  |

## Structs

### `ConsTuplesFn`

```rust
struct ConsTuplesFn;
```

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:21`](../../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L21)*

#### Trait Implementations

##### `impl Any for ConsTuplesFn`

- <span id="constuplesfn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ConsTuplesFn`

- <span id="constuplesfn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ConsTuplesFn`

- <span id="constuplesfn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ConsTuplesFn`

- <span id="constuplesfn-clone"></span>`fn clone(&self) -> ConsTuplesFn` — [`ConsTuplesFn`](#constuplesfn)

##### `impl CloneToUninit for ConsTuplesFn`

- <span id="constuplesfn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ConsTuplesFn`

- <span id="constuplesfn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ConsTuplesFn`

- <span id="constuplesfn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ConsTuplesFn`

- <span id="constuplesfn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ConsTuplesFn`

##### `impl<K, L, X> MapSpecialCaseFn for ConsTuplesFn`

- <span id="constuplesfn-mapspecialcasefn-type-out"></span>`type Out = (K, L, X)`

- <span id="constuplesfn-mapspecialcasefn-call"></span>`fn call(&mut self, ((K, L), X): ((K, L), X)) -> <Self as >::Out` — [`MapSpecialCaseFn`](../adaptors/map/index.md#mapspecialcasefn)

##### `impl ToOwned for ConsTuplesFn`

- <span id="constuplesfn-toowned-type-owned"></span>`type Owned = T`

- <span id="constuplesfn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="constuplesfn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ConsTuplesFn`

- <span id="constuplesfn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="constuplesfn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ConsTuplesFn`

- <span id="constuplesfn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="constuplesfn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `cons_tuples`

```rust
fn cons_tuples<I>(iterable: I) -> ConsTuples<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:31-39`](../../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L31-L39)*

Create an iterator that maps for example iterators of
`((A, B), C)` to `(A, B, C)`.

## Type Aliases

### `ConsTuples<I>`

```rust
type ConsTuples<I> = crate::adaptors::map::MapSpecialCase<I, ConsTuplesFn>;
```

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:27`](../../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L27)*

An iterator that maps an iterator of tuples like
`((A, B), C)` to an iterator of `(A, B, C)`.

Used by the `iproduct!()` macro.

## Macros

### `impl_cons_iter!`

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:3-16`](../../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L3-L16)*

