*[memchr](../../../index.md) / [arch](../../index.md) / [generic](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

Generic crate-internal routines for the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A generic architecture dependent "packed pair" finder. |
| [`matched`](#matched) | fn | Accepts a chunk-relative offset and returns a haystack relative offset. |

## Structs

### `Finder<V>`

```rust
struct Finder<V> {
    pair: crate::arch::all::packedpair::Pair,
    v1: V,
    v2: V,
    min_haystack_len: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/generic/packedpair.rs:35-40`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/generic/packedpair.rs#L35-L40)*

A generic architecture dependent "packed pair" finder.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`](../../all/packedpair/index.md) of bytes given matches.

This is architecture dependent because it uses specific vector operations
to look for occurrences of the pair of bytes.

This type is not meant to be exported and is instead meant to be used as
the implementation for architecture specific facades. Why? Because it's a
bit of a quirky API that requires `inline(always)` annotations. And pretty
much everything has safety obligations due (at least) to the caller needing
to inline calls into routines marked with
`#[target_feature(enable = "...")]`.

#### Implementations

- <span id="finder-new"></span>`unsafe fn new(needle: &[u8], pair: Pair) -> Finder<V>` — [`Pair`](../../all/packedpair/index.md#pair), [`Finder`](#finder)

  Create a new pair searcher. The searcher returned can either report

  exact matches of `needle` or act as a prefilter and report candidate

  positions of `needle`.

  

  # Safety

  

  Callers must ensure that whatever vector type this routine is called

  with is supported by the current environment.

  

  Callers must also ensure that `needle.len() >= 2`.

- <span id="finder-find"></span>`unsafe fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Searches the given haystack for the given needle. The needle given

  should be the same as the needle that this finder was initialized

  with.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

  

  # Safety

  

  Since this is meant to be used with vector functions, callers need to

  specialize this inside of a function with a `target_feature` attribute.

  Therefore, callers must ensure that whatever target feature is being

  used supports the vector functions that this function is specialized

  for. (For the specific vector functions used, see the Vector trait

  implementations.)

- <span id="finder-find-prefilter"></span>`unsafe fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

  Searches the given haystack for offsets that represent candidate

  matches of the `needle` given to this finder's constructor. The offsets

  returned, if they are a match, correspond to the starting offset of

  `needle` in the given `haystack`.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

  

  # Safety

  

  Since this is meant to be used with vector functions, callers need to

  specialize this inside of a function with a `target_feature` attribute.

  Therefore, callers must ensure that whatever target feature is being

  used supports the vector functions that this function is specialized

  for. (For the specific vector functions used, see the Vector trait

  implementations.)

- <span id="finder-find-in-chunk"></span>`unsafe fn find_in_chunk(&self, needle: &[u8], cur: *const u8, end: *const u8, mask: <V as >::Mask) -> Option<usize>` — [`Vector`](../../../vector/index.md#vector)

  Search for an occurrence of our byte pair from the needle in the chunk

  pointed to by cur, with the end of the haystack pointed to by end.

  When an occurrence is found, memcmp is run to check if a match occurs

  at the corresponding position.

  

  `mask` should have bits set corresponding the positions in the chunk

  in which matches are considered. This is only used for the last vector

  load where the beginning of the vector might have overlapped with the

  last load in the main loop. The mask lets us avoid visiting positions

  that have already been discarded as matches.

  

  # Safety

  

  It must be safe to do an unaligned read of size(V) bytes starting at

  both (cur + self.index1) and (cur + self.index2). It must also be safe

  to do unaligned loads on cur up to (end - needle.len()).

- <span id="finder-find-prefilter-in-chunk"></span>`unsafe fn find_prefilter_in_chunk(&self, cur: *const u8) -> Option<usize>`

  Search for an occurrence of our byte pair from the needle in the chunk

  pointed to by cur, with the end of the haystack pointed to by end.

  When an occurrence is found, memcmp is run to check if a match occurs

  at the corresponding position.

  

  # Safety

  

  It must be safe to do an unaligned read of size(V) bytes starting at

  both (cur + self.index1) and (cur + self.index2). It must also be safe

  to do unaligned reads on cur up to (end - needle.len()).

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](../../all/packedpair/index.md#pair)

  Returns the pair of offsets (into the needle) used to check as a

  predicate before confirming whether a needle exists at a particular

  position.

- <span id="finder-min-haystack-len"></span>`fn min_haystack_len(&self) -> usize`

  Returns the minimum haystack length that this `Finder` can search.

  

  Providing a haystack to this `Finder` shorter than this length is

  guaranteed to result in a panic.

#### Trait Implementations

##### `impl Any for Finder<V>`

- <span id="finder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Finder<V>`

- <span id="finder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Finder<V>`

- <span id="finder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<V: clone::Clone> Clone for Finder<V>`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder<V>` — [`Finder`](#finder)

##### `impl CloneToUninit for Finder<V>`

- <span id="finder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<V: marker::Copy> Copy for Finder<V>`

##### `impl<V: fmt::Debug> Debug for Finder<V>`

- <span id="finder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Finder<V>`

- <span id="finder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Finder<V>`

- <span id="finder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Finder<V>`

- <span id="finder-toowned-type-owned"></span>`type Owned = T`

- <span id="finder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="finder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Finder<V>`

- <span id="finder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="finder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Finder<V>`

- <span id="finder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="finder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `matched`

```rust
unsafe fn matched(start: *const u8, cur: *const u8, chunki: usize) -> usize
```

*Defined in [`memchr-2.7.6/src/arch/generic/packedpair.rs:312-314`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/generic/packedpair.rs#L312-L314)*

Accepts a chunk-relative offset and returns a haystack relative offset.

This used to be marked `#[cold]` and `#[inline(never)]`, but I couldn't
observe a consistent measureable difference between that and just inlining
it. So we go with inlining it.

# Safety

Same at `ptr::offset_from` in addition to `cur >= start`.

