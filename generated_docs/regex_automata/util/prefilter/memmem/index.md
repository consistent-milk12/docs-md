*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memmem](index.md)*

---

# Module `memmem`

## Structs

### `Memmem`

```rust
struct Memmem {
    finder: memchr::memmem::Finder<'static>,
}
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memmem>` — [`MatchKind`](../../../index.md), [`Memmem`](#memmem)

#### Trait Implementations

##### `impl Clone for Memmem`

- `fn clone(self: &Self) -> Memmem` — [`Memmem`](#memmem)

##### `impl Debug for Memmem`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Memmem`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

