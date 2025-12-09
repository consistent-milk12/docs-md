*[clap_builder](../../index.md) / [output](../index.md) / [fmt](index.md)*

---

# Module `fmt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Colorizer`](#colorizer) | struct |  |
| [`Stream`](#stream) | enum |  |

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

- <span id="colorizer-print"></span>`fn print(&self) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Clone for Colorizer`

- <span id="colorizer-clone"></span>`fn clone(&self) -> Colorizer` — [`Colorizer`](#colorizer)

##### `impl Debug for Colorizer`

- <span id="colorizer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Colorizer`

- <span id="colorizer-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> ToString for Colorizer`

- <span id="colorizer-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="stream-clone"></span>`fn clone(&self) -> Stream` — [`Stream`](#stream)

##### `impl Copy for Stream`

##### `impl Debug for Stream`

- <span id="stream-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Stream`

##### `impl PartialEq for Stream`

- <span id="stream-eq"></span>`fn eq(&self, other: &Stream) -> bool` — [`Stream`](#stream)

##### `impl StructuralPartialEq for Stream`

