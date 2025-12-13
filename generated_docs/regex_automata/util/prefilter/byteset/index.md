*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [byteset](index.md)*

---

# Module `byteset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ByteSet`](#byteset) | struct |  |

## Structs

### `ByteSet`

```rust
struct ByteSet([bool; 256]);
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/byteset.rs:7`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/byteset.rs#L7)*

#### Implementations

- <span id="byteset-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<ByteSet>` — [`MatchKind`](../../../index.md#matchkind), [`ByteSet`](#byteset)

#### Trait Implementations

##### `impl Any for ByteSet`

- <span id="byteset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteSet`

- <span id="byteset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteSet`

- <span id="byteset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteSet`

- <span id="byteset-clone"></span>`fn clone(&self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl CloneToUninit for ByteSet`

- <span id="byteset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ByteSet`

- <span id="byteset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteSet`

- <span id="byteset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteSet`

- <span id="byteset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for ByteSet`

- <span id="byteset-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="byteset-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="byteset-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="byteset-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for ByteSet`

- <span id="byteset-toowned-type-owned"></span>`type Owned = T`

- <span id="byteset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byteset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteSet`

- <span id="byteset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteSet`

- <span id="byteset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

