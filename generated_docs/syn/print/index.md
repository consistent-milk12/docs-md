*[syn](../index.md) / [print](index.md)*

---

# Module `print`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokensOrDefault`](#tokensordefault) | struct |  |

## Structs

### `TokensOrDefault<'a, T: 'a>`

```rust
struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);
```

*Defined in [`syn-2.0.111/src/print.rs:4`](../../../.source_1765210505/syn-2.0.111/src/print.rs#L4)*

#### Trait Implementations

##### `impl<T> Sealed for TokensOrDefault<'a, T>`

##### `impl<T> Spanned for TokensOrDefault<'a, T>`

- <span id="tokensordefault-span"></span>`fn span(&self) -> Span`

##### `impl<T> ToTokens for TokensOrDefault<'a, T>`

- <span id="tokensordefault-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

