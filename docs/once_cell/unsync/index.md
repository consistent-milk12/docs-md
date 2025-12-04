*[once_cell](../index.md) / [unsync](index.md)*

---

# Module `unsync`

Single-threaded version of `OnceCell`.

## Structs

### `OnceCell<T>`

```rust
struct OnceCell<T> {
    // [REDACTED: Private Fields]
}
```

A cell which can be written to only once. It is not thread safe.

Unlike `std::cell::RefCell`, a `OnceCell` provides simple `&`
references to the contents.

# Example
```
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

- `const fn new() -> OnceCell<T>`
  Creates a new empty cell.

- `const fn with_value(value: T) -> OnceCell<T>`
  Creates a new initialized cell.

- `fn get(self: &Self) -> Option<&T>`
  Gets a reference to the underlying value.

- `fn get_mut(self: &mut Self) -> Option<&mut T>`
  Gets a mutable reference to the underlying value.

- `fn set(self: &Self, value: T) -> Result<(), T>`
  Sets the contents of this cell to `value`.

- `fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)>`
  Like [`set`](Self::set), but also returns a reference to the final cell value.

- `fn get_or_init<F>(self: &Self, f: F) -> &T`
  Gets the contents of the cell, initializing it with `f`

- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>`
  Gets the contents of the cell, initializing it with `f` if

- `fn take(self: &mut Self) -> Option<T>`
  Takes the value out of this `OnceCell`, moving it back to an uninitialized state.

- `fn into_inner(self: Self) -> Option<T>`
  Consumes the `OnceCell`, returning the wrapped value.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<T>`

- `fn from(t: never) -> T`

##### `impl From<T>`

- `fn from(value: T) -> Self`

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

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl RefUnwindSafe<T: RefUnwindSafe + UnwindSafe>`

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

##### `impl UnwindSafe<T: UnwindSafe>`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default<T>`

- `fn default() -> Self`

### `Lazy<T, F>`

```rust
struct Lazy<T, F> {
    // [REDACTED: Private Fields]
}
```

A value which is initialized on the first access.

# Example
```
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

- `const fn new(init: F) -> Lazy<T, F>`
  Creates a new lazy value with the given initializing function.

- `fn into_value(this: Lazy<T, F>) -> Result<T, F>`
  Consumes this `Lazy` returning the stored value.

- `fn force(this: &Lazy<T, F>) -> &T`
  Forces the evaluation of this lazy value and returns a reference to

- `fn force_mut(this: &mut Lazy<T, F>) -> &mut T`
  Forces the evaluation of this lazy value and returns a mutable reference to

- `fn get(this: &Lazy<T, F>) -> Option<&T>`
  Gets the reference to the result of this lazy value if

- `fn get_mut(this: &mut Lazy<T, F>) -> Option<&mut T>`
  Gets the mutable reference to the result of this lazy value if

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

