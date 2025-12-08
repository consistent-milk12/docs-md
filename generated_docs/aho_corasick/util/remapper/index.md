*[aho_corasick](../../index.md) / [util](../index.md) / [remapper](index.md)*

---

# Module `remapper`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Remapper`](#remapper) | struct | Remapper is an abstraction the manages the remapping of state IDs in a |
| [`IndexMapper`](#indexmapper) | struct | A simple type for mapping between state indices and state IDs. |
| [`Remappable`](#remappable) | trait | Remappable is a tightly coupled abstraction that facilitates remapping |

## Structs

### `Remapper`

```rust
struct Remapper {
    map: alloc::vec::Vec<crate::util::primitives::StateID>,
    idx: IndexMapper,
}
```

Remapper is an abstraction the manages the remapping of state IDs in a
finite state machine. This is useful when one wants to shuffle states into
different positions in the machine.

One of the key complexities this manages is the ability to correctly move
one state multiple times.

Once shuffling is complete, `remap` must be called, which will rewrite
all pertinent transitions to updated state IDs. Neglecting to call `remap`
will almost certainly result in a corrupt machine.

#### Fields

- **`map`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  A map from the index of a state to its pre-multiplied identifier.
  
  When a state is swapped with another, then their corresponding
  locations in this map are also swapped. Thus, its new position will
  still point to its old pre-multiplied StateID.
  
  While there is a bit more to it, this then allows us to rewrite the
  state IDs in a DFA's transition table in a single pass. This is done
  by iterating over every ID in this map, then iterating over each
  transition for the state at that ID and re-mapping the transition from
  `old_id` to `map[dfa.to_index(old_id)]`. That is, we find the position
  in this map where `old_id` *started*, and set it to where it ended up
  after all swaps have been completed.

- **`idx`**: `IndexMapper`

  A way to map indices to state IDs (and back).

#### Implementations

- <span id="remapper-new"></span>`fn new(r: &impl Remappable, stride2: usize) -> Remapper` — [`Remappable`](#remappable), [`Remapper`](#remapper)

- <span id="remapper-swap"></span>`fn swap(&mut self, r: &mut impl Remappable, id1: StateID, id2: StateID)` — [`Remappable`](#remappable), [`StateID`](../primitives/index.md)

- <span id="remapper-remap"></span>`fn remap(self, r: &mut impl Remappable)` — [`Remappable`](#remappable)

#### Trait Implementations

##### `impl Debug for Remapper`

- <span id="remapper-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `IndexMapper`

```rust
struct IndexMapper {
    stride2: usize,
}
```

A simple type for mapping between state indices and state IDs.

The reason why this exists is because state IDs are "premultiplied" in a
DFA. That is, in order to get to the transitions for a particular state,
one need only use the state ID as-is, instead of having to multiply it by
transition table's stride.

The downside of this is that it's inconvenient to map between state IDs
using a dense map, e.g., Vec<StateID>. That's because state IDs look like
`0`, `stride`, `2*stride`, `3*stride`, etc., instead of `0`, `1`, `2`, `3`,
etc.

Since our state IDs are premultiplied, we can convert back-and-forth
between IDs and indices by simply unmultiplying the IDs and multiplying the
indices.

Note that for a sparse NFA, state IDs and indices are equivalent. In this
case, we set the stride of the index mapped to be `0`, which acts as an
identity.

#### Fields

- **`stride2`**: `usize`

  The power of 2 corresponding to the stride of the corresponding
  transition table. 'id >> stride2' de-multiplies an ID while 'index <<
  stride2' pre-multiplies an index to an ID.

#### Implementations

- <span id="indexmapper-to-index"></span>`fn to_index(&self, id: StateID) -> usize` — [`StateID`](../primitives/index.md)

- <span id="indexmapper-to-state-id"></span>`fn to_state_id(&self, index: usize) -> StateID` — [`StateID`](../primitives/index.md)

#### Trait Implementations

##### `impl Debug for IndexMapper`

- <span id="indexmapper-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Remappable`

```rust
trait Remappable: core::fmt::Debug { ... }
```

Remappable is a tightly coupled abstraction that facilitates remapping
state identifiers in DFAs.

The main idea behind remapping state IDs is that DFAs often need to check
if a certain state is a "special" state of some kind (like a match state)
during a search. Since this is extremely perf critical code, we want this
check to be as fast as possible. Partitioning state IDs into, for example,
into "non-match" and "match" states means one can tell if a state is a
match state via a simple comparison of the state ID.

The issue is that during the DFA construction process, it's not
particularly easy to partition the states. Instead, the simplest thing is
to often just do a pass over all of the states and shuffle them into their
desired partitionings. To do that, we need a mechanism for swapping states.
Hence, this abstraction.

Normally, for such little code, I would just duplicate it. But this is a
key optimization and the implementation is a bit subtle. So the abstraction
is basically a ham-fisted attempt at DRY. The only place we use this is in
the dense and one-pass DFAs.

See also src/dfa/special.rs for a more detailed explanation of how dense
DFAs are partitioned.

#### Required Methods

- `fn state_len(&self) -> usize`

  Return the total number of states.

- `fn swap_states(&mut self, id1: StateID, id2: StateID)`

  Swap the states pointed to by the given IDs. The underlying finite

- `fn remap(&mut self, map: impl Fn(StateID) -> StateID)`

  This must remap every single state ID in the underlying value according

