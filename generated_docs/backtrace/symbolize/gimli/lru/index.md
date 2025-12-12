*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [lru](index.md)*

---

# Module `lru`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Lru`](#lru) | struct | least-recently-used cache with static size |

## Structs

### `Lru<T, const N: usize>`

```rust
struct Lru<T, const N: usize> {
    len: usize,
    arr: [core::mem::MaybeUninit<T>; N],
}
```

*Defined in [`backtrace-0.3.76/src/symbolize/gimli/lru.rs:5-9`](../../../../../.source_1765210505/backtrace-0.3.76/src/symbolize/gimli/lru.rs#L5-L9)*

least-recently-used cache with static size

#### Implementations

- <span id="lru-clear"></span>`fn clear(&mut self)`

- <span id="lru-iter"></span>`fn iter(&self) -> impl Iterator<Item = &T>`

- <span id="lru-push-front"></span>`fn push_front(&mut self, value: T) -> Option<&mut T>`

- <span id="lru-move-to-front"></span>`fn move_to_front(&mut self, idx: usize) -> Option<&mut T>`

#### Trait Implementations

##### `impl<T> Default for Lru<T, N>`

- <span id="lru-default"></span>`fn default() -> Self`

