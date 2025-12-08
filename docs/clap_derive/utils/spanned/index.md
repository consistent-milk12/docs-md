*[clap_derive](../../index.md) / [utils](../index.md) / [spanned](index.md)*

---

# Module `spanned`

## Structs

### `Sp<T>`

```rust
struct Sp<T> {
    val: T,
    span: proc_macro2::Span,
}
```

An entity with a span attached.

#### Implementations

- `fn new(val: T, span: Span) -> Self`

- `fn get(self: &Self) -> &T`

- `fn span(self: &Self) -> Span`

#### Trait Implementations

##### `impl<T: AsRef<str>> AsRef for Sp<T>`

- `fn as_ref(self: &Self) -> &str`

##### `impl<T: $crate::clone::Clone> Clone for Sp<T>`

- `fn clone(self: &Self) -> Sp<T>` — [`Sp`](#sp)

##### `impl<T: $crate::marker::Copy> Copy for Sp<T>`

##### `impl<T: $crate::fmt::Debug> Debug for Sp<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Deref for Sp<T>`

- `type Target = T`

- `fn deref(self: &Self) -> &T`

##### `impl<T> DerefMut for Sp<T>`

- `fn deref_mut(self: &mut Self) -> &mut T`

##### `impl<U, T: PartialEq<U>> PartialEq for Sp<T>`

- `fn eq(self: &Self, other: &U) -> bool`

##### `impl<P, T> Receiver for Sp<T>`

- `type Target = T`

##### `impl<T> Spanned for Sp<T>`

- `fn span(self: &Self) -> Span`

##### `impl<T: ToTokens> ToTokens for Sp<T>`

- `fn to_tokens(self: &Self, stream: &mut TokenStream)`

