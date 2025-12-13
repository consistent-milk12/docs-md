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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:10`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L10)*

A representation of byte oriented equivalence classes.

This is used in finite state machines to reduce the size of the transition
table. This can have a particularly large impact not only on the total size
of an FSM, but also on FSM build times because it reduces the number of
transitions that need to be visited/set.

#### Implementations

- <span id="byteclasses-empty"></span>`fn empty() -> ByteClasses` — [`ByteClasses`](#byteclasses)

  Creates a new set of equivalence classes where all bytes are mapped to

  the same class.

- <span id="byteclasses-singletons"></span>`fn singletons() -> ByteClasses` — [`ByteClasses`](#byteclasses)

  Creates a new set of equivalence classes where each byte belongs to

  its own equivalence class.

- <span id="byteclasses-set"></span>`fn set(&mut self, byte: u8, class: u8)`

  Set the equivalence class for the given byte.

- <span id="byteclasses-get"></span>`fn get(&self, byte: u8) -> u8`

  Get the equivalence class for the given byte.

- <span id="byteclasses-alphabet-len"></span>`fn alphabet_len(&self) -> usize`

  Return the total number of elements in the alphabet represented by

  these equivalence classes. Equivalently, this returns the total number

  of equivalence classes.

- <span id="byteclasses-stride2"></span>`fn stride2(&self) -> usize`

  Returns the stride, as a base-2 exponent, required for these

  equivalence classes.

  

  The stride is always the smallest power of 2 that is greater than or

  equal to the alphabet length. This is done so that converting between

  state IDs and indices can be done with shifts alone, which is much

  faster than integer division. The "stride2" is the exponent. i.e.,

  `2^stride2 = stride`.

- <span id="byteclasses-stride"></span>`fn stride(&self) -> usize`

  Returns the stride for these equivalence classes, which corresponds

  to the smallest power of 2 greater than or equal to the number of

  equivalence classes.

- <span id="byteclasses-is-singleton"></span>`fn is_singleton(&self) -> bool`

  Returns true if and only if every byte in this class maps to its own

  equivalence class. Equivalently, there are 257 equivalence classes

  and each class contains exactly one byte (plus the special EOI class).

- <span id="byteclasses-iter"></span>`fn iter(&self) -> ByteClassIter` — [`ByteClassIter`](#byteclassiter)

  Returns an iterator over all equivalence classes in this set.

- <span id="byteclasses-elements"></span>`fn elements(&self, class: u8) -> ByteClassElements<'_>` — [`ByteClassElements`](#byteclasselements)

  Returns an iterator of the bytes in the given equivalence class.

- <span id="byteclasses-element-ranges"></span>`fn element_ranges(&self, class: u8) -> ByteClassElementRanges<'_>` — [`ByteClassElementRanges`](#byteclasselementranges)

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

### `ByteClassIter`

```rust
struct ByteClassIter {
    it: core::ops::Range<usize>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:125-127`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L125-L127)*

An iterator over each equivalence class.

#### Trait Implementations

##### `impl Any for ByteClassIter`

- <span id="byteclassiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ByteClassIter`

- <span id="byteclassiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ByteClassIter`

- <span id="byteclassiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ByteClassIter`

- <span id="byteclassiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ByteClassIter`

- <span id="byteclassiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ByteClassIter`

- <span id="byteclassiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ByteClassIter`

- <span id="byteclassiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="byteclassiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="byteclassiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ByteClassIter`

- <span id="byteclassiter-iterator-type-item"></span>`type Item = u8`

- <span id="byteclassiter-iterator-next"></span>`fn next(&mut self) -> Option<u8>`

##### `impl<U> TryFrom for ByteClassIter`

- <span id="byteclassiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="byteclassiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ByteClassIter`

- <span id="byteclassiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="byteclassiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ByteClassElements<'a>`

```rust
struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: u8,
    bytes: core::ops::RangeInclusive<u8>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:139-143`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L139-L143)*

An iterator over all elements in a specific equivalence class.

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

- <span id="byteclasselements-iterator-type-item"></span>`type Item = u8`

- <span id="byteclasselements-iterator-next"></span>`fn next(&mut self) -> Option<u8>`

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
    range: Option<(u8, u8)>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:161-164`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L161-L164)*

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

- <span id="byteclasselementranges-iterator-type-item"></span>`type Item = (u8, u8)`

- <span id="byteclasselementranges-iterator-next"></span>`fn next(&mut self) -> Option<(u8, u8)>`

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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:207`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L207)*

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

  Create a new set of byte classes where all bytes are part of the same

  equivalence class.

- <span id="byteclassset-set-range"></span>`fn set_range(&mut self, start: u8, end: u8)`

  Indicate the the range of byte given (inclusive) can discriminate a

  match between it and all other bytes outside of the range.

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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:255-257`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L255-L257)*

A simple set of bytes that is reasonably cheap to copy and allocation free.

#### Implementations

- <span id="byteset-empty"></span>`fn empty() -> ByteSet` — [`ByteSet`](#byteset)

  Create an empty set of bytes.

- <span id="byteset-add"></span>`fn add(&mut self, byte: u8)`

  Add a byte to this set.

  

  If the given byte already belongs to this set, then this is a no-op.

- <span id="byteset-contains"></span>`fn contains(&self, byte: u8) -> bool`

  Return true if and only if the given byte is in this set.

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

*Defined in [`aho-corasick-1.1.4/src/util/alphabet.rs:262`](../../../../.source_1765633015/aho-corasick-1.1.4/src/util/alphabet.rs#L262)*

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

