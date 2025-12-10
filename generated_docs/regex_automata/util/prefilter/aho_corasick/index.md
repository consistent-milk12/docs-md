*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [aho_corasick](index.md)*

---

# Module `aho_corasick`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AhoCorasick`](#ahocorasick) | struct |  |

## Structs

### `AhoCorasick`

```rust
struct AhoCorasick {
    ac: aho_corasick::AhoCorasick,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/aho_corasick.rs:7-12`](../../../../../.source_1765210505/regex-automata-0.4.13/src/util/prefilter/aho_corasick.rs#L7-L12)*

#### Implementations

- <span id="ahocorasick-new"></span>`fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<AhoCorasick>` — [`MatchKind`](../../../index.md#matchkind), [`AhoCorasick`](#ahocorasick)

#### Trait Implementations

##### `impl Clone for AhoCorasick`

- <span id="ahocorasick-clone"></span>`fn clone(&self) -> AhoCorasick` — [`AhoCorasick`](#ahocorasick)

##### `impl Debug for AhoCorasick`

- <span id="ahocorasick-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for AhoCorasick`

- <span id="ahocorasick-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="ahocorasick-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="ahocorasick-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="ahocorasick-is-fast"></span>`fn is_fast(&self) -> bool`

