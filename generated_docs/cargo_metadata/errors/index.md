*[cargo_metadata](../index.md) / [errors](index.md)*

---

# Module `errors`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | Error returned when executing/parsing `cargo metadata` fails. |
| [`Result`](#result) | type | Custom result type for `cargo_metadata::Error` |

## Enums

### `Error`

```rust
enum Error {
    CargoMetadata {
        stderr: String,
    },
    Io(io::Error),
    Utf8(std::str::Utf8Error),
    ErrUtf8(std::string::FromUtf8Error),
    Json(::serde_json::Error),
    NoJson,
}
```

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:25-52`](../../../.source_1765633015/cargo_metadata-0.23.1/src/errors.rs#L25-L52)*

Error returned when executing/parsing `cargo metadata` fails.

# Note about Backtraces

This error type does not contain backtraces, but each error variant
comes from _one_ specific place, so it's not really needed for the
inside of this crate. If you need a backtrace down to, but not inside
of, a failed call of `cargo_metadata` you can do one of multiple thinks:

1. Convert it to a `failure::Error` (possible using the `?` operator),
   which is similar to a `Box<::std::error::Error + 'static + Send  + Sync>`.
2. Have appropriate variants in your own error type. E.g. you could wrap
   a `failure::Context<Error>` or add a `failure::Backtrace` field (which
   is empty if `RUST_BACKTRACE` is not set, so it's simple to use).
3. You still can place a failure based error into a `error_chain` if you
   really want to. (Either through foreign_links or by making it a field
   value of a `ErrorKind` variant).


#### Variants

- **`CargoMetadata`**

  Error during execution of `cargo metadata`

- **`Io`**

  IO Error during execution of `cargo metadata`

- **`Utf8`**

  Output of `cargo metadata` was not valid utf8

- **`ErrUtf8`**

  Error output of `cargo metadata` was not valid utf8

- **`Json`**

  Deserialization error (structure of json did not match expected structure)

- **`NoJson`**

  The output did not contain any json

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for Error`

- <span id="error-error-source"></span>`fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private17::Error>`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = ::std::result::Result<T, E>;
```

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:4`](../../../.source_1765633015/cargo_metadata-0.23.1/src/errors.rs#L4)*

Custom result type for `cargo_metadata::Error`

