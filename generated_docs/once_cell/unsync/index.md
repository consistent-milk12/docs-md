*[once_cell](../index.md) / [unsync](index.md)*

---

# Module `unsync`

Single-threaded version of `OnceCell`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceCell`](#oncecell) | struct | A cell which can be written to only once. |
| [`Lazy`](#lazy) | struct | A value which is initialized on the first access. |

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    inner: core::cell::UnsafeCell<Option<T>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:411-414`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L411-L414)*

A cell which can be written to only once. It is not thread safe.

Unlike `std::cell::RefCell`, a `OnceCell` provides simple `&`
references to the contents.

# Example
```rust
use once_cell::unsync::OnceCell;

let cell = OnceCell::new();
assert!(cell.get().is_none());

let value: &String = cell.get_or_init(|| {
    "Hello, World!".to_string()
});
assert_eq!(value, "Hello, World!");
assert!(cell.get().is_some());
```

#### Implementations

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-get"></span>`fn get(&self) -> Option<&T>`

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

- <span id="oncecell-set"></span>`fn set(&self, value: T) -> Result<(), T>`

- <span id="oncecell-try-insert"></span>`fn try_insert(&self, value: T) -> Result<&T, (&T, T)>`

- <span id="oncecell-get-or-init"></span>`fn get_or_init<F>(&self, f: F) -> &T`

- <span id="oncecell-get-or-try-init"></span>`fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>`

- <span id="oncecell-take"></span>`fn take(&mut self) -> Option<T>`

- <span id="oncecell-into-inner"></span>`fn into_inner(self) -> Option<T>`

#### Trait Implementations

##### `impl<T: Clone> Clone for OnceCell<T>`

- <span id="oncecell-clone"></span>`fn clone(&self) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T: fmt::Debug> Debug for OnceCell<T>`

- <span id="oncecell-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for OnceCell<T>`

- <span id="oncecell-default"></span>`fn default() -> Self`

##### `impl<T: Eq> Eq for OnceCell<T>`

##### `impl<T: PartialEq> PartialEq for OnceCell<T>`

- <span id="oncecell-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T>`

##### `impl<T: UnwindSafe> UnwindSafe for OnceCell<T>`

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:714-717`](../../../.source_1765521767/once_cell-1.21.3/src/lib.rs#L714-L717)*

A value which is initialized on the first access.

# Example
```rust
use once_cell::unsync::Lazy;

let lazy: Lazy<i32> = Lazy::new(|| {
    println!("initializing");
    92
});
println!("ready");
println!("{}", *lazy);
println!("{}", *lazy);

// Prints:
//   ready
//   initializing
//   92
//   92
```

#### Implementations

- <span id="lazy-new"></span>`const fn new(init: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

- <span id="lazy-into-value"></span>`fn into_value(this: Lazy<T, F>) -> Result<T, F>` — [`Lazy`](#lazy)

#### Trait Implementations

##### `impl<T: fmt::Debug, F> Debug for Lazy<T, F>`

- <span id="lazy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for Lazy<T>`

- <span id="lazy-default"></span>`fn default() -> Lazy<T>` — [`Lazy`](#lazy)

##### `impl<T, F: FnOnce() -> T> Deref for Lazy<T, F>`

- <span id="lazy-deref-type-target"></span>`type Target = T`

- <span id="lazy-deref"></span>`fn deref(&self) -> &T`

##### `impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F>`

- <span id="lazy-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T> Receiver for Lazy<T, F>`

- <span id="lazy-receiver-type-target"></span>`type Target = T`

##### `impl<T, F: RefUnwindSafe> RefUnwindSafe for Lazy<T, F>`

