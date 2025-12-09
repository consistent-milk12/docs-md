*[rustix](../../index.md) / [buffer](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait |  |

## Traits

### `Sealed<T>`

```rust
trait Sealed<T> { ... }
```

#### Associated Types

- `type Output`

#### Required Methods

- `fn parts_mut(&mut self) -> (*mut T, usize)`

  Return a pointer and length for this buffer.

- `fn assume_init(self, len: usize) -> <Self as >::Output`

  Convert a finished buffer pointer into its result.

#### Implementors

- [`SpareCapacity`](../index.md)
- `&'a mut [core::mem::MaybeUninit<T>; N]`
- `&'a mut [core::mem::MaybeUninit<T>]`
- `&'a mut alloc::vec::Vec<core::mem::MaybeUninit<T>>`
- `&mut [T; N]`
- `&mut [T]`
- `&mut alloc::vec::Vec<T>`

