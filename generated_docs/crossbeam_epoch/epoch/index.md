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

*Defined in [`crossbeam-epoch-0.9.18/src/epoch.rs:17-20`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/epoch.rs#L17-L20)*

An epoch that can be marked as pinned or unpinned.

Internally, the epoch is represented as an integer that wraps around at some unspecified point
and a flag that represents whether it is pinned or unpinned.

#### Fields

- **`data`**: `usize`

  The least significant bit is set if pinned. The rest of the bits hold the epoch.

#### Implementations

- <span id="epoch-starting"></span>`fn starting() -> Self`

  Returns the starting epoch in unpinned state.

- <span id="epoch-wrapping-sub"></span>`fn wrapping_sub(self, rhs: Self) -> isize`

  Returns the number of epochs `self` is ahead of `rhs`.

  

  Internally, epochs are represented as numbers in the range `(isize::MIN / 2) .. (isize::MAX

  / 2)`, so the returned distance will be in the same interval.

- <span id="epoch-is-pinned"></span>`fn is_pinned(self) -> bool`

  Returns `true` if the epoch is marked as pinned.

- <span id="epoch-pinned"></span>`fn pinned(self) -> Epoch` — [`Epoch`](#epoch)

  Returns the same epoch, but marked as pinned.

- <span id="epoch-unpinned"></span>`fn unpinned(self) -> Epoch` — [`Epoch`](#epoch)

  Returns the same epoch, but marked as unpinned.

- <span id="epoch-successor"></span>`fn successor(self) -> Epoch` — [`Epoch`](#epoch)

  Returns the successor epoch.

  

  The returned epoch will be marked as pinned only if the previous one was as well.

#### Trait Implementations

##### `impl Any for Epoch`

- <span id="epoch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Epoch`

- <span id="epoch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Epoch`

- <span id="epoch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Epoch`

- <span id="epoch-clone"></span>`fn clone(&self) -> Epoch` — [`Epoch`](#epoch)

##### `impl CloneToUninit for Epoch`

- <span id="epoch-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Epoch`

##### `impl Debug for Epoch`

- <span id="epoch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Epoch`

- <span id="epoch-default"></span>`fn default() -> Epoch` — [`Epoch`](#epoch)

##### `impl Eq for Epoch`

##### `impl<T> From for Epoch`

- <span id="epoch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Epoch`

- <span id="epoch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Epoch`

- <span id="epoch-partialeq-eq"></span>`fn eq(&self, other: &Epoch) -> bool` — [`Epoch`](#epoch)

##### `impl Pointable for Epoch`

- <span id="epoch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="epoch-pointable-type-init"></span>`type Init = T`

- <span id="epoch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="epoch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="epoch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="epoch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for Epoch`

##### `impl ToOwned for Epoch`

- <span id="epoch-toowned-type-owned"></span>`type Owned = T`

- <span id="epoch-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="epoch-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Epoch`

- <span id="epoch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="epoch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Epoch`

- <span id="epoch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="epoch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicEpoch`

```rust
struct AtomicEpoch {
    data: core::sync::atomic::AtomicUsize,
}
```

*Defined in [`crossbeam-epoch-0.9.18/src/epoch.rs:75-79`](../../../.source_1765633015/crossbeam-epoch-0.9.18/src/epoch.rs#L75-L79)*

An atomic value that holds an `Epoch`.

#### Fields

- **`data`**: `core::sync::atomic::AtomicUsize`

  Since `Epoch` is just a wrapper around `usize`, an `AtomicEpoch` is similarly represented
  using an `AtomicUsize`.

#### Implementations

- <span id="atomicepoch-new"></span>`fn new(epoch: Epoch) -> Self` — [`Epoch`](#epoch)

  Creates a new atomic epoch.

- <span id="atomicepoch-load"></span>`fn load(&self, ord: Ordering) -> Epoch` — [`Epoch`](#epoch)

  Loads a value from the atomic epoch.

- <span id="atomicepoch-store"></span>`fn store(&self, epoch: Epoch, ord: Ordering)` — [`Epoch`](#epoch)

  Stores a value into the atomic epoch.

- <span id="atomicepoch-compare-exchange"></span>`fn compare_exchange(&self, current: Epoch, new: Epoch, success: Ordering, failure: Ordering) -> Result<Epoch, Epoch>` — [`Epoch`](#epoch)

  Stores a value into the atomic epoch if the current value is the same as `current`.

  

  The return value is a result indicating whether the new value was written and containing

  the previous value. On success this value is guaranteed to be equal to `current`.

  

  This method takes two `Ordering` arguments to describe the memory

  ordering of this operation. `success` describes the required ordering for the

  read-modify-write operation that takes place if the comparison with `current` succeeds.

  `failure` describes the required ordering for the load operation that takes place when

  the comparison fails. Using `Acquire` as success ordering makes the store part

  of this operation `Relaxed`, and using `Release` makes the successful load

  `Relaxed`. The failure ordering can only be `SeqCst`, `Acquire` or `Relaxed`

  and must be equivalent to or weaker than the success ordering.

#### Trait Implementations

##### `impl Any for AtomicEpoch`

- <span id="atomicepoch-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicEpoch`

- <span id="atomicepoch-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicEpoch`

- <span id="atomicepoch-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicEpoch`

- <span id="atomicepoch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicEpoch`

- <span id="atomicepoch-default"></span>`fn default() -> AtomicEpoch` — [`AtomicEpoch`](#atomicepoch)

##### `impl<T> From for AtomicEpoch`

- <span id="atomicepoch-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicEpoch`

- <span id="atomicepoch-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Pointable for AtomicEpoch`

- <span id="atomicepoch-pointable-const-align"></span>`const ALIGN: usize`

- <span id="atomicepoch-pointable-type-init"></span>`type Init = T`

- <span id="atomicepoch-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize` — [`Pointable`](../atomic/index.md#pointable)

- <span id="atomicepoch-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="atomicepoch-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="atomicepoch-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for AtomicEpoch`

- <span id="atomicepoch-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicepoch-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicEpoch`

- <span id="atomicepoch-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicepoch-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

