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

##### `impl<T, U> Display for StyledList<T, U>`

- <span id="styledlist-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for StyledList<T, U>`

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

##### `impl Display for Transition<'_>`

- <span id="transition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for Transition<'a>`

