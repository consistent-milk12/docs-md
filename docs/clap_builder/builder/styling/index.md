*[clap_builder](../../index.md) / [builder](../index.md) / [styling](index.md)*

---

# Module `styling`

Terminal [`Styles`](clap_builder/builder/styling/index.md) for help and error output

## Structs

### `Styles`

```rust
struct Styles {
    // [REDACTED: Private Fields]
}
```

Terminal styling definitions

See also [`Command::styles`](#styles).

# Example

clap v3 styling
```rust
# use clap_builder as clap;
# use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

#### Implementations

- `const fn plain() -> Self`
  No terminal styling

- `const fn styled() -> Self`
  Default terminal styling

- `const fn header(self: Self, style: Style) -> Self`
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- `const fn error(self: Self, style: Style) -> Self`
  Error heading

- `const fn usage(self: Self, style: Style) -> Self`
  Usage heading

- `const fn literal(self: Self, style: Style) -> Self`
  Literal command-line syntax, e.g. `--help`

- `const fn placeholder(self: Self, style: Style) -> Self`
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- `const fn valid(self: Self, style: Style) -> Self`
  Highlight suggested usage

- `const fn invalid(self: Self, style: Style) -> Self`
  Highlight invalid usage

- `const fn context(self: Self, style: Style) -> Self`
  Highlight all specified contexts, e.g. `[default: false]`

- `const fn context_value(self: Self, style: Style) -> Self`
  Highlight values within all of the context, e.g. the `false` in `[default: false]`

- `const fn get_header(self: &Self) -> &Style`
  General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]

- `const fn get_error(self: &Self) -> &Style`
  Error heading

- `const fn get_usage(self: &Self) -> &Style`
  Usage heading

- `const fn get_literal(self: &Self) -> &Style`
  Literal command-line syntax, e.g. `--help`

- `const fn get_placeholder(self: &Self) -> &Style`
  Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]

- `const fn get_valid(self: &Self) -> &Style`
  Highlight suggested usage

- `const fn get_invalid(self: &Self) -> &Style`
  Highlight invalid usage

- `const fn get_context(self: &Self) -> &Style`
  Highlight all specified contexts, e.g. `[default: false]`

- `const fn get_context_value(self: &Self) -> &Style`
  Highlight values within all of the context, e.g. the `false` in `[default: false]`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Styles`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Self`

