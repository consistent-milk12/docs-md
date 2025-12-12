*[serde_derive](../../index.md) / [internals](../index.md) / [ctxt](index.md)*

---

# Module `ctxt`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ctxt`](#ctxt) | struct | A type to collect errors together and format them. |

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

*Defined in [`serde_derive-1.0.228/src/internals/ctxt.rs:12-16`](../../../../.source_1765521767/serde_derive-1.0.228/src/internals/ctxt.rs#L12-L16)*

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- <span id="ctxt-new"></span>`fn new() -> Self`

- <span id="ctxt-error-spanned-by"></span>`fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T)`

- <span id="ctxt-syn-error"></span>`fn syn_error(&self, err: syn::Error)`

- <span id="ctxt-check"></span>`fn check(self) -> syn::Result<()>`

#### Trait Implementations

##### `impl Default for Ctxt`

- <span id="ctxt-default"></span>`fn default() -> Ctxt` — [`Ctxt`](#ctxt)

##### `impl Drop for Ctxt`

- <span id="ctxt-drop"></span>`fn drop(&mut self)`

