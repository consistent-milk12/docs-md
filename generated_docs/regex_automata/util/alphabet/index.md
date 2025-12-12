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

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:79`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L79)*

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

- <span id="unit-eoi"></span>`fn eoi(num_byte_equiv_classes: usize) -> Unit` — [`Unit`](#unit)

- <span id="unit-as-u8"></span>`fn as_u8(self) -> Option<u8>`

- <span id="unit-as-eoi"></span>`fn as_eoi(self) -> Option<u16>`

- <span id="unit-as-usize"></span>`fn as_usize(self) -> usize`

- <span id="unit-is-byte"></span>`fn is_byte(self, byte: u8) -> bool`

- <span id="unit-is-eoi"></span>`fn is_eoi(self) -> bool`

- <span id="unit-is-word-byte"></span>`fn is_word_byte(self) -> bool`

#### Trait Implementations

##### `impl Clone for Unit`

- <span id="unit-clone"></span>`fn clone(&self) -> Unit` — [`Unit`](#unit)

##### `impl Copy for Unit`

##### `impl Debug for Unit`

- <span id="unit-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Unit`

##### `impl Ord for Unit`

- <span id="unit-cmp"></span>`fn cmp(&self, other: &Unit) -> cmp::Ordering` — [`Unit`](#unit)

##### `impl PartialEq for Unit`

- <span id="unit-eq"></span>`fn eq(&self, other: &Unit) -> bool` — [`Unit`](#unit)

##### `impl PartialOrd for Unit`

- <span id="unit-partial-cmp"></span>`fn partial_cmp(&self, other: &Unit) -> option::Option<cmp::Ordering>` — [`Unit`](#unit)

##### `impl StructuralPartialEq for Unit`

### `ByteClasses`

```rust
struct ByteClasses([u8; 256]);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:215`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L215)*

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

- <span id="byteclasses-singletons"></span>`fn singletons() -> ByteClasses` — [`ByteClasses`](#byteclasses)

- <span id="byteclasses-from-bytes"></span>`fn from_bytes(slice: &[u8]) -> Result<(ByteClasses, usize), DeserializeError>` — [`ByteClasses`](#byteclasses), [`DeserializeError`](../wire/index.md#deserializeerror)

- <span id="byteclasses-write-to"></span>`fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md#serializeerror)

- <span id="byteclasses-write-to-len"></span>`fn write_to_len(&self) -> usize`

- <span id="byteclasses-set"></span>`fn set(&mut self, byte: u8, class: u8)`

- <span id="byteclasses-get"></span>`fn get(&self, byte: u8) -> u8`

- <span id="byteclasses-get-by-unit"></span>`fn get_by_unit(&self, unit: Unit) -> usize` — [`Unit`](#unit)

- <span id="byteclasses-eoi"></span>`fn eoi(&self) -> Unit` — [`Unit`](#unit)

- <span id="byteclasses-alphabet-len"></span>`fn alphabet_len(&self) -> usize`

- <span id="byteclasses-stride2"></span>`fn stride2(&self) -> usize`

- <span id="byteclasses-is-singleton"></span>`fn is_singleton(&self) -> bool`

- <span id="byteclasses-iter"></span>`fn iter(&self) -> ByteClassIter<'_>` — [`ByteClassIter`](#byteclassiter)

- <span id="byteclasses-representatives"></span>`fn representatives<R: core::ops::RangeBounds<u8>>(&self, range: R) -> ByteClassRepresentatives<'_>` — [`ByteClassRepresentatives`](#byteclassrepresentatives)

- <span id="byteclasses-elements"></span>`fn elements(&self, class: Unit) -> ByteClassElements<'_>` — [`Unit`](#unit), [`ByteClassElements`](#byteclasselements)

- <span id="byteclasses-element-ranges"></span>`fn element_ranges(&self, class: Unit) -> ByteClassElementRanges<'_>` — [`Unit`](#unit), [`ByteClassElementRanges`](#byteclasselementranges)

#### Trait Implementations

##### `impl Clone for ByteClasses`

- <span id="byteclasses-clone"></span>`fn clone(&self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

##### `impl Copy for ByteClasses`

##### `impl Debug for ByteClasses`

- <span id="byteclasses-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for ByteClasses`

- <span id="byteclasses-default"></span>`fn default() -> ByteClasses` — [`ByteClasses`](#byteclasses)

### `ByteClassIter<'a>`

```rust
struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:525-528`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L525-L528)*

An iterator over each equivalence class.

The last element in this iterator always corresponds to `Unit::eoi`.

This is created by the `ByteClasses::iter` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug for ByteClassIter<'a>`

- <span id="byteclassiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassIter<'a>`

- <span id="byteclassiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassIter<'a>`

- <span id="byteclassiter-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclassiter-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

### `ByteClassRepresentatives<'a>`

```rust
struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    cur_byte: usize,
    end_byte: Option<usize>,
    last_class: Option<u8>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:554-559`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L554-L559)*

An iterator over representative bytes from each equivalence class.

This is created by the `ByteClasses::representatives` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassrepresentatives-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassrepresentatives-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassRepresentatives<'a>`

- <span id="byteclassrepresentatives-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclassrepresentatives-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:599-603`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L599-L603)*

An iterator over all elements in an equivalence class.

This is created by the `ByteClasses::elements` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug for ByteClassElements<'a>`

- <span id="byteclasselements-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassElements<'a>`

- <span id="byteclasselements-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclasselements-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclasselements-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassElements<'a>`

- <span id="byteclasselements-iterator-type-item"></span>`type Item = Unit`

- <span id="byteclasselements-next"></span>`fn next(&mut self) -> Option<Unit>` — [`Unit`](#unit)

### `ByteClassElementRanges<'a>`

```rust
struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(Unit, Unit)>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:629-632`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L629-L632)*

An iterator over all elements in an equivalence class expressed as a
sequence of contiguous ranges.

#### Trait Implementations

##### `impl Debug for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclasselementranges-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclasselementranges-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassElementRanges<'a>`

- <span id="byteclasselementranges-iterator-type-item"></span>`type Item = (Unit, Unit)`

- <span id="byteclasselementranges-next"></span>`fn next(&mut self) -> Option<(Unit, Unit)>` — [`Unit`](#unit)

### `ByteClassSet`

```rust
struct ByteClassSet(ByteSet);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:685`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L685)*

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

- <span id="byteclassset-set-range"></span>`fn set_range(&mut self, start: u8, end: u8)`

- <span id="byteclassset-add-set"></span>`fn add_set(&mut self, set: &ByteSet)` — [`ByteSet`](#byteset)

- <span id="byteclassset-byte-classes"></span>`fn byte_classes(&self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

#### Trait Implementations

##### `impl Clone for ByteClassSet`

- <span id="byteclassset-clone"></span>`fn clone(&self) -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

##### `impl Debug for ByteClassSet`

- <span id="byteclassset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteClassSet`

- <span id="byteclassset-default"></span>`fn default() -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

### `ByteSet`

```rust
struct ByteSet {
    bits: BitSet,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:742-744`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L742-L744)*

A simple set of bytes that is reasonably cheap to copy and allocation free.

#### Implementations

- <span id="byteset-empty"></span>`fn empty() -> ByteSet` — [`ByteSet`](#byteset)

- <span id="byteset-add"></span>`fn add(&mut self, byte: u8)`

- <span id="byteset-remove"></span>`fn remove(&mut self, byte: u8)`

- <span id="byteset-contains"></span>`fn contains(&self, byte: u8) -> bool`

- <span id="byteset-contains-range"></span>`fn contains_range(&self, start: u8, end: u8) -> bool`

- <span id="byteset-iter"></span>`fn iter(&self) -> ByteSetIter<'_>` — [`ByteSetIter`](#bytesetiter)

- <span id="byteset-iter-ranges"></span>`fn iter_ranges(&self) -> ByteSetRangeIter<'_>` — [`ByteSetRangeIter`](#bytesetrangeiter)

- <span id="byteset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="byteset-from-bytes"></span>`fn from_bytes(slice: &[u8]) -> Result<(ByteSet, usize), DeserializeError>` — [`ByteSet`](#byteset), [`DeserializeError`](../wire/index.md#deserializeerror)

- <span id="byteset-write-to"></span>`fn write_to<E: crate::util::wire::Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md#serializeerror)

- <span id="byteset-write-to-len"></span>`fn write_to_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for ByteSet`

- <span id="byteset-clone"></span>`fn clone(&self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Copy for ByteSet`

##### `impl Debug for ByteSet`

- <span id="byteset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteSet`

- <span id="byteset-default"></span>`fn default() -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Eq for ByteSet`

##### `impl PartialEq for ByteSet`

- <span id="byteset-eq"></span>`fn eq(&self, other: &ByteSet) -> bool` — [`ByteSet`](#byteset)

##### `impl StructuralPartialEq for ByteSet`

### `BitSet`

```rust
struct BitSet([u128; 2]);
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:749`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L749)*

The representation of a byte set. Split out so that we can define a
convenient Debug impl for it while keeping "ByteSet" in the output.

#### Trait Implementations

##### `impl Clone for BitSet`

- <span id="bitset-clone"></span>`fn clone(&self) -> BitSet` — [`BitSet`](#bitset)

##### `impl Copy for BitSet`

##### `impl Debug for BitSet`

- <span id="bitset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for BitSet`

- <span id="bitset-default"></span>`fn default() -> BitSet` — [`BitSet`](#bitset)

##### `impl Eq for BitSet`

##### `impl PartialEq for BitSet`

- <span id="bitset-eq"></span>`fn eq(&self, other: &BitSet) -> bool` — [`BitSet`](#bitset)

##### `impl StructuralPartialEq for BitSet`

### `ByteSetIter<'a>`

```rust
struct ByteSetIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:869-872`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L869-L872)*

#### Trait Implementations

##### `impl Debug for ByteSetIter<'a>`

- <span id="bytesetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteSetIter<'a>`

- <span id="bytesetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bytesetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bytesetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteSetIter<'a>`

- <span id="bytesetiter-iterator-type-item"></span>`type Item = u8`

- <span id="bytesetiter-next"></span>`fn next(&mut self) -> Option<u8>`

### `ByteSetRangeIter<'a>`

```rust
struct ByteSetRangeIter<'a> {
    set: &'a ByteSet,
    b: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:890-893`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L890-L893)*

#### Trait Implementations

##### `impl Debug for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bytesetrangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bytesetrangeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteSetRangeIter<'a>`

- <span id="bytesetrangeiter-iterator-type-item"></span>`type Item = (u8, u8)`

- <span id="bytesetrangeiter-next"></span>`fn next(&mut self) -> Option<(u8, u8)>`

## Enums

### `UnitKind`

```rust
enum UnitKind {
    U8(u8),
    EOI(u16),
}
```

*Defined in [`regex-automata-0.4.13/src/util/alphabet.rs:82-91`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/alphabet.rs#L82-L91)*

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

##### `impl Clone for UnitKind`

- <span id="unitkind-clone"></span>`fn clone(&self) -> UnitKind` — [`UnitKind`](#unitkind)

##### `impl Copy for UnitKind`

##### `impl Eq for UnitKind`

##### `impl Ord for UnitKind`

- <span id="unitkind-cmp"></span>`fn cmp(&self, other: &UnitKind) -> cmp::Ordering` — [`UnitKind`](#unitkind)

##### `impl PartialEq for UnitKind`

- <span id="unitkind-eq"></span>`fn eq(&self, other: &UnitKind) -> bool` — [`UnitKind`](#unitkind)

##### `impl PartialOrd for UnitKind`

- <span id="unitkind-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitKind) -> option::Option<cmp::Ordering>` — [`UnitKind`](#unitkind)

##### `impl StructuralPartialEq for UnitKind`

