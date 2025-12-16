*[allocator_api2](../../index.md) / [stable](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceExt`](#sliceext) | trait | Slice methods that use `Box` and `Vec` from this crate. |

## Traits

### `SliceExt<T>`

```rust
trait SliceExt<T> { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/slice.rs:7-67`](../../../../.source_1765900590/allocator-api2-0.2.21/src/stable/slice.rs#L7-L67)*

Slice methods that use `Box` and `Vec` from this crate.

#### Required Methods

- `fn SliceExt::to_vec_in<A: Allocator>(&self, alloc: A) -> Vec<T, A>`

  Copies `self` into a new `Vec` with an allocator.
  
  ##### Examples
  
  ```rust
  #![feature(allocator_api)]
  
  use std::alloc::System;
  
  let s = [10, 40, 30];
  let x = s.to_vec_in(System);
  // Here, `s` and `x` can be modified independently.
  ```

- `fn SliceExt::repeat(&self, n: usize) -> Vec<T, Global>`

  Creates a vector by copying a slice `n` times.
  
  ##### Panics
  
  This function will panic if the capacity would overflow.
  
  ##### Examples
  
  Basic usage:
  
  ```rust
  assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
  ```
  
  A panic upon overflow:
  
  ```should_panic
  // this will panic at runtime
  b"0123456789abcdef".repeat(usize::MAX);
  ```

#### Provided Methods

- `fn SliceExt::to_vec(&self) -> Vec<T, Global>`

  Copies `self` into a new `Vec`.
  
  ##### Examples
  
  ```rust
  let s = [10, 40, 30];
  let x = s.to_vec();
  // Here, `s` and `x` can be modified independently.
  ```

#### Implementors

- `[T]`

