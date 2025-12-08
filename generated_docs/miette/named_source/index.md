*[miette](../index.md) / [named_source](index.md)*

---

# Module `named_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NamedSource`](#namedsource) | struct | Utility struct for when you have a regular [`SourceCode`] type that doesn't |

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

- <span id="namedsource-new"></span>`fn new(name: impl AsRef<str>, source: S) -> Self`

- <span id="namedsource-name"></span>`fn name(&self) -> &str`

- <span id="namedsource-inner"></span>`fn inner(&self) -> &S`

- <span id="namedsource-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl<S: clone::Clone + SourceCode + 'static> Clone for NamedSource<S>`

- <span id="namedsource-clone"></span>`fn clone(&self) -> NamedSource<S>` — [`NamedSource`](../index.md)

##### `impl<S: SourceCode> Debug for NamedSource<S>`

- <span id="namedsource-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<S: cmp::Eq + SourceCode + 'static> Eq for NamedSource<S>`

##### `impl<S: hash::Hash + SourceCode + 'static> Hash for NamedSource<S>`

- <span id="namedsource-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<S: cmp::Ord + SourceCode + 'static> Ord for NamedSource<S>`

- <span id="namedsource-cmp"></span>`fn cmp(&self, other: &NamedSource<S>) -> cmp::Ordering` — [`NamedSource`](../index.md)

##### `impl<D> OwoColorize for NamedSource<S>`

##### `impl<S: cmp::PartialEq + SourceCode + 'static> PartialEq for NamedSource<S>`

- <span id="namedsource-eq"></span>`fn eq(&self, other: &NamedSource<S>) -> bool` — [`NamedSource`](../index.md)

##### `impl<S: cmp::PartialOrd + SourceCode + 'static> PartialOrd for NamedSource<S>`

- <span id="namedsource-partial-cmp"></span>`fn partial_cmp(&self, other: &NamedSource<S>) -> option::Option<cmp::Ordering>` — [`NamedSource`](../index.md)

##### `impl<S: SourceCode + 'static> SourceCode for NamedSource<S>`

- <span id="namedsource-read-span"></span>`fn read_span<'a>(self: &'a Self, span: &crate::SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>` — [`SourceSpan`](../index.md), [`SpanContents`](../index.md), [`MietteError`](../index.md)

##### `impl<S: SourceCode + 'static> StructuralPartialEq for NamedSource<S>`

