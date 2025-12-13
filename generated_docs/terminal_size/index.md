# Crate `terminal_size`

A simple utility for getting the size of a terminal.

Works on Linux, macOS, Windows, and illumos.

This crate requires a minimum Rust version of 1.63.0 (2022-08-11).

# Example

```rust
use terminal_size::{Width, Height, terminal_size};

let size = terminal_size();
if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
} else {
    println!("Unable to get terminal size");
}
```


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`unix`](#unix) | mod |  |
| [`Width`](#width) | struct |  |
| [`Height`](#height) | struct |  |
| [`terminal_size`](#terminal-size) | fn |  |
| [`terminal_size_of`](#terminal-size-of) | fn |  |
| [`terminal_size_using_fd`](#terminal-size-using-fd) | fn |  |

## Modules

- [`unix`](unix/index.md)

## Structs

### `Width`

```rust
struct Width(u16);
```

*Defined in [`terminal_size-0.4.3/src/lib.rs:22`](../../.source_1765633015/terminal_size-0.4.3/src/lib.rs#L22)*

#### Trait Implementations

##### `impl Any for Width`

- <span id="width-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Width`

- <span id="width-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Width`

- <span id="width-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Width`

- <span id="width-clone"></span>`fn clone(&self) -> Width` — [`Width`](#width)

##### `impl CloneToUninit for Width`

- <span id="width-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Width`

##### `impl Debug for Width`

- <span id="width-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Width`

##### `impl<T> From for Width`

- <span id="width-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Width`

- <span id="width-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Width`

- <span id="width-ord-cmp"></span>`fn cmp(&self, other: &Width) -> cmp::Ordering` — [`Width`](#width)

##### `impl PartialEq for Width`

- <span id="width-partialeq-eq"></span>`fn eq(&self, other: &Width) -> bool` — [`Width`](#width)

##### `impl PartialOrd for Width`

- <span id="width-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Width) -> option::Option<cmp::Ordering>` — [`Width`](#width)

##### `impl StructuralPartialEq for Width`

##### `impl ToOwned for Width`

- <span id="width-toowned-type-owned"></span>`type Owned = T`

- <span id="width-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="width-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Width`

- <span id="width-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="width-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Width`

- <span id="width-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="width-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Height`

```rust
struct Height(u16);
```

*Defined in [`terminal_size-0.4.3/src/lib.rs:24`](../../.source_1765633015/terminal_size-0.4.3/src/lib.rs#L24)*

#### Trait Implementations

##### `impl Any for Height`

- <span id="height-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Height`

- <span id="height-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Height`

- <span id="height-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Height`

- <span id="height-clone"></span>`fn clone(&self) -> Height` — [`Height`](#height)

##### `impl CloneToUninit for Height`

- <span id="height-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Height`

##### `impl Debug for Height`

- <span id="height-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Height`

##### `impl<T> From for Height`

- <span id="height-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Height`

- <span id="height-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Height`

- <span id="height-ord-cmp"></span>`fn cmp(&self, other: &Height) -> cmp::Ordering` — [`Height`](#height)

##### `impl PartialEq for Height`

- <span id="height-partialeq-eq"></span>`fn eq(&self, other: &Height) -> bool` — [`Height`](#height)

##### `impl PartialOrd for Height`

- <span id="height-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Height) -> option::Option<cmp::Ordering>` — [`Height`](#height)

##### `impl StructuralPartialEq for Height`

##### `impl ToOwned for Height`

- <span id="height-toowned-type-owned"></span>`type Owned = T`

- <span id="height-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="height-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Height`

- <span id="height-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="height-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Height`

- <span id="height-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="height-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `terminal_size`

```rust
fn terminal_size() -> Option<(super::Width, super::Height)>
```

*Defined in [`terminal_size-0.4.3/src/unix.rs:9-19`](../../.source_1765633015/terminal_size-0.4.3/src/unix.rs#L9-L19)*

Returns the size of the terminal.

This function checks the stdout, stderr, and stdin streams (in that order).
The size of the first stream that is a TTY will be returned.  If nothing
is a TTY, then `None` is returned.

### `terminal_size_of`

```rust
fn terminal_size_of<Fd: AsFd>(fd: Fd) -> Option<(super::Width, super::Height)>
```

*Defined in [`terminal_size-0.4.3/src/unix.rs:24-41`](../../.source_1765633015/terminal_size-0.4.3/src/unix.rs#L24-L41)*

Returns the size of the terminal using the given file descriptor, if available.

If the given file descriptor is not a tty, returns `None`

### `terminal_size_using_fd`

```rust
unsafe fn terminal_size_using_fd(fd: std::os::unix::io::RawFd) -> Option<(super::Width, super::Height)>
```

*Defined in [`terminal_size-0.4.3/src/unix.rs:54-56`](../../.source_1765633015/terminal_size-0.4.3/src/unix.rs#L54-L56)*

Returns the size of the terminal using the given raw file descriptor, if available.

The given file descriptor must be an open file descriptor.

If the given file descriptor is not a tty, returns `None`

# Safety

`fd` must be a valid open file descriptor.

