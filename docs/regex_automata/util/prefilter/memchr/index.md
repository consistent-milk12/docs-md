*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memchr](index.md)*

---

# Module `memchr`

## Structs

### `Memchr`

```rust
struct Memchr(u8);
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr>` — [`MatchKind`](../../../index.md), [`Memchr`](#memchr)

#### Trait Implementations

##### `impl Clone for Memchr`

- `fn clone(self: &Self) -> Memchr` — [`Memchr`](#memchr)

##### `impl Debug for Memchr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Memchr`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

### `Memchr2`

```rust
struct Memchr2(u8, u8);
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr2>` — [`MatchKind`](../../../index.md), [`Memchr2`](#memchr2)

#### Trait Implementations

##### `impl Clone for Memchr2`

- `fn clone(self: &Self) -> Memchr2` — [`Memchr2`](#memchr2)

##### `impl Debug for Memchr2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Memchr2`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

### `Memchr3`

```rust
struct Memchr3(u8, u8, u8);
```

#### Implementations

- `fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr3>` — [`MatchKind`](../../../index.md), [`Memchr3`](#memchr3)

#### Trait Implementations

##### `impl Clone for Memchr3`

- `fn clone(self: &Self) -> Memchr3` — [`Memchr3`](#memchr3)

##### `impl Debug for Memchr3`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Memchr3`

- `fn find(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn prefix(self: &Self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn is_fast(self: &Self) -> bool`

