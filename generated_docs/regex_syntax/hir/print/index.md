*[regex_syntax](../../index.md) / [hir](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Hir`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PrinterBuilder`](#printerbuilder) | struct | A builder for constructing a printer. |
| [`Printer`](#printer) | struct | A printer for a regular expression's high-level intermediate representation. |
| [`Writer`](#writer) | struct |  |

## Structs

### `PrinterBuilder`

```rust
struct PrinterBuilder {
    _priv: (),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:21-23`](../../../../.source_1765633015/regex-syntax-0.8.8/src/hir/print.rs#L21-L23)*

A builder for constructing a printer.

Note that since a printer doesn't have any configuration knobs, this type
remains unexported.

#### Implementations

- <span id="printerbuilder-new"></span>`fn new() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

- <span id="printerbuilder-build"></span>`fn build(&self) -> Printer` — [`Printer`](#printer)

#### Trait Implementations

##### `impl Any for PrinterBuilder`

- <span id="printerbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrinterBuilder`

- <span id="printerbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrinterBuilder`

- <span id="printerbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PrinterBuilder`

- <span id="printerbuilder-clone"></span>`fn clone(&self) -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

##### `impl CloneToUninit for PrinterBuilder`

- <span id="printerbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PrinterBuilder`

- <span id="printerbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PrinterBuilder`

- <span id="printerbuilder-default"></span>`fn default() -> PrinterBuilder` — [`PrinterBuilder`](#printerbuilder)

##### `impl<T> From for PrinterBuilder`

- <span id="printerbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrinterBuilder`

- <span id="printerbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PrinterBuilder`

- <span id="printerbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="printerbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="printerbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PrinterBuilder`

- <span id="printerbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="printerbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrinterBuilder`

- <span id="printerbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="printerbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Printer`

```rust
struct Printer {
    _priv: (),
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:58-60`](../../../../.source_1765633015/regex-syntax-0.8.8/src/hir/print.rs#L58-L60)*

A printer for a regular expression's high-level intermediate
representation.

A printer converts a high-level intermediate representation (HIR) to a
regular expression pattern string. This particular printer uses constant
stack space and heap space proportional to the size of the HIR.

Since this printer is only using the HIR, the pattern it prints will likely
not resemble the original pattern at all. For example, a pattern like
`\pL` will have its entire class written out.

The purpose of this printer is to provide a means to mutate an HIR and then
build a regular expression from the result of that mutation. (A regex
library could provide a constructor from this HIR explicitly, but that
creates an unnecessary public coupling between the regex library and this
specific HIR representation.)

#### Implementations

- <span id="printer-new"></span>`fn new() -> Printer` — [`Printer`](#printer)

  Create a new printer.

- <span id="printer-print"></span>`fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result` — [`Hir`](../index.md#hir)

  Print the given `Ast` to the given writer. The writer must implement

  `fmt::Write`. Typical implementations of `fmt::Write` that can be used

  here are a `fmt::Formatter` (which is available in `fmt::Display`

  implementations) or a `&mut String`.

#### Trait Implementations

##### `impl Any for Printer`

- <span id="printer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Printer`

- <span id="printer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Printer`

- <span id="printer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Printer`

- <span id="printer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Printer`

- <span id="printer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Printer`

- <span id="printer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Printer`

- <span id="printer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="printer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Printer`

- <span id="printer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="printer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Writer<W>`

```rust
struct Writer<W> {
    wtr: W,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/print.rs:78-80`](../../../../.source_1765633015/regex-syntax-0.8.8/src/hir/print.rs#L78-L80)*

#### Implementations

- <span id="writer-write-literal-char"></span>`fn write_literal_char(&mut self, c: char) -> fmt::Result`

- <span id="writer-write-literal-byte"></span>`fn write_literal_byte(&mut self, b: u8) -> fmt::Result`

- <span id="writer-write-literal-class-byte"></span>`fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Writer<W>`

- <span id="writer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Writer<W>`

- <span id="writer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Writer<W>`

- <span id="writer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<W: fmt::Debug> Debug for Writer<W>`

- <span id="writer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Writer<W>`

- <span id="writer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Writer<W>`

- <span id="writer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Writer<W>`

- <span id="writer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="writer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Writer<W>`

- <span id="writer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="writer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<W: fmt::Write> Visitor for Writer<W>`

- <span id="writer-visitor-type-output"></span>`type Output = ()`

- <span id="writer-visitor-type-err"></span>`type Err = Error`

- <span id="writer-visitor-finish"></span>`fn finish(self) -> fmt::Result`

- <span id="writer-visitor-visit-pre"></span>`fn visit_pre(&mut self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md#hir)

- <span id="writer-visitor-visit-post"></span>`fn visit_post(&mut self, hir: &Hir) -> fmt::Result` — [`Hir`](../index.md#hir)

- <span id="writer-visitor-visit-alternation-in"></span>`fn visit_alternation_in(&mut self) -> fmt::Result`

