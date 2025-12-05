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

- `const fn new() -> OnceCell<T>` — [`OnceCell`](../../sync/index.md)

- `const fn with_value(value: T) -> OnceCell<T>` — [`OnceCell`](../../sync/index.md)

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

##### `impl Clone<T: Clone>`

- `fn clone(self: &Self) -> OnceCell<T>` — [`OnceCell`](../../sync/index.md)

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> OnceCell<T>` — [`OnceCell`](../../sync/index.md)

##### `impl Eq<T: Eq>`

##### `impl PartialEq<T: PartialEq>`

- `fn eq(self: &Self, other: &OnceCell<T>) -> bool` — [`OnceCell`](../../sync/index.md)

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

- `const fn new(f: F) -> Lazy<T, F>` — [`Lazy`](../../sync/index.md)

- `fn into_value(this: Lazy<T, F>) -> Result<T, F>` — [`Lazy`](../../sync/index.md)

#### Trait Implementations

##### `impl Debug<T: fmt::Debug, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> Lazy<T>` — [`Lazy`](../../sync/index.md)

##### `impl Deref<T, F: FnOnce() -> T>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl DerefMut<T, F: FnOnce() -> T>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl RefUnwindSafe<T, F: RefUnwindSafe>`

##### `impl Sync<T, F: Send>`

