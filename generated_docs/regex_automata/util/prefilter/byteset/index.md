*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [byteset](index.md)*

---

# Module `byteset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ByteSet`](#byteset) | struct |  |

## Structs

### `ByteSet`

```rust
struct ByteSet([bool; 256]);
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/byteset.rs:7`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/byteset.rs#L7)*

#### Implementations

- <span id="byteset-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<ByteSet>` — [`MatchKind`](../../../index.md#matchkind), [`ByteSet`](#byteset)

#### Trait Implementations

##### `impl Clone for ByteSet`

- <span id="byteset-clone"></span>`fn clone(&self) -> ByteSet` — [`ByteSet`](#byteset)

##### `impl Debug for ByteSet`

- <span id="byteset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for ByteSet`

- <span id="byteset-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="byteset-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="byteset-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="byteset-is-fast"></span>`fn is_fast(&self) -> bool`

