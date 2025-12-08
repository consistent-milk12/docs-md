*[miette](../../index.md) / [eyreish](../index.md) / [wrapper](index.md)*

---

# Module `wrapper`

## Structs

### `DisplayError<M>`

```rust
struct DisplayError<M>(M);
```

#### Trait Implementations

##### `impl<M> Debug for DisplayError<M>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for DisplayError<M>`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl<M> Diagnostic for DisplayError<M>`

##### `impl<M> Display for DisplayError<M>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for DisplayError<M>`

##### `impl<D> OwoColorize for DisplayError<M>`

##### `impl<T> ToString for DisplayError<M>`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for DisplayError<M>`

### `MessageError<M>`

```rust
struct MessageError<M>(M);
```

#### Trait Implementations

##### `impl<M> Debug for MessageError<M>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for MessageError<M>`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl<M> Diagnostic for MessageError<M>`

##### `impl<M> Display for MessageError<M>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for MessageError<M>`

##### `impl<D> OwoColorize for MessageError<M>`

##### `impl<T> ToString for MessageError<M>`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for MessageError<M>`

### `BoxedError`

```rust
struct BoxedError(Box<dyn Diagnostic + Send + Sync>);
```

#### Trait Implementations

##### `impl Debug for BoxedError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for BoxedError`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl Diagnostic for BoxedError`

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn severity(self: &Self) -> Option<miette::Severity>` — [`Severity`](../../index.md)

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md)

- `fn source_code(self: &Self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md)

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md)

- `fn diagnostic_source(self: &Self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md)

##### `impl Display for BoxedError`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoxedError`

- `fn source(self: &Self) -> Option<&dyn StdError>`

- `fn description(self: &Self) -> &str`

- `fn cause(self: &Self) -> Option<&dyn StdError>`

##### `impl<D> OwoColorize for BoxedError`

##### `impl<T> ToString for BoxedError`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for BoxedError`

### `WithSourceCode<E, C>`

```rust
struct WithSourceCode<E, C> {
    error: E,
    source_code: C,
}
```

#### Trait Implementations

##### `impl<E: Debug, C> Debug for WithSourceCode<E, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for WithSourceCode<E, C>`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl<E: Diagnostic, C: SourceCode> Diagnostic for WithSourceCode<E, C>`

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn severity(self: &Self) -> Option<miette::Severity>` — [`Severity`](../../index.md)

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md)

- `fn source_code(self: &Self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md)

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md)

- `fn diagnostic_source(self: &Self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md)

##### `impl<E: Display, C> Display for WithSourceCode<E, C>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: StdError, C> Error for WithSourceCode<E, C>`

- `fn source(self: &Self) -> Option<&dyn StdError>`

##### `impl<D> OwoColorize for WithSourceCode<E, C>`

##### `impl<T> ToString for WithSourceCode<E, C>`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for WithSourceCode<E, C>`

