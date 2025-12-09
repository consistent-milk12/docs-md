*[clap_builder](../../index.md) / [builder](../index.md) / [styling](index.md)*

---

# Module `styling`

Terminal [`Styles`](#styles) for help and error output

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Styles`](#styles) | struct | Terminal styling definitions |

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

*Defined in [`clap_builder-4.5.53/src/builder/styling.rs:23-33`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/styling.rs#L23-L33)*

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

- <span id="styles-plain"></span>`const fn plain() -> Self`

- <span id="styles-styled"></span>`const fn styled() -> Self`

- <span id="styles-header"></span>`const fn header(self, style: Style) -> Self`

- <span id="styles-error"></span>`const fn error(self, style: Style) -> Self`

- <span id="styles-usage"></span>`const fn usage(self, style: Style) -> Self`

- <span id="styles-literal"></span>`const fn literal(self, style: Style) -> Self`

- <span id="styles-placeholder"></span>`const fn placeholder(self, style: Style) -> Self`

- <span id="styles-valid"></span>`const fn valid(self, style: Style) -> Self`

- <span id="styles-invalid"></span>`const fn invalid(self, style: Style) -> Self`

- <span id="styles-context"></span>`const fn context(self, style: Style) -> Self`

- <span id="styles-context-value"></span>`const fn context_value(self, style: Style) -> Self`

#### Trait Implementations

##### `impl AppExt for Styles`

##### `impl Clone for Styles`

- <span id="styles-clone"></span>`fn clone(&self) -> Styles` — [`Styles`](#styles)

##### `impl Debug for Styles`

- <span id="styles-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Styles`

- <span id="styles-default"></span>`fn default() -> Self`

