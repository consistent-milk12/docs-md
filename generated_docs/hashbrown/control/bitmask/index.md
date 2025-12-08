*[hashbrown](../../index.md) / [control](../index.md) / [bitmask](index.md)*

---

# Module `bitmask`

## Structs

### `BitMask`

```rust
struct BitMask(u16);
```

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

- `fn invert(self: Self) -> Self`

- `fn remove_lowest_bit(self: Self) -> Self`

- `fn any_bit_set(self: Self) -> bool`

- `fn lowest_set_bit(self: Self) -> Option<usize>`

- `fn trailing_zeros(self: Self) -> usize`

- `fn nonzero_trailing_zeros(nonzero: core::num::NonZeroU16) -> usize`

- `fn leading_zeros(self: Self) -> usize`

#### Trait Implementations

##### `impl Clone for BitMask`

- `fn clone(self: &Self) -> BitMask` — [`BitMask`](#bitmask)

##### `impl Copy for BitMask`

##### `impl IntoIterator for BitMask`

- `type Item = usize`

- `type IntoIter = BitMaskIter`

- `fn into_iter(self: Self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

### `BitMaskIter`

```rust
struct BitMaskIter(BitMask);
```

Iterator over the contents of a `BitMask`, returning the indices of set
bits.

#### Trait Implementations

##### `impl Clone for BitMaskIter`

- `fn clone(self: &Self) -> BitMaskIter` — [`BitMaskIter`](#bitmaskiter)

##### `impl<I> IntoIterator for BitMaskIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for BitMaskIter`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

