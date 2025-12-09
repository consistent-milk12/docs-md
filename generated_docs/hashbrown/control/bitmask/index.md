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

*Defined in [`hashbrown-0.16.1/src/control/bitmask.rs:22`](../../../../.source_1765210505/hashbrown-0.16.1/src/control/bitmask.rs#L22)*

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

- <span id="bitmask-remove-lowest-bit"></span>`fn remove_lowest_bit(self) -> Self`

- <span id="bitmask-any-bit-set"></span>`fn any_bit_set(self) -> bool`

- <span id="bitmask-lowest-set-bit"></span>`fn lowest_set_bit(self) -> Option<usize>`

- <span id="bitmask-trailing-zeros"></span>`fn trailing_zeros(self) -> usize`

- <span id="bitmask-nonzero-trailing-zeros"></span>`fn nonzero_trailing_zeros(nonzero: core::num::NonZeroU16) -> usize`

- <span id="bitmask-leading-zeros"></span>`fn leading_zeros(self) -> usize`

#### Trait Implementations

##### `impl Clone for BitMask`

- <span id="bitmask-clone"></span>`fn clone(&self) -> BitMask` — [`BitMask`](#bitmask)

##### `impl Copy for BitMask`

##### `impl IntoIterator for BitMask`

- <span id="bitmask-type-item"></span>`type Item = usize`

- <span id="bitmask-type-intoiter"></span>`type IntoIter = BitMaskIter`

- <span id="bitmask-into-iter"></span>`fn into_iter(self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

### `BitMaskIter`

```rust
struct BitMaskIter(BitMask);
```

*Defined in [`hashbrown-0.16.1/src/control/bitmask.rs:106`](../../../../.source_1765210505/hashbrown-0.16.1/src/control/bitmask.rs#L106)*

Iterator over the contents of a `BitMask`, returning the indices of set
bits.

#### Trait Implementations

##### `impl Clone for BitMaskIter`

- <span id="bitmaskiter-clone"></span>`fn clone(&self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

##### `impl IntoIterator for BitMaskIter`

- <span id="bitmaskiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="bitmaskiter-type-intoiter"></span>`type IntoIter = I`

- <span id="bitmaskiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BitMaskIter`

- <span id="bitmaskiter-type-item"></span>`type Item = usize`

- <span id="bitmaskiter-next"></span>`fn next(&mut self) -> Option<usize>`

