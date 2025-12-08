*[serde_derive](../../index.md) / [internals](../index.md) / [attr](index.md)*

---

# Module `attr`

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

- `fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../ctxt/index.md), [`Symbol`](../symbol/index.md)

- `fn set<A: ToTokens>(self: &mut Self, obj: A, value: T)`

- `fn set_opt<A: ToTokens>(self: &mut Self, obj: A, value: Option<T>)`

- `fn set_if_none(self: &mut Self, value: T)`

- `fn get(self: Self) -> Option<T>`

- `fn get_with_tokens(self: Self) -> Option<(TokenStream, T)>`

### `BoolAttr<'c>`

```rust
struct BoolAttr<'c>(Attr<'c, ()>);
```

#### Implementations

- `fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../ctxt/index.md), [`Symbol`](../symbol/index.md)

- `fn set_true<A: ToTokens>(self: &mut Self, obj: A)`

- `fn get(self: &Self) -> bool`

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

- `fn none(cx: &'c Ctxt, name: Symbol) -> Self` — [`Ctxt`](../ctxt/index.md), [`Symbol`](../symbol/index.md)

- `fn insert<A: ToTokens>(self: &mut Self, obj: A, value: T)`

- `fn at_most_one(self: Self) -> Option<T>`

- `fn get(self: Self) -> Vec<T>`

### `RenameAllRules`

```rust
struct RenameAllRules {
    pub serialize: RenameRule,
    pub deserialize: RenameRule,
}
```

#### Implementations

- `fn or(self: Self, other_rules: Self) -> Self`

#### Trait Implementations

##### `impl Clone for RenameAllRules`

- `fn clone(self: &Self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

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

- `fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Self` — [`Ctxt`](../ctxt/index.md)

- `fn name(self: &Self) -> &MultiName` — [`MultiName`](../name/index.md)

- `fn rename_all_rules(self: &Self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- `fn rename_all_fields_rules(self: &Self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- `fn transparent(self: &Self) -> bool`

- `fn deny_unknown_fields(self: &Self) -> bool`

- `fn default(self: &Self) -> &Default` — [`Default`](#default)

- `fn ser_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn de_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn tag(self: &Self) -> &TagType` — [`TagType`](#tagtype)

- `fn type_from(self: &Self) -> Option<&syn::Type>`

- `fn type_try_from(self: &Self) -> Option<&syn::Type>`

- `fn type_into(self: &Self) -> Option<&syn::Type>`

- `fn remote(self: &Self) -> Option<&syn::Path>`

- `fn is_packed(self: &Self) -> bool`

- `fn identifier(self: &Self) -> Identifier` — [`Identifier`](#identifier)

- `fn custom_serde_path(self: &Self) -> Option<&syn::Path>`

- `fn expecting(self: &Self) -> Option<&str>`

- `fn non_exhaustive(self: &Self) -> bool`

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

- `fn from_ast(cx: &Ctxt, variant: &syn::Variant) -> Self` — [`Ctxt`](../ctxt/index.md)

- `fn name(self: &Self) -> &MultiName` — [`MultiName`](../name/index.md)

- `fn aliases(self: &Self) -> &BTreeSet<Name>` — [`Name`](../name/index.md)

- `fn rename_by_rules(self: &mut Self, rules: RenameAllRules)` — [`RenameAllRules`](#renameallrules)

- `fn rename_all_rules(self: &Self) -> RenameAllRules` — [`RenameAllRules`](#renameallrules)

- `fn ser_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn de_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn skip_deserializing(self: &Self) -> bool`

- `fn skip_serializing(self: &Self) -> bool`

- `fn other(self: &Self) -> bool`

- `fn serialize_with(self: &Self) -> Option<&syn::ExprPath>`

- `fn deserialize_with(self: &Self) -> Option<&syn::ExprPath>`

- `fn untagged(self: &Self) -> bool`

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

- `fn from_ast(cx: &Ctxt, index: usize, field: &syn::Field, attrs: Option<&Variant>, container_default: &Default, private: &Ident) -> Self` — [`Ctxt`](../ctxt/index.md), [`Variant`](#variant), [`Default`](#default)

- `fn name(self: &Self) -> &MultiName` — [`MultiName`](../name/index.md)

- `fn aliases(self: &Self) -> &BTreeSet<Name>` — [`Name`](../name/index.md)

- `fn rename_by_rules(self: &mut Self, rules: RenameAllRules)` — [`RenameAllRules`](#renameallrules)

- `fn skip_serializing(self: &Self) -> bool`

- `fn skip_deserializing(self: &Self) -> bool`

- `fn skip_serializing_if(self: &Self) -> Option<&syn::ExprPath>`

- `fn default(self: &Self) -> &Default` — [`Default`](#default)

- `fn serialize_with(self: &Self) -> Option<&syn::ExprPath>`

- `fn deserialize_with(self: &Self) -> Option<&syn::ExprPath>`

- `fn ser_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn de_bound(self: &Self) -> Option<&[syn::WherePredicate]>`

- `fn borrowed_lifetimes(self: &Self) -> &BTreeSet<syn::Lifetime>`

- `fn getter(self: &Self) -> Option<&syn::ExprPath>`

- `fn flatten(self: &Self) -> bool`

- `fn transparent(self: &Self) -> bool`

- `fn mark_transparent(self: &mut Self)`

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

- `fn from_str(rename_all_str: &str) -> Result<Self, ParseError<'_>>` — [`ParseError`](../case/index.md)

- `fn apply_to_variant(self: Self, variant: &str) -> String`

- `fn apply_to_field(self: Self, field: &str) -> String`

- `fn or(self: Self, rule_b: Self) -> Self`

#### Trait Implementations

##### `impl Clone for RenameRule`

- `fn clone(self: &Self) -> RenameRule` — [`RenameRule`](../case/index.md)

##### `impl Copy for RenameRule`

##### `impl PartialEq for RenameRule`

- `fn eq(self: &Self, other: &RenameRule) -> bool` — [`RenameRule`](../case/index.md)

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

- `fn clone(self: &Self) -> Identifier` — [`Identifier`](#identifier)

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

- `fn is_none(self: &Self) -> bool`

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

