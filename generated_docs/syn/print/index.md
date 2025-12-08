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

#### Trait Implementations

##### `impl<T> Sealed for TokensOrDefault<'a, T>`

##### `impl<T> Spanned for TokensOrDefault<'a, T>`

- <span id="tokensordefault-span"></span>`fn span(&self) -> Span`

##### `impl<'a, T> ToTokens for TokensOrDefault<'a, T>`

- <span id="tokensordefault-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

