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

*Defined in [`clap_builder-4.5.53/src/util/color.rs:6-58`](../../../../.source_1765210505/clap_builder-4.5.53/src/util/color.rs#L6-L58)*

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

- <span id="colorchoice-possible-values"></span>`fn possible_values() -> impl Iterator<Item = PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md)

#### Trait Implementations

##### `impl Clone for ColorChoice`

- <span id="colorchoice-clone"></span>`fn clone(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- <span id="colorchoice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ColorChoice`

- <span id="colorchoice-default"></span>`fn default() -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Display for ColorChoice`

- <span id="colorchoice-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for ColorChoice`

##### `impl FromStr for ColorChoice`

- <span id="colorchoice-type-err"></span>`type Err = String`

- <span id="colorchoice-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

##### `impl ToString for ColorChoice`

- <span id="colorchoice-to-string"></span>`fn to_string(&self) -> String`

##### `impl ValueEnum for ColorChoice`

- <span id="colorchoice-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="colorchoice-to-possible-value"></span>`fn to_possible_value(&self) -> Option<PossibleValue>` — [`PossibleValue`](../../builder/possible_value/index.md)

