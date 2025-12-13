*[rustix](../index.md) / [pid](index.md)*

---

# Module `pid`

The `Pid` type.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pid`](#pid) | struct | `pid_t`—A non-zero Unix process ID. |
| [`RawPid`](#rawpid) | type | A process identifier as a raw integer. |

## Structs

### `Pid`

```rust
struct Pid(core::num::NonZeroI32);
```

*Defined in [`rustix-1.1.2/src/pid.rs:19`](../../../.source_1765521767/rustix-1.1.2/src/pid.rs#L19)*

`pid_t`—A non-zero Unix process ID.

This is a pid, and not a pidfd. It is not a file descriptor, and the
process it refers to could disappear at any time and be replaced by
another, unrelated, process.

On Linux, `Pid` values are also used to identify threads.

#### Implementations

- <span id="pid-const-init"></span>`const INIT: Self`

- <span id="pid-from-raw"></span>`const fn from_raw(raw: i32) -> Option<Self>`

  Converts a `RawPid` into a `Pid`.

  

  Returns `Some` for positive values, and `None` for zero values.

  

  This is safe because a `Pid` is a number without any guarantees for the

  kernel. Non-child `Pid`s are always racy for any syscalls, but can only

  cause logic errors. If you want race-free access to or control of

  non-child processes, please consider other mechanisms like [pidfd] on

  Linux.

  

  Passing a negative number doesn't invoke undefined behavior, but it

  may cause unexpected behavior.

- <span id="pid-from-raw-unchecked"></span>`const unsafe fn from_raw_unchecked(raw: i32) -> Self`

  Converts a known positive `RawPid` into a `Pid`.

  

  Passing a negative number doesn't invoke undefined behavior, but it

  may cause unexpected behavior.

  

  # Safety

  

  The caller must guarantee `raw` is non-zero.

- <span id="pid-from-child"></span>`fn from_child(child: &std::process::Child) -> Self`

  Creates a `Pid` holding the ID of the given child process.

- <span id="pid-as-raw-nonzero"></span>`const fn as_raw_nonzero(self) -> NonZeroI32`

  Converts a `Pid` into a `NonZeroI32`.

- <span id="pid-as-raw-pid"></span>`const fn as_raw_pid(self) -> i32`

  Converts a `Pid` into a `RawPid`.

  

  This is the same as `self.as_raw_nonzero().get()`.

- <span id="pid-as-raw"></span>`const fn as_raw(pid: Option<Self>) -> i32`

  Converts an `Option<Pid>` into a `RawPid`.

- <span id="pid-is-init"></span>`const fn is_init(self) -> bool`

  Test whether this pid represents the init process (`Pid::INIT`).

#### Trait Implementations

##### `impl Any for Pid`

- <span id="pid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for Pid`

- <span id="pid-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for Pid`

- <span id="pid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pid`

- <span id="pid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pid`

- <span id="pid-clone"></span>`fn clone(&self) -> Pid` — [`Pid`](#pid)

##### `impl CloneToUninit for Pid`

- <span id="pid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pid`

##### `impl Debug for Pid`

- <span id="pid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Pid`

- <span id="pid-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Pid`

##### `impl<T> From for Pid`

- <span id="pid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Pid`

- <span id="pid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Pid`

- <span id="pid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl LowerExp for Pid`

- <span id="pid-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl LowerHex for Pid`

- <span id="pid-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Octal for Pid`

- <span id="pid-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Pid`

- <span id="pid-partialeq-eq"></span>`fn eq(&self, other: &Pid) -> bool` — [`Pid`](#pid)

##### `impl StructuralPartialEq for Pid`

##### `impl ToOwned for Pid`

- <span id="pid-toowned-type-owned"></span>`type Owned = T`

- <span id="pid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Pid`

- <span id="pid-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Pid`

- <span id="pid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pid`

- <span id="pid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperExp for Pid`

- <span id="pid-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl UpperHex for Pid`

- <span id="pid-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `RawPid`

```rust
type RawPid = i32;
```

*Defined in [`rustix-1.1.2/src/pid.rs:8`](../../../.source_1765521767/rustix-1.1.2/src/pid.rs#L8)*

A process identifier as a raw integer.

