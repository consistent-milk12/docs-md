*[allocator_api2](../../index.md) / [stable](../index.md) / [alloc](index.md)*

---

# Module `alloc`

Memory allocation APIs

## Modules

- [`global`](global/index.md) - 
- [`Layout`](Layout/index.md) - 

## Structs

### `GlobalAlloc<R: gimli::Reader>`

```rust
struct GlobalAlloc<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    call_file: Option<u64>,
    call_line: u32,
    call_column: u32,
}
```

*Re-exported from `addr2line`*

#### Implementations

- `fn parse(state: &mut InlinedState<'_, R>, dw_die_offset: gimli::UnitOffset<<R as >::Offset>, abbrev: &gimli::Abbreviation, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`LayoutError`](#layouterror)

### `Global`

```rust
struct Global;
```

The global memory allocator.

This type implements the [`Allocator`](#allocator) trait by forwarding calls
to the allocator registered with the `#[global_allocator]` attribute
if there is one, or the `std` crate’s default.

Note: while this type is unstable, the functionality it provides can be
accessed through the [free functions in `alloc`](crate#functions).

#### Implementations

- `fn alloc_impl(self: &Self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

- `unsafe fn grow_impl(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

#### Trait Implementations

##### `impl Allocator for Global`

- `fn allocate(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

- `fn allocate_zeroed(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

- `unsafe fn deallocate(self: &Self, ptr: NonNull<u8>, layout: Layout)`

- `unsafe fn grow(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

- `unsafe fn grow_zeroed(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

- `unsafe fn shrink(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`AllocError`](#allocerror)

##### `impl Clone for Global`

- `fn clone(self: &Self) -> Global` — [`Global`](global/index.md)

##### `impl Copy for Global`

##### `impl Debug for Global`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Global`

- `fn default() -> Global` — [`Global`](global/index.md)

### `AllocError`

```rust
struct AllocError;
```

The `AllocError` error indicates an allocation failure
that may be due to resource exhaustion or to
something wrong when combining the given input arguments with this
allocator.

#### Trait Implementations

##### `impl Clone for AllocError`

- `fn clone(self: &Self) -> AllocError` — [`AllocError`](#allocerror)

##### `impl Copy for AllocError`

##### `impl Debug for AllocError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for AllocError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AllocError`

##### `impl PartialEq for AllocError`

- `fn eq(self: &Self, other: &AllocError) -> bool` — [`AllocError`](#allocerror)

##### `impl StructuralPartialEq for AllocError`

##### `impl<T> ToString for AllocError`

- `fn to_string(self: &Self) -> String`

## Traits

### `Allocator`

```rust
trait Allocator { ... }
```

An implementation of `Allocator` can allocate, grow, shrink, and deallocate arbitrary blocks of
data described via [`Layout`][].

`Allocator` is designed to be implemented on ZSTs, references, or smart pointers because having
an allocator like `MyAlloc([u8; N])` cannot be moved, without updating the pointers to the
allocated memory.

Unlike [`GlobalAlloc`][], zero-sized allocations are allowed in `Allocator`. If an underlying
allocator does not support this (like jemalloc) or return a null pointer (such as
`libc::malloc`), this must be caught by the implementation.

### Currently allocated memory

Some of the methods require that a memory block be *currently allocated* via an allocator. This
means that:

* the starting address for that memory block was previously returned by `allocate`, `grow`, or
  `shrink`, and

* the memory block has not been subsequently deallocated, where blocks are either deallocated
  directly by being passed to `deallocate` or were changed by being passed to `grow` or
  `shrink` that returns `Ok`. If `grow` or `shrink` have returned `Err`, the passed pointer
  remains valid.




### Memory fitting

Some of the methods require that a layout *fit* a memory block. What it means for a layout to
"fit" a memory block means (or equivalently, for a memory block to "fit" a layout) is that the
following conditions must hold:

* The block must be allocated with the same alignment as `layout.align()`, and

* The provided `layout.size()` must fall in the range `min ..= max`, where:
  - `min` is the size of the layout most recently used to allocate the block, and
  - `max` is the latest actual size returned from `allocate`, `grow`, or `shrink`.


# Safety

* Memory blocks returned from an allocator must point to valid memory and retain their validity
  until the instance and all of its clones are dropped,

* cloning or moving the allocator must not invalidate memory blocks returned from this
  allocator. A cloned allocator must behave like the same allocator, and

* any pointer to a memory block which is [*currently allocated*] may be passed to any other
  method of the allocator.


#### Required Methods

- `fn allocate(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to allocate a block of memory.

- `fn allocate_zeroed(self: &Self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Behaves like `allocate`, but also ensures that the returned memory is zero-initialized.

- `fn deallocate(self: &Self, ptr: NonNull<u8>, layout: Layout)`

  Deallocates the memory referenced by `ptr`.

- `fn grow(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to extend the memory block.

- `fn grow_zeroed(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Behaves like `grow`, but also ensures that the new contents are set to zero before being

- `fn shrink(self: &Self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to shrink the memory block.

- `fn by_ref(self: &Self) -> &Self`

  Creates a "by reference" adapter for this instance of `Allocator`.

