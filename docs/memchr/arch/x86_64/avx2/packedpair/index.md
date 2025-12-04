*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [avx2](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

A 256-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last

## Structs

### `Finder`

```rust
struct Finder {
}
```

A "packed pair" finder that uses 256-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`](memchr/arch/all/packedpair/index.md) of bytes given matches.

#### Implementations

- `fn new(needle: &[u8]) -> Option<Finder>`
  Create a new pair searcher. The searcher returned can either report

- `fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>`
  Create a new "packed pair" finder using the pair of bytes given.

- `fn is_available() -> bool`
  Returns true when this implementation is available in the current

- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`
  Execute a search using AVX2 vectors and routines.

- `fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize>`
  Run this finder on the given haystack as a prefilter.

- `fn pair(self: &Self) -> &Pair`
  Returns the pair of offsets (into the needle) used to check as a

- `fn min_haystack_len(self: &Self) -> usize`
  Returns the minimum haystack length that this `Finder` can search.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Finder`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

