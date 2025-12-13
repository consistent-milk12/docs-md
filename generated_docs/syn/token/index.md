*[syn](../index.md) / [token](index.md)*

---

# Module `token`

Tokens representing Rust punctuation, keywords, and delimiters.

The type names in this module can be difficult to keep straight, so we
prefer to use the `Token!` macro instead. This is a type-macro that
expands to the token type of the given token.

# Example

The [`ItemStatic`](../item/index.md) syntax tree node is defined like this.

```rust
use syn::{Attribute, Expr, Ident, Token, Type, Visibility};

pub struct ItemStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Token![static],
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Box<Type>,
    pub eq_token: Token![=],
    pub expr: Box<Expr>,
    pub semi_token: Token![;],
}
```

# Parsing

Keywords and punctuation can be parsed through the `ParseStream::parse`
method. Delimiter tokens are parsed using the `parenthesized!`,
`bracketed!` and `braced!` macros.




```rust
use syn::{Attribute, Result};
use syn::parse::{Parse, ParseStream};

enum ItemStatic {}

// Parse the ItemStatic struct shown above.
impl Parse for ItemStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::ItemStatic;
        fn parse(input: ParseStream) -> Result<ItemStatic> {
        Ok(ItemStatic {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            mutability: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            eq_token: input.parse()?,
            expr: input.parse()?,
            semi_token: input.parse()?,
        })
        }
        unimplemented!()
    }
}
```

# Other operations

Every keyword and punctuation token supports the following operations.

- [Peeking] — `input.peek(Token![...])`

- [Parsing] — `input.parse::<Token![...]>()?`

- [Printing] — `quote!( ... #the_token ... )`

- Construction from a `Span` — `let the_token = Token![...](sp)`

- Field access to its span — `let sp = the_token.span`





## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Underscore`](#underscore)
  - [`Group`](#group)
  - [`Abstract`](#abstract)
  - [`As`](#as)
  - [`Async`](#async)
  - [`Auto`](#auto)
  - [`Await`](#await)
  - [`Become`](#become)
  - [`Box`](#box)
  - [`Break`](#break)
  - [`Const`](#const)
  - [`Continue`](#continue)
  - [`Crate`](#crate)
  - [`Default`](#default)
  - [`Do`](#do)
  - [`Dyn`](#dyn)
  - [`Else`](#else)
  - [`Enum`](#enum)
  - [`Extern`](#extern)
  - [`Final`](#final)
  - [`Fn`](#fn)
  - [`For`](#for)
  - [`If`](#if)
  - [`Impl`](#impl)
  - [`In`](#in)
  - [`Let`](#let)
  - [`Loop`](#loop)
  - [`Macro`](#macro)
  - [`Match`](#match)
  - [`Mod`](#mod)
  - [`Move`](#move)
  - [`Mut`](#mut)
  - [`Override`](#override)
  - [`Priv`](#priv)
  - [`Pub`](#pub)
  - [`Raw`](#raw)
  - [`Ref`](#ref)
  - [`Return`](#return)
  - [`SelfType`](#selftype)
  - [`SelfValue`](#selfvalue)
  - [`Static`](#static)
  - [`Struct`](#struct)
  - [`Super`](#super)
  - [`Trait`](#trait)
  - [`Try`](#try)
  - [`Type`](#type)
  - [`Typeof`](#typeof)
  - [`Union`](#union)
  - [`Unsafe`](#unsafe)
  - [`Unsized`](#unsized)
  - [`Use`](#use)
  - [`Virtual`](#virtual)
  - [`Where`](#where)
  - [`While`](#while)
  - [`Yield`](#yield)
  - [`And`](#and)
  - [`AndAnd`](#andand)
  - [`AndEq`](#andeq)
  - [`At`](#at)
  - [`Caret`](#caret)
  - [`CaretEq`](#careteq)
  - [`Colon`](#colon)
  - [`Comma`](#comma)
  - [`Dollar`](#dollar)
  - [`Dot`](#dot)
  - [`DotDot`](#dotdot)
  - [`DotDotDot`](#dotdotdot)
  - [`DotDotEq`](#dotdoteq)
  - [`Eq`](#eq)
  - [`EqEq`](#eqeq)
  - [`FatArrow`](#fatarrow)
  - [`Ge`](#ge)
  - [`Gt`](#gt)
  - [`LArrow`](#larrow)
  - [`Le`](#le)
  - [`Lt`](#lt)
  - [`Minus`](#minus)
  - [`MinusEq`](#minuseq)
  - [`Ne`](#ne)
  - [`Not`](#not)
  - [`Or`](#or)
  - [`OrEq`](#oreq)
  - [`OrOr`](#oror)
  - [`PathSep`](#pathsep)
  - [`Percent`](#percent)
  - [`PercentEq`](#percenteq)
  - [`Plus`](#plus)
  - [`PlusEq`](#pluseq)
  - [`Pound`](#pound)
  - [`Question`](#question)
  - [`RArrow`](#rarrow)
  - [`Semi`](#semi)
  - [`Shl`](#shl)
  - [`ShlEq`](#shleq)
  - [`Shr`](#shr)
  - [`ShrEq`](#shreq)
  - [`Slash`](#slash)
  - [`SlashEq`](#slasheq)
  - [`Star`](#star)
  - [`StarEq`](#stareq)
  - [`Tilde`](#tilde)
  - [`Brace`](#brace)
  - [`Bracket`](#bracket)
  - [`Paren`](#paren)
- [Traits](#traits)
  - [`Token`](#token)
- [Macros](#macros)
  - [`impl_low_level_token!`](#impl-low-level-token)
  - [`define_keywords!`](#define-keywords)
  - [`impl_deref_if_len_is_1!`](#impl-deref-if-len-is-1)
  - [`define_punctuation_structs!`](#define-punctuation-structs)
  - [`define_punctuation!`](#define-punctuation)
  - [`define_delimiters!`](#define-delimiters)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Underscore`](#underscore) | struct | `_` |
| [`Group`](#group) | struct | None-delimited group |
| [`Abstract`](#abstract) | struct | `abstract` |
| [`As`](#as) | struct | `as` |
| [`Async`](#async) | struct | `async` |
| [`Auto`](#auto) | struct | `auto` |
| [`Await`](#await) | struct | `await` |
| [`Become`](#become) | struct | `become` |
| [`Box`](#box) | struct | `box` |
| [`Break`](#break) | struct | `break` |
| [`Const`](#const) | struct | `const` |
| [`Continue`](#continue) | struct | `continue` |
| [`Crate`](#crate) | struct | `crate` |
| [`Default`](#default) | struct | `default` |
| [`Do`](#do) | struct | `do` |
| [`Dyn`](#dyn) | struct | `dyn` |
| [`Else`](#else) | struct | `else` |
| [`Enum`](#enum) | struct | `enum` |
| [`Extern`](#extern) | struct | `extern` |
| [`Final`](#final) | struct | `final` |
| [`Fn`](#fn) | struct | `fn` |
| [`For`](#for) | struct | `for` |
| [`If`](#if) | struct | `if` |
| [`Impl`](#impl) | struct | `impl` |
| [`In`](#in) | struct | `in` |
| [`Let`](#let) | struct | `let` |
| [`Loop`](#loop) | struct | `loop` |
| [`Macro`](#macro) | struct | `macro` |
| [`Match`](#match) | struct | `match` |
| [`Mod`](#mod) | struct | `mod` |
| [`Move`](#move) | struct | `move` |
| [`Mut`](#mut) | struct | `mut` |
| [`Override`](#override) | struct | `override` |
| [`Priv`](#priv) | struct | `priv` |
| [`Pub`](#pub) | struct | `pub` |
| [`Raw`](#raw) | struct | `raw` |
| [`Ref`](#ref) | struct | `ref` |
| [`Return`](#return) | struct | `return` |
| [`SelfType`](#selftype) | struct | `Self` |
| [`SelfValue`](#selfvalue) | struct | `self` |
| [`Static`](#static) | struct | `static` |
| [`Struct`](#struct) | struct | `struct` |
| [`Super`](#super) | struct | `super` |
| [`Trait`](#trait) | struct | `trait` |
| [`Try`](#try) | struct | `try` |
| [`Type`](#type) | struct | `type` |
| [`Typeof`](#typeof) | struct | `typeof` |
| [`Union`](#union) | struct | `union` |
| [`Unsafe`](#unsafe) | struct | `unsafe` |
| [`Unsized`](#unsized) | struct | `unsized` |
| [`Use`](#use) | struct | `use` |
| [`Virtual`](#virtual) | struct | `virtual` |
| [`Where`](#where) | struct | `where` |
| [`While`](#while) | struct | `while` |
| [`Yield`](#yield) | struct | `yield` |
| [`And`](#and) | struct | `&` |
| [`AndAnd`](#andand) | struct | `&&` |
| [`AndEq`](#andeq) | struct | `&=` |
| [`At`](#at) | struct | `@` |
| [`Caret`](#caret) | struct | `^` |
| [`CaretEq`](#careteq) | struct | `^=` |
| [`Colon`](#colon) | struct | `:` |
| [`Comma`](#comma) | struct | `,` |
| [`Dollar`](#dollar) | struct | `$` |
| [`Dot`](#dot) | struct | `.` |
| [`DotDot`](#dotdot) | struct | `..` |
| [`DotDotDot`](#dotdotdot) | struct | `...` |
| [`DotDotEq`](#dotdoteq) | struct | `..=` |
| [`Eq`](#eq) | struct | `=` |
| [`EqEq`](#eqeq) | struct | `==` |
| [`FatArrow`](#fatarrow) | struct | `=>` |
| [`Ge`](#ge) | struct | `>=` |
| [`Gt`](#gt) | struct | `>` |
| [`LArrow`](#larrow) | struct | `<-` |
| [`Le`](#le) | struct | `<=` |
| [`Lt`](#lt) | struct | `<` |
| [`Minus`](#minus) | struct | `-` |
| [`MinusEq`](#minuseq) | struct | `-=` |
| [`Ne`](#ne) | struct | `!=` |
| [`Not`](#not) | struct | `!` |
| [`Or`](#or) | struct | `\|` |
| [`OrEq`](#oreq) | struct | `\|=` |
| [`OrOr`](#oror) | struct | `\|\|` |
| [`PathSep`](#pathsep) | struct | `::` |
| [`Percent`](#percent) | struct | `%` |
| [`PercentEq`](#percenteq) | struct | `%=` |
| [`Plus`](#plus) | struct | `+` |
| [`PlusEq`](#pluseq) | struct | `+=` |
| [`Pound`](#pound) | struct | `#` |
| [`Question`](#question) | struct | `?` |
| [`RArrow`](#rarrow) | struct | `->` |
| [`Semi`](#semi) | struct | `;` |
| [`Shl`](#shl) | struct | `<<` |
| [`ShlEq`](#shleq) | struct | `<<=` |
| [`Shr`](#shr) | struct | `>>` |
| [`ShrEq`](#shreq) | struct | `>>=` |
| [`Slash`](#slash) | struct | `/` |
| [`SlashEq`](#slasheq) | struct | `/=` |
| [`Star`](#star) | struct | `*` |
| [`StarEq`](#stareq) | struct | `*=` |
| [`Tilde`](#tilde) | struct | `~` |
| [`Brace`](#brace) | struct | `{`&hellip;`}` |
| [`Bracket`](#bracket) | struct | `[`&hellip;`]` |
| [`Paren`](#paren) | struct | `(`&hellip;`)` |
| [`Token`](#token) | trait | Marker trait for types that represent single tokens. |
| [`impl_low_level_token!`](#impl-low-level-token) | macro |  |
| [`define_keywords!`](#define-keywords) | macro |  |
| [`impl_deref_if_len_is_1!`](#impl-deref-if-len-is-1) | macro |  |
| [`define_punctuation_structs!`](#define-punctuation-structs) | macro |  |
| [`define_punctuation!`](#define-punctuation) | macro |  |
| [`define_delimiters!`](#define-delimiters) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Underscore`

```rust
struct Underscore {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:521-523`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L521-L523)*

`_`

Usage:
 wildcard patterns, inferred types, unnamed items in constants, extern crates, use declarations, and destructuring assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Underscore`

- <span id="underscore-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Underscore`

- <span id="underscore-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Underscore`

- <span id="underscore-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Underscore`

- <span id="underscore-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Underscore`

- <span id="underscore-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Underscore`

##### `impl Debug for Underscore`

- <span id="underscore-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Underscore`

- <span id="underscore-default"></span>`fn default() -> Self`

##### `impl Deref for Underscore`

- <span id="underscore-deref-type-target"></span>`type Target = WithSpan`

- <span id="underscore-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Underscore`

- <span id="underscore-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Underscore`

##### `impl<T> From for Underscore`

- <span id="underscore-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Underscore`

- <span id="underscore-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Underscore`

- <span id="underscore-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Underscore`

- <span id="underscore-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Underscore`

- <span id="underscore-partialeq-eq"></span>`fn eq(&self, _other: &Underscore) -> bool` — [`Underscore`](#underscore)

##### `impl Receiver for Underscore`

- <span id="underscore-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Underscore`

##### `impl Spanned for Underscore`

- <span id="underscore-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Underscore`

- <span id="underscore-toowned-type-owned"></span>`type Owned = T`

- <span id="underscore-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="underscore-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Underscore`

- <span id="underscore-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Underscore`

##### `impl<U> TryFrom for Underscore`

- <span id="underscore-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="underscore-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Underscore`

- <span id="underscore-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="underscore-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Group`

```rust
struct Group {
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:574-576`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L574-L576)*

None-delimited group

#### Implementations

- <span id="group-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Any for Group`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Group`

- <span id="group-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Group`

##### `impl Debug for Group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Group`

- <span id="group-default"></span>`fn default() -> Self`

##### `impl Eq for Group`

##### `impl<T> From for Group`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Group`

- <span id="group-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Group`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Group`

- <span id="group-partialeq-eq"></span>`fn eq(&self, _other: &Group) -> bool` — [`Group`](#group)

##### `impl Sealed for Group`

##### `impl ToOwned for Group`

- <span id="group-toowned-type-owned"></span>`type Owned = T`

- <span id="group-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="group-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl Token for Group`

##### `impl<U> TryFrom for Group`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Abstract`

```rust
struct Abstract {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`abstract`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Abstract`

- <span id="abstract-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Abstract`

- <span id="abstract-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Abstract`

- <span id="abstract-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Abstract`

- <span id="abstract-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Abstract`

- <span id="abstract-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Abstract`

##### `impl Debug for Abstract`

- <span id="abstract-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abstract`

- <span id="abstract-default"></span>`fn default() -> Self`

##### `impl Eq for Abstract`

##### `impl<T> From for Abstract`

- <span id="abstract-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Abstract`

- <span id="abstract-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Abstract`

- <span id="abstract-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Abstract`

- <span id="abstract-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Abstract`

- <span id="abstract-partialeq-eq"></span>`fn eq(&self, _other: &Abstract) -> bool` — [`Abstract`](#abstract)

##### `impl Sealed for Abstract`

##### `impl Spanned for Abstract`

- <span id="abstract-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Abstract`

- <span id="abstract-toowned-type-owned"></span>`type Owned = T`

- <span id="abstract-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="abstract-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Abstract`

- <span id="abstract-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Abstract`

##### `impl<U> TryFrom for Abstract`

- <span id="abstract-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="abstract-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Abstract`

- <span id="abstract-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="abstract-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `As`

```rust
struct As {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`as`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for As`

- <span id="as-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for As`

- <span id="as-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for As`

- <span id="as-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for As`

- <span id="as-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for As`

- <span id="as-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for As`

##### `impl Debug for As`

- <span id="as-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for As`

- <span id="as-default"></span>`fn default() -> Self`

##### `impl Eq for As`

##### `impl<T> From for As`

- <span id="as-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for As`

- <span id="as-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for As`

- <span id="as-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for As`

- <span id="as-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for As`

- <span id="as-partialeq-eq"></span>`fn eq(&self, _other: &As) -> bool` — [`As`](#as)

##### `impl Sealed for As`

##### `impl Spanned for As`

- <span id="as-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for As`

- <span id="as-toowned-type-owned"></span>`type Owned = T`

- <span id="as-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="as-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for As`

- <span id="as-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for As`

##### `impl<U> TryFrom for As`

- <span id="as-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="as-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for As`

- <span id="as-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="as-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Async`

```rust
struct Async {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`async`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Async`

- <span id="async-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Async`

- <span id="async-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Async`

- <span id="async-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Async`

- <span id="async-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Async`

- <span id="async-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Async`

##### `impl Debug for Async`

- <span id="async-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Async`

- <span id="async-default"></span>`fn default() -> Self`

##### `impl Eq for Async`

##### `impl<T> From for Async`

- <span id="async-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Async`

- <span id="async-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Async`

- <span id="async-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Async`

- <span id="async-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Async`

- <span id="async-partialeq-eq"></span>`fn eq(&self, _other: &Async) -> bool` — [`Async`](#async)

##### `impl Sealed for Async`

##### `impl Spanned for Async`

- <span id="async-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Async`

- <span id="async-toowned-type-owned"></span>`type Owned = T`

- <span id="async-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="async-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Async`

- <span id="async-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Async`

##### `impl<U> TryFrom for Async`

- <span id="async-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="async-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Async`

- <span id="async-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="async-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Auto`

```rust
struct Auto {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`auto`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Auto`

- <span id="auto-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Auto`

- <span id="auto-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Auto`

- <span id="auto-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Auto`

- <span id="auto-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Auto`

- <span id="auto-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Auto`

##### `impl Debug for Auto`

- <span id="auto-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Auto`

- <span id="auto-default"></span>`fn default() -> Self`

##### `impl Eq for Auto`

##### `impl<T> From for Auto`

- <span id="auto-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Auto`

- <span id="auto-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Auto`

- <span id="auto-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Auto`

- <span id="auto-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Auto`

- <span id="auto-partialeq-eq"></span>`fn eq(&self, _other: &Auto) -> bool` — [`Auto`](#auto)

##### `impl Sealed for Auto`

##### `impl Spanned for Auto`

- <span id="auto-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Auto`

- <span id="auto-toowned-type-owned"></span>`type Owned = T`

- <span id="auto-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="auto-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Auto`

- <span id="auto-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Auto`

##### `impl<U> TryFrom for Auto`

- <span id="auto-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="auto-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Auto`

- <span id="auto-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="auto-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Await`

```rust
struct Await {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`await`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Await`

- <span id="await-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Await`

- <span id="await-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Await`

- <span id="await-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Await`

- <span id="await-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Await`

- <span id="await-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Await`

##### `impl Debug for Await`

- <span id="await-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Await`

- <span id="await-default"></span>`fn default() -> Self`

##### `impl Eq for Await`

##### `impl<T> From for Await`

- <span id="await-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Await`

- <span id="await-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Await`

- <span id="await-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Await`

- <span id="await-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Await`

- <span id="await-partialeq-eq"></span>`fn eq(&self, _other: &Await) -> bool` — [`Await`](#await)

##### `impl Sealed for Await`

##### `impl Spanned for Await`

- <span id="await-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Await`

- <span id="await-toowned-type-owned"></span>`type Owned = T`

- <span id="await-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="await-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Await`

- <span id="await-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Await`

##### `impl<U> TryFrom for Await`

- <span id="await-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="await-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Await`

- <span id="await-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="await-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Become`

```rust
struct Become {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`become`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Become`

- <span id="become-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Become`

- <span id="become-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Become`

- <span id="become-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Become`

- <span id="become-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Become`

- <span id="become-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Become`

##### `impl Debug for Become`

- <span id="become-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Become`

- <span id="become-default"></span>`fn default() -> Self`

##### `impl Eq for Become`

##### `impl<T> From for Become`

- <span id="become-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Become`

- <span id="become-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Become`

- <span id="become-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Become`

- <span id="become-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Become`

- <span id="become-partialeq-eq"></span>`fn eq(&self, _other: &Become) -> bool` — [`Become`](#become)

##### `impl Sealed for Become`

##### `impl Spanned for Become`

- <span id="become-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Become`

- <span id="become-toowned-type-owned"></span>`type Owned = T`

- <span id="become-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="become-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Become`

- <span id="become-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Become`

##### `impl<U> TryFrom for Become`

- <span id="become-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="become-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Become`

- <span id="become-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="become-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Box`

```rust
struct Box {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`box`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Box`

- <span id="box-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Box`

- <span id="box-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Box`

- <span id="box-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Box`

- <span id="box-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Box`

- <span id="box-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Box`

##### `impl Debug for Box`

- <span id="box-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Box`

- <span id="box-default"></span>`fn default() -> Self`

##### `impl Eq for Box`

##### `impl<T> From for Box`

- <span id="box-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Box`

- <span id="box-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Box`

- <span id="box-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Box`

- <span id="box-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Box`

- <span id="box-partialeq-eq"></span>`fn eq(&self, _other: &Box) -> bool` — [`Box`](#box)

##### `impl Sealed for Box`

##### `impl Spanned for Box`

- <span id="box-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Box`

- <span id="box-toowned-type-owned"></span>`type Owned = T`

- <span id="box-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="box-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Box`

- <span id="box-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Box`

##### `impl<U> TryFrom for Box`

- <span id="box-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="box-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Box`

- <span id="box-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="box-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Break`

```rust
struct Break {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`break`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Break`

- <span id="break-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Break`

- <span id="break-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Break`

- <span id="break-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Break`

- <span id="break-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Break`

- <span id="break-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Break`

##### `impl Debug for Break`

- <span id="break-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Break`

- <span id="break-default"></span>`fn default() -> Self`

##### `impl Eq for Break`

##### `impl<T> From for Break`

- <span id="break-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Break`

- <span id="break-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Break`

- <span id="break-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Break`

- <span id="break-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Break`

- <span id="break-partialeq-eq"></span>`fn eq(&self, _other: &Break) -> bool` — [`Break`](#break)

##### `impl Sealed for Break`

##### `impl Spanned for Break`

- <span id="break-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Break`

- <span id="break-toowned-type-owned"></span>`type Owned = T`

- <span id="break-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="break-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Break`

- <span id="break-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Break`

##### `impl<U> TryFrom for Break`

- <span id="break-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="break-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Break`

- <span id="break-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="break-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Const`

```rust
struct Const {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`const`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Const`

- <span id="const-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Const`

- <span id="const-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Const`

- <span id="const-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Const`

- <span id="const-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Const`

- <span id="const-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Const`

##### `impl Debug for Const`

- <span id="const-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Const`

- <span id="const-default"></span>`fn default() -> Self`

##### `impl Eq for Const`

##### `impl<T> From for Const`

- <span id="const-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Const`

- <span id="const-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Const`

- <span id="const-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Const`

- <span id="const-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Const`

- <span id="const-partialeq-eq"></span>`fn eq(&self, _other: &Const) -> bool` — [`Const`](#const)

##### `impl Sealed for Const`

##### `impl Spanned for Const`

- <span id="const-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Const`

- <span id="const-toowned-type-owned"></span>`type Owned = T`

- <span id="const-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="const-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Const`

- <span id="const-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Const`

##### `impl<U> TryFrom for Const`

- <span id="const-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="const-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Const`

- <span id="const-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="const-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Continue`

```rust
struct Continue {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`continue`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Continue`

- <span id="continue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Continue`

- <span id="continue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Continue`

- <span id="continue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Continue`

- <span id="continue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Continue`

- <span id="continue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Continue`

##### `impl Debug for Continue`

- <span id="continue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Continue`

- <span id="continue-default"></span>`fn default() -> Self`

##### `impl Eq for Continue`

##### `impl<T> From for Continue`

- <span id="continue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Continue`

- <span id="continue-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Continue`

- <span id="continue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Continue`

- <span id="continue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Continue`

- <span id="continue-partialeq-eq"></span>`fn eq(&self, _other: &Continue) -> bool` — [`Continue`](#continue)

##### `impl Sealed for Continue`

##### `impl Spanned for Continue`

- <span id="continue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Continue`

- <span id="continue-toowned-type-owned"></span>`type Owned = T`

- <span id="continue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="continue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Continue`

- <span id="continue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Continue`

##### `impl<U> TryFrom for Continue`

- <span id="continue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="continue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Continue`

- <span id="continue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="continue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Crate`

```rust
struct Crate {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`crate`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Crate`

- <span id="crate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Crate`

- <span id="crate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Crate`

- <span id="crate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Crate`

- <span id="crate-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Crate`

- <span id="crate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Crate`

##### `impl Debug for Crate`

- <span id="crate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Crate`

- <span id="crate-default"></span>`fn default() -> Self`

##### `impl Eq for Crate`

##### `impl<T> From for Crate`

- <span id="crate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Crate`

- <span id="crate-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Crate`

- <span id="crate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Crate`

- <span id="crate-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Crate`

- <span id="crate-partialeq-eq"></span>`fn eq(&self, _other: &Crate) -> bool` — [`Crate`](#crate)

##### `impl Sealed for Crate`

##### `impl Spanned for Crate`

- <span id="crate-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Crate`

- <span id="crate-toowned-type-owned"></span>`type Owned = T`

- <span id="crate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="crate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Crate`

- <span id="crate-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Crate`

##### `impl<U> TryFrom for Crate`

- <span id="crate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="crate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Crate`

- <span id="crate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="crate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Default`

```rust
struct Default {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`default`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Default`

- <span id="default-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Default`

- <span id="default-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Default`

- <span id="default-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Default`

- <span id="default-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Default`

- <span id="default-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Default`

##### `impl Debug for Default`

- <span id="default-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Default`

- <span id="default-default"></span>`fn default() -> Self`

##### `impl Eq for Default`

##### `impl<T> From for Default`

- <span id="default-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Default`

- <span id="default-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Default`

- <span id="default-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Default`

- <span id="default-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Default`

- <span id="default-partialeq-eq"></span>`fn eq(&self, _other: &Default) -> bool` — [`Default`](#default)

##### `impl Sealed for Default`

##### `impl Spanned for Default`

- <span id="default-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Default`

- <span id="default-toowned-type-owned"></span>`type Owned = T`

- <span id="default-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="default-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Default`

- <span id="default-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Default`

##### `impl<U> TryFrom for Default`

- <span id="default-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="default-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Default`

- <span id="default-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="default-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Do`

```rust
struct Do {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`do`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Do`

- <span id="do-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Do`

- <span id="do-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Do`

- <span id="do-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Do`

- <span id="do-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Do`

- <span id="do-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Do`

##### `impl Debug for Do`

- <span id="do-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Do`

- <span id="do-default"></span>`fn default() -> Self`

##### `impl Eq for Do`

##### `impl<T> From for Do`

- <span id="do-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Do`

- <span id="do-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Do`

- <span id="do-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Do`

- <span id="do-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Do`

- <span id="do-partialeq-eq"></span>`fn eq(&self, _other: &Do) -> bool` — [`Do`](#do)

##### `impl Sealed for Do`

##### `impl Spanned for Do`

- <span id="do-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Do`

- <span id="do-toowned-type-owned"></span>`type Owned = T`

- <span id="do-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="do-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Do`

- <span id="do-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Do`

##### `impl<U> TryFrom for Do`

- <span id="do-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="do-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Do`

- <span id="do-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="do-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dyn`

```rust
struct Dyn {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`dyn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Dyn`

- <span id="dyn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dyn`

- <span id="dyn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dyn`

- <span id="dyn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dyn`

- <span id="dyn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Dyn`

- <span id="dyn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Dyn`

##### `impl Debug for Dyn`

- <span id="dyn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dyn`

- <span id="dyn-default"></span>`fn default() -> Self`

##### `impl Eq for Dyn`

##### `impl<T> From for Dyn`

- <span id="dyn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Dyn`

- <span id="dyn-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Dyn`

- <span id="dyn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Dyn`

- <span id="dyn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dyn`

- <span id="dyn-partialeq-eq"></span>`fn eq(&self, _other: &Dyn) -> bool` — [`Dyn`](#dyn)

##### `impl Sealed for Dyn`

##### `impl Spanned for Dyn`

- <span id="dyn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Dyn`

- <span id="dyn-toowned-type-owned"></span>`type Owned = T`

- <span id="dyn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dyn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Dyn`

- <span id="dyn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dyn`

##### `impl<U> TryFrom for Dyn`

- <span id="dyn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dyn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dyn`

- <span id="dyn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dyn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Else`

```rust
struct Else {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`else`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Else`

- <span id="else-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Else`

- <span id="else-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Else`

- <span id="else-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Else`

- <span id="else-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Else`

- <span id="else-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Else`

##### `impl Debug for Else`

- <span id="else-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Else`

- <span id="else-default"></span>`fn default() -> Self`

##### `impl Eq for Else`

##### `impl<T> From for Else`

- <span id="else-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Else`

- <span id="else-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Else`

- <span id="else-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Else`

- <span id="else-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Else`

- <span id="else-partialeq-eq"></span>`fn eq(&self, _other: &Else) -> bool` — [`Else`](#else)

##### `impl Sealed for Else`

##### `impl Spanned for Else`

- <span id="else-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Else`

- <span id="else-toowned-type-owned"></span>`type Owned = T`

- <span id="else-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="else-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Else`

- <span id="else-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Else`

##### `impl<U> TryFrom for Else`

- <span id="else-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="else-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Else`

- <span id="else-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="else-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Enum`

```rust
struct Enum {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`enum`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Enum`

- <span id="enum-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Enum`

- <span id="enum-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Enum`

- <span id="enum-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Enum`

- <span id="enum-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Enum`

- <span id="enum-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Enum`

##### `impl Debug for Enum`

- <span id="enum-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Enum`

- <span id="enum-default"></span>`fn default() -> Self`

##### `impl Eq for Enum`

##### `impl<T> From for Enum`

- <span id="enum-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Enum`

- <span id="enum-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Enum`

- <span id="enum-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Enum`

- <span id="enum-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Enum`

- <span id="enum-partialeq-eq"></span>`fn eq(&self, _other: &Enum) -> bool` — [`Enum`](#enum)

##### `impl Sealed for Enum`

##### `impl Spanned for Enum`

- <span id="enum-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Enum`

- <span id="enum-toowned-type-owned"></span>`type Owned = T`

- <span id="enum-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="enum-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Enum`

- <span id="enum-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Enum`

##### `impl<U> TryFrom for Enum`

- <span id="enum-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="enum-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Enum`

- <span id="enum-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="enum-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Extern`

```rust
struct Extern {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`extern`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Extern`

- <span id="extern-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Extern`

- <span id="extern-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Extern`

- <span id="extern-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Extern`

- <span id="extern-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Extern`

- <span id="extern-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Extern`

##### `impl Debug for Extern`

- <span id="extern-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Extern`

- <span id="extern-default"></span>`fn default() -> Self`

##### `impl Eq for Extern`

##### `impl<T> From for Extern`

- <span id="extern-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Extern`

- <span id="extern-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Extern`

- <span id="extern-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Extern`

- <span id="extern-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Extern`

- <span id="extern-partialeq-eq"></span>`fn eq(&self, _other: &Extern) -> bool` — [`Extern`](#extern)

##### `impl Sealed for Extern`

##### `impl Spanned for Extern`

- <span id="extern-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Extern`

- <span id="extern-toowned-type-owned"></span>`type Owned = T`

- <span id="extern-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="extern-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Extern`

- <span id="extern-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Extern`

##### `impl<U> TryFrom for Extern`

- <span id="extern-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="extern-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Extern`

- <span id="extern-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="extern-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Final`

```rust
struct Final {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`final`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Final`

- <span id="final-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Final`

- <span id="final-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Final`

- <span id="final-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Final`

- <span id="final-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Final`

- <span id="final-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Final`

##### `impl Debug for Final`

- <span id="final-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Final`

- <span id="final-default"></span>`fn default() -> Self`

##### `impl Eq for Final`

##### `impl<T> From for Final`

- <span id="final-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Final`

- <span id="final-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Final`

- <span id="final-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Final`

- <span id="final-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Final`

- <span id="final-partialeq-eq"></span>`fn eq(&self, _other: &Final) -> bool` — [`Final`](#final)

##### `impl Sealed for Final`

##### `impl Spanned for Final`

- <span id="final-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Final`

- <span id="final-toowned-type-owned"></span>`type Owned = T`

- <span id="final-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="final-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Final`

- <span id="final-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Final`

##### `impl<U> TryFrom for Final`

- <span id="final-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="final-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Final`

- <span id="final-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="final-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Fn`

```rust
struct Fn {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`fn`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Fn`

- <span id="fn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fn`

- <span id="fn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fn`

- <span id="fn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Fn`

- <span id="fn-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Fn`

- <span id="fn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Fn`

##### `impl Debug for Fn`

- <span id="fn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Fn`

- <span id="fn-default"></span>`fn default() -> Self`

##### `impl Eq for Fn`

##### `impl<T> From for Fn`

- <span id="fn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Fn`

- <span id="fn-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Fn`

- <span id="fn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Fn`

- <span id="fn-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Fn`

- <span id="fn-partialeq-eq"></span>`fn eq(&self, _other: &Fn) -> bool` — [`Fn`](#fn)

##### `impl Sealed for Fn`

##### `impl Spanned for Fn`

- <span id="fn-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Fn`

- <span id="fn-toowned-type-owned"></span>`type Owned = T`

- <span id="fn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Fn`

- <span id="fn-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Fn`

##### `impl<U> TryFrom for Fn`

- <span id="fn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fn`

- <span id="fn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `For`

```rust
struct For {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`for`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for For`

- <span id="for-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for For`

- <span id="for-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for For`

- <span id="for-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for For`

- <span id="for-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for For`

- <span id="for-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for For`

##### `impl Debug for For`

- <span id="for-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for For`

- <span id="for-default"></span>`fn default() -> Self`

##### `impl Eq for For`

##### `impl<T> From for For`

- <span id="for-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for For`

- <span id="for-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for For`

- <span id="for-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for For`

- <span id="for-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for For`

- <span id="for-partialeq-eq"></span>`fn eq(&self, _other: &For) -> bool` — [`For`](#for)

##### `impl Sealed for For`

##### `impl Spanned for For`

- <span id="for-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for For`

- <span id="for-toowned-type-owned"></span>`type Owned = T`

- <span id="for-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="for-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for For`

- <span id="for-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for For`

##### `impl<U> TryFrom for For`

- <span id="for-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="for-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for For`

- <span id="for-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="for-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `If`

```rust
struct If {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`if`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for If`

- <span id="if-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for If`

- <span id="if-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for If`

- <span id="if-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for If`

- <span id="if-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for If`

- <span id="if-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for If`

##### `impl Debug for If`

- <span id="if-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for If`

- <span id="if-default"></span>`fn default() -> Self`

##### `impl Eq for If`

##### `impl<T> From for If`

- <span id="if-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for If`

- <span id="if-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for If`

- <span id="if-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for If`

- <span id="if-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for If`

- <span id="if-partialeq-eq"></span>`fn eq(&self, _other: &If) -> bool` — [`If`](#if)

##### `impl Sealed for If`

##### `impl Spanned for If`

- <span id="if-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for If`

- <span id="if-toowned-type-owned"></span>`type Owned = T`

- <span id="if-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="if-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for If`

- <span id="if-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for If`

##### `impl<U> TryFrom for If`

- <span id="if-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="if-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for If`

- <span id="if-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="if-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Impl`

```rust
struct Impl {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`impl`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Impl`

- <span id="impl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Impl`

- <span id="impl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Impl`

- <span id="impl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Impl`

- <span id="impl-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Impl`

- <span id="impl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Impl`

##### `impl Debug for Impl`

- <span id="impl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Impl`

- <span id="impl-default"></span>`fn default() -> Self`

##### `impl Eq for Impl`

##### `impl<T> From for Impl`

- <span id="impl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Impl`

- <span id="impl-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Impl`

- <span id="impl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Impl`

- <span id="impl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Impl`

- <span id="impl-partialeq-eq"></span>`fn eq(&self, _other: &Impl) -> bool` — [`Impl`](#impl)

##### `impl Sealed for Impl`

##### `impl Spanned for Impl`

- <span id="impl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Impl`

- <span id="impl-toowned-type-owned"></span>`type Owned = T`

- <span id="impl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="impl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Impl`

- <span id="impl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Impl`

##### `impl<U> TryFrom for Impl`

- <span id="impl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="impl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Impl`

- <span id="impl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="impl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `In`

```rust
struct In {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`in`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for In`

- <span id="in-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for In`

- <span id="in-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for In`

- <span id="in-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for In`

- <span id="in-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for In`

- <span id="in-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for In`

##### `impl Debug for In`

- <span id="in-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for In`

- <span id="in-default"></span>`fn default() -> Self`

##### `impl Eq for In`

##### `impl<T> From for In`

- <span id="in-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for In`

- <span id="in-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for In`

- <span id="in-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for In`

- <span id="in-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for In`

- <span id="in-partialeq-eq"></span>`fn eq(&self, _other: &In) -> bool` — [`In`](#in)

##### `impl Sealed for In`

##### `impl Spanned for In`

- <span id="in-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for In`

- <span id="in-toowned-type-owned"></span>`type Owned = T`

- <span id="in-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="in-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for In`

- <span id="in-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for In`

##### `impl<U> TryFrom for In`

- <span id="in-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="in-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for In`

- <span id="in-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="in-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Let`

```rust
struct Let {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`let`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Let`

- <span id="let-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Let`

- <span id="let-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Let`

- <span id="let-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Let`

- <span id="let-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Let`

- <span id="let-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Let`

##### `impl Debug for Let`

- <span id="let-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Let`

- <span id="let-default"></span>`fn default() -> Self`

##### `impl Eq for Let`

##### `impl<T> From for Let`

- <span id="let-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Let`

- <span id="let-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Let`

- <span id="let-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Let`

- <span id="let-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Let`

- <span id="let-partialeq-eq"></span>`fn eq(&self, _other: &Let) -> bool` — [`Let`](#let)

##### `impl Sealed for Let`

##### `impl Spanned for Let`

- <span id="let-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Let`

- <span id="let-toowned-type-owned"></span>`type Owned = T`

- <span id="let-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="let-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Let`

- <span id="let-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Let`

##### `impl<U> TryFrom for Let`

- <span id="let-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="let-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Let`

- <span id="let-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="let-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Loop`

```rust
struct Loop {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`loop`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Loop`

- <span id="loop-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Loop`

- <span id="loop-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Loop`

- <span id="loop-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Loop`

- <span id="loop-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Loop`

- <span id="loop-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Loop`

##### `impl Debug for Loop`

- <span id="loop-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Loop`

- <span id="loop-default"></span>`fn default() -> Self`

##### `impl Eq for Loop`

##### `impl<T> From for Loop`

- <span id="loop-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Loop`

- <span id="loop-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Loop`

- <span id="loop-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Loop`

- <span id="loop-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Loop`

- <span id="loop-partialeq-eq"></span>`fn eq(&self, _other: &Loop) -> bool` — [`Loop`](#loop)

##### `impl Sealed for Loop`

##### `impl Spanned for Loop`

- <span id="loop-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Loop`

- <span id="loop-toowned-type-owned"></span>`type Owned = T`

- <span id="loop-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="loop-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Loop`

- <span id="loop-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Loop`

##### `impl<U> TryFrom for Loop`

- <span id="loop-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="loop-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Loop`

- <span id="loop-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="loop-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Macro`

```rust
struct Macro {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`macro`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Macro`

- <span id="macro-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Macro`

- <span id="macro-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Macro`

- <span id="macro-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Macro`

- <span id="macro-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Macro`

- <span id="macro-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Macro`

##### `impl Debug for Macro`

- <span id="macro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Macro`

- <span id="macro-default"></span>`fn default() -> Self`

##### `impl Eq for Macro`

##### `impl<T> From for Macro`

- <span id="macro-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Macro`

- <span id="macro-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Macro`

- <span id="macro-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Macro`

- <span id="macro-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Macro`

- <span id="macro-partialeq-eq"></span>`fn eq(&self, _other: &Macro) -> bool` — [`Macro`](#macro)

##### `impl Sealed for Macro`

##### `impl Spanned for Macro`

- <span id="macro-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Macro`

- <span id="macro-toowned-type-owned"></span>`type Owned = T`

- <span id="macro-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="macro-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Macro`

- <span id="macro-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Macro`

##### `impl<U> TryFrom for Macro`

- <span id="macro-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="macro-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Macro`

- <span id="macro-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="macro-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match`

```rust
struct Match {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`match`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Match`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Match`

- <span id="match-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Match`

- <span id="match-default"></span>`fn default() -> Self`

##### `impl Eq for Match`

##### `impl<T> From for Match`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Match`

- <span id="match-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Match`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Match`

- <span id="match-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Match`

- <span id="match-partialeq-eq"></span>`fn eq(&self, _other: &Match) -> bool` — [`Match`](#match)

##### `impl Sealed for Match`

##### `impl Spanned for Match`

- <span id="match-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Match`

- <span id="match-toowned-type-owned"></span>`type Owned = T`

- <span id="match-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="match-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Match`

- <span id="match-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Match`

##### `impl<U> TryFrom for Match`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mod`

```rust
struct Mod {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`mod`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Mod`

- <span id="mod-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mod`

- <span id="mod-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mod`

- <span id="mod-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Mod`

- <span id="mod-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Mod`

- <span id="mod-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Mod`

##### `impl Debug for Mod`

- <span id="mod-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mod`

- <span id="mod-default"></span>`fn default() -> Self`

##### `impl Eq for Mod`

##### `impl<T> From for Mod`

- <span id="mod-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Mod`

- <span id="mod-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Mod`

- <span id="mod-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Mod`

- <span id="mod-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Mod`

- <span id="mod-partialeq-eq"></span>`fn eq(&self, _other: &Mod) -> bool` — [`Mod`](#mod)

##### `impl Sealed for Mod`

##### `impl Spanned for Mod`

- <span id="mod-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Mod`

- <span id="mod-toowned-type-owned"></span>`type Owned = T`

- <span id="mod-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mod-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Mod`

- <span id="mod-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Mod`

##### `impl<U> TryFrom for Mod`

- <span id="mod-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mod-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mod`

- <span id="mod-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mod-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Move`

```rust
struct Move {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`move`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Move`

- <span id="move-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Move`

- <span id="move-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Move`

- <span id="move-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Move`

- <span id="move-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Move`

- <span id="move-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Move`

##### `impl Debug for Move`

- <span id="move-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Move`

- <span id="move-default"></span>`fn default() -> Self`

##### `impl Eq for Move`

##### `impl<T> From for Move`

- <span id="move-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Move`

- <span id="move-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Move`

- <span id="move-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Move`

- <span id="move-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Move`

- <span id="move-partialeq-eq"></span>`fn eq(&self, _other: &Move) -> bool` — [`Move`](#move)

##### `impl Sealed for Move`

##### `impl Spanned for Move`

- <span id="move-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Move`

- <span id="move-toowned-type-owned"></span>`type Owned = T`

- <span id="move-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="move-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Move`

- <span id="move-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Move`

##### `impl<U> TryFrom for Move`

- <span id="move-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="move-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Move`

- <span id="move-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="move-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Mut`

```rust
struct Mut {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`mut`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Mut`

- <span id="mut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Mut`

- <span id="mut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Mut`

- <span id="mut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Mut`

- <span id="mut-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Mut`

- <span id="mut-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Mut`

##### `impl Debug for Mut`

- <span id="mut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Mut`

- <span id="mut-default"></span>`fn default() -> Self`

##### `impl Eq for Mut`

##### `impl<T> From for Mut`

- <span id="mut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Mut`

- <span id="mut-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Mut`

- <span id="mut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Mut`

- <span id="mut-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Mut`

- <span id="mut-partialeq-eq"></span>`fn eq(&self, _other: &Mut) -> bool` — [`Mut`](#mut)

##### `impl Sealed for Mut`

##### `impl Spanned for Mut`

- <span id="mut-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Mut`

- <span id="mut-toowned-type-owned"></span>`type Owned = T`

- <span id="mut-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mut-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Mut`

- <span id="mut-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Mut`

##### `impl<U> TryFrom for Mut`

- <span id="mut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Mut`

- <span id="mut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Override`

```rust
struct Override {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`override`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Override`

- <span id="override-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Override`

- <span id="override-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Override`

- <span id="override-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Override`

- <span id="override-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Override`

- <span id="override-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Override`

##### `impl Debug for Override`

- <span id="override-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Override`

- <span id="override-default"></span>`fn default() -> Self`

##### `impl Eq for Override`

##### `impl<T> From for Override`

- <span id="override-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Override`

- <span id="override-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Override`

- <span id="override-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Override`

- <span id="override-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Override`

- <span id="override-partialeq-eq"></span>`fn eq(&self, _other: &Override) -> bool` — [`Override`](#override)

##### `impl Sealed for Override`

##### `impl Spanned for Override`

- <span id="override-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Override`

- <span id="override-toowned-type-owned"></span>`type Owned = T`

- <span id="override-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="override-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Override`

- <span id="override-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Override`

##### `impl<U> TryFrom for Override`

- <span id="override-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="override-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Override`

- <span id="override-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="override-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Priv`

```rust
struct Priv {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`priv`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Priv`

- <span id="priv-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Priv`

- <span id="priv-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Priv`

- <span id="priv-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Priv`

- <span id="priv-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Priv`

- <span id="priv-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Priv`

##### `impl Debug for Priv`

- <span id="priv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Priv`

- <span id="priv-default"></span>`fn default() -> Self`

##### `impl Eq for Priv`

##### `impl<T> From for Priv`

- <span id="priv-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Priv`

- <span id="priv-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Priv`

- <span id="priv-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Priv`

- <span id="priv-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Priv`

- <span id="priv-partialeq-eq"></span>`fn eq(&self, _other: &Priv) -> bool` — [`Priv`](#priv)

##### `impl Sealed for Priv`

##### `impl Spanned for Priv`

- <span id="priv-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Priv`

- <span id="priv-toowned-type-owned"></span>`type Owned = T`

- <span id="priv-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="priv-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Priv`

- <span id="priv-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Priv`

##### `impl<U> TryFrom for Priv`

- <span id="priv-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="priv-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Priv`

- <span id="priv-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="priv-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pub`

```rust
struct Pub {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`pub`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Pub`

- <span id="pub-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pub`

- <span id="pub-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pub`

- <span id="pub-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pub`

- <span id="pub-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Pub`

- <span id="pub-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pub`

##### `impl Debug for Pub`

- <span id="pub-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pub`

- <span id="pub-default"></span>`fn default() -> Self`

##### `impl Eq for Pub`

##### `impl<T> From for Pub`

- <span id="pub-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Pub`

- <span id="pub-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Pub`

- <span id="pub-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Pub`

- <span id="pub-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Pub`

- <span id="pub-partialeq-eq"></span>`fn eq(&self, _other: &Pub) -> bool` — [`Pub`](#pub)

##### `impl Sealed for Pub`

##### `impl Spanned for Pub`

- <span id="pub-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Pub`

- <span id="pub-toowned-type-owned"></span>`type Owned = T`

- <span id="pub-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pub-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Pub`

- <span id="pub-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Pub`

##### `impl<U> TryFrom for Pub`

- <span id="pub-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pub-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pub`

- <span id="pub-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pub-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Raw`

```rust
struct Raw {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`raw`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Raw`

- <span id="raw-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Raw`

- <span id="raw-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Raw`

- <span id="raw-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Raw`

- <span id="raw-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Raw`

- <span id="raw-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Raw`

##### `impl Debug for Raw`

- <span id="raw-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Raw`

- <span id="raw-default"></span>`fn default() -> Self`

##### `impl Eq for Raw`

##### `impl<T> From for Raw`

- <span id="raw-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Raw`

- <span id="raw-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Raw`

- <span id="raw-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Raw`

- <span id="raw-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Raw`

- <span id="raw-partialeq-eq"></span>`fn eq(&self, _other: &Raw) -> bool` — [`Raw`](#raw)

##### `impl Sealed for Raw`

##### `impl Spanned for Raw`

- <span id="raw-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Raw`

- <span id="raw-toowned-type-owned"></span>`type Owned = T`

- <span id="raw-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="raw-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Raw`

- <span id="raw-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Raw`

##### `impl<U> TryFrom for Raw`

- <span id="raw-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="raw-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Raw`

- <span id="raw-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="raw-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ref`

```rust
struct Ref {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`ref`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Ref`

- <span id="ref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ref`

- <span id="ref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ref`

- <span id="ref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ref`

- <span id="ref-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Ref`

- <span id="ref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ref`

##### `impl Debug for Ref`

- <span id="ref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ref`

- <span id="ref-default"></span>`fn default() -> Self`

##### `impl Eq for Ref`

##### `impl<T> From for Ref`

- <span id="ref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Ref`

- <span id="ref-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Ref`

- <span id="ref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Ref`

- <span id="ref-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ref`

- <span id="ref-partialeq-eq"></span>`fn eq(&self, _other: &Ref) -> bool` — [`Ref`](#ref)

##### `impl Sealed for Ref`

##### `impl Spanned for Ref`

- <span id="ref-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Ref`

- <span id="ref-toowned-type-owned"></span>`type Owned = T`

- <span id="ref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Ref`

- <span id="ref-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ref`

##### `impl<U> TryFrom for Ref`

- <span id="ref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ref`

- <span id="ref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Return`

```rust
struct Return {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`return`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Return`

- <span id="return-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Return`

- <span id="return-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Return`

- <span id="return-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Return`

- <span id="return-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Return`

- <span id="return-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Return`

##### `impl Debug for Return`

- <span id="return-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Return`

- <span id="return-default"></span>`fn default() -> Self`

##### `impl Eq for Return`

##### `impl<T> From for Return`

- <span id="return-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Return`

- <span id="return-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Return`

- <span id="return-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Return`

- <span id="return-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Return`

- <span id="return-partialeq-eq"></span>`fn eq(&self, _other: &Return) -> bool` — [`Return`](#return)

##### `impl Sealed for Return`

##### `impl Spanned for Return`

- <span id="return-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Return`

- <span id="return-toowned-type-owned"></span>`type Owned = T`

- <span id="return-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="return-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Return`

- <span id="return-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Return`

##### `impl<U> TryFrom for Return`

- <span id="return-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="return-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Return`

- <span id="return-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="return-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SelfType`

```rust
struct SelfType {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`Self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for SelfType`

- <span id="selftype-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SelfType`

- <span id="selftype-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SelfType`

- <span id="selftype-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SelfType`

- <span id="selftype-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for SelfType`

- <span id="selftype-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SelfType`

##### `impl Debug for SelfType`

- <span id="selftype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfType`

- <span id="selftype-default"></span>`fn default() -> Self`

##### `impl Eq for SelfType`

##### `impl<T> From for SelfType`

- <span id="selftype-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SelfType`

- <span id="selftype-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for SelfType`

- <span id="selftype-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for SelfType`

- <span id="selftype-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SelfType`

- <span id="selftype-partialeq-eq"></span>`fn eq(&self, _other: &SelfType) -> bool` — [`SelfType`](#selftype)

##### `impl Sealed for SelfType`

##### `impl Spanned for SelfType`

- <span id="selftype-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for SelfType`

- <span id="selftype-toowned-type-owned"></span>`type Owned = T`

- <span id="selftype-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="selftype-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for SelfType`

- <span id="selftype-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SelfType`

##### `impl<U> TryFrom for SelfType`

- <span id="selftype-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="selftype-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SelfType`

- <span id="selftype-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="selftype-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SelfValue`

```rust
struct SelfValue {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`self`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for SelfValue`

- <span id="selfvalue-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SelfValue`

- <span id="selfvalue-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SelfValue`

- <span id="selfvalue-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SelfValue`

- <span id="selfvalue-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for SelfValue`

- <span id="selfvalue-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SelfValue`

##### `impl Debug for SelfValue`

- <span id="selfvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SelfValue`

- <span id="selfvalue-default"></span>`fn default() -> Self`

##### `impl Eq for SelfValue`

##### `impl<T> From for SelfValue`

- <span id="selfvalue-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SelfValue`

- <span id="selfvalue-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for SelfValue`

- <span id="selfvalue-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for SelfValue`

- <span id="selfvalue-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SelfValue`

- <span id="selfvalue-partialeq-eq"></span>`fn eq(&self, _other: &SelfValue) -> bool` — [`SelfValue`](#selfvalue)

##### `impl Sealed for SelfValue`

##### `impl Spanned for SelfValue`

- <span id="selfvalue-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for SelfValue`

- <span id="selfvalue-toowned-type-owned"></span>`type Owned = T`

- <span id="selfvalue-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="selfvalue-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for SelfValue`

- <span id="selfvalue-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SelfValue`

##### `impl<U> TryFrom for SelfValue`

- <span id="selfvalue-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="selfvalue-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SelfValue`

- <span id="selfvalue-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="selfvalue-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Static`

```rust
struct Static {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`static`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Static`

- <span id="static-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Static`

- <span id="static-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Static`

- <span id="static-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Static`

- <span id="static-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Static`

- <span id="static-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Static`

##### `impl Debug for Static`

- <span id="static-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Static`

- <span id="static-default"></span>`fn default() -> Self`

##### `impl Eq for Static`

##### `impl<T> From for Static`

- <span id="static-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Static`

- <span id="static-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Static`

- <span id="static-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Static`

- <span id="static-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Static`

- <span id="static-partialeq-eq"></span>`fn eq(&self, _other: &Static) -> bool` — [`Static`](#static)

##### `impl Sealed for Static`

##### `impl Spanned for Static`

- <span id="static-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Static`

- <span id="static-toowned-type-owned"></span>`type Owned = T`

- <span id="static-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="static-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Static`

- <span id="static-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Static`

##### `impl<U> TryFrom for Static`

- <span id="static-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="static-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Static`

- <span id="static-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="static-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Struct`

```rust
struct Struct {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`struct`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Struct`

- <span id="struct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Struct`

- <span id="struct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Struct`

- <span id="struct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Struct`

- <span id="struct-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Struct`

- <span id="struct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Struct`

##### `impl Debug for Struct`

- <span id="struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Struct`

- <span id="struct-default"></span>`fn default() -> Self`

##### `impl Eq for Struct`

##### `impl<T> From for Struct`

- <span id="struct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Struct`

- <span id="struct-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Struct`

- <span id="struct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Struct`

- <span id="struct-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Struct`

- <span id="struct-partialeq-eq"></span>`fn eq(&self, _other: &Struct) -> bool` — [`Struct`](#struct)

##### `impl Sealed for Struct`

##### `impl Spanned for Struct`

- <span id="struct-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Struct`

- <span id="struct-toowned-type-owned"></span>`type Owned = T`

- <span id="struct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="struct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Struct`

- <span id="struct-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Struct`

##### `impl<U> TryFrom for Struct`

- <span id="struct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="struct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Struct`

- <span id="struct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="struct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Super`

```rust
struct Super {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`super`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Super`

- <span id="super-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Super`

- <span id="super-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Super`

- <span id="super-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Super`

- <span id="super-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Super`

- <span id="super-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Super`

##### `impl Debug for Super`

- <span id="super-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Super`

- <span id="super-default"></span>`fn default() -> Self`

##### `impl Eq for Super`

##### `impl<T> From for Super`

- <span id="super-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Super`

- <span id="super-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Super`

- <span id="super-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Super`

- <span id="super-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Super`

- <span id="super-partialeq-eq"></span>`fn eq(&self, _other: &Super) -> bool` — [`Super`](#super)

##### `impl Sealed for Super`

##### `impl Spanned for Super`

- <span id="super-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Super`

- <span id="super-toowned-type-owned"></span>`type Owned = T`

- <span id="super-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="super-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Super`

- <span id="super-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Super`

##### `impl<U> TryFrom for Super`

- <span id="super-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="super-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Super`

- <span id="super-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="super-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Trait`

```rust
struct Trait {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`trait`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Trait`

- <span id="trait-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Trait`

- <span id="trait-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Trait`

- <span id="trait-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Trait`

- <span id="trait-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Trait`

- <span id="trait-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Trait`

##### `impl Debug for Trait`

- <span id="trait-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Trait`

- <span id="trait-default"></span>`fn default() -> Self`

##### `impl Eq for Trait`

##### `impl<T> From for Trait`

- <span id="trait-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Trait`

- <span id="trait-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Trait`

- <span id="trait-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Trait`

- <span id="trait-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Trait`

- <span id="trait-partialeq-eq"></span>`fn eq(&self, _other: &Trait) -> bool` — [`Trait`](#trait)

##### `impl Sealed for Trait`

##### `impl Spanned for Trait`

- <span id="trait-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Trait`

- <span id="trait-toowned-type-owned"></span>`type Owned = T`

- <span id="trait-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="trait-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Trait`

- <span id="trait-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Trait`

##### `impl<U> TryFrom for Trait`

- <span id="trait-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trait-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Trait`

- <span id="trait-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trait-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Try`

```rust
struct Try {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`try`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Try`

- <span id="try-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Try`

- <span id="try-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Try`

- <span id="try-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Try`

- <span id="try-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Try`

- <span id="try-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Try`

##### `impl Debug for Try`

- <span id="try-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Try`

- <span id="try-default"></span>`fn default() -> Self`

##### `impl Eq for Try`

##### `impl<T> From for Try`

- <span id="try-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Try`

- <span id="try-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Try`

- <span id="try-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Try`

- <span id="try-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Try`

- <span id="try-partialeq-eq"></span>`fn eq(&self, _other: &Try) -> bool` — [`Try`](#try)

##### `impl Sealed for Try`

##### `impl Spanned for Try`

- <span id="try-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Try`

- <span id="try-toowned-type-owned"></span>`type Owned = T`

- <span id="try-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="try-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Try`

- <span id="try-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Try`

##### `impl<U> TryFrom for Try`

- <span id="try-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="try-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Try`

- <span id="try-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="try-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Type`

```rust
struct Type {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`type`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Type`

- <span id="type-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Type`

- <span id="type-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Type`

- <span id="type-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Type`

- <span id="type-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Type`

- <span id="type-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Type`

##### `impl Debug for Type`

- <span id="type-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Type`

- <span id="type-default"></span>`fn default() -> Self`

##### `impl Eq for Type`

##### `impl<T> From for Type`

- <span id="type-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Type`

- <span id="type-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Type`

- <span id="type-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Type`

- <span id="type-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Type`

- <span id="type-partialeq-eq"></span>`fn eq(&self, _other: &Type) -> bool` — [`Type`](#type)

##### `impl Sealed for Type`

##### `impl Spanned for Type`

- <span id="type-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Type`

- <span id="type-toowned-type-owned"></span>`type Owned = T`

- <span id="type-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="type-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Type`

- <span id="type-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Type`

##### `impl<U> TryFrom for Type`

- <span id="type-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="type-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Type`

- <span id="type-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="type-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Typeof`

```rust
struct Typeof {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`typeof`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Typeof`

- <span id="typeof-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Typeof`

- <span id="typeof-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Typeof`

- <span id="typeof-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Typeof`

- <span id="typeof-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Typeof`

- <span id="typeof-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Typeof`

##### `impl Debug for Typeof`

- <span id="typeof-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Typeof`

- <span id="typeof-default"></span>`fn default() -> Self`

##### `impl Eq for Typeof`

##### `impl<T> From for Typeof`

- <span id="typeof-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Typeof`

- <span id="typeof-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Typeof`

- <span id="typeof-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Typeof`

- <span id="typeof-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Typeof`

- <span id="typeof-partialeq-eq"></span>`fn eq(&self, _other: &Typeof) -> bool` — [`Typeof`](#typeof)

##### `impl Sealed for Typeof`

##### `impl Spanned for Typeof`

- <span id="typeof-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Typeof`

- <span id="typeof-toowned-type-owned"></span>`type Owned = T`

- <span id="typeof-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="typeof-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Typeof`

- <span id="typeof-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Typeof`

##### `impl<U> TryFrom for Typeof`

- <span id="typeof-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="typeof-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Typeof`

- <span id="typeof-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="typeof-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Union`

```rust
struct Union {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`union`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Union`

- <span id="union-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Union`

- <span id="union-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Union`

- <span id="union-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Union`

- <span id="union-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Union`

- <span id="union-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Union`

##### `impl Debug for Union`

- <span id="union-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Union`

- <span id="union-default"></span>`fn default() -> Self`

##### `impl Eq for Union`

##### `impl<T> From for Union`

- <span id="union-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Union`

- <span id="union-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Union`

- <span id="union-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Union`

- <span id="union-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Union`

- <span id="union-partialeq-eq"></span>`fn eq(&self, _other: &Union) -> bool` — [`Union`](#union)

##### `impl Sealed for Union`

##### `impl Spanned for Union`

- <span id="union-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Union`

- <span id="union-toowned-type-owned"></span>`type Owned = T`

- <span id="union-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="union-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Union`

- <span id="union-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Union`

##### `impl<U> TryFrom for Union`

- <span id="union-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="union-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Union`

- <span id="union-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="union-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unsafe`

```rust
struct Unsafe {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`unsafe`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Unsafe`

- <span id="unsafe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unsafe`

- <span id="unsafe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unsafe`

- <span id="unsafe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Unsafe`

- <span id="unsafe-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Unsafe`

- <span id="unsafe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Unsafe`

##### `impl Debug for Unsafe`

- <span id="unsafe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsafe`

- <span id="unsafe-default"></span>`fn default() -> Self`

##### `impl Eq for Unsafe`

##### `impl<T> From for Unsafe`

- <span id="unsafe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Unsafe`

- <span id="unsafe-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Unsafe`

- <span id="unsafe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Unsafe`

- <span id="unsafe-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Unsafe`

- <span id="unsafe-partialeq-eq"></span>`fn eq(&self, _other: &Unsafe) -> bool` — [`Unsafe`](#unsafe)

##### `impl Sealed for Unsafe`

##### `impl Spanned for Unsafe`

- <span id="unsafe-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Unsafe`

- <span id="unsafe-toowned-type-owned"></span>`type Owned = T`

- <span id="unsafe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unsafe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Unsafe`

- <span id="unsafe-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Unsafe`

##### `impl<U> TryFrom for Unsafe`

- <span id="unsafe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unsafe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unsafe`

- <span id="unsafe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unsafe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unsized`

```rust
struct Unsized {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`unsized`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Unsized`

- <span id="unsized-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unsized`

- <span id="unsized-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unsized`

- <span id="unsized-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Unsized`

- <span id="unsized-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Unsized`

- <span id="unsized-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Unsized`

##### `impl Debug for Unsized`

- <span id="unsized-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Unsized`

- <span id="unsized-default"></span>`fn default() -> Self`

##### `impl Eq for Unsized`

##### `impl<T> From for Unsized`

- <span id="unsized-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Unsized`

- <span id="unsized-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Unsized`

- <span id="unsized-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Unsized`

- <span id="unsized-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Unsized`

- <span id="unsized-partialeq-eq"></span>`fn eq(&self, _other: &Unsized) -> bool` — [`Unsized`](#unsized)

##### `impl Sealed for Unsized`

##### `impl Spanned for Unsized`

- <span id="unsized-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Unsized`

- <span id="unsized-toowned-type-owned"></span>`type Owned = T`

- <span id="unsized-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unsized-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Unsized`

- <span id="unsized-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Unsized`

##### `impl<U> TryFrom for Unsized`

- <span id="unsized-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unsized-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unsized`

- <span id="unsized-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unsized-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Use`

```rust
struct Use {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`use`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Use`

- <span id="use-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Use`

- <span id="use-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Use`

- <span id="use-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Use`

- <span id="use-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Use`

- <span id="use-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Use`

##### `impl Debug for Use`

- <span id="use-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Use`

- <span id="use-default"></span>`fn default() -> Self`

##### `impl Eq for Use`

##### `impl<T> From for Use`

- <span id="use-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Use`

- <span id="use-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Use`

- <span id="use-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Use`

- <span id="use-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Use`

- <span id="use-partialeq-eq"></span>`fn eq(&self, _other: &Use) -> bool` — [`Use`](#use)

##### `impl Sealed for Use`

##### `impl Spanned for Use`

- <span id="use-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Use`

- <span id="use-toowned-type-owned"></span>`type Owned = T`

- <span id="use-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="use-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Use`

- <span id="use-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Use`

##### `impl<U> TryFrom for Use`

- <span id="use-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="use-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Use`

- <span id="use-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="use-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Virtual`

```rust
struct Virtual {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`virtual`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Virtual`

- <span id="virtual-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Virtual`

- <span id="virtual-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Virtual`

- <span id="virtual-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Virtual`

- <span id="virtual-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Virtual`

- <span id="virtual-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Virtual`

##### `impl Debug for Virtual`

- <span id="virtual-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Virtual`

- <span id="virtual-default"></span>`fn default() -> Self`

##### `impl Eq for Virtual`

##### `impl<T> From for Virtual`

- <span id="virtual-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Virtual`

- <span id="virtual-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Virtual`

- <span id="virtual-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Virtual`

- <span id="virtual-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Virtual`

- <span id="virtual-partialeq-eq"></span>`fn eq(&self, _other: &Virtual) -> bool` — [`Virtual`](#virtual)

##### `impl Sealed for Virtual`

##### `impl Spanned for Virtual`

- <span id="virtual-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Virtual`

- <span id="virtual-toowned-type-owned"></span>`type Owned = T`

- <span id="virtual-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="virtual-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Virtual`

- <span id="virtual-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Virtual`

##### `impl<U> TryFrom for Virtual`

- <span id="virtual-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="virtual-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Virtual`

- <span id="virtual-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="virtual-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Where`

```rust
struct Where {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`where`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Where`

- <span id="where-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Where`

- <span id="where-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Where`

- <span id="where-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Where`

- <span id="where-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Where`

- <span id="where-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Where`

##### `impl Debug for Where`

- <span id="where-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Where`

- <span id="where-default"></span>`fn default() -> Self`

##### `impl Eq for Where`

##### `impl<T> From for Where`

- <span id="where-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Where`

- <span id="where-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Where`

- <span id="where-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Where`

- <span id="where-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Where`

- <span id="where-partialeq-eq"></span>`fn eq(&self, _other: &Where) -> bool` — [`Where`](#where)

##### `impl Sealed for Where`

##### `impl Spanned for Where`

- <span id="where-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Where`

- <span id="where-toowned-type-owned"></span>`type Owned = T`

- <span id="where-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="where-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Where`

- <span id="where-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Where`

##### `impl<U> TryFrom for Where`

- <span id="where-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="where-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Where`

- <span id="where-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="where-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `While`

```rust
struct While {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`while`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for While`

- <span id="while-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for While`

- <span id="while-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for While`

- <span id="while-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for While`

- <span id="while-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for While`

- <span id="while-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for While`

##### `impl Debug for While`

- <span id="while-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for While`

- <span id="while-default"></span>`fn default() -> Self`

##### `impl Eq for While`

##### `impl<T> From for While`

- <span id="while-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for While`

- <span id="while-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for While`

- <span id="while-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for While`

- <span id="while-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for While`

- <span id="while-partialeq-eq"></span>`fn eq(&self, _other: &While) -> bool` — [`While`](#while)

##### `impl Sealed for While`

##### `impl Spanned for While`

- <span id="while-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for While`

- <span id="while-toowned-type-owned"></span>`type Owned = T`

- <span id="while-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="while-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for While`

- <span id="while-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for While`

##### `impl<U> TryFrom for While`

- <span id="while-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="while-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for While`

- <span id="while-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="while-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Yield`

```rust
struct Yield {
    pub span: Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:692-746`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L692-L746)*

`yield`

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Yield`

- <span id="yield-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Yield`

- <span id="yield-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Yield`

- <span id="yield-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Yield`

- <span id="yield-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Yield`

- <span id="yield-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Yield`

##### `impl Debug for Yield`

- <span id="yield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Yield`

- <span id="yield-default"></span>`fn default() -> Self`

##### `impl Eq for Yield`

##### `impl<T> From for Yield`

- <span id="yield-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Yield`

- <span id="yield-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Yield`

- <span id="yield-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Yield`

- <span id="yield-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Yield`

- <span id="yield-partialeq-eq"></span>`fn eq(&self, _other: &Yield) -> bool` — [`Yield`](#yield)

##### `impl Sealed for Yield`

##### `impl Spanned for Yield`

- <span id="yield-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Yield`

- <span id="yield-toowned-type-owned"></span>`type Owned = T`

- <span id="yield-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="yield-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Yield`

- <span id="yield-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Yield`

##### `impl<U> TryFrom for Yield`

- <span id="yield-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="yield-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Yield`

- <span id="yield-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="yield-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `And`

```rust
struct And {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`&`

Usage:
 bitwise and logical AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for And`

- <span id="and-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for And`

- <span id="and-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for And`

- <span id="and-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for And`

- <span id="and-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for And`

- <span id="and-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for And`

##### `impl Debug for And`

- <span id="and-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for And`

- <span id="and-default"></span>`fn default() -> Self`

##### `impl Deref for And`

- <span id="and-deref-type-target"></span>`type Target = WithSpan`

- <span id="and-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for And`

- <span id="and-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for And`

##### `impl<T> From for And`

- <span id="and-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for And`

- <span id="and-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for And`

- <span id="and-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for And`

- <span id="and-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for And`

- <span id="and-partialeq-eq"></span>`fn eq(&self, _other: &And) -> bool` — [`And`](#and)

##### `impl Receiver for And`

- <span id="and-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for And`

##### `impl Spanned for And`

- <span id="and-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for And`

- <span id="and-toowned-type-owned"></span>`type Owned = T`

- <span id="and-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="and-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for And`

- <span id="and-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for And`

##### `impl<U> TryFrom for And`

- <span id="and-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="and-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for And`

- <span id="and-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="and-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AndAnd`

```rust
struct AndAnd {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`&&`

Usage:
 lazy AND, borrow, references, reference patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for AndAnd`

- <span id="andand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AndAnd`

- <span id="andand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AndAnd`

- <span id="andand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AndAnd`

- <span id="andand-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AndAnd`

- <span id="andand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AndAnd`

##### `impl Debug for AndAnd`

- <span id="andand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndAnd`

- <span id="andand-default"></span>`fn default() -> Self`

##### `impl Eq for AndAnd`

##### `impl<T> From for AndAnd`

- <span id="andand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AndAnd`

- <span id="andand-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for AndAnd`

- <span id="andand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for AndAnd`

- <span id="andand-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for AndAnd`

- <span id="andand-partialeq-eq"></span>`fn eq(&self, _other: &AndAnd) -> bool` — [`AndAnd`](#andand)

##### `impl Sealed for AndAnd`

##### `impl Spanned for AndAnd`

- <span id="andand-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AndAnd`

- <span id="andand-toowned-type-owned"></span>`type Owned = T`

- <span id="andand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="andand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for AndAnd`

- <span id="andand-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for AndAnd`

##### `impl<U> TryFrom for AndAnd`

- <span id="andand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="andand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AndAnd`

- <span id="andand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="andand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AndEq`

```rust
struct AndEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`&=`

Usage:
 bitwise AND assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for AndEq`

- <span id="andeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AndEq`

- <span id="andeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AndEq`

- <span id="andeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AndEq`

- <span id="andeq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for AndEq`

- <span id="andeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AndEq`

##### `impl Debug for AndEq`

- <span id="andeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AndEq`

- <span id="andeq-default"></span>`fn default() -> Self`

##### `impl Eq for AndEq`

##### `impl<T> From for AndEq`

- <span id="andeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for AndEq`

- <span id="andeq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for AndEq`

- <span id="andeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for AndEq`

- <span id="andeq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for AndEq`

- <span id="andeq-partialeq-eq"></span>`fn eq(&self, _other: &AndEq) -> bool` — [`AndEq`](#andeq)

##### `impl Sealed for AndEq`

##### `impl Spanned for AndEq`

- <span id="andeq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for AndEq`

- <span id="andeq-toowned-type-owned"></span>`type Owned = T`

- <span id="andeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="andeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for AndEq`

- <span id="andeq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for AndEq`

##### `impl<U> TryFrom for AndEq`

- <span id="andeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="andeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AndEq`

- <span id="andeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="andeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `At`

```rust
struct At {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`@`

Usage:
 subpattern binding.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for At`

- <span id="at-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for At`

- <span id="at-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for At`

- <span id="at-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for At`

- <span id="at-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for At`

- <span id="at-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for At`

##### `impl Debug for At`

- <span id="at-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for At`

- <span id="at-default"></span>`fn default() -> Self`

##### `impl Deref for At`

- <span id="at-deref-type-target"></span>`type Target = WithSpan`

- <span id="at-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for At`

- <span id="at-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for At`

##### `impl<T> From for At`

- <span id="at-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for At`

- <span id="at-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for At`

- <span id="at-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for At`

- <span id="at-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for At`

- <span id="at-partialeq-eq"></span>`fn eq(&self, _other: &At) -> bool` — [`At`](#at)

##### `impl Receiver for At`

- <span id="at-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for At`

##### `impl Spanned for At`

- <span id="at-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for At`

- <span id="at-toowned-type-owned"></span>`type Owned = T`

- <span id="at-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="at-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for At`

- <span id="at-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for At`

##### `impl<U> TryFrom for At`

- <span id="at-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="at-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for At`

- <span id="at-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="at-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Caret`

```rust
struct Caret {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`^`

Usage:
 bitwise and logical XOR.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Caret`

- <span id="caret-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Caret`

- <span id="caret-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Caret`

- <span id="caret-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Caret`

- <span id="caret-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Caret`

- <span id="caret-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Caret`

##### `impl Debug for Caret`

- <span id="caret-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Caret`

- <span id="caret-default"></span>`fn default() -> Self`

##### `impl Deref for Caret`

- <span id="caret-deref-type-target"></span>`type Target = WithSpan`

- <span id="caret-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Caret`

- <span id="caret-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Caret`

##### `impl<T> From for Caret`

- <span id="caret-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Caret`

- <span id="caret-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Caret`

- <span id="caret-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Caret`

- <span id="caret-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Caret`

- <span id="caret-partialeq-eq"></span>`fn eq(&self, _other: &Caret) -> bool` — [`Caret`](#caret)

##### `impl Receiver for Caret`

- <span id="caret-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Caret`

##### `impl Spanned for Caret`

- <span id="caret-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Caret`

- <span id="caret-toowned-type-owned"></span>`type Owned = T`

- <span id="caret-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="caret-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Caret`

- <span id="caret-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Caret`

##### `impl<U> TryFrom for Caret`

- <span id="caret-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="caret-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Caret`

- <span id="caret-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="caret-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaretEq`

```rust
struct CaretEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`^=`

Usage:
 bitwise XOR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for CaretEq`

- <span id="careteq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaretEq`

- <span id="careteq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaretEq`

- <span id="careteq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CaretEq`

- <span id="careteq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for CaretEq`

- <span id="careteq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CaretEq`

##### `impl Debug for CaretEq`

- <span id="careteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CaretEq`

- <span id="careteq-default"></span>`fn default() -> Self`

##### `impl Eq for CaretEq`

##### `impl<T> From for CaretEq`

- <span id="careteq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for CaretEq`

- <span id="careteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for CaretEq`

- <span id="careteq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for CaretEq`

- <span id="careteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for CaretEq`

- <span id="careteq-partialeq-eq"></span>`fn eq(&self, _other: &CaretEq) -> bool` — [`CaretEq`](#careteq)

##### `impl Sealed for CaretEq`

##### `impl Spanned for CaretEq`

- <span id="careteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for CaretEq`

- <span id="careteq-toowned-type-owned"></span>`type Owned = T`

- <span id="careteq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="careteq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for CaretEq`

- <span id="careteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for CaretEq`

##### `impl<U> TryFrom for CaretEq`

- <span id="careteq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="careteq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaretEq`

- <span id="careteq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="careteq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Colon`

```rust
struct Colon {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`:`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Colon`

- <span id="colon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Colon`

- <span id="colon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Colon`

- <span id="colon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Colon`

- <span id="colon-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Colon`

- <span id="colon-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Colon`

##### `impl Debug for Colon`

- <span id="colon-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Colon`

- <span id="colon-default"></span>`fn default() -> Self`

##### `impl Deref for Colon`

- <span id="colon-deref-type-target"></span>`type Target = WithSpan`

- <span id="colon-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Colon`

- <span id="colon-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Colon`

##### `impl<T> From for Colon`

- <span id="colon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Colon`

- <span id="colon-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Colon`

- <span id="colon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Colon`

- <span id="colon-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Colon`

- <span id="colon-partialeq-eq"></span>`fn eq(&self, _other: &Colon) -> bool` — [`Colon`](#colon)

##### `impl Receiver for Colon`

- <span id="colon-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Colon`

##### `impl Spanned for Colon`

- <span id="colon-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Colon`

- <span id="colon-toowned-type-owned"></span>`type Owned = T`

- <span id="colon-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="colon-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Colon`

- <span id="colon-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Colon`

##### `impl<U> TryFrom for Colon`

- <span id="colon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="colon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Colon`

- <span id="colon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="colon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Comma`

```rust
struct Comma {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`,`

Usage:
 various separators.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Comma`

- <span id="comma-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Comma`

- <span id="comma-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Comma`

- <span id="comma-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Comma`

- <span id="comma-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Comma`

- <span id="comma-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Comma`

##### `impl Debug for Comma`

- <span id="comma-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Comma`

- <span id="comma-default"></span>`fn default() -> Self`

##### `impl Deref for Comma`

- <span id="comma-deref-type-target"></span>`type Target = WithSpan`

- <span id="comma-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Comma`

- <span id="comma-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Comma`

##### `impl<T> From for Comma`

- <span id="comma-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Comma`

- <span id="comma-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Comma`

- <span id="comma-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Comma`

- <span id="comma-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Comma`

- <span id="comma-partialeq-eq"></span>`fn eq(&self, _other: &Comma) -> bool` — [`Comma`](#comma)

##### `impl Receiver for Comma`

- <span id="comma-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Comma`

##### `impl Spanned for Comma`

- <span id="comma-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Comma`

- <span id="comma-toowned-type-owned"></span>`type Owned = T`

- <span id="comma-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="comma-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Comma`

- <span id="comma-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Comma`

##### `impl<U> TryFrom for Comma`

- <span id="comma-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="comma-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Comma`

- <span id="comma-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="comma-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dollar`

```rust
struct Dollar {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`$`

Usage:
 macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Dollar`

- <span id="dollar-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dollar`

- <span id="dollar-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dollar`

- <span id="dollar-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dollar`

- <span id="dollar-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Dollar`

- <span id="dollar-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Dollar`

##### `impl Debug for Dollar`

- <span id="dollar-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dollar`

- <span id="dollar-default"></span>`fn default() -> Self`

##### `impl Deref for Dollar`

- <span id="dollar-deref-type-target"></span>`type Target = WithSpan`

- <span id="dollar-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Dollar`

- <span id="dollar-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Dollar`

##### `impl<T> From for Dollar`

- <span id="dollar-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Dollar`

- <span id="dollar-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Dollar`

- <span id="dollar-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Dollar`

- <span id="dollar-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dollar`

- <span id="dollar-partialeq-eq"></span>`fn eq(&self, _other: &Dollar) -> bool` — [`Dollar`](#dollar)

##### `impl Receiver for Dollar`

- <span id="dollar-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Dollar`

##### `impl Spanned for Dollar`

- <span id="dollar-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Dollar`

- <span id="dollar-toowned-type-owned"></span>`type Owned = T`

- <span id="dollar-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dollar-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Dollar`

- <span id="dollar-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dollar`

##### `impl<U> TryFrom for Dollar`

- <span id="dollar-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dollar-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dollar`

- <span id="dollar-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dollar-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Dot`

```rust
struct Dot {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`.`

Usage:
 field access, tuple index.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Dot`

- <span id="dot-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Dot`

- <span id="dot-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Dot`

- <span id="dot-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Dot`

- <span id="dot-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Dot`

- <span id="dot-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Dot`

##### `impl Debug for Dot`

- <span id="dot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Dot`

- <span id="dot-default"></span>`fn default() -> Self`

##### `impl Deref for Dot`

- <span id="dot-deref-type-target"></span>`type Target = WithSpan`

- <span id="dot-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Dot`

- <span id="dot-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Dot`

##### `impl<T> From for Dot`

- <span id="dot-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Dot`

- <span id="dot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Dot`

- <span id="dot-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Dot`

- <span id="dot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Dot`

- <span id="dot-partialeq-eq"></span>`fn eq(&self, _other: &Dot) -> bool` — [`Dot`](#dot)

##### `impl Receiver for Dot`

- <span id="dot-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Dot`

##### `impl Spanned for Dot`

- <span id="dot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Dot`

- <span id="dot-toowned-type-owned"></span>`type Owned = T`

- <span id="dot-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dot-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Dot`

- <span id="dot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Dot`

##### `impl<U> TryFrom for Dot`

- <span id="dot-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dot-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Dot`

- <span id="dot-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dot-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DotDot`

```rust
struct DotDot {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`..`

Usage:
 range, struct expressions, patterns, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for DotDot`

- <span id="dotdot-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DotDot`

- <span id="dotdot-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DotDot`

- <span id="dotdot-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DotDot`

- <span id="dotdot-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DotDot`

- <span id="dotdot-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DotDot`

##### `impl Debug for DotDot`

- <span id="dotdot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDot`

- <span id="dotdot-default"></span>`fn default() -> Self`

##### `impl Eq for DotDot`

##### `impl<T> From for DotDot`

- <span id="dotdot-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DotDot`

- <span id="dotdot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for DotDot`

- <span id="dotdot-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for DotDot`

- <span id="dotdot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDot`

- <span id="dotdot-partialeq-eq"></span>`fn eq(&self, _other: &DotDot) -> bool` — [`DotDot`](#dotdot)

##### `impl Sealed for DotDot`

##### `impl Spanned for DotDot`

- <span id="dotdot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for DotDot`

- <span id="dotdot-toowned-type-owned"></span>`type Owned = T`

- <span id="dotdot-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dotdot-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for DotDot`

- <span id="dotdot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDot`

##### `impl<U> TryFrom for DotDot`

- <span id="dotdot-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dotdot-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DotDot`

- <span id="dotdot-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dotdot-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DotDotDot`

```rust
struct DotDotDot {
    pub spans: [Span; 3],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`...`

Usage:
 variadic functions, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for DotDotDot`

- <span id="dotdotdot-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DotDotDot`

- <span id="dotdotdot-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DotDotDot`

- <span id="dotdotdot-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DotDotDot`

- <span id="dotdotdot-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DotDotDot`

- <span id="dotdotdot-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DotDotDot`

##### `impl Debug for DotDotDot`

- <span id="dotdotdot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotDot`

- <span id="dotdotdot-default"></span>`fn default() -> Self`

##### `impl Eq for DotDotDot`

##### `impl<T> From for DotDotDot`

- <span id="dotdotdot-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DotDotDot`

- <span id="dotdotdot-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for DotDotDot`

- <span id="dotdotdot-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for DotDotDot`

- <span id="dotdotdot-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDotDot`

- <span id="dotdotdot-partialeq-eq"></span>`fn eq(&self, _other: &DotDotDot) -> bool` — [`DotDotDot`](#dotdotdot)

##### `impl Sealed for DotDotDot`

##### `impl Spanned for DotDotDot`

- <span id="dotdotdot-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for DotDotDot`

- <span id="dotdotdot-toowned-type-owned"></span>`type Owned = T`

- <span id="dotdotdot-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dotdotdot-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for DotDotDot`

- <span id="dotdotdot-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDotDot`

##### `impl<U> TryFrom for DotDotDot`

- <span id="dotdotdot-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dotdotdot-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DotDotDot`

- <span id="dotdotdot-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dotdotdot-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DotDotEq`

```rust
struct DotDotEq {
    pub spans: [Span; 3],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`..=`

Usage:
 inclusive range, range patterns.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for DotDotEq`

- <span id="dotdoteq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DotDotEq`

- <span id="dotdoteq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DotDotEq`

- <span id="dotdoteq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DotDotEq`

- <span id="dotdoteq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for DotDotEq`

- <span id="dotdoteq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for DotDotEq`

##### `impl Debug for DotDotEq`

- <span id="dotdoteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for DotDotEq`

- <span id="dotdoteq-default"></span>`fn default() -> Self`

##### `impl Eq for DotDotEq`

##### `impl<T> From for DotDotEq`

- <span id="dotdoteq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for DotDotEq`

- <span id="dotdoteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for DotDotEq`

- <span id="dotdoteq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for DotDotEq`

- <span id="dotdoteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for DotDotEq`

- <span id="dotdoteq-partialeq-eq"></span>`fn eq(&self, _other: &DotDotEq) -> bool` — [`DotDotEq`](#dotdoteq)

##### `impl Sealed for DotDotEq`

##### `impl Spanned for DotDotEq`

- <span id="dotdoteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for DotDotEq`

- <span id="dotdoteq-toowned-type-owned"></span>`type Owned = T`

- <span id="dotdoteq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dotdoteq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for DotDotEq`

- <span id="dotdoteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for DotDotEq`

##### `impl<U> TryFrom for DotDotEq`

- <span id="dotdoteq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dotdoteq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DotDotEq`

- <span id="dotdoteq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dotdoteq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Eq`

```rust
struct Eq {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`=`

Usage:
 assignment, attributes, various type definitions.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Eq`

- <span id="eq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Eq`

- <span id="eq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Eq`

- <span id="eq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Eq`

- <span id="eq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Eq`

- <span id="eq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Eq`

##### `impl Debug for Eq`

- <span id="eq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Eq`

- <span id="eq-default"></span>`fn default() -> Self`

##### `impl Deref for Eq`

- <span id="eq-deref-type-target"></span>`type Target = WithSpan`

- <span id="eq-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Eq`

- <span id="eq-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Eq`

##### `impl<T> From for Eq`

- <span id="eq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Eq`

- <span id="eq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Eq`

- <span id="eq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Eq`

- <span id="eq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Eq`

- <span id="eq-partialeq-eq"></span>`fn eq(&self, _other: &Eq) -> bool` — [`Eq`](#eq)

##### `impl Receiver for Eq`

- <span id="eq-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Eq`

##### `impl Spanned for Eq`

- <span id="eq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Eq`

- <span id="eq-toowned-type-owned"></span>`type Owned = T`

- <span id="eq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="eq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Eq`

- <span id="eq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Eq`

##### `impl<U> TryFrom for Eq`

- <span id="eq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="eq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Eq`

- <span id="eq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="eq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EqEq`

```rust
struct EqEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`==`

Usage:
 equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for EqEq`

- <span id="eqeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EqEq`

- <span id="eqeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EqEq`

- <span id="eqeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for EqEq`

- <span id="eqeq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for EqEq`

- <span id="eqeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for EqEq`

##### `impl Debug for EqEq`

- <span id="eqeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EqEq`

- <span id="eqeq-default"></span>`fn default() -> Self`

##### `impl Eq for EqEq`

##### `impl<T> From for EqEq`

- <span id="eqeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for EqEq`

- <span id="eqeq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for EqEq`

- <span id="eqeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for EqEq`

- <span id="eqeq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for EqEq`

- <span id="eqeq-partialeq-eq"></span>`fn eq(&self, _other: &EqEq) -> bool` — [`EqEq`](#eqeq)

##### `impl Sealed for EqEq`

##### `impl Spanned for EqEq`

- <span id="eqeq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for EqEq`

- <span id="eqeq-toowned-type-owned"></span>`type Owned = T`

- <span id="eqeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="eqeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for EqEq`

- <span id="eqeq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for EqEq`

##### `impl<U> TryFrom for EqEq`

- <span id="eqeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="eqeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EqEq`

- <span id="eqeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="eqeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatArrow`

```rust
struct FatArrow {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`=>`

Usage:
 match arms, macros.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for FatArrow`

- <span id="fatarrow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatArrow`

- <span id="fatarrow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatArrow`

- <span id="fatarrow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatArrow`

- <span id="fatarrow-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FatArrow`

- <span id="fatarrow-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for FatArrow`

##### `impl Debug for FatArrow`

- <span id="fatarrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FatArrow`

- <span id="fatarrow-default"></span>`fn default() -> Self`

##### `impl Eq for FatArrow`

##### `impl<T> From for FatArrow`

- <span id="fatarrow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for FatArrow`

- <span id="fatarrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for FatArrow`

- <span id="fatarrow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for FatArrow`

- <span id="fatarrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for FatArrow`

- <span id="fatarrow-partialeq-eq"></span>`fn eq(&self, _other: &FatArrow) -> bool` — [`FatArrow`](#fatarrow)

##### `impl Sealed for FatArrow`

##### `impl Spanned for FatArrow`

- <span id="fatarrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for FatArrow`

- <span id="fatarrow-toowned-type-owned"></span>`type Owned = T`

- <span id="fatarrow-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatarrow-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for FatArrow`

- <span id="fatarrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for FatArrow`

##### `impl<U> TryFrom for FatArrow`

- <span id="fatarrow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatarrow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatArrow`

- <span id="fatarrow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatarrow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ge`

```rust
struct Ge {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`>=`

Usage:
 greater than or equal to, generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Ge`

- <span id="ge-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ge`

- <span id="ge-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ge`

- <span id="ge-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ge`

- <span id="ge-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Ge`

- <span id="ge-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ge`

##### `impl Debug for Ge`

- <span id="ge-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ge`

- <span id="ge-default"></span>`fn default() -> Self`

##### `impl Eq for Ge`

##### `impl<T> From for Ge`

- <span id="ge-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Ge`

- <span id="ge-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Ge`

- <span id="ge-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Ge`

- <span id="ge-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ge`

- <span id="ge-partialeq-eq"></span>`fn eq(&self, _other: &Ge) -> bool` — [`Ge`](#ge)

##### `impl Sealed for Ge`

##### `impl Spanned for Ge`

- <span id="ge-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Ge`

- <span id="ge-toowned-type-owned"></span>`type Owned = T`

- <span id="ge-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ge-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Ge`

- <span id="ge-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ge`

##### `impl<U> TryFrom for Ge`

- <span id="ge-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ge-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ge`

- <span id="ge-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ge-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Gt`

```rust
struct Gt {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`>`

Usage:
 greater than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Gt`

- <span id="gt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Gt`

- <span id="gt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Gt`

- <span id="gt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Gt`

- <span id="gt-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Gt`

- <span id="gt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Gt`

##### `impl Debug for Gt`

- <span id="gt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Gt`

- <span id="gt-default"></span>`fn default() -> Self`

##### `impl Deref for Gt`

- <span id="gt-deref-type-target"></span>`type Target = WithSpan`

- <span id="gt-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Gt`

- <span id="gt-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Gt`

##### `impl<T> From for Gt`

- <span id="gt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Gt`

- <span id="gt-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Gt`

- <span id="gt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Gt`

- <span id="gt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Gt`

- <span id="gt-partialeq-eq"></span>`fn eq(&self, _other: &Gt) -> bool` — [`Gt`](#gt)

##### `impl Receiver for Gt`

- <span id="gt-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Gt`

##### `impl Spanned for Gt`

- <span id="gt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Gt`

- <span id="gt-toowned-type-owned"></span>`type Owned = T`

- <span id="gt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="gt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Gt`

- <span id="gt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Gt`

##### `impl<U> TryFrom for Gt`

- <span id="gt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="gt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Gt`

- <span id="gt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="gt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LArrow`

```rust
struct LArrow {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`<-`

Usage:
 unused.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for LArrow`

- <span id="larrow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LArrow`

- <span id="larrow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LArrow`

- <span id="larrow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LArrow`

- <span id="larrow-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for LArrow`

- <span id="larrow-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LArrow`

##### `impl Debug for LArrow`

- <span id="larrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LArrow`

- <span id="larrow-default"></span>`fn default() -> Self`

##### `impl Eq for LArrow`

##### `impl<T> From for LArrow`

- <span id="larrow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LArrow`

- <span id="larrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for LArrow`

- <span id="larrow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for LArrow`

- <span id="larrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for LArrow`

- <span id="larrow-partialeq-eq"></span>`fn eq(&self, _other: &LArrow) -> bool` — [`LArrow`](#larrow)

##### `impl Sealed for LArrow`

##### `impl Spanned for LArrow`

- <span id="larrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for LArrow`

- <span id="larrow-toowned-type-owned"></span>`type Owned = T`

- <span id="larrow-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="larrow-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for LArrow`

- <span id="larrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for LArrow`

##### `impl<U> TryFrom for LArrow`

- <span id="larrow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="larrow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LArrow`

- <span id="larrow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="larrow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Le`

```rust
struct Le {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`<=`

Usage:
 less than or equal to.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Le`

- <span id="le-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Le`

- <span id="le-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Le`

- <span id="le-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Le`

- <span id="le-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Le`

- <span id="le-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Le`

##### `impl Debug for Le`

- <span id="le-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Le`

- <span id="le-default"></span>`fn default() -> Self`

##### `impl Eq for Le`

##### `impl<T> From for Le`

- <span id="le-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Le`

- <span id="le-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Le`

- <span id="le-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Le`

- <span id="le-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Le`

- <span id="le-partialeq-eq"></span>`fn eq(&self, _other: &Le) -> bool` — [`Le`](#le)

##### `impl Sealed for Le`

##### `impl Spanned for Le`

- <span id="le-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Le`

- <span id="le-toowned-type-owned"></span>`type Owned = T`

- <span id="le-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="le-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Le`

- <span id="le-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Le`

##### `impl<U> TryFrom for Le`

- <span id="le-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="le-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Le`

- <span id="le-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="le-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lt`

```rust
struct Lt {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`<`

Usage:
 less than, generics, paths.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Lt`

- <span id="lt-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lt`

- <span id="lt-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lt`

- <span id="lt-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Lt`

- <span id="lt-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Lt`

- <span id="lt-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Lt`

##### `impl Debug for Lt`

- <span id="lt-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Lt`

- <span id="lt-default"></span>`fn default() -> Self`

##### `impl Deref for Lt`

- <span id="lt-deref-type-target"></span>`type Target = WithSpan`

- <span id="lt-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Lt`

- <span id="lt-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Lt`

##### `impl<T> From for Lt`

- <span id="lt-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Lt`

- <span id="lt-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Lt`

- <span id="lt-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Lt`

- <span id="lt-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Lt`

- <span id="lt-partialeq-eq"></span>`fn eq(&self, _other: &Lt) -> bool` — [`Lt`](#lt)

##### `impl Receiver for Lt`

- <span id="lt-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Lt`

##### `impl Spanned for Lt`

- <span id="lt-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Lt`

- <span id="lt-toowned-type-owned"></span>`type Owned = T`

- <span id="lt-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lt-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Lt`

- <span id="lt-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Lt`

##### `impl<U> TryFrom for Lt`

- <span id="lt-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lt-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lt`

- <span id="lt-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lt-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Minus`

```rust
struct Minus {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`-`

Usage:
 subtraction, negation.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Minus`

- <span id="minus-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Minus`

- <span id="minus-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Minus`

- <span id="minus-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Minus`

- <span id="minus-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Minus`

- <span id="minus-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Minus`

##### `impl Debug for Minus`

- <span id="minus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Minus`

- <span id="minus-default"></span>`fn default() -> Self`

##### `impl Deref for Minus`

- <span id="minus-deref-type-target"></span>`type Target = WithSpan`

- <span id="minus-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Minus`

- <span id="minus-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Minus`

##### `impl<T> From for Minus`

- <span id="minus-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Minus`

- <span id="minus-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Minus`

- <span id="minus-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Minus`

- <span id="minus-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Minus`

- <span id="minus-partialeq-eq"></span>`fn eq(&self, _other: &Minus) -> bool` — [`Minus`](#minus)

##### `impl Receiver for Minus`

- <span id="minus-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Minus`

##### `impl Spanned for Minus`

- <span id="minus-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Minus`

- <span id="minus-toowned-type-owned"></span>`type Owned = T`

- <span id="minus-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minus-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Minus`

- <span id="minus-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Minus`

##### `impl<U> TryFrom for Minus`

- <span id="minus-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minus-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Minus`

- <span id="minus-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minus-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MinusEq`

```rust
struct MinusEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`-=`

Usage:
 subtraction assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for MinusEq`

- <span id="minuseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinusEq`

- <span id="minuseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinusEq`

- <span id="minuseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MinusEq`

- <span id="minuseq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MinusEq`

- <span id="minuseq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MinusEq`

##### `impl Debug for MinusEq`

- <span id="minuseq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MinusEq`

- <span id="minuseq-default"></span>`fn default() -> Self`

##### `impl Eq for MinusEq`

##### `impl<T> From for MinusEq`

- <span id="minuseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for MinusEq`

- <span id="minuseq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for MinusEq`

- <span id="minuseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for MinusEq`

- <span id="minuseq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for MinusEq`

- <span id="minuseq-partialeq-eq"></span>`fn eq(&self, _other: &MinusEq) -> bool` — [`MinusEq`](#minuseq)

##### `impl Sealed for MinusEq`

##### `impl Spanned for MinusEq`

- <span id="minuseq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for MinusEq`

- <span id="minuseq-toowned-type-owned"></span>`type Owned = T`

- <span id="minuseq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minuseq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for MinusEq`

- <span id="minuseq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for MinusEq`

##### `impl<U> TryFrom for MinusEq`

- <span id="minuseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minuseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MinusEq`

- <span id="minuseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minuseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Ne`

```rust
struct Ne {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`!=`

Usage:
 not equal.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Ne`

- <span id="ne-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ne`

- <span id="ne-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ne`

- <span id="ne-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ne`

- <span id="ne-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Ne`

- <span id="ne-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ne`

##### `impl Debug for Ne`

- <span id="ne-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Ne`

- <span id="ne-default"></span>`fn default() -> Self`

##### `impl Eq for Ne`

##### `impl<T> From for Ne`

- <span id="ne-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Ne`

- <span id="ne-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Ne`

- <span id="ne-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Ne`

- <span id="ne-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Ne`

- <span id="ne-partialeq-eq"></span>`fn eq(&self, _other: &Ne) -> bool` — [`Ne`](#ne)

##### `impl Sealed for Ne`

##### `impl Spanned for Ne`

- <span id="ne-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Ne`

- <span id="ne-toowned-type-owned"></span>`type Owned = T`

- <span id="ne-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ne-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Ne`

- <span id="ne-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Ne`

##### `impl<U> TryFrom for Ne`

- <span id="ne-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ne-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ne`

- <span id="ne-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ne-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Not`

```rust
struct Not {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`!`

Usage:
 bitwise and logical NOT, macro calls, inner attributes, never type, negative impls.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Not`

- <span id="not-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Not`

- <span id="not-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Not`

- <span id="not-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Not`

- <span id="not-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Not`

- <span id="not-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Not`

##### `impl Debug for Not`

- <span id="not-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Not`

- <span id="not-default"></span>`fn default() -> Self`

##### `impl Deref for Not`

- <span id="not-deref-type-target"></span>`type Target = WithSpan`

- <span id="not-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Not`

- <span id="not-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Not`

##### `impl<T> From for Not`

- <span id="not-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Not`

- <span id="not-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Not`

- <span id="not-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Not`

- <span id="not-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Not`

- <span id="not-partialeq-eq"></span>`fn eq(&self, _other: &Not) -> bool` — [`Not`](#not)

##### `impl Receiver for Not`

- <span id="not-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Not`

##### `impl Spanned for Not`

- <span id="not-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Not`

- <span id="not-toowned-type-owned"></span>`type Owned = T`

- <span id="not-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="not-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Not`

- <span id="not-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Not`

##### `impl<U> TryFrom for Not`

- <span id="not-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="not-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Not`

- <span id="not-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="not-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Or`

```rust
struct Or {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`|`

Usage:
 bitwise and logical OR, closures, patterns in match, if let, and while let.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Or`

- <span id="or-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Or`

- <span id="or-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Or`

- <span id="or-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Or`

- <span id="or-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Or`

- <span id="or-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Or`

##### `impl Debug for Or`

- <span id="or-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Or`

- <span id="or-default"></span>`fn default() -> Self`

##### `impl Deref for Or`

- <span id="or-deref-type-target"></span>`type Target = WithSpan`

- <span id="or-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Or`

- <span id="or-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Or`

##### `impl<T> From for Or`

- <span id="or-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Or`

- <span id="or-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Or`

- <span id="or-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Or`

- <span id="or-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Or`

- <span id="or-partialeq-eq"></span>`fn eq(&self, _other: &Or) -> bool` — [`Or`](#or)

##### `impl Receiver for Or`

- <span id="or-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Or`

##### `impl Spanned for Or`

- <span id="or-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Or`

- <span id="or-toowned-type-owned"></span>`type Owned = T`

- <span id="or-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="or-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Or`

- <span id="or-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Or`

##### `impl<U> TryFrom for Or`

- <span id="or-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="or-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Or`

- <span id="or-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="or-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OrEq`

```rust
struct OrEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`|=`

Usage:
 bitwise OR assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for OrEq`

- <span id="oreq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OrEq`

- <span id="oreq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OrEq`

- <span id="oreq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OrEq`

- <span id="oreq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for OrEq`

- <span id="oreq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OrEq`

##### `impl Debug for OrEq`

- <span id="oreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrEq`

- <span id="oreq-default"></span>`fn default() -> Self`

##### `impl Eq for OrEq`

##### `impl<T> From for OrEq`

- <span id="oreq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for OrEq`

- <span id="oreq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for OrEq`

- <span id="oreq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for OrEq`

- <span id="oreq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for OrEq`

- <span id="oreq-partialeq-eq"></span>`fn eq(&self, _other: &OrEq) -> bool` — [`OrEq`](#oreq)

##### `impl Sealed for OrEq`

##### `impl Spanned for OrEq`

- <span id="oreq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for OrEq`

- <span id="oreq-toowned-type-owned"></span>`type Owned = T`

- <span id="oreq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oreq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for OrEq`

- <span id="oreq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for OrEq`

##### `impl<U> TryFrom for OrEq`

- <span id="oreq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oreq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OrEq`

- <span id="oreq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oreq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OrOr`

```rust
struct OrOr {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`||`

Usage:
 lazy OR, closures.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for OrOr`

- <span id="oror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OrOr`

- <span id="oror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OrOr`

- <span id="oror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OrOr`

- <span id="oror-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for OrOr`

- <span id="oror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OrOr`

##### `impl Debug for OrOr`

- <span id="oror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OrOr`

- <span id="oror-default"></span>`fn default() -> Self`

##### `impl Eq for OrOr`

##### `impl<T> From for OrOr`

- <span id="oror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for OrOr`

- <span id="oror-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for OrOr`

- <span id="oror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for OrOr`

- <span id="oror-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for OrOr`

- <span id="oror-partialeq-eq"></span>`fn eq(&self, _other: &OrOr) -> bool` — [`OrOr`](#oror)

##### `impl Sealed for OrOr`

##### `impl Spanned for OrOr`

- <span id="oror-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for OrOr`

- <span id="oror-toowned-type-owned"></span>`type Owned = T`

- <span id="oror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for OrOr`

- <span id="oror-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for OrOr`

##### `impl<U> TryFrom for OrOr`

- <span id="oror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OrOr`

- <span id="oror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PathSep`

```rust
struct PathSep {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`::`

Usage:
 path separator.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for PathSep`

- <span id="pathsep-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PathSep`

- <span id="pathsep-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PathSep`

- <span id="pathsep-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PathSep`

- <span id="pathsep-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PathSep`

- <span id="pathsep-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PathSep`

##### `impl Debug for PathSep`

- <span id="pathsep-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathSep`

- <span id="pathsep-default"></span>`fn default() -> Self`

##### `impl Eq for PathSep`

##### `impl<T> From for PathSep`

- <span id="pathsep-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PathSep`

- <span id="pathsep-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for PathSep`

- <span id="pathsep-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for PathSep`

- <span id="pathsep-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PathSep`

- <span id="pathsep-partialeq-eq"></span>`fn eq(&self, _other: &PathSep) -> bool` — [`PathSep`](#pathsep)

##### `impl Sealed for PathSep`

##### `impl Spanned for PathSep`

- <span id="pathsep-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PathSep`

- <span id="pathsep-toowned-type-owned"></span>`type Owned = T`

- <span id="pathsep-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pathsep-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for PathSep`

- <span id="pathsep-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PathSep`

##### `impl<U> TryFrom for PathSep`

- <span id="pathsep-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pathsep-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PathSep`

- <span id="pathsep-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pathsep-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Percent`

```rust
struct Percent {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`%`

Usage:
 remainder.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Percent`

- <span id="percent-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Percent`

- <span id="percent-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Percent`

- <span id="percent-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Percent`

- <span id="percent-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Percent`

- <span id="percent-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Percent`

##### `impl Debug for Percent`

- <span id="percent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Percent`

- <span id="percent-default"></span>`fn default() -> Self`

##### `impl Deref for Percent`

- <span id="percent-deref-type-target"></span>`type Target = WithSpan`

- <span id="percent-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Percent`

- <span id="percent-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Percent`

##### `impl<T> From for Percent`

- <span id="percent-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Percent`

- <span id="percent-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Percent`

- <span id="percent-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Percent`

- <span id="percent-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Percent`

- <span id="percent-partialeq-eq"></span>`fn eq(&self, _other: &Percent) -> bool` — [`Percent`](#percent)

##### `impl Receiver for Percent`

- <span id="percent-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Percent`

##### `impl Spanned for Percent`

- <span id="percent-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Percent`

- <span id="percent-toowned-type-owned"></span>`type Owned = T`

- <span id="percent-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="percent-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Percent`

- <span id="percent-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Percent`

##### `impl<U> TryFrom for Percent`

- <span id="percent-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="percent-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Percent`

- <span id="percent-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="percent-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PercentEq`

```rust
struct PercentEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`%=`

Usage:
 remainder assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for PercentEq`

- <span id="percenteq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PercentEq`

- <span id="percenteq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PercentEq`

- <span id="percenteq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PercentEq`

- <span id="percenteq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PercentEq`

- <span id="percenteq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PercentEq`

##### `impl Debug for PercentEq`

- <span id="percenteq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PercentEq`

- <span id="percenteq-default"></span>`fn default() -> Self`

##### `impl Eq for PercentEq`

##### `impl<T> From for PercentEq`

- <span id="percenteq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PercentEq`

- <span id="percenteq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for PercentEq`

- <span id="percenteq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for PercentEq`

- <span id="percenteq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PercentEq`

- <span id="percenteq-partialeq-eq"></span>`fn eq(&self, _other: &PercentEq) -> bool` — [`PercentEq`](#percenteq)

##### `impl Sealed for PercentEq`

##### `impl Spanned for PercentEq`

- <span id="percenteq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PercentEq`

- <span id="percenteq-toowned-type-owned"></span>`type Owned = T`

- <span id="percenteq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="percenteq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for PercentEq`

- <span id="percenteq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PercentEq`

##### `impl<U> TryFrom for PercentEq`

- <span id="percenteq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="percenteq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PercentEq`

- <span id="percenteq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="percenteq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Plus`

```rust
struct Plus {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`+`

Usage:
 addition, trait bounds, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Plus`

- <span id="plus-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Plus`

- <span id="plus-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Plus`

- <span id="plus-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Plus`

- <span id="plus-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Plus`

- <span id="plus-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Plus`

##### `impl Debug for Plus`

- <span id="plus-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Plus`

- <span id="plus-default"></span>`fn default() -> Self`

##### `impl Deref for Plus`

- <span id="plus-deref-type-target"></span>`type Target = WithSpan`

- <span id="plus-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Plus`

- <span id="plus-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Plus`

##### `impl<T> From for Plus`

- <span id="plus-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Plus`

- <span id="plus-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Plus`

- <span id="plus-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Plus`

- <span id="plus-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Plus`

- <span id="plus-partialeq-eq"></span>`fn eq(&self, _other: &Plus) -> bool` — [`Plus`](#plus)

##### `impl Receiver for Plus`

- <span id="plus-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Plus`

##### `impl Spanned for Plus`

- <span id="plus-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Plus`

- <span id="plus-toowned-type-owned"></span>`type Owned = T`

- <span id="plus-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="plus-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Plus`

- <span id="plus-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Plus`

##### `impl<U> TryFrom for Plus`

- <span id="plus-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="plus-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Plus`

- <span id="plus-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="plus-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PlusEq`

```rust
struct PlusEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`+=`

Usage:
 addition assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for PlusEq`

- <span id="pluseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PlusEq`

- <span id="pluseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PlusEq`

- <span id="pluseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PlusEq`

- <span id="pluseq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for PlusEq`

- <span id="pluseq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PlusEq`

##### `impl Debug for PlusEq`

- <span id="pluseq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PlusEq`

- <span id="pluseq-default"></span>`fn default() -> Self`

##### `impl Eq for PlusEq`

##### `impl<T> From for PlusEq`

- <span id="pluseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PlusEq`

- <span id="pluseq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for PlusEq`

- <span id="pluseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for PlusEq`

- <span id="pluseq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for PlusEq`

- <span id="pluseq-partialeq-eq"></span>`fn eq(&self, _other: &PlusEq) -> bool` — [`PlusEq`](#pluseq)

##### `impl Sealed for PlusEq`

##### `impl Spanned for PlusEq`

- <span id="pluseq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for PlusEq`

- <span id="pluseq-toowned-type-owned"></span>`type Owned = T`

- <span id="pluseq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pluseq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for PlusEq`

- <span id="pluseq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for PlusEq`

##### `impl<U> TryFrom for PlusEq`

- <span id="pluseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pluseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PlusEq`

- <span id="pluseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pluseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pound`

```rust
struct Pound {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`#`

Usage:
 attributes.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Pound`

- <span id="pound-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pound`

- <span id="pound-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pound`

- <span id="pound-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pound`

- <span id="pound-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Pound`

- <span id="pound-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pound`

##### `impl Debug for Pound`

- <span id="pound-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pound`

- <span id="pound-default"></span>`fn default() -> Self`

##### `impl Deref for Pound`

- <span id="pound-deref-type-target"></span>`type Target = WithSpan`

- <span id="pound-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Pound`

- <span id="pound-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Pound`

##### `impl<T> From for Pound`

- <span id="pound-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Pound`

- <span id="pound-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Pound`

- <span id="pound-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Pound`

- <span id="pound-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Pound`

- <span id="pound-partialeq-eq"></span>`fn eq(&self, _other: &Pound) -> bool` — [`Pound`](#pound)

##### `impl Receiver for Pound`

- <span id="pound-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Pound`

##### `impl Spanned for Pound`

- <span id="pound-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Pound`

- <span id="pound-toowned-type-owned"></span>`type Owned = T`

- <span id="pound-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pound-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Pound`

- <span id="pound-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Pound`

##### `impl<U> TryFrom for Pound`

- <span id="pound-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pound-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pound`

- <span id="pound-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pound-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Question`

```rust
struct Question {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`?`

Usage:
 question mark operator, questionably sized, macro Kleene matcher.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Question`

- <span id="question-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Question`

- <span id="question-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Question`

- <span id="question-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Question`

- <span id="question-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Question`

- <span id="question-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Question`

##### `impl Debug for Question`

- <span id="question-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Question`

- <span id="question-default"></span>`fn default() -> Self`

##### `impl Deref for Question`

- <span id="question-deref-type-target"></span>`type Target = WithSpan`

- <span id="question-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Question`

- <span id="question-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Question`

##### `impl<T> From for Question`

- <span id="question-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Question`

- <span id="question-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Question`

- <span id="question-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Question`

- <span id="question-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Question`

- <span id="question-partialeq-eq"></span>`fn eq(&self, _other: &Question) -> bool` — [`Question`](#question)

##### `impl Receiver for Question`

- <span id="question-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Question`

##### `impl Spanned for Question`

- <span id="question-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Question`

- <span id="question-toowned-type-owned"></span>`type Owned = T`

- <span id="question-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="question-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Question`

- <span id="question-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Question`

##### `impl<U> TryFrom for Question`

- <span id="question-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="question-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Question`

- <span id="question-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="question-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RArrow`

```rust
struct RArrow {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`->`

Usage:
 function return type, closure return type, function pointer type.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for RArrow`

- <span id="rarrow-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RArrow`

- <span id="rarrow-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RArrow`

- <span id="rarrow-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RArrow`

- <span id="rarrow-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for RArrow`

- <span id="rarrow-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RArrow`

##### `impl Debug for RArrow`

- <span id="rarrow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RArrow`

- <span id="rarrow-default"></span>`fn default() -> Self`

##### `impl Eq for RArrow`

##### `impl<T> From for RArrow`

- <span id="rarrow-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for RArrow`

- <span id="rarrow-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for RArrow`

- <span id="rarrow-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for RArrow`

- <span id="rarrow-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for RArrow`

- <span id="rarrow-partialeq-eq"></span>`fn eq(&self, _other: &RArrow) -> bool` — [`RArrow`](#rarrow)

##### `impl Sealed for RArrow`

##### `impl Spanned for RArrow`

- <span id="rarrow-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for RArrow`

- <span id="rarrow-toowned-type-owned"></span>`type Owned = T`

- <span id="rarrow-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarrow-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for RArrow`

- <span id="rarrow-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for RArrow`

##### `impl<U> TryFrom for RArrow`

- <span id="rarrow-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarrow-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RArrow`

- <span id="rarrow-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarrow-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Semi`

```rust
struct Semi {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`;`

Usage:
 terminator for various items and statements, array types.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Semi`

- <span id="semi-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Semi`

- <span id="semi-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Semi`

- <span id="semi-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Semi`

- <span id="semi-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Semi`

- <span id="semi-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Semi`

##### `impl Debug for Semi`

- <span id="semi-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Semi`

- <span id="semi-default"></span>`fn default() -> Self`

##### `impl Deref for Semi`

- <span id="semi-deref-type-target"></span>`type Target = WithSpan`

- <span id="semi-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Semi`

- <span id="semi-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Semi`

##### `impl<T> From for Semi`

- <span id="semi-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Semi`

- <span id="semi-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Semi`

- <span id="semi-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Semi`

- <span id="semi-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Semi`

- <span id="semi-partialeq-eq"></span>`fn eq(&self, _other: &Semi) -> bool` — [`Semi`](#semi)

##### `impl Receiver for Semi`

- <span id="semi-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Semi`

##### `impl Spanned for Semi`

- <span id="semi-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Semi`

- <span id="semi-toowned-type-owned"></span>`type Owned = T`

- <span id="semi-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="semi-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Semi`

- <span id="semi-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Semi`

##### `impl<U> TryFrom for Semi`

- <span id="semi-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="semi-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Semi`

- <span id="semi-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="semi-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Shl`

```rust
struct Shl {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`<<`

Usage:
 shift left, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Shl`

- <span id="shl-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Shl`

- <span id="shl-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Shl`

- <span id="shl-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Shl`

- <span id="shl-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Shl`

- <span id="shl-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Shl`

##### `impl Debug for Shl`

- <span id="shl-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shl`

- <span id="shl-default"></span>`fn default() -> Self`

##### `impl Eq for Shl`

##### `impl<T> From for Shl`

- <span id="shl-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Shl`

- <span id="shl-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Shl`

- <span id="shl-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Shl`

- <span id="shl-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Shl`

- <span id="shl-partialeq-eq"></span>`fn eq(&self, _other: &Shl) -> bool` — [`Shl`](#shl)

##### `impl Sealed for Shl`

##### `impl Spanned for Shl`

- <span id="shl-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Shl`

- <span id="shl-toowned-type-owned"></span>`type Owned = T`

- <span id="shl-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shl-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Shl`

- <span id="shl-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Shl`

##### `impl<U> TryFrom for Shl`

- <span id="shl-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shl-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Shl`

- <span id="shl-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shl-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ShlEq`

```rust
struct ShlEq {
    pub spans: [Span; 3],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`<<=`

Usage:
 shift left assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for ShlEq`

- <span id="shleq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ShlEq`

- <span id="shleq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ShlEq`

- <span id="shleq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ShlEq`

- <span id="shleq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ShlEq`

- <span id="shleq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ShlEq`

##### `impl Debug for ShlEq`

- <span id="shleq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShlEq`

- <span id="shleq-default"></span>`fn default() -> Self`

##### `impl Eq for ShlEq`

##### `impl<T> From for ShlEq`

- <span id="shleq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ShlEq`

- <span id="shleq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for ShlEq`

- <span id="shleq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for ShlEq`

- <span id="shleq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for ShlEq`

- <span id="shleq-partialeq-eq"></span>`fn eq(&self, _other: &ShlEq) -> bool` — [`ShlEq`](#shleq)

##### `impl Sealed for ShlEq`

##### `impl Spanned for ShlEq`

- <span id="shleq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ShlEq`

- <span id="shleq-toowned-type-owned"></span>`type Owned = T`

- <span id="shleq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shleq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ShlEq`

- <span id="shleq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for ShlEq`

##### `impl<U> TryFrom for ShlEq`

- <span id="shleq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shleq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ShlEq`

- <span id="shleq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shleq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Shr`

```rust
struct Shr {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`>>`

Usage:
 shift right, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Shr`

- <span id="shr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Shr`

- <span id="shr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Shr`

- <span id="shr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Shr`

- <span id="shr-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Shr`

- <span id="shr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Shr`

##### `impl Debug for Shr`

- <span id="shr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Shr`

- <span id="shr-default"></span>`fn default() -> Self`

##### `impl Eq for Shr`

##### `impl<T> From for Shr`

- <span id="shr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Shr`

- <span id="shr-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Shr`

- <span id="shr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Shr`

- <span id="shr-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Shr`

- <span id="shr-partialeq-eq"></span>`fn eq(&self, _other: &Shr) -> bool` — [`Shr`](#shr)

##### `impl Sealed for Shr`

##### `impl Spanned for Shr`

- <span id="shr-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Shr`

- <span id="shr-toowned-type-owned"></span>`type Owned = T`

- <span id="shr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Shr`

- <span id="shr-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Shr`

##### `impl<U> TryFrom for Shr`

- <span id="shr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Shr`

- <span id="shr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ShrEq`

```rust
struct ShrEq {
    pub spans: [Span; 3],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`>>=`

Usage:
 shift right assignment, nested generics.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for ShrEq`

- <span id="shreq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ShrEq`

- <span id="shreq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ShrEq`

- <span id="shreq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ShrEq`

- <span id="shreq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ShrEq`

- <span id="shreq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ShrEq`

##### `impl Debug for ShrEq`

- <span id="shreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ShrEq`

- <span id="shreq-default"></span>`fn default() -> Self`

##### `impl Eq for ShrEq`

##### `impl<T> From for ShrEq`

- <span id="shreq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ShrEq`

- <span id="shreq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for ShrEq`

- <span id="shreq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for ShrEq`

- <span id="shreq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for ShrEq`

- <span id="shreq-partialeq-eq"></span>`fn eq(&self, _other: &ShrEq) -> bool` — [`ShrEq`](#shreq)

##### `impl Sealed for ShrEq`

##### `impl Spanned for ShrEq`

- <span id="shreq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for ShrEq`

- <span id="shreq-toowned-type-owned"></span>`type Owned = T`

- <span id="shreq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shreq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for ShrEq`

- <span id="shreq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for ShrEq`

##### `impl<U> TryFrom for ShrEq`

- <span id="shreq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shreq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ShrEq`

- <span id="shreq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shreq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Slash`

```rust
struct Slash {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`/`

Usage:
 division.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Slash`

- <span id="slash-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Slash`

- <span id="slash-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Slash`

- <span id="slash-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Slash`

- <span id="slash-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Slash`

- <span id="slash-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Slash`

##### `impl Debug for Slash`

- <span id="slash-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Slash`

- <span id="slash-default"></span>`fn default() -> Self`

##### `impl Deref for Slash`

- <span id="slash-deref-type-target"></span>`type Target = WithSpan`

- <span id="slash-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Slash`

- <span id="slash-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Slash`

##### `impl<T> From for Slash`

- <span id="slash-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Slash`

- <span id="slash-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Slash`

- <span id="slash-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Slash`

- <span id="slash-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Slash`

- <span id="slash-partialeq-eq"></span>`fn eq(&self, _other: &Slash) -> bool` — [`Slash`](#slash)

##### `impl Receiver for Slash`

- <span id="slash-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Slash`

##### `impl Spanned for Slash`

- <span id="slash-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Slash`

- <span id="slash-toowned-type-owned"></span>`type Owned = T`

- <span id="slash-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slash-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Slash`

- <span id="slash-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Slash`

##### `impl<U> TryFrom for Slash`

- <span id="slash-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slash-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Slash`

- <span id="slash-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slash-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlashEq`

```rust
struct SlashEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`/=`

Usage:
 division assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for SlashEq`

- <span id="slasheq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlashEq`

- <span id="slasheq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlashEq`

- <span id="slasheq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SlashEq`

- <span id="slasheq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for SlashEq`

- <span id="slasheq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SlashEq`

##### `impl Debug for SlashEq`

- <span id="slasheq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SlashEq`

- <span id="slasheq-default"></span>`fn default() -> Self`

##### `impl Eq for SlashEq`

##### `impl<T> From for SlashEq`

- <span id="slasheq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SlashEq`

- <span id="slasheq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for SlashEq`

- <span id="slasheq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for SlashEq`

- <span id="slasheq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for SlashEq`

- <span id="slasheq-partialeq-eq"></span>`fn eq(&self, _other: &SlashEq) -> bool` — [`SlashEq`](#slasheq)

##### `impl Sealed for SlashEq`

##### `impl Spanned for SlashEq`

- <span id="slasheq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for SlashEq`

- <span id="slasheq-toowned-type-owned"></span>`type Owned = T`

- <span id="slasheq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slasheq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for SlashEq`

- <span id="slasheq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for SlashEq`

##### `impl<U> TryFrom for SlashEq`

- <span id="slasheq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slasheq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlashEq`

- <span id="slasheq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slasheq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Star`

```rust
struct Star {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`*`

Usage:
 multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Star`

- <span id="star-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Star`

- <span id="star-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Star`

- <span id="star-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Star`

- <span id="star-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Star`

- <span id="star-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Star`

##### `impl Debug for Star`

- <span id="star-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Star`

- <span id="star-default"></span>`fn default() -> Self`

##### `impl Deref for Star`

- <span id="star-deref-type-target"></span>`type Target = WithSpan`

- <span id="star-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Star`

- <span id="star-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Star`

##### `impl<T> From for Star`

- <span id="star-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Star`

- <span id="star-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Star`

- <span id="star-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Star`

- <span id="star-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Star`

- <span id="star-partialeq-eq"></span>`fn eq(&self, _other: &Star) -> bool` — [`Star`](#star)

##### `impl Receiver for Star`

- <span id="star-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Star`

##### `impl Spanned for Star`

- <span id="star-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Star`

- <span id="star-toowned-type-owned"></span>`type Owned = T`

- <span id="star-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="star-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Star`

- <span id="star-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Star`

##### `impl<U> TryFrom for Star`

- <span id="star-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="star-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Star`

- <span id="star-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="star-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StarEq`

```rust
struct StarEq {
    pub spans: [Span; 2],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`*=`

Usage:
 multiplication assignment.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for StarEq`

- <span id="stareq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StarEq`

- <span id="stareq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StarEq`

- <span id="stareq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StarEq`

- <span id="stareq-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for StarEq`

- <span id="stareq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StarEq`

##### `impl Debug for StarEq`

- <span id="stareq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StarEq`

- <span id="stareq-default"></span>`fn default() -> Self`

##### `impl Eq for StarEq`

##### `impl<T> From for StarEq`

- <span id="stareq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for StarEq`

- <span id="stareq-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for StarEq`

- <span id="stareq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for StarEq`

- <span id="stareq-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for StarEq`

- <span id="stareq-partialeq-eq"></span>`fn eq(&self, _other: &StarEq) -> bool` — [`StarEq`](#stareq)

##### `impl Sealed for StarEq`

##### `impl Spanned for StarEq`

- <span id="stareq-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for StarEq`

- <span id="stareq-toowned-type-owned"></span>`type Owned = T`

- <span id="stareq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stareq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for StarEq`

- <span id="stareq-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for StarEq`

##### `impl<U> TryFrom for StarEq`

- <span id="stareq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stareq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StarEq`

- <span id="stareq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stareq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tilde`

```rust
struct Tilde {
    pub spans: [Span; 1],
}
```

*Defined in [`syn-2.0.111/src/token.rs:748-795`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L748-L795)*

`~`

Usage:
 unused since before Rust 1.0.

Don't try to remember the name of this type &mdash; use the
`Token!` macro instead.


#### Trait Implementations

##### `impl Any for Tilde`

- <span id="tilde-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tilde`

- <span id="tilde-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tilde`

- <span id="tilde-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Tilde`

- <span id="tilde-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Tilde`

- <span id="tilde-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Tilde`

##### `impl Debug for Tilde`

- <span id="tilde-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Tilde`

- <span id="tilde-default"></span>`fn default() -> Self`

##### `impl Deref for Tilde`

- <span id="tilde-deref-type-target"></span>`type Target = WithSpan`

- <span id="tilde-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for Tilde`

- <span id="tilde-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Eq for Tilde`

##### `impl<T> From for Tilde`

- <span id="tilde-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Tilde`

- <span id="tilde-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Tilde`

- <span id="tilde-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Parse for Tilde`

- <span id="tilde-parse"></span>`fn parse(input: ParseStream<'_>) -> Result<Self>` — [`ParseStream`](../parse/index.md#parsestream), [`Result`](../error/index.md#result)

##### `impl PartialEq for Tilde`

- <span id="tilde-partialeq-eq"></span>`fn eq(&self, _other: &Tilde) -> bool` — [`Tilde`](#tilde)

##### `impl Receiver for Tilde`

- <span id="tilde-receiver-type-target"></span>`type Target = T`

##### `impl Sealed for Tilde`

##### `impl Spanned for Tilde`

- <span id="tilde-spanned-span"></span>`fn span(&self) -> Span`

##### `impl ToOwned for Tilde`

- <span id="tilde-toowned-type-owned"></span>`type Owned = T`

- <span id="tilde-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tilde-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToTokens for Tilde`

- <span id="tilde-totokens-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

##### `impl Token for Tilde`

##### `impl<U> TryFrom for Tilde`

- <span id="tilde-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tilde-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tilde`

- <span id="tilde-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tilde-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Brace`

```rust
struct Brace {
    pub span: DelimSpan,
}
```

*Defined in [`syn-2.0.111/src/token.rs:797-801`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L797-L801)*

`{`&hellip;`}`

#### Implementations

- <span id="brace-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Any for Brace`

- <span id="brace-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Brace`

- <span id="brace-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Brace`

- <span id="brace-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Brace`

- <span id="brace-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Brace`

- <span id="brace-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Brace`

##### `impl Debug for Brace`

- <span id="brace-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Brace`

- <span id="brace-default"></span>`fn default() -> Self`

##### `impl Eq for Brace`

##### `impl<T> From for Brace`

- <span id="brace-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Brace`

- <span id="brace-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Brace`

- <span id="brace-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Brace`

- <span id="brace-partialeq-eq"></span>`fn eq(&self, _other: &Brace) -> bool` — [`Brace`](#brace)

##### `impl Sealed for Brace`

##### `impl ToOwned for Brace`

- <span id="brace-toowned-type-owned"></span>`type Owned = T`

- <span id="brace-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="brace-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl Token for Brace`

##### `impl<U> TryFrom for Brace`

- <span id="brace-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="brace-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Brace`

- <span id="brace-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="brace-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Bracket`

```rust
struct Bracket {
    pub span: DelimSpan,
}
```

*Defined in [`syn-2.0.111/src/token.rs:797-801`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L797-L801)*

``&hellip;``

#### Implementations

- <span id="bracket-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Any for Bracket`

- <span id="bracket-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Bracket`

- <span id="bracket-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Bracket`

- <span id="bracket-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Bracket`

- <span id="bracket-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Bracket`

- <span id="bracket-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Bracket`

##### `impl Debug for Bracket`

- <span id="bracket-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bracket`

- <span id="bracket-default"></span>`fn default() -> Self`

##### `impl Eq for Bracket`

##### `impl<T> From for Bracket`

- <span id="bracket-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Bracket`

- <span id="bracket-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Bracket`

- <span id="bracket-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Bracket`

- <span id="bracket-partialeq-eq"></span>`fn eq(&self, _other: &Bracket) -> bool` — [`Bracket`](#bracket)

##### `impl Sealed for Bracket`

##### `impl ToOwned for Bracket`

- <span id="bracket-toowned-type-owned"></span>`type Owned = T`

- <span id="bracket-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="bracket-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl Token for Bracket`

##### `impl<U> TryFrom for Bracket`

- <span id="bracket-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="bracket-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Bracket`

- <span id="bracket-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="bracket-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Paren`

```rust
struct Paren {
    pub span: DelimSpan,
}
```

*Defined in [`syn-2.0.111/src/token.rs:797-801`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L797-L801)*

`(`&hellip;`)`

#### Implementations

- <span id="paren-surround"></span>`fn surround<F>(&self, tokens: &mut TokenStream, f: F)`

#### Trait Implementations

##### `impl Any for Paren`

- <span id="paren-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Paren`

- <span id="paren-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Paren`

- <span id="paren-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Paren`

- <span id="paren-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Paren`

- <span id="paren-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Paren`

##### `impl Debug for Paren`

- <span id="paren-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Paren`

- <span id="paren-default"></span>`fn default() -> Self`

##### `impl Eq for Paren`

##### `impl<T> From for Paren`

- <span id="paren-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Paren`

- <span id="paren-hash"></span>`fn hash<H: Hasher>(&self, _state: &mut H)`

##### `impl<U> Into for Paren`

- <span id="paren-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Paren`

- <span id="paren-partialeq-eq"></span>`fn eq(&self, _other: &Paren) -> bool` — [`Paren`](#paren)

##### `impl Sealed for Paren`

##### `impl ToOwned for Paren`

- <span id="paren-toowned-type-owned"></span>`type Owned = T`

- <span id="paren-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="paren-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl Token for Paren`

##### `impl<U> TryFrom for Paren`

- <span id="paren-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="paren-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Paren`

- <span id="paren-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="paren-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Token`

```rust
trait Token: private::Sealed { ... }
```

*Defined in [`syn-2.0.111/src/token.rs:125-133`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L125-L133)*

Marker trait for types that represent single tokens.

This trait is sealed and cannot be implemented for types outside of Syn.

#### Implementors

- [`Abstract`](#abstract)
- [`AndAnd`](#andand)
- [`AndEq`](#andeq)
- [`And`](#and)
- [`As`](#as)
- [`Async`](#async)
- [`At`](#at)
- [`Auto`](#auto)
- [`Await`](#await)
- [`Become`](#become)
- [`Box`](#box)
- [`Brace`](#brace)
- [`Bracket`](#bracket)
- [`Break`](#break)
- [`CaretEq`](#careteq)
- [`Caret`](#caret)
- [`Colon`](#colon)
- [`Comma`](#comma)
- [`Const`](#const)
- [`Continue`](#continue)
- [`Crate`](#crate)
- [`Default`](#default)
- [`Do`](#do)
- [`Dollar`](#dollar)
- [`DotDotDot`](#dotdotdot)
- [`DotDotEq`](#dotdoteq)
- [`DotDot`](#dotdot)
- [`Dot`](#dot)
- [`Dyn`](#dyn)
- [`Else`](#else)
- [`Enum`](#enum)
- [`EqEq`](#eqeq)
- [`Eq`](#eq)
- [`Extern`](#extern)
- [`FatArrow`](#fatarrow)
- [`Final`](#final)
- [`Fn`](#fn)
- [`For`](#for)
- [`Ge`](#ge)
- [`Group`](#group)
- [`Gt`](#gt)
- [`Ident`](../ident/index.md#ident)
- [`If`](#if)
- [`Impl`](#impl)
- [`In`](#in)
- [`LArrow`](#larrow)
- [`Le`](#le)
- [`Let`](#let)
- [`Lifetime`](../lifetime/index.md#lifetime)
- [`LitBool`](../lit/index.md#litbool)
- [`LitByteStr`](../lit/index.md#litbytestr)
- [`LitByte`](../lit/index.md#litbyte)
- [`LitCStr`](../lit/index.md#litcstr)
- [`LitChar`](../lit/index.md#litchar)
- [`LitFloat`](../lit/index.md#litfloat)
- [`LitInt`](../lit/index.md#litint)
- [`LitStr`](../lit/index.md#litstr)
- [`Lit`](../lit/index.md#lit)
- [`Loop`](#loop)
- [`Lt`](#lt)
- [`Macro`](#macro)
- [`Match`](#match)
- [`MinusEq`](#minuseq)
- [`Minus`](#minus)
- [`Mod`](#mod)
- [`Move`](#move)
- [`Mut`](#mut)
- [`Ne`](#ne)
- [`Not`](#not)
- [`OrEq`](#oreq)
- [`OrOr`](#oror)
- [`Or`](#or)
- [`Override`](#override)
- [`Paren`](#paren)
- [`PathSep`](#pathsep)
- [`PercentEq`](#percenteq)
- [`Percent`](#percent)
- [`PlusEq`](#pluseq)
- [`Plus`](#plus)
- [`Pound`](#pound)
- [`Priv`](#priv)
- [`Pub`](#pub)
- [`Question`](#question)
- [`RArrow`](#rarrow)
- [`Raw`](#raw)
- [`Ref`](#ref)
- [`Return`](#return)
- [`SelfType`](#selftype)
- [`SelfValue`](#selfvalue)
- [`Semi`](#semi)
- [`ShlEq`](#shleq)
- [`Shl`](#shl)
- [`ShrEq`](#shreq)
- [`Shr`](#shr)
- [`SlashEq`](#slasheq)
- [`Slash`](#slash)
- [`StarEq`](#stareq)
- [`Star`](#star)
- [`Static`](#static)
- [`Struct`](#struct)
- [`Super`](#super)
- [`Tilde`](#tilde)
- [`Trait`](#trait)
- [`Try`](#try)
- [`Type`](#type)
- [`Typeof`](#typeof)
- [`Underscore`](#underscore)
- [`Union`](#union)
- [`Unsafe`](#unsafe)
- [`Unsized`](#unsized)
- [`Use`](#use)
- [`Virtual`](#virtual)
- [`Where`](#where)
- [`While`](#while)
- [`Yield`](#yield)
- `T`
- `proc_macro2::Group`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenTree`

## Macros

### `impl_low_level_token!`

*Defined in [`syn-2.0.111/src/token.rs:163-179`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L163-L179)*

### `define_keywords!`

*Defined in [`syn-2.0.111/src/token.rs:201-301`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L201-L301)*

### `impl_deref_if_len_is_1!`

*Defined in [`syn-2.0.111/src/token.rs:303-321`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L303-L321)*

### `define_punctuation_structs!`

*Defined in [`syn-2.0.111/src/token.rs:323-398`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L323-L398)*

### `define_punctuation!`

*Defined in [`syn-2.0.111/src/token.rs:400-440`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L400-L440)*

### `define_delimiters!`

*Defined in [`syn-2.0.111/src/token.rs:442-519`](../../../.source_1765521767/syn-2.0.111/src/token.rs#L442-L519)*

