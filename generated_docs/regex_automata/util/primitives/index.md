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

## Contents

- [Structs](#structs)
  - [`NonMaxUsize`](#nonmaxusize)
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
| [`NonMaxUsize`](#nonmaxusize) | struct | A `usize` that can never be `usize::MAX`. |
| [`SmallIndex`](#smallindex) | struct | A type that represents a "small" index. |
| [`SmallIndexError`](#smallindexerror) | struct | This error occurs when a small index could not be constructed. |
| [`SmallIndexIter`](#smallindexiter) | struct |  |
| [`PatternID`](#patternid) | struct | The identifier of a regex pattern, represented by a [`SmallIndex`]. |
| [`StateID`](#stateid) | struct | The identifier of a finite automaton state, represented by a [`SmallIndex`]. |
| [`PatternIDError`](#patterniderror) | struct | This error occurs when a value could not be constructed. |
| [`PatternIDIter`](#patterniditer) | struct |  |
| [`WithPatternIDIter`](#withpatterniditer) | struct | An iterator adapter that is like std::iter::Enumerate, but attaches small index values instead. |
| [`StateIDError`](#stateiderror) | struct | This error occurs when a value could not be constructed. |
| [`StateIDIter`](#stateiditer) | struct |  |
| [`WithStateIDIter`](#withstateiditer) | struct | An iterator adapter that is like std::iter::Enumerate, but attaches small index values instead. |
| [`IteratorIndexExt`](#iteratorindexext) | trait | A utility trait that defines a couple of adapters for making it convenient to access indices as "small index" types. |
| [`index_type_impls!`](#index-type-impls) | macro |  |

## Structs

### `NonMaxUsize`

```rust
struct NonMaxUsize(core::num::NonZeroUsize);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:56`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L56)*

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

- <span id="nonmaxusize-new"></span>`fn new(value: usize) -> Option<NonMaxUsize>` — [`NonMaxUsize`](#nonmaxusize)

  Create a new `NonMaxUsize` from the given value.

  

  This returns `None` only when the given value is equal to `usize::MAX`.

- <span id="nonmaxusize-get"></span>`fn get(self) -> usize`

  Return the underlying `usize` value. The returned value is guaranteed

  to not equal `usize::MAX`.

#### Trait Implementations

##### `impl Any for NonMaxUsize`

- <span id="nonmaxusize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NonMaxUsize`

- <span id="nonmaxusize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NonMaxUsize`

- <span id="nonmaxusize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NonMaxUsize`

- <span id="nonmaxusize-clone"></span>`fn clone(&self) -> NonMaxUsize` — [`NonMaxUsize`](#nonmaxusize)

##### `impl CloneToUninit for NonMaxUsize`

- <span id="nonmaxusize-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for NonMaxUsize`

##### `impl Debug for NonMaxUsize`

- <span id="nonmaxusize-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for NonMaxUsize`

##### `impl<T> From for NonMaxUsize`

- <span id="nonmaxusize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for NonMaxUsize`

- <span id="nonmaxusize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for NonMaxUsize`

- <span id="nonmaxusize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for NonMaxUsize`

- <span id="nonmaxusize-ord-cmp"></span>`fn cmp(&self, other: &NonMaxUsize) -> cmp::Ordering` — [`NonMaxUsize`](#nonmaxusize)

##### `impl PartialEq for NonMaxUsize`

- <span id="nonmaxusize-partialeq-eq"></span>`fn eq(&self, other: &NonMaxUsize) -> bool` — [`NonMaxUsize`](#nonmaxusize)

##### `impl PartialOrd for NonMaxUsize`

- <span id="nonmaxusize-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NonMaxUsize) -> option::Option<cmp::Ordering>` — [`NonMaxUsize`](#nonmaxusize)

##### `impl StructuralPartialEq for NonMaxUsize`

##### `impl ToOwned for NonMaxUsize`

- <span id="nonmaxusize-toowned-type-owned"></span>`type Owned = T`

- <span id="nonmaxusize-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nonmaxusize-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NonMaxUsize`

- <span id="nonmaxusize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nonmaxusize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NonMaxUsize`

- <span id="nonmaxusize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nonmaxusize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SmallIndex`

```rust
struct SmallIndex(u32);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:144`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L144)*

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

  Create a new small index.

  

  If the given index exceeds `SmallIndex::MAX`, then this returns

  an error.

- <span id="smallindex-new-unchecked"></span>`const fn new_unchecked(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

  Create a new small index without checking whether the given value

  exceeds `SmallIndex::MAX`.

  

  Using this routine with an invalid index value will result in

  unspecified behavior, but *not* undefined behavior. In particular, an

  invalid index value is likely to cause panics or possibly even silent

  logical errors.

  

  Callers must never rely on a `SmallIndex` to be within a certain range

  for memory safety.

- <span id="smallindex-must"></span>`fn must(index: usize) -> SmallIndex` — [`SmallIndex`](#smallindex)

  Like `SmallIndex::new`, but panics if the given index is not valid.

- <span id="smallindex-as-usize"></span>`const fn as_usize(&self) -> usize`

  Return this small index as a `usize`. This is guaranteed to never

  overflow `usize`.

- <span id="smallindex-as-u64"></span>`const fn as_u64(&self) -> u64`

  Return this small index as a `u64`. This is guaranteed to never

  overflow.

- <span id="smallindex-as-u32"></span>`const fn as_u32(&self) -> u32`

  Return the internal `u32` of this small index. This is guaranteed to

  never overflow `u32`.

- <span id="smallindex-as-i32"></span>`const fn as_i32(&self) -> i32`

  Return the internal `u32` of this small index represented as an `i32`.

  This is guaranteed to never overflow an `i32`.

- <span id="smallindex-one-more"></span>`fn one_more(&self) -> usize`

  Returns one more than this small index as a usize.

  

  Since a small index has constraints on its maximum value, adding `1` to

  it will always fit in a `usize`, `u32` and a `i32`.

- <span id="smallindex-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError>` — [`SmallIndex`](#smallindex), [`SmallIndexError`](#smallindexerror)

  Decode this small index from the bytes given using the native endian

  byte order for the current target.

  

  If the decoded integer is not representable as a small index for the

  current target, then this returns an error.

- <span id="smallindex-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex` — [`SmallIndex`](#smallindex)

  Decode this small index from the bytes given using the native endian

  byte order for the current target.

  

  This is analogous to `SmallIndex::new_unchecked` in that is does not

  check whether the decoded integer is representable as a small index.

- <span id="smallindex-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

  Return the underlying small index integer as raw bytes in native endian

  format.

#### Trait Implementations

##### `impl Any for SmallIndex`

- <span id="smallindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SmallIndex`

- <span id="smallindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SmallIndex`

- <span id="smallindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SmallIndex`

- <span id="smallindex-clone"></span>`fn clone(&self) -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl CloneToUninit for SmallIndex`

- <span id="smallindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SmallIndex`

##### `impl Debug for SmallIndex`

- <span id="smallindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SmallIndex`

- <span id="smallindex-default"></span>`fn default() -> SmallIndex` — [`SmallIndex`](#smallindex)

##### `impl Eq for SmallIndex`

##### `impl<T> From for SmallIndex`

- <span id="smallindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SmallIndex`

- <span id="smallindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: SmallIndex) -> &T` — [`SmallIndex`](#smallindex)

##### `impl<T> IndexMut for [T]`

- <span id="t-indexmut-index-mut"></span>`fn index_mut(&mut self, index: SmallIndex) -> &mut T` — [`SmallIndex`](#smallindex)

##### `impl<U> Into for SmallIndex`

- <span id="smallindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for SmallIndex`

- <span id="smallindex-ord-cmp"></span>`fn cmp(&self, other: &SmallIndex) -> cmp::Ordering` — [`SmallIndex`](#smallindex)

##### `impl PartialEq for SmallIndex`

- <span id="smallindex-partialeq-eq"></span>`fn eq(&self, other: &SmallIndex) -> bool` — [`SmallIndex`](#smallindex)

##### `impl PartialOrd for SmallIndex`

- <span id="smallindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SmallIndex) -> option::Option<cmp::Ordering>` — [`SmallIndex`](#smallindex)

##### `impl StructuralPartialEq for SmallIndex`

##### `impl ToOwned for SmallIndex`

- <span id="smallindex-toowned-type-owned"></span>`type Owned = T`

- <span id="smallindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="smallindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SmallIndex`

- <span id="smallindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="smallindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SmallIndex`

- <span id="smallindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="smallindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SmallIndexError`

```rust
struct SmallIndexError {
    attempted: u64,
}
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:376-378`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L376-L378)*

This error occurs when a small index could not be constructed.

This occurs when given an integer exceeding the maximum small index value.

When the `std` feature is enabled, this implements the `Error` trait.

#### Implementations

- <span id="smallindexerror-attempted"></span>`fn attempted(&self) -> u64`

  Returns the value that could not be converted to a small index.

#### Trait Implementations

##### `impl Any for SmallIndexError`

- <span id="smallindexerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SmallIndexError`

- <span id="smallindexerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SmallIndexError`

- <span id="smallindexerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SmallIndexError`

- <span id="smallindexerror-clone"></span>`fn clone(&self) -> SmallIndexError` — [`SmallIndexError`](#smallindexerror)

##### `impl CloneToUninit for SmallIndexError`

- <span id="smallindexerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SmallIndexError`

- <span id="smallindexerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SmallIndexError`

- <span id="smallindexerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SmallIndexError`

##### `impl Error for SmallIndexError`

##### `impl<T> From for SmallIndexError`

- <span id="smallindexerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SmallIndexError`

- <span id="smallindexerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SmallIndexError`

- <span id="smallindexerror-partialeq-eq"></span>`fn eq(&self, other: &SmallIndexError) -> bool` — [`SmallIndexError`](#smallindexerror)

##### `impl StructuralPartialEq for SmallIndexError`

##### `impl ToOwned for SmallIndexError`

- <span id="smallindexerror-toowned-type-owned"></span>`type Owned = T`

- <span id="smallindexerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="smallindexerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for SmallIndexError`

- <span id="smallindexerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for SmallIndexError`

- <span id="smallindexerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="smallindexerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SmallIndexError`

- <span id="smallindexerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="smallindexerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SmallIndexIter`

```rust
struct SmallIndexIter {
    rng: core::ops::Range<usize>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:402-404`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L402-L404)*

#### Trait Implementations

##### `impl Any for SmallIndexIter`

- <span id="smallindexiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SmallIndexIter`

- <span id="smallindexiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SmallIndexIter`

- <span id="smallindexiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SmallIndexIter`

- <span id="smallindexiter-clone"></span>`fn clone(&self) -> SmallIndexIter` — [`SmallIndexIter`](#smallindexiter)

##### `impl CloneToUninit for SmallIndexIter`

- <span id="smallindexiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SmallIndexIter`

- <span id="smallindexiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SmallIndexIter`

- <span id="smallindexiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SmallIndexIter`

- <span id="smallindexiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SmallIndexIter`

- <span id="smallindexiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="smallindexiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="smallindexiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SmallIndexIter`

- <span id="smallindexiter-iterator-type-item"></span>`type Item = SmallIndex`

- <span id="smallindexiter-iterator-next"></span>`fn next(&mut self) -> Option<SmallIndex>` — [`SmallIndex`](#smallindex)

##### `impl ToOwned for SmallIndexIter`

- <span id="smallindexiter-toowned-type-owned"></span>`type Owned = T`

- <span id="smallindexiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="smallindexiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SmallIndexIter`

- <span id="smallindexiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="smallindexiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SmallIndexIter`

- <span id="smallindexiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="smallindexiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:736`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L736)*

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

- <span id="patternid-const-max"></span>`const MAX: PatternID`

- <span id="patternid-const-limit"></span>`const LIMIT: usize`

- <span id="patternid-const-zero"></span>`const ZERO: PatternID`

- <span id="patternid-const-size"></span>`const SIZE: usize`

- <span id="patternid-new"></span>`fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

  Create a new value that is represented by a "small index."

  

  If the given index exceeds the maximum allowed value, then this

  returns an error.

- <span id="patternid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](#patternid)

  Create a new value without checking whether the given argument

  exceeds the maximum.

  

  Using this routine with an invalid value will result in

  unspecified behavior, but *not* undefined behavior. In

  particular, an invalid ID value is likely to cause panics or

  possibly even silent logical errors.

  

  Callers must never rely on this type to be within a certain

  range for memory safety.

- <span id="patternid-must"></span>`fn must(value: usize) -> PatternID` — [`PatternID`](#patternid)

  Like `new`, but panics if the given value is not valid.

- <span id="patternid-as-usize"></span>`const fn as_usize(&self) -> usize`

  Return the internal value as a `usize`. This is guaranteed to

  never overflow `usize`.

- <span id="patternid-as-u64"></span>`const fn as_u64(&self) -> u64`

  Return the internal value as a `u64`. This is guaranteed to

  never overflow.

- <span id="patternid-as-u32"></span>`const fn as_u32(&self) -> u32`

  Return the internal value as a `u32`. This is guaranteed to

  never overflow `u32`.

- <span id="patternid-as-i32"></span>`const fn as_i32(&self) -> i32`

  Return the internal value as a i32`. This is guaranteed to

  never overflow an `i32`.

- <span id="patternid-one-more"></span>`fn one_more(&self) -> usize`

  Returns one more than this value as a usize.

  

  Since values represented by a "small index" have constraints

  on their maximum value, adding `1` to it will always fit in a

  `usize`, `u32` and a `i32`.

- <span id="patternid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](#patternid), [`PatternIDError`](#patterniderror)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  If the decoded integer is not representable as a small index

  for the current target, then this returns an error.

- <span id="patternid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](#patternid)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  This is analogous to `new_unchecked` in that is does not check

  whether the decoded integer is representable as a small index.

- <span id="patternid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

  Return the underlying integer as raw bytes in native endian

  format.

- <span id="patternid-iter"></span>`fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

  Returns an iterator over all values from 0 up to and not

  including the given length.

  

  If the given length exceeds this type's limit, then this

  panics.

#### Trait Implementations

##### `impl Any for PatternID`

- <span id="patternid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternID`

- <span id="patternid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternID`

- <span id="patternid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternID`

- <span id="patternid-clone"></span>`fn clone(&self) -> PatternID` — [`PatternID`](#patternid)

##### `impl CloneToUninit for PatternID`

- <span id="patternid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- <span id="patternid-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- <span id="patternid-default"></span>`fn default() -> PatternID` — [`PatternID`](#patternid)

##### `impl Eq for PatternID`

##### `impl<T> From for PatternID`

- <span id="patternid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PatternID`

- <span id="patternid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: PatternID) -> &T` — [`PatternID`](#patternid)

##### `impl<T> IndexMut for [T]`

- <span id="t-indexmut-index-mut"></span>`fn index_mut(&mut self, index: PatternID) -> &mut T` — [`PatternID`](#patternid)

##### `impl<U> Into for PatternID`

- <span id="patternid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for PatternID`

- <span id="patternid-ord-cmp"></span>`fn cmp(&self, other: &PatternID) -> cmp::Ordering` — [`PatternID`](#patternid)

##### `impl PartialEq for PatternID`

- <span id="patternid-partialeq-eq"></span>`fn eq(&self, other: &PatternID) -> bool` — [`PatternID`](#patternid)

##### `impl PartialOrd for PatternID`

- <span id="patternid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PatternID) -> option::Option<cmp::Ordering>` — [`PatternID`](#patternid)

##### `impl StructuralPartialEq for PatternID`

##### `impl ToOwned for PatternID`

- <span id="patternid-toowned-type-owned"></span>`type Owned = T`

- <span id="patternid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patternid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PatternID`

- <span id="patternid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patternid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternID`

- <span id="patternid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patternid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StateID`

```rust
struct StateID(SmallIndex);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:751`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L751)*

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

- <span id="stateid-const-max"></span>`const MAX: StateID`

- <span id="stateid-const-limit"></span>`const LIMIT: usize`

- <span id="stateid-const-zero"></span>`const ZERO: StateID`

- <span id="stateid-const-size"></span>`const SIZE: usize`

- <span id="stateid-new"></span>`fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

  Create a new value that is represented by a "small index."

  

  If the given index exceeds the maximum allowed value, then this

  returns an error.

- <span id="stateid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> StateID` — [`StateID`](#stateid)

  Create a new value without checking whether the given argument

  exceeds the maximum.

  

  Using this routine with an invalid value will result in

  unspecified behavior, but *not* undefined behavior. In

  particular, an invalid ID value is likely to cause panics or

  possibly even silent logical errors.

  

  Callers must never rely on this type to be within a certain

  range for memory safety.

- <span id="stateid-must"></span>`fn must(value: usize) -> StateID` — [`StateID`](#stateid)

  Like `new`, but panics if the given value is not valid.

- <span id="stateid-as-usize"></span>`const fn as_usize(&self) -> usize`

  Return the internal value as a `usize`. This is guaranteed to

  never overflow `usize`.

- <span id="stateid-as-u64"></span>`const fn as_u64(&self) -> u64`

  Return the internal value as a `u64`. This is guaranteed to

  never overflow.

- <span id="stateid-as-u32"></span>`const fn as_u32(&self) -> u32`

  Return the internal value as a `u32`. This is guaranteed to

  never overflow `u32`.

- <span id="stateid-as-i32"></span>`const fn as_i32(&self) -> i32`

  Return the internal value as a i32`. This is guaranteed to

  never overflow an `i32`.

- <span id="stateid-one-more"></span>`fn one_more(&self) -> usize`

  Returns one more than this value as a usize.

  

  Since values represented by a "small index" have constraints

  on their maximum value, adding `1` to it will always fit in a

  `usize`, `u32` and a `i32`.

- <span id="stateid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>` — [`StateID`](#stateid), [`StateIDError`](#stateiderror)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  If the decoded integer is not representable as a small index

  for the current target, then this returns an error.

- <span id="stateid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID` — [`StateID`](#stateid)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  This is analogous to `new_unchecked` in that is does not check

  whether the decoded integer is representable as a small index.

- <span id="stateid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

  Return the underlying integer as raw bytes in native endian

  format.

- <span id="stateid-iter"></span>`fn iter(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

  Returns an iterator over all values from 0 up to and not

  including the given length.

  

  If the given length exceeds this type's limit, then this

  panics.

#### Trait Implementations

##### `impl Any for StateID`

- <span id="stateid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateID`

- <span id="stateid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateID`

- <span id="stateid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateID`

- <span id="stateid-clone"></span>`fn clone(&self) -> StateID` — [`StateID`](#stateid)

##### `impl CloneToUninit for StateID`

- <span id="stateid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StateID`

##### `impl Debug for StateID`

- <span id="stateid-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for StateID`

- <span id="stateid-default"></span>`fn default() -> StateID` — [`StateID`](#stateid)

##### `impl Eq for StateID`

##### `impl<T> From for StateID`

- <span id="stateid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for StateID`

- <span id="stateid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: StateID) -> &T` — [`StateID`](#stateid)

##### `impl<T> IndexMut for [T]`

- <span id="t-indexmut-index-mut"></span>`fn index_mut(&mut self, index: StateID) -> &mut T` — [`StateID`](#stateid)

##### `impl<U> Into for StateID`

- <span id="stateid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for StateID`

- <span id="stateid-ord-cmp"></span>`fn cmp(&self, other: &StateID) -> cmp::Ordering` — [`StateID`](#stateid)

##### `impl PartialEq for StateID`

- <span id="stateid-partialeq-eq"></span>`fn eq(&self, other: &StateID) -> bool` — [`StateID`](#stateid)

##### `impl PartialOrd for StateID`

- <span id="stateid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StateID) -> option::Option<cmp::Ordering>` — [`StateID`](#stateid)

##### `impl StructuralPartialEq for StateID`

##### `impl ToOwned for StateID`

- <span id="stateid-toowned-type-owned"></span>`type Owned = T`

- <span id="stateid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stateid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateID`

- <span id="stateid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stateid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateID`

- <span id="stateid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stateid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIDError`

```rust
struct PatternIDError(SmallIndexError);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:753`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L753)*

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="patterniderror-attempted"></span>`fn attempted(&self) -> u64`

  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl Any for PatternIDError`

- <span id="patterniderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIDError`

- <span id="patterniderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIDError`

- <span id="patterniderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternIDError`

- <span id="patterniderror-clone"></span>`fn clone(&self) -> PatternIDError` — [`PatternIDError`](#patterniderror)

##### `impl CloneToUninit for PatternIDError`

- <span id="patterniderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PatternIDError`

- <span id="patterniderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PatternIDError`

- <span id="patterniderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl<T> From for PatternIDError`

- <span id="patterniderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIDError`

- <span id="patterniderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PatternIDError`

- <span id="patterniderror-partialeq-eq"></span>`fn eq(&self, other: &PatternIDError) -> bool` — [`PatternIDError`](#patterniderror)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl ToOwned for PatternIDError`

- <span id="patterniderror-toowned-type-owned"></span>`type Owned = T`

- <span id="patterniderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patterniderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for PatternIDError`

- <span id="patterniderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for PatternIDError`

- <span id="patterniderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIDError`

- <span id="patterniderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIDIter`

```rust
struct PatternIDIter(SmallIndexIter);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:753`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L753)*

#### Implementations

- <span id="patterniditer-new"></span>`fn new(len: usize) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

#### Trait Implementations

##### `impl Any for PatternIDIter`

- <span id="patterniditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIDIter`

- <span id="patterniditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIDIter`

- <span id="patterniditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternIDIter`

- <span id="patterniditer-clone"></span>`fn clone(&self) -> PatternIDIter` — [`PatternIDIter`](#patterniditer)

##### `impl CloneToUninit for PatternIDIter`

- <span id="patterniditer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PatternIDIter`

- <span id="patterniditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PatternIDIter`

- <span id="patterniditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIDIter`

- <span id="patterniditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PatternIDIter`

- <span id="patterniditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIDIter`

- <span id="patterniditer-iterator-type-item"></span>`type Item = PatternID`

- <span id="patterniditer-iterator-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](#patternid)

##### `impl ToOwned for PatternIDIter`

- <span id="patterniditer-toowned-type-owned"></span>`type Owned = T`

- <span id="patterniditer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patterniditer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PatternIDIter`

- <span id="patterniditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIDIter`

- <span id="patterniditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithPatternIDIter<I>`

```rust
struct WithPatternIDIter<I> {
    it: I,
    ids: PatternIDIter,
}
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:753`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L753)*

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- <span id="withpatterniditer-new"></span>`fn new(it: I) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

#### Trait Implementations

##### `impl Any for WithPatternIDIter<I>`

- <span id="withpatterniditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithPatternIDIter<I>`

- <span id="withpatterniditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithPatternIDIter<I>`

- <span id="withpatterniditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for WithPatternIDIter<I>`

- <span id="withpatterniditer-clone"></span>`fn clone(&self) -> WithPatternIDIter<I>` — [`WithPatternIDIter`](#withpatterniditer)

##### `impl CloneToUninit for WithPatternIDIter<I>`

- <span id="withpatterniditer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for WithPatternIDIter<I>`

- <span id="withpatterniditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WithPatternIDIter<I>`

- <span id="withpatterniditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithPatternIDIter<I>`

- <span id="withpatterniditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for WithPatternIDIter<I>`

- <span id="withpatterniditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withpatterniditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withpatterniditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithPatternIDIter<I>`

- <span id="withpatterniditer-iterator-type-item"></span>`type Item = (PatternID, <I as Iterator>::Item)`

- <span id="withpatterniditer-iterator-next"></span>`fn next(&mut self) -> Option<(PatternID, <I as >::Item)>` — [`PatternID`](#patternid)

##### `impl ToOwned for WithPatternIDIter<I>`

- <span id="withpatterniditer-toowned-type-owned"></span>`type Owned = T`

- <span id="withpatterniditer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withpatterniditer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WithPatternIDIter<I>`

- <span id="withpatterniditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withpatterniditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithPatternIDIter<I>`

- <span id="withpatterniditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withpatterniditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StateIDError`

```rust
struct StateIDError(SmallIndexError);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:754`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L754)*

This error occurs when a value could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="stateiderror-attempted"></span>`fn attempted(&self) -> u64`

  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl Any for StateIDError`

- <span id="stateiderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateIDError`

- <span id="stateiderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateIDError`

- <span id="stateiderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateIDError`

- <span id="stateiderror-clone"></span>`fn clone(&self) -> StateIDError` — [`StateIDError`](#stateiderror)

##### `impl CloneToUninit for StateIDError`

- <span id="stateiderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateIDError`

- <span id="stateiderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StateIDError`

- <span id="stateiderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for StateIDError`

##### `impl Error for StateIDError`

##### `impl<T> From for StateIDError`

- <span id="stateiderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateIDError`

- <span id="stateiderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StateIDError`

- <span id="stateiderror-partialeq-eq"></span>`fn eq(&self, other: &StateIDError) -> bool` — [`StateIDError`](#stateiderror)

##### `impl StructuralPartialEq for StateIDError`

##### `impl ToOwned for StateIDError`

- <span id="stateiderror-toowned-type-owned"></span>`type Owned = T`

- <span id="stateiderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stateiderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StateIDError`

- <span id="stateiderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StateIDError`

- <span id="stateiderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stateiderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateIDError`

- <span id="stateiderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stateiderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StateIDIter`

```rust
struct StateIDIter(SmallIndexIter);
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:754`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L754)*

#### Implementations

- <span id="stateiditer-new"></span>`fn new(len: usize) -> StateIDIter` — [`StateIDIter`](#stateiditer)

#### Trait Implementations

##### `impl Any for StateIDIter`

- <span id="stateiditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateIDIter`

- <span id="stateiditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateIDIter`

- <span id="stateiditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateIDIter`

- <span id="stateiditer-clone"></span>`fn clone(&self) -> StateIDIter` — [`StateIDIter`](#stateiditer)

##### `impl CloneToUninit for StateIDIter`

- <span id="stateiditer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateIDIter`

- <span id="stateiditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StateIDIter`

- <span id="stateiditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateIDIter`

- <span id="stateiditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StateIDIter`

- <span id="stateiditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stateiditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stateiditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StateIDIter`

- <span id="stateiditer-iterator-type-item"></span>`type Item = StateID`

- <span id="stateiditer-iterator-next"></span>`fn next(&mut self) -> Option<StateID>` — [`StateID`](#stateid)

##### `impl ToOwned for StateIDIter`

- <span id="stateiditer-toowned-type-owned"></span>`type Owned = T`

- <span id="stateiditer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stateiditer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateIDIter`

- <span id="stateiditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stateiditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateIDIter`

- <span id="stateiditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stateiditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithStateIDIter<I>`

```rust
struct WithStateIDIter<I> {
    it: I,
    ids: StateIDIter,
}
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:754`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L754)*

An iterator adapter that is like std::iter::Enumerate, but attaches
small index values instead. It requires `ExactSizeIterator`. At
construction, it ensures that the index of each element in the
iterator is representable in the corresponding small index type.

#### Implementations

- <span id="withstateiditer-new"></span>`fn new(it: I) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

#### Trait Implementations

##### `impl Any for WithStateIDIter<I>`

- <span id="withstateiditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithStateIDIter<I>`

- <span id="withstateiditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithStateIDIter<I>`

- <span id="withstateiditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for WithStateIDIter<I>`

- <span id="withstateiditer-clone"></span>`fn clone(&self) -> WithStateIDIter<I>` — [`WithStateIDIter`](#withstateiditer)

##### `impl CloneToUninit for WithStateIDIter<I>`

- <span id="withstateiditer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for WithStateIDIter<I>`

- <span id="withstateiditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WithStateIDIter<I>`

- <span id="withstateiditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithStateIDIter<I>`

- <span id="withstateiditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for WithStateIDIter<I>`

- <span id="withstateiditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withstateiditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withstateiditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator + ExactSizeIterator> Iterator for WithStateIDIter<I>`

- <span id="withstateiditer-iterator-type-item"></span>`type Item = (StateID, <I as Iterator>::Item)`

- <span id="withstateiditer-iterator-next"></span>`fn next(&mut self) -> Option<(StateID, <I as >::Item)>` — [`StateID`](#stateid)

##### `impl ToOwned for WithStateIDIter<I>`

- <span id="withstateiditer-toowned-type-owned"></span>`type Owned = T`

- <span id="withstateiditer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withstateiditer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WithStateIDIter<I>`

- <span id="withstateiditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withstateiditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithStateIDIter<I>`

- <span id="withstateiditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withstateiditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IteratorIndexExt`

```rust
trait IteratorIndexExt: Iterator { ... }
```

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:760-774`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L760-L774)*

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

*Defined in [`regex-automata-0.4.13/src/util/primitives.rs:421-717`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/primitives.rs#L421-L717)*

