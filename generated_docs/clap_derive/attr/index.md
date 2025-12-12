*[clap_derive](../index.md) / [attr](index.md)*

---

# Module `attr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ClapAttr`](#clapattr) | struct |  |
| [`MagicAttrName`](#magicattrname) | enum |  |
| [`AttrValue`](#attrvalue) | enum |  |
| [`AttrKind`](#attrkind) | enum |  |

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

*Defined in [`clap_derive-4.5.49/src/attr.rs:17-22`](../../../.source_1765521767/clap_derive-4.5.49/src/attr.rs#L17-L22)*

#### Implementations

- <span id="clapattr-parse-all"></span>`fn parse_all(all_attrs: &[Attribute]) -> Result<Vec<Self>, syn::Error>`

- <span id="clapattr-value-or-abort"></span>`fn value_or_abort(&self) -> Result<&AttrValue, syn::Error>` — [`AttrValue`](#attrvalue)

- <span id="clapattr-lit-str-or-abort"></span>`fn lit_str_or_abort(&self) -> Result<&LitStr, syn::Error>`

#### Trait Implementations

##### `impl Clone for ClapAttr`

- <span id="clapattr-clone"></span>`fn clone(&self) -> ClapAttr` — [`ClapAttr`](#clapattr)

##### `impl Parse for ClapAttr`

- <span id="clapattr-parse"></span>`fn parse(input: ParseStream<'_>) -> syn::Result<Self>`

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

*Defined in [`clap_derive-4.5.49/src/attr.rs:145-171`](../../../.source_1765521767/clap_derive-4.5.49/src/attr.rs#L145-L171)*

#### Trait Implementations

##### `impl Clone for MagicAttrName`

- <span id="magicattrname-clone"></span>`fn clone(&self) -> MagicAttrName` — [`MagicAttrName`](#magicattrname)

##### `impl Copy for MagicAttrName`

##### `impl Eq for MagicAttrName`

##### `impl PartialEq for MagicAttrName`

- <span id="magicattrname-eq"></span>`fn eq(&self, other: &MagicAttrName) -> bool` — [`MagicAttrName`](#magicattrname)

##### `impl StructuralPartialEq for MagicAttrName`

### `AttrValue`

```rust
enum AttrValue {
    LitStr(syn::LitStr),
    Expr(syn::Expr),
    Call(Vec<syn::Expr>),
}
```

*Defined in [`clap_derive-4.5.49/src/attr.rs:175-179`](../../../.source_1765521767/clap_derive-4.5.49/src/attr.rs#L175-L179)*

#### Trait Implementations

##### `impl Clone for AttrValue`

- <span id="attrvalue-clone"></span>`fn clone(&self) -> AttrValue` — [`AttrValue`](#attrvalue)

##### `impl Spanned for AttrValue`

- <span id="attrvalue-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for AttrValue`

- <span id="attrvalue-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

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

*Defined in [`clap_derive-4.5.49/src/attr.rs:195-202`](../../../.source_1765521767/clap_derive-4.5.49/src/attr.rs#L195-L202)*

#### Implementations

- <span id="attrkind-as-str"></span>`fn as_str(&self) -> &'static str`

#### Trait Implementations

##### `impl Clone for AttrKind`

- <span id="attrkind-clone"></span>`fn clone(&self) -> AttrKind` — [`AttrKind`](#attrkind)

##### `impl Copy for AttrKind`

##### `impl Eq for AttrKind`

##### `impl PartialEq for AttrKind`

- <span id="attrkind-eq"></span>`fn eq(&self, other: &AttrKind) -> bool` — [`AttrKind`](#attrkind)

##### `impl StructuralPartialEq for AttrKind`

