*[clap_builder](../../index.md) / [builder](../index.md) / [styling](index.md)*

---

# Module `styling`

Terminal [`Styles`](#styles) for help and error output

## Structs

### `Styles`

```rust
struct Styles {
    header: Style,
    error: Style,
    usage: Style,
    literal: Style,
    placeholder: Style,
    valid: Style,
    invalid: Style,
    context: Style,
    context_value: Option<Style>,
}
```

Terminal styling definitions

See also `Command::styles`.

# Example

clap v3 styling
```rust
use clap_builder as clap;
use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

#### Implementations

- `const fn plain() -> Self`

- `const fn styled() -> Self`

- `const fn header(self: Self, style: Style) -> Self`

- `const fn error(self: Self, style: Style) -> Self`

- `const fn usage(self: Self, style: Style) -> Self`

- `const fn literal(self: Self, style: Style) -> Self`

- `const fn placeholder(self: Self, style: Style) -> Self`

- `const fn valid(self: Self, style: Style) -> Self`

- `const fn invalid(self: Self, style: Style) -> Self`

- `const fn context(self: Self, style: Style) -> Self`

- `const fn context_value(self: Self, style: Style) -> Self`

#### Trait Implementations

##### `impl AppExt`

##### `impl Clone`

- `fn clone(self: &Self) -> Styles` — [`Styles`](../../../builder/styling/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

