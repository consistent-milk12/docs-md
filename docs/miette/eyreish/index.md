*[miette](../index.md) / [eyreish](index.md)*

---

# Module `eyreish`

## Modules

- [`context`](context/index.md) - 
- [`error`](error/index.md) - 
- [`fmt`](fmt/index.md) - 
- [`into_diagnostic`](into_diagnostic/index.md) - 
- [`kind`](kind/index.md) - 
- [`macros`](macros/index.md) - 
- [`ptr`](ptr/index.md) - 
- [`wrapper`](wrapper/index.md) - 

## Structs

### `Error`

```rust
struct Error {
    inner: self::ptr::Own<error::ErrorImpl<()>>,
}
```

Core Diagnostic wrapper type.

## `eyre` Users

You can just replace `use`s of `eyre::Report` with `miette::Report`.

#### Implementations

- `fn new<E>(error: E) -> Self`

- `fn msg<M>(message: M) -> Self`

- `fn new_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- `fn from_std<E>(error: E) -> Self`

- `fn from_adhoc<M>(message: M) -> Self`

- `fn from_msg<D, E>(msg: D, error: E) -> Self`

- `fn from_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- `unsafe fn construct<E>(error: E, vtable: &'static ErrorVTable, handler: Option<Box<dyn ReportHandler>>) -> Self` — [`ErrorVTable`](error/index.md), [`ReportHandler`](../index.md)

- `fn wrap_err<D>(self: Self, msg: D) -> Self`

- `fn context<D>(self: Self, msg: D) -> Self`

- `fn chain(self: &Self) -> Chain<'_>` — [`Chain`](../chain/index.md)

- `fn root_cause(self: &Self) -> &dyn StdError`

- `fn is<E>(self: &Self) -> bool`

- `fn downcast<E>(self: Self) -> Result<E, Self>`

- `fn downcast_ref<E>(self: &Self) -> Option<&E>`

- `fn downcast_mut<E>(self: &mut Self) -> Option<&mut E>`

- `fn handler(self: &Self) -> &dyn ReportHandler` — [`ReportHandler`](../index.md)

- `fn handler_mut(self: &mut Self) -> &mut dyn ReportHandler` — [`ReportHandler`](../index.md)

- `fn with_source_code(self: Self, source_code: impl SourceCode + 'static) -> Report` — [`SourceCode`](../index.md), [`Report`](../index.md)

- `fn from_err<E>(err: E) -> Self`

#### Trait Implementations

##### `impl AsRef for super::Report`

- `fn as_ref(self: &Self) -> &dyn Diagnostic` — [`Diagnostic`](../index.md)

##### `impl Debug for super::Report`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for super::Report`

- `type Target = dyn Diagnostic + Send + Sync`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for super::Report`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Diag for super::Report`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Display for super::Report`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for super::Report`

- `fn drop(self: &mut Self)`

##### `impl<D> OwoColorize for Report`

##### `impl<P, T> Receiver for Report`

- `type Target = T`

##### `impl Send for Report`

##### `impl Sync for Report`

##### `impl<T> ToString for Report`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for Report`

### `Report`

```rust
struct Report {
    inner: self::ptr::Own<error::ErrorImpl<()>>,
}
```

Core Diagnostic wrapper type.

## `eyre` Users

You can just replace `use`s of `eyre::Report` with `miette::Report`.

#### Implementations

- `fn new<E>(error: E) -> Self`

- `fn msg<M>(message: M) -> Self`

- `fn new_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- `fn from_std<E>(error: E) -> Self`

- `fn from_adhoc<M>(message: M) -> Self`

- `fn from_msg<D, E>(msg: D, error: E) -> Self`

- `fn from_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](../index.md)

- `unsafe fn construct<E>(error: E, vtable: &'static ErrorVTable, handler: Option<Box<dyn ReportHandler>>) -> Self` — [`ErrorVTable`](error/index.md), [`ReportHandler`](../index.md)

- `fn wrap_err<D>(self: Self, msg: D) -> Self`

- `fn context<D>(self: Self, msg: D) -> Self`

- `fn chain(self: &Self) -> Chain<'_>` — [`Chain`](../chain/index.md)

- `fn root_cause(self: &Self) -> &dyn StdError`

- `fn is<E>(self: &Self) -> bool`

- `fn downcast<E>(self: Self) -> Result<E, Self>`

- `fn downcast_ref<E>(self: &Self) -> Option<&E>`

- `fn downcast_mut<E>(self: &mut Self) -> Option<&mut E>`

- `fn handler(self: &Self) -> &dyn ReportHandler` — [`ReportHandler`](../index.md)

- `fn handler_mut(self: &mut Self) -> &mut dyn ReportHandler` — [`ReportHandler`](../index.md)

- `fn with_source_code(self: Self, source_code: impl SourceCode + 'static) -> Report` — [`SourceCode`](../index.md), [`Report`](../index.md)

- `fn from_err<E>(err: E) -> Self`

#### Trait Implementations

##### `impl AsRef for super::Report`

- `fn as_ref(self: &Self) -> &dyn Diagnostic` — [`Diagnostic`](../index.md)

##### `impl Debug for super::Report`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for super::Report`

- `type Target = dyn Diagnostic + Send + Sync`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl DerefMut for super::Report`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl Diag for super::Report`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Display for super::Report`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for super::Report`

- `fn drop(self: &mut Self)`

##### `impl<D> OwoColorize for Report`

##### `impl<P, T> Receiver for Report`

- `type Target = T`

##### `impl Send for Report`

##### `impl Sync for Report`

##### `impl<T> ToString for Report`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for Report`

### `InstallError`

```rust
struct InstallError;
```

Error indicating that [`set_hook()`](../index.md) was unable to install the provided
[`ErrorHook`](../index.md).

#### Trait Implementations

##### `impl Debug for InstallError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for InstallError`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for InstallError`

##### `impl Display for InstallError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for InstallError`

##### `impl<D> OwoColorize for InstallError`

##### `impl<T> ToString for InstallError`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for InstallError`

### `DiagnosticError`

```rust
struct DiagnosticError(Box<dyn std::error::Error + Send + Sync>);
```

Convenience [`Diagnostic`](../index.md) that can be used as an "anonymous" wrapper for
Errors. This is intended to be paired with [`IntoDiagnostic`](#intodiagnostic).

#### Trait Implementations

##### `impl Debug for DiagnosticError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E> Diag for DiagnosticError`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../index.md)

##### `impl Diagnostic for DiagnosticError`

##### `impl Display for DiagnosticError`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for DiagnosticError`

- `fn source(self: &Self) -> Option<&dyn Error>`

##### `impl<D> OwoColorize for DiagnosticError`

##### `impl<T> ToString for DiagnosticError`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for DiagnosticError`

## Traits

### `ReportHandler`

```rust
trait ReportHandler: core::any::Any + Send + Sync { ... }
```

Error Report Handler trait for customizing `miette::Report`

#### Required Methods

- `fn debug(self: &Self, error: &dyn Diagnostic, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Define the report format

- `fn display(self: &Self, error: &dyn StdError, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Override for the `Display` format

- `fn track_caller(self: &mut Self, location: &'static std::panic::Location<'static>)`

  Store the location of the caller who constructed this error report

### `WrapErr<T, E>`

```rust
trait WrapErr<T, E>: context::private::Sealed { ... }
```

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

- `fn wrap_err<D>(self: Self, msg: D) -> Result<T, Report>`

  Wrap the error value with a new adhoc error

- `fn wrap_err_with<D, F>(self: Self, f: F) -> Result<T, Report>`

  Wrap the error value with a new adhoc error that is evaluated lazily

- `fn context<D>(self: Self, msg: D) -> Result<T, Report>`

  Compatibility re-export of `wrap_err()` for interop with `anyhow`

- `fn with_context<D, F>(self: Self, f: F) -> Result<T, Report>`

  Compatibility re-export of `wrap_err_with()` for interop with `anyhow`

### `IntoDiagnostic<T, E>`

```rust
trait IntoDiagnostic<T, E> { ... }
```

Convenience trait that adds a [`.into_diagnostic()`](IntoDiagnostic::into_diagnostic) method that converts a type implementing
[`std::error::Error`](../../docs_md/error/index.md) to a [`Result<T, Report>`](../../clap_builder/error/index.md).

## Warning

Calling this on a type implementing [`Diagnostic`](../index.md) will reduce it to the common denominator of
[`std::error::Error`](../../docs_md/error/index.md). Meaning all extra information provided by [`Diagnostic`](../index.md) will be
inaccessible. If you have a type implementing [`Diagnostic`](../index.md) consider simply returning it or using
`Into` or the [`Try`](std::ops::Try) operator (`?`).

#### Required Methods

- `fn into_diagnostic(self: Self) -> Result<T, Report>`

  Converts [`Result`](../../clap_builder/error/index.md) types that return regular [`std::error::Error`](../../docs_md/error/index.md)s

## Functions

### `set_hook`

```rust
fn set_hook(hook: ErrorHook) -> Result<(), InstallError>
```

Set the error hook.

### `capture_handler`

```rust
fn capture_handler(error: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

### `get_default_printer`

```rust
fn get_default_printer(_err: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

## Type Aliases

### `ErrorHook`

```rust
type ErrorHook = Box<dyn Fn(&dyn Diagnostic) -> Box<dyn ReportHandler> + Sync + Send>;
```

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

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

