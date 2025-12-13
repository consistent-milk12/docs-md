*[aho_corasick](../../../index.md) / [packed](../../index.md) / [teddy](../index.md) / [builder](index.md)*

---

# Module `builder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64`](#x86-64) | mod |  |
| [`Builder`](#builder) | struct | A builder for constructing a Teddy matcher. |
| [`Searcher`](#searcher) | struct | A searcher that dispatches to one of several possible Teddy variants. |
| [`SearcherT`](#searchert) | trait | A trait that provides dynamic dispatch over the different possible Teddy variants on the same algorithm. |

## Modules

- [`x86_64`](x86_64/index.md)

## Structs

### `Builder`

```rust
struct Builder {
    only_fat: Option<bool>,
    only_256bit: Option<bool>,
    heuristic_pattern_limits: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:17-34`](../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L17-L34)*

A builder for constructing a Teddy matcher.

The builder primarily permits fine grained configuration of the Teddy
matcher. Most options are made only available for testing/benchmarking
purposes. In reality, options are automatically determined by the nature
and number of patterns given to the builder.

#### Fields

- **`only_fat`**: `Option<bool>`

  When none, this is automatically determined. Otherwise, `false` means
  slim Teddy is used (8 buckets) and `true` means fat Teddy is used
  (16 buckets). Fat Teddy requires AVX2, so if that CPU feature isn't
  available and Fat Teddy was requested, no matcher will be built.

- **`only_256bit`**: `Option<bool>`

  When none, this is automatically determined. Otherwise, `false` means
  that 128-bit vectors will be used (up to SSSE3 instructions) where as
  `true` means that 256-bit vectors will be used. As with `fat`, if
  256-bit vectors are requested and they aren't available, then a
  searcher will not be built.

- **`heuristic_pattern_limits`**: `bool`

  When true (the default), the number of patterns will be used as a
  heuristic for refusing construction of a Teddy searcher. The point here
  is that too many patterns can overwhelm Teddy. But this can be disabled
  in cases where the caller knows better.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new builder for configuring a Teddy matcher.

- <span id="builder-build"></span>`fn build(&self, patterns: Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../pattern/index.md#patterns), [`Searcher`](#searcher)

  Build a matcher for the set of patterns given. If a matcher could not

  be built, then `None` is returned.

  

  Generally, a matcher isn't built if the necessary CPU features aren't

  available, an unsupported target or if the searcher is believed to be

  slower than standard techniques (i.e., if there are too many literals).

- <span id="builder-only-fat"></span>`fn only_fat(&mut self, yes: Option<bool>) -> &mut Builder` — [`Builder`](#builder)

  Require the use of Fat (true) or Slim (false) Teddy. Fat Teddy uses

  16 buckets where as Slim Teddy uses 8 buckets. More buckets are useful

  for a larger set of literals.

  

  `None` is the default, which results in an automatic selection based

  on the number of literals and available CPU features.

- <span id="builder-only-256bit"></span>`fn only_256bit(&mut self, yes: Option<bool>) -> &mut Builder` — [`Builder`](#builder)

  Request the use of 256-bit vectors (true) or 128-bit vectors (false).

  Generally, a larger vector size is better since it either permits

  matching more patterns or matching more bytes in the haystack at once.

  

  `None` is the default, which results in an automatic selection based on

  the number of literals and available CPU features.

- <span id="builder-heuristic-pattern-limits"></span>`fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  Request that heuristic limitations on the number of patterns be

  employed. This useful to disable for benchmarking where one wants to

  explore how Teddy performs on large number of patterns even if the

  heuristics would otherwise refuse construction.

  

  This is enabled by default.

- <span id="builder-build-imp"></span>`fn build_imp(&self, patterns: Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../pattern/index.md#patterns), [`Searcher`](#searcher)

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Builder`

- <span id="builder-toowned-type-owned"></span>`type Owned = T`

- <span id="builder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Searcher`

```rust
struct Searcher {
    imp: alloc::sync::Arc<dyn SearcherT>,
    memory_usage: usize,
    minimum_len: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:322-337`](../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L322-L337)*

A searcher that dispatches to one of several possible Teddy variants.

#### Fields

- **`imp`**: `alloc::sync::Arc<dyn SearcherT>`

  The Teddy variant we use. We use dynamic dispatch under the theory that
  it results in better codegen then a enum, although this is a specious
  claim.
  
  This `Searcher` is essentially a wrapper for a `SearcherT` trait
  object. We just make `memory_usage` and `minimum_len` available without
  going through dynamic dispatch.

- **`memory_usage`**: `usize`

  Total heap memory used by the Teddy variant.

- **`minimum_len`**: `usize`

  The minimum haystack length this searcher can handle. It is intended
  for callers to use some other search routine (such as Rabin-Karp) in
  cases where the haystack (or remainer of the haystack) is too short.

#### Implementations

- <span id="searcher-find"></span>`fn find(&self, haystack: &[u8], at: usize) -> Option<crate::Match>` — [`Match`](../../../util/search/index.md#match)

  Look for the leftmost occurrence of any pattern in this search in the

  given haystack starting at the given position.

  

  # Panics

  

  This panics when `haystack[at..].len()` is less than the minimum length

  for this haystack.

- <span id="searcher-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this type, in

  units of bytes.

- <span id="searcher-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the minimum length, in bytes, that a haystack must be in order

  to use it with this searcher.

#### Trait Implementations

##### `impl Any for Searcher`

- <span id="searcher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Searcher`

- <span id="searcher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Searcher`

- <span id="searcher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](#searcher)

##### `impl CloneToUninit for Searcher`

- <span id="searcher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Searcher`

- <span id="searcher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Searcher`

- <span id="searcher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Searcher`

- <span id="searcher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Searcher`

- <span id="searcher-toowned-type-owned"></span>`type Owned = T`

- <span id="searcher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Searcher`

- <span id="searcher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Searcher`

- <span id="searcher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `SearcherT`

```rust
trait SearcherT: Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:416-448`](../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L416-L448)*

A trait that provides dynamic dispatch over the different possible Teddy
variants on the same algorithm.

On `x86_64` for example, it isn't known until runtime which of 12 possible
variants will be used. One might use one of the four slim 128-bit vector
variants, or one of the four 256-bit vector variants or even one of the
four fat 256-bit vector variants.

Since this choice is generally made when the Teddy searcher is constructed
and this choice is based on the patterns given and what the current CPU
supports, it follows that there must be some kind of indirection at search
time that "selects" the variant chosen at build time.

There are a few different ways to go about this. One approach is to use an
enum. It works fine, but in my experiments, this generally results in worse
codegen. Another approach, which is what we use here, is dynamic dispatch
via a trait object. We basically implement this trait for each possible
variant, select the variant we want at build time and convert it to a
trait object for use at search time.

Another approach is to use function pointers and stick each of the possible
variants into a union. This is essentially isomorphic to the dynamic
dispatch approach, but doesn't require any allocations. Since this crate
requires `alloc`, there's no real reason (AFAIK) to go down this path. (The
`memchr` crate does this.)

#### Required Methods

- `fn find(&self, start: *const u8, end: *const u8) -> Option<Match>`

  Execute a search on the given haystack (identified by `start` and `end`

#### Implementors

- [`FatAVX2`](x86_64/index.md#fatavx2)
- [`SlimAVX2`](x86_64/index.md#slimavx2)
- [`SlimSSSE3`](x86_64/index.md#slimssse3)

