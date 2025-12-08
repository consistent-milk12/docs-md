*[serde_derive](../../index.md) / [internals](../index.md) / [attr](index.md)*

---

# Module `attr`

## Contents

- [Structs](#structs)
  - [`Attr`](#attr)
  - [`BoolAttr`](#boolattr)
  - [`VecAttr`](#vecattr)
  - [`RenameAllRules`](#renameallrules)
  - [`Container`](#container)
  - [`Variant`](#variant)
  - [`BorrowAttribute`](#borrowattribute)
  - [`Field`](#field)
- [Enums](#enums)
  - [`unnamed`](#unnamed)
  - [`TagType`](#tagtype)
  - [`Identifier`](#identifier)
  - [`Default`](#default)
- [Functions](#functions)
  - [`unraw`](#unraw)
  - [`decide_tag`](#decide_tag)
  - [`decide_identifier`](#decide_identifier)
  - [`get_ser_and_de`](#get_ser_and_de)
  - [`get_renames`](#get_renames)
  - [`get_multiple_renames`](#get_multiple_renames)
  - [`get_where_predicates`](#get_where_predicates)
  - [`get_lit_str`](#get_lit_str)
  - [`get_lit_str2`](#get_lit_str2)
  - [`parse_lit_into_path`](#parse_lit_into_path)
  - [`parse_lit_into_expr_path`](#parse_lit_into_expr_path)
  - [`parse_lit_into_where`](#parse_lit_into_where)
  - [`parse_lit_into_ty`](#parse_lit_into_ty)
  - [`parse_lit_into_lifetimes`](#parse_lit_into_lifetimes)
  - [`is_implicitly_borrowed`](#is_implicitly_borrowed)
  - [`is_implicitly_borrowed_reference`](#is_implicitly_borrowed_reference)
  - [`is_cow`](#is_cow)
  - [`is_option`](#is_option)
  - [`is_reference`](#is_reference)
  - [`is_str`](#is_str)
  - [`is_slice_u8`](#is_slice_u8)
  - [`is_primitive_type`](#is_primitive_type)
  - [`is_primitive_path`](#is_primitive_path)
  - [`borrowable_lifetimes`](#borrowable_lifetimes)
  - [`collect_lifetimes`](#collect_lifetimes)
  - [`collect_lifetimes_from_tokens`](#collect_lifetimes_from_tokens)
- [Type Aliases](#type-aliases)
  - [`SerAndDe`](#serandde)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Attr`](#attr) | struct |  |
| [`BoolAttr`](#boolattr) | struct |  |
| [`VecAttr`](#vecattr) | struct |  |
| [`RenameAllRules`](#renameallrules) | struct |  |
| [`Container`](#container) | struct | Represents struct or enum attribute information. |
| [`Variant`](#variant) | struct | Represents variant attribute information |
| [`BorrowAttribute`](#borrowattribute) | struct |  |
| [`Field`](#field) | struct | Represents field attribute information |
| [`unnamed`](#unnamed) | enum |  |
| [`TagType`](#tagtype) | enum | Styles of representing an enum. |
| [`Identifier`](#identifier) | enum | Whether this enum represents the fields of a struct or the variants of an |
| [`Default`](#default) | enum | Represents the default to use for a field when deserializing. |
| [`unraw`](#unraw) | fn |  |
| [`decide_tag`](#decide_tag) | fn |  |
| [`decide_identifier`](#decide_identifier) | fn |  |
| [`get_ser_and_de`](#get_ser_and_de) | fn |  |
| [`get_renames`](#get_renames) | fn |  |
| [`get_multiple_renames`](#get_multiple_renames) | fn |  |
| [`get_where_predicates`](#get_where_predicates) | fn |  |
| [`get_lit_str`](#get_lit_str) | fn |  |
| [`get_lit_str2`](#get_lit_str2) | fn |  |
| [`parse_lit_into_path`](#parse_lit_into_path) | fn |  |
| [`parse_lit_into_expr_path`](#parse_lit_into_expr_path) | fn |  |
| [`parse_lit_into_where`](#parse_lit_into_where) | fn |  |
| [`parse_lit_into_ty`](#parse_lit_into_ty) | fn |  |
| [`parse_lit_into_lifetimes`](#parse_lit_into_lifetimes) | fn |  |
| [`is_implicitly_borrowed`](#is_implicitly_borrowed) | fn |  |
| [`is_implicitly_borrowed_reference`](#is_implicitly_borrowed_reference) | fn |  |
| [`is_cow`](#is_cow) | fn |  |
| [`is_option`](#is_option) | fn |  |
| [`is_reference`](#is_reference) | fn |  |
| [`is_str`](#is_str) | fn |  |
| [`is_slice_u8`](#is_slice_u8) | fn |  |
| [`is_primitive_type`](#is_primitive_type) | fn |  |
| [`is_primitive_path`](#is_primitive_path) | fn |  |
| [`borrowable_lifetimes`](#borrowable_lifetimes) | fn |  |
| [`collect_lifetimes`](#collect_lifetimes) | fn |  |
| [`collect_lifetimes_from_tokens`](#collect_lifetimes_from_tokens) | fn |  |
| [`SerAndDe`](#serandde) | type |  |

## Structs

### `Attr<'c, T>`

```rust
struct Attr<'c, T> {
    cx: &'c crate::internals::Ctxt,
    name: Symbol,
    tokens: proc_macro2::TokenStream,
    value: Option<T>,
}
```

#### Implementations

- <span id="attr-none"></span>`fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../index.md), [`Symbol`](../symbol/index.md)

- <span id="attr-set"></span>`fn set<A: ToTokens>(&mut self, obj: A, value: T)`

- <span id="attr-set-opt"></span>`fn set_opt<A: ToTokens>(&mut self, obj: A, value: Option<T>)`

- <span id="attr-set-if-none"></span>`fn set_if_none(&mut self, value: T)`

- <span id="attr-get"></span>`fn get(self) -> Option<T>`

- <span id="attr-get-with-tokens"></span>`fn get_with_tokens(self) -> Option<(TokenStream, T)>`

### `BoolAttr<'c>`

```rust
struct BoolAttr<'c>(Attr<'c, ()>);
```

#### Implementations

- <span id="boolattr-none"></span>`fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../index.md), [`Symbol`](../symbol/index.md)

- <span id="boolattr-set-true"></span>`fn set_true<A: ToTokens>(&mut self, obj: A)`

- <span id="boolattr-get"></span>`fn get(&self) -> bool`

### `VecAttr<'c, T>`

```rust
struct VecAttr<'c, T> {
    cx: &'c crate::internals::Ctxt,
    name: Symbol,
    first_dup_tokens: proc_macro2::TokenStream,
    values: Vec<T>,
}
```

#### Implementations

- <span id="vecattr-none"></span>`fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../index.md), [`Symbol`](../symbol/index.md)

- <span id="vecattr-insert"></span>`fn insert<A: ToTokens>(&mut self, obj: A, value: T)`

- <span id="vecattr-at-most-one"></span>`fn at_most_one(self) -> Option<T>`

- <span id="vecattr-get"></span>`fn get(self) -> Vec<T>`

### `RenameAllRules`

```rust
struct RenameAllRules {
    pub serialize: RenameRule,
    pub deserialize: RenameRule,
}
```

#### Implementations

- <span id="renameallrules-or"></span>`fn or(self, other_rules: Self) -> Self`

#### Trait Implementations

##### `impl Clone for RenameAllRules`

- <span id="renameallrules-clone"></span>`fn clone(&self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

##### `impl Copy for RenameAllRules`

### `Container`

```rust
struct Container {
    name: crate::internals::name::MultiName,
    transparent: bool,
    deny_unknown_fields: bool,
    default: Default,
    rename_all_rules: RenameAllRules,
    rename_all_fields_rules: RenameAllRules,
    ser_bound: Option<Vec<syn::WherePredicate>>,
    de_bound: Option<Vec<syn::WherePredicate>>,
    tag: TagType,
    type_from: Option<syn::Type>,
    type_try_from: Option<syn::Type>,
    type_into: Option<syn::Type>,
    remote: Option<syn::Path>,
    identifier: Identifier,
    serde_path: Option<syn::Path>,
    is_packed: bool,
    expecting: Option<String>,
    non_exhaustive: bool,
}
```

Represents struct or enum attribute information.

#### Fields

- **`expecting`**: `Option<String>`

  Error message generated when type can't be deserialized

#### Implementations

- <span id="container-from-ast"></span>`fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Self` — [`Ctxt`](../index.md)

- <span id="container-name"></span>`fn name(&self) -> &MultiName` — [`MultiName`](../name/index.md)

- <span id="container-rename-all-rules"></span>`fn rename_all_rules(&self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- <span id="container-rename-all-fields-rules"></span>`fn rename_all_fields_rules(&self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- <span id="container-transparent"></span>`fn transparent(&self) -> bool`

- <span id="container-deny-unknown-fields"></span>`fn deny_unknown_fields(&self) -> bool`

- <span id="container-default"></span>`fn default(&self) -> &Default` — [`Default`](#default)

- <span id="container-ser-bound"></span>`fn ser_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="container-de-bound"></span>`fn de_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="container-tag"></span>`fn tag(&self) -> &TagType` — [`TagType`](#tagtype)

- <span id="container-type-from"></span>`fn type_from(&self) -> Option<&syn::Type>`

- <span id="container-type-try-from"></span>`fn type_try_from(&self) -> Option<&syn::Type>`

- <span id="container-type-into"></span>`fn type_into(&self) -> Option<&syn::Type>`

- <span id="container-remote"></span>`fn remote(&self) -> Option<&syn::Path>`

- <span id="container-is-packed"></span>`fn is_packed(&self) -> bool`

- <span id="container-identifier"></span>`fn identifier(&self) -> Identifier` — [`Identifier`](#identifier)

- <span id="container-custom-serde-path"></span>`fn custom_serde_path(&self) -> Option<&syn::Path>`

- <span id="container-expecting"></span>`fn expecting(&self) -> Option<&str>`

- <span id="container-non-exhaustive"></span>`fn non_exhaustive(&self) -> bool`

### `Variant`

```rust
struct Variant {
    name: crate::internals::name::MultiName,
    rename_all_rules: RenameAllRules,
    ser_bound: Option<Vec<syn::WherePredicate>>,
    de_bound: Option<Vec<syn::WherePredicate>>,
    skip_deserializing: bool,
    skip_serializing: bool,
    other: bool,
    serialize_with: Option<syn::ExprPath>,
    deserialize_with: Option<syn::ExprPath>,
    borrow: Option<BorrowAttribute>,
    untagged: bool,
}
```

Represents variant attribute information

#### Implementations

- <span id="variant-from-ast"></span>`fn from_ast(cx: &Ctxt, variant: &syn::Variant) -> Self` — [`Ctxt`](../index.md)

- <span id="variant-name"></span>`fn name(&self) -> &MultiName` — [`MultiName`](../name/index.md)

- <span id="variant-aliases"></span>`fn aliases(&self) -> &BTreeSet<Name>` — [`Name`](../name/index.md)

- <span id="variant-rename-by-rules"></span>`fn rename_by_rules(&mut self, rules: RenameAllRules)` — [`RenameAllRules`](#renameallrules)

- <span id="variant-rename-all-rules"></span>`fn rename_all_rules(&self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- <span id="variant-ser-bound"></span>`fn ser_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="variant-de-bound"></span>`fn de_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="variant-skip-deserializing"></span>`fn skip_deserializing(&self) -> bool`

- <span id="variant-skip-serializing"></span>`fn skip_serializing(&self) -> bool`

- <span id="variant-other"></span>`fn other(&self) -> bool`

- <span id="variant-serialize-with"></span>`fn serialize_with(&self) -> Option<&syn::ExprPath>`

- <span id="variant-deserialize-with"></span>`fn deserialize_with(&self) -> Option<&syn::ExprPath>`

- <span id="variant-untagged"></span>`fn untagged(&self) -> bool`

### `BorrowAttribute`

```rust
struct BorrowAttribute {
    path: syn::Path,
    lifetimes: Option<std::collections::BTreeSet<syn::Lifetime>>,
}
```

### `Field`

```rust
struct Field {
    name: crate::internals::name::MultiName,
    skip_serializing: bool,
    skip_deserializing: bool,
    skip_serializing_if: Option<syn::ExprPath>,
    default: Default,
    serialize_with: Option<syn::ExprPath>,
    deserialize_with: Option<syn::ExprPath>,
    ser_bound: Option<Vec<syn::WherePredicate>>,
    de_bound: Option<Vec<syn::WherePredicate>>,
    borrowed_lifetimes: std::collections::BTreeSet<syn::Lifetime>,
    getter: Option<syn::ExprPath>,
    flatten: bool,
    transparent: bool,
}
```

Represents field attribute information

#### Implementations

- <span id="field-from-ast"></span>`fn from_ast(cx: &Ctxt, index: usize, field: &syn::Field, attrs: Option<&Variant>, container_default: &Default, private: &Ident) -> Self` — [`Ctxt`](../index.md), [`Variant`](#variant), [`Default`](#default)

- <span id="field-name"></span>`fn name(&self) -> &MultiName` — [`MultiName`](../name/index.md)

- <span id="field-aliases"></span>`fn aliases(&self) -> &BTreeSet<Name>` — [`Name`](../name/index.md)

- <span id="field-rename-by-rules"></span>`fn rename_by_rules(&mut self, rules: RenameAllRules)` — [`RenameAllRules`](#renameallrules)

- <span id="field-skip-serializing"></span>`fn skip_serializing(&self) -> bool`

- <span id="field-skip-deserializing"></span>`fn skip_deserializing(&self) -> bool`

- <span id="field-skip-serializing-if"></span>`fn skip_serializing_if(&self) -> Option<&syn::ExprPath>`

- <span id="field-default"></span>`fn default(&self) -> &Default` — [`Default`](#default)

- <span id="field-serialize-with"></span>`fn serialize_with(&self) -> Option<&syn::ExprPath>`

- <span id="field-deserialize-with"></span>`fn deserialize_with(&self) -> Option<&syn::ExprPath>`

- <span id="field-ser-bound"></span>`fn ser_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="field-de-bound"></span>`fn de_bound(&self) -> Option<&[syn::WherePredicate]>`

- <span id="field-borrowed-lifetimes"></span>`fn borrowed_lifetimes(&self) -> &BTreeSet<syn::Lifetime>`

- <span id="field-getter"></span>`fn getter(&self) -> Option<&syn::ExprPath>`

- <span id="field-flatten"></span>`fn flatten(&self) -> bool`

- <span id="field-transparent"></span>`fn transparent(&self) -> bool`

- <span id="field-mark-transparent"></span>`fn mark_transparent(&mut self)`

## Enums

### `RenameRule`

```rust
enum RenameRule {
    None,
    LowerCase,
    UpperCase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}
```

The different possible ways to change case of fields in a struct, or variants in an enum.

#### Variants

- **`None`**

  Don't apply a default rename rule.

- **`LowerCase`**

  Rename direct children to "lowercase" style.

- **`UpperCase`**

  Rename direct children to "UPPERCASE" style.

- **`PascalCase`**

  Rename direct children to "PascalCase" style, as typically used for
  enum variants.

- **`CamelCase`**

  Rename direct children to "camelCase" style.

- **`SnakeCase`**

  Rename direct children to "snake_case" style, as commonly used for
  fields.

- **`ScreamingSnakeCase`**

  Rename direct children to "SCREAMING_SNAKE_CASE" style, as commonly
  used for constants.

- **`KebabCase`**

  Rename direct children to "kebab-case" style.

- **`ScreamingKebabCase`**

  Rename direct children to "SCREAMING-KEBAB-CASE" style.

#### Implementations

- <span id="renamerule-from-str"></span>`fn from_str(rename_all_str: &str) -> Result<Self, ParseError<'_>>` — [`ParseError`](../case/index.md)

- <span id="renamerule-apply-to-variant"></span>`fn apply_to_variant(self, variant: &str) -> String`

- <span id="renamerule-apply-to-field"></span>`fn apply_to_field(self, field: &str) -> String`

- <span id="renamerule-or"></span>`fn or(self, rule_b: Self) -> Self`

#### Trait Implementations

##### `impl Clone for RenameRule`

- <span id="renamerule-clone"></span>`fn clone(&self) -> RenameRule` — [`RenameRule`](../case/index.md)

##### `impl Copy for RenameRule`

##### `impl PartialEq for RenameRule`

- <span id="renamerule-eq"></span>`fn eq(&self, other: &RenameRule) -> bool` — [`RenameRule`](../case/index.md)

##### `impl StructuralPartialEq for RenameRule`

### `TagType`

```rust
enum TagType {
    External,
    Internal {
        tag: String,
    },
    Adjacent {
        tag: String,
        content: String,
    },
    None,
}
```

Styles of representing an enum.

#### Variants

- **`External`**

  The default.
  
  ```json
  {"variant1": {"key1": "value1", "key2": "value2"}}
  ```

- **`Internal`**

  `#[serde(tag = "type")]`
  
  ```json
  {"type": "variant1", "key1": "value1", "key2": "value2"}
  ```

- **`Adjacent`**

  `#[serde(tag = "t", content = "c")]`
  
  ```json
  {"t": "variant1", "c": {"key1": "value1", "key2": "value2"}}
  ```

- **`None`**

  `#[serde(untagged)]`
  
  ```json
  {"key1": "value1", "key2": "value2"}
  ```

### `Identifier`

```rust
enum Identifier {
    No,
    Field,
    Variant,
}
```

Whether this enum represents the fields of a struct or the variants of an
enum.

#### Variants

- **`No`**

  It does not.

- **`Field`**

  This enum represents the fields of a struct. All of the variants must be
  unit variants, except possibly one which is annotated with
  `#[serde(other)]` and is a newtype variant.

- **`Variant`**

  This enum represents the variants of an enum. All of the variants must
  be unit variants.

#### Trait Implementations

##### `impl Clone for Identifier`

- <span id="identifier-clone"></span>`fn clone(&self) -> Identifier` — [`Identifier`](#identifier)

##### `impl Copy for Identifier`

### `Default`

```rust
enum Default {
    None,
    Default,
    Path(syn::ExprPath),
}
```

Represents the default to use for a field when deserializing.

#### Variants

- **`None`**

  Field must always be specified because it does not have a default.

- **`Default`**

  The default is given by `std::default::Default::default()`.

- **`Path`**

  The default is given by this function.

#### Implementations

- <span id="default-is-none"></span>`fn is_none(&self) -> bool`

## Functions

### `unraw`

```rust
fn unraw(ident: &syn::Ident) -> syn::Ident
```

### `decide_tag`

```rust
fn decide_tag(cx: &crate::internals::Ctxt, item: &syn::DeriveInput, untagged: BoolAttr<'_>, internal_tag: Attr<'_, String>, content: Attr<'_, String>) -> TagType
```

### `decide_identifier`

```rust
fn decide_identifier(cx: &crate::internals::Ctxt, item: &syn::DeriveInput, field_identifier: BoolAttr<'_>, variant_identifier: BoolAttr<'_>) -> Identifier
```

### `get_ser_and_de`

```rust
fn get_ser_and_de<'c, T, F, R>(cx: &'c crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>, f: F) -> syn::Result<(VecAttr<'c, T>, VecAttr<'c, T>)>
where
    T: Clone,
    F: Fn(&crate::internals::Ctxt, Symbol, Symbol, &syn::meta::ParseNestedMeta<'_>) -> syn::Result<R>,
    R: Into<Option<T>>
```

### `get_renames`

```rust
fn get_renames(cx: &crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<(Option<syn::LitStr>, Option<syn::LitStr>)>
```

### `get_multiple_renames`

```rust
fn get_multiple_renames(cx: &crate::internals::Ctxt, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<(Option<syn::LitStr>, Vec<syn::LitStr>)>
```

### `get_where_predicates`

```rust
fn get_where_predicates(cx: &crate::internals::Ctxt, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<(Option<Vec<syn::WherePredicate>>, Option<Vec<syn::WherePredicate>>)>
```

### `get_lit_str`

```rust
fn get_lit_str(cx: &crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Option<syn::LitStr>>
```

### `get_lit_str2`

```rust
fn get_lit_str2(cx: &crate::internals::Ctxt, attr_name: Symbol, meta_item_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Option<syn::LitStr>>
```

### `parse_lit_into_path`

```rust
fn parse_lit_into_path(cx: &crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Option<syn::Path>>
```

### `parse_lit_into_expr_path`

```rust
fn parse_lit_into_expr_path(cx: &crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Option<syn::ExprPath>>
```

### `parse_lit_into_where`

```rust
fn parse_lit_into_where(cx: &crate::internals::Ctxt, attr_name: Symbol, meta_item_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Vec<syn::WherePredicate>>
```

### `parse_lit_into_ty`

```rust
fn parse_lit_into_ty(cx: &crate::internals::Ctxt, attr_name: Symbol, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Option<syn::Type>>
```

### `parse_lit_into_lifetimes`

```rust
fn parse_lit_into_lifetimes(cx: &crate::internals::Ctxt, meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<std::collections::BTreeSet<syn::Lifetime>>
```

### `is_implicitly_borrowed`

```rust
fn is_implicitly_borrowed(ty: &syn::Type) -> bool
```

### `is_implicitly_borrowed_reference`

```rust
fn is_implicitly_borrowed_reference(ty: &syn::Type) -> bool
```

### `is_cow`

```rust
fn is_cow(ty: &syn::Type, elem: fn(&syn::Type) -> bool) -> bool
```

### `is_option`

```rust
fn is_option(ty: &syn::Type, elem: fn(&syn::Type) -> bool) -> bool
```

### `is_reference`

```rust
fn is_reference(ty: &syn::Type, elem: fn(&syn::Type) -> bool) -> bool
```

### `is_str`

```rust
fn is_str(ty: &syn::Type) -> bool
```

### `is_slice_u8`

```rust
fn is_slice_u8(ty: &syn::Type) -> bool
```

### `is_primitive_type`

```rust
fn is_primitive_type(ty: &syn::Type, primitive: &str) -> bool
```

### `is_primitive_path`

```rust
fn is_primitive_path(path: &syn::Path, primitive: &str) -> bool
```

### `borrowable_lifetimes`

```rust
fn borrowable_lifetimes(cx: &crate::internals::Ctxt, name: &crate::internals::name::Name, field: &syn::Field) -> Result<std::collections::BTreeSet<syn::Lifetime>, ()>
```

### `collect_lifetimes`

```rust
fn collect_lifetimes(ty: &syn::Type, out: &mut std::collections::BTreeSet<syn::Lifetime>)
```

### `collect_lifetimes_from_tokens`

```rust
fn collect_lifetimes_from_tokens(tokens: proc_macro2::TokenStream, out: &mut std::collections::BTreeSet<syn::Lifetime>)
```

## Type Aliases

### `SerAndDe<T>`

```rust
type SerAndDe<T> = (Option<T>, Option<T>);
```

