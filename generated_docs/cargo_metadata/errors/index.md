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

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:25-52`](../../../.source_1765521767/cargo_metadata-0.23.1/src/errors.rs#L25-L52)*

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

##### `impl Debug for Error`

- <span id="error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-fmt"></span>`fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Error for Error`

- <span id="error-source"></span>`fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private17::Error>`

##### `impl ToString for Error`

- <span id="error-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `Result<T, E>`

```rust
type Result<T, E> = ::std::result::Result<T, E>;
```

*Defined in [`cargo_metadata-0.23.1/src/errors.rs:4`](../../../.source_1765521767/cargo_metadata-0.23.1/src/errors.rs#L4)*

Custom result type for `cargo_metadata::Error`

