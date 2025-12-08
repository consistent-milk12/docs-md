*[rustversion](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct |  |
| [`Result`](#result) | type |  |

## Structs

### `Error`

```rust
struct Error {
    begin: proc_macro::Span,
    end: proc_macro::Span,
    msg: String,
}
```

#### Implementations

- <span id="error-new"></span>`fn new(span: Span, msg: impl Display) -> Self`

- <span id="error-new2"></span>`fn new2(begin: Span, end: Span, msg: impl Display) -> Self`

- <span id="error-group"></span>`fn group(group: Group, msg: impl Display) -> Self`

- <span id="error-into-compile-error"></span>`fn into_compile_error(self) -> TokenStream`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

