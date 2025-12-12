*[clap_derive](../../index.md) / [utils](../index.md) / [spanned](index.md)*

---

# Module `spanned`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sp`](#sp) | struct | An entity with a span attached. |

## Structs

### `Sp<T>`

```rust
struct Sp<T> {
    val: T,
    span: proc_macro2::Span,
}
```

*Defined in [`clap_derive-4.5.49/src/utils/spanned.rs:9-12`](../../../../.source_1765521767/clap_derive-4.5.49/src/utils/spanned.rs#L9-L12)*

An entity with a span attached.

#### Implementations

- <span id="sp-new"></span>`fn new(val: T, span: Span) -> Self`

- <span id="sp-get"></span>`fn get(&self) -> &T`

- <span id="sp-span"></span>`fn span(&self) -> Span`

#### Trait Implementations

##### `impl<T: AsRef<str>> AsRef for Sp<T>`

- <span id="sp-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<T: clone::Clone> Clone for Sp<T>`

- <span id="sp-clone"></span>`fn clone(&self) -> Sp<T>` — [`Sp`](#sp)

##### `impl<T: marker::Copy> Copy for Sp<T>`

##### `impl<T: fmt::Debug> Debug for Sp<T>`

- <span id="sp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Deref for Sp<T>`

- <span id="sp-deref-type-target"></span>`type Target = T`

- <span id="sp-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> DerefMut for Sp<T>`

- <span id="sp-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<U, T: PartialEq<U>> PartialEq for Sp<T>`

- <span id="sp-eq"></span>`fn eq(&self, other: &U) -> bool`

##### `impl<T> Receiver for Sp<T>`

- <span id="sp-receiver-type-target"></span>`type Target = T`

##### `impl<T> Spanned for Sp<T>`

- <span id="sp-span"></span>`fn span(&self) -> Span`

##### `impl<T: ToTokens> ToTokens for Sp<T>`

- <span id="sp-to-tokens"></span>`fn to_tokens(&self, stream: &mut TokenStream)`

