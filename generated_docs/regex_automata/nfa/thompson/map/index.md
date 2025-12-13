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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:81-94`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L81-L94)*

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

  Create a new bounded map with the given capacity. The map will never

  grow beyond the given size.

  

  Note that this does not allocate. Instead, callers must call `clear`

  before using this map. `clear` will allocate space if necessary.

  

  This avoids the need to pay for the allocation of this map when

  compiling regexes that lack large Unicode character classes.

- <span id="utf8boundedmap-clear"></span>`fn clear(&mut self)`

  Clear this map of all entries, but permit the reuse of allocation

  if possible.

  

  This must be called before the map can be used.

- <span id="utf8boundedmap-hash"></span>`fn hash(&self, key: &[Transition]) -> usize` — [`Transition`](../nfa/index.md#transition)

  Return a hash of the given transitions.

- <span id="utf8boundedmap-get"></span>`fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID>` — [`Transition`](../nfa/index.md#transition), [`StateID`](../../../util/primitives/index.md#stateid)

  Retrieve the cached state ID corresponding to the given key. The hash

  given must have been computed with `hash` using the same key value.

  

  If there is no cached state with the given transitions, then None is

  returned.

- <span id="utf8boundedmap-set"></span>`fn set(&mut self, key: Vec<Transition>, hash: usize, state_id: StateID)` — [`Transition`](../nfa/index.md#transition), [`StateID`](../../../util/primitives/index.md#stateid)

  Add a cached state to this map with the given key. Callers should

  ensure that `state_id` points to a state that contains precisely the

  NFA transitions given.

  

  `hash` must have been computed using the `hash` method with the same

  key.

#### Trait Implementations

##### `impl Any for Utf8BoundedMap`

- <span id="utf8boundedmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8BoundedMap`

- <span id="utf8boundedmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8BoundedMap`

- <span id="utf8boundedmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8BoundedMap`

- <span id="utf8boundedmap-clone"></span>`fn clone(&self) -> Utf8BoundedMap` — [`Utf8BoundedMap`](#utf8boundedmap)

##### `impl CloneToUninit for Utf8BoundedMap`

- <span id="utf8boundedmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8BoundedMap`

- <span id="utf8boundedmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8BoundedMap`

- <span id="utf8boundedmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8BoundedMap`

- <span id="utf8boundedmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8BoundedMap`

- <span id="utf8boundedmap-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8boundedmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8boundedmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8BoundedMap`

- <span id="utf8boundedmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8boundedmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8BoundedMap`

- <span id="utf8boundedmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8boundedmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8BoundedEntry`

```rust
struct Utf8BoundedEntry {
    version: u16,
    key: alloc::vec::Vec<crate::nfa::thompson::Transition>,
    val: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:98-108`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L98-L108)*

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

##### `impl Any for Utf8BoundedEntry`

- <span id="utf8boundedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8BoundedEntry`

- <span id="utf8boundedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8BoundedEntry`

- <span id="utf8boundedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8BoundedEntry`

- <span id="utf8boundedentry-clone"></span>`fn clone(&self) -> Utf8BoundedEntry` — [`Utf8BoundedEntry`](#utf8boundedentry)

##### `impl CloneToUninit for Utf8BoundedEntry`

- <span id="utf8boundedentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8BoundedEntry`

- <span id="utf8boundedentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8BoundedEntry`

- <span id="utf8boundedentry-default"></span>`fn default() -> Utf8BoundedEntry` — [`Utf8BoundedEntry`](#utf8boundedentry)

##### `impl<T> From for Utf8BoundedEntry`

- <span id="utf8boundedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8BoundedEntry`

- <span id="utf8boundedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8BoundedEntry`

- <span id="utf8boundedentry-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8boundedentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8boundedentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8BoundedEntry`

- <span id="utf8boundedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8boundedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8BoundedEntry`

- <span id="utf8boundedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8boundedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8SuffixMap`

```rust
struct Utf8SuffixMap {
    version: u16,
    capacity: usize,
    map: alloc::vec::Vec<Utf8SuffixEntry>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:190-200`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L190-L200)*

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

  Create a new bounded map with the given capacity. The map will never

  grow beyond the given size.

  

  Note that this does not allocate. Instead, callers must call `clear`

  before using this map. `clear` will allocate space if necessary.

  

  This avoids the need to pay for the allocation of this map when

  compiling regexes that lack large Unicode character classes.

- <span id="utf8suffixmap-clear"></span>`fn clear(&mut self)`

  Clear this map of all entries, but permit the reuse of allocation

  if possible.

  

  This must be called before the map can be used.

- <span id="utf8suffixmap-hash"></span>`fn hash(&self, key: &Utf8SuffixKey) -> usize` — [`Utf8SuffixKey`](#utf8suffixkey)

  Return a hash of the given transition.

- <span id="utf8suffixmap-get"></span>`fn get(&mut self, key: &Utf8SuffixKey, hash: usize) -> Option<StateID>` — [`Utf8SuffixKey`](#utf8suffixkey), [`StateID`](../../../util/primitives/index.md#stateid)

  Retrieve the cached state ID corresponding to the given key. The hash

  given must have been computed with `hash` using the same key value.

  

  If there is no cached state with the given key, then None is returned.

- <span id="utf8suffixmap-set"></span>`fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID)` — [`Utf8SuffixKey`](#utf8suffixkey), [`StateID`](../../../util/primitives/index.md#stateid)

  Add a cached state to this map with the given key. Callers should

  ensure that `state_id` points to a state that contains precisely the

  NFA transition given.

  

  `hash` must have been computed using the `hash` method with the same

  key.

#### Trait Implementations

##### `impl Any for Utf8SuffixMap`

- <span id="utf8suffixmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8SuffixMap`

- <span id="utf8suffixmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8SuffixMap`

- <span id="utf8suffixmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8SuffixMap`

- <span id="utf8suffixmap-clone"></span>`fn clone(&self) -> Utf8SuffixMap` — [`Utf8SuffixMap`](#utf8suffixmap)

##### `impl CloneToUninit for Utf8SuffixMap`

- <span id="utf8suffixmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8SuffixMap`

- <span id="utf8suffixmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Utf8SuffixMap`

- <span id="utf8suffixmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8SuffixMap`

- <span id="utf8suffixmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8SuffixMap`

- <span id="utf8suffixmap-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8suffixmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8suffixmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8SuffixMap`

- <span id="utf8suffixmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8suffixmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8SuffixMap`

- <span id="utf8suffixmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8suffixmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8SuffixKey`

```rust
struct Utf8SuffixKey {
    pub from: crate::util::primitives::StateID,
    pub start: u8,
    pub end: u8,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:205-209`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L205-L209)*

A key that uniquely identifies an NFA state. It is a triple that represents
a transition from one state for a particular byte range.

#### Trait Implementations

##### `impl Any for Utf8SuffixKey`

- <span id="utf8suffixkey-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8SuffixKey`

- <span id="utf8suffixkey-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8SuffixKey`

- <span id="utf8suffixkey-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8SuffixKey`

- <span id="utf8suffixkey-clone"></span>`fn clone(&self) -> Utf8SuffixKey` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl CloneToUninit for Utf8SuffixKey`

- <span id="utf8suffixkey-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8SuffixKey`

- <span id="utf8suffixkey-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8SuffixKey`

- <span id="utf8suffixkey-default"></span>`fn default() -> Utf8SuffixKey` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl Eq for Utf8SuffixKey`

##### `impl<T> From for Utf8SuffixKey`

- <span id="utf8suffixkey-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8SuffixKey`

- <span id="utf8suffixkey-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Utf8SuffixKey`

- <span id="utf8suffixkey-partialeq-eq"></span>`fn eq(&self, other: &Utf8SuffixKey) -> bool` — [`Utf8SuffixKey`](#utf8suffixkey)

##### `impl StructuralPartialEq for Utf8SuffixKey`

##### `impl ToOwned for Utf8SuffixKey`

- <span id="utf8suffixkey-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8suffixkey-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8suffixkey-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8SuffixKey`

- <span id="utf8suffixkey-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8suffixkey-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8SuffixKey`

- <span id="utf8suffixkey-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8suffixkey-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Utf8SuffixEntry`

```rust
struct Utf8SuffixEntry {
    version: u16,
    key: Utf8SuffixKey,
    val: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:213-222`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L213-L222)*

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

##### `impl Any for Utf8SuffixEntry`

- <span id="utf8suffixentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Utf8SuffixEntry`

- <span id="utf8suffixentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Utf8SuffixEntry`

- <span id="utf8suffixentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Utf8SuffixEntry`

- <span id="utf8suffixentry-clone"></span>`fn clone(&self) -> Utf8SuffixEntry` — [`Utf8SuffixEntry`](#utf8suffixentry)

##### `impl CloneToUninit for Utf8SuffixEntry`

- <span id="utf8suffixentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Utf8SuffixEntry`

- <span id="utf8suffixentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Utf8SuffixEntry`

- <span id="utf8suffixentry-default"></span>`fn default() -> Utf8SuffixEntry` — [`Utf8SuffixEntry`](#utf8suffixentry)

##### `impl<T> From for Utf8SuffixEntry`

- <span id="utf8suffixentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Utf8SuffixEntry`

- <span id="utf8suffixentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Utf8SuffixEntry`

- <span id="utf8suffixentry-toowned-type-owned"></span>`type Owned = T`

- <span id="utf8suffixentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="utf8suffixentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Utf8SuffixEntry`

- <span id="utf8suffixentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="utf8suffixentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Utf8SuffixEntry`

- <span id="utf8suffixentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="utf8suffixentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `PRIME`
```rust
const PRIME: u64 = 1_099_511_628_211u64;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:48`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L48)*

### `INIT`
```rust
const INIT: u64 = 14_695_981_039_346_656_037u64;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/map.rs:49`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/map.rs#L49)*

