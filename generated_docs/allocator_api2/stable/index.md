*[allocator_api2](../index.md) / [stable](index.md)*

---

# Module `stable`

## Contents

- [Modules](#modules)
  - [`alloc`](#alloc)
  - [`boxed`](#boxed)
  - [`raw_vec`](#raw-vec)
  - [`vec`](#vec)
  - [`macros`](#macros)
  - [`slice`](#slice)
  - [`unique`](#unique)
  - [`collections`](#collections)
- [Traits](#traits)
  - [`SliceExt`](#sliceext)
- [Functions](#functions)
  - [`assume`](#assume)
  - [`addr`](#addr)
  - [`invalid_mut`](#invalid-mut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc`](#alloc) | mod | Memory allocation APIs |
| [`boxed`](#boxed) | mod | The `Box<T>` type for heap allocation. |
| [`raw_vec`](#raw-vec) | mod |  |
| [`vec`](#vec) | mod | A contiguous growable array type with heap-allocated contents, written `Vec<T>`. |
| [`macros`](#macros) | mod |  |
| [`slice`](#slice) | mod |  |
| [`unique`](#unique) | mod |  |
| [`collections`](#collections) | mod |  |
| [`SliceExt`](#sliceext) | trait |  |
| [`assume`](#assume) | fn |  |
| [`addr`](#addr) | fn |  |
| [`invalid_mut`](#invalid-mut) | fn |  |

## Modules

- [`alloc`](alloc/index.md) — Memory allocation APIs
- [`boxed`](boxed/index.md) — The `Box<T>` type for heap allocation.
- [`raw_vec`](raw_vec/index.md)
- [`vec`](vec/index.md) — A contiguous growable array type with heap-allocated contents, written
- [`macros`](macros/index.md)
- [`slice`](slice/index.md)
- [`unique`](unique/index.md)
- [`collections`](collections/index.md)

## Traits

### `SliceExt<T>`

```rust
trait SliceExt<T> { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/slice.rs:7-67`](../../../.source_1765900590/allocator-api2-0.2.21/src/stable/slice.rs#L7-L67)*

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

## Functions

### `assume`

```rust
unsafe fn assume(v: bool)
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:71-75`](../../../.source_1765900590/allocator-api2-0.2.21/src/stable/mod.rs#L71-L75)*

### `addr`

```rust
fn addr<T>(x: *const T) -> usize
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:91-96`](../../../.source_1765900590/allocator-api2-0.2.21/src/stable/mod.rs#L91-L96)*

### `invalid_mut`

```rust
fn invalid_mut<T>(addr: usize) -> *mut T
```

*Defined in [`allocator-api2-0.2.21/src/stable/mod.rs:100-105`](../../../.source_1765900590/allocator-api2-0.2.21/src/stable/mod.rs#L100-L105)*

