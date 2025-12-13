*[anstyle](../index.md) / [style](index.md)*

---

# Module `style`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Style`](#style) | struct | ANSI Text styling |
| [`StyleDisplay`](#styledisplay) | struct |  |

## Structs

### `Style`

```rust
struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}
```

*Defined in [`anstyle-1.0.13/src/style.rs:18-23`](../../../.source_1765521767/anstyle-1.0.13/src/style.rs#L18-L23)*

ANSI Text styling

You can print a `Style` to render the corresponding ANSI code.
Using the alternate flag `#` will render the ANSI reset code, if needed.
Together, this makes it convenient to render styles using inline format arguments.

# Examples

```rust
let style = anstyle::Style::new().bold();

let value = 42;
println!("{style}{value}{style:#}");
```

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

  No effects enabled

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new();

  ```

- <span id="style-fg-color"></span>`const fn fg_color(self, fg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set foreground color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-bg-color"></span>`const fn bg_color(self, bg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set background color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().bg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-underline-color"></span>`const fn underline_color(self, underline: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set underline color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().underline_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-effects"></span>`const fn effects(self, effects: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

  Set text effects

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE);

  ```

- <span id="style-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Style` also implements `Display` directly, so calling this method is optional.

- <span id="style-fmt-to"></span>`fn fmt_to(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

- <span id="style-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the ANSI code

- <span id="style-render-reset"></span>`fn render_reset(self) -> impl core::fmt::Display + Copy`

  Renders the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

- <span id="style-write-reset-to"></span>`fn write_reset_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

#### Trait Implementations

##### `impl Any for Style`

- <span id="style-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl BitOr for Style`

- <span id="style-bitor-type-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl BitOrAssign for Style`

- <span id="style-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl<T> Borrow for Style`

- <span id="style-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Style`

- <span id="style-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md#style)

##### `impl CloneToUninit for Style`

- <span id="style-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](../index.md#style)

##### `impl Display for Style`

- <span id="style-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl<T> From for Style`

- <span id="style-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Style`

- <span id="style-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Style`

- <span id="style-ord-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](../index.md#style)

##### `impl PartialEq for Style`

- <span id="style-partialeq-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md#style)

##### `impl PartialOrd for Style`

- <span id="style-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](../index.md#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-sub-type-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl SubAssign for Style`

- <span id="style-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl ToOwned for Style`

- <span id="style-toowned-type-owned"></span>`type Owned = T`

- <span id="style-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="style-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Style`

- <span id="style-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Style`

- <span id="style-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="style-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Style`

- <span id="style-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="style-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

*Defined in [`anstyle-1.0.13/src/style.rs:423`](../../../.source_1765521767/anstyle-1.0.13/src/style.rs#L423)*

#### Trait Implementations

##### `impl Any for StyleDisplay`

- <span id="styledisplay-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyleDisplay`

- <span id="styledisplay-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyleDisplay`

- <span id="styledisplay-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyleDisplay`

- <span id="styledisplay-clone"></span>`fn clone(&self) -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl CloneToUninit for StyleDisplay`

- <span id="styledisplay-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- <span id="styledisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleDisplay`

- <span id="styledisplay-default"></span>`fn default() -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Display for StyleDisplay`

- <span id="styledisplay-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for StyleDisplay`

- <span id="styledisplay-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyleDisplay`

- <span id="styledisplay-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StyleDisplay`

- <span id="styledisplay-toowned-type-owned"></span>`type Owned = T`

- <span id="styledisplay-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styledisplay-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StyleDisplay`

- <span id="styledisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StyleDisplay`

- <span id="styledisplay-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledisplay-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyleDisplay`

- <span id="styledisplay-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledisplay-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

