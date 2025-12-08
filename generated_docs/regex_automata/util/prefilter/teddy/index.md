*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [teddy](index.md)*

---

# Module `teddy`

## Structs

### `Teddy`

```rust
struct Teddy {
    searcher: aho_corasick::packed::Searcher,
    anchored_ac: aho_corasick::dfa::DFA,
    minimum_len: usize,
}
```

#### Fields

- **`searcher`**: `aho_corasick::packed::Searcher`

  The actual Teddy searcher.
  
  Technically, it's possible that Teddy doesn't actually get used, since
  Teddy does require its haystack to at least be of a certain size
  (usually around the size of whatever vector is being used, so ~16
  or ~32 bytes). For haystacks shorter than that, the implementation
  currently uses Rabin-Karp.

- **`anchored_ac`**: `aho_corasick::dfa::DFA`

  When running an anchored search, the packed searcher can't handle it so
  we defer to Aho-Corasick itself. Kind of sad, but changing the packed
  searchers to support anchored search would be difficult at worst and
  annoying at best. Since packed searchers only apply to small numbers of
  literals, we content ourselves that this is not much of an added cost.
  (That packed searchers only work with a small number of literals is
  also why we use a DFA here. Otherwise, the memory usage of a DFA would
  likely be unacceptable.)

- **`minimum_len`**: `usize`

  The length of the smallest literal we look for.
  
  We use this as a heuristic to figure out whether this will be "fast" or
  not. Generally, the longer the better, because longer needles are more
  discriminating and thus reduce false positive rate.

#### Implementations

- `fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Teddy>` — [`MatchKind`](../../../index.md), [`Teddy`](#teddy)

#### Trait Implementations

##### `impl Clone for Teddy`

- `fn clone(self: &Self) -> Teddy` — [`Teddy`](#teddy)

##### `impl Debug for Teddy`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Teddy`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

