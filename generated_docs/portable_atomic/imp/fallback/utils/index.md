*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [utils](index.md)*

---

# Module `utils`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CachePadded`](#cachepadded) | struct | Pads and aligns a value to the length of a cache line. |
| [`Backoff`](#backoff) | struct | Performs exponential backoff in spin loops. |
| [`SPIN_LIMIT`](#spin_limit) | const |  |

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

Pads and aligns a value to the length of a cache line.

#### Implementations

- <span id="cachepadded-new"></span>`const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Deref for CachePadded<T>`

- <span id="cachepadded-target"></span>`type Target = T`

- <span id="cachepadded-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for CachePadded<T>`

- <span id="cachepadded-target"></span>`type Target = T`

### `Backoff`

```rust
struct Backoff {
    step: u32,
}
```

Performs exponential backoff in spin loops.

#### Implementations

- <span id="backoff-new"></span>`const fn new() -> Self`

- <span id="backoff-snooze"></span>`fn snooze(&mut self)`

## Constants

### `SPIN_LIMIT`

```rust
const SPIN_LIMIT: u32 = 4u32;
```

