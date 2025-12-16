*[itertools](../index.md) / [lazy_buffer](index.md)*

---

# Module `lazy_buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyBuffer`](#lazybuffer) | struct |  |

## Structs

### `LazyBuffer<I: Iterator>`

```rust
struct LazyBuffer<I: Iterator> {
    it: std::iter::Fuse<I>,
    buffer: alloc::vec::Vec<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/lazy_buffer.rs:8-11`](../../../.source_1765900590/itertools-0.14.0/src/lazy_buffer.rs#L8-L11)*

#### Implementations

- <span id="lazybuffer-new"></span>`fn new(it: I) -> Self`

- <span id="lazybuffer-len"></span>`fn len(&self) -> usize`

- <span id="lazybuffer-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="lazybuffer-count"></span>`fn count(self) -> usize`

- <span id="lazybuffer-get-next"></span>`fn get_next(&mut self) -> bool`

- <span id="lazybuffer-prefill"></span>`fn prefill(&mut self, len: usize)`

#### Trait Implementations

##### `impl Any for LazyBuffer<I>`

- <span id="lazybuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LazyBuffer<I>`

- <span id="lazybuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LazyBuffer<I>`

- <span id="lazybuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for LazyBuffer<I>`

- <span id="lazybuffer-clone"></span>`fn clone(&self) -> LazyBuffer<I>` — [`LazyBuffer`](#lazybuffer)

##### `impl CloneToUninit for LazyBuffer<I>`

- <span id="lazybuffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for LazyBuffer<I>`

- <span id="lazybuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LazyBuffer<I>`

- <span id="lazybuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> Index for LazyBuffer<I>`

- <span id="lazybuffer-index-type-output"></span>`type Output = <Vec<<I as Iterator>::Item> as Index>::Output`

- <span id="lazybuffer-index"></span>`fn index(&self, index: J) -> &<Self as >::Output`

##### `impl<U> Into for LazyBuffer<I>`

- <span id="lazybuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for LazyBuffer<I>`

##### `impl ToOwned for LazyBuffer<I>`

- <span id="lazybuffer-toowned-type-owned"></span>`type Owned = T`

- <span id="lazybuffer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lazybuffer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LazyBuffer<I>`

- <span id="lazybuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazybuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LazyBuffer<I>`

- <span id="lazybuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazybuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

