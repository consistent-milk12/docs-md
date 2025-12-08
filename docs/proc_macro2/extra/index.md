*[proc_macro2](../index.md) / [extra](index.md)*

---

# Module `extra`

Items which do not have a correspondence to any API in the proc_macro crate,
but are necessary to include in proc-macro2.

## Structs

### `DelimSpan`

```rust
struct DelimSpan {
    inner: DelimSpanEnum,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

An object that holds a [`Group`](../imp/index.md)'s `span_open()` and `span_close()` together
in a more compact representation than holding those 2 spans individually.


#### Implementations

- `fn new(group: &imp::Group) -> Self` — [`Group`](../imp/index.md)

- `fn join(self: &Self) -> Span` — [`Span`](../index.md)

- `fn open(self: &Self) -> Span` — [`Span`](../index.md)

- `fn close(self: &Self) -> Span` — [`Span`](../index.md)

#### Trait Implementations

##### `impl Clone for DelimSpan`

- `fn clone(self: &Self) -> DelimSpan` — [`DelimSpan`](#delimspan)

##### `impl Copy for DelimSpan`

##### `impl Debug for DelimSpan`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DelimSpanEnum`

```rust
enum DelimSpanEnum {
    Compiler {
        join: proc_macro::Span,
        open: proc_macro::Span,
        close: proc_macro::Span,
    },
    Fallback(fallback::Span),
}
```

#### Trait Implementations

##### `impl Clone for DelimSpanEnum`

- `fn clone(self: &Self) -> DelimSpanEnum` — [`DelimSpanEnum`](#delimspanenum)

##### `impl Copy for DelimSpanEnum`

