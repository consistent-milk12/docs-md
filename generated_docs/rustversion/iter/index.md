*[rustversion](../index.md) / [iter](index.md)*

---

# Module `iter`

## Structs

### `IterImpl`

```rust
struct IterImpl {
    stack: Vec<token_stream::IntoIter>,
    peeked: Option<proc_macro::TokenTree>,
}
```

#### Implementations

- `fn peek(self: &mut Self) -> Option<&TokenTree>`

#### Trait Implementations

##### `impl<I> IntoIterator for IterImpl`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for IterImpl`

- `type Item = TokenTree`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Functions

### `new`

```rust
fn new(tokens: proc_macro::TokenStream) -> IterImpl
```

## Type Aliases

### `Iter<'a>`

```rust
type Iter<'a> = &'a mut IterImpl;
```

