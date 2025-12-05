*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [twoway](index.md)*

---

# Module `twoway`

An implementation of the [Two-Way substring search algorithm][two-way].

[`Finder`](../packedpair/index.md) can be built for forward searches, while [`FinderRev`](../../../memmem/index.md) can be built
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

[two-way]: https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm
[GNU libc]: https://www.gnu.org/software/libc/
[musl](#musl): https://www.musl-libc.org/

## Structs

### `Finder`

```rust
struct Finder();
```

A forward substring searcher that uses the Two-Way algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> Finder`
  Create a searcher that finds occurrences of the given `needle`.

- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`
  Returns the first occurrence of `needle` in the given `haystack`, or

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

### `FinderRev`

```rust
struct FinderRev();
```

A reverse substring searcher that uses the Two-Way algorithm.

#### Implementations

- `fn new(needle: &[u8]) -> FinderRev`
  Create a searcher that finds occurrences of the given `needle`.

- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`
  Returns the last occurrence of `needle` in the given `haystack`, or

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

- `fn clone(self: &Self) -> FinderRev`

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

