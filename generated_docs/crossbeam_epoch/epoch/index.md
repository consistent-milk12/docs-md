*[crossbeam_epoch](../index.md) / [epoch](index.md)*

---

# Module `epoch`

The global epoch

The last bit in this number is unused and is always zero. Every so often the global epoch is
incremented, i.e. we say it "advances". A pinned participant may advance the global epoch only
if all currently pinned participants have been pinned in the current epoch.

If an object became garbage in some epoch, then we can be sure that after two advancements no
participant will hold a reference to it. That is the crux of safe memory reclamation.

## Structs

### `Epoch`

```rust
struct Epoch {
    data: usize,
}
```

An epoch that can be marked as pinned or unpinned.

Internally, the epoch is represented as an integer that wraps around at some unspecified point
and a flag that represents whether it is pinned or unpinned.

#### Fields

- **`data`**: `usize`

  The least significant bit is set if pinned. The rest of the bits hold the epoch.

#### Implementations

- `fn starting() -> Self`

- `fn wrapping_sub(self: Self, rhs: Self) -> isize`

- `fn is_pinned(self: Self) -> bool`

- `fn pinned(self: Self) -> Epoch` — [`Epoch`](#epoch)

- `fn unpinned(self: Self) -> Epoch` — [`Epoch`](#epoch)

- `fn successor(self: Self) -> Epoch` — [`Epoch`](#epoch)

#### Trait Implementations

##### `impl Clone for Epoch`

- `fn clone(self: &Self) -> Epoch` — [`Epoch`](#epoch)

##### `impl Copy for Epoch`

##### `impl Debug for Epoch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Epoch`

- `fn default() -> Epoch` — [`Epoch`](#epoch)

##### `impl Eq for Epoch`

##### `impl PartialEq for Epoch`

- `fn eq(self: &Self, other: &Epoch) -> bool` — [`Epoch`](#epoch)

##### `impl<T> Pointable for Epoch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Epoch`

### `AtomicEpoch`

```rust
struct AtomicEpoch {
    data: core::sync::atomic::AtomicUsize,
}
```

An atomic value that holds an `Epoch`.

#### Fields

- **`data`**: `core::sync::atomic::AtomicUsize`

  Since `Epoch` is just a wrapper around `usize`, an `AtomicEpoch` is similarly represented
  using an `AtomicUsize`.

#### Implementations

- `fn new(epoch: Epoch) -> Self` — [`Epoch`](#epoch)

- `fn load(self: &Self, ord: Ordering) -> Epoch` — [`Epoch`](#epoch)

- `fn store(self: &Self, epoch: Epoch, ord: Ordering)` — [`Epoch`](#epoch)

- `fn compare_exchange(self: &Self, current: Epoch, new: Epoch, success: Ordering, failure: Ordering) -> Result<Epoch, Epoch>` — [`Epoch`](#epoch)

#### Trait Implementations

##### `impl Debug for AtomicEpoch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AtomicEpoch`

- `fn default() -> AtomicEpoch` — [`AtomicEpoch`](#atomicepoch)

##### `impl<T> Pointable for AtomicEpoch`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../index.md)

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

