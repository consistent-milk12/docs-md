*[once_cell](../index.md) / [imp](index.md)*

---

# Module `imp`

## Contents

- [Modules](#modules)
  - [`strict`](#strict)
- [Structs](#structs)
  - [`OnceCell`](#oncecell)
  - [`Waiter`](#waiter)
  - [`Guard`](#guard)
- [Functions](#functions)
  - [`initialize_or_wait`](#initialize-or-wait)
  - [`wait`](#wait)
- [Constants](#constants)
  - [`INCOMPLETE`](#incomplete)
  - [`RUNNING`](#running)
  - [`COMPLETE`](#complete)
  - [`INCOMPLETE_PTR`](#incomplete-ptr)
  - [`COMPLETE_PTR`](#complete-ptr)
  - [`STATE_MASK`](#state-mask)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`strict`](#strict) | mod |  |
| [`OnceCell`](#oncecell) | struct |  |
| [`Waiter`](#waiter) | struct | Representation of a node in the linked list of waiters in the RUNNING state. |
| [`Guard`](#guard) | struct | Drains and notifies the queue of waiters on drop. |
| [`initialize_or_wait`](#initialize-or-wait) | fn |  |
| [`wait`](#wait) | fn |  |
| [`INCOMPLETE`](#incomplete) | const |  |
| [`RUNNING`](#running) | const |  |
| [`COMPLETE`](#complete) | const |  |
| [`INCOMPLETE_PTR`](#incomplete-ptr) | const |  |
| [`COMPLETE_PTR`](#complete-ptr) | const |  |
| [`STATE_MASK`](#state-mask) | const |  |

## Modules

- [`strict`](strict/index.md)

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    queue: std::sync::atomic::AtomicPtr<Waiter>,
    value: std::cell::UnsafeCell<Option<T>>,
}
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:14-25`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L14-L25)*

#### Implementations

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-is-initialized"></span>`fn is_initialized(&self) -> bool`

  Safety: synchronizes with store to value via Release/(Acquire|SeqCst).

- <span id="oncecell-initialize"></span>`fn initialize<F, E>(&self, f: F) -> Result<(), E>`

  Safety: synchronizes with store to value via SeqCst read from state,

  writes value only once because we never get to INCOMPLETE state after a

  successful write.

- <span id="oncecell-wait"></span>`fn wait(&self)`

- <span id="oncecell-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

  Get the reference to the underlying value, without checking if the cell

  is initialized.

  

  # Safety

  

  Caller must ensure that the cell is in initialized state, and that

  the contents are acquired by (synchronized to) this thread.

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

  Gets the mutable reference to the underlying value.

  Returns `None` if the cell is empty.

- <span id="oncecell-into-inner"></span>`fn into_inner(self) -> Option<T>`

  Consumes this `OnceCell`, returning the wrapped value.

  Returns `None` if the cell was empty.

#### Trait Implementations

##### `impl<T> Any for OnceCell<T>`

- <span id="oncecell-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnceCell<T>`

- <span id="oncecell-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnceCell<T>`

- <span id="oncecell-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for OnceCell<T>`

- <span id="oncecell-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OnceCell<T>`

- <span id="oncecell-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OnceCell<T>`

- <span id="oncecell-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T>`

##### `impl<T: Send> Send for OnceCell<T>`

##### `impl<T: Sync + Send> Sync for OnceCell<T>`

##### `impl<T, U> TryFrom for OnceCell<T>`

- <span id="oncecell-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oncecell-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OnceCell<T>`

- <span id="oncecell-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oncecell-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: UnwindSafe> UnwindSafe for OnceCell<T>`

### `Waiter`

```rust
struct Waiter {
    thread: std::cell::Cell<Option<std::thread::Thread>>,
    signaled: std::sync::atomic::AtomicBool,
    next: *mut Waiter,
}
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:138-142`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L138-L142)*

Representation of a node in the linked list of waiters in the RUNNING state.
A waiters is stored on the stack of the waiting threads.

#### Trait Implementations

##### `impl Any for Waiter`

- <span id="waiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Waiter`

- <span id="waiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Waiter`

- <span id="waiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Waiter`

- <span id="waiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Waiter`

- <span id="waiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Waiter`

- <span id="waiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="waiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Waiter`

- <span id="waiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="waiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Guard<'a>`

```rust
struct Guard<'a> {
    queue: &'a std::sync::atomic::AtomicPtr<Waiter>,
    new_queue: *mut Waiter,
}
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:145-148`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L145-L148)*

Drains and notifies the queue of waiters on drop.

#### Trait Implementations

##### `impl Any for Guard<'a>`

- <span id="guard-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Guard<'a>`

- <span id="guard-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Guard<'a>`

- <span id="guard-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Drop for Guard<'_>`

- <span id="guard-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Guard<'a>`

- <span id="guard-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Guard<'a>`

- <span id="guard-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Guard<'a>`

- <span id="guard-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="guard-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Guard<'a>`

- <span id="guard-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="guard-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `initialize_or_wait`

```rust
fn initialize_or_wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, init: Option<&mut dyn FnMut() -> bool>)
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:177-208`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L177-L208)*

### `wait`

```rust
fn wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, curr_queue: *mut Waiter)
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:210-239`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L210-L239)*

## Constants

### `INCOMPLETE`
```rust
const INCOMPLETE: usize = 0usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:125`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L125)*

### `RUNNING`
```rust
const RUNNING: usize = 1usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:126`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L126)*

### `COMPLETE`
```rust
const COMPLETE: usize = 2usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:127`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L127)*

### `INCOMPLETE_PTR`
```rust
const INCOMPLETE_PTR: *mut Waiter = {0x0 as *mut imp::Waiter};
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:128`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L128)*

### `COMPLETE_PTR`
```rust
const COMPLETE_PTR: *mut Waiter = {0x2 as *mut imp::Waiter};
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:129`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L129)*

### `STATE_MASK`
```rust
const STATE_MASK: usize = 3usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:133`](../../../.source_1765633015/once_cell-1.21.3/src/imp_std.rs#L133)*

