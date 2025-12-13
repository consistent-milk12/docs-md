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

*Defined in [`clap_builder-4.5.53/src/builder/styling.rs:23-33`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/styling.rs#L23-L33)*

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

  No terminal styling

- <span id="styles-styled"></span>`const fn styled() -> Self`

  Default terminal styling

- <span id="styles-header"></span>`const fn header(self, style: Style) -> Self`

  General Heading style, e.g. `help_heading`

- <span id="styles-error"></span>`const fn error(self, style: Style) -> Self`

  Error heading

- <span id="styles-usage"></span>`const fn usage(self, style: Style) -> Self`

  Usage heading

- <span id="styles-literal"></span>`const fn literal(self, style: Style) -> Self`

  Literal command-line syntax, e.g. `--help`

- <span id="styles-placeholder"></span>`const fn placeholder(self, style: Style) -> Self`

  Descriptions within command-line syntax, e.g. `value_name`

- <span id="styles-valid"></span>`const fn valid(self, style: Style) -> Self`

  Highlight suggested usage

- <span id="styles-invalid"></span>`const fn invalid(self, style: Style) -> Self`

  Highlight invalid usage

- <span id="styles-context"></span>`const fn context(self, style: Style) -> Self`

  Highlight all specified contexts, e.g. `[default: false]`

  

  To specialize the style of the value within the context, see `Styles::context_value`

- <span id="styles-context-value"></span>`const fn context_value(self, style: Style) -> Self`

  Highlight values within all of the context, e.g. the `false` in `[default: false]`

  

  If not explicitly set, falls back to `context`'s style.

#### Trait Implementations

##### `impl Any for Styles`

- <span id="styles-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl AppExt for Styles`

##### `impl<T> Borrow for Styles`

- <span id="styles-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Styles`

- <span id="styles-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Styles`

- <span id="styles-clone"></span>`fn clone(&self) -> Styles` — [`Styles`](#styles)

##### `impl CloneToUninit for Styles`

- <span id="styles-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Styles`

- <span id="styles-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Styles`

- <span id="styles-default"></span>`fn default() -> Self`

##### `impl<T> From for Styles`

- <span id="styles-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Styles`

- <span id="styles-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Styles`

- <span id="styles-toowned-type-owned"></span>`type Owned = T`

- <span id="styles-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styles-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Styles`

- <span id="styles-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styles-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Styles`

- <span id="styles-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styles-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

