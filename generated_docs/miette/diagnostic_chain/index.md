*[miette](../index.md) / [diagnostic_chain](index.md)*

---

# Module `diagnostic_chain`

Iterate over error `.diagnostic_source()` chains.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticChain`](#diagnosticchain) | struct | Iterator of a chain of cause errors. |
| [`ErrorKind`](#errorkind) | enum |  |

## Structs

### `DiagnosticChain<'a>`

```rust
struct DiagnosticChain<'a> {
    state: Option<ErrorKind<'a>>,
}
```

*Defined in [`miette-7.6.0/src/diagnostic_chain.rs:10-12`](../../../.source_1765521767/miette-7.6.0/src/diagnostic_chain.rs#L10-L12)*

Iterator of a chain of cause errors.

#### Implementations

- <span id="diagnosticchain-from-diagnostic"></span>`fn from_diagnostic(head: &'a dyn Diagnostic) -> Self` — [`Diagnostic`](../index.md#diagnostic)

- <span id="diagnosticchain-from-stderror"></span>`fn from_stderror(head: &'a dyn std::error::Error) -> Self`

#### Trait Implementations

##### `impl Any for DiagnosticChain<'a>`

- <span id="diagnosticchain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticChain<'a>`

- <span id="diagnosticchain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticChain<'a>`

- <span id="diagnosticchain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DiagnosticChain<'a>`

- <span id="diagnosticchain-clone"></span>`fn clone(&self) -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl CloneToUninit for DiagnosticChain<'a>`

- <span id="diagnosticchain-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for DiagnosticChain<'a>`

- <span id="diagnosticchain-default"></span>`fn default() -> DiagnosticChain<'a>` — [`DiagnosticChain`](#diagnosticchain)

##### `impl ExactSizeIterator for DiagnosticChain<'_>`

- <span id="diagnosticchain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for DiagnosticChain<'a>`

- <span id="diagnosticchain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticChain<'a>`

- <span id="diagnosticchain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for DiagnosticChain<'a>`

- <span id="diagnosticchain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="diagnosticchain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="diagnosticchain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for DiagnosticChain<'a>`

- <span id="diagnosticchain-iterator-type-item"></span>`type Item = ErrorKind<'a>`

- <span id="diagnosticchain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="diagnosticchain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl OwoColorize for DiagnosticChain<'a>`

##### `impl ToOwned for DiagnosticChain<'a>`

- <span id="diagnosticchain-toowned-type-owned"></span>`type Owned = T`

- <span id="diagnosticchain-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="diagnosticchain-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DiagnosticChain<'a>`

- <span id="diagnosticchain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticchain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticChain<'a>`

- <span id="diagnosticchain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticchain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ErrorKind<'a>`

```rust
enum ErrorKind<'a> {
    Diagnostic(&'a dyn Diagnostic),
    StdError(&'a dyn std::error::Error),
}
```

*Defined in [`miette-7.6.0/src/diagnostic_chain.rs:60-63`](../../../.source_1765521767/miette-7.6.0/src/diagnostic_chain.rs#L60-L63)*

#### Implementations

- <span id="errorkind-get-nested"></span>`fn get_nested(&self) -> Option<ErrorKind<'a>>` — [`ErrorKind`](#errorkind)

#### Trait Implementations

##### `impl Any for ErrorKind<'a>`

- <span id="errorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ErrorKind<'a>`

- <span id="errorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ErrorKind<'a>`

- <span id="errorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ErrorKind<'a>`

- <span id="errorkind-clone"></span>`fn clone(&self) -> ErrorKind<'a>` — [`ErrorKind`](#errorkind)

##### `impl CloneToUninit for ErrorKind<'a>`

- <span id="errorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ErrorKind<'_>`

- <span id="errorkind-debug-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Display for ErrorKind<'_>`

- <span id="errorkind-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<T> From for ErrorKind<'a>`

- <span id="errorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ErrorKind<'a>`

- <span id="errorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for ErrorKind<'a>`

##### `impl ToOwned for ErrorKind<'a>`

- <span id="errorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="errorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="errorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ErrorKind<'a>`

- <span id="errorkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ErrorKind<'a>`

- <span id="errorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="errorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ErrorKind<'a>`

- <span id="errorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="errorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

