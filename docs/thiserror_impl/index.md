# Crate `thiserror_impl`

## Modules

- [`ast`](ast/index.md) - 
- [`attr`](attr/index.md) - 
- [`expand`](expand/index.md) - 
- [`fallback`](fallback/index.md) - 
- [`fmt`](fmt/index.md) - 
- [`generics`](generics/index.md) - 
- [`prop`](prop/index.md) - 
- [`scan_expr`](scan_expr/index.md) - 
- [`unraw`](unraw/index.md) - 
- [`valid`](valid/index.md) - 

## Structs

### `private`

```rust
struct private;
```

#### Trait Implementations

##### `impl<T> Spanned for private`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for private`

- `fn to_tokens(self: &Self, tokens: &mut proc_macro2::TokenStream)`

