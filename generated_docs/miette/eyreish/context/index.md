*[miette](../../index.md) / [eyreish](../index.md) / [context](index.md)*

---

# Module `context`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ext`](#ext) | mod |  |
| [`private`](#private) | mod |  |
| [`Quoted`](#quoted) | struct |  |

## Modules

- [`ext`](ext/index.md)
- [`private`](private/index.md)

## Structs

### `Quoted<D>`

```rust
struct Quoted<D>(D);
```

*Defined in [`miette-7.6.0/src/eyreish/context.rs:228`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/context.rs#L228)*

#### Trait Implementations

##### `impl<D> Debug for Quoted<D>`

- <span id="quoted-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for Quoted<D>`

##### `impl Write for Quoted<&mut fmt::Formatter<'_>>`

- <span id="quoted-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

