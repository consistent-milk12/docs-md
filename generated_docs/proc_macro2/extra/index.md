*[proc_macro2](../index.md) / [extra](index.md)*

---

# Module `extra`

Items which do not have a correspondence to any API in the proc_macro crate,
but are necessary to include in proc-macro2.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DelimSpan`](#delimspan) | struct | An object that holds a [`Group`]'s `span_open()` and `span_close()` together in a more compact representation than holding those 2 spans individually. |
| [`DelimSpanEnum`](#delimspanenum) | enum |  |

## Structs

### `DelimSpan`

```rust
struct DelimSpan {
    inner: DelimSpanEnum,
    _marker: crate::marker::ProcMacroAutoTraits,
}
```

*Defined in [`proc-macro2-1.0.103/src/extra.rs:82-85`](../../../.source_1765210505/proc-macro2-1.0.103/src/extra.rs#L82-L85)*

An object that holds a [`Group`](../index.md)'s `span_open()` and `span_close()` together
in a more compact representation than holding those 2 spans individually.


#### Implementations

- <span id="delimspan-new"></span>`fn new(group: &imp::Group) -> Self` — [`Group`](../imp/index.md#group)

- <span id="delimspan-join"></span>`fn join(&self) -> Span` — [`Span`](../index.md#span)

- <span id="delimspan-open"></span>`fn open(&self) -> Span` — [`Span`](../index.md#span)

- <span id="delimspan-close"></span>`fn close(&self) -> Span` — [`Span`](../index.md#span)

#### Trait Implementations

##### `impl Clone for DelimSpan`

- <span id="delimspan-clone"></span>`fn clone(&self) -> DelimSpan` — [`DelimSpan`](#delimspan)

##### `impl Copy for DelimSpan`

##### `impl Debug for DelimSpan`

- <span id="delimspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`proc-macro2-1.0.103/src/extra.rs:88-96`](../../../.source_1765210505/proc-macro2-1.0.103/src/extra.rs#L88-L96)*

#### Trait Implementations

##### `impl Clone for DelimSpanEnum`

- <span id="delimspanenum-clone"></span>`fn clone(&self) -> DelimSpanEnum` — [`DelimSpanEnum`](#delimspanenum)

##### `impl Copy for DelimSpanEnum`

