*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memmem](index.md)*

---

# Module `memmem`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Memmem`](#memmem) | struct |  |

## Structs

### `Memmem`

```rust
struct Memmem {
    finder: memchr::memmem::Finder<'static>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/memmem.rs:7-12`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/memmem.rs#L7-L12)*

#### Implementations

- <span id="memmem-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memmem>` — [`MatchKind`](../../../index.md#matchkind), [`Memmem`](#memmem)

#### Trait Implementations

##### `impl Clone for Memmem`

- <span id="memmem-clone"></span>`fn clone(&self) -> Memmem` — [`Memmem`](#memmem)

##### `impl Debug for Memmem`

- <span id="memmem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Memmem`

- <span id="memmem-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memmem-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memmem-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memmem-is-fast"></span>`fn is_fast(&self) -> bool`

