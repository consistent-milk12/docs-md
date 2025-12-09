*[miette](../../index.md) / [eyreish](../index.md) / [error](index.md)*

---

# Module `error`

## Contents

- [Structs](#structs)
  - [`ErrorVTable`](#errorvtable)
  - [`ErrorImpl`](#errorimpl)
  - [`ContextError`](#contexterror)
- [Functions](#functions)
  - [`object_drop`](#object_drop)
  - [`object_drop_front`](#object_drop_front)
  - [`object_ref`](#object_ref)
  - [`object_ref_stderr`](#object_ref_stderr)
  - [`object_boxed`](#object_boxed)
  - [`object_boxed_stderr`](#object_boxed_stderr)
  - [`object_downcast`](#object_downcast)
  - [`context_downcast`](#context_downcast)
  - [`context_drop_rest`](#context_drop_rest)
  - [`context_chain_downcast`](#context_chain_downcast)
  - [`context_chain_drop_rest`](#context_chain_drop_rest)
  - [`vtable`](#vtable)
- [Type Aliases](#type-aliases)
  - [`ErasedErrorImpl`](#erasederrorimpl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ErrorVTable`](#errorvtable) | struct |  |
| [`ErrorImpl`](#errorimpl) | struct |  |
| [`ContextError`](#contexterror) | struct |  |
| [`object_drop`](#object_drop) | fn |  |
| [`object_drop_front`](#object_drop_front) | fn |  |
| [`object_ref`](#object_ref) | fn |  |
| [`object_ref_stderr`](#object_ref_stderr) | fn |  |
| [`object_boxed`](#object_boxed) | fn |  |
| [`object_boxed_stderr`](#object_boxed_stderr) | fn |  |
| [`object_downcast`](#object_downcast) | fn |  |
| [`context_downcast`](#context_downcast) | fn |  |
| [`context_drop_rest`](#context_drop_rest) | fn |  |
| [`context_chain_downcast`](#context_chain_downcast) | fn |  |
| [`context_chain_drop_rest`](#context_chain_drop_rest) | fn |  |
| [`vtable`](#vtable) | fn |  |
| [`ErasedErrorImpl`](#erasederrorimpl) | type |  |

## Structs

### `ErrorVTable`

```rust
struct ErrorVTable {
    object_drop: fn(super::ptr::Own<ErrorImpl<()>>),
    object_ref: fn(super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn Diagnostic + Send + Sync>,
    object_ref_stderr: fn(super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn StdError + Send + Sync>,
    object_boxed: fn(super::ptr::Own<ErrorImpl<()>>) -> Box<dyn Diagnostic + Send + Sync>,
    object_boxed_stderr: fn(super::ptr::Own<ErrorImpl<()>>) -> Box<dyn StdError + Send + Sync>,
    object_downcast: fn(super::ptr::Ref<'_, ErrorImpl<()>>, core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>,
    object_drop_rest: fn(super::ptr::Own<ErrorImpl<()>>, core::any::TypeId),
}
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:484-497`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L484-L497)*

#### Trait Implementations

##### `impl OwoColorize for ErrorVTable`

### `ErrorImpl<E>`

```rust
struct ErrorImpl<E> {
    vtable: &'static ErrorVTable,
    handler: Option<Box<dyn ReportHandler>>,
    _object: E,
}
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:674-680`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L674-L680)*

#### Implementations

- <span id="errorimpl-erase"></span>`fn erase(&self) -> Ref<'_, ErrorImpl<()>>` — [`Ref`](../ptr/index.md), [`ErrorImpl`](#errorimpl)

#### Trait Implementations

##### `impl<E> Debug for ErrorImpl<E>`

- <span id="errorimpl-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Display for ErrorImpl<E>`

- <span id="errorimpl-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ErrorImpl<E>`

##### `impl<T> ToString for ErrorImpl<E>`

- <span id="errorimpl-to-string"></span>`fn to_string(&self) -> String`

### `ContextError<D, E>`

```rust
struct ContextError<D, E> {
    msg: D,
    error: E,
}
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:685-688`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L685-L688)*

#### Trait Implementations

##### `impl<D, E> Debug for super::error::ContextError<D, E>`

- <span id="supererrorcontexterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for ContextError<D, E>`

- <span id="contexterror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl<D, E> Diagnostic for super::error::ContextError<D, E>`

- <span id="supererrorcontexterror-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="supererrorcontexterror-severity"></span>`fn severity(&self) -> Option<crate::Severity>` — [`Severity`](../../index.md)

- <span id="supererrorcontexterror-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="supererrorcontexterror-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="supererrorcontexterror-labels"></span>`fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md)

- <span id="supererrorcontexterror-source-code"></span>`fn source_code(&self) -> Option<&dyn crate::SourceCode>` — [`SourceCode`](../../index.md)

- <span id="supererrorcontexterror-related"></span>`fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md)

##### `impl<D, E> Display for super::error::ContextError<D, E>`

- <span id="supererrorcontexterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D, E> Error for super::error::ContextError<D, E>`

- <span id="supererrorcontexterror-source"></span>`fn source(&self) -> Option<&dyn StdError>`

##### `impl<D> OwoColorize for ContextError<D, E>`

##### `impl<T> ToString for ContextError<D, E>`

- <span id="contexterror-to-string"></span>`fn to_string(&self) -> String`

##### `impl<E> TraitKind for ContextError<D, E>`

## Functions

### `object_drop`

```rust
unsafe fn object_drop<E>(e: super::ptr::Own<ErrorImpl<()>>)
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:500-505`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L500-L505)*

### `object_drop_front`

```rust
unsafe fn object_drop_front<E>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:508-515`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L508-L515)*

### `object_ref`

```rust
unsafe fn object_ref<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn Diagnostic + Send + Sync>
where
    E: Diagnostic + Send + Sync + 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:518-530`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L518-L530)*

### `object_ref_stderr`

```rust
unsafe fn object_ref_stderr<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn StdError + Send + Sync>
where
    E: StdError + Send + Sync + 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:533-545`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L533-L545)*

### `object_boxed`

```rust
unsafe fn object_boxed<E>(e: super::ptr::Own<ErrorImpl<()>>) -> Box<dyn Diagnostic + Send + Sync>
where
    E: Diagnostic + Send + Sync + 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:548-555`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L548-L555)*

### `object_boxed_stderr`

```rust
unsafe fn object_boxed_stderr<E>(e: super::ptr::Own<ErrorImpl<()>>) -> Box<dyn StdError + Send + Sync>
where
    E: StdError + Send + Sync + 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:558-567`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L558-L567)*

### `object_downcast`

```rust
unsafe fn object_downcast<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    E: 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:570-588`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L570-L588)*

### `context_downcast`

```rust
unsafe fn context_downcast<D, E>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    D: 'static,
    E: 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:591-605`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L591-L605)*

### `context_drop_rest`

```rust
unsafe fn context_drop_rest<D, E>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
where
    D: 'static,
    E: 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:608-626`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L608-L626)*

### `context_chain_downcast`

```rust
unsafe fn context_chain_downcast<D>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    D: 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:629-644`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L629-L644)*

### `context_chain_drop_rest`

```rust
unsafe fn context_chain_drop_rest<D>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
where
    D: 'static
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:647-670`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L647-L670)*

### `vtable`

```rust
unsafe fn vtable(p: core::ptr::NonNull<ErrorImpl<()>>) -> &'static ErrorVTable
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:693-695`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L693-L695)*

## Type Aliases

### `ErasedErrorImpl`

```rust
type ErasedErrorImpl = ErrorImpl<()>;
```

*Defined in [`miette-7.6.0/src/eyreish/error.rs:690`](../../../../.source_1765210505/miette-7.6.0/src/eyreish/error.rs#L690)*

