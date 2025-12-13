*[rustix](../../index.md) / [io](../index.md) / [errno](index.md)*

---

# Module `errno`

The `Errno` type, which is a minimal wrapper around an error code.

We define the error constants as individual `const`s instead of an enum
because we may not know about all of the host's error values and we don't
want unrecognized values to create undefined behavior.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Errno`](#errno) | struct |  |
| [`retry_on_intr`](#retry-on-intr) | fn | Call `f` until it either succeeds or fails other than [`Errno::INTR`]. |
| [`Result`](#result) | type | A specialized [`Result`] type for `rustix` APIs. |

## Structs

### `Errno`

```rust
struct Errno(u16);
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/io/errno.rs:51`](../../../../.source_1765521767/rustix-1.1.2/src/backend/linux_raw/io/errno.rs#L51)*

`errno`—An error code.

The error type for `rustix` APIs. This is similar to [`std::io::Error`](../../../cargo_docs_md/error/index.md),
but only holds an OS error code, and no extra error value.

# References
 - [POSIX]
 - [Linux]
 - [Winsock]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]
 - [DragonFly BSD]
 - [illumos]
 - [`glibc`](../../../libc/new/glibc/index.md)










#### Implementations

- <span id="errno-from-io-error"></span>`fn from_io_error(io_err: &std::io::Error) -> Option<Self>`

  Extract an `Errno` value from a `std::io::Error`.

  

  This isn't a `From` conversion because it's expected to be relatively

  uncommon.

- <span id="errno-raw-os-error"></span>`const fn raw_os_error(self) -> i32`

  Extract the raw OS error number from this error.

- <span id="errno-from-raw-os-error"></span>`const fn from_raw_os_error(raw: i32) -> Self`

  Construct an `Errno` from a raw OS error number.

- <span id="errno-from-errno"></span>`const fn from_errno(raw: u32) -> Self`

  Convert from a C `errno` value (which is positive) to an `Errno`.

#### Trait Implementations

##### `impl Any for Errno`

- <span id="errno-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Errno`

- <span id="errno-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Errno`

- <span id="errno-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Errno`

- <span id="errno-clone"></span>`fn clone(&self) -> Errno` — [`Errno`](../../backend/io/errno/index.md#errno)

##### `impl CloneToUninit for Errno`

- <span id="errno-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Errno`

##### `impl Debug for Errno`

- <span id="errno-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Errno`

- <span id="errno-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Errno`

##### `impl Error for Errno`

##### `impl<T> From for Errno`

- <span id="errno-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Errno`

- <span id="errno-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Errno`

- <span id="errno-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Errno`

- <span id="errno-partialeq-eq"></span>`fn eq(&self, other: &Errno) -> bool` — [`Errno`](../../backend/io/errno/index.md#errno)

##### `impl StructuralPartialEq for Errno`

##### `impl ToOwned for Errno`

- <span id="errno-toowned-type-owned"></span>`type Owned = T`

- <span id="errno-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errno-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Errno`

- <span id="errno-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Errno`

- <span id="errno-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errno-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Errno`

- <span id="errno-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errno-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `retry_on_intr`

```rust
fn retry_on_intr<T, F: FnMut() -> Result<T>>(f: F) -> Result<T>
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:67-74`](../../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L67-L74)*

Call `f` until it either succeeds or fails other than `Errno::INTR`.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Errno>;
```

*Defined in [`rustix-1.1.2/src/io/errno.rs:15`](../../../../.source_1765521767/rustix-1.1.2/src/io/errno.rs#L15)*

A specialized [`Result`](#result) type for `rustix` APIs.

