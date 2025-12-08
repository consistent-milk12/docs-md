*[clap_derive](../index.md) / [attr](index.md)*

---

# Module `attr`

## Structs

### `ClapAttr`

```rust
struct ClapAttr {
    kind: self::spanned::Sp<AttrKind>,
    name: syn::Ident,
    magic: Option<MagicAttrName>,
    value: Option<AttrValue>,
}
```

#### Implementations

- `fn parse_all(all_attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error>`

- `fn value_or_abort(self: &Self) -> Result<&AttrValue, syn::Error>` — [`AttrValue`](#attrvalue)

- `fn lit_str_or_abort(self: &Self) -> Result<&LitStr, syn::Error>`

#### Trait Implementations

##### `impl Clone for ClapAttr`

- `fn clone(self: &Self) -> ClapAttr` — [`ClapAttr`](#clapattr)

##### `impl Parse for ClapAttr`

- `fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

## Enums

### `MagicAttrName`

```rust
enum MagicAttrName {
    Short,
    Long,
    ValueParser,
    Action,
    Env,
    Flatten,
    ValueEnum,
    FromGlobal,
    Subcommand,
    VerbatimDocComment,
    ExternalSubcommand,
    About,
    LongAbout,
    LongHelp,
    Author,
    Version,
    RenameAllEnv,
    RenameAll,
    Skip,
    DefaultValueT,
    DefaultValuesT,
    DefaultValueOsT,
    DefaultValuesOsT,
    NextDisplayOrder,
    NextHelpHeading,
}
```

#### Trait Implementations

##### `impl Clone for MagicAttrName`

- `fn clone(self: &Self) -> MagicAttrName` — [`MagicAttrName`](#magicattrname)

##### `impl Copy for MagicAttrName`

##### `impl Eq for MagicAttrName`

##### `impl PartialEq for MagicAttrName`

- `fn eq(self: &Self, other: &MagicAttrName) -> bool` — [`MagicAttrName`](#magicattrname)

##### `impl StructuralPartialEq for MagicAttrName`

### `AttrValue`

```rust
enum AttrValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
    Call(Vec<syn::Expr>),
}
```

#### Trait Implementations

##### `impl Clone for AttrValue`

- `fn clone(self: &Self) -> AttrValue` — [`AttrValue`](#attrvalue)

##### `impl<T> Spanned for AttrValue`

- `fn span(self: &Self) -> Span`

##### `impl ToTokens for AttrValue`

- `fn to_tokens(self: &Self, tokens: &mut TokenStream)`

### `AttrKind`

```rust
enum AttrKind {
    Clap,
    StructOpt,
    Command,
    Group,
    Arg,
    Value,
}
```

#### Implementations

- `fn as_str(self: &Self) -> &'static str`

#### Trait Implementations

##### `impl Clone for AttrKind`

- `fn clone(self: &Self) -> AttrKind` — [`AttrKind`](#attrkind)

##### `impl Copy for AttrKind`

##### `impl Eq for AttrKind`

##### `impl PartialEq for AttrKind`

- `fn eq(self: &Self, other: &AttrKind) -> bool` — [`AttrKind`](#attrkind)

##### `impl StructuralPartialEq for AttrKind`

