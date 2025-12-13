*[aho_corasick](../../index.md) / [util](../index.md) / [special](index.md)*

---

# Module `special`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Special`](#special) | struct | A collection of sentinel state IDs for Aho-Corasick automata. |

## Structs

### `Special`

```rust
struct Special {
    max_special_id: crate::util::primitives::StateID,
    max_match_id: crate::util::primitives::StateID,
    start_unanchored_id: crate::util::primitives::StateID,
    start_anchored_id: crate::util::primitives::StateID,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/special.rs:10-28`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/special.rs#L10-L28)*

A collection of sentinel state IDs for Aho-Corasick automata.

This specifically enables the technique by which we determine which states
are dead, matches or start states. Namely, by arranging states in a
particular order, we can determine the type of a state simply by looking at
its ID.

#### Fields

- **`max_special_id`**: `crate::util::primitives::StateID`

  The maximum ID of all the "special" states. This corresponds either to
  start_anchored_id when a prefilter is active and max_match_id when a
  prefilter is not active. The idea here is that if there is no prefilter,
  then there is no point in treating start states as special.

- **`max_match_id`**: `crate::util::primitives::StateID`

  The maximum ID of all the match states. Any state ID bigger than this
  is guaranteed to be a non-match ID.
  
  It is possible and legal for max_match_id to be equal to
  start_anchored_id, which occurs precisely in the case where the empty
  string is a pattern that was added to the underlying automaton.

- **`start_unanchored_id`**: `crate::util::primitives::StateID`

  The state ID of the start state used for unanchored searches.

- **`start_anchored_id`**: `crate::util::primitives::StateID`

  The state ID of the start state used for anchored searches. This is
  always start_unanchored_id+1.

#### Implementations

- <span id="special-zero"></span>`fn zero() -> Special` — [`Special`](#special)

  Create a new set of "special" state IDs with all IDs initialized to

  zero. The general idea here is that they will be updated and set to

  correct values later.

#### Trait Implementations

##### `impl Any for Special`

- <span id="special-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Special`

- <span id="special-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Special`

- <span id="special-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Special`

- <span id="special-clone"></span>`fn clone(&self) -> Special` — [`Special`](#special)

##### `impl CloneToUninit for Special`

- <span id="special-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Special`

- <span id="special-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Special`

- <span id="special-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Special`

- <span id="special-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Special`

- <span id="special-toowned-type-owned"></span>`type Owned = T`

- <span id="special-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="special-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Special`

- <span id="special-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="special-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Special`

- <span id="special-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="special-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

