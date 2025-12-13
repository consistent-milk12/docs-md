*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [aho_corasick](index.md)*

---

# Module `aho_corasick`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AhoCorasick`](#ahocorasick) | struct |  |

## Structs

### `AhoCorasick`

```rust
struct AhoCorasick {
    ac: aho_corasick::AhoCorasick,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/aho_corasick.rs:7-12`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/aho_corasick.rs#L7-L12)*

#### Implementations

- <span id="ahocorasick-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<AhoCorasick>` — [`MatchKind`](../../../index.md#matchkind), [`AhoCorasick`](#ahocorasick)

#### Trait Implementations

##### `impl Any for AhoCorasick`

- <span id="ahocorasick-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AhoCorasick`

- <span id="ahocorasick-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AhoCorasick`

- <span id="ahocorasick-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AhoCorasick`

- <span id="ahocorasick-clone"></span>`fn clone(&self) -> AhoCorasick` — [`AhoCorasick`](#ahocorasick)

##### `impl CloneToUninit for AhoCorasick`

- <span id="ahocorasick-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AhoCorasick`

- <span id="ahocorasick-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for AhoCorasick`

- <span id="ahocorasick-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AhoCorasick`

- <span id="ahocorasick-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for AhoCorasick`

- <span id="ahocorasick-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="ahocorasick-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="ahocorasick-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="ahocorasick-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for AhoCorasick`

- <span id="ahocorasick-toowned-type-owned"></span>`type Owned = T`

- <span id="ahocorasick-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ahocorasick-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AhoCorasick`

- <span id="ahocorasick-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ahocorasick-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AhoCorasick`

- <span id="ahocorasick-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ahocorasick-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

