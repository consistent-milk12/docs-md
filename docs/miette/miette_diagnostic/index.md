*[miette](../index.md) / [miette_diagnostic](index.md)*

---

# Module `miette_diagnostic`

## Structs

### `MietteDiagnostic`

```rust
struct MietteDiagnostic {
    pub message: String,
    pub code: Option<String>,
    pub severity: Option<crate::Severity>,
    pub help: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<crate::LabeledSpan>>,
}
```

Diagnostic that can be created at runtime.

#### Fields

- **`message`**: `String`

  Displayed diagnostic message

- **`code`**: `Option<String>`

  Unique diagnostic code to look up more information
  about this Diagnostic. Ideally also globally unique, and documented
  in the toplevel crate's documentation for easy searching.
  Rust path format (`foo::bar::baz`) is recommended, but more classic
  codes like `E0123` will work just fine

- **`severity`**: `Option<crate::Severity>`

  [`Diagnostic`](../index.md) severity. Intended to be used by
  [`ReportHandler`](crate::ReportHandler)s to change the way different
  [`Diagnostic`](../index.md)s are displayed. Defaults to [`Severity::Error`](../index.md)

- **`help`**: `Option<String>`

  Additional help text related to this Diagnostic

- **`url`**: `Option<String>`

  URL to visit for a more detailed explanation/help about this
  [`Diagnostic`](../index.md).

- **`labels`**: `Option<Vec<crate::LabeledSpan>>`

  Labels to apply to this `Diagnostic`'s `Diagnostic::source_code`

#### Implementations

- `fn new(message: impl Into<String>) -> Self`

- `fn with_code(self: Self, code: impl Into<String>) -> Self`

- `fn with_severity(self: Self, severity: Severity) -> Self` — [`Severity`](../index.md)

- `fn with_help(self: Self, help: impl Into<String>) -> Self`

- `fn with_url(self: Self, url: impl Into<String>) -> Self`

- `fn with_label(self: Self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- `fn with_labels(self: Self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- `fn and_label(self: Self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- `fn and_labels(self: Self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

#### Trait Implementations

##### `impl Clone for MietteDiagnostic`

- `fn clone(self: &Self) -> MietteDiagnostic` — [`MietteDiagnostic`](../index.md)

##### `impl Debug for MietteDiagnostic`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for MietteDiagnostic`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for MietteDiagnostic`

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn severity(self: &Self) -> Option<Severity>` — [`Severity`](../index.md)

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn labels(self: &Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../index.md)

##### `impl Display for MietteDiagnostic`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for MietteDiagnostic`

##### `impl Error for MietteDiagnostic`

##### `impl<D> OwoColorize for MietteDiagnostic`

##### `impl PartialEq for MietteDiagnostic`

- `fn eq(self: &Self, other: &MietteDiagnostic) -> bool` — [`MietteDiagnostic`](../index.md)

##### `impl StructuralPartialEq for MietteDiagnostic`

##### `impl<T> ToString for MietteDiagnostic`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for MietteDiagnostic`

