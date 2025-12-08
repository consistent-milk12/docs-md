*[clap_derive](../index.md) / [item](index.md)*

---

# Module `item`

## Structs

### `Item`

```rust
struct Item {
    name: Name,
    casing: self::spanned::Sp<CasingStyle>,
    env_casing: self::spanned::Sp<CasingStyle>,
    ty: Option<syn::Type>,
    doc_comment: Vec<Method>,
    methods: Vec<Method>,
    deprecations: Vec<Deprecation>,
    value_parser: Option<ValueParser>,
    action: Option<Action>,
    verbatim_doc_comment: bool,
    force_long_help: bool,
    next_display_order: Option<Method>,
    next_help_heading: Option<Method>,
    is_enum: bool,
    is_positional: bool,
    skip_group: bool,
    group_id: Name,
    group_methods: Vec<Method>,
    kind: self::spanned::Sp<Kind>,
}
```

#### Implementations

- `fn from_args_struct(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- `fn from_subcommand_enum(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- `fn from_value_enum(input: &DeriveInput, name: Name) -> Result<Self, syn::Error>` — [`Name`](#name)

- `fn from_subcommand_variant(variant: &Variant, struct_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle)

- `fn from_value_enum_variant(variant: &Variant, argument_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle)

- `fn from_args_field(field: &Field, struct_casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>) -> Result<Self, syn::Error>` — [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle)

- `fn new(name: Name, ident: Ident, ty: Option<Type>, casing: Sp<CasingStyle>, env_casing: Sp<CasingStyle>, kind: Sp<Kind>) -> Self` — [`Name`](#name), [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle), [`Kind`](#kind)

- `fn push_method(self: &mut Self, kind: AttrKind, name: Ident, arg: impl ToTokens)` — [`AttrKind`](../attr/index.md)

- `fn push_method_(self: &mut Self, kind: AttrKind, name: Ident, arg: TokenStream)` — [`AttrKind`](../attr/index.md)

- `fn infer_kind(self: &mut Self, attrs: &[ClapAttr]) -> Result<(), syn::Error>` — [`ClapAttr`](../attr/index.md)

- `fn push_attrs(self: &mut Self, attrs: &[ClapAttr]) -> Result<(), syn::Error>` — [`ClapAttr`](../attr/index.md)

- `fn push_doc_comment(self: &mut Self, attrs: &[Attribute], short_name: &str, long_name: Option<&str>)`

- `fn set_kind(self: &mut Self, kind: Sp<Kind>) -> Result<(), syn::Error>` — [`Sp`](../utils/spanned/index.md), [`Kind`](#kind)

- `fn find_default_method(self: &Self) -> Option<&Method>` — [`Method`](#method)

- `fn initial_top_level_methods(self: &Self) -> TokenStream`

- `fn final_top_level_methods(self: &Self) -> TokenStream`

- `fn field_methods(self: &Self) -> TokenStream`

- `fn group_id(self: &Self) -> &Name` — [`Name`](#name)

- `fn group_methods(self: &Self) -> TokenStream`

- `fn deprecations(self: &Self) -> TokenStream`

- `fn next_display_order(self: &Self) -> TokenStream`

- `fn next_help_heading(self: &Self) -> TokenStream`

- `fn id(self: &Self) -> &Name` — [`Name`](#name)

- `fn cased_name(self: &Self) -> TokenStream`

- `fn value_name(self: &Self) -> TokenStream`

- `fn value_parser(self: &Self, field_type: &Type) -> Method` — [`Method`](#method)

- `fn action(self: &Self, field_type: &Type) -> Method` — [`Method`](#method)

- `fn kind(self: &Self) -> Sp<Kind>` — [`Sp`](../utils/spanned/index.md), [`Kind`](#kind)

- `fn is_positional(self: &Self) -> bool`

- `fn casing(self: &Self) -> Sp<CasingStyle>` — [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle)

- `fn env_casing(self: &Self) -> Sp<CasingStyle>` — [`Sp`](../utils/spanned/index.md), [`CasingStyle`](#casingstyle)

- `fn has_explicit_methods(self: &Self) -> bool`

- `fn skip_group(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Item`

- `fn clone(self: &Self) -> Item` — [`Item`](#item)

### `Method`

```rust
struct Method {
    name: syn::Ident,
    args: proc_macro2::TokenStream,
}
```

#### Implementations

- `fn new(name: Ident, args: TokenStream) -> Self`

- `fn from_env(ident: Ident, env_var: &str) -> Result<Option<Self>, syn::Error>`

- `fn args(self: &Self) -> &TokenStream`

#### Trait Implementations

##### `impl Clone for Method`

- `fn clone(self: &Self) -> Method` — [`Method`](#method)

##### `impl<T> Spanned for Method`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Method`

- `fn to_tokens(self: &Self, ts: &mut TokenStream)`

### `Deprecation`

```rust
struct Deprecation {
    span: proc_macro2::Span,
    id: &'static str,
    version: &'static str,
    description: String,
}
```

#### Implementations

- `fn attribute(version: &'static str, old: AttrKind, new: AttrKind, span: Span) -> Self` — [`AttrKind`](../attr/index.md)

#### Trait Implementations

##### `impl Clone for Deprecation`

- `fn clone(self: &Self) -> Deprecation` — [`Deprecation`](#deprecation)

##### `impl<T> Spanned for Deprecation`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Deprecation`

- `fn to_tokens(self: &Self, ts: &mut TokenStream)`

## Enums

### `ValueParser`

```rust
enum ValueParser {
    Explicit(Method),
    Implicit(syn::Ident),
}
```

#### Implementations

- `fn resolve(self: Self, _inner_type: &Type) -> Method` — [`Method`](#method)

- `fn span(self: &Self) -> Span`

#### Trait Implementations

##### `impl Clone for ValueParser`

- `fn clone(self: &Self) -> ValueParser` — [`ValueParser`](#valueparser)

### `Action`

```rust
enum Action {
    Explicit(Method),
    Implicit(syn::Ident),
}
```

#### Implementations

- `fn resolve(self: Self, _field_type: &Type) -> Method` — [`Method`](#method)

- `fn span(self: &Self) -> Span`

#### Trait Implementations

##### `impl Clone for Action`

- `fn clone(self: &Self) -> Action` — [`Action`](#action)

### `Kind`

```rust
enum Kind {
    Arg(self::spanned::Sp<self::ty::Ty>),
    Command(self::spanned::Sp<self::ty::Ty>),
    Value,
    FromGlobal(self::spanned::Sp<self::ty::Ty>),
    Subcommand(self::spanned::Sp<self::ty::Ty>),
    Flatten(self::spanned::Sp<self::ty::Ty>),
    Skip(Option<crate::attr::AttrValue>, crate::attr::AttrKind),
    ExternalSubcommand,
}
```

#### Implementations

- `fn name(self: &Self) -> &'static str`

- `fn attr_kind(self: &Self) -> AttrKind` — [`AttrKind`](../attr/index.md)

- `fn ty(self: &Self) -> Option<&Sp<Ty>>` — [`Sp`](../utils/spanned/index.md), [`Ty`](../utils/ty/index.md)

#### Trait Implementations

##### `impl Clone for Kind`

- `fn clone(self: &Self) -> Kind` — [`Kind`](#kind)

### `CasingStyle`

```rust
enum CasingStyle {
    Camel,
    Kebab,
    Pascal,
    ScreamingSnake,
    Snake,
    Lower,
    Upper,
    Verbatim,
}
```

Defines the casing for the attributes long representation.

#### Variants

- **`Camel`**

  Indicate word boundaries with uppercase letter, excluding the first word.

- **`Kebab`**

  Keep all letters lowercase and indicate word boundaries with hyphens.

- **`Pascal`**

  Indicate word boundaries with uppercase letter, including the first word.

- **`ScreamingSnake`**

  Keep all letters uppercase and indicate word boundaries with underscores.

- **`Snake`**

  Keep all letters lowercase and indicate word boundaries with underscores.

- **`Lower`**

  Keep all letters lowercase and remove word boundaries.

- **`Upper`**

  Keep all letters uppercase and remove word boundaries.

- **`Verbatim`**

  Use the original attribute name defined in the code.

#### Implementations

- `fn from_lit(name: &LitStr) -> Result<Sp<Self>, syn::Error>` — [`Sp`](../utils/spanned/index.md)

#### Trait Implementations

##### `impl Clone for CasingStyle`

- `fn clone(self: &Self) -> CasingStyle` — [`CasingStyle`](#casingstyle)

##### `impl Copy for CasingStyle`

##### `impl Debug for CasingStyle`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for CasingStyle`

##### `impl PartialEq for CasingStyle`

- `fn eq(self: &Self, other: &CasingStyle) -> bool` — [`CasingStyle`](#casingstyle)

##### `impl StructuralPartialEq for CasingStyle`

### `Name`

```rust
enum Name {
    Derived(syn::Ident),
    Assigned(proc_macro2::TokenStream),
}
```

#### Implementations

- `fn translate(self: Self, style: CasingStyle) -> TokenStream` — [`CasingStyle`](#casingstyle)

- `fn translate_char(self: Self, style: CasingStyle) -> TokenStream` — [`CasingStyle`](#casingstyle)

#### Trait Implementations

##### `impl Clone for Name`

- `fn clone(self: &Self) -> Name` — [`Name`](#name)

##### `impl<T> Spanned for Name`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for Name`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

## Functions

### `default_value_parser`

```rust
fn default_value_parser(inner_type: &syn::Type, span: proc_macro2::Span) -> Method
```

### `default_action`

```rust
fn default_action(field_type: &syn::Type, span: proc_macro2::Span) -> Method
```

### `assert_attr_kind`

```rust
fn assert_attr_kind(attr: &crate::attr::ClapAttr, possible_kind: &[crate::attr::AttrKind]) -> Result<(), syn::Error>
```

### `process_author_str`

```rust
fn process_author_str(author: &str) -> String
```

replace all `:` with `, ` when not inside the `<>`

`"author1:author2:author3" => "author1, author2, author3"`
`"author1 <http://website1.com>:author2" => "author1 <http://website1.com>, author2"`

## Constants

### `DEFAULT_CASING`

```rust
const DEFAULT_CASING: CasingStyle;
```

Default casing style for generated arguments.

### `DEFAULT_ENV_CASING`

```rust
const DEFAULT_ENV_CASING: CasingStyle;
```

Default casing style for environment variables

