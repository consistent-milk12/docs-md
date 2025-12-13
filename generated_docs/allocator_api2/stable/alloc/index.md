*[allocator_api2](../../index.md) / [stable](../index.md) / [alloc](index.md)*

---

# Module `alloc`

Memory allocation APIs

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`global`](#global) | mod |  |
| [`Layout`](#layout) | mod |  |
| [`GlobalAlloc`](#globalalloc) | struct |  |
| [`Global`](#global) | struct |  |
| [`AllocError`](#allocerror) | struct | The `AllocError` error indicates an allocation failure that may be due to resource exhaustion or to something wrong when combining the given input arguments with this allocator. |
| [`Allocator`](#allocator) | trait | An implementation of `Allocator` can allocate, grow, shrink, and deallocate arbitrary blocks of data described via [`Layout`][]. |

## Modules

- [`global`](global/index.md)
- [`Layout`](Layout/index.md)

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

*Defined in [`addr2line-0.25.1/src/function.rs:83-89`](../../../../.source_1765633015/addr2line-0.25.1/src/function.rs#L83-L89)*

*Re-exported from `addr2line`*

#### Implementations

- <span id="inlinedfunction-parse"></span>`fn parse(state: &mut InlinedState<'_, R>, dw_die_offset: gimli::UnitOffset<<R as >::Offset>, abbrev: &gimli::Abbreviation, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>` — [`LayoutError`](#layouterror)

#### Trait Implementations

##### `impl Any for InlinedFunction<R>`

- <span id="inlinedfunction-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InlinedFunction<R>`

- <span id="inlinedfunction-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InlinedFunction<R>`

- <span id="inlinedfunction-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InlinedFunction<R>`

- <span id="inlinedfunction-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InlinedFunction<R>`

- <span id="inlinedfunction-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for InlinedFunction<R>`

- <span id="inlinedfunction-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inlinedfunction-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InlinedFunction<R>`

- <span id="inlinedfunction-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inlinedfunction-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Global`

```rust
struct Global;
```

*Defined in [`allocator-api2-0.2.21/src/stable/alloc/global.rs:18`](../../../../.source_1765633015/allocator-api2-0.2.21/src/stable/alloc/global.rs#L18)*

The global memory allocator.

This type implements the [`Allocator`](#allocator) trait by forwarding calls
to the allocator registered with the `#[global_allocator]` attribute
if there is one, or the `std` crate’s default.

Note: while this type is unstable, the functionality it provides can be
accessed through the [free functions in `alloc`](crate#functions).

#### Implementations

- <span id="global-alloc-impl"></span>`fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

- <span id="global-grow-impl"></span>`unsafe fn grow_impl(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

#### Trait Implementations

##### `impl Allocator for Global`

- <span id="global-allocator-allocate"></span>`fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

- <span id="global-allocator-allocate-zeroed"></span>`fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

- <span id="global-allocator-deallocate"></span>`unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)` — [`Layout`](#layout)

- <span id="global-allocator-grow"></span>`unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

- <span id="global-allocator-grow-zeroed"></span>`unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

- <span id="global-allocator-shrink"></span>`unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](#layout), [`AllocError`](#allocerror)

##### `impl Any for Global`

- <span id="global-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Global`

- <span id="global-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Global`

- <span id="global-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Global`

- <span id="global-clone"></span>`fn clone(&self) -> Global` — [`Global`](global/index.md#global)

##### `impl CloneToUninit for Global`

- <span id="global-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Global`

##### `impl Debug for Global`

- <span id="global-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Global`

- <span id="global-default"></span>`fn default() -> Global` — [`Global`](global/index.md#global)

##### `impl<T> From for Global`

- <span id="global-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Global`

- <span id="global-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Global`

- <span id="global-toowned-type-owned"></span>`type Owned = T`

- <span id="global-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="global-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Global`

- <span id="global-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="global-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Global`

- <span id="global-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="global-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AllocError`

```rust
struct AllocError;
```

*Defined in [`allocator-api2-0.2.21/src/stable/alloc/mod.rs:33`](../../../../.source_1765633015/allocator-api2-0.2.21/src/stable/alloc/mod.rs#L33)*

The `AllocError` error indicates an allocation failure
that may be due to resource exhaustion or to
something wrong when combining the given input arguments with this
allocator.

#### Trait Implementations

##### `impl Any for AllocError`

- <span id="allocerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AllocError`

- <span id="allocerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AllocError`

- <span id="allocerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AllocError`

- <span id="allocerror-clone"></span>`fn clone(&self) -> AllocError` — [`AllocError`](#allocerror)

##### `impl CloneToUninit for AllocError`

- <span id="allocerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AllocError`

##### `impl Debug for AllocError`

- <span id="allocerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for AllocError`

- <span id="allocerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AllocError`

##### `impl<T> From for AllocError`

- <span id="allocerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AllocError`

- <span id="allocerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AllocError`

- <span id="allocerror-partialeq-eq"></span>`fn eq(&self, other: &AllocError) -> bool` — [`AllocError`](#allocerror)

##### `impl StructuralPartialEq for AllocError`

##### `impl ToOwned for AllocError`

- <span id="allocerror-toowned-type-owned"></span>`type Owned = T`

- <span id="allocerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="allocerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for AllocError`

- <span id="allocerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for AllocError`

- <span id="allocerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="allocerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AllocError`

- <span id="allocerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="allocerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Allocator`

```rust
trait Allocator { ... }
```

*Defined in [`allocator-api2-0.2.21/src/stable/alloc/mod.rs:101-362`](../../../../.source_1765633015/allocator-api2-0.2.21/src/stable/alloc/mod.rs#L101-L362)*

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

- `fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to allocate a block of memory.

- `fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)`

  Deallocates the memory referenced by `ptr`.

#### Provided Methods

- `fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Behaves like `allocate`, but also ensures that the returned memory is zero-initialized.

- `fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to extend the memory block.

- `fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Behaves like `grow`, but also ensures that the new contents are set to zero before being

- `fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>`

  Attempts to shrink the memory block.

- `fn by_ref(&self) -> &Self`

  Creates a "by reference" adapter for this instance of `Allocator`.

#### Implementors

- [`Global`](global/index.md#global)
- `&A`

