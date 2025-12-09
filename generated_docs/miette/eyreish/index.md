*[miette](../index.md) / [eyreish](index.md)*

---

# Module `eyreish`

## Contents

- [Modules](#modules)
  - [`context`](#context)
  - [`error`](#error)
  - [`fmt`](#fmt)
  - [`into_diagnostic`](#into_diagnostic)
  - [`kind`](#kind)
  - [`macros`](#macros)
  - [`ptr`](#ptr)
  - [`wrapper`](#wrapper)
- [Structs](#structs)
  - [`Error`](#error)
  - [`Report`](#report)
  - [`InstallError`](#installerror)
  - [`DiagnosticError`](#diagnosticerror)
- [Traits](#traits)
  - [`Context`](#context)
  - [`ReportHandler`](#reporthandler)
  - [`WrapErr`](#wraperr)
  - [`IntoDiagnostic`](#intodiagnostic)
- [Functions](#functions)
  - [`set_hook`](#set_hook)
  - [`capture_handler`](#capture_handler)
  - [`get_default_printer`](#get_default_printer)
- [Type Aliases](#type-aliases)
  - [`ErrorHook`](#errorhook)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`context`](#context) | mod |  |
| [`error`](#error) | mod |  |
| [`fmt`](#fmt) | mod |  |
| [`into_diagnostic`](#into_diagnostic) | mod |  |
| [`kind`](#kind) | mod |  |
| [`macros`](#macros) | mod |  |
| [`ptr`](#ptr) | mod |  |
| [`wrapper`](#wrapper) | mod |  |
| [`Error`](#error) | struct | Compatibility re-export of `Report` for interop with `anyhow` |
| [`Report`](#report) | struct | Core Diagnostic wrapper type. |
| [`InstallError`](#installerror) | struct | Error indicating that [`set_hook()`] was unable to install the provided [`ErrorHook`]. |
| [`DiagnosticError`](#diagnosticerror) | struct | Convenience [`Diagnostic`] that can be used as an "anonymous" wrapper for Errors. |
| [`Context`](#context) | trait | Compatibility re-export of `WrapErr` for interop with `anyhow` |
| [`ReportHandler`](#reporthandler) | trait | Error Report Handler trait for customizing `miette::Report` |
| [`WrapErr`](#wraperr) | trait | Provides the [`wrap_err()`](WrapErr::wrap_err) method for [`Result`]. |
| [`IntoDiagnostic`](#intodiagnostic) | trait | Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing [`std::error::Error`] to a [`Result<T, Report>`]. |
| [`set_hook`](#set_hook) | fn | Set the error hook. |
| [`capture_handler`](#capture_handler) | fn |  |
| [`get_default_printer`](#get_default_printer) | fn |  |
| [`ErrorHook`](#errorhook) | type |  |
| [`Result`](#result) | type | type alias for `Result<T, Report>` |

## Modules

- [`context`](context/index.md)
- [`error`](error/index.md)
- [`fmt`](fmt/index.md)
- [`into_diagnostic`](into_diagnostic/index.md)
- [`kind`](kind/index.md)
- [`macros`](macros/index.md)
- [`ptr`](ptr/index.md)
- [`wrapper`](wrapper/index.md)

## Structs

### `Error`

```rust
struct Error {
    inner: self::ptr::Own<error::ErrorImpl<()>>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:53-55`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L53-L55)*

Core Diagnostic wrapper type.

## `eyre` Users

You can just replace `use`s of `eyre::Report` with `miette::Report`.

#### Implementations

- <span id="superreport-new"></span>`fn new<E>(error: E) -> Self`

- <span id="superreport-msg"></span>`fn msg<M>(message: M) -> Self`

- <span id="superreport-new-boxed"></span>`fn new_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- <span id="superreport-from-std"></span>`fn from_std<E>(error: E) -> Self`

- <span id="superreport-from-adhoc"></span>`fn from_adhoc<M>(message: M) -> Self`

- <span id="superreport-from-msg"></span>`fn from_msg<D, E>(msg: D, error: E) -> Self`

- <span id="superreport-from-boxed"></span>`fn from_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- <span id="superreport-construct"></span>`unsafe fn construct<E>(error: E, vtable: &'static ErrorVTable, handler: Option<Box<dyn ReportHandler>>) -> Self` — [`ErrorVTable`](error/index.md), [`ReportHandler`](../index.md)

- <span id="superreport-wrap-err"></span>`fn wrap_err<D>(self, msg: D) -> Self`

- <span id="superreport-context"></span>`fn context<D>(self, msg: D) -> Self`

- <span id="superreport-chain"></span>`fn chain(&self) -> Chain<'_>` — [`Chain`](../chain/index.md)

- <span id="superreport-root-cause"></span>`fn root_cause(&self) -> &dyn StdError`

- <span id="superreport-is"></span>`fn is<E>(&self) -> bool`

- <span id="superreport-downcast"></span>`fn downcast<E>(self) -> Result<E, Self>`

- <span id="superreport-downcast-ref"></span>`fn downcast_ref<E>(&self) -> Option<&E>`

- <span id="superreport-downcast-mut"></span>`fn downcast_mut<E>(&mut self) -> Option<&mut E>`

- <span id="superreport-handler"></span>`fn handler(&self) -> &dyn ReportHandler` — [`ReportHandler`](../index.md)

- <span id="superreport-handler-mut"></span>`fn handler_mut(&mut self) -> &mut dyn ReportHandler` — [`ReportHandler`](../index.md)

- <span id="superreport-with-source-code"></span>`fn with_source_code(self, source_code: impl SourceCode + 'static) -> Report` — [`SourceCode`](../index.md), [`Report`](../index.md)

- <span id="superreport-from-err"></span>`fn from_err<E>(err: E) -> Self`

#### Trait Implementations

##### `impl AsRef for super::Report`

- <span id="superreport-as-ref"></span>`fn as_ref(&self) -> &dyn Diagnostic + Send + Sync` — [`Diagnostic`](../index.md)

##### `impl Debug for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for super::Report`

- <span id="superreport-type-target"></span>`type Target = dyn Diagnostic + Send + Sync`

- <span id="superreport-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for super::Report`

- <span id="superreport-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Diag for super::Report`

- <span id="superreport-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Display for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for super::Report`

- <span id="superreport-drop"></span>`fn drop(&mut self)`

##### `impl OwoColorize for Report`

##### `impl Receiver for Report`

- <span id="report-type-target"></span>`type Target = T`

##### `impl Send for Report`

##### `impl Sync for Report`

##### `impl ToString for Report`

- <span id="report-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Report`

### `Report`

```rust
struct Report {
    inner: self::ptr::Own<error::ErrorImpl<()>>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:53-55`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L53-L55)*

Core Diagnostic wrapper type.

## `eyre` Users

You can just replace `use`s of `eyre::Report` with `miette::Report`.

#### Implementations

- <span id="superreport-new"></span>`fn new<E>(error: E) -> Self`

- <span id="superreport-msg"></span>`fn msg<M>(message: M) -> Self`

- <span id="superreport-new-boxed"></span>`fn new_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- <span id="superreport-from-std"></span>`fn from_std<E>(error: E) -> Self`

- <span id="superreport-from-adhoc"></span>`fn from_adhoc<M>(message: M) -> Self`

- <span id="superreport-from-msg"></span>`fn from_msg<D, E>(msg: D, error: E) -> Self`

- <span id="superreport-from-boxed"></span>`fn from_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- <span id="superreport-construct"></span>`unsafe fn construct<E>(error: E, vtable: &'static ErrorVTable, handler: Option<Box<dyn ReportHandler>>) -> Self` — [`ErrorVTable`](error/index.md), [`ReportHandler`](../index.md)

- <span id="superreport-wrap-err"></span>`fn wrap_err<D>(self, msg: D) -> Self`

- <span id="superreport-context"></span>`fn context<D>(self, msg: D) -> Self`

- <span id="superreport-chain"></span>`fn chain(&self) -> Chain<'_>` — [`Chain`](../chain/index.md)

- <span id="superreport-root-cause"></span>`fn root_cause(&self) -> &dyn StdError`

- <span id="superreport-is"></span>`fn is<E>(&self) -> bool`

- <span id="superreport-downcast"></span>`fn downcast<E>(self) -> Result<E, Self>`

- <span id="superreport-downcast-ref"></span>`fn downcast_ref<E>(&self) -> Option<&E>`

- <span id="superreport-downcast-mut"></span>`fn downcast_mut<E>(&mut self) -> Option<&mut E>`

- <span id="superreport-handler"></span>`fn handler(&self) -> &dyn ReportHandler` — [`ReportHandler`](../index.md)

- <span id="superreport-handler-mut"></span>`fn handler_mut(&mut self) -> &mut dyn ReportHandler` — [`ReportHandler`](../index.md)

- <span id="superreport-with-source-code"></span>`fn with_source_code(self, source_code: impl SourceCode + 'static) -> Report` — [`SourceCode`](../index.md), [`Report`](../index.md)

- <span id="superreport-from-err"></span>`fn from_err<E>(err: E) -> Self`

#### Trait Implementations

##### `impl AsRef for super::Report`

- <span id="superreport-as-ref"></span>`fn as_ref(&self) -> &dyn Diagnostic + Send + Sync` — [`Diagnostic`](../index.md)

##### `impl Debug for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for super::Report`

- <span id="superreport-type-target"></span>`type Target = dyn Diagnostic + Send + Sync`

- <span id="superreport-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for super::Report`

- <span id="superreport-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Diag for super::Report`

- <span id="superreport-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Display for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for super::Report`

- <span id="superreport-drop"></span>`fn drop(&mut self)`

##### `impl OwoColorize for Report`

##### `impl Receiver for Report`

- <span id="report-type-target"></span>`type Target = T`

##### `impl Send for Report`

##### `impl Sync for Report`

##### `impl ToString for Report`

- <span id="report-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Report`

### `InstallError`

```rust
struct InstallError;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:69`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L69)*

Error indicating that [`set_hook()`](../index.md) was unable to install the provided
[`ErrorHook`](../index.md).

#### Trait Implementations

##### `impl Debug for InstallError`

- <span id="installerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for InstallError`

- <span id="installerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for InstallError`

##### `impl Display for InstallError`

- <span id="installerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for InstallError`

##### `impl OwoColorize for InstallError`

##### `impl ToString for InstallError`

- <span id="installerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for InstallError`

### `DiagnosticError`

```rust
struct DiagnosticError(Box<dyn std::error::Error + Send + Sync>);
```

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:8`](../../../.source_1765210505/miette-7.6.0/src/eyreish/into_diagnostic.rs#L8)*

Convenience [`Diagnostic`](../index.md) that can be used as an "anonymous" wrapper for
Errors. This is intended to be paired with [`IntoDiagnostic`](#intodiagnostic).

#### Trait Implementations

##### `impl Debug for DiagnosticError`

- <span id="diagnosticerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for DiagnosticError`

- <span id="diagnosticerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for DiagnosticError`

##### `impl Display for DiagnosticError`

- <span id="diagnosticerror-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for DiagnosticError`

- <span id="diagnosticerror-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl OwoColorize for DiagnosticError`

##### `impl ToString for DiagnosticError`

- <span id="diagnosticerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for DiagnosticError`

## Traits

### `Context<T, E>`

```rust
trait Context<T, E>: context::private::Sealed { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:433-460`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L433-L460)*

Provides the [`wrap_err()`](WrapErr::wrap_err) method for [`Result`](../index.md).

This trait is sealed and cannot be implemented for types outside of
`miette`.

# Example

```rust
use miette::{WrapErr, IntoDiagnostic, Result};
use std::{fs, path::PathBuf};

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    const IGNORE: &'static str = stringify! {
    pub fn detach(&mut self) -> Result<()> {...}
    };
    fn detach(&mut self) -> Result<()> {
        unimplemented!()
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().wrap_err("Failed to detach the important thing")?;

    let path = &it.path;
    let content = fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!(
            "Failed to read instrs from {}",
            path.display())
        )?;

    Ok(content)
}
```

When printed, the outermost error would be printed first and the lower
level underlying causes would be enumerated below.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

# Wrapping Types That Do Not Implement `Error`

For example `&str` and `Box<dyn Error>`.

Due to restrictions for coherence `Report` cannot implement `From` for types
that don't implement `Error`. Attempts to do so will give `"this type might
implement Error in the future"` as an error. As such, `wrap_err()`, which
uses `From` under the hood, cannot be used to wrap these types. Instead we
encourage you to use the combinators provided for `Result` in `std`/`core`.

For example, instead of this:

```rust,compile_fail
use std::error::Error;
use miette::{WrapErr, Report};

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>)
    -> Result<(), Report>
{
    err.wrap_err("saw a downstream error")
}
```

We encourage you to write this:

```rust
use miette::{miette, Report, WrapErr};
use std::error::Error;

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>) -> Result<(), Report> {
    err.map_err(|e| miette!(e))
        .wrap_err("saw a downstream error")
}
```

# Effect on Downcasting

After attaching a message of type `D` onto an error of type `E`, the
resulting `miette::Error` may be downcast to `D` **or** to `E`.

That is, in codebases that rely on downcasting, `miette`'s `wrap_err()`
supports both of the following use cases:

  - **Attaching messages whose type is insignificant onto errors whose type
    is used in downcasts.**

    In other error libraries whose `wrap_err()` is not designed this way, it
    can be risky to introduce messages to existing code because new message
    might break existing working downcasts. In miette, any downcast that
    worked before adding the message will continue to work after you add a
    message, so you should freely wrap errors wherever it would be helpful.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct SuspiciousError;

    fn helper() -> Result<()> {
        bail!(SuspiciousError);
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err("Failed to complete the work")?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<SuspiciousError>() {
            // If helper() returned SuspiciousError, this downcast will
            // correctly succeed even with the message in between.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

  - **Attaching message whose type is used in downcasts onto errors whose
    type is insignificant.**

    Some codebases prefer to use machine-readable messages to categorize
    lower level errors in a way that will be actionable to higher levels of
    the application.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct HelperFailed;

    fn helper() -> Result<()> {
        bail!("no such file or directory");
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err(HelperFailed)?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<HelperFailed>() {
            // If helper failed, this downcast will succeed because
            // HelperFailed is the message that has been attached to
            // that error.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

#### Required Methods

- `fn wrap_err<D>(self, msg: D) -> Result<T, Report>`

  Wrap the error value with a new adhoc error

- `fn wrap_err_with<D, F>(self, f: F) -> Result<T, Report>`

  Wrap the error value with a new adhoc error that is evaluated lazily

- `fn context<D>(self, msg: D) -> Result<T, Report>`

  Compatibility re-export of `wrap_err()` for interop with `anyhow`

- `fn with_context<D, F>(self, f: F) -> Result<T, Report>`

  Compatibility re-export of `wrap_err_with()` for interop with `anyhow`

#### Implementors

- `Option<T>`
- `Result<T, E>`

### `ReportHandler`

```rust
trait ReportHandler: core::any::Any + Send + Sync { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:144-201`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L144-L201)*

Error Report Handler trait for customizing `miette::Report`

#### Required Methods

- `fn debug(&self, error: &dyn Diagnostic, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Define the report format

#### Provided Methods

- `fn display(&self, error: &dyn StdError, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Override for the `Display` format

- `fn track_caller(&mut self, location: &'static std::panic::Location<'static>)`

  Store the location of the caller who constructed this error report

#### Implementors

- [`DebugReportHandler`](../handlers/index.md)
- [`GraphicalReportHandler`](../handlers/index.md)
- [`JSONReportHandler`](../handlers/index.md)
- [`MietteHandler`](../index.md)
- [`NarratableReportHandler`](../handlers/index.md)

### `WrapErr<T, E>`

```rust
trait WrapErr<T, E>: context::private::Sealed { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:433-460`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L433-L460)*

Provides the [`wrap_err()`](WrapErr::wrap_err) method for [`Result`](../index.md).

This trait is sealed and cannot be implemented for types outside of
`miette`.

# Example

```rust
use miette::{WrapErr, IntoDiagnostic, Result};
use std::{fs, path::PathBuf};

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    const IGNORE: &'static str = stringify! {
    pub fn detach(&mut self) -> Result<()> {...}
    };
    fn detach(&mut self) -> Result<()> {
        unimplemented!()
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().wrap_err("Failed to detach the important thing")?;

    let path = &it.path;
    let content = fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!(
            "Failed to read instrs from {}",
            path.display())
        )?;

    Ok(content)
}
```

When printed, the outermost error would be printed first and the lower
level underlying causes would be enumerated below.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

# Wrapping Types That Do Not Implement `Error`

For example `&str` and `Box<dyn Error>`.

Due to restrictions for coherence `Report` cannot implement `From` for types
that don't implement `Error`. Attempts to do so will give `"this type might
implement Error in the future"` as an error. As such, `wrap_err()`, which
uses `From` under the hood, cannot be used to wrap these types. Instead we
encourage you to use the combinators provided for `Result` in `std`/`core`.

For example, instead of this:

```rust,compile_fail
use std::error::Error;
use miette::{WrapErr, Report};

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>)
    -> Result<(), Report>
{
    err.wrap_err("saw a downstream error")
}
```

We encourage you to write this:

```rust
use miette::{miette, Report, WrapErr};
use std::error::Error;

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>) -> Result<(), Report> {
    err.map_err(|e| miette!(e))
        .wrap_err("saw a downstream error")
}
```

# Effect on Downcasting

After attaching a message of type `D` onto an error of type `E`, the
resulting `miette::Error` may be downcast to `D` **or** to `E`.

That is, in codebases that rely on downcasting, `miette`'s `wrap_err()`
supports both of the following use cases:

  - **Attaching messages whose type is insignificant onto errors whose type
    is used in downcasts.**

    In other error libraries whose `wrap_err()` is not designed this way, it
    can be risky to introduce messages to existing code because new message
    might break existing working downcasts. In miette, any downcast that
    worked before adding the message will continue to work after you add a
    message, so you should freely wrap errors wherever it would be helpful.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct SuspiciousError;

    fn helper() -> Result<()> {
        bail!(SuspiciousError);
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err("Failed to complete the work")?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<SuspiciousError>() {
            // If helper() returned SuspiciousError, this downcast will
            // correctly succeed even with the message in between.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

  - **Attaching message whose type is used in downcasts onto errors whose
    type is insignificant.**

    Some codebases prefer to use machine-readable messages to categorize
    lower level errors in a way that will be actionable to higher levels of
    the application.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct HelperFailed;

    fn helper() -> Result<()> {
        bail!("no such file or directory");
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err(HelperFailed)?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<HelperFailed>() {
            // If helper failed, this downcast will succeed because
            // HelperFailed is the message that has been attached to
            // that error.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

#### Required Methods

- `fn wrap_err<D>(self, msg: D) -> Result<T, Report>`

  Wrap the error value with a new adhoc error

- `fn wrap_err_with<D, F>(self, f: F) -> Result<T, Report>`

  Wrap the error value with a new adhoc error that is evaluated lazily

- `fn context<D>(self, msg: D) -> Result<T, Report>`

  Compatibility re-export of `wrap_err()` for interop with `anyhow`

- `fn with_context<D, F>(self, f: F) -> Result<T, Report>`

  Compatibility re-export of `wrap_err_with()` for interop with `anyhow`

#### Implementors

- `Option<T>`
- `Result<T, E>`

### `IntoDiagnostic<T, E>`

```rust
trait IntoDiagnostic<T, E> { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/into_diagnostic.rs:35-39`](../../../.source_1765210505/miette-7.6.0/src/eyreish/into_diagnostic.rs#L35-L39)*

Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing
[`std::error::Error`](../../addr2line/index.md) to a [`Result<T, Report>`](../../clap_builder/error/index.md).

## Warning

Calling this on a type implementing [`Diagnostic`](../index.md) will reduce it to the common denominator of
[`std::error::Error`](../../addr2line/index.md). Meaning all extra information provided by [`Diagnostic`](../index.md) will be
inaccessible. If you have a type implementing [`Diagnostic`](../index.md) consider simply returning it or using
`Into` or the [`Try`](std::ops::Try) operator (`?`).

#### Required Methods

- `fn into_diagnostic(self) -> Result<T, Report>`

  Converts [`Result`](../../clap_builder/error/index.md) types that return regular [`std::error::Error`](../../addr2line/index.md)s

#### Implementors

- `Result<T, E>`

## Functions

### `set_hook`

```rust
fn set_hook(hook: ErrorHook) -> Result<(), InstallError>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:83-85`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L83-L85)*

Set the error hook.

### `capture_handler`

```rust
fn capture_handler(error: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:89-102`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L89-L102)*

### `get_default_printer`

```rust
fn get_default_printer(_err: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:104-109`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L104-L109)*

## Type Aliases

### `ErrorHook`

```rust
type ErrorHook = Box<dyn Fn(&dyn Diagnostic) -> Box<dyn ReportHandler> + Sync + Send>;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:61-62`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L61-L62)*

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:257`](../../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L257)*

type alias for `Result<T, Report>`

This is a reasonable return type to use throughout your application but also
for `main()`. If you do, failures will be printed along with a backtrace if
one was captured.

`miette::Result` may be used with one *or* two type parameters.

```rust
use miette::Result;

const IGNORE: &str = stringify! {
fn demo1() -> Result<T> {...}
           // ^ equivalent to std::result::Result<T, miette::Error>

fn demo2() -> Result<T, OtherError> {...}
           // ^ equivalent to std::result::Result<T, OtherError>
};
```

# Example

```rust
pub trait Deserialize {}

mod serde_json {
    use super::Deserialize;
    use std::io;

    pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct ClusterMap;

impl Deserialize for ClusterMap {}

use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    return Ok(());
    let config = std::fs::read_to_string("cluster.json").into_diagnostic()?;
    let map: ClusterMap = serde_json::from_str(&config).into_diagnostic()?;
    println!("cluster info: {:#?}", map);
    Ok(())
}
```

## `anyhow`/`eyre` Users

You can just replace `use`s of `anyhow::Result`/`eyre::Result` with
`miette::Result`.

