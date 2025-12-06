*[regex_automata](../../index.md) / [util](../index.md) / [primitives](index.md)*

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
* [`NonMaxUsize`](#nonmaxusize) - Represents a `usize` that cannot be `usize::MAX`. As a
result, `Option<NonMaxUsize>` has the same size in memory as a `usize`. This
useful, for example, when representing the offsets of submatches since it
reduces memory usage by a factor of 2. It is a legal optimization since Rust
guarantees that slices never have a length that exceeds `isize::MAX`.

## Structs

### `NonMaxUsize`

```rust
struct NonMaxUsize(core::num::NonZeroUsize);
```

A `usize` that can never be `usize::MAX`.

This is similar to `core::num::NonZeroUsize`, but instead of not permitting
a zero value, this does not permit a max value.

This is useful in certain contexts where one wants to optimize the memory
usage of things that contain match offsets. Namely, since Rust slices
are guaranteed to never have a length exceeding `isize::MAX`, we can use
`usize::MAX` as a sentinel to indicate that no match was found. Indeed,
types like `Option<NonMaxUsize>` have exactly the same size in memory as a
`usize`.

This type is defined to be `repr(transparent)` for
`core::num::NonZeroUsize`, which is in turn defined to be
`repr(transparent)` for `usize`.

#### Implementations

- `fn new(value: usize) -> Option<NonMaxUsize>` — [`NonMaxUsize`](#nonmaxusize)

- `fn get(self: Self) -> usize`

#### Trait Implementations

##### `impl Clone for NonMaxUsize`

- `fn clone(self: &Self) -> NonMaxUsize` — [`NonMaxUsize`](#nonmaxusize)

##### `impl Copy for NonMaxUsize`

##### `impl Debug for NonMaxUsize`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for NonMaxUsize`

##### `impl Hash for NonMaxUsize`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for NonMaxUsize`

- `fn cmp(self: &Self, other: &NonMaxUsize) -> $crate::cmp::Ordering` — [`NonMaxUsize`](#nonmaxusize)

##### `impl PartialEq for NonMaxUsize`

- `fn eq(self: &Self, other: &NonMaxUsize) -> bool` — [`NonMaxUsize`](#nonmaxusize)

##### `impl PartialOrd for NonMaxUsize`

- `fn partial_cmp(self: &Self, other: &NonMaxUsize) -> $crate::option::Option<$crate::cmp::Ordering>` — [`NonMaxUsize`](#nonmaxusize)

##### `impl StructuralPartialEq for NonMaxUsize`

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

- `const MAX: SmallIndex`

- `const LIMIT: usize`

- `const ZERO: SmallIndex`

- `const SIZE: usize`

- `fn new(index: usize) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

- `const fn new_unchecked(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

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

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

The identifier of a regex pattern, represented by a [`SmallIndex`](#smallindex).

The identifier for a pattern corresponds to its relative position among
other patterns in a single finite state machine. Namely, when building
a multi-pattern regex engine, one must supply a sequence of patterns to
match. The position (starting at 0) of each pattern in that sequence
represents its identifier. This identifier is in turn used to identify and
report matches of that pattern in various APIs.

See the [`SmallIndex`](#smallindex) type for more information about what it means for
a pattern ID to be a "small index."

Note that this type is defined in the
[`util::primitives`](crate::util::primitives) module, but it is also
re-exported at the crate root due to how common it is.

#### Implementations

- `const MAX: PatternID`

- `const LIMIT: usize`

- `const ZERO: PatternID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

- `const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](#patternid)

- `fn must(value: usize) -> PatternID` — [`PatternID`](#patternid)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](#patternid)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternID`

- `fn clone(self: &Self) -> PatternID` — [`PatternID`](#patternid)

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- `fn default() -> PatternID` — [`PatternID`](#patternid)

##### `impl Eq for PatternID`

##### `impl Hash for PatternID`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for PatternID`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering` — [`PatternID`](#patternid)

##### `impl PartialEq for PatternID`

- `fn eq(self: &Self, other: &PatternID) -> bool` — [`PatternID`](#patternid)

##### `impl PartialOrd for PatternID`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`PatternID`](#patternid)

##### `impl StructuralPartialEq for PatternID`

### `StateID`

```rust
struct StateID(SmallIndex);
```

The identifier of a finite automaton state, represented by a
[`SmallIndex`](#smallindex).

Most regex engines in this crate are built on top of finite automata. Each
state in a finite automaton defines transitions from its state to another.
Those transitions point to other states via their identifiers, i.e., a
`StateID`. Since finite automata tend to contain many transitions, it is
much more memory efficient to define state IDs as small indices.

See the [`SmallIndex`](#smallindex) type for more information about what it means for
a state ID to be a "small index."

#### Implementations

- `const MAX: StateID`

- `const LIMIT: usize`

- `const ZERO: StateID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

- `const fn new_unchecked(value: usize) -> StateID` — [`StateID`](#stateid)

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

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`

#### Trait Implementations

##### `impl Clone for PatternIDError`

- `fn clone(self: &Self) -> PatternIDError` — [`PatternIDError`](#patterniderror)

##### `impl Debug for PatternIDError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for PatternIDError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl PartialEq for PatternIDError`

- `fn eq(self: &Self, other: &PatternIDError) -> bool` — [`PatternIDError`](#patterniderror)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl<T> ToString for PatternIDError`

- `fn to_string(self: &Self) -> String`

### `StateIDError`

```rust
struct StateIDError(SmallIndexError);
```

This error occurs when a value could not be constructed.

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

