*[rayon](../../index.md) / [iter](../index.md) / [private](index.md)*

---

# Module `private`

We hide the `Try` trait in a private module, as it's only meant to be a
stable clone of the standard library's `Try` trait, as yet unstable.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Try`](#try) | trait | Clone of `std::ops::Try`. |

## Traits

### `Try`

```rust
trait Try { ... }
```

Clone of `std::ops::Try`.

Implementing this trait is not permitted outside of `rayon`.

#### Required Methods

- `type Output`

- `type Residual`

- `fn from_output(output: <Self as >::Output) -> Self`

- `fn from_residual(residual: <Self as >::Residual) -> Self`

- `fn branch(self) -> ControlFlow<<Self as >::Residual, <Self as >::Output>`

