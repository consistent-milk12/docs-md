*[rustversion](../index.md) / [error](index.md)*

---

# Module `error`

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

- `fn new(span: Span, msg: impl Display) -> Self`

- `fn new2(begin: Span, end: Span, msg: impl Display) -> Self`

- `fn group(group: Group, msg: impl Display) -> Self`

- `fn into_compile_error(self: Self) -> TokenStream`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = std::result::Result<T, E>;
```

