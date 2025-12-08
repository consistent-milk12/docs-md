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
  - [`unnamed`](#unnamed)
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
| [`unnamed`](#unnamed) | trait |  |
| [`assume`](#assume) | fn |  |
| [`addr`](#addr) | fn |  |
| [`invalid_mut`](#invalid_mut) | fn |  |

## Modules

- [`alloc`](alloc/index.md) - Memory allocation APIs
- [`boxed`](boxed/index.md) - The `Box<T>` type for heap allocation.
- [`raw_vec`](raw_vec/index.md) - 
- [`vec`](vec/index.md) - A contiguous growable array type with heap-allocated contents, written
- [`macros`](macros/index.md) - 
- [`slice`](slice/index.md) - 
- [`unique`](unique/index.md) - 
- [`collections`](collections/index.md) - 

## Traits

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

