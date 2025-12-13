*[hashbrown](../../index.md) / [control](../index.md) / [bitmask](index.md)*

---

# Module `bitmask`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitMask`](#bitmask) | struct | A bit mask which contains the result of a `Match` operation on a `Group` and allows iterating through them. |
| [`BitMaskIter`](#bitmaskiter) | struct | Iterator over the contents of a `BitMask`, returning the indices of set bits. |

## Structs

### `BitMask`

```rust
struct BitMask(u16);
```

*Defined in [`hashbrown-0.16.1/src/control/bitmask.rs:22`](../../../../.source_1765521767/hashbrown-0.16.1/src/control/bitmask.rs#L22)*

A bit mask which contains the result of a `Match` operation on a `Group` and
allows iterating through them.

The bit mask is arranged so that low-order bits represent lower memory
addresses for group match results.

For implementation reasons, the bits in the set may be sparsely packed with
groups of 8 bits representing one element. If any of these bits are non-zero
then this element is considered to true in the mask. If this is the
case, `BITMASK_STRIDE` will be 8 to indicate a divide-by-8 should be
performed on counts/indices to normalize this difference. `BITMASK_MASK` is
similarly a mask of all the actually-used bits.

To iterate over a bit mask, it must be converted to a form where only 1 bit
is set per element. This is done by applying `BITMASK_ITER_MASK` on the
mask bits.

#### Implementations

- <span id="bitmask-invert"></span>`fn invert(self) -> Self`

  Returns a new `BitMask` with all bits inverted.

- <span id="bitmask-remove-lowest-bit"></span>`fn remove_lowest_bit(self) -> Self`

  Returns a new `BitMask` with the lowest bit removed.

- <span id="bitmask-any-bit-set"></span>`fn any_bit_set(self) -> bool`

  Returns whether the `BitMask` has at least one set bit.

- <span id="bitmask-lowest-set-bit"></span>`fn lowest_set_bit(self) -> Option<usize>`

  Returns the first set bit in the `BitMask`, if there is one.

- <span id="bitmask-trailing-zeros"></span>`fn trailing_zeros(self) -> usize`

  Returns the number of trailing zeroes in the `BitMask`.

- <span id="bitmask-nonzero-trailing-zeros"></span>`fn nonzero_trailing_zeros(nonzero: core::num::NonZeroU16) -> usize`

  Same as above but takes a `NonZeroBitMaskWord`.

- <span id="bitmask-leading-zeros"></span>`fn leading_zeros(self) -> usize`

  Returns the number of leading zeroes in the `BitMask`.

#### Trait Implementations

##### `impl Any for BitMask`

- <span id="bitmask-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BitMask`

- <span id="bitmask-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BitMask`

- <span id="bitmask-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BitMask`

- <span id="bitmask-clone"></span>`fn clone(&self) -> BitMask` — [`BitMask`](#bitmask)

##### `impl CloneToUninit for BitMask`

- <span id="bitmask-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for BitMask`

##### `impl<T> From for BitMask`

- <span id="bitmask-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BitMask`

- <span id="bitmask-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for BitMask`

- <span id="bitmask-intoiterator-type-item"></span>`type Item = usize`

- <span id="bitmask-intoiterator-type-intoiter"></span>`type IntoIter = BitMaskIter`

- <span id="bitmask-intoiterator-into-iter"></span>`fn into_iter(self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

##### `impl ToOwned for BitMask`

- <span id="bitmask-toowned-type-owned"></span>`type Owned = T`

- <span id="bitmask-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bitmask-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BitMask`

- <span id="bitmask-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bitmask-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BitMask`

- <span id="bitmask-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bitmask-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BitMaskIter`

```rust
struct BitMaskIter(BitMask);
```

*Defined in [`hashbrown-0.16.1/src/control/bitmask.rs:106`](../../../../.source_1765521767/hashbrown-0.16.1/src/control/bitmask.rs#L106)*

Iterator over the contents of a `BitMask`, returning the indices of set
bits.

#### Trait Implementations

##### `impl Any for BitMaskIter`

- <span id="bitmaskiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BitMaskIter`

- <span id="bitmaskiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BitMaskIter`

- <span id="bitmaskiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BitMaskIter`

- <span id="bitmaskiter-clone"></span>`fn clone(&self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

##### `impl CloneToUninit for BitMaskIter`

- <span id="bitmaskiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for BitMaskIter`

- <span id="bitmaskiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BitMaskIter`

- <span id="bitmaskiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for BitMaskIter`

- <span id="bitmaskiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bitmaskiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="bitmaskiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BitMaskIter`

- <span id="bitmaskiter-iterator-type-item"></span>`type Item = usize`

- <span id="bitmaskiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

##### `impl ToOwned for BitMaskIter`

- <span id="bitmaskiter-toowned-type-owned"></span>`type Owned = T`

- <span id="bitmaskiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bitmaskiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BitMaskIter`

- <span id="bitmaskiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bitmaskiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BitMaskIter`

- <span id="bitmaskiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bitmaskiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

