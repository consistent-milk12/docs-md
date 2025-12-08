*[miette](../index.md) / [named_source](index.md)*

---

# Module `named_source`

## Structs

### `NamedSource<S: SourceCode + 'static>`

```rust
struct NamedSource<S: SourceCode + 'static> {
    source: S,
    name: String,
    language: Option<String>,
}
```

Utility struct for when you have a regular [`SourceCode`](../index.md) type that doesn't
implement `name`. For example [`String`](../../clap_builder/index.md). Or if you want to override the
`name` returned by the `SourceCode`.

#### Implementations

- `fn new(name: impl AsRef<str>, source: S) -> Self`

- `fn name(self: &Self) -> &str`

- `fn inner(self: &Self) -> &S`

- `fn with_language(self: Self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl<S: $crate::clone::Clone + SourceCode + 'static> Clone for NamedSource<S>`

- `fn clone(self: &Self) -> NamedSource<S>` — [`NamedSource`](../index.md)

##### `impl<S: SourceCode> Debug for NamedSource<S>`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<S: $crate::cmp::Eq + SourceCode + 'static> Eq for NamedSource<S>`

##### `impl<S: $crate::hash::Hash + SourceCode + 'static> Hash for NamedSource<S>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<S: $crate::cmp::Ord + SourceCode + 'static> Ord for NamedSource<S>`

- `fn cmp(self: &Self, other: &NamedSource<S>) -> $crate::cmp::Ordering` — [`NamedSource`](../index.md)

##### `impl<D> OwoColorize for NamedSource<S>`

##### `impl<S: $crate::cmp::PartialEq + SourceCode + 'static> PartialEq for NamedSource<S>`

- `fn eq(self: &Self, other: &NamedSource<S>) -> bool` — [`NamedSource`](../index.md)

##### `impl<S: $crate::cmp::PartialOrd + SourceCode + 'static> PartialOrd for NamedSource<S>`

- `fn partial_cmp(self: &Self, other: &NamedSource<S>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`NamedSource`](../index.md)

##### `impl<S: SourceCode + 'static> SourceCode for NamedSource<S>`

- `fn read_span<'a>(self: &'a Self, span: &crate::SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>` — [`SourceSpan`](../index.md), [`SpanContents`](../index.md), [`MietteError`](../index.md)

##### `impl<S: SourceCode + 'static> StructuralPartialEq for NamedSource<S>`

