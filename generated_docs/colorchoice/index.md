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

*Defined in [`colorchoice-1.0.4/src/lib.rs:49`](../../.source_1765210505/colorchoice-1.0.4/src/lib.rs#L49)*

#### Implementations

- <span id="atomicchoice-new"></span>`const fn new() -> Self`

- <span id="atomicchoice-get"></span>`fn get(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-set"></span>`fn set(&self, choice: ColorChoice)` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-from-choice"></span>`const fn from_choice(choice: ColorChoice) -> usize` — [`ColorChoice`](#colorchoice)

- <span id="atomicchoice-to-choice"></span>`const fn to_choice(choice: usize) -> Option<ColorChoice>` — [`ColorChoice`](#colorchoice)

#### Trait Implementations

##### `impl Debug for AtomicChoice`

- <span id="atomicchoice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AtomicChoice`

- <span id="atomicchoice-default"></span>`fn default() -> Self`

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

*Defined in [`colorchoice-1.0.4/src/lib.rs:14-26`](../../.source_1765210505/colorchoice-1.0.4/src/lib.rs#L14-L26)*

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

- <span id="colorchoice-write-global"></span>`fn write_global(self)`

#### Trait Implementations

##### `impl Clone for ColorChoice`

- <span id="colorchoice-clone"></span>`fn clone(&self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- <span id="colorchoice-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ColorChoice`

- <span id="colorchoice-default"></span>`fn default() -> Self`

##### `impl Eq for ColorChoice`

##### `impl PartialEq for ColorChoice`

- <span id="colorchoice-eq"></span>`fn eq(&self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

