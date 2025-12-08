*[owo_colors](../index.md) / [styled_list](index.md)*

---

# Module `styled_list`

## Modules

- [`sealed`](sealed/index.md) - 

## Structs

### `StyledList<T, U>`

```rust
struct StyledList<T, U>(T, core::marker::PhantomData<fn(U)>)
where
    T: AsRef<[U]>,
    U: IsStyled;
```

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for StyledList<T, U>`

## Enums

### `Transition<'a>`

```rust
enum Transition<'a> {
    Noop,
    FullReset(&'a crate::Style),
    Style(crate::Style),
}
```

How the transition between two styles should be printed

#### Trait Implementations

##### `impl Display for Transition<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Transition<'a>`

