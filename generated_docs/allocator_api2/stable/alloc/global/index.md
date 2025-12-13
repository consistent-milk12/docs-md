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

*Defined in [`allocator-api2-0.2.21/src/stable/alloc/global.rs:18`](../../../../../.source_1765633015/allocator-api2-0.2.21/src/stable/alloc/global.rs#L18)*

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

- <span id="global-allocator-allocate"></span>`fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-allocator-allocate-zeroed"></span>`fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-allocator-deallocate"></span>`unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout)` — [`Layout`](../index.md#layout)

- <span id="global-allocator-grow"></span>`unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-allocator-grow-zeroed"></span>`unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

- <span id="global-allocator-shrink"></span>`unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<NonNull<[u8]>, AllocError>` — [`Layout`](../index.md#layout), [`AllocError`](../index.md#allocerror)

##### `impl Any for Global`

- <span id="global-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Global`

- <span id="global-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Global`

- <span id="global-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Global`

- <span id="global-clone"></span>`fn clone(&self) -> Global` — [`Global`](#global)

##### `impl CloneToUninit for Global`

- <span id="global-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Global`

##### `impl Debug for Global`

- <span id="global-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Global`

- <span id="global-default"></span>`fn default() -> Global` — [`Global`](#global)

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

