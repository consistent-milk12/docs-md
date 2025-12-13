*[serde_derive](../index.md) / [ser](index.md)*

---

# Module `ser`

## Contents

- [Structs](#structs)
  - [`Parameters`](#parameters)
- [Enums](#enums)
  - [`TupleVariant`](#tuplevariant)
  - [`StructVariant`](#structvariant)
  - [`StructTrait`](#structtrait)
  - [`TupleTrait`](#tupletrait)
- [Functions](#functions)
  - [`expand_derive_serialize`](#expand-derive-serialize)
  - [`precondition`](#precondition)
  - [`build_generics`](#build-generics)
  - [`needs_serialize_bound`](#needs-serialize-bound)
  - [`serialize_body`](#serialize-body)
  - [`serialize_transparent`](#serialize-transparent)
  - [`serialize_into`](#serialize-into)
  - [`serialize_unit_struct`](#serialize-unit-struct)
  - [`serialize_newtype_struct`](#serialize-newtype-struct)
  - [`serialize_tuple_struct`](#serialize-tuple-struct)
  - [`serialize_struct`](#serialize-struct)
  - [`serialize_struct_tag_field`](#serialize-struct-tag-field)
  - [`serialize_struct_as_struct`](#serialize-struct-as-struct)
  - [`serialize_struct_as_map`](#serialize-struct-as-map)
  - [`serialize_enum`](#serialize-enum)
  - [`serialize_variant`](#serialize-variant)
  - [`serialize_externally_tagged_variant`](#serialize-externally-tagged-variant)
  - [`serialize_internally_tagged_variant`](#serialize-internally-tagged-variant)
  - [`serialize_adjacently_tagged_variant`](#serialize-adjacently-tagged-variant)
  - [`serialize_untagged_variant`](#serialize-untagged-variant)
  - [`serialize_tuple_variant`](#serialize-tuple-variant)
  - [`serialize_struct_variant`](#serialize-struct-variant)
  - [`serialize_struct_variant_with_flatten`](#serialize-struct-variant-with-flatten)
  - [`serialize_tuple_struct_visitor`](#serialize-tuple-struct-visitor)
  - [`serialize_struct_visitor`](#serialize-struct-visitor)
  - [`wrap_serialize_field_with`](#wrap-serialize-field-with)
  - [`wrap_serialize_variant_with`](#wrap-serialize-variant-with)
  - [`wrap_serialize_with`](#wrap-serialize-with)
  - [`mut_if`](#mut-if)
  - [`get_member`](#get-member)
  - [`effective_style`](#effective-style)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parameters`](#parameters) | struct |  |
| [`TupleVariant`](#tuplevariant) | enum |  |
| [`StructVariant`](#structvariant) | enum |  |
| [`StructTrait`](#structtrait) | enum |  |
| [`TupleTrait`](#tupletrait) | enum |  |
| [`expand_derive_serialize`](#expand-derive-serialize) | fn |  |
| [`precondition`](#precondition) | fn |  |
| [`build_generics`](#build-generics) | fn |  |
| [`needs_serialize_bound`](#needs-serialize-bound) | fn |  |
| [`serialize_body`](#serialize-body) | fn |  |
| [`serialize_transparent`](#serialize-transparent) | fn |  |
| [`serialize_into`](#serialize-into) | fn |  |
| [`serialize_unit_struct`](#serialize-unit-struct) | fn |  |
| [`serialize_newtype_struct`](#serialize-newtype-struct) | fn |  |
| [`serialize_tuple_struct`](#serialize-tuple-struct) | fn |  |
| [`serialize_struct`](#serialize-struct) | fn |  |
| [`serialize_struct_tag_field`](#serialize-struct-tag-field) | fn |  |
| [`serialize_struct_as_struct`](#serialize-struct-as-struct) | fn |  |
| [`serialize_struct_as_map`](#serialize-struct-as-map) | fn |  |
| [`serialize_enum`](#serialize-enum) | fn |  |
| [`serialize_variant`](#serialize-variant) | fn |  |
| [`serialize_externally_tagged_variant`](#serialize-externally-tagged-variant) | fn |  |
| [`serialize_internally_tagged_variant`](#serialize-internally-tagged-variant) | fn |  |
| [`serialize_adjacently_tagged_variant`](#serialize-adjacently-tagged-variant) | fn |  |
| [`serialize_untagged_variant`](#serialize-untagged-variant) | fn |  |
| [`serialize_tuple_variant`](#serialize-tuple-variant) | fn |  |
| [`serialize_struct_variant`](#serialize-struct-variant) | fn |  |
| [`serialize_struct_variant_with_flatten`](#serialize-struct-variant-with-flatten) | fn |  |
| [`serialize_tuple_struct_visitor`](#serialize-tuple-struct-visitor) | fn |  |
| [`serialize_struct_visitor`](#serialize-struct-visitor) | fn |  |
| [`wrap_serialize_field_with`](#wrap-serialize-field-with) | fn |  |
| [`wrap_serialize_variant_with`](#wrap-serialize-variant-with) | fn |  |
| [`wrap_serialize_with`](#wrap-serialize-with) | fn |  |
| [`mut_if`](#mut-if) | fn |  |
| [`get_member`](#get-member) | fn |  |
| [`effective_style`](#effective-style) | fn |  |

## Structs

### `Parameters`

```rust
struct Parameters {
    self_var: syn::Ident,
    this_type: syn::Path,
    this_value: syn::Path,
    generics: syn::Generics,
    is_remote: bool,
    is_packed: bool,
}
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:78-100`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L78-L100)*

#### Fields

- **`self_var`**: `syn::Ident`

  Variable holding the value being serialized. Either `self` for local
  types or `__self` for remote types.

- **`this_type`**: `syn::Path`

  Path to the type the impl is for. Either a single `Ident` for local
  types (does not include generic parameters) or `some::remote::Path` for
  remote types.

- **`this_value`**: `syn::Path`

  Same as `this_type` but using `::<T>` for generic parameters for use in
  expression position.

- **`generics`**: `syn::Generics`

  Generics including any explicit and inferred bounds for the impl.

- **`is_remote`**: `bool`

  Type has a `serde(remote = "...")` attribute.

- **`is_packed`**: `bool`

  Type has a repr(packed) attribute.

#### Implementations

- <span id="parameters-new"></span>`fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md#container)

- <span id="parameters-type-name"></span>`fn type_name(&self) -> String`

  Type name to use in error messages and `&'static str` arguments to

  various Serializer methods.

#### Trait Implementations

##### `impl Any for Parameters`

- <span id="parameters-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parameters`

- <span id="parameters-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parameters`

- <span id="parameters-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Parameters`

- <span id="parameters-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parameters`

- <span id="parameters-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Parameters`

- <span id="parameters-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parameters-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parameters`

- <span id="parameters-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parameters-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `TupleVariant<'a>`

```rust
enum TupleVariant<'a> {
    ExternallyTagged {
        type_name: &'a crate::internals::name::Name,
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:805-812`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L805-L812)*

#### Trait Implementations

##### `impl Any for TupleVariant<'a>`

- <span id="tuplevariant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleVariant<'a>`

- <span id="tuplevariant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleVariant<'a>`

- <span id="tuplevariant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TupleVariant<'a>`

- <span id="tuplevariant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TupleVariant<'a>`

- <span id="tuplevariant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TupleVariant<'a>`

- <span id="tuplevariant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplevariant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TupleVariant<'a>`

- <span id="tuplevariant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplevariant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StructVariant<'a>`

```rust
enum StructVariant<'a> {
    ExternallyTagged {
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    InternallyTagged {
        tag: &'a str,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:873-883`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L873-L883)*

#### Trait Implementations

##### `impl Any for StructVariant<'a>`

- <span id="structvariant-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StructVariant<'a>`

- <span id="structvariant-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StructVariant<'a>`

- <span id="structvariant-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StructVariant<'a>`

- <span id="structvariant-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StructVariant<'a>`

- <span id="structvariant-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StructVariant<'a>`

- <span id="structvariant-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="structvariant-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StructVariant<'a>`

- <span id="structvariant-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="structvariant-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StructTrait`

```rust
enum StructTrait {
    SerializeMap,
    SerializeStruct,
    SerializeStructVariant,
}
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1315-1319`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1315-L1319)*

#### Implementations

- <span id="structtrait-serialize-field"></span>`fn serialize_field(&self, span: Span) -> TokenStream`

- <span id="structtrait-skip-field"></span>`fn skip_field(&self, span: Span) -> Option<TokenStream>`

#### Trait Implementations

##### `impl Any for StructTrait`

- <span id="structtrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StructTrait`

- <span id="structtrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StructTrait`

- <span id="structtrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StructTrait`

- <span id="structtrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StructTrait`

- <span id="structtrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for StructTrait`

- <span id="structtrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="structtrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StructTrait`

- <span id="structtrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="structtrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TupleTrait`

```rust
enum TupleTrait {
    SerializeTuple,
    SerializeTupleStruct,
    SerializeTupleVariant,
}
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1349-1353`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1349-L1353)*

#### Implementations

- <span id="tupletrait-serialize-element"></span>`fn serialize_element(&self, span: Span) -> TokenStream`

#### Trait Implementations

##### `impl Any for TupleTrait`

- <span id="tupletrait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleTrait`

- <span id="tupletrait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleTrait`

- <span id="tupletrait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for TupleTrait`

- <span id="tupletrait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TupleTrait`

- <span id="tupletrait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for TupleTrait`

- <span id="tupletrait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tupletrait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TupleTrait`

- <span id="tupletrait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tupletrait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `expand_derive_serialize`

```rust
fn expand_derive_serialize(input: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:12-64`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L12-L64)*

### `precondition`

```rust
fn precondition(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:66-76`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L66-L76)*

### `build_generics`

```rust
fn build_generics(cont: &crate::internals::ast::Container<'_>) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:135-153`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L135-L153)*

### `needs_serialize_bound`

```rust
fn needs_serialize_bound(field: &attr::Field, variant: Option<&attr::Variant>) -> bool
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:160-169`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L160-L169)*

### `serialize_body`

```rust
fn serialize_body(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:171-189`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L171-L189)*

### `serialize_transparent`

```rust
fn serialize_transparent(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:191-212`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L191-L212)*

### `serialize_into`

```rust
fn serialize_into(params: &Parameters, type_into: &syn::Type) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:214-221`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L214-L221)*

### `serialize_unit_struct`

```rust
fn serialize_unit_struct(cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:223-229`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L223-L229)*

### `serialize_newtype_struct`

```rust
fn serialize_newtype_struct(params: &Parameters, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:231-255`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L231-L255)*

### `serialize_tuple_struct`

```rust
fn serialize_tuple_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:257-294`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L257-L294)*

### `serialize_struct`

```rust
fn serialize_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:296-313`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L296-L313)*

### `serialize_struct_tag_field`

```rust
fn serialize_struct_tag_field(cattrs: &attr::Container, struct_trait: &StructTrait) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:315-326`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L315-L326)*

### `serialize_struct_as_struct`

```rust
fn serialize_struct_as_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:328-367`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L328-L367)*

### `serialize_struct_as_map`

```rust
fn serialize_struct_as_map(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:369-393`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L369-L393)*

### `serialize_enum`

```rust
fn serialize_enum(params: &Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:395-419`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L395-L419)*

### `serialize_variant`

```rust
fn serialize_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:421-501`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L421-L501)*

### `serialize_externally_tagged_variant`

```rust
fn serialize_externally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:503-574`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L503-L574)*

### `serialize_internally_tagged_variant`

```rust
fn serialize_internally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, tag: &str) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:576-640`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L576-L640)*

### `serialize_adjacently_tagged_variant`

```rust
fn serialize_adjacently_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, variant_index: u32, tag: &str, content: &str) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:642-764`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L642-L764)*

### `serialize_untagged_variant`

```rust
fn serialize_untagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:766-803`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L766-L803)*

### `serialize_tuple_variant`

```rust
fn serialize_tuple_variant(context: TupleVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>]) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:814-871`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L814-L871)*

### `serialize_struct_variant`

```rust
fn serialize_struct_variant(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:885-967`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L885-L967)*

### `serialize_struct_variant_with_flatten`

```rust
fn serialize_struct_variant_with_flatten(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:969-1055`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L969-L1055)*

### `serialize_tuple_struct_visitor`

```rust
fn serialize_tuple_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, tuple_trait: &TupleTrait) -> Vec<proc_macro2::TokenStream>
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1057-1103`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1057-L1103)*

### `serialize_struct_visitor`

```rust
fn serialize_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, struct_trait: &StructTrait) -> Vec<proc_macro2::TokenStream>
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1105-1169`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1105-L1169)*

### `wrap_serialize_field_with`

```rust
fn wrap_serialize_field_with(params: &Parameters, field_ty: &syn::Type, serialize_with: &syn::ExprPath, field_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1171-1178`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1171-L1178)*

### `wrap_serialize_variant_with`

```rust
fn wrap_serialize_variant_with(params: &Parameters, serialize_with: &syn::ExprPath, variant: &crate::internals::ast::Variant<'_>) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1180-1205`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1180-L1205)*

### `wrap_serialize_with`

```rust
fn wrap_serialize_with(params: &Parameters, serialize_with: &syn::ExprPath, field_tys: &[&syn::Type], field_exprs: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1207-1263`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1207-L1263)*

### `mut_if`

```rust
fn mut_if(is_mut: bool) -> Option<proc_macro2::TokenStream>
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1271-1277`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1271-L1277)*

### `get_member`

```rust
fn get_member(params: &Parameters, field: &crate::internals::ast::Field<'_>, member: &syn::Member) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1279-1306`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1279-L1306)*

### `effective_style`

```rust
fn effective_style(variant: &crate::internals::ast::Variant<'_>) -> crate::internals::ast::Style
```

*Defined in [`serde_derive-1.0.228/src/ser.rs:1308-1313`](../../../.source_1765633015/serde_derive-1.0.228/src/ser.rs#L1308-L1313)*

