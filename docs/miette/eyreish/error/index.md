*[miette](../../index.md) / [eyreish](../index.md) / [error](index.md)*

---

# Module `error`

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

#### Trait Implementations

##### `impl<D> OwoColorize for ErrorVTable`

### `ErrorImpl<E>`

```rust
struct ErrorImpl<E> {
    vtable: &'static ErrorVTable,
    handler: Option<Box<dyn ReportHandler>>,
    _object: E,
}
```

#### Implementations

- `fn erase(self: &Self) -> Ref<'_, ErrorImpl<()>>` — [`Ref`](../ptr/index.md), [`ErrorImpl`](#errorimpl)

#### Trait Implementations

##### `impl<E> Debug for ErrorImpl<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Display for ErrorImpl<E>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> OwoColorize for ErrorImpl<E>`

##### `impl<T> ToString for ErrorImpl<E>`

- `fn to_string(self: &Self) -> String`

### `ContextError<D, E>`

```rust
struct ContextError<D, E> {
    msg: D,
    error: E,
}
```

#### Trait Implementations

##### `impl<D, E> Debug for super::error::ContextError<D, E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E> Diag for ContextError<D, E>`

- `fn ext_report<D>(self: Self, msg: D) -> Report` — [`Report`](../../index.md)

##### `impl<D> Diagnostic for super::error::ContextError<D, super::Report>`

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn severity(self: &Self) -> Option<crate::Severity>` — [`Severity`](../../index.md)

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- `fn labels<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](../../index.md)

- `fn source_code(self: &Self) -> Option<&dyn crate::SourceCode>` — [`SourceCode`](../../index.md)

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>` — [`Diagnostic`](../../index.md)

##### `impl<D, E> Display for super::error::ContextError<D, E>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<D> Error for super::error::ContextError<D, super::Report>`

- `fn source(self: &Self) -> Option<&dyn StdError>`

##### `impl<D> OwoColorize for ContextError<D, E>`

##### `impl<T> ToString for ContextError<D, E>`

- `fn to_string(self: &Self) -> String`

##### `impl<E> TraitKind for ContextError<D, E>`

## Functions

### `object_drop`

```rust
unsafe fn object_drop<E>(e: super::ptr::Own<ErrorImpl<()>>)
```

### `object_drop_front`

```rust
unsafe fn object_drop_front<E>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
```

### `object_ref`

```rust
unsafe fn object_ref<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn Diagnostic + Send + Sync>
where
    E: Diagnostic + Send + Sync + 'static
```

### `object_ref_stderr`

```rust
unsafe fn object_ref_stderr<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>) -> super::ptr::Ref<'_, dyn StdError + Send + Sync>
where
    E: StdError + Send + Sync + 'static
```

### `object_boxed`

```rust
unsafe fn object_boxed<E>(e: super::ptr::Own<ErrorImpl<()>>) -> Box<dyn Diagnostic + Send + Sync>
where
    E: Diagnostic + Send + Sync + 'static
```

### `object_boxed_stderr`

```rust
unsafe fn object_boxed_stderr<E>(e: super::ptr::Own<ErrorImpl<()>>) -> Box<dyn StdError + Send + Sync>
where
    E: StdError + Send + Sync + 'static
```

### `object_downcast`

```rust
unsafe fn object_downcast<E>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    E: 'static
```

### `context_downcast`

```rust
unsafe fn context_downcast<D, E>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    D: 'static,
    E: 'static
```

### `context_drop_rest`

```rust
unsafe fn context_drop_rest<D, E>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
where
    D: 'static,
    E: 'static
```

### `context_chain_downcast`

```rust
unsafe fn context_chain_downcast<D>(e: super::ptr::Ref<'_, ErrorImpl<()>>, target: core::any::TypeId) -> Option<super::ptr::Ref<'_, ()>>
where
    D: 'static
```

### `context_chain_drop_rest`

```rust
unsafe fn context_chain_drop_rest<D>(e: super::ptr::Own<ErrorImpl<()>>, target: core::any::TypeId)
where
    D: 'static
```

### `vtable`

```rust
unsafe fn vtable(p: core::ptr::NonNull<ErrorImpl<()>>) -> &'static ErrorVTable
```

## Type Aliases

### `ErasedErrorImpl`

```rust
type ErasedErrorImpl = ErrorImpl<()>;
```

