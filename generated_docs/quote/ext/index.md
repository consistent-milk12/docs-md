*[quote](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`TokenStreamExt`](#tokenstreamext) | trait | TokenStream extension trait with methods for appending tokens. |

## Modules

- [`private`](private/index.md)

## Traits

### `TokenStreamExt`

```rust
trait TokenStreamExt: private::Sealed { ... }
```

TokenStream extension trait with methods for appending tokens.

This trait is sealed and cannot be implemented outside of the `quote` crate.

#### Required Methods

- `fn append<U>(&mut self, token: U)`

  For use by `ToTokens` implementations.

- `fn append_all<I>(&mut self, iter: I)`

  For use by `ToTokens` implementations.

- `fn append_separated<I, U>(&mut self, iter: I, op: U)`

  For use by `ToTokens` implementations.

- `fn append_terminated<I, U>(&mut self, iter: I, term: U)`

  For use by `ToTokens` implementations.

#### Implementors

- `proc_macro2::TokenStream`

