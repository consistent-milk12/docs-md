*[allocator_api2](../index.md) / [stable](index.md)*

---

# Module `stable`

## Contents

- [Modules](#modules)
  - [`alloc`](#alloc)
  - [`boxed`](#boxed)
  - [`raw_vec`](#raw_vec)
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
  - [`invalid_mut`](#invalid_mut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc`](#alloc) | mod | Memory allocation APIs |
| [`boxed`](#boxed) | mod | The `Box<T>` type for heap allocation. |
| [`raw_vec`](#raw_vec) | mod |  |
| [`vec`](#vec) | mod | A contiguous growable array type with heap-allocated contents, written |
| [`macros`](#macros) | mod |  |
| [`slice`](#slice) | mod |  |
| [`unique`](#unique) | mod |  |
| [`collections`](#collections) | mod |  |
| [`SliceExt`](#sliceext) | trait |  |
| [`assume`](#assume) | fn |  |
| [`addr`](#addr) | fn |  |
| [`invalid_mut`](#invalid_mut) | fn |  |

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

Slice methods that use `Box` and `Vec` from this crate.

#### Required Methods

- `fn to_vec_in<A: Allocator>(&self, alloc: A) -> Vec<T, A>`

  Copies `self` into a new `Vec` with an allocator.

- `fn repeat(&self, n: usize) -> Vec<T, Global>`

  Creates a vector by copying a slice `n` times.

#### Provided Methods

- `fn to_vec(&self) -> Vec<T, Global>`

  Copies `self` into a new `Vec`.

#### Implementors

- `[T]`

## Functions

### `assume`

```rust
unsafe fn assume(v: bool)
```

### `addr`

```rust
fn addr<T>(x: *const T) -> usize
```

### `invalid_mut`

```rust
fn invalid_mut<T>(addr: usize) -> *mut T
```

