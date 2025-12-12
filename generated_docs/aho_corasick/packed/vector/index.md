*[aho_corasick](../../index.md) / [packed](../index.md) / [vector](index.md)*

---

# Module `vector`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64_ssse3`](#x86-64-ssse3) | mod |  |
| [`x86_64_avx2`](#x86-64-avx2) | mod |  |
| [`Vector`](#vector) | trait | A trait for describing vector operations used by vectorized searchers. |
| [`FatVector`](#fatvector) | trait | This trait extends the `Vector` trait with additional operations to support Fat Teddy. |

## Modules

- [`x86_64_ssse3`](x86_64_ssse3/index.md)
- [`x86_64_avx2`](x86_64_avx2/index.md)

## Traits

### `Vector`

```rust
trait Vector: Copy + Debug + Send + Sync + UnwindSafe + RefUnwindSafe { ... }
```

*Defined in [`aho-corasick-1.1.4/src/packed/vector.rs:28-207`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/vector.rs#L28-L207)*

A trait for describing vector operations used by vectorized searchers.

The trait is highly constrained to low level vector operations needed for
the specific algorithms used in this crate. In general, it was invented
mostly to be generic over x86's __m128i and __m256i types. At time of
writing, it also supports wasm and aarch64 128-bit vector types as well.

# Safety

All methods are not safe since they are intended to be implemented using
vendor intrinsics, which are also not safe. Callers must ensure that
the appropriate target features are enabled in the calling function,
and that the current CPU supports them. All implementations should
avoid marking the routines with `#[target_feature]` and instead mark
them as `#[inline(always)]` to ensure they get appropriately inlined.
(`inline(always)` cannot be used with target_feature.)

#### Associated Constants

- `const BITS: usize`

- `const BYTES: usize`

#### Required Methods

- `fn splat(byte: u8) -> Self`

  Create a vector with 8-bit lanes with the given byte repeated into each

- `fn load_unaligned(data: *const u8) -> Self`

  Read a vector-size number of bytes from the given pointer. The pointer

- `fn is_zero(self) -> bool`

  Returns true if and only if this vector has zero in all of its lanes.

- `fn cmpeq(self, vector2: Self) -> Self`

  Do an 8-bit pairwise equality check. If lane `i` is equal in this

- `fn and(self, vector2: Self) -> Self`

  Perform a bitwise 'and' of this vector and the one given and return

- `fn or(self, vector2: Self) -> Self`

  Perform a bitwise 'or' of this vector and the one given and return

- `fn shift_8bit_lane_right<const BITS: i32>(self) -> Self`

  Shift each 8-bit lane in this vector to the right by the number of

- `fn shift_in_one_byte(self, vector2: Self) -> Self`

  Shift this vector to the left by one byte and shift the most

- `fn shift_in_two_bytes(self, vector2: Self) -> Self`

  Shift this vector to the left by two bytes and shift the two most

- `fn shift_in_three_bytes(self, vector2: Self) -> Self`

  Shift this vector to the left by three bytes and shift the three most

- `fn shuffle_bytes(self, indices: Self) -> Self`

  Shuffles the bytes in this vector according to the indices in each of

- `fn for_each_64bit_lane<T>(self, f: impl FnMut(usize, u64) -> Option<T>) -> Option<T>`

  Call the provided function for each 64-bit lane in this vector. The

#### Implementors

- `__m128i`
- `__m256i`

### `FatVector`

```rust
trait FatVector: Vector { ... }
```

*Defined in [`aho-corasick-1.1.4/src/packed/vector.rs:232-318`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/vector.rs#L232-L318)*

This trait extends the `Vector` trait with additional operations to support
Fat Teddy.

Fat Teddy uses 16 buckets instead of 8, but reads half as many bytes (as
the vector size) instead of the full size of a vector per iteration. For
example, when using a 256-bit vector, Slim Teddy reads 32 bytes at a timr
but Fat Teddy reads 16 bytes at a time.

Fat Teddy is useful when searching for a large number of literals.
The extra number of buckets spreads the literals out more and reduces
verification time.

Currently we only implement this for AVX on x86_64. It would be nice to
implement this for SSE on x86_64 and NEON on aarch64, with the latter two
only reading 8 bytes at a time. It's not clear how well it would work, but
there are some tricky things to figure out in terms of implementation. The
`half_shift_in_{one,two,three}_bytes` methods in particular are probably
the trickiest of the bunch. For AVX2, these are implemented by taking
advantage of the fact that `_mm256_alignr_epi8` operates on each 128-bit
half instead of the full 256-bit vector. (Where as `_mm_alignr_epi8`
operates on the full 128-bit vector and not on each 64-bit half.) I didn't
do a careful survey of NEON to see if it could easily support these
operations.

#### Associated Types

- `type Half: 1`

#### Required Methods

- `fn load_half_unaligned(data: *const u8) -> Self`

  Read a half-vector-size number of bytes from the given pointer, and

- `fn half_shift_in_one_byte(self, vector2: Self) -> Self`

  Like `Vector::shift_in_one_byte`, except this is done for each half

- `fn half_shift_in_two_bytes(self, vector2: Self) -> Self`

  Like `Vector::shift_in_two_bytes`, except this is done for each half

- `fn half_shift_in_three_bytes(self, vector2: Self) -> Self`

  Like `Vector::shift_in_two_bytes`, except this is done for each half

- `fn swap_halves(self) -> Self`

  Swap the 128-bit lanes in this vector.

- `fn interleave_low_8bit_lanes(self, vector2: Self) -> Self`

  Unpack and interleave the 8-bit lanes from the low 128 bits of each

- `fn interleave_high_8bit_lanes(self, vector2: Self) -> Self`

  Unpack and interleave the 8-bit lanes from the high 128 bits of each

- `fn for_each_low_64bit_lane<T>(self, vector2: Self, f: impl FnMut(usize, u64) -> Option<T>) -> Option<T>`

  Call the provided function for each 64-bit lane in the lower half

#### Implementors

- `__m256i`

