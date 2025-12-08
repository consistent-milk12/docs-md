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

## Type Aliases

### `Mask`

```rust
type Mask = u16;
```

The type of our mask.

While we don't expose anyway to configure this in the public API, if one
really needs less memory usage or support for longer needles, then it is
suggested to copy the code from this module and modify it to fit your
needs. The code below is written to be correct regardless of whether Mask
is a u8, u16, u32, u64 or u128.

