*[aho_corasick](../../../index.md) / [packed](../../index.md) / [teddy](../index.md) / [generic](index.md)*

---

# Module `generic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Match`](#match) | struct | A match type specialized to the Teddy implementations below. |
| [`Slim`](#slim) | struct | A "slim" Teddy implementation that is generic over both the vector type and the minimum length of the patterns being searched for. |
| [`Fat`](#fat) | struct | A "fat" Teddy implementation that is generic over both the vector type and the minimum length of the patterns being searched for. |
| [`Teddy`](#teddy) | struct | The common elements of all "slim" and "fat" Teddy search implementations. |
| [`Mask`](#mask) | struct | A vector generic mask for the low and high nybbles in a set of patterns. |
| [`SlimMaskBuilder`](#slimmaskbuilder) | struct | Represents the low and high nybble masks that will be used during search. |
| [`FatMaskBuilder`](#fatmaskbuilder) | struct | Represents the low and high nybble masks that will be used during "fat" Teddy search. |

## Structs

### `Match`

```rust
struct Match {
    pid: crate::PatternID,
    start: *const u8,
    end: *const u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:26-30`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L26-L30)*

A match type specialized to the Teddy implementations below.

Essentially, instead of representing a match at byte offsets, we use
raw pointers. This is because the implementations below operate on raw
pointers, and so this is a more natural return type based on how the
implementation works.

Also, the `PatternID` used here is a `u16`.

#### Implementations

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../../../util/primitives/index.md#patternid)

  Returns the ID of the pattern that matched.

- <span id="match-start"></span>`fn start(&self) -> *const u8`

  Returns a pointer into the haystack at which the match starts.

- <span id="match-end"></span>`fn end(&self) -> *const u8`

  Returns a pointer into the haystack at which the match ends.

#### Trait Implementations

##### `impl Any for Match`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](#match)

##### `impl CloneToUninit for Match`

- <span id="match-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Match`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Match`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Match`

- <span id="match-toowned-type-owned"></span>`type Owned = T`

- <span id="match-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="match-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Match`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Slim<V, const BYTES: usize>`

```rust
struct Slim<V, const BYTES: usize> {
    teddy: Teddy<8>,
    masks: [Mask<V>; BYTES],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:54-60`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L54-L60)*

A "slim" Teddy implementation that is generic over both the vector type
and the minimum length of the patterns being searched for.

Only 1, 2, 3 and 4 bytes are supported as minimum lengths.

#### Fields

- **`teddy`**: `Teddy<8>`

  A generic data structure for doing "slim" Teddy verification.

- **`masks`**: `[Mask<V>; BYTES]`

  The masks used as inputs to the shuffle operation to generate
  candidates (which are fed into the verification routines).

#### Implementations

- <span id="slim-new"></span>`unsafe fn new(patterns: Arc<Patterns>) -> Slim<V, BYTES>` — [`Patterns`](../../pattern/index.md#patterns), [`Slim`](#slim)

  Create a new "slim" Teddy searcher for the given patterns.

  

  # Panics

  

  This panics when `BYTES` is any value other than 1, 2, 3 or 4.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

- <span id="slim-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this type, in

  units of bytes.

- <span id="slim-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the minimum length, in bytes, that a haystack must be in order

  to use it with this searcher.

#### Trait Implementations

##### `impl Any for Slim<V, BYTES>`

- <span id="slim-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Slim<V, BYTES>`

- <span id="slim-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Slim<V, BYTES>`

- <span id="slim-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V: clone::Clone> Clone for Slim<V, BYTES>`

- <span id="slim-clone"></span>`fn clone(&self) -> Slim<V, BYTES>` — [`Slim`](#slim)

##### `impl CloneToUninit for Slim<V, BYTES>`

- <span id="slim-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<V: fmt::Debug> Debug for Slim<V, BYTES>`

- <span id="slim-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Slim<V, BYTES>`

- <span id="slim-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Slim<V, BYTES>`

- <span id="slim-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Slim<V, BYTES>`

- <span id="slim-toowned-type-owned"></span>`type Owned = T`

- <span id="slim-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slim-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Slim<V, BYTES>`

- <span id="slim-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slim-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Slim<V, BYTES>`

- <span id="slim-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slim-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fat<V, const BYTES: usize>`

```rust
struct Fat<V, const BYTES: usize> {
    teddy: Teddy<16>,
    masks: [Mask<V>; BYTES],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:387-393`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L387-L393)*

A "fat" Teddy implementation that is generic over both the vector type
and the minimum length of the patterns being searched for.

Only 1, 2, 3 and 4 bytes are supported as minimum lengths.

#### Fields

- **`teddy`**: `Teddy<16>`

  A generic data structure for doing "fat" Teddy verification.

- **`masks`**: `[Mask<V>; BYTES]`

  The masks used as inputs to the shuffle operation to generate
  candidates (which are fed into the verification routines).

#### Implementations

- <span id="fat-new"></span>`unsafe fn new(patterns: Arc<Patterns>) -> Fat<V, BYTES>` — [`Patterns`](../../pattern/index.md#patterns), [`Fat`](#fat)

  Create a new "fat" Teddy searcher for the given patterns.

  

  # Panics

  

  This panics when `BYTES` is any value other than 1, 2, 3 or 4.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

- <span id="fat-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this type, in

  units of bytes.

- <span id="fat-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the minimum length, in bytes, that a haystack must be in order

  to use it with this searcher.

#### Trait Implementations

##### `impl Any for Fat<V, BYTES>`

- <span id="fat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fat<V, BYTES>`

- <span id="fat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fat<V, BYTES>`

- <span id="fat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V: clone::Clone> Clone for Fat<V, BYTES>`

- <span id="fat-clone"></span>`fn clone(&self) -> Fat<V, BYTES>` — [`Fat`](#fat)

##### `impl CloneToUninit for Fat<V, BYTES>`

- <span id="fat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<V: fmt::Debug> Debug for Fat<V, BYTES>`

- <span id="fat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Fat<V, BYTES>`

- <span id="fat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fat<V, BYTES>`

- <span id="fat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Fat<V, BYTES>`

- <span id="fat-toowned-type-owned"></span>`type Owned = T`

- <span id="fat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fat<V, BYTES>`

- <span id="fat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fat<V, BYTES>`

- <span id="fat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Teddy<const BUCKETS: usize>`

```rust
struct Teddy<const BUCKETS: usize> {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    buckets: [alloc::vec::Vec<crate::PatternID>; BUCKETS],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:728-747`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L728-L747)*

The common elements of all "slim" and "fat" Teddy search implementations.

Essentially, this contains the patterns and the buckets. Namely, it
contains enough to implement the verification step after candidates are
identified via the shuffle masks.

It is generic over the number of buckets used. In general, the number of
buckets is either 8 (for "slim" Teddy) or 16 (for "fat" Teddy). The generic
parameter isn't really meant to be instantiated for any value other than
8 or 16, although it is technically possible. The main hiccup is that there
is some bit-shifting done in the critical part of verification that could
be quite expensive if `N` is not a multiple of 2.

#### Fields

- **`patterns`**: `alloc::sync::Arc<crate::packed::pattern::Patterns>`

  The patterns we are searching for.
  
  A pattern string can be found by its `PatternID`.

- **`buckets`**: `[alloc::vec::Vec<crate::PatternID>; BUCKETS]`

  The allocation of patterns in buckets. This only contains the IDs of
  patterns. In order to do full verification, callers must provide the
  actual patterns when using Teddy.

#### Implementations

- <span id="teddy-new"></span>`fn new(patterns: Arc<Patterns>) -> Teddy<BUCKETS>` — [`Patterns`](../../pattern/index.md#patterns), [`Teddy`](#teddy)

  Create a new generic data structure for Teddy verification.

- <span id="teddy-verify64"></span>`unsafe fn verify64(&self, cur: *const u8, end: *const u8, candidate_chunk: u64) -> Option<Match>` — [`Match`](#match)

  Verify whether there are any matches starting at or after `cur` in the

  haystack. The candidate chunk given should correspond to 8-bit bitsets

  for N buckets.

  

  # Safety

  

  The given pointers representing the haystack must be valid to read

  from.

- <span id="teddy-verify-bucket"></span>`unsafe fn verify_bucket(&self, cur: *const u8, end: *const u8, bucket: usize) -> Option<Match>` — [`Match`](#match)

  Verify whether there are any matches starting at `at` in the given

  `haystack` corresponding only to patterns in the given bucket.

  

  # Safety

  

  The given pointers representing the haystack must be valid to read

  from.

  

  The bucket index must be less than or equal to `self.buckets.len()`.

- <span id="teddy-mask-len"></span>`fn mask_len(&self) -> usize`

  Returns the total number of masks required by the patterns in this

  Teddy searcher.

  

  Basically, the mask length corresponds to the type of Teddy searcher

  to use: a 1-byte, 2-byte, 3-byte or 4-byte searcher. The bigger the

  better, typically, since searching for longer substrings usually

  decreases the rate of false positives. Therefore, the number of masks

  needed is the length of the shortest pattern in this searcher. If the

  length of the shortest pattern (in bytes) is bigger than 4, then the

  mask length is 4 since there are no Teddy searchers for more than 4

  bytes.

- <span id="teddy-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this type, in

  units of bytes.

#### Trait Implementations

##### `impl Any for Teddy<BUCKETS>`

- <span id="teddy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Teddy<BUCKETS>`

- <span id="teddy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Teddy<BUCKETS>`

- <span id="teddy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Teddy<BUCKETS>`

- <span id="teddy-clone"></span>`fn clone(&self) -> Teddy<BUCKETS>` — [`Teddy`](#teddy)

##### `impl CloneToUninit for Teddy<BUCKETS>`

- <span id="teddy-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Teddy<BUCKETS>`

- <span id="teddy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Teddy<BUCKETS>`

- <span id="teddy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Teddy<BUCKETS>`

- <span id="teddy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Teddy<BUCKETS>`

- <span id="teddy-toowned-type-owned"></span>`type Owned = T`

- <span id="teddy-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="teddy-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Teddy<BUCKETS>`

- <span id="teddy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="teddy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Teddy<BUCKETS>`

- <span id="teddy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="teddy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mask<V>`

```rust
struct Mask<V> {
    lo: V,
    hi: V,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1016-1019`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1016-L1019)*

A vector generic mask for the low and high nybbles in a set of patterns.
Each 8-bit lane `j` in a vector corresponds to a bitset where the `i`th bit
is set if and only if the nybble `j` is in the bucket `i` at a particular
position.

This is slightly tweaked dependending on whether Slim or Fat Teddy is being
used. For Slim Teddy, the bitsets in the lower half are the same as the
bitsets in the higher half, so that we can search `V::BYTES` bytes at a
time. (Remember, the nybbles in the haystack are used as indices into these
masks, and 256-bit shuffles only operate on 128-bit lanes.)

For Fat Teddy, the bitsets are not repeated, but instead, the high half
bits correspond to an addition 8 buckets. So that a bitset `00100010` has
buckets 1 and 5 set if it's in the lower half, but has buckets 9 and 13 set
if it's in the higher half.

#### Implementations

- <span id="mask-members1"></span>`unsafe fn members1(chunk: V, masks: [Mask<V>; 1]) -> V` — [`Mask`](#mask)

  Return a candidate for Teddy (fat or slim) that is searching for 1-byte

  candidates.

  

  If a candidate is returned, it will be a collection of 8-bit bitsets

  (one bitset per lane), where the ith bit is set in the jth lane if and

  only if the byte occurring at the jth lane in `chunk` is in the bucket

  `i`. If no candidate is found, then the vector returned will have all

  lanes set to zero.

  

  `chunk` should correspond to a `V::BYTES` window of the haystack (where

  the least significant byte corresponds to the start of the window). For

  fat Teddy, the haystack window length should be `V::BYTES / 2`, with

  the window repeated in each half of the vector.

  

  `mask1` should correspond to a low/high mask for the first byte of all

  patterns that are being searched.

- <span id="mask-members2"></span>`unsafe fn members2(chunk: V, masks: [Mask<V>; 2]) -> (V, V)` — [`Mask`](#mask)

  Return a candidate for Teddy (fat or slim) that is searching for 2-byte

  candidates.

  

  If candidates are returned, each will be a collection of 8-bit bitsets

  (one bitset per lane), where the ith bit is set in the jth lane if and

  only if the byte occurring at the jth lane in `chunk` is in the bucket

  `i`. Each candidate returned corresponds to the first and second bytes

  of the patterns being searched. If no candidate is found, then all of

  the lanes will be set to zero in at least one of the vectors returned.

  

  `chunk` should correspond to a `V::BYTES` window of the haystack (where

  the least significant byte corresponds to the start of the window). For

  fat Teddy, the haystack window length should be `V::BYTES / 2`, with

  the window repeated in each half of the vector.

  

  The masks should correspond to the masks computed for the first and

  second bytes of all patterns that are being searched.

- <span id="mask-members3"></span>`unsafe fn members3(chunk: V, masks: [Mask<V>; 3]) -> (V, V, V)` — [`Mask`](#mask)

  Return a candidate for Teddy (fat or slim) that is searching for 3-byte

  candidates.

  

  If candidates are returned, each will be a collection of 8-bit bitsets

  (one bitset per lane), where the ith bit is set in the jth lane if and

  only if the byte occurring at the jth lane in `chunk` is in the bucket

  `i`. Each candidate returned corresponds to the first, second and third

  bytes of the patterns being searched. If no candidate is found, then

  all of the lanes will be set to zero in at least one of the vectors

  returned.

  

  `chunk` should correspond to a `V::BYTES` window of the haystack (where

  the least significant byte corresponds to the start of the window). For

  fat Teddy, the haystack window length should be `V::BYTES / 2`, with

  the window repeated in each half of the vector.

  

  The masks should correspond to the masks computed for the first, second

  and third bytes of all patterns that are being searched.

- <span id="mask-members4"></span>`unsafe fn members4(chunk: V, masks: [Mask<V>; 4]) -> (V, V, V, V)` — [`Mask`](#mask)

  Return a candidate for Teddy (fat or slim) that is searching for 4-byte

  candidates.

  

  If candidates are returned, each will be a collection of 8-bit bitsets

  (one bitset per lane), where the ith bit is set in the jth lane if and

  only if the byte occurring at the jth lane in `chunk` is in the bucket

  `i`. Each candidate returned corresponds to the first, second, third

  and fourth bytes of the patterns being searched. If no candidate is

  found, then all of the lanes will be set to zero in at least one of the

  vectors returned.

  

  `chunk` should correspond to a `V::BYTES` window of the haystack (where

  the least significant byte corresponds to the start of the window). For

  fat Teddy, the haystack window length should be `V::BYTES / 2`, with

  the window repeated in each half of the vector.

  

  The masks should correspond to the masks computed for the first,

  second, third and fourth bytes of all patterns that are being searched.

#### Trait Implementations

##### `impl Any for Mask<V>`

- <span id="mask-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mask<V>`

- <span id="mask-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mask<V>`

- <span id="mask-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V: clone::Clone> Clone for Mask<V>`

- <span id="mask-clone"></span>`fn clone(&self) -> Mask<V>` — [`Mask`](#mask)

##### `impl CloneToUninit for Mask<V>`

- <span id="mask-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<V: marker::Copy> Copy for Mask<V>`

##### `impl<V: fmt::Debug> Debug for Mask<V>`

- <span id="mask-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Mask<V>`

- <span id="mask-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Mask<V>`

- <span id="mask-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Mask<V>`

- <span id="mask-toowned-type-owned"></span>`type Owned = T`

- <span id="mask-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mask-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Mask<V>`

- <span id="mask-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mask-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mask<V>`

- <span id="mask-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mask-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlimMaskBuilder`

```rust
struct SlimMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1178-1181`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1178-L1181)*

Represents the low and high nybble masks that will be used during
search. Each mask is 32 bytes wide, although only the first 16 bytes are
used for 128-bit vectors.

Each byte in the mask corresponds to a 8-bit bitset, where bit `i` is set
if and only if the corresponding nybble is in the ith bucket. The index of
the byte (0-15, inclusive) corresponds to the nybble.

Each mask is used as the target of a shuffle, where the indices for the
shuffle are taken from the haystack. AND'ing the shuffles for both the
low and high masks together also results in 8-bit bitsets, but where bit
`i` is set if and only if the correspond *byte* is in the ith bucket.

#### Implementations

- <span id="slimmaskbuilder-add"></span>`fn add(&mut self, bucket: usize, byte: u8)`

  Update this mask by adding the given byte to the given bucket. The

  given bucket must be in the range 0-7.

  

  # Panics

  

  When `bucket >= 8`.

- <span id="slimmaskbuilder-build"></span>`unsafe fn build<V: Vector>(&self) -> Mask<V>` — [`Mask`](#mask)

  Turn this builder into a vector mask.

  

  # Panics

  

  When `V` represents a vector bigger than what `MaskBytes` can contain.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

- <span id="slimmaskbuilder-from-teddy"></span>`unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<8>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

  A convenience function for building `N` vector masks from a slim

  `Teddy` value.

  

  # Panics

  

  When `V` represents a vector bigger than what `MaskBytes` can contain.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

#### Trait Implementations

##### `impl Any for SlimMaskBuilder`

- <span id="slimmaskbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlimMaskBuilder`

- <span id="slimmaskbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlimMaskBuilder`

- <span id="slimmaskbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SlimMaskBuilder`

- <span id="slimmaskbuilder-clone"></span>`fn clone(&self) -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

##### `impl CloneToUninit for SlimMaskBuilder`

- <span id="slimmaskbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SlimMaskBuilder`

- <span id="slimmaskbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for SlimMaskBuilder`

- <span id="slimmaskbuilder-default"></span>`fn default() -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

##### `impl<T> From for SlimMaskBuilder`

- <span id="slimmaskbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlimMaskBuilder`

- <span id="slimmaskbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SlimMaskBuilder`

- <span id="slimmaskbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="slimmaskbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slimmaskbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SlimMaskBuilder`

- <span id="slimmaskbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slimmaskbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlimMaskBuilder`

- <span id="slimmaskbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slimmaskbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatMaskBuilder`

```rust
struct FatMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1288-1291`](../../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1288-L1291)*

Represents the low and high nybble masks that will be used during "fat"
Teddy search.

Each mask is 32 bytes wide, and at the time of writing, only 256-bit vectors
support fat Teddy.

A fat Teddy mask is like a slim Teddy mask, except that instead of
repeating the bitsets in the high and low 128-bits in 256-bit vectors, the
high and low 128-bit halves each represent distinct buckets. (Bringing the
total to 16 instead of 8.) This permits spreading the patterns out a bit
more and thus putting less pressure on verification to be fast.

Each byte in the mask corresponds to a 8-bit bitset, where bit `i` is set
if and only if the corresponding nybble is in the ith bucket. The index of
the byte (0-15, inclusive) corresponds to the nybble.

#### Implementations

- <span id="fatmaskbuilder-add"></span>`fn add(&mut self, bucket: usize, byte: u8)`

  Update this mask by adding the given byte to the given bucket. The

  given bucket must be in the range 0-15.

  

  # Panics

  

  When `bucket >= 16`.

- <span id="fatmaskbuilder-build"></span>`unsafe fn build<V: Vector>(&self) -> Mask<V>` — [`Mask`](#mask)

  Turn this builder into a vector mask.

  

  # Panics

  

  When `V` represents a vector bigger than what `MaskBytes` can contain.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

- <span id="fatmaskbuilder-from-teddy"></span>`unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<16>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

  A convenience function for building `N` vector masks from a fat

  `Teddy` value.

  

  # Panics

  

  When `V` represents a vector bigger than what `MaskBytes` can contain.

  

  # Safety

  

  Callers must ensure that this is okay to call in the current target for

  the current CPU.

#### Trait Implementations

##### `impl Any for FatMaskBuilder`

- <span id="fatmaskbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatMaskBuilder`

- <span id="fatmaskbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatMaskBuilder`

- <span id="fatmaskbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatMaskBuilder`

- <span id="fatmaskbuilder-clone"></span>`fn clone(&self) -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

##### `impl CloneToUninit for FatMaskBuilder`

- <span id="fatmaskbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FatMaskBuilder`

##### `impl Debug for FatMaskBuilder`

- <span id="fatmaskbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for FatMaskBuilder`

- <span id="fatmaskbuilder-default"></span>`fn default() -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

##### `impl<T> From for FatMaskBuilder`

- <span id="fatmaskbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FatMaskBuilder`

- <span id="fatmaskbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for FatMaskBuilder`

- <span id="fatmaskbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="fatmaskbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatmaskbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FatMaskBuilder`

- <span id="fatmaskbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatmaskbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatMaskBuilder`

- <span id="fatmaskbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatmaskbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

