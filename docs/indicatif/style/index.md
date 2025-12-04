*[indicatif](../index.md) / [style](index.md)*

---

# Module `style`

## Structs

### `ProgressStyle`

```rust
struct ProgressStyle {
}
```

#### Implementations

- `fn default_bar() -> Self`
  Returns the default progress bar style for bars

- `fn default_spinner() -> Self`
  Returns the default progress bar style for spinners

- `fn with_template(template: &str) -> Result<Self, TemplateError>`
  Sets the template string for the progress bar

- `fn tick_chars(self: Self, s: &str) -> Self`
  Sets the tick character sequence for spinners

- `fn tick_strings(self: Self, s: &[&str]) -> Self`
  Sets the tick string sequence for spinners

- `fn progress_chars(self: Self, s: &str) -> Self`
  Sets the progress characters `(filled, current, to do)`

- `fn with_key<S: ProgressTracker + 'static>(self: Self, key: &'static str, f: S) -> Self`
  Adds a custom key that owns a [`ProgressTracker`] to the template

- `fn template(self: Self, s: &str) -> Result<Self, TemplateError>`
  Sets the template string for the progress bar

- `fn get_tick_str(self: &Self, idx: u64) -> &str`
  Returns the tick string for a given number

- `fn get_final_tick_str(self: &Self) -> &str`
  Returns the tick string for the finished state

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

##### `impl Clone`

- `fn clone(self: &Self) -> ProgressStyle`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

### `TemplateError`

```rust
struct TemplateError {
}
```

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `ProgressTracker`

```rust
trait ProgressTracker: Send + Sync { ... }
```

Trait for defining stateful or stateless formatters

#### Required Methods

- `fn clone_box(self: &Self) -> Box<dyn ProgressTracker>`

  Creates a new instance of the progress tracker

- `fn tick(self: &mut Self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a tick event

- `fn reset(self: &mut Self, state: &ProgressState, now: Instant)`

  Notifies the progress tracker of a reset event

- `fn write(self: &Self, state: &ProgressState, w: &mut dyn fmt::Write)`

  Provides access to the progress bar display buffer for custom messages

## Functions

### `write_ansi_range`

```rust
fn write_ansi_range(formatter: &mut std::fmt::Formatter<'_>, text: &str, start: usize, end: usize) -> fmt::Result
```

Write the visible text between start and end. The ansi escape
sequences are written unchanged.

