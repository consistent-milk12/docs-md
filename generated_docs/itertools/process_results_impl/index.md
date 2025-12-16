*[itertools](../index.md) / [process_results_impl](index.md)*

---

# Module `process_results_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProcessResults`](#processresults) | struct | An iterator that produces only the `T` values as long as the inner iterator produces `Ok(T)`. |
| [`process_results`](#process-results) | fn | “Lift” a function of the values of an iterator so that it can process an iterator of `Result` values instead. |

## Structs

### `ProcessResults<'a, I, E: 'a>`

```rust
struct ProcessResults<'a, I, E: 'a> {
    error: &'a mut Result<(), E>,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/process_results_impl.rs:11-14`](../../../.source_1765900590/itertools-0.14.0/src/process_results_impl.rs#L11-L14)*

An iterator that produces only the `T` values as long as the
inner iterator produces `Ok(T)`.

Used by [`process_results`](crate::process_results), see its docs
for more information.

#### Implementations

- <span id="processresults-next-body"></span>`fn next_body<T>(&mut self, item: Option<Result<T, E>>) -> Option<T>`

#### Trait Implementations

##### `impl Any for ProcessResults<'a, I, E>`

- <span id="processresults-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProcessResults<'a, I, E>`

- <span id="processresults-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProcessResults<'a, I, E>`

- <span id="processresults-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug, E: fmt::Debug + 'a> Debug for ProcessResults<'a, I, E>`

- <span id="processresults-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, E> DoubleEndedIterator for ProcessResults<'_, I, E>`

- <span id="processresults-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="processresults-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> From for ProcessResults<'a, I, E>`

- <span id="processresults-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProcessResults<'a, I, E>`

- <span id="processresults-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ProcessResults<'a, I, E>`

##### `impl<I> IntoIterator for ProcessResults<'a, I, E>`

- <span id="processresults-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="processresults-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="processresults-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, E> Iterator for ProcessResults<'_, I, E>`

- <span id="processresults-iterator-type-item"></span>`type Item = T`

- <span id="processresults-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="processresults-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="processresults-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for ProcessResults<'a, I, E>`

##### `impl MultiUnzip for ProcessResults<'a, I, E>`

- <span id="processresults-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for ProcessResults<'a, I, E>`

- <span id="processresults-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="processresults-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProcessResults<'a, I, E>`

- <span id="processresults-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="processresults-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `process_results`

```rust
fn process_results<I, F, T, E, R>(iterable: I, processor: F) -> Result<R, E>
where
    I: IntoIterator<Item = Result<T, E>>,
    F: FnOnce(ProcessResults<'_, <I as >::IntoIter, E>) -> R
```

*Defined in [`itertools-0.14.0/src/process_results_impl.rs:94-108`](../../../.source_1765900590/itertools-0.14.0/src/process_results_impl.rs#L94-L108)*

“Lift” a function of the values of an iterator so that it can process
an iterator of `Result` values instead.

`IntoIterator` enabled version of `Itertools::process_results`.

