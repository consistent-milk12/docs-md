*[memchr](../index.md) / [cow](index.md)*

---

# Module `cow`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CowBytes`](#cowbytes) | struct | A specialized copy-on-write byte string. |
| [`Imp`](#imp) | enum |  |

## Structs

### `CowBytes<'a>`

```rust
struct CowBytes<'a>(Imp<'a>);
```

*Defined in [`memchr-2.7.6/src/cow.rs:11`](../../../.source_1765633015/memchr-2.7.6/src/cow.rs#L11)*

A specialized copy-on-write byte string.

The purpose of this type is to permit usage of a "borrowed or owned
byte string" in a way that keeps std/no-std compatibility. That is, in
no-std/alloc mode, this type devolves into a simple &[u8] with no owned
variant available. We can't just use a plain Cow because Cow is not in
core.

#### Implementations

- <span id="cowbytes-new"></span>`fn new<B: ?Sized + AsRef<[u8]>>(bytes: &'a B) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

  Create a new borrowed CowBytes.

- <span id="cowbytes-new-owned"></span>`fn new_owned(bytes: alloc::boxed::Box<[u8]>) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

  Create a new owned CowBytes.

- <span id="cowbytes-as-slice"></span>`fn as_slice(&self) -> &[u8]`

  Return a borrowed byte string, regardless of whether this is an owned

  or borrowed byte string internally.

- <span id="cowbytes-into-owned"></span>`fn into_owned(self) -> CowBytes<'static>` — [`CowBytes`](#cowbytes)

  Return an owned version of this copy-on-write byte string.

  

  If this is already an owned byte string internally, then this is a

  no-op. Otherwise, the internal byte string is copied.

#### Trait Implementations

##### `impl Any for CowBytes<'a>`

- <span id="cowbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CowBytes<'a>`

- <span id="cowbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CowBytes<'a>`

- <span id="cowbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CowBytes<'a>`

- <span id="cowbytes-clone"></span>`fn clone(&self) -> CowBytes<'a>` — [`CowBytes`](#cowbytes)

##### `impl CloneToUninit for CowBytes<'a>`

- <span id="cowbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CowBytes<'a>`

- <span id="cowbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for CowBytes<'a>`

- <span id="cowbytes-deref-type-target"></span>`type Target = [u8]`

- <span id="cowbytes-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl<T> From for CowBytes<'a>`

- <span id="cowbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CowBytes<'a>`

- <span id="cowbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Receiver for CowBytes<'a>`

- <span id="cowbytes-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for CowBytes<'a>`

- <span id="cowbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="cowbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cowbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CowBytes<'a>`

- <span id="cowbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cowbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CowBytes<'a>`

- <span id="cowbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cowbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Imp<'a>`

```rust
enum Imp<'a> {
    Borrowed(&'a [u8]),
    Owned(alloc::boxed::Box<[u8]>),
}
```

*Defined in [`memchr-2.7.6/src/cow.rs:18-21`](../../../.source_1765633015/memchr-2.7.6/src/cow.rs#L18-L21)*

#### Implementations

- <span id="imp-new"></span>`fn new(bytes: &'a [u8]) -> Imp<'a>` — [`Imp`](#imp)

- <span id="imp-as-slice"></span>`fn as_slice(&self) -> &[u8]`

#### Trait Implementations

##### `impl Any for Imp<'a>`

- <span id="imp-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Imp<'a>`

- <span id="imp-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Imp<'a>`

- <span id="imp-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Imp<'a>`

- <span id="imp-clone"></span>`fn clone(&self) -> Imp<'a>` — [`Imp`](#imp)

##### `impl CloneToUninit for Imp<'a>`

- <span id="imp-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Imp<'a>`

- <span id="imp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Imp<'a>`

- <span id="imp-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Imp<'a>`

- <span id="imp-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Imp<'a>`

- <span id="imp-toowned-type-owned"></span>`type Owned = T`

- <span id="imp-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="imp-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Imp<'a>`

- <span id="imp-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="imp-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Imp<'a>`

- <span id="imp-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="imp-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

