*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [twoway](index.md)*

---

# Module `twoway`

An implementation of the [Two-Way substring search algorithm][two-way].

[`Finder`](../rabinkarp/index.md) can be built for forward searches, while [`FinderRev`](../rabinkarp/index.md) can be built
for reverse searches.

Two-Way makes for a nice general purpose substring search algorithm because of
its time and space complexity properties. It also performs well in practice.
Namely, with `m = len(needle)` and `n = len(haystack)`, Two-Way takes `O(m)`
time to create a finder, `O(1)` space and `O(n)` search time. In other words,
the preprocessing step is quick, doesn't require any heap memory and the worst
case search time is guaranteed to be linear in the haystack regardless of the
size of the needle.

While vector algorithms will usually beat Two-Way handedly, vector algorithms
also usually have pathological or edge cases that are better handled by Two-Way.
Moreover, not all targets support vector algorithms or implementations for them
simply may not exist yet.

Two-Way can be found in the `memmem` implementations in at least [GNU libc] and
[musl](#musl).




## Structs

### `Finder`

```rust
struct Finder(TwoWay);
```

A forward substring searcher that uses the Two-Way algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> Finder` — [`Finder`](#finder)

- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `fn find_with_prefilter(self: &Self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8]) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

- `fn find_small_imp(self: &Self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8], period: usize) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

- `fn find_large_imp(self: &Self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8], shift: usize) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

#### Trait Implementations

##### `impl Clone for Finder`

- `fn clone(self: &Self) -> Finder` — [`Finder`](#finder)

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderRev`

```rust
struct FinderRev(TwoWay);
```

A reverse substring searcher that uses the Two-Way algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> FinderRev` — [`FinderRev`](#finderrev)

- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `fn rfind_small_imp(self: &Self, haystack: &[u8], needle: &[u8], period: usize) -> Option<usize>`

- `fn rfind_large_imp(self: &Self, haystack: &[u8], needle: &[u8], shift: usize) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for FinderRev`

- `fn clone(self: &Self) -> FinderRev` — [`FinderRev`](#finderrev)

##### `impl Copy for FinderRev`

##### `impl Debug for FinderRev`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

