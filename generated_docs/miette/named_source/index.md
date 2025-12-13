*[miette](../index.md) / [named_source](index.md)*

---

# Module `named_source`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NamedSource`](#namedsource) | struct | Utility struct for when you have a regular [`SourceCode`] type that doesn't implement `name`. |

## Structs

### `NamedSource<S: SourceCode + 'static>`

```rust
struct NamedSource<S: SourceCode + 'static> {
    source: S,
    name: String,
    language: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/named_source.rs:7-11`](../../../.source_1765521767/miette-7.6.0/src/named_source.rs#L7-L11)*

Utility struct for when you have a regular [`SourceCode`](../index.md) type that doesn't
implement `name`. For example [`String`](../../cargo_platform/index.md). Or if you want to override the
`name` returned by the `SourceCode`.

#### Implementations

- <span id="namedsource-new"></span>`fn new(name: impl AsRef<str>, source: S) -> Self`

  Create a new `NamedSource` using a regular [`SourceCode`](../index.md) and giving

  its returned [`SpanContents`](../index.md) a name.

- <span id="namedsource-name"></span>`fn name(&self) -> &str`

  Gets the name of this `NamedSource`.

- <span id="namedsource-inner"></span>`fn inner(&self) -> &S`

  Returns a reference the inner [`SourceCode`](../index.md) type for this

  `NamedSource`.

- <span id="namedsource-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

  Sets the [`language`](SpanContents::language) for this source code.

#### Trait Implementations

##### `impl Any for NamedSource<S>`

- <span id="namedsource-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NamedSource<S>`

- <span id="namedsource-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NamedSource<S>`

- <span id="namedsource-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<S: clone::Clone + SourceCode + 'static> Clone for NamedSource<S>`

- <span id="namedsource-clone"></span>`fn clone(&self) -> NamedSource<S>` — [`NamedSource`](../index.md#namedsource)

##### `impl CloneToUninit for NamedSource<S>`

- <span id="namedsource-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<S: SourceCode> Debug for NamedSource<S>`

- <span id="namedsource-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<S: cmp::Eq + SourceCode + 'static> Eq for NamedSource<S>`

##### `impl<T> From for NamedSource<S>`

- <span id="namedsource-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<S: hash::Hash + SourceCode + 'static> Hash for NamedSource<S>`

- <span id="namedsource-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for NamedSource<S>`

- <span id="namedsource-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<S: cmp::Ord + SourceCode + 'static> Ord for NamedSource<S>`

- <span id="namedsource-ord-cmp"></span>`fn cmp(&self, other: &NamedSource<S>) -> cmp::Ordering` — [`NamedSource`](../index.md#namedsource)

##### `impl OwoColorize for NamedSource<S>`

##### `impl<S: cmp::PartialEq + SourceCode + 'static> PartialEq for NamedSource<S>`

- <span id="namedsource-partialeq-eq"></span>`fn eq(&self, other: &NamedSource<S>) -> bool` — [`NamedSource`](../index.md#namedsource)

##### `impl<S: cmp::PartialOrd + SourceCode + 'static> PartialOrd for NamedSource<S>`

- <span id="namedsource-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NamedSource<S>) -> option::Option<cmp::Ordering>` — [`NamedSource`](../index.md#namedsource)

##### `impl<S: SourceCode + 'static> SourceCode for NamedSource<S>`

- <span id="namedsource-sourcecode-read-span"></span>`fn read_span<'a>(self: &'a Self, span: &crate::SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>` — [`SourceSpan`](../index.md#sourcespan), [`SpanContents`](../index.md#spancontents), [`MietteError`](../index.md#mietteerror)

##### `impl<S: SourceCode + 'static> StructuralPartialEq for NamedSource<S>`

##### `impl ToOwned for NamedSource<S>`

- <span id="namedsource-toowned-type-owned"></span>`type Owned = T`

- <span id="namedsource-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="namedsource-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NamedSource<S>`

- <span id="namedsource-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="namedsource-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NamedSource<S>`

- <span id="namedsource-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="namedsource-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

