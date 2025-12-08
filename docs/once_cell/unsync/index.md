*[once_cell](../index.md) / [unsync](index.md)*

---

# Module `unsync`

Single-threaded version of `OnceCell`.

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    inner: core::cell::UnsafeCell<Option<T>>,
}
```

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

- `const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `fn get(self: &Self) -> Option<&T>`

- `fn get_mut(self: &mut Self) -> Option<&mut T>`

- `fn set(self: &Self, value: T) -> Result<(), T>`

- `fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)>`

- `fn get_or_init<F>(self: &Self, f: F) -> &T`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>`

- `fn take(self: &mut Self) -> Option<T>`

- `fn into_inner(self: Self) -> Option<T>`

#### Trait Implementations

##### `impl<T: Clone> Clone for OnceCell<T>`

- `fn clone(self: &Self) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<T: fmt::Debug> Debug for OnceCell<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for OnceCell<T>`

- `fn default() -> Self`

##### `impl<T: Eq> Eq for OnceCell<T>`

##### `impl<T: PartialEq> PartialEq for OnceCell<T>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<T: RefUnwindSafe + UnwindSafe> RefUnwindSafe for OnceCell<T>`

##### `impl<T: UnwindSafe> UnwindSafe for OnceCell<T>`

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

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

- `const fn new(init: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

- `fn into_value(this: Lazy<T, F>) -> Result<T, F>` — [`Lazy`](#lazy)

#### Trait Implementations

##### `impl<T: fmt::Debug, F> Debug for Lazy<T, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default> Default for Lazy<T>`

- `fn default() -> Lazy<T>` — [`Lazy`](#lazy)

##### `impl<T, F: FnOnce() -> T> Deref for Lazy<T, F>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl<P, T> Receiver for Lazy<T, F>`

- `type Target = T`

##### `impl<T, F: RefUnwindSafe> RefUnwindSafe for Lazy<T, F>`

