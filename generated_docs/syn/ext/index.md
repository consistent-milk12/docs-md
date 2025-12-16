*[syn](../index.md) / [ext](index.md)*

---

# Module `ext`

Extension traits to provide parsing methods on foreign types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`IdentExt`](#identext) | trait | Additional methods for `Ident` not provided by proc-macro2 or libproc_macro. |
| [`TokenStreamExt`](#tokenstreamext) | trait |  |
| [`PunctExt`](#punctext) | trait |  |

## Modules

- [`private`](private/index.md)

## Traits

### `IdentExt`

```rust
trait IdentExt: Sized + private::Sealed { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:22-95`](../../../.source_1765894658/syn-2.0.111/src/ext.rs#L22-L95)*

Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.

This trait is sealed and cannot be implemented for types outside of Syn. It
is implemented only for `proc_macro2::Ident`.

#### Associated Constants

- `const peek_any: private::PeekFn`

#### Required Methods

- `fn parse_any(input: ParseStream<'_>) -> Result<Self>`

  Parses any identifier including keywords.
  
  This is useful when parsing macro input which allows Rust keywords as
  identifiers.
  
  # Example
  
  ```rust
  use syn::{Error, Ident, Result, Token};
  use syn::ext::IdentExt;
  use syn::parse::ParseStream;
  
  mod kw {
      syn::custom_keyword!(name);
  }
  
  // Parses input that looks like `name = NAME` where `NAME` can be
  // any identifier.
  //
  // Examples:
  //
  //     name = anything
  //     name = impl
  fn parse_dsl(input: ParseStream) -> Result<Ident> {
      input.parse::<kw::name>()?;
      input.parse::<Token![=]>()?;
      let name = input.call(Ident::parse_any)?;
      Ok(name)
  }
  ```

- `fn unraw(&self) -> Ident`

  Strips the raw marker `r#`, if any, from the beginning of an ident.
  
    - unraw(`x`) = `x`
    - unraw(`move`) = `move`
    - unraw(`r#move`) = `move`
  
  # Example
  
  In the case of interop with other languages like Python that have a
  different set of keywords than Rust, we might come across macro input
  that involves raw identifiers to refer to ordinary variables in the
  other language with a name that happens to be a Rust keyword.
  
  The function below appends an identifier from the caller's input onto a
  fixed prefix. Without using `unraw()`, this would tend to produce
  invalid identifiers like `__pyo3_get_r#move`.
  
  ```rust
  use proc_macro2::Span;
  use syn::Ident;
  use syn::ext::IdentExt;
  
  fn ident_for_getter(variable: &Ident) -> Ident {
      let getter = format!("__pyo3_get_{}", variable.unraw());
      Ident::new(&getter, Span::call_site())
  }
  ```

#### Implementors

- [`Ident`](../ident/index.md#ident)

### `TokenStreamExt`

```rust
trait TokenStreamExt { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:135-137`](../../../.source_1765894658/syn-2.0.111/src/ext.rs#L135-L137)*

#### Required Methods

- `fn append(&mut self, token: TokenTree)`

#### Implementors

- `proc_macro2::TokenStream`

### `PunctExt`

```rust
trait PunctExt { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:145-147`](../../../.source_1765894658/syn-2.0.111/src/ext.rs#L145-L147)*

#### Required Methods

- `fn new_spanned(ch: char, spacing: Spacing, span: Span) -> Self`

#### Implementors

- `proc_macro2::Punct`

