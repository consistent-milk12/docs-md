*[miette](../index.md) / [miette_diagnostic](index.md)*

---

# Module `miette_diagnostic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MietteDiagnostic`](#miettediagnostic) | struct | Diagnostic that can be created at runtime. |

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

- <span id="miettediagnostic-new"></span>`fn new(message: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-code"></span>`fn with_code(self, code: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-severity"></span>`fn with_severity(self, severity: Severity) -> Self` — [`Severity`](../index.md)

- <span id="miettediagnostic-with-help"></span>`fn with_help(self, help: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-url"></span>`fn with_url(self, url: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-label"></span>`fn with_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- <span id="miettediagnostic-with-labels"></span>`fn with_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- <span id="miettediagnostic-and-label"></span>`fn and_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

- <span id="miettediagnostic-and-labels"></span>`fn and_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md)

#### Trait Implementations

##### `impl Clone for MietteDiagnostic`

- <span id="miettediagnostic-clone"></span>`fn clone(&self) -> MietteDiagnostic` — [`MietteDiagnostic`](../index.md)

##### `impl Debug for MietteDiagnostic`

- <span id="miettediagnostic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for MietteDiagnostic`

- <span id="miettediagnostic-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for MietteDiagnostic`

- <span id="miettediagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-severity"></span>`fn severity(&self) -> Option<Severity>` — [`Severity`](../index.md)

- <span id="miettediagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-labels"></span>`fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../index.md)

##### `impl Display for MietteDiagnostic`

- <span id="miettediagnostic-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for MietteDiagnostic`

##### `impl Error for MietteDiagnostic`

##### `impl<D> OwoColorize for MietteDiagnostic`

##### `impl PartialEq for MietteDiagnostic`

- <span id="miettediagnostic-eq"></span>`fn eq(&self, other: &MietteDiagnostic) -> bool` — [`MietteDiagnostic`](../index.md)

##### `impl StructuralPartialEq for MietteDiagnostic`

##### `impl<T> ToString for MietteDiagnostic`

- <span id="miettediagnostic-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for MietteDiagnostic`

