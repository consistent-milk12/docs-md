*[quote](../index.md) / [ext](index.md)*

---

# Module `ext`

## Modules

- [`private`](private/index.md) - 

## Traits

### `TokenStreamExt`

```rust
trait TokenStreamExt: private::Sealed { ... }
```

TokenStream extension trait with methods for appending tokens.

This trait is sealed and cannot be implemented outside of the `quote` crate.

#### Required Methods

- `fn append<U>(self: &mut Self, token: U)`

  For use by `ToTokens` implementations.

- `fn append_all<I>(self: &mut Self, iter: I)`

  For use by `ToTokens` implementations.

- `fn append_separated<I, U>(self: &mut Self, iter: I, op: U)`

  For use by `ToTokens` implementations.

- `fn append_terminated<I, U>(self: &mut Self, iter: I, term: U)`

  For use by `ToTokens` implementations.

