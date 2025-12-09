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

*Defined in [`rayon-1.11.0/src/iter/mod.rs:3479-3490`](../../../../.source_1765210505/rayon-1.11.0/src/iter/mod.rs#L3479-L3490)*

Clone of `std::ops::Try`.

Implementing this trait is not permitted outside of `rayon`.

#### Associated Types

- `type Output`

- `type Residual`

#### Required Methods

- `fn from_output(output: <Self as >::Output) -> Self`

- `fn from_residual(residual: <Self as >::Residual) -> Self`

- `fn branch(self) -> ControlFlow<<Self as >::Residual, <Self as >::Output>`

#### Implementors

- `Option<T>`
- `Result<T, E>`
- `std::ops::ControlFlow<B, C>`
- `std::task::Poll<Option<Result<T, E>>>`
- `std::task::Poll<Result<T, E>>`

