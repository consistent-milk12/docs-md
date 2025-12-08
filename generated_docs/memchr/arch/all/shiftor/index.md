*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [shiftor](index.md)*

---

# Module `shiftor`

An implementation of the [Shift-Or substring search algorithm][shiftor](#shiftor).


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A forward substring searcher using the Shift-Or algorithm. |
| [`Mask`](#mask) | type | The type of our mask. |

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

- <span id="finder-max-needle-len"></span>`const MAX_NEEDLE_LEN: usize`

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

#### Trait Implementations

##### `impl Debug for Finder`

- <span id="finder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

