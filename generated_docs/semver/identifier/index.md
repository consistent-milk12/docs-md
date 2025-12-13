*[semver](../index.md) / [identifier](index.md)*

---

# Module `identifier`

## Contents

- [Structs](#structs)
  - [`Identifier`](#identifier)
- [Functions](#functions)
  - [`ptr_to_repr`](#ptr-to-repr)
  - [`repr_to_ptr`](#repr-to-ptr)
  - [`repr_to_ptr_mut`](#repr-to-ptr-mut)
  - [`inline_len`](#inline-len)
  - [`inline_as_str`](#inline-as-str)
  - [`decode_len`](#decode-len)
  - [`ptr_as_str`](#ptr-as-str)
  - [`bytes_for_varint`](#bytes-for-varint)
- [Constants](#constants)
  - [`PTR_BYTES`](#ptr-bytes)
  - [`TAIL_BYTES`](#tail-bytes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Identifier`](#identifier) | struct |  |
| [`ptr_to_repr`](#ptr-to-repr) | fn |  |
| [`repr_to_ptr`](#repr-to-ptr) | fn |  |
| [`repr_to_ptr_mut`](#repr-to-ptr-mut) | fn |  |
| [`inline_len`](#inline-len) | fn |  |
| [`inline_as_str`](#inline-as-str) | fn |  |
| [`decode_len`](#decode-len) | fn |  |
| [`ptr_as_str`](#ptr-as-str) | fn |  |
| [`bytes_for_varint`](#bytes-for-varint) | fn |  |
| [`PTR_BYTES`](#ptr-bytes) | const |  |
| [`TAIL_BYTES`](#tail-bytes) | const |  |

## Structs

### `Identifier`

```rust
struct Identifier {
    head: core::ptr::NonNull<u8>,
    tail: [u8; 0],
}
```

*Defined in [`semver-1.0.27/src/identifier.rs:84-87`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L84-L87)*

#### Implementations

- <span id="identifier-empty"></span>`const fn empty() -> Self`

- <span id="identifier-new-unchecked"></span>`unsafe fn new_unchecked(string: &str) -> Self`

- <span id="identifier-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="identifier-is-inline"></span>`fn is_inline(&self) -> bool`

- <span id="identifier-is-empty-or-inline"></span>`fn is_empty_or_inline(&self) -> bool`

- <span id="identifier-as-str"></span>`fn as_str(&self) -> &str`

- <span id="identifier-ptr-eq"></span>`fn ptr_eq(&self, rhs: &Self) -> bool`

#### Trait Implementations

##### `impl Any for Identifier`

- <span id="identifier-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Identifier`

- <span id="identifier-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Identifier`

- <span id="identifier-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Identifier`

- <span id="identifier-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Identifier`

- <span id="identifier-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for crate::identifier::Identifier`

- <span id="crateidentifieridentifier-default"></span>`fn default() -> Self`

##### `impl Drop for Identifier`

- <span id="identifier-drop"></span>`fn drop(&mut self)`

##### `impl Eq for crate::identifier::Identifier`

##### `impl<T> From for Identifier`

- <span id="identifier-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for crate::identifier::Identifier`

- <span id="crateidentifieridentifier-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl<U> Into for Identifier`

- <span id="identifier-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Identifier`

- <span id="identifier-partialeq-eq"></span>`fn eq(&self, rhs: &Self) -> bool`

##### `impl Send for Identifier`

##### `impl Sync for Identifier`

##### `impl ToOwned for Identifier`

- <span id="identifier-toowned-type-owned"></span>`type Owned = T`

- <span id="identifier-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="identifier-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Identifier`

- <span id="identifier-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="identifier-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Identifier`

- <span id="identifier-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="identifier-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `ptr_to_repr`

```rust
fn ptr_to_repr(original: *mut u8) -> core::ptr::NonNull<u8>
```

*Defined in [`semver-1.0.27/src/identifier.rs:281-293`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L281-L293)*

### `repr_to_ptr`

```rust
fn repr_to_ptr(modified: core::ptr::NonNull<u8>) -> *const u8
```

*Defined in [`semver-1.0.27/src/identifier.rs:298-306`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L298-L306)*

### `repr_to_ptr_mut`

```rust
fn repr_to_ptr_mut(repr: core::ptr::NonNull<u8>) -> *mut u8
```

*Defined in [`semver-1.0.27/src/identifier.rs:308-310`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L308-L310)*

### `inline_len`

```rust
unsafe fn inline_len(repr: &Identifier) -> core::num::NonZeroUsize
```

*Defined in [`semver-1.0.27/src/identifier.rs:317-333`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L317-L333)*

### `inline_as_str`

```rust
unsafe fn inline_as_str(repr: &Identifier) -> &str
```

*Defined in [`semver-1.0.27/src/identifier.rs:337-347`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L337-L347)*

### `decode_len`

```rust
unsafe fn decode_len(ptr: *const u8) -> core::num::NonZeroUsize
```

*Defined in [`semver-1.0.27/src/identifier.rs:356-393`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L356-L393)*

### `ptr_as_str`

```rust
unsafe fn ptr_as_str(repr: &core::ptr::NonNull<u8>) -> &str
```

*Defined in [`semver-1.0.27/src/identifier.rs:397-405`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L397-L405)*

### `bytes_for_varint`

```rust
fn bytes_for_varint(len: core::num::NonZeroUsize) -> usize
```

*Defined in [`semver-1.0.27/src/identifier.rs:408-412`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L408-L412)*

## Constants

### `PTR_BYTES`
```rust
const PTR_BYTES: usize = 8usize;
```

*Defined in [`semver-1.0.27/src/identifier.rs:76`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L76)*

### `TAIL_BYTES`
```rust
const TAIL_BYTES: usize = 0usize;
```

*Defined in [`semver-1.0.27/src/identifier.rs:81`](../../../.source_1765633015/semver-1.0.27/src/identifier.rs#L81)*

