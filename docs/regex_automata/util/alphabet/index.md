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

## Structs

### `Unit`

```rust
struct Unit(UnitKind);
```

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

- `fn u8(byte: u8) -> Unit` — [`Unit`](../../../util/alphabet/index.md)

- `fn eoi(num_byte_equiv_classes: usize) -> Unit` — [`Unit`](../../../util/alphabet/index.md)

- `fn as_u8(self: Self) -> Option<u8>`

- `fn as_eoi(self: Self) -> Option<u16>`

- `fn as_usize(self: Self) -> usize`

- `fn is_byte(self: Self, byte: u8) -> bool`

- `fn is_eoi(self: Self) -> bool`

- `fn is_word_byte(self: Self) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Unit` — [`Unit`](../../../util/alphabet/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Unit) -> $crate::cmp::Ordering` — [`Unit`](../../../util/alphabet/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Unit) -> bool` — [`Unit`](../../../util/alphabet/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Unit) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Unit`](../../../util/alphabet/index.md)

##### `impl StructuralPartialEq`

### `ByteClasses`

```rust
struct ByteClasses([u8; 256]);
```

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

- `fn empty() -> ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

- `fn singletons() -> ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

- `fn from_bytes(slice: &[u8]) -> Result<(ByteClasses, usize), DeserializeError>` — [`ByteClasses`](../../../util/alphabet/index.md), [`DeserializeError`](../../../util/wire/index.md)

- `fn write_to(self: &Self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../../../util/wire/index.md)

- `fn write_to_len(self: &Self) -> usize`

- `fn set(self: &mut Self, byte: u8, class: u8)`

- `fn get(self: &Self, byte: u8) -> u8`

- `fn get_by_unit(self: &Self, unit: Unit) -> usize` — [`Unit`](../../../util/alphabet/index.md)

- `fn eoi(self: &Self) -> Unit` — [`Unit`](../../../util/alphabet/index.md)

- `fn alphabet_len(self: &Self) -> usize`

- `fn stride2(self: &Self) -> usize`

- `fn is_singleton(self: &Self) -> bool`

- `fn iter(self: &Self) -> ByteClassIter<'_>` — [`ByteClassIter`](../../../util/alphabet/index.md)

- `fn representatives<R: core::ops::RangeBounds<u8>>(self: &Self, range: R) -> ByteClassRepresentatives<'_>` — [`ByteClassRepresentatives`](../../../util/alphabet/index.md)

- `fn elements(self: &Self, class: Unit) -> ByteClassElements<'_>` — [`Unit`](../../../util/alphabet/index.md), [`ByteClassElements`](../../../util/alphabet/index.md)

- `fn element_ranges(self: &Self, class: Unit) -> ByteClassElementRanges<'_>` — [`Unit`](../../../util/alphabet/index.md), [`ByteClassElementRanges`](../../../util/alphabet/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

### `ByteClassIter<'a>`

```rust
struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}
```

An iterator over each equivalence class.

The last element in this iterator always corresponds to `Unit::eoi`.

This is created by the `ByteClasses::iter` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = Unit`

- `fn next(self: &mut Self) -> Option<Unit>` — [`Unit`](../../../util/alphabet/index.md)

### `ByteClassRepresentatives<'a>`

```rust
struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    cur_byte: usize,
    end_byte: Option<usize>,
    last_class: Option<u8>,
}
```

An iterator over representative bytes from each equivalence class.

This is created by the `ByteClasses::representatives` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = Unit`

- `fn next(self: &mut Self) -> Option<Unit>` — [`Unit`](../../../util/alphabet/index.md)

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}
```

An iterator over all elements in an equivalence class.

This is created by the `ByteClasses::elements` method.

The lifetime `'a` refers to the lifetime of the byte classes that this
iterator was created from.

#### Trait Implementations

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a>`

- `type Item = Unit`

- `fn next(self: &mut Self) -> Option<Unit>` — [`Unit`](../../../util/alphabet/index.md)

