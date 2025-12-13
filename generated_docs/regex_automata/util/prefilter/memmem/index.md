*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memmem](index.md)*

---

# Module `memmem`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Memmem`](#memmem) | struct |  |

## Structs

### `Memmem`

```rust
struct Memmem {
    finder: memchr::memmem::Finder<'static>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/memmem.rs:7-12`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/memmem.rs#L7-L12)*

#### Implementations

- <span id="memmem-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memmem>` — [`MatchKind`](../../../index.md#matchkind), [`Memmem`](#memmem)

#### Trait Implementations

##### `impl Any for Memmem`

- <span id="memmem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memmem`

- <span id="memmem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memmem`

- <span id="memmem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memmem`

- <span id="memmem-clone"></span>`fn clone(&self) -> Memmem` — [`Memmem`](#memmem)

##### `impl CloneToUninit for Memmem`

- <span id="memmem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memmem`

- <span id="memmem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Memmem`

- <span id="memmem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Memmem`

- <span id="memmem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Memmem`

- <span id="memmem-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memmem-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memmem-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memmem-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for Memmem`

- <span id="memmem-toowned-type-owned"></span>`type Owned = T`

- <span id="memmem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memmem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memmem`

- <span id="memmem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memmem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memmem`

- <span id="memmem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memmem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

