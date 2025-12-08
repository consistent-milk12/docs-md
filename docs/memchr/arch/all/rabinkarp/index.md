*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [rabinkarp](index.md)*

---

# Module `rabinkarp`

An implementation of the [Rabin-Karp substring search algorithm][rabinkarp](#rabinkarp).

Rabin-Karp works by creating a hash of the needle provided and then computing
a rolling hash for each needle sized window in the haystack. When the rolling
hash matches the hash of the needle, a byte-wise comparison is done to check
if a match exists. The worst case time complexity of Rabin-Karp is `O(m *
n)` where `m ~ len(needle)` and `n ~ len(haystack)`. Its worst case space
complexity is constant.

The main utility of Rabin-Karp is that the searcher can be constructed very
quickly with very little memory. This makes it especially useful when searching
for small needles in small haystacks, as it might finish its search before a
beefier algorithm (like Two-Way) even starts.


## Structs

### `Finder`

```rust
struct Finder {
    hash: Hash,
    hash_2pow: u32,
}
```

A forward substring searcher using the Rabin-Karp algorithm.

Note that, as a lower level API, a `Finder` does not have access to the
needle it was constructed with. For this reason, executing a search
with a `Finder` requires passing both the needle and the haystack,
where the needle is exactly equivalent to the one given to the `Finder`
at construction time. This design was chosen so that callers can have
more precise control over where and how many times a needle is stored.
For example, in cases where Rabin-Karp is just one of several possible
substring search algorithms.

#### Fields

- **`hash`**: `Hash`

  The actual hash.

- **`hash_2pow`**: `u32`

  The factor needed to multiply a byte by in order to subtract it from
  the hash. It is defined to be 2^(n-1) (using wrapping exponentiation),
  where n is the length of the needle. This is how we "remove" a byte
  from the hash once the hash window rolls past it.

#### Implementations

- `fn new(needle: &[u8]) -> Finder` — [`Finder`](#finder)

- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `unsafe fn find_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>`

#### Trait Implementations

##### `impl Clone for Finder`

- `fn clone(self: &Self) -> Finder` — [`Finder`](#finder)

##### `impl Debug for Finder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderRev`

```rust
struct FinderRev(Finder);
```

A reverse substring searcher using the Rabin-Karp algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> FinderRev` — [`FinderRev`](#finderrev)

- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `unsafe fn rfind_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>`

#### Trait Implementations

##### `impl Clone for FinderRev`

- `fn clone(self: &Self) -> FinderRev` — [`FinderRev`](#finderrev)

##### `impl Debug for FinderRev`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Hash`

```rust
struct Hash(u32);
```

A Rabin-Karp hash. This might represent the hash of a needle, or the hash
of a rolling window in the haystack.

#### Implementations

- `fn new() -> Hash` — [`Hash`](#hash)

- `unsafe fn forward(start: *const u8, end: *const u8) -> Hash` — [`Hash`](#hash)

- `unsafe fn reverse(start: *const u8, end: *const u8) -> Hash` — [`Hash`](#hash)

- `fn roll(self: &mut Self, finder: &Finder, old: u8, new: u8)` — [`Finder`](#finder)

- `fn add(self: &mut Self, byte: u8)`

- `fn del(self: &mut Self, finder: &Finder, byte: u8)` — [`Finder`](#finder)

#### Trait Implementations

##### `impl Clone for Hash`

- `fn clone(self: &Self) -> Hash` — [`Hash`](#hash)

##### `impl Copy for Hash`

##### `impl Debug for Hash`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Hash`

- `fn default() -> Hash` — [`Hash`](#hash)

##### `impl Eq for Hash`

##### `impl PartialEq for Hash`

- `fn eq(self: &Self, other: &Hash) -> bool` — [`Hash`](#hash)

##### `impl StructuralPartialEq for Hash`

## Functions

### `is_fast`

```rust
fn is_fast(haystack: &[u8], _needle: &[u8]) -> bool
```

Whether RK is believed to be very fast for the given needle/haystack.

### `is_equal_raw`

```rust
unsafe fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool
```

Returns true when `x[i] == y[i]` for all `0 <= i < n`.

We forcefully don't inline this to hint at the compiler that it is unlikely
to be called. This causes the inner rabinkarp loop above to be a bit
tighter and leads to some performance improvement. See the
memmem/krate/prebuilt/sliceslice-words/words benchmark.

# Safety

Same as `crate::arch::all::is_equal_raw`.

