*[allocator_api2](../../../index.md) / [stable](../../index.md) / [alloc](../index.md) / [global](index.md)*

---

# Module `global`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Global`](#global) | struct | The global memory allocator. |

## Structs

### `Global`

```rust
struct Global;
```

*Defined in [`allocator-api2-0.2.21/src/stable/alloc/global.rs:18`](../../../../../.source_1765521767/allocator-api2-0.2.21/src/stable/alloc/global.rs#L18)*

The global memory allocator.

This type implements the [`Allocator`](../index.md) trait by forwarding calls
to the allocator registered with the `#[global_allocator]` attribute
if there is one, or the `std` crate’s default.

Note: while this type is unstable, the functionality it provides can be
accessed through the [free functions in `alloc`](crate#functions).

#### Implementations

- <span id="global-alloc-impl"></span>`fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-grow-impl"></span>`unsafe fn grow_impl(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

#### Trait Implementations

##### `impl Allocator for Global`

- <span id="global-allocate"></span>`fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-allocate-zeroed"></span>`fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-deallocate"></span>`unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)` — [`Layout`](../index.md#layout)

- <span id="global-grow"></span>`unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-grow-zeroed"></span>`unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-shrink"></span>`unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

##### `impl Clone for Global`

- <span id="global-clone"></span>`fn clone(&self) -> Global` — [`Global`](#global)

##### `impl Copy for Global`

##### `impl Debug for Global`

- <span id="global-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Global`

- <span id="global-default"></span>`fn default() -> Global` — [`Global`](#global)

