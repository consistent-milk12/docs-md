*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [byteset](index.md)*

---

# Module `byteset`

## Structs

### `ByteSet`

```rust
struct ByteSet([bool; 256]);
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<ByteSet>` — [`MatchKind`](../../../index.md), [`ByteSet`](#byteset)

#### Trait Implementations

##### `impl Clone for ByteSet`

- `fn clone(self: &Self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Debug for ByteSet`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for ByteSet`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

