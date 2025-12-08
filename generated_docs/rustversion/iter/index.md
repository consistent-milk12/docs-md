*[rustversion](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterImpl`](#iterimpl) | struct |  |
| [`new`](#new) | fn |  |
| [`Iter`](#iter) | type |  |

## Structs

### `IterImpl`

```rust
struct IterImpl {
    stack: Vec<token_stream::IntoIter>,
    peeked: Option<proc_macro::TokenTree>,
}
```

#### Implementations

- <span id="iterimpl-peek"></span>`fn peek(&mut self) -> Option<&TokenTree>`

#### Trait Implementations

##### `impl<I> IntoIterator for IterImpl`

- <span id="iterimpl-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterimpl-intoiter"></span>`type IntoIter = I`

- <span id="iterimpl-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterImpl`

- <span id="iterimpl-item"></span>`type Item = TokenTree`

- <span id="iterimpl-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

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

