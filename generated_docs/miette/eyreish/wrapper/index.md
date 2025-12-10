*[miette](../../index.md) / [eyreish](../index.md) / [wrapper](index.md)*

---

# Module `wrapper`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DisplayError`](#displayerror) | struct |  |
| [`MessageError`](#messageerror) | struct |  |
| [`BoxedError`](#boxederror) | struct |  |
| [`WithSourceCode`](#withsourcecode) | struct |  |

## Structs

### `DisplayError<M>`

```rust
struct DisplayError<M>(M);
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:10`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/wrapper.rs#L10)*

#### Trait Implementations

##### `impl<M> Debug for DisplayError<M>`

- <span id="displayerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for DisplayError<M>`

- <span id="displayerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<M> Diagnostic for DisplayError<M>`

##### `impl<M> Display for DisplayError<M>`

- <span id="displayerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for DisplayError<M>`

##### `impl<D> OwoColorize for DisplayError<M>`

##### `impl<T> ToString for DisplayError<M>`

- <span id="displayerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for DisplayError<M>`

### `MessageError<M>`

```rust
struct MessageError<M>(M);
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:34`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/wrapper.rs#L34)*

#### Trait Implementations

##### `impl<M> Debug for MessageError<M>`

- <span id="messageerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for MessageError<M>`

- <span id="messageerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<M> Diagnostic for MessageError<M>`

##### `impl<M> Display for MessageError<M>`

- <span id="messageerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for MessageError<M>`

##### `impl<D> OwoColorize for MessageError<M>`

##### `impl<T> ToString for MessageError<M>`

- <span id="messageerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for MessageError<M>`

### `BoxedError`

```rust
struct BoxedError(Box<dyn Diagnostic + Send + Sync>);
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:58`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/wrapper.rs#L58)*

#### Trait Implementations

##### `impl Debug for BoxedError`

- <span id="boxederror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for BoxedError`

- <span id="boxederror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl Diagnostic for BoxedError`

- <span id="boxederror-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-severity"></span>`fn severity(&self) -> Option<miette::Severity>` — [`Severity`](../../index.md#severity)

- <span id="boxederror-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-labels"></span>`fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md#labeledspan)

- <span id="boxederror-source-code"></span>`fn source_code(&self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md#sourcecode)

- <span id="boxederror-related"></span>`fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md#diagnostic)

- <span id="boxederror-diagnostic-source"></span>`fn diagnostic_source(&self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl Display for BoxedError`

- <span id="boxederror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoxedError`

- <span id="boxederror-source"></span>`fn source(&self) -> Option<&dyn StdError>`

- <span id="boxederror-description"></span>`fn description(&self) -> &str`

- <span id="boxederror-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

##### `impl OwoColorize for BoxedError`

##### `impl ToString for BoxedError`

- <span id="boxederror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for BoxedError`

### `WithSourceCode<E, C>`

```rust
struct WithSourceCode<E, C> {
    error: E,
    source_code: C,
}
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:122-125`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/wrapper.rs#L122-L125)*

#### Trait Implementations

##### `impl<E: Debug, C> Debug for WithSourceCode<E, C>`

- <span id="withsourcecode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for WithSourceCode<E, C>`

- <span id="withsourcecode-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<E: Diagnostic, C: SourceCode> Diagnostic for WithSourceCode<E, C>`

- <span id="withsourcecode-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-severity"></span>`fn severity(&self) -> Option<miette::Severity>` — [`Severity`](../../index.md#severity)

- <span id="withsourcecode-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-labels"></span>`fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md#labeledspan)

- <span id="withsourcecode-source-code"></span>`fn source_code(&self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md#sourcecode)

- <span id="withsourcecode-related"></span>`fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md#diagnostic)

- <span id="withsourcecode-diagnostic-source"></span>`fn diagnostic_source(&self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl<E: Display, C> Display for WithSourceCode<E, C>`

- <span id="withsourcecode-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: StdError, C> Error for WithSourceCode<E, C>`

- <span id="withsourcecode-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<D> OwoColorize for WithSourceCode<E, C>`

##### `impl<T> ToString for WithSourceCode<E, C>`

- <span id="withsourcecode-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for WithSourceCode<E, C>`

