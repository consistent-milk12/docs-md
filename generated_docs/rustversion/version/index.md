*[rustversion](../index.md) / [version](index.md)*

---

# Module `version`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Version`](#version) | struct |  |
| [`Channel`](#channel) | enum |  |

## Structs

### `Version`

```rust
struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}
```

*Defined in [`rustversion-1.0.22/src/version.rs:6-10`](../../../.source_1765633015/rustversion-1.0.22/src/version.rs#L6-L10)*

#### Trait Implementations

##### `impl Any for Version`

- <span id="version-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Version`

- <span id="version-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Version`

- <span id="version-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Version`

- <span id="version-clone"></span>`fn clone(&self) -> Version` — [`Version`](#version)

##### `impl CloneToUninit for Version`

- <span id="version-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Version`

##### `impl Debug for Version`

- <span id="version-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Version`

- <span id="version-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Version`

- <span id="version-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for crate::version::Version`

- <span id="crateversionversion-partialeq-eq"></span>`fn eq(&self, rhs: &Bound) -> bool` — [`Bound`](../bound/index.md#bound)

##### `impl PartialOrd for crate::version::Version`

- <span id="crateversionversion-partialord-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](../bound/index.md#bound)

##### `impl StructuralPartialEq for Version`

##### `impl ToOwned for Version`

- <span id="version-toowned-type-owned"></span>`type Owned = T`

- <span id="version-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="version-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Version`

- <span id="version-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="version-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Version`

- <span id="version-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="version-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Channel`

```rust
enum Channel {
    Stable,
    Beta,
    Nightly(crate::date::Date),
    Dev,
}
```

*Defined in [`rustversion-1.0.22/src/version.rs:13-18`](../../../.source_1765633015/rustversion-1.0.22/src/version.rs#L13-L18)*

#### Trait Implementations

##### `impl Any for Channel`

- <span id="channel-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Channel`

- <span id="channel-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Channel`

- <span id="channel-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Channel`

- <span id="channel-clone"></span>`fn clone(&self) -> Channel` — [`Channel`](#channel)

##### `impl CloneToUninit for Channel`

- <span id="channel-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Channel`

##### `impl Debug for Channel`

- <span id="channel-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Channel`

- <span id="channel-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Channel`

- <span id="channel-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Channel`

- <span id="channel-partialeq-eq"></span>`fn eq(&self, other: &Channel) -> bool` — [`Channel`](#channel)

##### `impl StructuralPartialEq for Channel`

##### `impl ToOwned for Channel`

- <span id="channel-toowned-type-owned"></span>`type Owned = T`

- <span id="channel-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="channel-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Channel`

- <span id="channel-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="channel-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Channel`

- <span id="channel-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="channel-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

