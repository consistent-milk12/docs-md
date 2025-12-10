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

*Defined in [`syn-2.0.111/src/ext.rs:22-95`](../../../.source_1765210505/syn-2.0.111/src/ext.rs#L22-L95)*

Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.

This trait is sealed and cannot be implemented for types outside of Syn. It
is implemented only for `proc_macro2::Ident`.

#### Associated Constants

- `const peek_any: private::PeekFn`

#### Required Methods

- `fn parse_any(input: ParseStream<'_>) -> Result<Self>`

  Parses any identifier including keywords.

- `fn unraw(&self) -> Ident`

  Strips the raw marker `r#`, if any, from the beginning of an ident.

#### Implementors

- [`Ident`](../ident/index.md#ident)

### `TokenStreamExt`

```rust
trait TokenStreamExt { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:135-137`](../../../.source_1765210505/syn-2.0.111/src/ext.rs#L135-L137)*

#### Required Methods

- `fn append(&mut self, token: TokenTree)`

#### Implementors

- `proc_macro2::TokenStream`

### `PunctExt`

```rust
trait PunctExt { ... }
```

*Defined in [`syn-2.0.111/src/ext.rs:145-147`](../../../.source_1765210505/syn-2.0.111/src/ext.rs#L145-L147)*

#### Required Methods

- `fn new_spanned(ch: char, spacing: Spacing, span: Span) -> Self`

#### Implementors

- `proc_macro2::Punct`

