*[syn](../index.md) / [print](index.md)*

---

# Module `print`

## Structs

### `TokensOrDefault<'a, T: 'a>`

```rust
struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);
```

#### Trait Implementations

##### `impl<T> Sealed for TokensOrDefault<'a, T>`

##### `impl<T> Spanned for TokensOrDefault<'a, T>`

- `fn span(self: &Self) -> Span`

##### `impl<'a, T> ToTokens for TokensOrDefault<'a, T>`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

