*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memchr](index.md)*

---

# Module `memchr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Memchr`](#memchr) | struct |  |
| [`Memchr2`](#memchr2) | struct |  |
| [`Memchr3`](#memchr3) | struct |  |

## Structs

### `Memchr`

```rust
struct Memchr(u8);
```

#### Implementations

- <span id="memchr-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr>` — [`MatchKind`](../../../index.md), [`Memchr`](#memchr)

#### Trait Implementations

##### `impl Clone for Memchr`

- <span id="memchr-clone"></span>`fn clone(&self) -> Memchr` — [`Memchr`](#memchr)

##### `impl Debug for Memchr`

- <span id="memchr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Memchr`

- <span id="memchr-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr-is-fast"></span>`fn is_fast(&self) -> bool`

### `Memchr2`

```rust
struct Memchr2(u8, u8);
```

#### Implementations

- <span id="memchr2-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr2>` — [`MatchKind`](../../../index.md), [`Memchr2`](#memchr2)

#### Trait Implementations

##### `impl Clone for Memchr2`

- <span id="memchr2-clone"></span>`fn clone(&self) -> Memchr2` — [`Memchr2`](#memchr2)

##### `impl Debug for Memchr2`

- <span id="memchr2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Memchr2`

- <span id="memchr2-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr2-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr2-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr2-is-fast"></span>`fn is_fast(&self) -> bool`

### `Memchr3`

```rust
struct Memchr3(u8, u8, u8);
```

#### Implementations

- <span id="memchr3-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr3>` — [`MatchKind`](../../../index.md), [`Memchr3`](#memchr3)

#### Trait Implementations

##### `impl Clone for Memchr3`

- <span id="memchr3-clone"></span>`fn clone(&self) -> Memchr3` — [`Memchr3`](#memchr3)

##### `impl Debug for Memchr3`

- <span id="memchr3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Memchr3`

- <span id="memchr3-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr3-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- <span id="memchr3-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr3-is-fast"></span>`fn is_fast(&self) -> bool`

