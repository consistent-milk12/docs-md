*[once_cell](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread-safe, blocking version of `OnceCell`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OnceCell`](#oncecell) | struct | A thread-safe cell which can be written to only once. |
| [`Lazy`](#lazy) | struct | A value which is initialized on the first access. |
| [`_dummy`](#dummy) | fn | ```compile_fail struct S(*mut ()); unsafe impl Sync for S {} |

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T>(super::imp::OnceCell<T>);
```

*Defined in [`once_cell-1.21.3/src/lib.rs:901`](../../../.source_1765210505/once_cell-1.21.3/src/lib.rs#L901)*

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

- <span id="oncecell-new"></span>`const fn new() -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-with-value"></span>`const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](#oncecell)

- <span id="oncecell-get"></span>`fn get(&self) -> Option<&T>`

- <span id="oncecell-wait"></span>`fn wait(&self) -> &T`

- <span id="oncecell-get-mut"></span>`fn get_mut(&mut self) -> Option<&mut T>`

- <span id="oncecell-get-unchecked"></span>`unsafe fn get_unchecked(&self) -> &T`

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

- <span id="oncecell-default"></span>`fn default() -> OnceCell<T>` — [`OnceCell`](#oncecell)

##### `impl<T: Eq> Eq for OnceCell<T>`

##### `impl<T: PartialEq> PartialEq for OnceCell<T>`

- <span id="oncecell-eq"></span>`fn eq(&self, other: &OnceCell<T>) -> bool` — [`OnceCell`](#oncecell)

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    cell: OnceCell<T>,
    init: core::cell::Cell<Option<F>>,
}
```

*Defined in [`once_cell-1.21.3/src/lib.rs:1255-1258`](../../../.source_1765210505/once_cell-1.21.3/src/lib.rs#L1255-L1258)*

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

- <span id="lazy-new"></span>`const fn new(f: F) -> Lazy<T, F>` — [`Lazy`](#lazy)

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

##### `impl<T, F: Send> Sync for Lazy<T, F>`

## Functions

### `_dummy`

```rust
fn _dummy()
```

*Defined in [`once_cell-1.21.3/src/lib.rs:1408`](../../../.source_1765210505/once_cell-1.21.3/src/lib.rs#L1408)*

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

