*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [teddy](index.md)*

---

# Module `teddy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Teddy`](#teddy) | struct |  |

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

- <span id="teddy-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Teddy>` — [`MatchKind`](../../../index.md), [`Teddy`](#teddy)

#### Trait Implementations

##### `impl Clone for Teddy`

- <span id="teddy-clone"></span>`fn clone(&self) -> Teddy` — [`Teddy`](#teddy)

##### `impl Debug for Teddy`

- <span id="teddy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Teddy`

- <span id="teddy-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="teddy-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="teddy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="teddy-is-fast"></span>`fn is_fast(&self) -> bool`

