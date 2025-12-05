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

- `fn write_global(self: Self)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ColorChoice` — [`ColorChoice`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ColorChoice) -> bool` — [`ColorChoice`](../index.md)

##### `impl StructuralPartialEq`

