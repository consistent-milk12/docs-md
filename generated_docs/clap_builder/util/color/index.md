*[clap_builder](../../index.md) / [util](../index.md) / [color](index.md)*

---

# Module `color`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ColorChoice`](#colorchoice) | enum | Represents the color preferences for program output |

## Enums

### `ColorChoice`

```rust
enum ColorChoice {
    Auto,
    Always,
    Never,
}
```

*Defined in [`clap_builder-4.5.53/src/util/color.rs:6-58`](../../../../.source_1765633015/clap_builder-4.5.53/src/util/color.rs#L6-L58)*

Represents the color preferences for program output

#### Variants

- **`Auto`**

  Enables colored output only when the output is going to a terminal or TTY.
  
  <div class="warning">
  
  **NOTE:** This is the default behavior of `clap`.
  
  </div>
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Auto)
      .get_matches();
  }
  ```

- **`Always`**

  Enables colored output regardless of whether or not the output is going to a terminal/TTY.
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Always)
      .get_matches();
  }
  ```

- **`Never`**

  Disables colored output no matter if the output is going to a terminal/TTY, or not.
  
  # Examples
  
  ```rust
  #[cfg(feature = "color")] {
  use clap_builder as clap;
  use clap::{Command, ColorChoice};
  Command::new("myprog")
      .color(ColorChoice::Never)
      .get_matches();
  }
  ```

#### Implementations

- <span id="colorchoice-possible-values"></span>`fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md#possiblevalue)

  Report all `possible_values`

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

- <span id="colorchoice-default"></span>`fn default() -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Display for ColorChoice`

- <span id="colorchoice-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl<T> From for ColorChoice`

- <span id="colorchoice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for ColorChoice`

- <span id="colorchoice-fromstr-type-err"></span>`type Err = String`

- <span id="colorchoice-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<U> Into for ColorChoice`

- <span id="colorchoice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-partialeq-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl ToOwned for ColorChoice`

- <span id="colorchoice-toowned-type-owned"></span>`type Owned = T`

- <span id="colorchoice-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colorchoice-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ColorChoice`

- <span id="colorchoice-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ColorChoice`

- <span id="colorchoice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colorchoice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ColorChoice`

- <span id="colorchoice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colorchoice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum for ColorChoice`

- <span id="colorchoice-valueenum-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="colorchoice-valueenum-to-possible-value"></span>`fn to_possible_value(&self) -> Option<PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md#possiblevalue)

