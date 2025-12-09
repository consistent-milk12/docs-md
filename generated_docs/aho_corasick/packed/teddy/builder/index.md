*[aho_corasick](../../../index.md) / [packed](../../index.md) / [teddy](../index.md) / [builder](index.md)*

---

# Module `builder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64`](#x86_64) | mod |  |
| [`Builder`](#builder) | struct | A builder for constructing a Teddy matcher. |
| [`Searcher`](#searcher) | struct | A searcher that dispatches to one of several possible Teddy variants. |
| [`SearcherT`](#searchert) | trait | A trait that provides dynamic dispatch over the different possible Teddy |

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

- <span id="builder-build"></span>`fn build(&self, patterns: Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../pattern/index.md), [`Searcher`](#searcher)

- <span id="builder-only-fat"></span>`fn only_fat(&mut self, yes: Option<bool>) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-only-256bit"></span>`fn only_256bit(&mut self, yes: Option<bool>) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-heuristic-pattern-limits"></span>`fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-build-imp"></span>`fn build_imp(&self, patterns: Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../pattern/index.md), [`Searcher`](#searcher)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

### `Searcher`

```rust
struct Searcher {
    imp: alloc::sync::Arc<dyn SearcherT>,
    memory_usage: usize,
    minimum_len: usize,
}
```

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

- <span id="searcher-find"></span>`fn find(&self, haystack: &[u8], at: usize) -> Option<crate::Match>` — [`Match`](../../../index.md)

- <span id="searcher-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="searcher-minimum-len"></span>`fn minimum_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](#searcher)

##### `impl Debug for Searcher`

- <span id="searcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `SearcherT`

```rust
trait SearcherT: Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

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

- [`FatAVX2`](x86_64/index.md)
- [`SlimAVX2`](x86_64/index.md)
- [`SlimSSSE3`](x86_64/index.md)

