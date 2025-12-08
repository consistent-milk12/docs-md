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

#### Trait Implementations

##### `impl Clone for Special`

- <span id="special-clone"></span>`fn clone(&self) -> Special` — [`Special`](#special)

##### `impl Debug for Special`

- <span id="special-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

