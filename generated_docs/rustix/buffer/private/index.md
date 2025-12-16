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

*Defined in [`rustix-1.1.2/src/buffer.rs:299-322`](../../../../.source_1765894658/rustix-1.1.2/src/buffer.rs#L299-L322)*

#### Associated Types

- `type Output`

#### Required Methods

- `fn parts_mut(&mut self) -> (*mut T, usize)`

  Return a pointer and length for this buffer.
  
  The length is the number of elements of type `T`, not a number of
  bytes.
  
  It's tempting to have this return `&mut [MaybeUninit<T>]` instead,
  however that would require this function to be `unsafe`, because
  callers could use the `&mut [MaybeUninit<T>]` slice to set elements
  to `MaybeUninit::<T>::uninit()`, which would be a problem if `Self`
  is `&mut [T]` or similar.

- `fn assume_init(self, len: usize) -> <Self as >::Output`

  Convert a finished buffer pointer into its result.
  
  # Safety
  
  At least `len` elements of the buffer must now be initialized.

#### Implementors

- [`SpareCapacity`](../index.md#sparecapacity)
- `&'a mut [core::mem::MaybeUninit<T>; N]`
- `&'a mut [core::mem::MaybeUninit<T>]`
- `&'a mut alloc::vec::Vec<core::mem::MaybeUninit<T>>`
- `&mut [T; N]`
- `&mut [T]`
- `&mut alloc::vec::Vec<T>`

