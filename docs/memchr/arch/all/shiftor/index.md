*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [shiftor](index.md)*

---

# Module `shiftor`

An implementation of the [Shift-Or substring search algorithm][shiftor](#shiftor).


## Structs

### `Finder`

```rust
struct Finder {
    masks: alloc::boxed::Box<[u16; 256]>,
    needle_len: usize,
}
```

A forward substring searcher using the Shift-Or algorithm.

#### Implementations

- `const MAX_NEEDLE_LEN: usize`

- `fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

#### Trait Implementations

##### `impl Debug for Finder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

