*[aho_corasick](../../index.md) / [util](../index.md) / [alphabet](index.md)*

---

# Module `alphabet`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ByteClasses`](#byteclasses) | struct | A representation of byte oriented equivalence classes. |
| [`ByteClassIter`](#byteclassiter) | struct | An iterator over each equivalence class. |
| [`ByteClassElements`](#byteclasselements) | struct | An iterator over all elements in a specific equivalence class. |
| [`ByteClassElementRanges`](#byteclasselementranges) | struct | An iterator over all elements in an equivalence class expressed as a sequence of contiguous ranges. |
| [`ByteClassSet`](#byteclassset) | struct | A partitioning of bytes into equivalence classes. |
| [`ByteSet`](#byteset) | struct | A simple set of bytes that is reasonably cheap to copy and allocation free. |
| [`BitSet`](#bitset) | struct | The representation of a byte set. |

## Structs

### `ByteClasses`

```rust
struct ByteClasses([u8; 256]);
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:10`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L10)*

A representation of byte oriented equivalence classes.

This is used in finite state machines to reduce the size of the transition
table. This can have a particularly large impact not only on the total size
of an FSM, but also on FSM build times because it reduces the number of
transitions that need to be visited/set.

#### Implementations

- <span id="byteclasses-empty"></span>`fn empty() -> ByteClasses` — [`ByteClasses`](#byteclasses)

- <span id="byteclasses-singletons"></span>`fn singletons() -> ByteClasses` — [`ByteClasses`](#byteclasses)

- <span id="byteclasses-set"></span>`fn set(&mut self, byte: u8, class: u8)`

- <span id="byteclasses-get"></span>`fn get(&self, byte: u8) -> u8`

- <span id="byteclasses-alphabet-len"></span>`fn alphabet_len(&self) -> usize`

- <span id="byteclasses-stride2"></span>`fn stride2(&self) -> usize`

- <span id="byteclasses-stride"></span>`fn stride(&self) -> usize`

- <span id="byteclasses-is-singleton"></span>`fn is_singleton(&self) -> bool`

- <span id="byteclasses-iter"></span>`fn iter(&self) -> ByteClassIter` — [`ByteClassIter`](#byteclassiter)

- <span id="byteclasses-elements"></span>`fn elements(&self, class: u8) -> ByteClassElements<'_>` — [`ByteClassElements`](#byteclasselements)

- <span id="byteclasses-element-ranges"></span>`fn element_ranges(&self, class: u8) -> ByteClassElementRanges<'_>` — [`ByteClassElementRanges`](#byteclasselementranges)

#### Trait Implementations

##### `impl Clone for ByteClasses`

- <span id="byteclasses-clone"></span>`fn clone(&self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

##### `impl Copy for ByteClasses`

##### `impl Debug for ByteClasses`

- <span id="byteclasses-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ByteClassIter`

```rust
struct ByteClassIter {
    it: core::ops::Range<usize>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:125-127`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L125-L127)*

An iterator over each equivalence class.

#### Trait Implementations

##### `impl Debug for ByteClassIter`

- <span id="byteclassiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassIter`

- <span id="byteclassiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassIter`

- <span id="byteclassiter-iterator-type-item"></span>`type Item = u8`

- <span id="byteclassiter-next"></span>`fn next(&mut self) -> Option<u8>`

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: u8,
    bytes: core::ops::RangeInclusive<u8>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:139-143`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L139-L143)*

An iterator over all elements in a specific equivalence class.

#### Trait Implementations

##### `impl Debug for ByteClassElements<'a>`

- <span id="byteclasselements-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ByteClassElements<'a>`

- <span id="byteclasselements-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclasselements-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclasselements-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassElements<'a>`

- <span id="byteclasselements-iterator-type-item"></span>`type Item = u8`

- <span id="byteclasselements-next"></span>`fn next(&mut self) -> Option<u8>`

### `ByteClassElementRanges<'a>`

```rust
struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(u8, u8)>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:161-164`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L161-L164)*

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

- <span id="byteclasselementranges-iterator-type-item"></span>`type Item = (u8, u8)`

- <span id="byteclasselementranges-next"></span>`fn next(&mut self) -> Option<(u8, u8)>`

### `ByteClassSet`

```rust
struct ByteClassSet(ByteSet);
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:207`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L207)*

A partitioning of bytes into equivalence classes.

A byte class set keeps track of an *approximation* of equivalence classes
of bytes during NFA construction. That is, every byte in an equivalence
class cannot discriminate between a match and a non-match.

Note that this may not compute the minimal set of equivalence classes.
Basically, any byte in a pattern given to the noncontiguous NFA builder
will automatically be treated as its own equivalence class. All other
bytes---any byte not in any pattern---will be treated as their own
equivalence classes. In theory, all bytes not in any pattern should
be part of a single equivalence class, but in practice, we only treat
contiguous ranges of bytes as an equivalence class. So the number of
classes computed may be bigger than necessary. This usually doesn't make
much of a difference, and keeps the implementation simple.

#### Implementations

- <span id="byteclassset-empty"></span>`fn empty() -> Self`

- <span id="byteclassset-set-range"></span>`fn set_range(&mut self, start: u8, end: u8)`

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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:255-257`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L255-L257)*

A simple set of bytes that is reasonably cheap to copy and allocation free.

#### Implementations

- <span id="byteset-empty"></span>`fn empty() -> ByteSet` — [`ByteSet`](#byteset)

- <span id="byteset-add"></span>`fn add(&mut self, byte: u8)`

- <span id="byteset-contains"></span>`fn contains(&self, byte: u8) -> bool`

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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:262`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/alphabet.rs#L262)*

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

