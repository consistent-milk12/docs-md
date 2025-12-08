*[aho_corasick](../../index.md) / [util](../index.md) / [primitives](index.md)*

---

# Module `primitives`

Lower level primitive types that are useful in a variety of circumstances.

# Overview

This list represents the principle types in this module and briefly describes
when you might want to use them.

* [`PatternID`](../../index.md) - A type that represents the identifier of a regex pattern.
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

## Structs

### `SmallIndex`

```rust
struct SmallIndex(u32);
```

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

* [`PatternID`](../../index.md) is for representing the identifiers of patterns.
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

- `const MAX: SmallIndex`

- `const LIMIT: usize`

- `const ZERO: SmallIndex`

- `const SIZE: usize`

- `fn new(index: usize) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

- `const fn new_unchecked(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

- `const fn from_u32_unchecked(index: u32) -> SmallIndex` — [`SmallIndex`](#smallindex)

- `fn must(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex` — [`SmallIndex`](#smallindex)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

#### Trait Implementations

##### `impl Clone for SmallIndex`

- `fn clone(self: &Self) -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl Copy for SmallIndex`

##### `impl Debug for SmallIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for SmallIndex`

- `fn default() -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl Eq for SmallIndex`

##### `impl Hash for SmallIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for SmallIndex`

- `fn cmp(self: &Self, other: &SmallIndex) -> $crate::cmp::Ordering` — [`SmallIndex`](#smallindex)

##### `impl PartialEq for SmallIndex`

- `fn eq(self: &Self, other: &SmallIndex) -> bool` — [`SmallIndex`](#smallindex)

##### `impl PartialOrd for SmallIndex`

- `fn partial_cmp(self: &Self, other: &SmallIndex) -> $crate::option::Option<$crate::cmp::Ordering>` — [`SmallIndex`](#smallindex)

##### `impl StructuralPartialEq for SmallIndex`

### `SmallIndexError`

```rust
struct SmallIndexError {
    attempted: u64,
}
```

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for SmallIndexError`

- `fn clone(self: &Self) -> SmallIndexError` — [`SmallIndexError`](#smallindexerror)

##### `impl Debug for SmallIndexError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for SmallIndexError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SmallIndexError`

##### `impl Error for SmallIndexError`

##### `impl PartialEq for SmallIndexError`

- `fn eq(self: &Self, other: &SmallIndexError) -> bool` — [`SmallIndexError`](#smallindexerror)

##### `impl StructuralPartialEq for SmallIndexError`

##### `impl<T> ToString for SmallIndexError`

- `fn to_string(self: &Self) -> String`

### `SmallIndexIter`

```rust
struct SmallIndexIter {
    rng: core::ops::Range<usize>,
}
```

#### Trait Implementations

##### `impl Clone for SmallIndexIter`

- `fn clone(self: &Self) -> SmallIndexIter` — [`SmallIndexIter`](#smallindexiter)

##### `impl Debug for SmallIndexIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for SmallIndexIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for SmallIndexIter`

- `type Item = SmallIndex`

- `fn next(self: &mut Self) -> Option<SmallIndex>` — [`SmallIndex`](#smallindex)

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

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

- `const MAX: PatternID`

- `const LIMIT: usize`

- `const ZERO: PatternID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](../../index.md), [`PatternIDError`](../../index.md)

- `const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](../../index.md)

- `const fn from_u32_unchecked(index: u32) -> PatternID` — [`PatternID`](../../index.md)

- `fn must(value: usize) -> PatternID` — [`PatternID`](../../index.md)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](../../index.md), [`PatternIDError`](../../index.md)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](../../index.md)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternID`

- `fn clone(self: &Self) -> PatternID` — [`PatternID`](../../index.md)

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- `fn default() -> PatternID` — [`PatternID`](../../index.md)

##### `impl Eq for PatternID`

##### `impl Hash for PatternID`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for PatternID`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering` — [`PatternID`](../../index.md)

##### `impl PartialEq for PatternID`

- `fn eq(self: &Self, other: &PatternID) -> bool` — [`PatternID`](../../index.md)

##### `impl PartialOrd for PatternID`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`PatternID`](../../index.md)

##### `impl StructuralPartialEq for PatternID`

### `StateID`

```rust
struct StateID(SmallIndex);
```

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

- `const MAX: StateID`

- `const LIMIT: usize`

- `const ZERO: StateID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

- `const fn new_unchecked(value: usize) -> StateID` — [`StateID`](#stateid)

- `const fn from_u32_unchecked(index: u32) -> StateID` — [`StateID`](#stateid)

- `fn must(value: usize) -> StateID` — [`StateID`](#stateid)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID` — [`StateID`](#stateid)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

#### Trait Implementations

##### `impl Clone for StateID`

- `fn clone(self: &Self) -> StateID` — [`StateID`](#stateid)

##### `impl Copy for StateID`

##### `impl Debug for StateID`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for StateID`

- `fn default() -> StateID` — [`StateID`](#stateid)

##### `impl Eq for StateID`

##### `impl Hash for StateID`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for StateID`

- `fn cmp(self: &Self, other: &StateID) -> $crate::cmp::Ordering` — [`StateID`](#stateid)

##### `impl PartialEq for StateID`

- `fn eq(self: &Self, other: &StateID) -> bool` — [`StateID`](#stateid)

##### `impl PartialOrd for StateID`

- `fn partial_cmp(self: &Self, other: &StateID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`StateID`](#stateid)

##### `impl StructuralPartialEq for StateID`

### `PatternIDError`

```rust
struct PatternIDError(SmallIndexError);
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for PatternIDError`

- `fn clone(self: &Self) -> PatternIDError` — [`PatternIDError`](../../index.md)

##### `impl Debug for PatternIDError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for PatternIDError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl PartialEq for PatternIDError`

- `fn eq(self: &Self, other: &PatternIDError) -> bool` — [`PatternIDError`](../../index.md)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl<T> ToString for PatternIDError`

- `fn to_string(self: &Self) -> String`

### `PatternIDIter`

```rust
struct PatternIDIter(SmallIndexIter);
```

#### Implementations

- `fn new(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternIDIter`

- `fn clone(self: &Self) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

##### `impl Debug for PatternIDIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PatternIDIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for PatternIDIter`

- `type Item = PatternID`

- `fn next(self: &mut Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

### `WithPatternIDIter<I>`

```rust
struct WithPatternIDIter<I> {
    it: I,
    ids: PatternIDIter,
}
```

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- `fn new(it: I) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for WithPatternIDIter<I>`

- `fn clone(self: &Self) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

##### `impl<I: $crate::fmt::Debug> Debug for WithPatternIDIter<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for WithPatternIDIter<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithPatternIDIter<I>`

- `type Item = (PatternID, <I as Iterator>::Item)`

- `fn next(self: &mut Self) -> Option<(PatternID, <I as >::Item)>` — [`PatternID`](../../index.md)

### `StateIDError`

```rust
struct StateIDError(SmallIndexError);
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for StateIDError`

- `fn clone(self: &Self) -> StateIDError` — [`StateIDError`](#stateiderror)

##### `impl Debug for StateIDError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for StateIDError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for StateIDError`

##### `impl Error for StateIDError`

##### `impl PartialEq for StateIDError`

- `fn eq(self: &Self, other: &StateIDError) -> bool` — [`StateIDError`](#stateiderror)

##### `impl StructuralPartialEq for StateIDError`

##### `impl<T> ToString for StateIDError`

- `fn to_string(self: &Self) -> String`

### `StateIDIter`

```rust
struct StateIDIter(SmallIndexIter);
```

#### Implementations

- `fn new(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

#### Trait Implementations

##### `impl Clone for StateIDIter`

- `fn clone(self: &Self) -> StateIDIter` — [`StateIDIter`](#stateiditer)

##### `impl Debug for StateIDIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for StateIDIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for StateIDIter`

- `type Item = StateID`

- `fn next(self: &mut Self) -> Option<StateID>` — [`StateID`](#stateid)

### `WithStateIDIter<I>`

```rust
struct WithStateIDIter<I> {
    it: I,
    ids: StateIDIter,
}
```

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- `fn new(it: I) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for WithStateIDIter<I>`

- `fn clone(self: &Self) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

##### `impl<I: $crate::fmt::Debug> Debug for WithStateIDIter<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for WithStateIDIter<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithStateIDIter<I>`

- `type Item = (StateID, <I as Iterator>::Item)`

- `fn next(self: &mut Self) -> Option<(StateID, <I as >::Item)>` — [`StateID`](#stateid)

## Traits

### `IteratorIndexExt`

```rust
trait IteratorIndexExt: Iterator { ... }
```

A utility trait that defines a couple of adapters for making it convenient
to access indices as "small index" types. We require ExactSizeIterator so
that iterator construction can do a single check to make sure the index of
each element is representable by its small index type.

#### Required Methods

- `fn with_pattern_ids(self: Self) -> WithPatternIDIter<Self>`

- `fn with_state_ids(self: Self) -> WithStateIDIter<Self>`

## Macros

### `index_type_impls!`

