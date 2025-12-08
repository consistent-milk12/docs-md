*[clap_builder](../../index.md) / [util](../index.md) / [color](index.md)*

---

# Module `color`

## Enums

### `ColorChoice`

```rust
enum ColorChoice {
    Auto,
    Always,
    Never,
}
```

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

- `fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md)

#### Trait Implementations

##### `impl Clone for ColorChoice`

- `fn clone(self: &Self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ColorChoice`

- `fn default() -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Display for ColorChoice`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl FromStr for ColorChoice`

- `type Err = String`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq for ColorChoice`

- `fn eq(self: &Self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl<T> ToString for ColorChoice`

- `fn to_string(self: &Self) -> String`

##### `impl ValueEnum for ColorChoice`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value(self: &Self) -> Option<PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md)

