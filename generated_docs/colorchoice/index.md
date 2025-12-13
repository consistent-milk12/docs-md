# Crate `colorchoice`

Global override of color control

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AtomicChoice`](#atomicchoice) | struct |  |
| [`ColorChoice`](#colorchoice) | enum | Selection for overriding color output |

## Structs

### `AtomicChoice`

```rust
struct AtomicChoice(core::sync::atomic::AtomicUsize);
```

*Defined in [`colorchoice-1.0.4/src/lib.rs:49`](../../.source_1765633015/colorchoice-1.0.4/src/lib.rs#L49)*

#### Implementations

- <span id="atomicchoice-new"></span>`const fn new() -> Self`

- <span id="atomicchoice-get"></span>`fn get(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-set"></span>`fn set(&self, choice: ColorChoice)` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-from-choice"></span>`const fn from_choice(choice: ColorChoice) -> usize` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-to-choice"></span>`const fn to_choice(choice: usize) -> Option<ColorChoice>` — [`ColorChoice`](#colorchoice)

#### Trait Implementations

##### `impl Any for AtomicChoice`

- <span id="atomicchoice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicChoice`

- <span id="atomicchoice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicChoice`

- <span id="atomicchoice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for AtomicChoice`

- <span id="atomicchoice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicChoice`

- <span id="atomicchoice-default"></span>`fn default() -> Self`

##### `impl<T> From for AtomicChoice`

- <span id="atomicchoice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicChoice`

- <span id="atomicchoice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for AtomicChoice`

- <span id="atomicchoice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicchoice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicChoice`

- <span id="atomicchoice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicchoice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`colorchoice-1.0.4/src/lib.rs:14-26`](../../.source_1765633015/colorchoice-1.0.4/src/lib.rs#L14-L26)*

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

- <span id="colorchoice-global"></span>`fn global() -> Self`

  Get the current [`ColorChoice`](#colorchoice) state

- <span id="colorchoice-write-global"></span>`fn write_global(self)`

  Override the detected [`ColorChoice`](#colorchoice)

#### Trait Implementations

##### `impl Any for ColorChoice`

- <span id="colorchoice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ColorChoice`

- <span id="colorchoice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ColorChoice`

- <span id="colorchoice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ColorChoice`

- <span id="colorchoice-clone"></span>`fn clone(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl CloneToUninit for ColorChoice`

- <span id="colorchoice-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- <span id="colorchoice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ColorChoice`

- <span id="colorchoice-default"></span>`fn default() -> Self`

##### `impl Eq for ColorChoice`

##### `impl<T> From for ColorChoice`

- <span id="colorchoice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ColorChoice`

- <span id="colorchoice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-partialeq-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl<U> TryFrom for ColorChoice`

- <span id="colorchoice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colorchoice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColorChoice`

- <span id="colorchoice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colorchoice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

