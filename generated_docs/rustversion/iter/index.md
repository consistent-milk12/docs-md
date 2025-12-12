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

*Defined in [`rustversion-1.0.22/src/iter.rs:5-8`](../../../.source_1765210505/rustversion-1.0.22/src/iter.rs#L5-L8)*

#### Implementations

- <span id="iterimpl-peek"></span>`fn peek(&mut self) -> Option<&TokenTree>`

#### Trait Implementations

##### `impl IntoIterator for IterImpl`

- <span id="iterimpl-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterimpl-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterimpl-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterImpl`

- <span id="iterimpl-iterator-type-item"></span>`type Item = TokenTree`

- <span id="iterimpl-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Functions

### `new`

```rust
fn new(tokens: proc_macro::TokenStream) -> IterImpl
```

*Defined in [`rustversion-1.0.22/src/iter.rs:10-15`](../../../.source_1765210505/rustversion-1.0.22/src/iter.rs#L10-L15)*

## Type Aliases

### `Iter<'a>`

```rust
type Iter<'a> = &'a mut IterImpl;
```

*Defined in [`rustversion-1.0.22/src/iter.rs:3`](../../../.source_1765210505/rustversion-1.0.22/src/iter.rs#L3)*

