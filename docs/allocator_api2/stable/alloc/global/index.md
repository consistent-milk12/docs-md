*[allocator_api2](../../../index.md) / [stable](../../index.md) / [alloc](../index.md) / [global](index.md)*

---

# Module `global`

## Structs

### `Global`

```rust
struct Global;
```

The global memory allocator.

This type implements the [`Allocator`](../index.md) trait by forwarding calls
to the allocator registered with the `#[global_allocator]` attribute
if there is one, or the `std` crate’s default.

Note: while this type is unstable, the functionality it provides can be
accessed through the [free functions in `alloc`](crate#functions).

#### Implementations

- `fn alloc_impl(self: &Self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

- `unsafe fn grow_impl(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

#### Trait Implementations

##### `impl Allocator for Global`

- `fn allocate(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

- `fn allocate_zeroed(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

- `unsafe fn deallocate(self: &Self, ptr: NonNull<u8>, layout: Layout)`

- `unsafe fn grow(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

- `unsafe fn grow_zeroed(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

- `unsafe fn shrink(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](../index.md)

##### `impl Clone for Global`

- `fn clone(self: &Self) -> Global` — [`Global`](#global)

##### `impl Copy for Global`

##### `impl Debug for Global`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Global`

- `fn default() -> Global` — [`Global`](#global)

