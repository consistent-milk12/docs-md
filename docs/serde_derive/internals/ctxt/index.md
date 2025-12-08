*[serde_derive](../../index.md) / [internals](../index.md) / [ctxt](index.md)*

---

# Module `ctxt`

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- `fn new() -> Self`

- `fn error_spanned_by<A: ToTokens, T: Display>(self: &Self, obj: A, msg: T)`

- `fn syn_error(self: &Self, err: syn::Error)`

- `fn check(self: Self) -> syn::Result<()>`

#### Trait Implementations

##### `impl Default for Ctxt`

- `fn default() -> Ctxt` — [`Ctxt`](../index.md)

##### `impl Drop for Ctxt`

- `fn drop(self: &mut Self)`

