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

*Defined in [`rustversion-1.0.22/src/iter.rs:5-8`](../../../.source_1765633015/rustversion-1.0.22/src/iter.rs#L5-L8)*

#### Implementations

- <span id="iterimpl-peek"></span>`fn peek(&mut self) -> Option<&TokenTree>`

#### Trait Implementations

##### `impl Any for IterImpl`

- <span id="iterimpl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterImpl`

- <span id="iterimpl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterImpl`

- <span id="iterimpl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterImpl`

- <span id="iterimpl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterImpl`

- <span id="iterimpl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterImpl`

- <span id="iterimpl-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterimpl-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterimpl-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for IterImpl`

- <span id="iterimpl-iterator-type-item"></span>`type Item = TokenTree`

- <span id="iterimpl-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for IterImpl`

- <span id="iterimpl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterimpl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterImpl`

- <span id="iterimpl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterimpl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `new`

```rust
fn new(tokens: proc_macro::TokenStream) -> IterImpl
```

*Defined in [`rustversion-1.0.22/src/iter.rs:10-15`](../../../.source_1765633015/rustversion-1.0.22/src/iter.rs#L10-L15)*

## Type Aliases

### `Iter<'a>`

```rust
type Iter<'a> = &'a mut IterImpl;
```

*Defined in [`rustversion-1.0.22/src/iter.rs:3`](../../../.source_1765633015/rustversion-1.0.22/src/iter.rs#L3)*

