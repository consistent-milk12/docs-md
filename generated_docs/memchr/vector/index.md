*[memchr](../index.md) / [vector](index.md)*

---

# Module `vector`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86sse2`](#x86sse2) | mod |  |
| [`x86avx2`](#x86avx2) | mod |  |
| [`SensibleMoveMask`](#sensiblemovemask) | struct | This is a "sensible" movemask implementation where each bit represents |
| [`Vector`](#vector) | trait | A trait for describing vector operations used by vectorized searchers. |
| [`MoveMask`](#movemask) | trait | A trait that abstracts over a vector-to-scalar operation called |

## Modules

- [`x86sse2`](x86sse2/index.md) - 
- [`x86avx2`](x86avx2/index.md) - 

## Structs

### `SensibleMoveMask`

```rust
struct SensibleMoveMask(u32);
```

This is a "sensible" movemask implementation where each bit represents
whether the most significant bit is set in each corresponding lane of a
vector. This is used on x86-64 and wasm, but such a mask is more expensive
to get on aarch64 so we use something a little different.

We call this "sensible" because this is what we get using native sse/avx
movemask instructions. But neon has no such native equivalent.

#### Implementations

- <span id="sensiblemovemask-get-for-offset"></span>`fn get_for_offset(self) -> u32`

#### Trait Implementations

##### `impl Clone for SensibleMoveMask`

- <span id="sensiblemovemask-clone"></span>`fn clone(&self) -> SensibleMoveMask` — [`SensibleMoveMask`](#sensiblemovemask)

##### `impl Copy for SensibleMoveMask`

##### `impl Debug for SensibleMoveMask`

- <span id="sensiblemovemask-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl MoveMask for SensibleMoveMask`

- <span id="sensiblemovemask-all-zeros-except-least-significant"></span>`fn all_zeros_except_least_significant(n: usize) -> SensibleMoveMask` — [`SensibleMoveMask`](#sensiblemovemask)

- <span id="sensiblemovemask-has-non-zero"></span>`fn has_non_zero(self) -> bool`

- <span id="sensiblemovemask-count-ones"></span>`fn count_ones(self) -> usize`

- <span id="sensiblemovemask-and"></span>`fn and(self, other: SensibleMoveMask) -> SensibleMoveMask` — [`SensibleMoveMask`](#sensiblemovemask)

- <span id="sensiblemovemask-or"></span>`fn or(self, other: SensibleMoveMask) -> SensibleMoveMask` — [`SensibleMoveMask`](#sensiblemovemask)

- <span id="sensiblemovemask-clear-least-significant-bit"></span>`fn clear_least_significant_bit(self) -> SensibleMoveMask` — [`SensibleMoveMask`](#sensiblemovemask)

- <span id="sensiblemovemask-first-offset"></span>`fn first_offset(self) -> usize`

- <span id="sensiblemovemask-last-offset"></span>`fn last_offset(self) -> usize`

## Traits

### `Vector`

```rust
trait Vector: Copy + core::fmt::Debug { ... }
```

A trait for describing vector operations used by vectorized searchers.

The trait is highly constrained to low level vector operations needed.
In general, it was invented mostly to be generic over x86's __m128i and
__m256i types. At time of writing, it also supports wasm and aarch64
128-bit vector types as well.

# Safety

All methods are not safe since they are intended to be implemented using
vendor intrinsics, which are also not safe. Callers must ensure that the
appropriate target features are enabled in the calling function, and that
the current CPU supports them. All implementations should avoid marking the
routines with #[target_feature] and instead mark them as #[inline(always)]
to ensure they get appropriately inlined. (inline(always) cannot be used
with target_feature.)

#### Required Methods

- `const BYTES: usize`

- `const ALIGN: usize`

- `type Mask: 1`

- `fn splat(byte: u8) -> Self`

  Create a vector with 8-bit lanes with the given byte repeated into each

- `fn load_aligned(data: *const u8) -> Self`

  Read a vector-size number of bytes from the given pointer. The pointer

- `fn load_unaligned(data: *const u8) -> Self`

  Read a vector-size number of bytes from the given pointer. The pointer

- `fn movemask(self) -> <Self as >::Mask`

  _mm_movemask_epi8 or _mm256_movemask_epi8

- `fn cmpeq(self, vector2: Self) -> Self`

  _mm_cmpeq_epi8 or _mm256_cmpeq_epi8

- `fn and(self, vector2: Self) -> Self`

  _mm_and_si128 or _mm256_and_si256

- `fn or(self, vector2: Self) -> Self`

  _mm_or or _mm256_or_si256

- `fn movemask_will_have_non_zero(self) -> bool`

  Returns true if and only if `Self::movemask` would return a mask that

### `MoveMask`

```rust
trait MoveMask: Copy + core::fmt::Debug { ... }
```

A trait that abstracts over a vector-to-scalar operation called
"move mask."

On x86-64, this is `_mm_movemask_epi8` for SSE2 and `_mm256_movemask_epi8`
for AVX2. It takes a vector of `u8` lanes and returns a scalar where the
`i`th bit is set if and only if the most significant bit in the `i`th lane
of the vector is set. The simd128 ISA for wasm32 also supports this
exact same operation natively.

... But aarch64 doesn't. So we have to fake it with more instructions and
a slightly different representation. We could do extra work to unify the
representations, but then would require additional costs in the hot path
for `memchr` and `packedpair`. So instead, we abstraction over the specific
representation with this trait and define the operations we actually need.

#### Required Methods

- `fn all_zeros_except_least_significant(n: usize) -> Self`

  Return a mask that is all zeros except for the least significant `n`

- `fn has_non_zero(self) -> bool`

  Returns true if and only if this mask has a a non-zero bit anywhere.

- `fn count_ones(self) -> usize`

  Returns the number of bits set to 1 in this mask.

- `fn and(self, other: Self) -> Self`

  Does a bitwise `and` operation between `self` and `other`.

- `fn or(self, other: Self) -> Self`

  Does a bitwise `or` operation between `self` and `other`.

- `fn clear_least_significant_bit(self) -> Self`

  Returns a mask that is equivalent to `self` but with the least

- `fn first_offset(self) -> usize`

  Returns the offset of the first non-zero lane this mask represents.

- `fn last_offset(self) -> usize`

  Returns the offset of the last non-zero lane this mask represents.

