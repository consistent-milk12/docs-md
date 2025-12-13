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

*Defined in [`miette-7.6.0/src/miette_diagnostic.rs:14-39`](../../../.source_1765633015/miette-7.6.0/src/miette_diagnostic.rs#L14-L39)*

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

  Create a new dynamic diagnostic with the given message.

  

  # Examples

  ```rust

  use miette::{Diagnostic, MietteDiagnostic, Severity};

  

  let diag = MietteDiagnostic::new("Oops, something went wrong!");

  assert_eq!(diag.to_string(), "Oops, something went wrong!");

  assert_eq!(diag.message, "Oops, something went wrong!");

  ```

- <span id="miettediagnostic-with-code"></span>`fn with_code(self, code: impl Into<String>) -> Self`

  Return new diagnostic with the given code.

  

  # Examples

  ```rust

  use miette::{Diagnostic, MietteDiagnostic};

  

  let diag = MietteDiagnostic::new("Oops, something went wrong!").with_code("foo::bar::baz");

  assert_eq!(diag.message, "Oops, something went wrong!");

  assert_eq!(diag.code, Some("foo::bar::baz".to_string()));

  ```

- <span id="miettediagnostic-with-severity"></span>`fn with_severity(self, severity: Severity) -> Self` — [`Severity`](../index.md#severity)

  Return new diagnostic with the given severity.

  

  # Examples

  ```rust

  use miette::{Diagnostic, MietteDiagnostic, Severity};

  

  let diag = MietteDiagnostic::new("I warn you to stop!").with_severity(Severity::Warning);

  assert_eq!(diag.message, "I warn you to stop!");

  assert_eq!(diag.severity, Some(Severity::Warning));

  ```

- <span id="miettediagnostic-with-help"></span>`fn with_help(self, help: impl Into<String>) -> Self`

  Return new diagnostic with the given help message.

  

  # Examples

  ```rust

  use miette::{Diagnostic, MietteDiagnostic};

  

  let diag = MietteDiagnostic::new("PC is not working").with_help("Try to reboot it again");

  assert_eq!(diag.message, "PC is not working");

  assert_eq!(diag.help, Some("Try to reboot it again".to_string()));

  ```

- <span id="miettediagnostic-with-url"></span>`fn with_url(self, url: impl Into<String>) -> Self`

  Return new diagnostic with the given URL.

  

  # Examples

  ```rust

  use miette::{Diagnostic, MietteDiagnostic};

  

  let diag = MietteDiagnostic::new("PC is not working")

      .with_url("https://letmegooglethat.com/?q=Why+my+pc+doesn%27t+work");

  assert_eq!(diag.message, "PC is not working");

  assert_eq!(

      diag.url,

      Some("https://letmegooglethat.com/?q=Why+my+pc+doesn%27t+work".to_string())

  );

  ```

- <span id="miettediagnostic-with-label"></span>`fn with_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md#labeledspan)

  Return new diagnostic with the given label.

  

  Discards previous labels

  

  # Examples

  ```rust

  use miette::{Diagnostic, LabeledSpan, MietteDiagnostic};

  

  let source = "cpp is the best language";

  

  let label = LabeledSpan::at(0..3, "This should be Rust");

  let diag = MietteDiagnostic::new("Wrong best language").with_label(label.clone());

  assert_eq!(diag.message, "Wrong best language");

  assert_eq!(diag.labels, Some(vec![label]));

  ```

- <span id="miettediagnostic-with-labels"></span>`fn with_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md#labeledspan)

  Return new diagnostic with the given labels.

  

  Discards previous labels

  

  # Examples

  ```rust

  use miette::{Diagnostic, LabeledSpan, MietteDiagnostic};

  

  let source = "helo wrld";

  

  let labels = vec![

      LabeledSpan::at_offset(3, "add 'l'"),

      LabeledSpan::at_offset(6, "add 'r'"),

  ];

  let diag = MietteDiagnostic::new("Typos in 'hello world'").with_labels(labels.clone());

  assert_eq!(diag.message, "Typos in 'hello world'");

  assert_eq!(diag.labels, Some(labels));

  ```

- <span id="miettediagnostic-and-label"></span>`fn and_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md#labeledspan)

  Return new diagnostic with new label added to the existing ones.

  

  # Examples

  ```rust

  use miette::{Diagnostic, LabeledSpan, MietteDiagnostic};

  

  let source = "helo wrld";

  

  let label1 = LabeledSpan::at_offset(3, "add 'l'");

  let label2 = LabeledSpan::at_offset(6, "add 'r'");

  let diag = MietteDiagnostic::new("Typos in 'hello world'")

      .and_label(label1.clone())

      .and_label(label2.clone());

  assert_eq!(diag.message, "Typos in 'hello world'");

  assert_eq!(diag.labels, Some(vec![label1, label2]));

  ```

- <span id="miettediagnostic-and-labels"></span>`fn and_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](../index.md#labeledspan)

  Return new diagnostic with new labels added to the existing ones.

  

  # Examples

  ```rust

  use miette::{Diagnostic, LabeledSpan, MietteDiagnostic};

  

  let source = "helo wrld";

  

  let label1 = LabeledSpan::at_offset(3, "add 'l'");

  let label2 = LabeledSpan::at_offset(6, "add 'r'");

  let label3 = LabeledSpan::at_offset(9, "add '!'");

  let diag = MietteDiagnostic::new("Typos in 'hello world!'")

      .and_label(label1.clone())

      .and_labels([label2.clone(), label3.clone()]);

  assert_eq!(diag.message, "Typos in 'hello world!'");

  assert_eq!(diag.labels, Some(vec![label1, label2, label3]));

  ```

#### Trait Implementations

##### `impl Any for MietteDiagnostic`

- <span id="miettediagnostic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MietteDiagnostic`

- <span id="miettediagnostic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MietteDiagnostic`

- <span id="miettediagnostic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MietteDiagnostic`

- <span id="miettediagnostic-clone"></span>`fn clone(&self) -> MietteDiagnostic` — [`MietteDiagnostic`](../index.md#miettediagnostic)

##### `impl CloneToUninit for MietteDiagnostic`

- <span id="miettediagnostic-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MietteDiagnostic`

- <span id="miettediagnostic-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for MietteDiagnostic`

- <span id="miettediagnostic-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md#report)

##### `impl Diagnostic for MietteDiagnostic`

- <span id="miettediagnostic-diagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-diagnostic-severity"></span>`fn severity(&self) -> Option<Severity>` — [`Severity`](../index.md#severity)

- <span id="miettediagnostic-diagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-diagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-diagnostic-labels"></span>`fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../index.md#labeledspan)

##### `impl Display for MietteDiagnostic`

- <span id="miettediagnostic-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for MietteDiagnostic`

##### `impl Error for MietteDiagnostic`

##### `impl<T> From for MietteDiagnostic`

- <span id="miettediagnostic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MietteDiagnostic`

- <span id="miettediagnostic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MietteDiagnostic`

##### `impl PartialEq for MietteDiagnostic`

- <span id="miettediagnostic-partialeq-eq"></span>`fn eq(&self, other: &MietteDiagnostic) -> bool` — [`MietteDiagnostic`](../index.md#miettediagnostic)

##### `impl StructuralPartialEq for MietteDiagnostic`

##### `impl ToOwned for MietteDiagnostic`

- <span id="miettediagnostic-toowned-type-owned"></span>`type Owned = T`

- <span id="miettediagnostic-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="miettediagnostic-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for MietteDiagnostic`

- <span id="miettediagnostic-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for MietteDiagnostic`

##### `impl<U> TryFrom for MietteDiagnostic`

- <span id="miettediagnostic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="miettediagnostic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MietteDiagnostic`

- <span id="miettediagnostic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="miettediagnostic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

