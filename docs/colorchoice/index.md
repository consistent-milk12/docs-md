# Crate `colorchoice`

Global override of color control

## Structs

### `AtomicChoice`

```rust
struct AtomicChoice(core::sync::atomic::AtomicUsize);
```

#### Implementations

- `const fn new() -> Self`

- `fn get(self: &Self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

- `fn set(self: &Self, choice: ColorChoice)` — [`ColorChoice`](#colorchoice)

- `const fn from_choice(choice: ColorChoice) -> usize` — [`ColorChoice`](#colorchoice)

- `const fn to_choice(choice: usize) -> Option<ColorChoice>` — [`ColorChoice`](#colorchoice)

#### Trait Implementations

##### `impl Debug for AtomicChoice`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AtomicChoice`

- `fn default() -> Self`

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

- `fn write_global(self: Self)`

#### Trait Implementations

##### `impl Clone for ColorChoice`

- `fn clone(self: &Self) -> ColorChoice` — [`ColorChoice`](#colorchoice)

##### `impl Copy for ColorChoice`

##### `impl Debug for ColorChoice`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ColorChoice`

- `fn default() -> Self`

##### `impl Eq for ColorChoice`

##### `impl PartialEq for ColorChoice`

- `fn eq(self: &Self, other: &ColorChoice) -> bool` — [`ColorChoice`](#colorchoice)

##### `impl StructuralPartialEq for ColorChoice`

