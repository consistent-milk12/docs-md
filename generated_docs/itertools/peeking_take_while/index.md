*[itertools](../index.md) / [peeking_take_while](index.md)*

---

# Module `peeking_take_while`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeekingTakeWhile`](#peekingtakewhile) | struct | An iterator adaptor that takes items while a closure returns `true`. |
| [`PeekingNext`](#peekingnext) | trait | An iterator that allows peeking at an element before deciding to accept it. |
| [`peeking_take_while`](#peeking-take-while) | fn | Create a `PeekingTakeWhile` |
| [`peeking_next_by_clone!`](#peeking-next-by-clone) | macro |  |

## Structs

### `PeekingTakeWhile<'a, I, F>`

```rust
struct PeekingTakeWhile<'a, I, F>
where
    I: Iterator + 'a {
    iter: &'a mut I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:113-119`](../../../.source_1765900590/itertools-0.14.0/src/peeking_take_while.rs#L113-L119)*

An iterator adaptor that takes items while a closure returns `true`.

See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)
for more information.

#### Trait Implementations

##### `impl Any for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Debug for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PeekingTakeWhile<'a, I, F>`

##### `impl<I> IntoIterator for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peekingtakewhile-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="peekingtakewhile-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for PeekingTakeWhile<'_, I, F>`

- <span id="peekingtakewhile-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peekingtakewhile-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="peekingtakewhile-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for PeekingTakeWhile<'a, I, F>`

##### `impl MultiUnzip for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I, F> PeekingNext for PeekingTakeWhile<'_, I, F>`

- <span id="peekingtakewhile-peekingnext-peeking-next"></span>`fn peeking_next<G>(&mut self, g: G) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="peekingtakewhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="peekingtakewhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `PeekingNext`

```rust
trait PeekingNext: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:15-23`](../../../.source_1765900590/itertools-0.14.0/src/peeking_take_while.rs#L15-L23)*

An iterator that allows peeking at an element before deciding to accept it.

See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)
for more information.

This is implemented by peeking adaptors like peekable and put back,
but also by a few iterators that can be peeked natively, like the slice’s
by reference iterator (`std::slice::Iter`).

#### Required Methods

- `fn PeekingNext::peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

  Pass a reference to the next iterator element to the closure `accept`;
  if `accept` returns `true`, return it as the next element,
  else `None`.

#### Implementors

- [`MultiPeek`](../multipeek_impl/index.md#multipeek)
- [`PeekNth`](../peek_nth/index.md#peeknth)
- [`PeekingTakeWhile`](#peekingtakewhile)
- [`PutBackN`](../put_back_n_impl/index.md#putbackn)
- [`PutBack`](../adaptors/index.md#putback)
- [`RepeatN`](../repeatn/index.md#repeatn)
- `&mut I`
- `::std::iter::Empty<T>`
- `::std::iter::Rev<I>`
- `::std::option::Iter<'a, T>`
- `::std::result::Iter<'a, T>`
- `::std::slice::Iter<'a, T>`
- `::std::str::Bytes<'a>`
- `::std::str::CharIndices<'a>`
- `::std::str::Chars<'a>`
- `alloc::collections::linked_list::Iter<'a, T>`
- `alloc::collections::vec_deque::Iter<'a, T>`
- `std::iter::Peekable<I>`

## Functions

### `peeking_take_while`

```rust
fn peeking_take_while<I, F>(iter: &mut I, f: F) -> PeekingTakeWhile<'_, I, F>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:129-134`](../../../.source_1765900590/itertools-0.14.0/src/peeking_take_while.rs#L129-L134)*

Create a `PeekingTakeWhile`

## Macros

### `peeking_next_by_clone!`

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:167-185`](../../../.source_1765900590/itertools-0.14.0/src/peeking_take_while.rs#L167-L185)*

