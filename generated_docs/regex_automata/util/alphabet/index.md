*[regex_automata](../../index.md) / [util](../index.md) / [alphabet](index.md)*

---

# Module `alphabet`

This module provides APIs for dealing with the alphabets of finite state
machines.

There are two principal types in this module, [`ByteClasses`](#byteclasses) and [`Unit`](#unit).
The former defines the alphabet of a finite state machine while the latter
represents an element of that alphabet.

To a first approximation, the alphabet of all automata in this crate is just
a `u8`. Namely, every distinct byte value. All 256 of them. In practice, this
can be quite wasteful when building a transition table for a DFA, since it
requires storing a state identifier for each element in the alphabet. Instead,
we collapse the alphabet of an automaton down into equivalence classes, where
every byte in the same equivalence class never discriminates between a match or
a non-match from any other byte in the same class. For example, in the regex
`[a-z]+`, then you could consider it having an alphabet consisting of two
equivalence classes: `a-z` and everything else. In terms of the transitions on
an automaton, it doesn't actually require representing every distinct byte.
Just the equivalence classes.

The downside of equivalence classes is that, of course, searching a haystack
deals with individual byte values. Those byte values need to be mapped to
their corresponding equivalence class. This is what `ByteClasses` does. In
practice, doing this for every state transition has negligible impact on modern
CPUs. Moreover, it helps make more efficient use of the CPU cache by (possibly
considerably) shrinking the size of the transition table.

One last hiccup concerns `Unit`. Namely, because of look-around and how the
DFAs in this crate work, we need to add a sentinel value to our alphabet
of equivalence classes that represents the "end" of a search. We call that
sentinel `Unit::eoi` or "end of input." Thus, a `Unit` is either an
equivalence class corresponding to a set of bytes, or it is a special "end of
input" sentinel.

In general, you should not expect to need either of these types unless you're
doing lower level shenanigans with DFAs, or even building your own DFAs.
(Although, you don't have to use these types to build your own DFAs of course.)
For example, if you're walking a DFA's state graph, it's probably useful to
make use of [`ByteClasses`](#byteclasses) to visit each element in the DFA's alphabet instead
of just visiting every distinct `u8` value. The latter isn't necessarily wrong,
but it could be potentially very wasteful.

## Contents

- [Structs](#structs)
  - [`Unit`](#unit)
  - [`ByteClasses`](#byteclasses)
  - [`ByteClassIter`](#byteclassiter)
  - [`ByteClassRepresentatives`](#byteclassrepresentatives)
  - [`ByteClassElements`](#byteclasselements)
  - [`ByteClassElementRanges`](#byteclasselementranges)
  - [`ByteClassSet`](#byteclassset)
  - [`ByteSet`](#byteset)
  - [`BitSet`](#bitset)
  - [`ByteSetIter`](#bytesetiter)
  - [`ByteSetRangeIter`](#bytesetrangeiter)
- [Enums](#enums)
  - [`UnitKind`](#unitkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Unit`](#unit) | struct | Unit represents a single unit of haystack for DFA based regex engines. |
| [`ByteClasses`](#byteclasses) | struct | A representation of byte oriented equivalence classes. |
| [`ByteClassIter`](#byteclassiter) | struct | An iterator over each equivalence class. |
| [`ByteClassRepresentatives`](#byteclassrepresentatives) | struct | An iterator over representative bytes from each equivalence class. |
| [`ByteClassElements`](#byteclasselements) | struct | An iterator over all elements in an equivalence class. |
| [`ByteClassElementRanges`](#byteclasselementranges) | struct | An iterator over all elements in an equivalence class expressed as a sequence of contiguous ranges. |
| [`ByteClassSet`](#byteclassset) | struct | A partitioning of bytes into equivalence classes. |
| [`ByteSet`](#byteset) | struct | A simple set of bytes that is reasonably cheap to copy and allocation free. |
| [`BitSet`](#bitset) | struct | The representation of a byte set. |
| [`ByteSetIter`](#bytesetiter) | struct |  |
| [`ByteSetRangeIter`](#bytesetrangeiter) | struct |  |
| [`UnitKind`](#unitkind) | enum |  |

## Structs

### `Unit`

```rust
struct Unit(UnitKind);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:79`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L79)*

Unit represents a single unit of haystack for DFA based regex engines.

It is not expected for consumers of this crate to need to use this type
unless they are implementing their own DFA. And even then, it's not
required: implementors may use other techniques to handle haystack units.

Typically, a single unit of haystack for a DFA would be a single byte.
However, for the DFAs in this crate, matches are delayed by a single byte
in order to handle look-ahead assertions (`\b`, `$` and `\z`). Thus, once
we have consumed the haystack, we must run the DFA through one additional
transition using a unit that indicates the haystack has ended.

There is no way to represent a sentinel with a `u8` since all possible
values *may* be valid haystack units to a DFA, therefore this type
explicitly adds room for a sentinel value.

The sentinel EOI value is always its own equivalence class and is
ultimately represented by adding 1 to the maximum equivalence class value.
So for example, the regex `^[a-z]+$` might be split into the following
equivalence classes:

```text
0 => [\x00-`]
1 => [a-z]
2 => [{-\xFF]
3 => [EOI]
```

Where EOI is the special sentinel value that is always in its own
singleton equivalence class.

#### Implementations

- <span id="unit-u8"></span>`fn u8(byte: u8) -> Unit` — [`Unit`](#unit)

  Create a new haystack unit from a byte value.

  

  All possible byte values are legal. However, when creating a haystack

  unit for a specific DFA, one should be careful to only construct units

  that are in that DFA's alphabet. Namely, one way to compact a DFA's

  in-memory representation is to collapse its transitions to a set of

  equivalence classes into a set of all possible byte values. If a DFA

  uses equivalence classes instead of byte values, then the byte given

  here should be the equivalence class.

- <span id="unit-eoi"></span>`fn eoi(num_byte_equiv_classes: usize) -> Unit` — [`Unit`](#unit)

  Create a new "end of input" haystack unit.

  

  The value given is the sentinel value used by this unit to represent

  the "end of input." The value should be the total number of equivalence

  classes in the corresponding alphabet. Its maximum value is `256`,

  which occurs when every byte is its own equivalence class.

  

  # Panics

  

  This panics when `num_byte_equiv_classes` is greater than `256`.

- <span id="unit-as-u8"></span>`fn as_u8(self) -> Option<u8>`

  If this unit is not an "end of input" sentinel, then returns its

  underlying byte value. Otherwise return `None`.

- <span id="unit-as-eoi"></span>`fn as_eoi(self) -> Option<u16>`

  If this unit is an "end of input" sentinel, then return the underlying

  sentinel value that was given to `Unit::eoi`. Otherwise return

  `None`.

- <span id="unit-as-usize"></span>`fn as_usize(self) -> usize`

  Return this unit as a `usize`, regardless of whether it is a byte value

  or an "end of input" sentinel. In the latter case, the underlying

  sentinel value given to `Unit::eoi` is returned.

- <span id="unit-is-byte"></span>`fn is_byte(self, byte: u8) -> bool`

  Returns true if and only of this unit is a byte value equivalent to the

  byte given. This always returns false when this is an "end of input"

  sentinel.

- <span id="unit-is-eoi"></span>`fn is_eoi(self) -> bool`

  Returns true when this unit represents an "end of input" sentinel.

- <span id="unit-is-word-byte"></span>`fn is_word_byte(self) -> bool`

  Returns true when this unit corresponds to an ASCII word byte.

  

  This always returns false when this unit represents an "end of input"

  sentinel.

#### Trait Implementations

##### `impl Any for Unit`

- <span id="unit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unit`

- <span id="unit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unit`

- <span id="unit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Unit`

- <span id="unit-clone"></span>`fn clone(&self) -> Unit` — [`Unit`](#unit)

##### `impl CloneToUninit for Unit`

- <span id="unit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Unit`

##### `impl Debug for Unit`

- <span id="unit-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Unit`

##### `impl<T> From for Unit`

- <span id="unit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unit`

- <span id="unit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Unit`

- <span id="unit-ord-cmp"></span>`fn cmp(&self, other: &Unit) -> cmp::Ordering` — [`Unit`](#unit)

##### `impl PartialEq for Unit`

- <span id="unit-partialeq-eq"></span>`fn eq(&self, other: &Unit) -> bool` — [`Unit`](#unit)

##### `impl PartialOrd for Unit`

- <span id="unit-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Unit) -> option::Option<cmp::Ordering>` — [`Unit`](#unit)

##### `impl StructuralPartialEq for Unit`

##### `impl ToOwned for Unit`

- <span id="unit-toowned-type-owned"></span>`type Owned = T`

- <span id="unit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unit`

- <span id="unit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unit`

- <span id="unit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClasses`

```rust
struct ByteClasses([u8; 256]);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:215`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L215)*

A representation of byte oriented equivalence classes.

This is used in a DFA to reduce the size of the transition table. This can
have a particularly large impact not only on the total size of a dense DFA,
but also on compile times.

The essential idea here is that the alphabet of a DFA is shrunk from the
usual 256 distinct byte values down to a set of equivalence classes. The
guarantee you get is that any byte belonging to the same equivalence class
can be treated as if it were any other byte in the same class, and the
result of a search wouldn't change.

# Example

This example shows how to get byte classes from an
[`NFA`](crate::nfa::thompson::NFA) and ask for the class of various bytes.

```rust
use regex_automata::nfa::thompson::NFA;

let nfa = NFA::new("[a-z]+")?;
let classes = nfa.byte_classes();
// 'a' and 'z' are in the same class for this regex.
assert_eq!(classes.get(b'a'), classes.get(b'z'));
// But 'a' and 'A' are not.
assert_ne!(classes.get(b'a'), classes.get(b'A'));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="byteclasses-empty"></span>`fn empty() -> ByteClasses` — [`ByteClasses`](#byteclasses)

  Creates a new set of equivalence classes where all bytes are mapped to

  the same class.

- <span id="byteclasses-singletons"></span>`fn singletons() -> ByteClasses` — [`ByteClasses`](#byteclasses)

  Creates a new set of equivalence classes where each byte belongs to

  its own equivalence class.

- <span id="byteclasses-from-bytes"></span>`fn from_bytes(slice: &[u8]) -> Result<(ByteClasses, usize), DeserializeError>` — [`ByteClasses`](#byteclasses), [`DeserializeError`](../wire/index.md#deserializeerror)

  Deserializes a byte class map from the given slice. If the slice is of

  insufficient length or otherwise contains an impossible mapping, then

  an error is returned. Upon success, the number of bytes read along with

  the map are returned. The number of bytes read is always a multiple of

  8.

- <span id="byteclasses-write-to"></span>`fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md#serializeerror)

  Writes this byte class map to the given byte buffer. if the given

  buffer is too small, then an error is returned. Upon success, the total

  number of bytes written is returned. The number of bytes written is

  guaranteed to be a multiple of 8.

- <span id="byteclasses-write-to-len"></span>`fn write_to_len(&self) -> usize`

  Returns the total number of bytes written by `write_to`.

- <span id="byteclasses-set"></span>`fn set(&mut self, byte: u8, class: u8)`

  Set the equivalence class for the given byte.

- <span id="byteclasses-get"></span>`fn get(&self, byte: u8) -> u8`

  Get the equivalence class for the given byte.

- <span id="byteclasses-get-by-unit"></span>`fn get_by_unit(&self, unit: Unit) -> usize` — [`Unit`](#unit)

  Get the equivalence class for the given haystack unit and return the

  class as a `usize`.

- <span id="byteclasses-eoi"></span>`fn eoi(&self) -> Unit` — [`Unit`](#unit)

  Create a unit that represents the "end of input" sentinel based on the

  number of equivalence classes.

- <span id="byteclasses-alphabet-len"></span>`fn alphabet_len(&self) -> usize`

  Return the total number of elements in the alphabet represented by

  these equivalence classes. Equivalently, this returns the total number

  of equivalence classes.

- <span id="byteclasses-stride2"></span>`fn stride2(&self) -> usize`

  Returns the stride, as a base-2 exponent, required for these

  equivalence classes.

  

  The stride is always the smallest power of 2 that is greater than or

  equal to the alphabet length, and the `stride2` returned here is the

  exponent applied to `2` to get the smallest power. This is done so that

  converting between premultiplied state IDs and indices can be done with

  shifts alone, which is much faster than integer division.

- <span id="byteclasses-is-singleton"></span>`fn is_singleton(&self) -> bool`

  Returns true if and only if every byte in this class maps to its own

  equivalence class. Equivalently, there are 257 equivalence classes

  and each class contains either exactly one byte or corresponds to the

  singleton class containing the "end of input" sentinel.

- <span id="byteclasses-iter"></span>`fn iter(&self) -> ByteClassIter<'_>` — [`ByteClassIter`](#byteclassiter)

  Returns an iterator over all equivalence classes in this set.

- <span id="byteclasses-representatives"></span>`fn representatives<R: core::ops::RangeBounds<u8>>(&self, range: R) -> ByteClassRepresentatives<'_>` — [`ByteClassRepresentatives`](#byteclassrepresentatives)

  Returns an iterator over a sequence of representative bytes from each

  equivalence class within the range of bytes given.

  

  When the given range is unbounded on both sides, the iterator yields

  exactly N items, where N is equivalent to the number of equivalence

  classes. Each item is an arbitrary byte drawn from each equivalence

  class.

  

  This is useful when one is determinizing an NFA and the NFA's alphabet

  hasn't been converted to equivalence classes. Picking an arbitrary byte

  from each equivalence class then permits a full exploration of the NFA

  instead of using every possible byte value and thus potentially saves

  quite a lot of redundant work.

  

  # Example

  

  This shows an example of what a complete sequence of representatives

  might look like from a real example.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};

  

  let nfa = NFA::new("[a-z]+")?;

  let classes = nfa.byte_classes();

  let reps: Vec<Unit> = classes.representatives(..).collect();

  // Note that the specific byte values yielded are not guaranteed!

  let expected = vec![

      Unit::u8(b'\x00'),

      Unit::u8(b'a'),

      Unit::u8(b'{'),

      Unit::eoi(3),

  ];

  assert_eq!(expected, reps);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Note though, that you can ask for an arbitrary range of bytes, and only

  representatives for that range will be returned:

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};

  

  let nfa = NFA::new("[a-z]+")?;

  let classes = nfa.byte_classes();

  let reps: Vec<Unit> = classes.representatives(b'A'..=b'z').collect();

  // Note that the specific byte values yielded are not guaranteed!

  let expected = vec![

      Unit::u8(b'A'),

      Unit::u8(b'a'),

  ];

  assert_eq!(expected, reps);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="byteclasses-elements"></span>`fn elements(&self, class: Unit) -> ByteClassElements<'_>` — [`Unit`](#unit), [`ByteClassElements`](#byteclasselements)

  Returns an iterator of the bytes in the given equivalence class.

  

  This is useful when one needs to know the actual bytes that belong to

  an equivalence class. For example, conceptually speaking, accelerating

  a DFA state occurs when a state only has a few outgoing transitions.

  But in reality, what is required is that there are only a small

  number of distinct bytes that can lead to an outgoing transition. The

  difference is that any one transition can correspond to an equivalence

  class which may contains many bytes. Therefore, DFA state acceleration

  considers the actual elements in each equivalence class of each

  outgoing transition.

  

  # Example

  

  This shows an example of how to get all of the elements in an

  equivalence class.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::alphabet::Unit};

  

  let nfa = NFA::new("[a-z]+")?;

  let classes = nfa.byte_classes();

  let elements: Vec<Unit> = classes.elements(Unit::u8(1)).collect();

  let expected: Vec<Unit> = (b'a'..=b'z').map(Unit::u8).collect();

  assert_eq!(expected, elements);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="byteclasses-element-ranges"></span>`fn element_ranges(&self, class: Unit) -> ByteClassElementRanges<'_>` — [`Unit`](#unit), [`ByteClassElementRanges`](#byteclasselementranges)

  Returns an iterator of byte ranges in the given equivalence class.

  

  That is, a sequence of contiguous ranges are returned. Typically, every

  class maps to a single contiguous range.

#### Trait Implementations

##### `impl Any for ByteClasses`

- <span id="byteclasses-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClasses`

- <span id="byteclasses-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClasses`

- <span id="byteclasses-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteClasses`

- <span id="byteclasses-clone"></span>`fn clone(&self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

##### `impl CloneToUninit for ByteClasses`

- <span id="byteclasses-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ByteClasses`

##### `impl Debug for ByteClasses`

- <span id="byteclasses-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ByteClasses`

- <span id="byteclasses-default"></span>`fn default() -> ByteClasses` — [`ByteClasses`](#byteclasses)

##### `impl<T> From for ByteClasses`

- <span id="byteclasses-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClasses`

- <span id="byteclasses-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ByteClasses`

- <span id="byteclasses-toowned-type-owned"></span>`type Owned = T`

- <span id="byteclasses-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byteclasses-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteClasses`

- <span id="byteclasses-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclasses-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClasses`

- <span id="byteclasses-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclasses-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassIter<'a>`

```rust
struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:525-528`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L525-L528)*

An iterator over each equivalence class.

The last element in this iterator always corresponds to `Unit::eoi`.

This is created by the `ByteClasses::iter` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Any for ByteClassIter<'a>`

- <span id="byteclassiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassIter<'a>`

- <span id="byteclassiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassIter<'a>`

- <span id="byteclassiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteClassIter<'a>`

- <span id="byteclassiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteClassIter<'a>`

- <span id="byteclassiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassIter<'a>`

- <span id="byteclassiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteClassIter<'a>`

- <span id="byteclassiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassIter<'a>`

- <span id="byteclassiter-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclassiter-iterator-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

##### `impl<U> TryFrom for ByteClassIter<'a>`

- <span id="byteclassiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclassiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassIter<'a>`

- <span id="byteclassiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclassiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassRepresentatives<'a>`

```rust
struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    cur_byte: usize,
    end_byte: Option<usize>,
    last_class: Option<u8>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:554-559`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L554-L559)*

An iterator over representative bytes from each equivalence class.

This is created by the `ByteClasses::representatives` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Any for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassrepresentatives-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassrepresentatives-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclassrepresentatives-iterator-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

##### `impl<U> TryFrom for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclassrepresentatives-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclassrepresentatives-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:599-603`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L599-L603)*

An iterator over all elements in an equivalence class.

This is created by the `ByteClasses::elements` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Any for ByteClassElements<'a>`

- <span id="byteclasselements-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassElements<'a>`

- <span id="byteclasselements-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassElements<'a>`

- <span id="byteclasselements-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteClassElements<'a>`

- <span id="byteclasselements-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteClassElements<'a>`

- <span id="byteclasselements-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassElements<'a>`

- <span id="byteclasselements-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteClassElements<'a>`

- <span id="byteclasselements-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclasselements-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclasselements-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassElements<'a>`

- <span id="byteclasselements-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclasselements-iterator-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

##### `impl<U> TryFrom for ByteClassElements<'a>`

- <span id="byteclasselements-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclasselements-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassElements<'a>`

- <span id="byteclasselements-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclasselements-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassElementRanges<'a>`

```rust
struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(Unit, Unit)>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:629-632`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L629-L632)*

An iterator over all elements in an equivalence class expressed as a
sequence of contiguous ranges.

#### Trait Implementations

##### `impl Any for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclasselementranges-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclasselementranges-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-iterator-type-item"></span>`type Item = (Unit, Unit)`

- <span id="byteclasselementranges-iterator-next"></span>`fn next(&mut self) -> Option<(Unit, Unit)>` — [`Unit`](#unit)

##### `impl<U> TryFrom for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclasselementranges-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclasselementranges-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassSet`

```rust
struct ByteClassSet(ByteSet);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:685`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L685)*

A partitioning of bytes into equivalence classes.

A byte class set keeps track of an *approximation* of equivalence classes
of bytes during NFA construction. That is, every byte in an equivalence
class cannot discriminate between a match and a non-match.

For example, in the regex `[ab]+`, the bytes `a` and `b` would be in the
same equivalence class because it never matters whether an `a` or a `b` is
seen, and no combination of `a`s and `b`s in the text can discriminate a
match.

Note though that this does not compute the minimal set of equivalence
classes. For example, in the regex `[ac]+`, both `a` and `c` are in the
same equivalence class for the same reason that `a` and `b` are in the
same equivalence class in the aforementioned regex. However, in this
implementation, `a` and `c` are put into distinct equivalence classes. The
reason for this is implementation complexity. In the future, we should
endeavor to compute the minimal equivalence classes since they can have a
rather large impact on the size of the DFA. (Doing this will likely require
rethinking how equivalence classes are computed, including changing the
representation here, which is only able to group contiguous bytes into the
same equivalence class.)

#### Implementations

- <span id="byteclassset-empty"></span>`fn empty() -> Self`

  Create a new set of byte classes where all bytes are part of the same

  equivalence class.

- <span id="byteclassset-set-range"></span>`fn set_range(&mut self, start: u8, end: u8)`

  Indicate the range of byte given (inclusive) can discriminate a

  match between it and all other bytes outside of the range.

- <span id="byteclassset-add-set"></span>`fn add_set(&mut self, set: &ByteSet)` — [`ByteSet`](#byteset)

  Add the contiguous ranges in the set given to this byte class set.

- <span id="byteclassset-byte-classes"></span>`fn byte_classes(&self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

  Convert this boolean set to a map that maps all byte values to their

  corresponding equivalence class. The last mapping indicates the largest

  equivalence class identifier (which is never bigger than 255).

#### Trait Implementations

##### `impl Any for ByteClassSet`

- <span id="byteclassset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassSet`

- <span id="byteclassset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassSet`

- <span id="byteclassset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteClassSet`

- <span id="byteclassset-clone"></span>`fn clone(&self) -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

##### `impl CloneToUninit for ByteClassSet`

- <span id="byteclassset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ByteClassSet`

- <span id="byteclassset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteClassSet`

- <span id="byteclassset-default"></span>`fn default() -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

##### `impl<T> From for ByteClassSet`

- <span id="byteclassset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassSet`

- <span id="byteclassset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ByteClassSet`

- <span id="byteclassset-toowned-type-owned"></span>`type Owned = T`

- <span id="byteclassset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byteclassset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteClassSet`

- <span id="byteclassset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclassset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassSet`

- <span id="byteclassset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclassset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteSet`

```rust
struct ByteSet {
    bits: BitSet,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:742-744`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L742-L744)*

A simple set of bytes that is reasonably cheap to copy and allocation free.

#### Implementations

- <span id="byteset-empty"></span>`fn empty() -> ByteSet` — [`ByteSet`](#byteset)

  Create an empty set of bytes.

- <span id="byteset-add"></span>`fn add(&mut self, byte: u8)`

  Add a byte to this set.

  

  If the given byte already belongs to this set, then this is a no-op.

- <span id="byteset-remove"></span>`fn remove(&mut self, byte: u8)`

  Remove a byte from this set.

  

  If the given byte is not in this set, then this is a no-op.

- <span id="byteset-contains"></span>`fn contains(&self, byte: u8) -> bool`

  Return true if and only if the given byte is in this set.

- <span id="byteset-contains-range"></span>`fn contains_range(&self, start: u8, end: u8) -> bool`

  Return true if and only if the given inclusive range of bytes is in

  this set.

- <span id="byteset-iter"></span>`fn iter(&self) -> ByteSetIter<'_>` — [`ByteSetIter`](#bytesetiter)

  Returns an iterator over all bytes in this set.

- <span id="byteset-iter-ranges"></span>`fn iter_ranges(&self) -> ByteSetRangeIter<'_>` — [`ByteSetRangeIter`](#bytesetrangeiter)

  Returns an iterator over all contiguous ranges of bytes in this set.

- <span id="byteset-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if and only if this set is empty.

- <span id="byteset-from-bytes"></span>`fn from_bytes(slice: &[u8]) -> Result<(ByteSet, usize), DeserializeError>` — [`ByteSet`](#byteset), [`DeserializeError`](../wire/index.md#deserializeerror)

  Deserializes a byte set from the given slice. If the slice is of

  incorrect length or is otherwise malformed, then an error is returned.

  Upon success, the number of bytes read along with the set are returned.

  The number of bytes read is always a multiple of 8.

- <span id="byteset-write-to"></span>`fn write_to<E: crate::util::wire::Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md#serializeerror)

  Writes this byte set to the given byte buffer. If the given buffer is

  too small, then an error is returned. Upon success, the total number of

  bytes written is returned. The number of bytes written is guaranteed to

  be a multiple of 8.

- <span id="byteset-write-to-len"></span>`fn write_to_len(&self) -> usize`

  Returns the total number of bytes written by `write_to`.

#### Trait Implementations

##### `impl Any for ByteSet`

- <span id="byteset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteSet`

- <span id="byteset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteSet`

- <span id="byteset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ByteSet`

- <span id="byteset-clone"></span>`fn clone(&self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl CloneToUninit for ByteSet`

- <span id="byteset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ByteSet`

##### `impl Debug for ByteSet`

- <span id="byteset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteSet`

- <span id="byteset-default"></span>`fn default() -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Eq for ByteSet`

##### `impl<T> From for ByteSet`

- <span id="byteset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteSet`

- <span id="byteset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ByteSet`

- <span id="byteset-partialeq-eq"></span>`fn eq(&self, other: &ByteSet) -> bool` — [`ByteSet`](#byteset)

##### `impl StructuralPartialEq for ByteSet`

##### `impl ToOwned for ByteSet`

- <span id="byteset-toowned-type-owned"></span>`type Owned = T`

- <span id="byteset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="byteset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ByteSet`

- <span id="byteset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteSet`

- <span id="byteset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BitSet`

```rust
struct BitSet([u128; 2]);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:749`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L749)*

The representation of a byte set. Split out so that we can define a
convenient Debug impl for it while keeping "ByteSet" in the output.

#### Trait Implementations

##### `impl Any for BitSet`

- <span id="bitset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BitSet`

- <span id="bitset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BitSet`

- <span id="bitset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BitSet`

- <span id="bitset-clone"></span>`fn clone(&self) -> BitSet` — [`BitSet`](#bitset)

##### `impl CloneToUninit for BitSet`

- <span id="bitset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BitSet`

##### `impl Debug for BitSet`

- <span id="bitset-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for BitSet`

- <span id="bitset-default"></span>`fn default() -> BitSet` — [`BitSet`](#bitset)

##### `impl Eq for BitSet`

##### `impl<T> From for BitSet`

- <span id="bitset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BitSet`

- <span id="bitset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for BitSet`

- <span id="bitset-partialeq-eq"></span>`fn eq(&self, other: &BitSet) -> bool` — [`BitSet`](#bitset)

##### `impl StructuralPartialEq for BitSet`

##### `impl ToOwned for BitSet`

- <span id="bitset-toowned-type-owned"></span>`type Owned = T`

- <span id="bitset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bitset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BitSet`

- <span id="bitset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bitset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BitSet`

- <span id="bitset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bitset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteSetIter<'a>`

```rust
struct ByteSetIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:869-872`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L869-L872)*

#### Trait Implementations

##### `impl Any for ByteSetIter<'a>`

- <span id="bytesetiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteSetIter<'a>`

- <span id="bytesetiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteSetIter<'a>`

- <span id="bytesetiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteSetIter<'a>`

- <span id="bytesetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteSetIter<'a>`

- <span id="bytesetiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteSetIter<'a>`

- <span id="bytesetiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteSetIter<'a>`

- <span id="bytesetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bytesetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bytesetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteSetIter<'a>`

- <span id="bytesetiter-iterator-type-item"></span>`type Item = u8`

- <span id="bytesetiter-iterator-next"></span>`fn next(&mut self) -> Option<u8>`

##### `impl<U> TryFrom for ByteSetIter<'a>`

- <span id="bytesetiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytesetiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteSetIter<'a>`

- <span id="bytesetiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytesetiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteSetRangeIter<'a>`

```rust
struct ByteSetRangeIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:890-893`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L890-L893)*

#### Trait Implementations

##### `impl Any for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bytesetrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bytesetrangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-iterator-type-item"></span>`type Item = (u8, u8)`

- <span id="bytesetrangeiter-iterator-next"></span>`fn next(&mut self) -> Option<(u8, u8)>`

##### `impl<U> TryFrom for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bytesetrangeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bytesetrangeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `UnitKind`

```rust
enum UnitKind {
    U8(u8),
    EOI(u16),
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:82-91`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/alphabet.rs#L82-L91)*

#### Variants

- **`U8`**

  Represents a byte value, or more typically, an equivalence class
  represented as a byte value.

- **`EOI`**

  Represents the "end of input" sentinel. We regrettably use a `u16`
  here since the maximum sentinel value is `256`. Thankfully, we don't
  actually store a `Unit` anywhere, so this extra space shouldn't be too
  bad.

#### Trait Implementations

##### `impl Any for UnitKind`

- <span id="unitkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnitKind`

- <span id="unitkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnitKind`

- <span id="unitkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for UnitKind`

- <span id="unitkind-clone"></span>`fn clone(&self) -> UnitKind` — [`UnitKind`](#unitkind)

##### `impl CloneToUninit for UnitKind`

- <span id="unitkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for UnitKind`

##### `impl Eq for UnitKind`

##### `impl<T> From for UnitKind`

- <span id="unitkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UnitKind`

- <span id="unitkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for UnitKind`

- <span id="unitkind-ord-cmp"></span>`fn cmp(&self, other: &UnitKind) -> cmp::Ordering` — [`UnitKind`](#unitkind)

##### `impl PartialEq for UnitKind`

- <span id="unitkind-partialeq-eq"></span>`fn eq(&self, other: &UnitKind) -> bool` — [`UnitKind`](#unitkind)

##### `impl PartialOrd for UnitKind`

- <span id="unitkind-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitKind) -> option::Option<cmp::Ordering>` — [`UnitKind`](#unitkind)

##### `impl StructuralPartialEq for UnitKind`

##### `impl ToOwned for UnitKind`

- <span id="unitkind-toowned-type-owned"></span>`type Owned = T`

- <span id="unitkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unitkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UnitKind`

- <span id="unitkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unitkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnitKind`

- <span id="unitkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unitkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

