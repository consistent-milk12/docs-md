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

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:10`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/wrapper.rs#L10)*

#### Trait Implementations

##### `impl Any for DisplayError<M>`

- <span id="displayerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DisplayError<M>`

- <span id="displayerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DisplayError<M>`

- <span id="displayerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<M> Debug for DisplayError<M>`

- <span id="displayerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for DisplayError<M>`

- <span id="displayerror-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<M> Diagnostic for DisplayError<M>`

##### `impl<M> Display for DisplayError<M>`

- <span id="displayerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for DisplayError<M>`

##### `impl<T> From for DisplayError<M>`

- <span id="displayerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DisplayError<M>`

- <span id="displayerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for DisplayError<M>`

##### `impl ToString for DisplayError<M>`

- <span id="displayerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for DisplayError<M>`

##### `impl<U> TryFrom for DisplayError<M>`

- <span id="displayerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="displayerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DisplayError<M>`

- <span id="displayerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="displayerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MessageError<M>`

```rust
struct MessageError<M>(M);
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:34`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/wrapper.rs#L34)*

#### Trait Implementations

##### `impl Any for MessageError<M>`

- <span id="messageerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MessageError<M>`

- <span id="messageerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MessageError<M>`

- <span id="messageerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<M> Debug for MessageError<M>`

- <span id="messageerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for MessageError<M>`

- <span id="messageerror-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<M> Diagnostic for MessageError<M>`

##### `impl<M> Display for MessageError<M>`

- <span id="messageerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<M> Error for MessageError<M>`

##### `impl<T> From for MessageError<M>`

- <span id="messageerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MessageError<M>`

- <span id="messageerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MessageError<M>`

##### `impl ToString for MessageError<M>`

- <span id="messageerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for MessageError<M>`

##### `impl<U> TryFrom for MessageError<M>`

- <span id="messageerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="messageerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MessageError<M>`

- <span id="messageerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="messageerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoxedError`

```rust
struct BoxedError(Box<dyn Diagnostic + Send + Sync>);
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:58`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/wrapper.rs#L58)*

#### Trait Implementations

##### `impl Any for BoxedError`

- <span id="boxederror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoxedError`

- <span id="boxederror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoxedError`

- <span id="boxederror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BoxedError`

- <span id="boxederror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for BoxedError`

- <span id="boxederror-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl Diagnostic for BoxedError`

- <span id="boxederror-diagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-diagnostic-severity"></span>`fn severity(&self) -> Option<miette::Severity>` — [`Severity`](../../index.md#severity)

- <span id="boxederror-diagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-diagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="boxederror-diagnostic-labels"></span>`fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md#labeledspan)

- <span id="boxederror-diagnostic-source-code"></span>`fn source_code(&self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md#sourcecode)

- <span id="boxederror-diagnostic-related"></span>`fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md#diagnostic)

- <span id="boxederror-diagnostic-diagnostic-source"></span>`fn diagnostic_source(&self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl Display for BoxedError`

- <span id="boxederror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for BoxedError`

- <span id="boxederror-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

- <span id="boxederror-error-description"></span>`fn description(&self) -> &str`

- <span id="boxederror-error-cause"></span>`fn cause(&self) -> Option<&dyn StdError>`

##### `impl<T> From for BoxedError`

- <span id="boxederror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoxedError`

- <span id="boxederror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for BoxedError`

##### `impl ToString for BoxedError`

- <span id="boxederror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for BoxedError`

##### `impl<U> TryFrom for BoxedError`

- <span id="boxederror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boxederror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoxedError`

- <span id="boxederror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boxederror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithSourceCode<E, C>`

```rust
struct WithSourceCode<E, C> {
    error: E,
    source_code: C,
}
```

*Defined in [`miette-7.6.0/src/eyreish/wrapper.rs:122-125`](../../../../.source_1765521767/miette-7.6.0/src/eyreish/wrapper.rs#L122-L125)*

#### Trait Implementations

##### `impl Any for WithSourceCode<E, C>`

- <span id="withsourcecode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithSourceCode<E, C>`

- <span id="withsourcecode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithSourceCode<E, C>`

- <span id="withsourcecode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<E: Debug, C> Debug for WithSourceCode<E, C>`

- <span id="withsourcecode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for WithSourceCode<E, C>`

- <span id="withsourcecode-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md#report)

##### `impl<E: Diagnostic, C: SourceCode> Diagnostic for WithSourceCode<E, C>`

- <span id="withsourcecode-diagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-diagnostic-severity"></span>`fn severity(&self) -> Option<miette::Severity>` — [`Severity`](../../index.md#severity)

- <span id="withsourcecode-diagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-diagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="withsourcecode-diagnostic-labels"></span>`fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md#labeledspan)

- <span id="withsourcecode-diagnostic-source-code"></span>`fn source_code(&self) -> Option<&dyn miette::SourceCode>` — [`SourceCode`](../../index.md#sourcecode)

- <span id="withsourcecode-diagnostic-related"></span>`fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md#diagnostic)

- <span id="withsourcecode-diagnostic-diagnostic-source"></span>`fn diagnostic_source(&self) -> Option<&dyn Diagnostic>` — [`Diagnostic`](../../index.md#diagnostic)

##### `impl<E: Display, C> Display for WithSourceCode<E, C>`

- <span id="withsourcecode-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: StdError, C> Error for WithSourceCode<E, C>`

- <span id="withsourcecode-error-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<T> From for WithSourceCode<E, C>`

- <span id="withsourcecode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithSourceCode<E, C>`

- <span id="withsourcecode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for WithSourceCode<E, C>`

##### `impl ToString for WithSourceCode<E, C>`

- <span id="withsourcecode-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for WithSourceCode<E, C>`

##### `impl<U> TryFrom for WithSourceCode<E, C>`

- <span id="withsourcecode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withsourcecode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithSourceCode<E, C>`

- <span id="withsourcecode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withsourcecode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

