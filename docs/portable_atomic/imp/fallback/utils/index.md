*[portable_atomic](../../../index.md) / [imp](../../index.md) / [fallback](../index.md) / [utils](index.md)*

---

# Module `utils`

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

Pads and aligns a value to the length of a cache line.

#### Implementations

- `const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Deref for CachePadded<T>`

- `type Target = T`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for CachePadded<T>`

- `type Target = T`

### `Backoff`

```rust
struct Backoff {
    step: u32,
}
```

Performs exponential backoff in spin loops.

#### Implementations

- `const fn new() -> Self`

- `fn snooze(self: &mut Self)`

## Constants

### `SPIN_LIMIT`

```rust
const SPIN_LIMIT: u32 = 4u32;
```

