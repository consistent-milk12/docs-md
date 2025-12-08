*[aho_corasick](../../index.md) / [util](../index.md) / [alphabet](index.md)*

---

# Module `alphabet`

## Structs

### `ByteClasses`

```rust
struct ByteClasses([u8; 256]);
```

A representation of byte oriented equivalence classes.

This is used in finite state machines to reduce the size of the transition
table. This can have a particularly large impact not only on the total size
of an FSM, but also on FSM build times because it reduces the number of
transitions that need to be visited/set.

#### Implementations

- `fn empty() -> ByteClasses` — [`ByteClasses`](#byteclasses)

- `fn singletons() -> ByteClasses` — [`ByteClasses`](#byteclasses)

- `fn set(self: &mut Self, byte: u8, class: u8)`

- `fn get(self: &Self, byte: u8) -> u8`

- `fn alphabet_len(self: &Self) -> usize`

- `fn stride2(self: &Self) -> usize`

- `fn stride(self: &Self) -> usize`

- `fn is_singleton(self: &Self) -> bool`

- `fn iter(self: &Self) -> ByteClassIter` — [`ByteClassIter`](#byteclassiter)

- `fn elements(self: &Self, class: u8) -> ByteClassElements<'_>` — [`ByteClassElements`](#byteclasselements)

- `fn element_ranges(self: &Self, class: u8) -> ByteClassElementRanges<'_>` — [`ByteClassElementRanges`](#byteclasselementranges)

#### Trait Implementations

##### `impl Clone for ByteClasses`

- `fn clone(self: &Self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

##### `impl Copy for ByteClasses`

##### `impl Debug for ByteClasses`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ByteClassIter`

```rust
struct ByteClassIter {
    it: core::ops::Range<usize>,
}
```

An iterator over each equivalence class.

#### Trait Implementations

##### `impl Debug for ByteClassIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ByteClassIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for ByteClassIter`

- `type Item = u8`

- `fn next(self: &mut Self) -> Option<u8>`

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: u8,
    bytes: core::ops::RangeInclusive<u8>,
}
```

An iterator over all elements in a specific equivalence class.

#### Trait Implementations

##### `impl<'a> Debug for ByteClassElements<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ByteClassElements<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ByteClassElements<'a>`

- `type Item = u8`

- `fn next(self: &mut Self) -> Option<u8>`

### `ByteClassElementRanges<'a>`

```rust
struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(u8, u8)>,
}
```

An iterator over all elements in an equivalence class expressed as a
sequence of contiguous ranges.

#### Trait Implementations

##### `impl<'a> Debug for ByteClassElementRanges<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for ByteClassElementRanges<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ByteClassElementRanges<'a>`

- `type Item = (u8, u8)`

- `fn next(self: &mut Self) -> Option<(u8, u8)>`

### `ByteClassSet`

```rust
struct ByteClassSet(ByteSet);
```

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

- `fn empty() -> Self`

- `fn set_range(self: &mut Self, start: u8, end: u8)`

- `fn byte_classes(self: &Self) -> ByteClasses` — [`ByteClasses`](#byteclasses)

#### Trait Implementations

##### `impl Clone for ByteClassSet`

- `fn clone(self: &Self) -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

##### `impl Debug for ByteClassSet`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ByteClassSet`

- `fn default() -> ByteClassSet` — [`ByteClassSet`](#byteclassset)

### `ByteSet`

```rust
struct ByteSet {
    bits: BitSet,
}
```

A simple set of bytes that is reasonably cheap to copy and allocation free.

#### Implementations

- `fn empty() -> ByteSet` — [`ByteSet`](#byteset)

- `fn add(self: &mut Self, byte: u8)`

- `fn contains(self: &Self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for ByteSet`

- `fn clone(self: &Self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Copy for ByteSet`

##### `impl Debug for ByteSet`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ByteSet`

- `fn default() -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Eq for ByteSet`

##### `impl PartialEq for ByteSet`

- `fn eq(self: &Self, other: &ByteSet) -> bool` — [`ByteSet`](#byteset)

##### `impl StructuralPartialEq for ByteSet`

### `BitSet`

```rust
struct BitSet([u128; 2]);
```

The representation of a byte set. Split out so that we can define a
convenient Debug impl for it while keeping "ByteSet" in the output.

#### Trait Implementations

##### `impl Clone for BitSet`

- `fn clone(self: &Self) -> BitSet` — [`BitSet`](#bitset)

##### `impl Copy for BitSet`

##### `impl Debug for BitSet`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for BitSet`

- `fn default() -> BitSet` — [`BitSet`](#bitset)

##### `impl Eq for BitSet`

##### `impl PartialEq for BitSet`

- `fn eq(self: &Self, other: &BitSet) -> bool` — [`BitSet`](#bitset)

##### `impl StructuralPartialEq for BitSet`

