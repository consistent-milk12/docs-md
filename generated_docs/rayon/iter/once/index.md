*[rayon](../../index.md) / [iter](../index.md) / [once](index.md)*

---

# Module `once`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Once`](#once) | struct | Iterator adaptor for [the `once()` function]. |
| [`once`](#once) | fn | Creates a parallel iterator that produces an element exactly once. |

## Structs

### `Once<T>`

```rust
struct Once<T> {
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:32-34`](../../../../.source_1765633015/rayon-1.11.0/src/iter/once.rs#L32-L34)*

Iterator adaptor for [the `once()` function].


#### Trait Implementations

##### `impl<T> Any for Once<T>`

- <span id="once-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Once<T>`

- <span id="once-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Once<T>`

- <span id="once-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Once<T>`

- <span id="once-clone"></span>`fn clone(&self) -> Once<T>` — [`Once`](#once)

##### `impl<T> CloneToUninit for Once<T>`

- <span id="once-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Once<T>`

- <span id="once-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Once<T>`

- <span id="once-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for Once<T>`

- <span id="once-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="once-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="once-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T, U> Into for Once<T>`

- <span id="once-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Once<T>`

##### `impl<T> IntoParallelIterator for Once<T>`

- <span id="once-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="once-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="once-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Once<T>`

- <span id="once-paralleliterator-type-item"></span>`type Item = T`

- <span id="once-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="once-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Once<T>`

- <span id="once-pointable-const-align"></span>`const ALIGN: usize`

- <span id="once-pointable-type-init"></span>`type Init = T`

- <span id="once-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="once-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="once-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="once-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Once<T>`

- <span id="once-toowned-type-owned"></span>`type Owned = T`

- <span id="once-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="once-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Once<T>`

- <span id="once-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="once-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Once<T>`

- <span id="once-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="once-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `once`

```rust
fn once<T: Send>(item: T) -> Once<T>
```

*Defined in [`rayon-1.11.0/src/iter/once.rs:24-26`](../../../../.source_1765633015/rayon-1.11.0/src/iter/once.rs#L24-L26)*

Creates a parallel iterator that produces an element exactly once.

This admits no parallelism on its own, but it could be chained to existing
parallel iterators to extend their contents, or otherwise used for any code
that deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::once;

let pi = (0..1234).into_par_iter()
    .chain(once(-1))
    .chain(1234..10_000);

assert_eq!(pi.clone().count(), 10_001);
assert_eq!(pi.clone().filter(|&x| x < 0).count(), 1);
assert_eq!(pi.position_any(|x| x < 0), Some(1234));
```

