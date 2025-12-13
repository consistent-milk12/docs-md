*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [teddy](index.md)*

---

# Module `teddy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Teddy`](#teddy) | struct |  |

## Structs

### `Teddy`

```rust
struct Teddy {
    searcher: aho_corasick::packed::Searcher,
    anchored_ac: aho_corasick::dfa::DFA,
    minimum_len: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/teddy.rs:7-36`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/prefilter/teddy.rs#L7-L36)*

#### Fields

- **`searcher`**: `aho_corasick::packed::Searcher`

  The actual Teddy searcher.
  
  Technically, it's possible that Teddy doesn't actually get used, since
  Teddy does require its haystack to at least be of a certain size
  (usually around the size of whatever vector is being used, so ~16
  or ~32 bytes). For haystacks shorter than that, the implementation
  currently uses Rabin-Karp.

- **`anchored_ac`**: `aho_corasick::dfa::DFA`

  When running an anchored search, the packed searcher can't handle it so
  we defer to Aho-Corasick itself. Kind of sad, but changing the packed
  searchers to support anchored search would be difficult at worst and
  annoying at best. Since packed searchers only apply to small numbers of
  literals, we content ourselves that this is not much of an added cost.
  (That packed searchers only work with a small number of literals is
  also why we use a DFA here. Otherwise, the memory usage of a DFA would
  likely be unacceptable.)

- **`minimum_len`**: `usize`

  The length of the smallest literal we look for.
  
  We use this as a heuristic to figure out whether this will be "fast" or
  not. Generally, the longer the better, because longer needles are more
  discriminating and thus reduce false positive rate.

#### Implementations

- <span id="teddy-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Teddy>` — [`MatchKind`](../../../index.md#matchkind), [`Teddy`](#teddy)

#### Trait Implementations

##### `impl Any for Teddy`

- <span id="teddy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Teddy`

- <span id="teddy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Teddy`

- <span id="teddy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Teddy`

- <span id="teddy-clone"></span>`fn clone(&self) -> Teddy` — [`Teddy`](#teddy)

##### `impl CloneToUninit for Teddy`

- <span id="teddy-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Teddy`

- <span id="teddy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Teddy`

- <span id="teddy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Teddy`

- <span id="teddy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Teddy`

- <span id="teddy-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="teddy-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="teddy-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="teddy-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for Teddy`

- <span id="teddy-toowned-type-owned"></span>`type Owned = T`

- <span id="teddy-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="teddy-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Teddy`

- <span id="teddy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="teddy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Teddy`

- <span id="teddy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="teddy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

