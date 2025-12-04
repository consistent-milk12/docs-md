*[once_cell](../index.md) / [sync](index.md)*

---

# Module `sync`

Thread-safe, blocking version of `OnceCell`.

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T>();
```

A thread-safe cell which can be written to only once.

`OnceCell` provides `&` references to the contents without RAII guards.

Reading a non-`None` value out of `OnceCell` establishes a
happens-before relationship with a corresponding write. For example, if
thread A initializes the cell with `get_or_init(f)`, and thread B
subsequently reads the result of this call, B also observes all the side
effects of `f`.

# Example
```
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

- `const fn new() -> OnceCell<T>`
  Creates a new empty cell.

- `const fn with_value(value: T) -> OnceCell<T>`
  Creates a new initialized cell.

- `fn get(self: &Self) -> Option<&T>`
  Gets the reference to the underlying value.

- `fn wait(self: &Self) -> &T`
  Gets the reference to the underlying value, blocking the current

- `fn get_mut(self: &mut Self) -> Option<&mut T>`
  Gets the mutable reference to the underlying value.

- `unsafe fn get_unchecked(self: &Self) -> &T`
  Get the reference to the underlying value, without checking if the

- `fn set(self: &Self, value: T) -> Result<(), T>`
  Sets the contents of this cell to `value`.

- `fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)>`
  Like [`set`](Self::set), but also returns a reference to the final cell value.

- `fn get_or_init<F>(self: &Self, f: F) -> &T`
  Gets the contents of the cell, initializing it with `f` if the cell

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>`
  Gets the contents of the cell, initializing it with `f` if

- `fn take(self: &mut Self) -> Option<T>`
  Takes the value out of this `OnceCell`, moving it back to an uninitialized state.

- `fn into_inner(self: Self) -> Option<T>`
  Consumes the `OnceCell`, returning the wrapped value. Returns

#### Trait Implementations

##### `impl From<T>`

- `fn from(value: T) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<T>`

- `fn from(t: never) -> T`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: Clone>`

- `fn clone(self: &Self) -> OnceCell<T>`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq<T: Eq>`

##### `impl PartialEq<T: PartialEq>`

- `fn eq(self: &Self, other: &OnceCell<T>) -> bool`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> OnceCell<T>`

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
}
```

A value which is initialized on the first access.

This type is thread-safe and can be used in statics.

# Example

```
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

- `const fn new(f: F) -> Lazy<T, F>`
  Creates a new lazy value with the given initializing

- `fn into_value(this: Lazy<T, F>) -> Result<T, F>`
  Consumes this `Lazy` returning the stored value.

- `fn force(this: &Lazy<T, F>) -> &T`
  Forces the evaluation of this lazy value and

- `fn force_mut(this: &mut Lazy<T, F>) -> &mut T`
  Forces the evaluation of this lazy value and

- `fn get(this: &Lazy<T, F>) -> Option<&T>`
  Gets the reference to the result of this lazy value if

- `fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T>`
  Gets the reference to the result of this lazy value if

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl RefUnwindSafe<T, F: RefUnwindSafe>`

##### `impl Sync<T, F: Send>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: fmt::Debug, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T: Default>`

- `fn default() -> Lazy<T>`
  Creates a new lazy value using `Default` as the initializing function.

##### `impl Deref<T, F: FnOnce() -> T>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl DerefMut<T, F: FnOnce() -> T>`

- `fn deref_mut(self: &mut Self) -> &mut T`

