*[aho_corasick](../../index.md) / [packed](../index.md) / [rabinkarp](index.md)*

---

# Module `rabinkarp`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RabinKarp`](#rabinkarp) | struct | An implementation of the Rabin-Karp algorithm. |
| [`Hash`](#hash) | type | The type of the rolling hash used in the Rabin-Karp algorithm. |
| [`NUM_BUCKETS`](#num-buckets) | const | The number of buckets to store our patterns in. |

## Structs

### `RabinKarp`

```rust
struct RabinKarp {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    buckets: alloc::vec::Vec<alloc::vec::Vec<(usize, crate::PatternID)>>,
    hash_len: usize,
    hash_2pow: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/rabinkarp.rs:36-53`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/rabinkarp.rs#L36-L53)*

An implementation of the Rabin-Karp algorithm. The main idea of this
algorithm is to maintain a rolling hash as it moves through the input, and
then check whether that hash corresponds to the same hash for any of the
patterns we're looking for.

A draw back of naively scaling Rabin-Karp to multiple patterns is that
it requires all of the patterns to be the same length, which in turn
corresponds to the number of bytes to hash. We adapt this to work for
multiple patterns of varying size by fixing the number of bytes to hash
to be the length of the smallest pattern. We also split the patterns into
several buckets to hopefully make the confirmation step faster.

Wikipedia has a decent explanation, if a bit heavy on the theory:
https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm

But ESMAJ provides something a bit more concrete:
https://www-igm.univ-mlv.fr/~lecroq/string/node5.html

#### Fields

- **`patterns`**: `alloc::sync::Arc<crate::packed::pattern::Patterns>`

  The patterns we're searching for.

- **`buckets`**: `alloc::vec::Vec<alloc::vec::Vec<(usize, crate::PatternID)>>`

  The order of patterns in each bucket is significant. Namely, they are
  arranged such that the first one to match is the correct match. This
  may not necessarily correspond to the order provided by the caller.
  For example, if leftmost-longest semantics are used, then the patterns
  are sorted by their length in descending order. If leftmost-first
  semantics are used, then the patterns are sorted by their pattern ID
  in ascending order (which corresponds to the caller's order).

- **`hash_len`**: `usize`

  The length of the hashing window. Generally, this corresponds to the
  length of the smallest pattern.

- **`hash_2pow`**: `usize`

  The factor to subtract out of a hash before updating it with a new
  byte.

#### Implementations

- <span id="rabinkarp-new"></span>`fn new(patterns: &Arc<Patterns>) -> RabinKarp` — [`Patterns`](../pattern/index.md#patterns), [`RabinKarp`](#rabinkarp)

  Compile a new Rabin-Karp matcher from the patterns given.

  

  This panics if any of the patterns in the collection are empty, or if

  the collection is itself empty.

- <span id="rabinkarp-find-at"></span>`fn find_at(&self, haystack: &[u8], at: usize) -> Option<Match>` — [`Match`](../../util/search/index.md#match)

  Return the first matching pattern in the given haystack, begining the

  search at `at`.

- <span id="rabinkarp-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this searcher, in

  units of bytes.

- <span id="rabinkarp-verify"></span>`fn verify(&self, id: PatternID, haystack: &[u8], at: usize) -> Option<Match>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Match`](../../util/search/index.md#match)

  Verify whether the pattern with the given id matches at

  `haystack[at..]`.

  

  We tag this function as `cold` because it helps improve codegen.

  Intuitively, it would seem like inlining it would be better. However,

  the only time this is called and a match is not found is when there

  there is a hash collision, or when a prefix of a pattern matches but

  the entire pattern doesn't match. This is hopefully fairly rare, and

  if it does occur a lot, it's going to be slow no matter what we do.

- <span id="rabinkarp-hash"></span>`fn hash(&self, bytes: &[u8]) -> usize`

  Hash the given bytes.

- <span id="rabinkarp-update-hash"></span>`fn update_hash(&self, prev: usize, old_byte: u8, new_byte: u8) -> usize`

  Update the hash given based on removing `old_byte` at the beginning

  of some byte string, and appending `new_byte` to the end of that same

  byte string.

#### Trait Implementations

##### `impl Any for RabinKarp`

- <span id="rabinkarp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RabinKarp`

- <span id="rabinkarp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RabinKarp`

- <span id="rabinkarp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RabinKarp`

- <span id="rabinkarp-clone"></span>`fn clone(&self) -> RabinKarp` — [`RabinKarp`](#rabinkarp)

##### `impl CloneToUninit for RabinKarp`

- <span id="rabinkarp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RabinKarp`

- <span id="rabinkarp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RabinKarp`

- <span id="rabinkarp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RabinKarp`

- <span id="rabinkarp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RabinKarp`

- <span id="rabinkarp-toowned-type-owned"></span>`type Owned = T`

- <span id="rabinkarp-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rabinkarp-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RabinKarp`

- <span id="rabinkarp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rabinkarp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RabinKarp`

- <span id="rabinkarp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rabinkarp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `Hash`

```rust
type Hash = usize;
```

*Defined in [`aho-corasick-1.1.4/src/packed/rabinkarp.rs:6`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/rabinkarp.rs#L6)*

The type of the rolling hash used in the Rabin-Karp algorithm.

## Constants

### `NUM_BUCKETS`
```rust
const NUM_BUCKETS: usize = 64usize;
```

*Defined in [`aho-corasick-1.1.4/src/packed/rabinkarp.rs:16`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/rabinkarp.rs#L16)*

The number of buckets to store our patterns in. We don't want this to be
too big in order to avoid wasting memory, but we don't want it to be too
small either to avoid spending too much time confirming literals.

The number of buckets MUST be a power of two. Otherwise, determining the
bucket from a hash will slow down the code considerably. Using a power
of two means `hash % NUM_BUCKETS` can compile down to a simple `and`
instruction.

