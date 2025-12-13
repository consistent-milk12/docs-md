*[regex_automata](../../index.md) / [dfa](../index.md) / [remapper](index.md)*

---

# Module `remapper`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`onepass`](#onepass) | mod |  |
| [`Remapper`](#remapper) | struct | Remapper is an abstraction the manages the remapping of state IDs in a finite state machine. |
| [`IndexMapper`](#indexmapper) | struct | A simple type for mapping between state indices and state IDs. |
| [`Remappable`](#remappable) | trait | Remappable is a tightly coupled abstraction that facilitates remapping state identifiers in DFAs. |

## Modules

- [`onepass`](onepass/index.md)

## Structs

### `Remapper`

```rust
struct Remapper {
    map: alloc::vec::Vec<crate::util::primitives::StateID>,
    idxmap: IndexMapper,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/remapper.rs:68-85`](../../../../.source_1765521767/regex-automata-0.4.13/src/dfa/remapper.rs#L68-L85)*

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

- **`idxmap`**: `IndexMapper`

  A mapper from state index to state ID (and back).

#### Implementations

- <span id="remapper-new"></span>`fn new(r: &impl Remappable) -> Remapper` — [`Remappable`](#remappable), [`Remapper`](#remapper)

  Create a new remapper from the given remappable implementation. The

  remapper can then be used to swap states. The remappable value given

  here must the same one given to `swap` and `remap`.

- <span id="remapper-swap"></span>`fn swap(&mut self, r: &mut impl Remappable, id1: StateID, id2: StateID)` — [`Remappable`](#remappable), [`StateID`](../../util/primitives/index.md#stateid)

  Swap two states. Once this is called, callers must follow through to

  call `remap`, or else it's possible for the underlying remappable

  value to be in a corrupt state.

- <span id="remapper-remap"></span>`fn remap(self, r: &mut impl Remappable)` — [`Remappable`](#remappable)

  Complete the remapping process by rewriting all state IDs in the

  remappable value according to the swaps performed.

#### Trait Implementations

##### `impl Any for Remapper`

- <span id="remapper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Remapper`

- <span id="remapper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Remapper`

- <span id="remapper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Remapper`

- <span id="remapper-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Remapper`

- <span id="remapper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Remapper`

- <span id="remapper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Remapper`

- <span id="remapper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="remapper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Remapper`

- <span id="remapper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="remapper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IndexMapper`

```rust
struct IndexMapper {
    stride2: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/dfa/remapper.rs:169-174`](../../../../.source_1765521767/regex-automata-0.4.13/src/dfa/remapper.rs#L169-L174)*

A simple type for mapping between state indices and state IDs.

The reason why this exists is because state IDs are "premultiplied." That
is, in order to get to the transitions for a particular state, one need
only use the state ID as-is, instead of having to multiple it by transition
table's stride.

The downside of this is that it's inconvenient to map between state IDs
using a dense map, e.g., Vec<StateID>. That's because state IDs look like
`0`, `0+stride`, `0+2*stride`, `0+3*stride`, etc., instead of `0`, `1`,
`2`, `3`, etc.

Since our state IDs are premultiplied, we can convert back-and-forth
between IDs and indices by simply unmultiplying the IDs and multiplying the
indices.

#### Fields

- **`stride2`**: `usize`

  The power of 2 corresponding to the stride of the corresponding
  transition table. 'id >> stride2' de-multiplies an ID while 'index <<
  stride2' pre-multiplies an index to an ID.

#### Implementations

- <span id="indexmapper-to-index"></span>`fn to_index(&self, id: StateID) -> usize` — [`StateID`](../../util/primitives/index.md#stateid)

  Convert a state ID to a state index.

- <span id="indexmapper-to-state-id"></span>`fn to_state_id(&self, index: usize) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Convert a state index to a state ID.

#### Trait Implementations

##### `impl Any for IndexMapper`

- <span id="indexmapper-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IndexMapper`

- <span id="indexmapper-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IndexMapper`

- <span id="indexmapper-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for IndexMapper`

- <span id="indexmapper-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IndexMapper`

- <span id="indexmapper-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IndexMapper`

- <span id="indexmapper-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for IndexMapper`

- <span id="indexmapper-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="indexmapper-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IndexMapper`

- <span id="indexmapper-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="indexmapper-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Remappable`

```rust
trait Remappable: core::fmt::Debug { ... }
```

*Defined in [`regex-automata-0.4.13/src/dfa/remapper.rs:28-55`](../../../../.source_1765521767/regex-automata-0.4.13/src/dfa/remapper.rs#L28-L55)*

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

- `fn stride2(&self) -> usize`

  Return the power-of-2 exponent that yields the stride. The pertinent

- `fn swap_states(&mut self, id1: StateID, id2: StateID)`

  Swap the states pointed to by the given IDs. The underlying finite

- `fn remap(&mut self, map: impl Fn(StateID) -> StateID)`

  This must remap every single state ID in the underlying value according

#### Implementors

- [`DFA`](../onepass/index.md#dfa)

