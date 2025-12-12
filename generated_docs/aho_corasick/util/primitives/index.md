*[aho_corasick](../../index.md) / [util](../index.md) / [primitives](index.md)*

---

# Module `primitives`

Lower level primitive types that are useful in a variety of circumstances.

# Overview

This list represents the principle types in this module and briefly describes
when you might want to use them.

* [`PatternID`](#patternid) - A type that represents the identifier of a regex pattern.
This is probably the most widely used type in this module (which is why it's
also re-exported in the crate root).
* [`StateID`](#stateid) - A type the represents the identifier of a finite automaton
state. This is used for both NFAs and DFAs, with the notable exception of
the hybrid NFA/DFA. (The hybrid NFA/DFA uses a special purpose "lazy" state
identifier.)
* [`SmallIndex`](#smallindex) - The internal representation of both a `PatternID` and a
`StateID`. Its purpose is to serve as a type that can index memory without
being as big as a `usize` on 64-bit targets. The main idea behind this type
is that there are many things in regex engines that will, in practice, never
overflow a 32-bit integer. (For example, like the number of patterns in a regex
or the number of states in an NFA.) Thus, a `SmallIndex` can be used to index
memory without peppering `as` casts everywhere. Moreover, it forces callers
to handle errors in the case where, somehow, the value would otherwise overflow
either a 32-bit integer or a `usize` (e.g., on 16-bit targets).

## Contents

- [Structs](#structs)
  - [`SmallIndex`](#smallindex)
  - [`SmallIndexError`](#smallindexerror)
  - [`SmallIndexIter`](#smallindexiter)
  - [`PatternID`](#patternid)
  - [`StateID`](#stateid)
  - [`PatternIDError`](#patterniderror)
  - [`PatternIDIter`](#patterniditer)
  - [`WithPatternIDIter`](#withpatterniditer)
  - [`StateIDError`](#stateiderror)
  - [`StateIDIter`](#stateiditer)
  - [`WithStateIDIter`](#withstateiditer)
- [Traits](#traits)
  - [`IteratorIndexExt`](#iteratorindexext)
- [Macros](#macros)
  - [`index_type_impls!`](#index-type-impls)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SmallIndex`](#smallindex) | struct | A type that represents a "small" index. |
| [`SmallIndexError`](#smallindexerror) | struct | This error occurs when a small index could not be constructed. |
| [`SmallIndexIter`](#smallindexiter) | struct |  |
| [`PatternID`](#patternid) | struct | The identifier of a pattern in an Aho-Corasick automaton. |
| [`StateID`](#stateid) | struct | The identifier of a finite automaton state. |
| [`PatternIDError`](#patterniderror) | struct | This error occurs when an ID could not be constructed. |
| [`PatternIDIter`](#patterniditer) | struct |  |
| [`WithPatternIDIter`](#withpatterniditer) | struct | An iterator adapter that is like std::iter::Enumerate, but attaches small index values instead. |
| [`StateIDError`](#stateiderror) | struct | This error occurs when an ID could not be constructed. |
| [`StateIDIter`](#stateiditer) | struct |  |
| [`WithStateIDIter`](#withstateiditer) | struct | An iterator adapter that is like std::iter::Enumerate, but attaches small index values instead. |
| [`IteratorIndexExt`](#iteratorindexext) | trait | A utility trait that defines a couple of adapters for making it convenient to access indices as "small index" types. |
| [`index_type_impls!`](#index-type-impls) | macro |  |

## Structs

### `SmallIndex`

```rust
struct SmallIndex(u32);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:96`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L96)*

A type that represents a "small" index.

The main idea of this type is to provide something that can index memory,
but uses less memory than `usize` on 64-bit systems. Specifically, its
representation is always a `u32` and has `repr(transparent)` enabled. (So
it is safe to transmute between a `u32` and a `SmallIndex`.)

A small index is typically useful in cases where there is no practical way
that the index will overflow a 32-bit integer. A good example of this is
an NFA state. If you could somehow build an NFA with `2^30` states, its
memory usage would be exorbitant and its runtime execution would be so
slow as to be completely worthless. Therefore, this crate generally deems
it acceptable to return an error if it would otherwise build an NFA that
requires a slice longer than what a 32-bit integer can index. In exchange,
we can use 32-bit indices instead of 64-bit indices in various places.

This type ensures this by providing a constructor that will return an error
if its argument cannot fit into the type. This makes it much easier to
handle these sorts of boundary cases that are otherwise extremely subtle.

On all targets, this type guarantees that its value will fit in a `u32`,
`i32`, `usize` and an `isize`. This means that on 16-bit targets, for
example, this type's maximum value will never overflow an `isize`,
which means it will never overflow a `i16` even though its internal
representation is still a `u32`.

The purpose for making the type fit into even signed integer types like
`isize` is to guarantee that the difference between any two small indices
is itself also a small index. This is useful in certain contexts, e.g.,
for delta encoding.

# Other types

The following types wrap `SmallIndex` to provide a more focused use case:

* [`PatternID`](#patternid) is for representing the identifiers of patterns.
* [`StateID`](#stateid) is for representing the identifiers of states in finite
automata. It is used for both NFAs and DFAs.

# Representation

This type is always represented internally by a `u32` and is marked as
`repr(transparent)`. Thus, this type always has the same representation as
a `u32`. It is thus safe to transmute between a `u32` and a `SmallIndex`.

# Indexing

For convenience, callers may use a `SmallIndex` to index slices.

# Safety

While a `SmallIndex` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `SmallIndex` with
an invalid value can be done in entirely safe code. This may in turn result
in panics or silent logical errors.

#### Implementations

- <span id="smallindex-const-max"></span>`const MAX: SmallIndex`

- <span id="smallindex-const-limit"></span>`const LIMIT: usize`

- <span id="smallindex-const-zero"></span>`const ZERO: SmallIndex`

- <span id="smallindex-const-size"></span>`const SIZE: usize`

- <span id="smallindex-new"></span>`fn new(index: usize) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

- <span id="smallindex-new-unchecked"></span>`const fn new_unchecked(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

- <span id="smallindex-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> SmallIndex` — [`SmallIndex`](#smallindex)

- <span id="smallindex-must"></span>`fn must(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

- <span id="smallindex-as-usize"></span>`const fn as_usize(&self) -> usize`

- <span id="smallindex-as-u64"></span>`const fn as_u64(&self) -> u64`

- <span id="smallindex-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="smallindex-as-i32"></span>`const fn as_i32(&self) -> i32`

- <span id="smallindex-one-more"></span>`fn one_more(&self) -> usize`

- <span id="smallindex-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

- <span id="smallindex-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex` — [`SmallIndex`](#smallindex)

- <span id="smallindex-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

#### Trait Implementations

##### `impl Clone for SmallIndex`

- <span id="smallindex-clone"></span>`fn clone(&self) -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl Copy for SmallIndex`

##### `impl Debug for SmallIndex`

- <span id="smallindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SmallIndex`

- <span id="smallindex-default"></span>`fn default() -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl Eq for SmallIndex`

##### `impl Hash for SmallIndex`

- <span id="smallindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: SmallIndex) -> &T` — [`SmallIndex`](#smallindex)

##### `impl<T> IndexMut for [T]`

- <span id="t-index-mut"></span>`fn index_mut(&mut self, index: SmallIndex) -> &mut T` — [`SmallIndex`](#smallindex)

##### `impl Ord for SmallIndex`

- <span id="smallindex-cmp"></span>`fn cmp(&self, other: &SmallIndex) -> cmp::Ordering` — [`SmallIndex`](#smallindex)

##### `impl PartialEq for SmallIndex`

- <span id="smallindex-eq"></span>`fn eq(&self, other: &SmallIndex) -> bool` — [`SmallIndex`](#smallindex)

##### `impl PartialOrd for SmallIndex`

- <span id="smallindex-partial-cmp"></span>`fn partial_cmp(&self, other: &SmallIndex) -> option::Option<cmp::Ordering>` — [`SmallIndex`](#smallindex)

##### `impl StructuralPartialEq for SmallIndex`

### `SmallIndexError`

```rust
struct SmallIndexError {
    attempted: u64,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:339-341`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L339-L341)*

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

#### Implementations

- <span id="smallindexerror-attempted"></span>`fn attempted(&self) -> u64`

#### Trait Implementations

##### `impl Clone for SmallIndexError`

- <span id="smallindexerror-clone"></span>`fn clone(&self) -> SmallIndexError` — [`SmallIndexError`](#smallindexerror)

##### `impl Debug for SmallIndexError`

- <span id="smallindexerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SmallIndexError`

- <span id="smallindexerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SmallIndexError`

##### `impl Error for SmallIndexError`

##### `impl PartialEq for SmallIndexError`

- <span id="smallindexerror-eq"></span>`fn eq(&self, other: &SmallIndexError) -> bool` — [`SmallIndexError`](#smallindexerror)

##### `impl StructuralPartialEq for SmallIndexError`

##### `impl ToString for SmallIndexError`

- <span id="smallindexerror-to-string"></span>`fn to_string(&self) -> String`

### `SmallIndexIter`

```rust
struct SmallIndexIter {
    rng: core::ops::Range<usize>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:365-367`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L365-L367)*

#### Trait Implementations

##### `impl Clone for SmallIndexIter`

- <span id="smallindexiter-clone"></span>`fn clone(&self) -> SmallIndexIter` — [`SmallIndexIter`](#smallindexiter)

##### `impl Debug for SmallIndexIter`

- <span id="smallindexiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SmallIndexIter`

- <span id="smallindexiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="smallindexiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="smallindexiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SmallIndexIter`

- <span id="smallindexiter-iterator-type-item"></span>`type Item = SmallIndex`

- <span id="smallindexiter-next"></span>`fn next(&mut self) -> Option<SmallIndex>` — [`SmallIndex`](#smallindex)

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:713`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L713)*

The identifier of a pattern in an Aho-Corasick automaton.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `PatternID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

#### Implementations

- <span id="patternid-const-max"></span>`const MAX: PatternID`

- <span id="patternid-const-limit"></span>`const LIMIT: usize`

- <span id="patternid-const-zero"></span>`const ZERO: PatternID`

- <span id="patternid-const-size"></span>`const SIZE: usize`

- <span id="patternid-new"></span>`fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

- <span id="patternid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](#patternid)

- <span id="patternid-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> PatternID` — [`PatternID`](#patternid)

- <span id="patternid-must"></span>`fn must(value: usize) -> PatternID` — [`PatternID`](#patternid)

- <span id="patternid-as-usize"></span>`const fn as_usize(&self) -> usize`

- <span id="patternid-as-u64"></span>`const fn as_u64(&self) -> u64`

- <span id="patternid-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="patternid-as-i32"></span>`const fn as_i32(&self) -> i32`

- <span id="patternid-one-more"></span>`fn one_more(&self) -> usize`

- <span id="patternid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

- <span id="patternid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](#patternid)

- <span id="patternid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

- <span id="patternid-iter"></span>`fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternID`

- <span id="patternid-clone"></span>`fn clone(&self) -> PatternID` — [`PatternID`](#patternid)

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- <span id="patternid-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- <span id="patternid-default"></span>`fn default() -> PatternID` — [`PatternID`](#patternid)

##### `impl Eq for PatternID`

##### `impl Hash for PatternID`

- <span id="patternid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: PatternID) -> &T` — [`PatternID`](#patternid)

##### `impl<T> IndexMut for [T]`

- <span id="t-index-mut"></span>`fn index_mut(&mut self, index: PatternID) -> &mut T` — [`PatternID`](#patternid)

##### `impl Ord for PatternID`

- <span id="patternid-cmp"></span>`fn cmp(&self, other: &PatternID) -> cmp::Ordering` — [`PatternID`](#patternid)

##### `impl PartialEq for PatternID`

- <span id="patternid-eq"></span>`fn eq(&self, other: &PatternID) -> bool` — [`PatternID`](#patternid)

##### `impl PartialOrd for PatternID`

- <span id="patternid-partial-cmp"></span>`fn partial_cmp(&self, other: &PatternID) -> option::Option<cmp::Ordering>` — [`PatternID`](#patternid)

##### `impl StructuralPartialEq for PatternID`

### `StateID`

```rust
struct StateID(SmallIndex);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:734`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L734)*

The identifier of a finite automaton state.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `StateID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

#### Implementations

- <span id="stateid-const-max"></span>`const MAX: StateID`

- <span id="stateid-const-limit"></span>`const LIMIT: usize`

- <span id="stateid-const-zero"></span>`const ZERO: StateID`

- <span id="stateid-const-size"></span>`const SIZE: usize`

- <span id="stateid-new"></span>`fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

- <span id="stateid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> StateID` — [`StateID`](#stateid)

- <span id="stateid-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> StateID` — [`StateID`](#stateid)

- <span id="stateid-must"></span>`fn must(value: usize) -> StateID` — [`StateID`](#stateid)

- <span id="stateid-as-usize"></span>`const fn as_usize(&self) -> usize`

- <span id="stateid-as-u64"></span>`const fn as_u64(&self) -> u64`

- <span id="stateid-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="stateid-as-i32"></span>`const fn as_i32(&self) -> i32`

- <span id="stateid-one-more"></span>`fn one_more(&self) -> usize`

- <span id="stateid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

- <span id="stateid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID` — [`StateID`](#stateid)

- <span id="stateid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

- <span id="stateid-iter"></span>`fn iter(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

#### Trait Implementations

##### `impl Clone for StateID`

- <span id="stateid-clone"></span>`fn clone(&self) -> StateID` — [`StateID`](#stateid)

##### `impl Copy for StateID`

##### `impl Debug for StateID`

- <span id="stateid-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for StateID`

- <span id="stateid-default"></span>`fn default() -> StateID` — [`StateID`](#stateid)

##### `impl Eq for StateID`

##### `impl Hash for StateID`

- <span id="stateid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: StateID) -> &T` — [`StateID`](#stateid)

##### `impl<T> IndexMut for [T]`

- <span id="t-index-mut"></span>`fn index_mut(&mut self, index: StateID) -> &mut T` — [`StateID`](#stateid)

##### `impl Ord for StateID`

- <span id="stateid-cmp"></span>`fn cmp(&self, other: &StateID) -> cmp::Ordering` — [`StateID`](#stateid)

##### `impl PartialEq for StateID`

- <span id="stateid-eq"></span>`fn eq(&self, other: &StateID) -> bool` — [`StateID`](#stateid)

##### `impl PartialOrd for StateID`

- <span id="stateid-partial-cmp"></span>`fn partial_cmp(&self, other: &StateID) -> option::Option<cmp::Ordering>` — [`StateID`](#stateid)

##### `impl StructuralPartialEq for StateID`

### `PatternIDError`

```rust
struct PatternIDError(SmallIndexError);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:736`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L736)*

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="patterniderror-attempted"></span>`fn attempted(&self) -> u64`

#### Trait Implementations

##### `impl Clone for PatternIDError`

- <span id="patterniderror-clone"></span>`fn clone(&self) -> PatternIDError` — [`PatternIDError`](#patterniderror)

##### `impl Debug for PatternIDError`

- <span id="patterniderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PatternIDError`

- <span id="patterniderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl PartialEq for PatternIDError`

- <span id="patterniderror-eq"></span>`fn eq(&self, other: &PatternIDError) -> bool` — [`PatternIDError`](#patterniderror)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl ToString for PatternIDError`

- <span id="patterniderror-to-string"></span>`fn to_string(&self) -> String`

### `PatternIDIter`

```rust
struct PatternIDIter(SmallIndexIter);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:736`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L736)*

#### Implementations

- <span id="patterniditer-new"></span>`fn new(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternIDIter`

- <span id="patterniditer-clone"></span>`fn clone(&self) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

##### `impl Debug for PatternIDIter`

- <span id="patterniditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PatternIDIter`

- <span id="patterniditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIDIter`

- <span id="patterniditer-iterator-type-item"></span>`type Item = PatternID`

- <span id="patterniditer-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](#patternid)

### `WithPatternIDIter<I>`

```rust
struct WithPatternIDIter<I> {
    it: I,
    ids: PatternIDIter,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:736`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L736)*

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- <span id="withpatterniditer-new"></span>`fn new(it: I) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for WithPatternIDIter<I>`

- <span id="withpatterniditer-clone"></span>`fn clone(&self) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

##### `impl<I: fmt::Debug> Debug for WithPatternIDIter<I>`

- <span id="withpatterniditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for WithPatternIDIter<I>`

- <span id="withpatterniditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withpatterniditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withpatterniditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithPatternIDIter<I>`

- <span id="withpatterniditer-iterator-type-item"></span>`type Item = (PatternID, <I as Iterator>::Item)`

- <span id="withpatterniditer-next"></span>`fn next(&mut self) -> Option<(PatternID, <I as >::Item)>` — [`PatternID`](#patternid)

### `StateIDError`

```rust
struct StateIDError(SmallIndexError);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:737`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L737)*

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="stateiderror-attempted"></span>`fn attempted(&self) -> u64`

#### Trait Implementations

##### `impl Clone for StateIDError`

- <span id="stateiderror-clone"></span>`fn clone(&self) -> StateIDError` — [`StateIDError`](#stateiderror)

##### `impl Debug for StateIDError`

- <span id="stateiderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StateIDError`

- <span id="stateiderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for StateIDError`

##### `impl Error for StateIDError`

##### `impl PartialEq for StateIDError`

- <span id="stateiderror-eq"></span>`fn eq(&self, other: &StateIDError) -> bool` — [`StateIDError`](#stateiderror)

##### `impl StructuralPartialEq for StateIDError`

##### `impl ToString for StateIDError`

- <span id="stateiderror-to-string"></span>`fn to_string(&self) -> String`

### `StateIDIter`

```rust
struct StateIDIter(SmallIndexIter);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:737`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L737)*

#### Implementations

- <span id="stateiditer-new"></span>`fn new(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

#### Trait Implementations

##### `impl Clone for StateIDIter`

- <span id="stateiditer-clone"></span>`fn clone(&self) -> StateIDIter` — [`StateIDIter`](#stateiditer)

##### `impl Debug for StateIDIter`

- <span id="stateiditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for StateIDIter`

- <span id="stateiditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stateiditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stateiditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StateIDIter`

- <span id="stateiditer-iterator-type-item"></span>`type Item = StateID`

- <span id="stateiditer-next"></span>`fn next(&mut self) -> Option<StateID>` — [`StateID`](#stateid)

### `WithStateIDIter<I>`

```rust
struct WithStateIDIter<I> {
    it: I,
    ids: StateIDIter,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:737`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L737)*

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- <span id="withstateiditer-new"></span>`fn new(it: I) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for WithStateIDIter<I>`

- <span id="withstateiditer-clone"></span>`fn clone(&self) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

##### `impl<I: fmt::Debug> Debug for WithStateIDIter<I>`

- <span id="withstateiditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for WithStateIDIter<I>`

- <span id="withstateiditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withstateiditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withstateiditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithStateIDIter<I>`

- <span id="withstateiditer-iterator-type-item"></span>`type Item = (StateID, <I as Iterator>::Item)`

- <span id="withstateiditer-next"></span>`fn next(&mut self) -> Option<(StateID, <I as >::Item)>` — [`StateID`](#stateid)

## Traits

### `IteratorIndexExt`

```rust
trait IteratorIndexExt: Iterator { ... }
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:743-757`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L743-L757)*

A utility trait that defines a couple of adapters for making it convenient
to access indices as "small index" types. We require ExactSizeIterator so
that iterator construction can do a single check to make sure the index of
each element is representable by its small index type.

#### Provided Methods

- `fn with_pattern_ids(self) -> WithPatternIDIter<Self>`

- `fn with_state_ids(self) -> WithStateIDIter<Self>`

#### Implementors

- `I`

## Macros

### `index_type_impls!`

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:384-692`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L384-L692)*

