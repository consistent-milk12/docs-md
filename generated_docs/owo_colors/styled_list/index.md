*[owo_colors](../index.md) / [styled_list](index.md)*

---

# Module `styled_list`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`StyledList`](#styledlist) | struct | A collection of [`Styled`] items that are displayed in such a way as to minimize the amount of characters that are written when displayed. |
| [`Transition`](#transition) | enum | How the transition between two styles should be printed |

## Modules

- [`sealed`](sealed/index.md)

## Structs

### `StyledList<T, U>`

```rust
struct StyledList<T, U>(T, core::marker::PhantomData<fn(U)>)
where
    T: AsRef<[U]>,
    U: IsStyled;
```

*Defined in [`owo-colors-4.2.3/src/styled_list.rs:64-67`](../../../.source_1765521767/owo-colors-4.2.3/src/styled_list.rs#L64-L67)*

A collection of [`Styled`](../index.md) items that are displayed in such a way as to minimize the amount of characters
that are written when displayed.

```rust
use owo_colors::{Style, Styled, StyledList};

let styled_items = [
    Style::new().red().style("Hello "),
    Style::new().green().style("World"),
 ];

// 29 characters
let normal_length = styled_items.iter().map(|item| format!("{}", item).len()).sum::<usize>();
// 25 characters
let styled_length = format!("{}", StyledList::from(styled_items)).len();

assert!(styled_length < normal_length);
```

#### Trait Implementations

##### `impl<T> Any for StyledList<T, U>`

- <span id="styledlist-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyledList<T, U>`

- <span id="styledlist-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyledList<T, U>`

- <span id="styledlist-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, U> Display for StyledList<T, U>`

- <span id="styledlist-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StyledList<T, U>`

- <span id="styledlist-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for StyledList<T, U>`

- <span id="styledlist-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for StyledList<T, U>`

##### `impl<T, U> TryFrom for StyledList<T, U>`

- <span id="styledlist-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledlist-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for StyledList<T, U>`

- <span id="styledlist-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledlist-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Transition<'a>`

```rust
enum Transition<'a> {
    Noop,
    FullReset(&'a crate::Style),
    Style(crate::Style),
}
```

*Defined in [`owo-colors-4.2.3/src/styled_list.rs:157-161`](../../../.source_1765521767/owo-colors-4.2.3/src/styled_list.rs#L157-L161)*

How the transition between two styles should be printed

#### Trait Implementations

##### `impl Any for Transition<'a>`

- <span id="transition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transition<'a>`

- <span id="transition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transition<'a>`

- <span id="transition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Display for Transition<'_>`

- <span id="transition-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Transition<'a>`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Transition<'a>`

- <span id="transition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for Transition<'a>`

##### `impl<U> TryFrom for Transition<'a>`

- <span id="transition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="transition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Transition<'a>`

- <span id="transition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="transition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

