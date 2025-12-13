*[miette_derive](../index.md) / [diagnostic](index.md)*

---

# Module `diagnostic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DiagnosticDef`](#diagnosticdef) | struct |  |
| [`DiagnosticConcreteArgs`](#diagnosticconcreteargs) | struct |  |
| [`Diagnostic`](#diagnostic) | enum |  |
| [`DiagnosticDefArgs`](#diagnosticdefargs) | enum |  |

## Structs

### `DiagnosticDef`

```rust
struct DiagnosticDef {
    pub ident: syn::Ident,
    pub fields: syn::Fields,
    pub args: DiagnosticDefArgs,
}
```

*Defined in [`miette-derive-7.6.0/src/diagnostic.rs:30-34`](../../../.source_1765633015/miette-derive-7.6.0/src/diagnostic.rs#L30-L34)*

#### Trait Implementations

##### `impl Any for DiagnosticDef`

- <span id="diagnosticdef-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticDef`

- <span id="diagnosticdef-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticDef`

- <span id="diagnosticdef-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DiagnosticDef`

- <span id="diagnosticdef-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticDef`

- <span id="diagnosticdef-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DiagnosticDef`

- <span id="diagnosticdef-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticdef-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticDef`

- <span id="diagnosticdef-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticdef-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DiagnosticConcreteArgs`

```rust
struct DiagnosticConcreteArgs {
    pub code: Option<crate::code::Code>,
    pub severity: Option<crate::severity::Severity>,
    pub help: Option<crate::help::Help>,
    pub labels: Option<crate::label::Labels>,
    pub source_code: Option<crate::source_code::SourceCode>,
    pub url: Option<crate::url::Url>,
    pub forward: Option<crate::forward::Forward>,
    pub related: Option<crate::related::Related>,
    pub diagnostic_source: Option<crate::diagnostic_source::DiagnosticSource>,
}
```

*Defined in [`miette-derive-7.6.0/src/diagnostic.rs:61-71`](../../../.source_1765633015/miette-derive-7.6.0/src/diagnostic.rs#L61-L71)*

#### Implementations

- <span id="diagnosticconcreteargs-for-fields"></span>`fn for_fields(fields: &syn::Fields) -> Result<Self, syn::Error>`

- <span id="diagnosticconcreteargs-add-args"></span>`fn add_args(&mut self, attr: &syn::Attribute, args: impl Iterator<Item = DiagnosticArg>, errors: &mut Vec<syn::Error>)` — [`DiagnosticArg`](../diagnostic_arg/index.md#diagnosticarg)

#### Trait Implementations

##### `impl Any for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Default for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-default"></span>`fn default() -> DiagnosticConcreteArgs` — [`DiagnosticConcreteArgs`](#diagnosticconcreteargs)

##### `impl<T> From for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticconcreteargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticConcreteArgs`

- <span id="diagnosticconcreteargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticconcreteargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Diagnostic`

```rust
enum Diagnostic {
    Struct {
        generics: syn::Generics,
        ident: syn::Ident,
        fields: syn::Fields,
        args: DiagnosticDefArgs,
    },
    Enum {
        ident: syn::Ident,
        generics: syn::Generics,
        variants: Vec<DiagnosticDef>,
    },
}
```

*Defined in [`miette-derive-7.6.0/src/diagnostic.rs:16-28`](../../../.source_1765633015/miette-derive-7.6.0/src/diagnostic.rs#L16-L28)*

#### Implementations

- <span id="diagnostic-from-derive-input"></span>`fn from_derive_input(input: DeriveInput) -> Result<Self, syn::Error>`

- <span id="diagnostic-gen"></span>`fn gen(&self) -> TokenStream`

#### Trait Implementations

##### `impl Any for Diagnostic`

- <span id="diagnostic-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Diagnostic`

- <span id="diagnostic-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Diagnostic`

- <span id="diagnostic-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Diagnostic`

- <span id="diagnostic-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Diagnostic`

- <span id="diagnostic-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Diagnostic`

- <span id="diagnostic-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnostic-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Diagnostic`

- <span id="diagnostic-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnostic-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DiagnosticDefArgs`

```rust
enum DiagnosticDefArgs {
    Transparent(crate::forward::Forward),
    Concrete(Box<DiagnosticConcreteArgs>),
}
```

*Defined in [`miette-derive-7.6.0/src/diagnostic.rs:36-39`](../../../.source_1765633015/miette-derive-7.6.0/src/diagnostic.rs#L36-L39)*

#### Implementations

- <span id="diagnosticdefargs-forward-or-override-enum"></span>`fn forward_or_override_enum(&self, variant: &syn::Ident, which_fn: WhichFn, f: impl FnMut(&DiagnosticConcreteArgs) -> Option<TokenStream>) -> Option<TokenStream>` — [`WhichFn`](../forward/index.md#whichfn), [`DiagnosticConcreteArgs`](#diagnosticconcreteargs)

#### Trait Implementations

##### `impl Any for DiagnosticDefArgs`

- <span id="diagnosticdefargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DiagnosticDefArgs`

- <span id="diagnosticdefargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DiagnosticDefArgs`

- <span id="diagnosticdefargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DiagnosticDefArgs`

- <span id="diagnosticdefargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DiagnosticDefArgs`

- <span id="diagnosticdefargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DiagnosticDefArgs`

- <span id="diagnosticdefargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diagnosticdefargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DiagnosticDefArgs`

- <span id="diagnosticdefargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diagnosticdefargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

