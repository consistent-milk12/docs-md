*[once_cell](../index.md) / [imp](index.md)*

---

# Module `imp`

## Modules

- [`strict`](strict/index.md) - 

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    queue: std::sync::atomic::AtomicPtr<Waiter>,
    value: std::cell::UnsafeCell<Option<T>>,
}
```

#### Implementations

- `const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `fn is_initialized(self: &Self) -> bool`

- `fn initialize<F, E>(self: &Self, f: F) -> Result<(), E>`

- `fn wait(self: &Self)`

- `unsafe fn get_unchecked(self: &Self) -> &T`

- `fn get_mut(self: &mut Self) -> Option<&mut T>`

- `fn into_inner(self: Self) -> Option<T>`

#### Trait Implementations

##### `impl<T: $crate::fmt::Debug> Debug for OnceCell<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

Representation of a node in the linked list of waiters in the RUNNING state.
A waiters is stored on the stack of the waiting threads.

### `Guard<'a>`

```rust
struct Guard<'a> {
    queue: &'a std::sync::atomic::AtomicPtr<Waiter>,
    new_queue: *mut Waiter,
}
```

Drains and notifies the queue of waiters on drop.

#### Trait Implementations

##### `impl Drop for Guard<'_>`

- `fn drop(self: &mut Self)`

## Functions

### `initialize_or_wait`

```rust
fn initialize_or_wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, init: Option<&mut dyn FnMut() -> bool>)
```

### `wait`

```rust
fn wait(queue: &std::sync::atomic::AtomicPtr<Waiter>, curr_queue: *mut Waiter)
```

## Constants

### `INCOMPLETE`

```rust
const INCOMPLETE: usize = 0usize;
```

### `RUNNING`

```rust
const RUNNING: usize = 1usize;
```

### `COMPLETE`

```rust
const COMPLETE: usize = 2usize;
```

### `INCOMPLETE_PTR`

```rust
const INCOMPLETE_PTR: *mut Waiter = {0x0 as *mut imp::Waiter};
```

### `COMPLETE_PTR`

```rust
const COMPLETE_PTR: *mut Waiter = {0x2 as *mut imp::Waiter};
```

### `STATE_MASK`

```rust
const STATE_MASK: usize = 3usize;
```

