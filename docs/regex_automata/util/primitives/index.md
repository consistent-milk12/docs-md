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
struct NonMaxUsize();
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

- `fn new(value: usize) -> Option<NonMaxUsize>`
  Create a new `NonMaxUsize` from the given value.

- `fn get(self: Self) -> usize`
  Return the underlying `usize` value. The returned value is guaranteed

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> NonMaxUsize`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &NonMaxUsize) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &NonMaxUsize) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &NonMaxUsize) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SmallIndex`

```rust
struct SmallIndex();
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

- `fn new(index: usize) -> Result<SmallIndex, SmallIndexError>`
  Create a new small index.

- `const fn new_unchecked(index: usize) -> SmallIndex`
  Create a new small index without checking whether the given value

- `fn must(index: usize) -> SmallIndex`
  Like [`SmallIndex::new`], but panics if the given index is not valid.

- `const fn as_usize(self: &Self) -> usize`
  Return this small index as a `usize`. This is guaranteed to never

- `const fn as_u64(self: &Self) -> u64`
  Return this small index as a `u64`. This is guaranteed to never

- `const fn as_u32(self: &Self) -> u32`
  Return the internal `u32` of this small index. This is guaranteed to

- `const fn as_i32(self: &Self) -> i32`
  Return the internal `u32` of this small index represented as an `i32`.

- `fn one_more(self: &Self) -> usize`
  Returns one more than this small index as a usize.

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>`
  Decode this small index from the bytes given using the native endian

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex`
  Decode this small index from the bytes given using the native endian

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`
  Return the underlying small index integer as raw bytes in native endian

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(index: u8) -> SmallIndex`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> SmallIndex`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &SmallIndex) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SmallIndex) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &SmallIndex) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = SmallIndexError`

- `fn try_from(index: u16) -> Result<SmallIndex, SmallIndexError>`

##### `impl TryFrom`

- `type Error = SmallIndexError`

- `fn try_from(index: u64) -> Result<SmallIndex, SmallIndexError>`

##### `impl TryFrom`

- `type Error = SmallIndexError`

- `fn try_from(index: u32) -> Result<SmallIndex, SmallIndexError>`

##### `impl TryFrom`

- `type Error = SmallIndexError`

- `fn try_from(index: usize) -> Result<SmallIndex, SmallIndexError>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> SmallIndex`

### `SmallIndexError`

```rust
struct SmallIndexError {
    // [REDACTED: Private Fields]
}
```

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`
  Returns the value that could not be converted to a small index.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> SmallIndexError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &SmallIndexError) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PatternID`

```rust
struct PatternID();
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

- `fn new(value: usize) -> Result<PatternID, PatternIDError>`
  Create a new value that is represented by a "small index."

- `const fn new_unchecked(value: usize) -> PatternID`
  Create a new value without checking whether the given argument

- `fn must(value: usize) -> PatternID`
  Like `new`, but panics if the given value is not valid.

- `const fn as_usize(self: &Self) -> usize`
  Return the internal value as a `usize`. This is guaranteed to

- `const fn as_u64(self: &Self) -> u64`
  Return the internal value as a `u64`. This is guaranteed to

- `const fn as_u32(self: &Self) -> u32`
  Return the internal value as a `u32`. This is guaranteed to

- `const fn as_i32(self: &Self) -> i32`
  Return the internal value as a i32`. This is guaranteed to

- `fn one_more(self: &Self) -> usize`
  Returns one more than this value as a usize.

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>`
  Decode this value from the bytes given using the native endian

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID`
  Decode this value from the bytes given using the native endian

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`
  Return the underlying integer as raw bytes in native endian

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: u8) -> PatternID`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PatternID`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternID) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u32) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u16) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u64) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: usize) -> Result<PatternID, PatternIDError>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> PatternID`

### `StateID`

```rust
struct StateID();
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

- `fn new(value: usize) -> Result<StateID, StateIDError>`
  Create a new value that is represented by a "small index."

- `const fn new_unchecked(value: usize) -> StateID`
  Create a new value without checking whether the given argument

- `fn must(value: usize) -> StateID`
  Like `new`, but panics if the given value is not valid.

- `const fn as_usize(self: &Self) -> usize`
  Return the internal value as a `usize`. This is guaranteed to

- `const fn as_u64(self: &Self) -> u64`
  Return the internal value as a `u64`. This is guaranteed to

- `const fn as_u32(self: &Self) -> u32`
  Return the internal value as a `u32`. This is guaranteed to

- `const fn as_i32(self: &Self) -> i32`
  Return the internal value as a i32`. This is guaranteed to

- `fn one_more(self: &Self) -> usize`
  Returns one more than this value as a usize.

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>`
  Decode this value from the bytes given using the native endian

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID`
  Decode this value from the bytes given using the native endian

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`
  Return the underlying integer as raw bytes in native endian

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: u8) -> StateID`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StateID`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &StateID) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateID) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StateID) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u64) -> Result<StateID, StateIDError>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: usize) -> Result<StateID, StateIDError>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u32) -> Result<StateID, StateIDError>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u16) -> Result<StateID, StateIDError>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> StateID`

### `PatternIDError`

```rust
struct PatternIDError();
```

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`
  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PatternIDError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternIDError) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StateIDError`

```rust
struct StateIDError();
```

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`
  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StateIDError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateIDError) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

