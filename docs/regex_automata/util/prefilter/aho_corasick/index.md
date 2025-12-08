*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [aho_corasick](index.md)*

---

# Module `aho_corasick`

## Structs

### `AhoCorasick`

```rust
struct AhoCorasick {
    ac: aho_corasick::AhoCorasick,
}
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<AhoCorasick>` — [`MatchKind`](../../../index.md), [`AhoCorasick`](#ahocorasick)

#### Trait Implementations

##### `impl Clone for AhoCorasick`

- `fn clone(self: &Self) -> AhoCorasick` — [`AhoCorasick`](#ahocorasick)

##### `impl Debug for AhoCorasick`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for AhoCorasick`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

