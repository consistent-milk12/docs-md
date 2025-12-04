*[syn](../index.md) / [ext](index.md)*

---

# Module `ext`

Extension traits to provide parsing methods on foreign types.

## Traits

### `IdentExt`

```rust
trait IdentExt: Sized + private::Sealed { ... }
```

Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.

This trait is sealed and cannot be implemented for types outside of Syn. It
is implemented only for `proc_macro2::Ident`.

#### Required Methods

- `fn parse_any(input: ParseStream<'_>) -> Result<Self>`

  Parses any identifier including keywords.

- `const peek_any: private::PeekFn`

- `fn unraw(self: &Self) -> Ident`

  Strips the raw marker `r#`, if any, from the beginning of an ident.

