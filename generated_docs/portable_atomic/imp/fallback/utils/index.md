*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [utils](index.md)*

---

# Module `utils`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CachePadded`](#cachepadded) | struct | Pads and aligns a value to the length of a cache line. |
| [`Backoff`](#backoff) | struct | Performs exponential backoff in spin loops. |
| [`SPIN_LIMIT`](#spin-limit) | const |  |

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:92-94`](../../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L92-L94)*

Pads and aligns a value to the length of a cache line.

#### Implementations

- <span id="cachepadded-new"></span>`const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Deref for CachePadded<T>`

- <span id="cachepadded-deref-type-target"></span>`type Target = T`

- <span id="cachepadded-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> Receiver for CachePadded<T>`

- <span id="cachepadded-receiver-type-target"></span>`type Target = T`

### `Backoff`

```rust
struct Backoff {
    step: u32,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:115-117`](../../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L115-L117)*

Performs exponential backoff in spin loops.

#### Implementations

- <span id="backoff-new"></span>`const fn new() -> Self`

- <span id="backoff-snooze"></span>`fn snooze(&mut self)`

## Constants

### `SPIN_LIMIT`
```rust
const SPIN_LIMIT: u32 = 4u32;
```

*Defined in [`portable-atomic-1.11.1/src/imp/fallback/utils.rs:120`](../../../../../.source_1765210505/portable-atomic-1.11.1/src/imp/fallback/utils.rs#L120)*

