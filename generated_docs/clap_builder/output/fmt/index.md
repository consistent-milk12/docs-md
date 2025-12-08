*[clap_builder](../../index.md) / [output](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Structs

### `Colorizer`

```rust
struct Colorizer {
    stream: Stream,
    color_when: crate::util::color::ColorChoice,
    content: crate::builder::StyledStr,
}
```

#### Implementations

- `fn new(stream: Stream, color_when: ColorChoice) -> Self` — [`Stream`](#stream), [`ColorChoice`](../../index.md)

- `fn with_content(self: Self, content: StyledStr) -> Self` — [`StyledStr`](../../builder/index.md)

#### Trait Implementations

##### `impl Clone for Colorizer`

- `fn clone(self: &Self) -> Colorizer` — [`Colorizer`](#colorizer)

##### `impl Debug for Colorizer`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for Colorizer`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> ToString for Colorizer`

- `fn to_string(self: &Self) -> String`

## Enums

### `Stream`

```rust
enum Stream {
    Stdout,
    Stderr,
}
```

#### Trait Implementations

##### `impl Clone for Stream`

- `fn clone(self: &Self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Stream`

##### `impl PartialEq for Stream`

- `fn eq(self: &Self, other: &Stream) -> bool` — [`Stream`](#stream)

##### `impl StructuralPartialEq for Stream`

