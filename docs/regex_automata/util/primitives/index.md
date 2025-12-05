*[regex_automata](../../index.md) / [util](../index.md) / [primitives](index.md)*

---

# Module `primitives`

Lower level primitive types that are useful in a variety of circumstances.

# Overview

This list represents the principle types in this module and briefly describes
when you might want to use them.

* [`PatternID`](../../index.md) - A type that represents the identifier of a regex pattern.
This is probably the most widely used type in this module (which is why it's
also re-exported in the crate root).
* [`StateID`](../../index.md) - A type the represents the identifier of a finite automaton
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

- `fn new(value: usize) -> Option<NonMaxUsize>` — [`NonMaxUsize`](../../../util/primitives/index.md)

- `fn get(self: Self) -> usize`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> NonMaxUsize` — [`NonMaxUsize`](../../../util/primitives/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &NonMaxUsize) -> $crate::cmp::Ordering` — [`NonMaxUsize`](../../../util/primitives/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &NonMaxUsize) -> bool` — [`NonMaxUsize`](../../../util/primitives/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &NonMaxUsize) -> $crate::option::Option<$crate::cmp::Ordering>` — [`NonMaxUsize`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

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
* [`StateID`](../../index.md) is for representing the identifiers of states in finite
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

- `fn new(index: usize) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](../../../util/primitives/index.md), [`SmallIndexError`](../../../util/primitives/index.md)

- `const fn new_unchecked(index: usize) -> SmallIndex` — [`SmallIndex`](../../../util/primitives/index.md)

- `fn must(index: usize) -> SmallIndex` — [`SmallIndex`](../../../util/primitives/index.md)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](../../../util/primitives/index.md), [`SmallIndexError`](../../../util/primitives/index.md)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex` — [`SmallIndex`](../../../util/primitives/index.md)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> SmallIndex` — [`SmallIndex`](../../../util/primitives/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> SmallIndex` — [`SmallIndex`](../../../util/primitives/index.md)

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &SmallIndex) -> $crate::cmp::Ordering` — [`SmallIndex`](../../../util/primitives/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SmallIndex) -> bool` — [`SmallIndex`](../../../util/primitives/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &SmallIndex) -> $crate::option::Option<$crate::cmp::Ordering>` — [`SmallIndex`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> SmallIndexError` — [`SmallIndexError`](../../../util/primitives/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SmallIndexError) -> bool` — [`SmallIndexError`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

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

- `fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](../../../util/primitives/index.md), [`PatternIDError`](../../../util/primitives/index.md)

- `const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](../../../util/primitives/index.md)

- `fn must(value: usize) -> PatternID` — [`PatternID`](../../../util/primitives/index.md)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](../../../util/primitives/index.md), [`PatternIDError`](../../../util/primitives/index.md)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](../../../util/primitives/index.md)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> PatternID` — [`PatternID`](../../../util/primitives/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> PatternID` — [`PatternID`](../../../util/primitives/index.md)

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering` — [`PatternID`](../../../util/primitives/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternID) -> bool` — [`PatternID`](../../../util/primitives/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`PatternID`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

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

- `fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](../../../util/primitives/index.md), [`StateIDError`](../../../util/primitives/index.md)

- `const fn new_unchecked(value: usize) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn must(value: usize) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `const fn as_usize(self: &Self) -> usize`

- `const fn as_u64(self: &Self) -> u64`

- `const fn as_u32(self: &Self) -> u32`

- `const fn as_i32(self: &Self) -> i32`

- `fn one_more(self: &Self) -> usize`

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>` — [`StateID`](../../../util/primitives/index.md), [`StateIDError`](../../../util/primitives/index.md)

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`

- `fn iter(len: usize) -> StateIDIter` — [`StateIDIter`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StateID` — [`StateID`](../../../util/primitives/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> StateID` — [`StateID`](../../../util/primitives/index.md)

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &StateID) -> $crate::cmp::Ordering` — [`StateID`](../../../util/primitives/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateID) -> bool` — [`StateID`](../../../util/primitives/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StateID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`StateID`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

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

##### `impl Clone`

- `fn clone(self: &Self) -> PatternIDError` — [`PatternIDError`](../../../util/primitives/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternIDError) -> bool` — [`PatternIDError`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

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

##### `impl Clone`

- `fn clone(self: &Self) -> StateIDError` — [`StateIDError`](../../../util/primitives/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateIDError) -> bool` — [`StateIDError`](../../../util/primitives/index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

