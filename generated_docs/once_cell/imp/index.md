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
  - [`initialize_or_wait`](#initialize_or_wait)
  - [`wait`](#wait)
- [Constants](#constants)
  - [`INCOMPLETE`](#incomplete)
  - [`RUNNING`](#running)
  - [`COMPLETE`](#complete)
  - [`INCOMPLETE_PTR`](#incomplete_ptr)
  - [`COMPLETE_PTR`](#complete_ptr)
  - [`STATE_MASK`](#state_mask)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`strict`](#strict) | mod |  |
| [`OnceCell`](#oncecell) | struct |  |
| [`Waiter`](#waiter) | struct | Representation of a node in the linked list of waiters in the RUNNING state. |
| [`Guard`](#guard) | struct | Drains and notifies the queue of waiters on drop. |
| [`initialize_or_wait`](#initialize_or_wait) | fn |  |
| [`wait`](#wait) | fn |  |
| [`INCOMPLETE`](#incomplete) | const |  |
| [`RUNNING`](#running) | const |  |
| [`COMPLETE`](#complete) | const |  |
| [`INCOMPLETE_PTR`](#incomplete_ptr) | const |  |
| [`COMPLETE_PTR`](#complete_ptr) | const |  |
| [`STATE_MASK`](#state_mask) | const |  |

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

*Defined in [`once_cell-1.21.3/src/imp_std.rs:14-25`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L14-L25)*

#### Implementations

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-is-initialized"></span>`fn is_initialized(&self) -> bool`

- <span id="oncecell-initialize"></span>`fn initialize<F, E>(&self, f: F) -> Result<(), E>`

- <span id="oncecell-wait"></span>`fn wait(&self)`

- <span id="oncecell-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

- <span id="oncecell-into-inner"></span>`fn into_inner(self) -> Option<T>`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for OnceCell<T>`

- <span id="oncecell-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T>`

##### `impl<T: Send> Send for OnceCell<T>`

##### `impl<T: Sync + Send> Sync for OnceCell<T>`

##### `impl<T: UnwindSafe> UnwindSafe for OnceCell<T>`

### `Waiter`

```rust
struct Waiter {
    thread: std::cell::Cell<Option<std::thread::Thread>>,
    signaled: std::sync::atomic::AtomicBool,
    next: *mut Waiter,
}
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:138-142`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L138-L142)*

Representation of a node in the linked list of waiters in the RUNNING state.
A waiters is stored on the stack of the waiting threads.

### `Guard<'a>`

```rust
struct Guard<'a> {
    queue: &'a std::sync::atomic::AtomicPtr<Waiter>,
    new_queue: *mut Waiter,
}
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:145-148`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L145-L148)*

Drains and notifies the queue of waiters on drop.

#### Trait Implementations

##### `impl Drop for Guard<'_>`

- <span id="guard-drop"></span>`fn drop(&mut self)`

## Functions

### `initialize_or_wait`

```rust
fn initialize_or_wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, init: Option<&mut dyn FnMut() -> bool>)
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:177-208`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L177-L208)*

### `wait`

```rust
fn wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, curr_queue: *mut Waiter)
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:210-239`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L210-L239)*

## Constants

### `INCOMPLETE`
```rust
const INCOMPLETE: usize = 0usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:125`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L125)*

### `RUNNING`
```rust
const RUNNING: usize = 1usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:126`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L126)*

### `COMPLETE`
```rust
const COMPLETE: usize = 2usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:127`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L127)*

### `INCOMPLETE_PTR`
```rust
const INCOMPLETE_PTR: *mut Waiter = {0x0 as *mut imp::Waiter};
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:128`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L128)*

### `COMPLETE_PTR`
```rust
const COMPLETE_PTR: *mut Waiter = {0x2 as *mut imp::Waiter};
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:129`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L129)*

### `STATE_MASK`
```rust
const STATE_MASK: usize = 3usize;
```

*Defined in [`once_cell-1.21.3/src/imp_std.rs:133`](../../../.source_1765210505/once_cell-1.21.3/src/imp_std.rs#L133)*

