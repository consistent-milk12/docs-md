# Crate `colorchoice`

Global override of color control

## Enums

### `ColorChoice`

```rust
enum ColorChoice {
    Auto,
    AlwaysAnsi,
    Always,
    Never,
}
```

Selection for overriding color output

#### Variants

- **`Auto`**

  Use colors if the output device appears to support them

- **`AlwaysAnsi`**

  Like `Always`, except it never tries to use anything other than emitting ANSI
  color codes.

- **`Always`**

  Try very hard to emit colors.
  
  This includes emitting ANSI colors on Windows if the console API is unavailable.

- **`Never`**

  Never emit colors.

#### Implementations

- `fn global() -> Self`
  Get the current [`ColorChoice`] state

- `fn write_global(self: Self)`
  Override the detected [`ColorChoice`]

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

- `fn clone(self: &Self) -> ColorChoice`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ColorChoice) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

