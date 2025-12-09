*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [map](index.md)*

---

# Module `map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8BoundedMap`](#utf8boundedmap) | struct | A bounded hash map where the key is a sequence of NFA transitions and the value is a pre-existing NFA state ID. |
| [`Utf8BoundedEntry`](#utf8boundedentry) | struct | An entry in this map. |
| [`Utf8SuffixMap`](#utf8suffixmap) | struct | A cache of suffixes used to modestly compress UTF-8 automata for large Unicode character classes. |
| [`Utf8SuffixKey`](#utf8suffixkey) | struct | A key that uniquely identifies an NFA state. |
| [`Utf8SuffixEntry`](#utf8suffixentry) | struct | An entry in this map. |
| [`PRIME`](#prime) | const |  |
| [`INIT`](#init) | const |  |

## Structs

### `Utf8BoundedMap`

```rust
struct Utf8BoundedMap {
    version: u16,
    capacity: usize,
    map: alloc::vec::Vec<Utf8BoundedEntry>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:81-94`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L81-L94)*

A bounded hash map where the key is a sequence of NFA transitions and the
value is a pre-existing NFA state ID.

std's hashmap can be used for this, however, this map has two important
advantages. Firstly, it has lower overhead. Secondly, it permits us to
control our memory usage by limited the number of slots. In general, the
cost here is that this map acts as a cache. That is, inserting a new entry
may remove an old entry. We are okay with this, since it does not impact
correctness in the cases where it is used. The only effect that dropping
states from the cache has is that the resulting NFA generated may be bigger
than it otherwise would be.

This improves benchmarks that compile large Unicode character classes,
since it makes the generation of (almost) minimal UTF-8 automaton faster.
Specifically, one could observe the difference with std's hashmap via
something like the following benchmark:

  hyperfine "regex-cli debug thompson -qr --captures none '\w{90} ecurB'"

But to observe that difference, you'd have to modify the code to use
std's hashmap.

It is quite possible that there is a better way to approach this problem.
For example, if there happens to be a very common state that collides with
a lot of less frequent states, then we could wind up with very poor caching
behavior. Alas, the effectiveness of this cache has not been measured.
Instead, ad hoc experiments suggest that it is "good enough." Additional
smarts (such as an LRU eviction policy) have to be weighed against the
amount of extra time they cost.

#### Fields

- **`version`**: `u16`

  The current version of this map. Only entries with matching versions
  are considered during lookups. If an entry is found with a mismatched
  version, then the map behaves as if the entry does not exist.
  
  This makes it possible to clear the map by simply incrementing the
  version number instead of actually deallocating any storage.

- **`capacity`**: `usize`

  The total number of entries this map can store.

- **`map`**: `alloc::vec::Vec<Utf8BoundedEntry>`

  The actual entries, keyed by hash. Collisions between different states
  result in the old state being dropped.

#### Implementations

- <span id="utf8boundedmap-new"></span>`fn new(capacity: usize) -> Utf8BoundedMap` — [`Utf8BoundedMap`](#utf8boundedmap)

- <span id="utf8boundedmap-clear"></span>`fn clear(&mut self)`

- <span id="utf8boundedmap-hash"></span>`fn hash(&self, key: &[Transition]) -> usize` — [`Transition`](../nfa/index.md)

- <span id="utf8boundedmap-get"></span>`fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID>` — [`Transition`](../nfa/index.md), [`StateID`](../../../util/primitives/index.md)

- <span id="utf8boundedmap-set"></span>`fn set(&mut self, key: Vec<Transition>, hash: usize, state_id: StateID)` — [`Transition`](../nfa/index.md), [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Utf8BoundedMap`

- <span id="utf8boundedmap-clone"></span>`fn clone(&self) -> Utf8BoundedMap` — [`Utf8BoundedMap`](#utf8boundedmap)

##### `impl Debug for Utf8BoundedMap`

- <span id="utf8boundedmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8BoundedEntry`

```rust
struct Utf8BoundedEntry {
    version: u16,
    key: alloc::vec::Vec<crate::nfa::thompson::Transition>,
    val: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:98-108`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L98-L108)*

An entry in this map.

#### Fields

- **`version`**: `u16`

  The version of the map used to produce this entry. If this entry's
  version does not match the current version of the map, then the map
  should behave as if this entry does not exist.

- **`key`**: `alloc::vec::Vec<crate::nfa::thompson::Transition>`

  The key, which is a sorted sequence of non-overlapping NFA transitions.

- **`val`**: `crate::util::primitives::StateID`

  The state ID corresponding to the state containing the transitions in
  this entry.

#### Trait Implementations

##### `impl Clone for Utf8BoundedEntry`

- <span id="utf8boundedentry-clone"></span>`fn clone(&self) -> Utf8BoundedEntry` — [`Utf8BoundedEntry`](#utf8boundedentry)

##### `impl Debug for Utf8BoundedEntry`

- <span id="utf8boundedentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8BoundedEntry`

- <span id="utf8boundedentry-default"></span>`fn default() -> Utf8BoundedEntry` — [`Utf8BoundedEntry`](#utf8boundedentry)

### `Utf8SuffixMap`

```rust
struct Utf8SuffixMap {
    version: u16,
    capacity: usize,
    map: alloc::vec::Vec<Utf8SuffixEntry>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:190-200`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L190-L200)*

A cache of suffixes used to modestly compress UTF-8 automata for large
Unicode character classes.

#### Fields

- **`version`**: `u16`

  The current version of this map. Only entries with matching versions
  are considered during lookups. If an entry is found with a mismatched
  version, then the map behaves as if the entry does not exist.

- **`capacity`**: `usize`

  The total number of entries this map can store.

- **`map`**: `alloc::vec::Vec<Utf8SuffixEntry>`

  The actual entries, keyed by hash. Collisions between different states
  result in the old state being dropped.

#### Implementations

- <span id="utf8suffixmap-new"></span>`fn new(capacity: usize) -> Utf8SuffixMap` — [`Utf8SuffixMap`](#utf8suffixmap)

- <span id="utf8suffixmap-clear"></span>`fn clear(&mut self)`

- <span id="utf8suffixmap-hash"></span>`fn hash(&self, key: &Utf8SuffixKey) -> usize` — [`Utf8SuffixKey`](#utf8suffixkey)

- <span id="utf8suffixmap-get"></span>`fn get(&mut self, key: &Utf8SuffixKey, hash: usize) -> Option<StateID>` — [`Utf8SuffixKey`](#utf8suffixkey), [`StateID`](../../../util/primitives/index.md)

- <span id="utf8suffixmap-set"></span>`fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID)` — [`Utf8SuffixKey`](#utf8suffixkey), [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Utf8SuffixMap`

- <span id="utf8suffixmap-clone"></span>`fn clone(&self) -> Utf8SuffixMap` — [`Utf8SuffixMap`](#utf8suffixmap)

##### `impl Debug for Utf8SuffixMap`

- <span id="utf8suffixmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Utf8SuffixKey`

```rust
struct Utf8SuffixKey {
    pub from: crate::util::primitives::StateID,
    pub start: u8,
    pub end: u8,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:205-209`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L205-L209)*

A key that uniquely identifies an NFA state. It is a triple that represents
a transition from one state for a particular byte range.

#### Trait Implementations

##### `impl Clone for Utf8SuffixKey`

- <span id="utf8suffixkey-clone"></span>`fn clone(&self) -> Utf8SuffixKey` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl Debug for Utf8SuffixKey`

- <span id="utf8suffixkey-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8SuffixKey`

- <span id="utf8suffixkey-default"></span>`fn default() -> Utf8SuffixKey` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl Eq for Utf8SuffixKey`

##### `impl PartialEq for Utf8SuffixKey`

- <span id="utf8suffixkey-eq"></span>`fn eq(&self, other: &Utf8SuffixKey) -> bool` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl StructuralPartialEq for Utf8SuffixKey`

### `Utf8SuffixEntry`

```rust
struct Utf8SuffixEntry {
    version: u16,
    key: Utf8SuffixKey,
    val: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:213-222`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L213-L222)*

An entry in this map.

#### Fields

- **`version`**: `u16`

  The version of the map used to produce this entry. If this entry's
  version does not match the current version of the map, then the map
  should behave as if this entry does not exist.

- **`key`**: `Utf8SuffixKey`

  The key, which consists of a transition in a particular state.

- **`val`**: `crate::util::primitives::StateID`

  The identifier that the transition in the key maps to.

#### Trait Implementations

##### `impl Clone for Utf8SuffixEntry`

- <span id="utf8suffixentry-clone"></span>`fn clone(&self) -> Utf8SuffixEntry` — [`Utf8SuffixEntry`](#utf8suffixentry)

##### `impl Debug for Utf8SuffixEntry`

- <span id="utf8suffixentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8SuffixEntry`

- <span id="utf8suffixentry-default"></span>`fn default() -> Utf8SuffixEntry` — [`Utf8SuffixEntry`](#utf8suffixentry)

## Constants

### `PRIME`
```rust
const PRIME: u64 = 1_099_511_628_211u64;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:48`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L48)*

### `INIT`
```rust
const INIT: u64 = 14_695_981_039_346_656_037u64;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:49`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/map.rs#L49)*

