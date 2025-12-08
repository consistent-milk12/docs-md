*[once_cell](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread-safe, blocking version of `OnceCell`.

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T>(super::imp::OnceCell<T>);
```

A thread-safe cell which can be written to only once.

`OnceCell` provides `&` references to the contents without RAII guards.

Reading a non-`None` value out of `OnceCell` establishes a
happens-before relationship with a corresponding write. For example, if
thread A initializes the cell with `get_or_init(f)`, and thread B
subsequently reads the result of this call, B also observes all the side
effects of `f`.

# Example
```rust
use once_cell::sync::OnceCell;

static CELL: OnceCell<String> = OnceCell::new();
assert!(CELL.get().is_none());

std::thread::spawn(|| {
    let value: &String = CELL.get_or_init(|| {
        "Hello, World!".to_string()
    });
    assert_eq!(value, "Hello, World!");
}).join().unwrap();

let value: Option<&String> = CELL.get();
assert!(value.is_some());
assert_eq!(value.unwrap().as_str(), "Hello, World!");
```

#### Implementations

- `const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- `fn get(self: &Self) -> Option<&T>`

- `fn wait(self: &Self) -> &T`

- `fn get_mut(self: &mut Self) -> Option<&mut T>`

- `unsafe fn get_unchecked(self: &Self) -> &T`

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

- `fn default() -> OnceCell<T>` — [`OnceCell`](#oncecell)

##### `impl<T: Eq> Eq for OnceCell<T>`

##### `impl<T: PartialEq> PartialEq for OnceCell<T>`

- `fn eq(self: &Self, other: &OnceCell<T>) -> bool` — [`OnceCell`](#oncecell)

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

A value which is initialized on the first access.

This type is thread-safe and can be used in statics.

# Example

```rust
use std::collections::HashMap;

use once_cell::sync::Lazy;

static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

fn main() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));

    // Prints:
    //   ready
    //   initializing
    //   Some("Spica")
    //   Some("Hoyten")
}
```

#### Implementations

- `const fn new(f: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

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

##### `impl<T, F: Send> Sync for Lazy<T, F>`

## Functions

### `_dummy`

```rust
fn _dummy()
```

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::sync::OnceCell::<S>::new());
```

```compile_fail
struct S(*mut ());
unsafe impl Sync for S {}

fn share<T: Sync>(_: &T) {}
share(&once_cell::sync::Lazy::<S>::new(|| unimplemented!()));
```

