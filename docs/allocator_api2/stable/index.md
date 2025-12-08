*[allocator_api2](../index.md) / [stable](index.md)*

---

# Module `stable`

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

