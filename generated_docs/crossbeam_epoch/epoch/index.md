*[crossbeam_epoch](../index.md) / [epoch](index.md)*

---

# Module `epoch`

The global epoch

The last bit in this number is unused and is always zero. Every so often the global epoch is
incremented, i.e. we say it "advances". A pinned participant may advance the global epoch only
if all currently pinned participants have been pinned in the current epoch.

If an object became garbage in some epoch, then we can be sure that after two advancements no
participant will hold a reference to it. That is the crux of safe memory reclamation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Epoch`](#epoch) | struct | An epoch that can be marked as pinned or unpinned. |
| [`AtomicEpoch`](#atomicepoch) | struct | An atomic value that holds an `Epoch`. |

## Structs

### `Epoch`

```rust
struct Epoch {
    data: usize,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/epoch.rs:17-20`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/epoch.rs#L17-L20)*

An epoch that can be marked as pinned or unpinned.

Internally, the epoch is represented as an integer that wraps around at some unspecified point
and a flag that represents whether it is pinned or unpinned.

#### Fields

- **`data`**: `usize`

  The least significant bit is set if pinned. The rest of the bits hold the epoch.

#### Implementations

- <span id="epoch-starting"></span>`fn starting() -> Self`

- <span id="epoch-wrapping-sub"></span>`fn wrapping_sub(self, rhs: Self) -> isize`

- <span id="epoch-is-pinned"></span>`fn is_pinned(self) -> bool`

- <span id="epoch-pinned"></span>`fn pinned(self) -> Epoch` — [`Epoch`](#epoch)

- <span id="epoch-unpinned"></span>`fn unpinned(self) -> Epoch` — [`Epoch`](#epoch)

- <span id="epoch-successor"></span>`fn successor(self) -> Epoch` — [`Epoch`](#epoch)

#### Trait Implementations

##### `impl Clone for Epoch`

- <span id="epoch-clone"></span>`fn clone(&self) -> Epoch` — [`Epoch`](#epoch)

##### `impl Copy for Epoch`

##### `impl Debug for Epoch`

- <span id="epoch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Epoch`

- <span id="epoch-default"></span>`fn default() -> Epoch` — [`Epoch`](#epoch)

##### `impl Eq for Epoch`

##### `impl PartialEq for Epoch`

- <span id="epoch-eq"></span>`fn eq(&self, other: &Epoch) -> bool` — [`Epoch`](#epoch)

##### `impl Pointable for Epoch`

- <span id="epoch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="epoch-pointable-type-init"></span>`type Init = T`

- <span id="epoch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="epoch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="epoch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="epoch-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Epoch`

### `AtomicEpoch`

```rust
struct AtomicEpoch {
    data: core::sync::atomic::AtomicUsize,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/epoch.rs:75-79`](../../../.source_1765210505/crossbeam-epoch-0.9.18/src/epoch.rs#L75-L79)*

An atomic value that holds an `Epoch`.

#### Fields

- **`data`**: `core::sync::atomic::AtomicUsize`

  Since `Epoch` is just a wrapper around `usize`, an `AtomicEpoch` is similarly represented
  using an `AtomicUsize`.

#### Implementations

- <span id="atomicepoch-new"></span>`fn new(epoch: Epoch) -> Self` — [`Epoch`](#epoch)

- <span id="atomicepoch-load"></span>`fn load(&self, ord: Ordering) -> Epoch` — [`Epoch`](#epoch)

- <span id="atomicepoch-store"></span>`fn store(&self, epoch: Epoch, ord: Ordering)` — [`Epoch`](#epoch)

- <span id="atomicepoch-compare-exchange"></span>`fn compare_exchange(&self, current: Epoch, new: Epoch, success: Ordering, failure: Ordering) -> Result<Epoch, Epoch>` — [`Epoch`](#epoch)

#### Trait Implementations

##### `impl Debug for AtomicEpoch`

- <span id="atomicepoch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicEpoch`

- <span id="atomicepoch-default"></span>`fn default() -> AtomicEpoch` — [`AtomicEpoch`](#atomicepoch)

##### `impl Pointable for AtomicEpoch`

- <span id="atomicepoch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomicepoch-pointable-type-init"></span>`type Init = T`

- <span id="atomicepoch-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="atomicepoch-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomicepoch-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomicepoch-drop"></span>`unsafe fn drop(ptr: usize)`

