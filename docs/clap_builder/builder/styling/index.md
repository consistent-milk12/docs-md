*[clap_builder](../../index.md) / [builder](../index.md) / [styling](index.md)*

---

# Module `styling`

Terminal [`Styles`](../index.md) for help and error output

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

- `const fn get_header(self: &Self) -> &Style`

- `const fn get_error(self: &Self) -> &Style`

- `const fn get_usage(self: &Self) -> &Style`

- `const fn get_literal(self: &Self) -> &Style`

- `const fn get_placeholder(self: &Self) -> &Style`

- `const fn get_valid(self: &Self) -> &Style`

- `const fn get_invalid(self: &Self) -> &Style`

- `const fn get_context(self: &Self) -> &Style`

- `const fn get_context_value(self: &Self) -> &Style`

#### Trait Implementations

##### `impl AppExt for Styles`

##### `impl Clone for Styles`

- `fn clone(self: &Self) -> Styles` — [`Styles`](../index.md)

##### `impl Debug for Styles`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Styles`

- `fn default() -> Self`

