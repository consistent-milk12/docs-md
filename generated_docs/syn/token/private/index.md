*[syn](../../index.md) / [token](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithSpan`](#withspan) | struct | Support writing `token.span` rather than `token.spans[0]` on tokens that hold a single span. |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `WithSpan`

```rust
struct WithSpan {
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:147-149`](../../../../.source_1765521767/syn-2.0.111/src/token.rs#L147-L149)*

Support writing `token.span` rather than `token.spans[0]` on tokens that
hold a single span.

#### Trait Implementations

##### `impl Any for WithSpan`

- <span id="withspan-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithSpan`

- <span id="withspan-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithSpan`

- <span id="withspan-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for WithSpan`

- <span id="withspan-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithSpan`

- <span id="withspan-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for WithSpan`

- <span id="withspan-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withspan-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithSpan`

- <span id="withspan-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withspan-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`syn-2.0.111/src/token.rs:141`](../../../../.source_1765521767/syn-2.0.111/src/token.rs#L141)*

#### Implementors

- [`Abstract`](../index.md#abstract)
- [`AndAnd`](../index.md#andand)
- [`AndEq`](../index.md#andeq)
- [`And`](../index.md#and)
- [`As`](../index.md#as)
- [`Async`](../index.md#async)
- [`At`](../index.md#at)
- [`Auto`](../index.md#auto)
- [`Await`](../index.md#await)
- [`Become`](../index.md#become)
- [`Box`](../index.md#box)
- [`Brace`](../index.md#brace)
- [`Bracket`](../index.md#bracket)
- [`Break`](../index.md#break)
- [`CaretEq`](../index.md#careteq)
- [`Caret`](../index.md#caret)
- [`Colon`](../index.md#colon)
- [`Comma`](../index.md#comma)
- [`Const`](../index.md#const)
- [`Continue`](../index.md#continue)
- [`Crate`](../index.md#crate)
- [`Default`](../index.md#default)
- [`Do`](../index.md#do)
- [`Dollar`](../index.md#dollar)
- [`DotDotDot`](../index.md#dotdotdot)
- [`DotDotEq`](../index.md#dotdoteq)
- [`DotDot`](../index.md#dotdot)
- [`Dot`](../index.md#dot)
- [`Dyn`](../index.md#dyn)
- [`Else`](../index.md#else)
- [`Enum`](../index.md#enum)
- [`EqEq`](../index.md#eqeq)
- [`Eq`](../index.md#eq)
- [`Extern`](../index.md#extern)
- [`FatArrow`](../index.md#fatarrow)
- [`Final`](../index.md#final)
- [`Fn`](../index.md#fn)
- [`For`](../index.md#for)
- [`Ge`](../index.md#ge)
- [`Group`](../index.md#group)
- [`Gt`](../index.md#gt)
- [`Ident`](../../ident/index.md#ident)
- [`If`](../index.md#if)
- [`Impl`](../index.md#impl)
- [`In`](../index.md#in)
- [`LArrow`](../index.md#larrow)
- [`Le`](../index.md#le)
- [`Let`](../index.md#let)
- [`Lifetime`](../../lifetime/index.md#lifetime)
- [`LitBool`](../../lit/index.md#litbool)
- [`LitByteStr`](../../lit/index.md#litbytestr)
- [`LitByte`](../../lit/index.md#litbyte)
- [`LitCStr`](../../lit/index.md#litcstr)
- [`LitChar`](../../lit/index.md#litchar)
- [`LitFloat`](../../lit/index.md#litfloat)
- [`LitInt`](../../lit/index.md#litint)
- [`LitStr`](../../lit/index.md#litstr)
- [`Lit`](../../lit/index.md#lit)
- [`Loop`](../index.md#loop)
- [`Lt`](../index.md#lt)
- [`Macro`](../index.md#macro)
- [`Match`](../index.md#match)
- [`MinusEq`](../index.md#minuseq)
- [`Minus`](../index.md#minus)
- [`Mod`](../index.md#mod)
- [`Move`](../index.md#move)
- [`Mut`](../index.md#mut)
- [`Ne`](../index.md#ne)
- [`Not`](../index.md#not)
- [`OrEq`](../index.md#oreq)
- [`OrOr`](../index.md#oror)
- [`Or`](../index.md#or)
- [`Override`](../index.md#override)
- [`Paren`](../index.md#paren)
- [`PathSep`](../index.md#pathsep)
- [`PercentEq`](../index.md#percenteq)
- [`Percent`](../index.md#percent)
- [`PlusEq`](../index.md#pluseq)
- [`Plus`](../index.md#plus)
- [`Pound`](../index.md#pound)
- [`Priv`](../index.md#priv)
- [`Pub`](../index.md#pub)
- [`Question`](../index.md#question)
- [`RArrow`](../index.md#rarrow)
- [`Raw`](../index.md#raw)
- [`Ref`](../index.md#ref)
- [`Return`](../index.md#return)
- [`SelfType`](../index.md#selftype)
- [`SelfValue`](../index.md#selfvalue)
- [`Semi`](../index.md#semi)
- [`ShlEq`](../index.md#shleq)
- [`Shl`](../index.md#shl)
- [`ShrEq`](../index.md#shreq)
- [`Shr`](../index.md#shr)
- [`SlashEq`](../index.md#slasheq)
- [`Slash`](../index.md#slash)
- [`StarEq`](../index.md#stareq)
- [`Star`](../index.md#star)
- [`Static`](../index.md#static)
- [`Struct`](../index.md#struct)
- [`Super`](../index.md#super)
- [`Tilde`](../index.md#tilde)
- [`Trait`](../index.md#trait)
- [`Try`](../index.md#try)
- [`Type`](../index.md#type)
- [`Typeof`](../index.md#typeof)
- [`Underscore`](../index.md#underscore)
- [`Union`](../index.md#union)
- [`Unsafe`](../index.md#unsafe)
- [`Unsized`](../index.md#unsized)
- [`Use`](../index.md#use)
- [`Virtual`](../index.md#virtual)
- [`Where`](../index.md#where)
- [`While`](../index.md#while)
- [`Yield`](../index.md#yield)
- `T`
- `proc_macro2::Group`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenTree`

